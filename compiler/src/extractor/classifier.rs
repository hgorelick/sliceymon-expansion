use crate::error::CompilerError;
use crate::ir::HeroFormat;

/// The type of a modifier, determined by classification.
#[derive(Debug, Clone, PartialEq)]
pub enum ModifierType {
    Hero,
    HeroPoolBase,
    Monster,
    Boss,
    Capture,
    Legendary,
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
    Unknown,
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
pub fn classify(modifier: &str, _modifier_index: usize) -> Result<ModifierType, CompilerError> {
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
    if contains_ci(modifier, "monsterpool.") {
        return Ok(ModifierType::Monster);
    }

    // Boss: contains ".fight."
    if contains_ci(modifier, ".fight.") {
        return Ok(ModifierType::Boss);
    }

    // Capture: starts with "itempool." AND contains "hat.replica"
    if starts_with_ci(modifier, "itempool.") && contains_ci(modifier, "hat.replica") {
        return Ok(ModifierType::Capture);
    }

    // Legendary: starts with "itempool." AND contains legendary patterns
    if starts_with_ci(modifier, "itempool.")
        && contains_ci(modifier, "hat.(replica")
        && contains_ci(modifier, "cast.")
    {
        return Ok(ModifierType::Legendary);
    }

    // Compound hero item pools: ph.b[name];1;!mitempool.
    if contains_ci(modifier, "!mitempool.") {
        return Ok(ModifierType::ItemPool);
    }

    // ItemPool: starts with "itempool."
    if starts_with_ci(modifier, "itempool.") {
        return Ok(ModifierType::ItemPool);
    }

    // PartyConfig: starts with "=party."
    if modifier.starts_with("=party.") {
        return Ok(ModifierType::PartyConfig);
    }

    // EventModifier: starts with "="
    if modifier.starts_with('=') {
        return Ok(ModifierType::EventModifier);
    }

    // LevelUpAction: hidden level-up trigger
    if contains_ci(modifier, "level up") && contains_ci(modifier, "ph.!m") {
        return Ok(ModifierType::LevelUpAction);
    }

    // ArtCredits: .ph.4 with .mn.Art Credits (before generic Dialog)
    if modifier.contains(".ph.4") && modifier.contains(".mn.Art Credits") {
        return Ok(ModifierType::ArtCredits);
    }

    // Dialog: contains ".ph.4"
    if modifier.contains(".ph.4") {
        return Ok(ModifierType::Dialog);
    }

    // GenSelect: contains `.ph.c` and `ch.ov`
    if modifier.contains(".ph.c") && modifier.contains("ch.ov") {
        return Ok(ModifierType::GenSelect);
    }

    // Difficulty: .ph.s with diff. or "difficulty" (before generic Selector)
    if modifier.contains(".ph.s") && (modifier.contains("diff.") || contains_ci(modifier, "difficulty")) {
        return Ok(ModifierType::Difficulty);
    }

    // Selector: contains ".ph.s"
    if modifier.contains(".ph.s") {
        return Ok(ModifierType::Selector);
    }

    // BossModifier
    if contains_ci(modifier, "levelstart.") && modifier.contains("ch.om") {
        return Ok(ModifierType::BossModifier);
    }
    if (modifier.contains("no flee") || modifier.contains("horde") || modifier.contains("noflee"))
        && modifier.contains(".mn.")
        && !modifier.contains(".fight.")
    {
        return Ok(ModifierType::BossModifier);
    }

    Ok(ModifierType::Unknown)
}
