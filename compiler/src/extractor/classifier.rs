use crate::error::CompilerError;
use crate::ir::HeroFormat;

/// The type of a modifier, determined by classification.
#[derive(Debug, Clone, PartialEq)]
pub enum ModifierType {
    Hero,
    HeroPoolBase,
    Monster,
    Boss,
    BossEncounter,
    ItemPool,
    PartyConfig,
    EventModifier,
    Dialog,
    Selector,
    GenSelect,
    BossModifier,
    LevelUpAction,
    PoolReplacement,
    Difficulty,
    ArtCredits,
    EndScreen,
    PhaseModifier,
    Choosable,
    ValueModifier,
    HiddenModifier,
    FightModifier,
}

/// Case-insensitive ASCII contains check — avoids allocating a lowercased copy.
fn contains_ci(haystack: &str, needle: &str) -> bool {
    let needle_bytes = needle.as_bytes();
    if needle_bytes.len() > haystack.len() {
        return false;
    }
    haystack.as_bytes().windows(needle_bytes.len()).any(|window| {
        window.iter().zip(needle_bytes).all(|(h, n)| h.to_ascii_lowercase() == *n)
    })
}

/// Case-insensitive ASCII starts_with check.
fn starts_with_ci(haystack: &str, needle: &str) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }
    haystack.as_bytes()[..needle.len()]
        .iter()
        .zip(needle.as_bytes())
        .all(|(h, n)| h.to_ascii_lowercase() == *n)
}

/// Check whether a phase code like `ph.s` appears in `modifier` at a valid
/// boundary. Valid boundaries are: start of string, preceded by `.`, `(`, `[`,
/// `&`, `;`, whitespace. This matches level-scoped phases (`1.(ph.s...`), flat
/// phases (`.ph.s`), compound modifiers (`hidden&ph.!m(...)`), and ph.s used
/// inside paren or brace groups.
fn contains_phase_code(modifier: &str, code: &str) -> bool {
    let bytes = modifier.as_bytes();
    let needle = code.as_bytes();
    if needle.is_empty() || needle.len() > bytes.len() { return false; }
    for i in 0..=(bytes.len() - needle.len()) {
        if &bytes[i..i + needle.len()] == needle {
            let before_ok = i == 0
                || matches!(bytes[i - 1], b'.' | b'(' | b'[' | b'&' | b';' | b' ' | b'\t' | b'\n');
            if before_ok {
                return true;
            }
        }
    }
    false
}

/// Detect the hero format of a modifier string.
pub fn detect_hero_format(modifier: &str) -> HeroFormat {
    if contains_ci(modifier, "ph.b") && contains_ci(modifier, "!mheropool.") {
        HeroFormat::Sliceymon
    } else if contains_ci(modifier, "heropool.") && contains_ci(modifier, "replica.") {
        HeroFormat::Grouped
    } else {
        HeroFormat::Unknown
    }
}

/// Classify a modifier string by its type.
///
/// Classification patterns (order matters -- first match wins).
/// The logic below must handle all mod formats (pansaer, punpuns, sliceymon, community).
/// Unrecognized patterns produce `ErrorKind::Classify` rather than a silent Unknown —
/// this surfaces new constructs so they can be explicitly added via a typed variant.
pub fn classify(modifier: &str, modifier_index: usize) -> Result<ModifierType, CompilerError> {
    // PoolReplacement: starts with "((" and contains "heropool."
    // Must be before Hero check — these contain heropool + replica but are pool overrides.
    if starts_with_ci(modifier, "((heropool.") {
        return Ok(ModifierType::PoolReplacement);
    }

    // Hero: contains "heropool" AND contains "replica."
    if contains_ci(modifier, "heropool") && contains_ci(modifier, "replica.") {
        return Ok(ModifierType::Hero);
    }

    // HeroPoolBase: contains "heropool" but NOT "replica."
    if contains_ci(modifier, "heropool") && !contains_ci(modifier, "replica.") {
        return Ok(ModifierType::HeroPoolBase);
    }

    // Monster: contains "monsterpool."
    // Compound monster pools contain multiple entries. Three forms:
    // (a) `(X+Y+...)` wrapped list — detected by `modifier.starts_with('(')`
    // (b) `monsterpool.X+Y+...` flat list with `+` at paren-depth 0
    // (c) `monsterpool.((nested)...)` triple-nested complex content too rich
    //     for the flat Monster IR — keep as an ItemPool structural.
    // Single-monster modifiers have no depth-0 `+` and at most one leading `(`.
    if contains_ci(modifier, "monsterpool.") {
        if modifier.starts_with('(') {
            return Ok(ModifierType::ItemPool);
        }
        let pool_pos = modifier.to_ascii_lowercase().find("monsterpool.").unwrap_or(0);
        let pool_content = &modifier[pool_pos + "monsterpool.".len()..];
        // Skip leading whitespace/newlines to see the first real character.
        let content_trimmed = pool_content.trim_start();
        if content_trimmed.starts_with("((") {
            return Ok(ModifierType::ItemPool);
        }
        let mut depth: i32 = 0;
        for b in pool_content.bytes() {
            match b {
                b'(' => depth += 1,
                b')' => depth -= 1,
                b'+' if depth == 0 => return Ok(ModifierType::ItemPool),
                _ => {}
            }
        }
        return Ok(ModifierType::Monster);
    }

    // ItemPool variants must be detected before the BossEncounter catchall —
    // `ph.b{name};1;!mitempool.(...)` is an item-injection modifier, not a boss.
    // Check ItemPool first:
    if contains_ci(modifier, "!mitempool.") || contains_ci(modifier, "!m\nitempool.") {
        return Ok(ModifierType::ItemPool);
    }

    // BossEncounter: ph.b format with .fight. OR .add. (NOT heropool — those are heroes).
    // Community variant uses `add.` instead of `fight.` inside the !m(...) body.
    // Guide-documented: ph.b is BooleanPhase; encounter idiom uses ph.b{id};1;!m(level.fight.X)
    if contains_phase_code(modifier, "ph.b")
        && (contains_ci(modifier, ".fight.")
            || contains_ci(modifier, "!m(add.")
            || contains_ci(modifier, "!mphi.")
            || contains_ci(modifier, "!m(party.")
            || contains_ci(modifier, "!m\nm(party."))
        && !contains_ci(modifier, "heropool")
    {
        return Ok(ModifierType::BossEncounter);
    }

    // Standard Boss: starts with `ch.om` (ChoiceOnMap / override-on-map).
    // Variants: ch.om{N}.fight.(UNITS), ch.om(N.ph.s...) event, ch.omN.add.(X) community.
    if starts_with_ci(modifier, "ch.om") {
        return Ok(ModifierType::Boss);
    }
    // Still allow .fight. anywhere as a Boss fallback.
    if contains_ci(modifier, ".fight.") {
        return Ok(ModifierType::Boss);
    }

    // ItemPool: starts with "itempool."
    if starts_with_ci(modifier, "itempool.") {
        return Ok(ModifierType::ItemPool);
    }

    // Top-level `item.<…>` modifiers are retired. The four working mods
    // contain zero top-level `item.` modifiers (verified 2026-04-24:
    // `rg -o '^item\.|[,!+]item\.[a-z]' working-mods/*.txt` returns empty).
    // Any future mod that uses this shape is a new corpus that needs a
    // design decision, not a silent fallthrough.
    if starts_with_ci(modifier, "item.") {
        let preview: String = modifier.chars().take(120).collect();
        return Err(CompilerError::classify(
            modifier_index,
            preview,
            "Top-level `item.<…>` modifiers are not currently modeled. \
             Summon items belong inside `itempool.((…))` envelopes.",
        ));
    }

    // PartyConfig: starts with "=party." OR starts with "((party." OR "party."
    // (community mod uses party.(...) without the `=` prefix, and punpuns
    // double-wraps: `((party.gambler.n.X...))`).
    if modifier.starts_with("=party.")
        || starts_with_ci(modifier, "((party.")
        || starts_with_ci(modifier, "party.")
    {
        return Ok(ModifierType::PartyConfig);
    }

    // EventModifier: starts with "="
    if modifier.starts_with('=') {
        return Ok(ModifierType::EventModifier);
    }

    // LevelUpAction: hidden level-up trigger (Delevel / Level Up / Hidden) in a ch.m modifier
    if contains_ci(modifier, "level up") && contains_phase_code(modifier, "ph.!m") {
        return Ok(ModifierType::LevelUpAction);
    }
    // community uses `ch.m(Delevel&Level Up&Hidden)&Hidden.mn.Delevel`
    if contains_ci(modifier, "ch.m(") && contains_ci(modifier, "level up") {
        return Ok(ModifierType::LevelUpAction);
    }

    // ArtCredits: .ph.4 with .mn.Art Credits (before generic Dialog)
    if contains_phase_code(modifier, "ph.4") && modifier.contains(".mn.Art Credits") {
        return Ok(ModifierType::ArtCredits);
    }

    // Dialog: contains ph.4 (MessagePhase) at any boundary
    if contains_phase_code(modifier, "ph.4") {
        return Ok(ModifierType::Dialog);
    }

    // GenSelect: ph.c and ch.ov
    if contains_phase_code(modifier, "ph.c") && modifier.contains("ch.ov") {
        return Ok(ModifierType::GenSelect);
    }

    // Choosable: ph.c (ChoicePhase) — documented in guide
    if contains_phase_code(modifier, "ph.c") {
        return Ok(ModifierType::Choosable);
    }

    // Difficulty: ph.s with diff. or "difficulty" — (before generic Selector)
    if contains_phase_code(modifier, "ph.s")
        && (modifier.contains("diff.") || contains_ci(modifier, "difficulty"))
    {
        return Ok(ModifierType::Difficulty);
    }

    // Selector: contains ph.s (SeqPhase — guide uses SeqPhase but we name it Selector).
    // Matches level-scoped variants like `1.(ph.s...)` and bare `.ph.s`.
    if contains_phase_code(modifier, "ph.s") {
        return Ok(ModifierType::Selector);
    }

    // PhaseModifier: ph.! (SimpleChoicePhase) or phi. (phase-indexed shorthand)
    if contains_phase_code(modifier, "ph.!") || contains_phase_code(modifier, "phi.") {
        return Ok(ModifierType::PhaseModifier);
    }

    // BooleanPhase at top level — narrative `(ph.b...)`. Already caught above if
    // paired with .fight./!m(...), so any remaining ph.b is narrative/value.
    if contains_phase_code(modifier, "ph.b") {
        return Ok(ModifierType::Selector);
    }

    // RandomRevealPhase
    if contains_phase_code(modifier, "ph.r") {
        return Ok(ModifierType::Selector);
    }

    // BossModifier — ch.om-less patterns
    if contains_ci(modifier, "levelstart.") && modifier.contains("ch.om") {
        return Ok(ModifierType::BossModifier);
    }
    if (modifier.contains("no flee") || contains_ci(modifier, "noflee") || contains_ci(modifier, "no flee"))
        && modifier.contains(".mn.")
    {
        return Ok(ModifierType::BossModifier);
    }
    if modifier.contains("horde") && modifier.contains(".mn.") {
        return Ok(ModifierType::BossModifier);
    }

    // ValueModifier: ch.v (value set/add)
    if contains_ci(modifier, "ch.v") {
        return Ok(ModifierType::ValueModifier);
    }

    // HiddenModifier: any modifier with "&hidden" but no other classification
    if contains_ci(modifier, "&hidden") {
        return Ok(ModifierType::HiddenModifier);
    }

    // No pattern matched — surface an explicit error rather than silently
    // bucketing as Unknown. Per the design principle, every modifier in the
    // 4 working mods must classify into a typed variant. A new pattern here
    // means either (a) add a classifier rule for a documented construct, or
    // (b) add a new ModifierType variant for a new construct found in a mod.
    let preview: String = modifier.chars().take(120).collect();
    Err(CompilerError::classify(
        modifier_index,
        preview,
        "no classifier pattern matched this modifier",
    ))
}
