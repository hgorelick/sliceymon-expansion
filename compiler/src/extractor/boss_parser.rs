use crate::ir::{Boss, FightDefinition, BossFormat, Source, Phase, PhaseType, PhaseContent, RichText};
use crate::extractor::fight_parser;
use crate::util;

/// Parse a standard ch.om boss modifier into a Boss struct.
///
/// Handles three sub-formats:
/// 1. Paren-wrapped: `ch.omN.fight.(units+...)`
/// 2. Flat fight: `ch.omN.fight.Template+Template2...`
/// 3. Event: `ch.om(N.ph.s...narrative...)` — interactive boss event
pub fn parse_boss(modifier: &str, _modifier_index: usize) -> Boss {
    let name = util::extract_mn_name(modifier).unwrap_or_default();

    // Detect event format: ch.om( — content starts with ( immediately
    if modifier.starts_with("ch.om(") {
        return parse_event_boss(modifier, name);
    }

    let level = extract_level(modifier);

    // Split into fight definitions by finding all .fight. blocks
    let fights = extract_fights(modifier);

    // Boss-level doc/chain come from content AFTER the last fight.
    // For ch.om bosses with flat fight content, fight units have depth-0 `.doc.`
    // and `.i.` markers that must not be mistaken for boss-level properties.
    let boss_tail = extract_boss_tail(modifier);
    let doc = extract_depth0_doc(&boss_tail);
    let modifier_chain = util::extract_modifier_chain(&boss_tail)
        .map(|s| crate::ir::ModifierChain::parse(&s));

    Boss {
        name,
        level,
        format: BossFormat::Standard,
        encounter_id: None,
        fights,
        event_phases: None,
        doc,
        modifier_chain,
        source: Source::Base,
    }
}

/// Parse a ch.om( event boss — stores the full event body as a Message phase and extracts level + name.
///
/// Event bosses have complex structure extending beyond the initial ch.om(...):
/// `ch.om(initial_body)&Hidden.mn.A@4m(branch_A)...&Hidden.mn.G)&Hidden.mn.OverallName`
///
/// The event body is stored as a single Message phase for round-trip safety.
fn parse_event_boss(modifier: &str, name: String) -> Boss {
    let prefix = "ch.om";
    let after_prefix = &modifier[prefix.len()..];

    // Find the final .mn. (the overall name) using rfind
    let body_end = modifier.rfind(".mn.")
        .unwrap_or(modifier.len());
    let body = modifier[prefix.len()..body_end].to_string();

    // Level: first digits inside the initial (...)
    let level = if after_prefix.starts_with('(') {
        after_prefix[1..].chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u8>()
            .ok()
    } else {
        None
    };

    // Store event body as a single Message phase for round-trip safety
    let event_phases = Some(vec![Phase {
        phase_type: PhaseType::Message,
        level_scope: None,
        content: PhaseContent::Message {
            text: RichText::new(body),
            button_text: None,
        },
    }]);

    Boss {
        name,
        level,
        format: BossFormat::Event,
        encounter_id: None,
        fights: vec![],
        event_phases,
        doc: None,
        modifier_chain: None,
        source: Source::Base,
    }
}

/// Parse a ph.b+fight encounter modifier into a Boss struct.
///
/// Format: `1.ph.bX;1;!m(N.fight.Template.n.Name...&hidden).mn.BossName@2!m(skip&hidden&temporary)`
pub fn parse_encounter(modifier: &str, _modifier_index: usize) -> Boss {
    let name = util::extract_mn_name(modifier).unwrap_or_default();

    // Extract encounter ID: letter after "ph.b"
    let encounter_id = modifier.to_lowercase().find("ph.b")
        .and_then(|pos| modifier.as_bytes().get(pos + 4).copied())
        .and_then(|b| {
            let c = b as char;
            if c.is_ascii_alphabetic() { Some(c) } else { None }
        });

    // Extract content inside !m(...)
    let im_pos = modifier.find("!m(");
    let (level, enemies) = if let Some(im) = im_pos {
        let inner_start = im + 3;
        // Find the matching close paren for !m(...)
        let inner_end = util::find_matching_close_paren(modifier, im + 2)
            .unwrap_or(modifier.len());
        let inner = &modifier[inner_start..inner_end];

        // Level: number before .fight.
        let level = if let Some(fight_pos) = inner.find(".fight.") {
            let before = &inner[..fight_pos];
            before.parse::<u8>().ok()
        } else {
            None
        };

        // Fight content: everything after .fight.
        let enemies = if let Some(fight_pos) = inner.find(".fight.") {
            let fight_content = &inner[fight_pos + ".fight.".len()..];
            // Strip trailing &hidden if present
            let fight_content = fight_content.strip_suffix("&hidden")
                .unwrap_or(fight_content);
            fight_parser::parse_fight_units(fight_content)
        } else {
            vec![]
        };

        (level, enemies)
    } else {
        (None, vec![])
    };

    let fight = FightDefinition {
        level: None,
        enemies,
        name: Some(name.clone()),
        trigger: None,
    };

    Boss {
        name,
        level,
        format: BossFormat::Encounter,
        encounter_id,
        fights: vec![fight],
        event_phases: None,
        doc: None,
        modifier_chain: None,
        source: Source::Base,
    }
}

/// Extract fight definitions from a standard ch.om modifier.
///
/// Multi-variant modifiers have the pattern:
/// `.fight.(units...).mn.Name@trigger.fight.(units...)...mn.OverallName`
///
/// Single-variant: one `.fight.(...)` block.
fn extract_fights(modifier: &str) -> Vec<FightDefinition> {
    // Find all .fight. positions at depth 0
    let mut fight_positions = Vec::new();
    let bytes = modifier.as_bytes();
    let mut depth: i32 = 0;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0 && modifier[i..].starts_with(".fight.") {
            fight_positions.push(i);
        }
    }

    if fight_positions.is_empty() {
        return vec![FightDefinition {
            level: None,
            enemies: vec![],
            name: None,
            trigger: None,
        }];
    }

    let mut fights = Vec::new();

    for (vi, &fight_pos) in fight_positions.iter().enumerate() {
        let after_fight = &modifier[fight_pos + ".fight.".len()..];
        // Upper bound: the next depth-0 `.fight.` position (or end of modifier).
        // Without this bound, flat-format fight content bleeds into subsequent
        // variants and produces garbage units.
        let bound = if vi + 1 < fight_positions.len() {
            fight_positions[vi + 1] - (fight_pos + ".fight.".len())
        } else {
            after_fight.len()
        };

        // Parse fight units from fight content.
        let bounded = &after_fight[..bound.min(after_fight.len())];
        let trimmed = drop_variant_trailer(bounded);

        let enemies = if after_fight.starts_with('(') {
            let abs_start = fight_pos + ".fight.".len();
            let close = util::find_matching_close_paren(modifier, abs_start);

            // Detect mixed format: paren-wrapped first unit followed by
            // +unit2+unit3 at depth 0 (challenge-mode bosses like ch.omN).
            // vs. all-inside format: (unit1+unit2+unit3) with no content
            // after the close paren.
            let has_units_after_paren = close.map_or(false, |c| {
                let after_close = c + 1 - (fight_pos + ".fight.".len());
                let remaining = &trimmed[after_close.min(trimmed.len())..];
                remaining.contains('+')
            });

            if has_units_after_paren {
                // Mixed format: split the FULL content at depth-0 '+'.
                // Each segment is a complete fight unit (may include
                // paren-wrapped content with trailing properties).
                let unit_strs = util::split_at_depth0(trimmed, '+');
                let trimmed_last = trim_at_boss_props(unit_strs.last().map_or("", |s| s));
                let mut enemies: Vec<_> = unit_strs[..unit_strs.len() - 1]
                    .iter()
                    .filter_map(|s| fight_parser::parse_fight_unit(s))
                    .collect();
                if let Some(last) = fight_parser::parse_fight_unit(trimmed_last) {
                    enemies.push(last);
                }
                enemies
            } else if let Some(close) = close {
                // All units inside one paren group: (unit1+unit2+unit3)
                let content = &modifier[abs_start + 1..close];
                util::split_at_depth0(content, '+')
                    .iter()
                    .filter_map(|s| fight_parser::parse_fight_unit(s))
                    .collect()
            } else {
                vec![]
            }
        } else {
            // Flat fight content (non-paren-wrapped): community + pansaer formats.
            fight_parser::parse_fight_units(trimmed)
        };

        // Extract variant name and trigger from content BETWEEN this fight block's close
        // and the next .fight. (or end of modifier)
        let (variant_name, trigger) = if vi + 1 < fight_positions.len() {
            // There's a next fight — extract .mn.Name@trigger between the two fights
            let next_fight = fight_positions[vi + 1];
            let between = &modifier[fight_pos..next_fight];
            extract_variant_name_and_trigger(between)
        } else {
            // Last fight: in multi-variant bosses, a penultimate depth-0 `.mn.`
            // is the last fight's variant label; the final `.mn.` is the boss.
            extract_last_fight_variant(&modifier[fight_pos..])
        };

        fights.push(FightDefinition {
            level: None,
            enemies,
            name: if variant_name.is_empty() { None } else { Some(variant_name) },
            trigger,
        });
    }

    fights
}

/// Extract the variant name for the LAST fight of a multi-variant ch.om boss.
/// `fight_content` is the modifier substring starting at the last `.fight.`.
/// Returns (variant_name, trigger) — variant_name is non-empty only when the
/// content has two or more depth-0 `.mn.` (penultimate = variant, last = boss).
fn extract_last_fight_variant(fight_content: &str) -> (String, Option<String>) {
    let bytes = fight_content.as_bytes();
    let mut depth: i32 = 0;
    let mut mn_positions: Vec<usize> = Vec::new();
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0 && fight_content[i..].starts_with(".mn.") {
            mn_positions.push(i);
        }
    }
    if mn_positions.len() < 2 {
        return (String::new(), None);
    }
    // Penultimate `.mn.` is the variant; its value ends at the last `.mn.` (boss).
    let variant_pos = mn_positions[mn_positions.len() - 2];
    let boss_pos = mn_positions[mn_positions.len() - 1];
    let variant_value_start = variant_pos + ".mn.".len();
    let raw = &fight_content[variant_value_start..boss_pos];
    // Trigger is the `@...` tail of the variant (if any).
    if let Some(at) = raw.find('@') {
        (raw[..at].to_string(), Some(raw[at..].to_string()))
    } else {
        (raw.to_string(), None)
    }
}

/// Extract variant name and trigger from content between two .fight. blocks.
///
/// Two patterns:
/// 1. `.mn.Name@trigger` — named variant with trigger
/// 2. `...content)@trigger` — bare trigger without .mn. (the variant has no name)
fn extract_variant_name_and_trigger(content: &str) -> (String, Option<String>) {
    // Find .mn. in this content
    if let Some(mn_pos) = content.rfind(".mn.") {
        let after_mn = &content[mn_pos + 4..];
        // Name ends at @
        if let Some(at_pos) = after_mn.find('@') {
            let name = after_mn[..at_pos].to_string();
            let trigger = after_mn[at_pos..].to_string();
            // Trim trigger at next .fight. or end
            let trigger = if let Some(f) = trigger.find(".fight.") {
                trigger[..f].to_string()
            } else {
                trigger
            };
            (name, Some(trigger))
        } else {
            (after_mn.to_string(), None)
        }
    } else {
        // No .mn. — look for bare @trigger at depth 0
        let bytes = content.as_bytes();
        let mut depth: i32 = 0;
        for i in 0..bytes.len() {
            match bytes[i] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                b'@' if depth == 0 => {
                    let trigger = content[i..].to_string();
                    let trigger = if let Some(f) = trigger.find(".fight.") {
                        trigger[..f].to_string()
                    } else {
                        trigger
                    };
                    return (String::new(), Some(trigger));
                }
                _ => {}
            }
        }
        (String::new(), None)
    }
}

/// Extract .doc. at depth 0, stopping at `)` which would close an outer scope.
fn extract_depth0_doc(modifier: &str) -> Option<String> {
    let pos = util::find_at_depth0(modifier, ".doc.")?;
    let val_start = pos + ".doc.".len();
    let remaining = &modifier[val_start..];
    let boundary = util::find_next_prop_boundary(remaining);
    let mut end = boundary.min(remaining.len());
    let mut depth: i32 = 0;
    for (i, ch) in remaining.char_indices() {
        if i >= end { break; }
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
    }
    let val = &remaining[..end];
    if val.is_empty() { None } else { Some(val.to_string()) }
}

/// Trim a flat fight's unit-list content at the point where a variant trailer
/// begins. Trailers take forms like `...&Hidden@4m...` or `....mn.Name@4m...` —
/// the `@` at depth 0 indicates the start of a trigger to the next variant.
fn drop_variant_trailer(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'@' if depth == 0 => {
                // Trim any `&Hidden`, `.mn.Name`, or similar trailer ending at
                // this `@`. We walk back to the last natural boundary.
                // The safest heuristic: trim back to the nearest `&` or `.mn.`
                // at depth 0; otherwise cut right before `@`.
                let prefix = &s[..i];
                if let Some(p) = prefix.rfind('&') {
                    return &s[..p];
                }
                if let Some(p) = prefix.rfind(".mn.") {
                    return &s[..p];
                }
                return &s[..i];
            }
            _ => {}
        }
    }
    s
}

/// Extract boss-level content that follows all fight variants.
///
/// For ch.om bosses, the structure is:
/// `ch.omN.fight.UNITS@trigger.fight.UNITS...@trigger.fight.UNITS.doc.X.mn.Name`
///
/// Boss-level `.doc.` and modifier chain appear between the last fight's content
/// and `.mn.`. Fight unit content (even at depth 0 when units are flat) must
/// NOT be treated as boss-level properties.
fn extract_boss_tail(modifier: &str) -> String {
    // Find last `.mn.` at depth 0 — this is the overall boss name.
    // Also collect all depth-0 `.mn.` positions so we can detect multi-variant
    // bosses where a penultimate `.mn.VariantName` sits between the last fight
    // and `.mn.BossName`. In that layout the doc/img preceding `.mn.VariantName`
    // belong to the last fight unit, NOT the boss — return empty tail.
    let bytes = modifier.as_bytes();
    let mut depth: i32 = 0;
    let mut mn_positions: Vec<usize> = Vec::new();
    let mut last_fight: Option<usize> = None;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            if modifier[i..].starts_with(".mn.") {
                mn_positions.push(i);
            } else if modifier[i..].starts_with(".fight.") {
                last_fight = Some(i);
            }
        }
    }
    // Multi-variant last-fight with named variant: two+ depth-0 `.mn.` after
    // last `.fight.` means the penultimate is a variant label, and the doc/img
    // preceding it is unit content. No extractable boss tail.
    if let Some(fp) = last_fight {
        let after_fight_mn = mn_positions.iter().filter(|&&p| p > fp).count();
        if after_fight_mn >= 2 {
            return String::new();
        }
    }
    let mn_pos = mn_positions.last().copied().unwrap_or(modifier.len());

    // The last fight runs from `last_fight` position up to `.mn.` (or next fight).
    // Everything BEFORE the last `.fight.` that could be boss-tail is trailing
    // props of the prior fight. Strictly, the boss tail is content between the
    // last fight's actual END and `.mn.`. For flat fight content there's no
    // paren close — boss-level props would have to appear AFTER the last unit's
    // properties. But since fight units carry their own props and chains, any
    // `.doc.X`/chain after the last unit is ambiguous. Safest: only extract
    // boss tail from content after the LAST depth-0 `)` when that `)` appears
    // AFTER the last `.fight.` position; otherwise return empty (flat last
    // fight, no boss-level tail).
    let fight_pos = last_fight.unwrap_or(0);
    let scan_end = mn_pos;
    let mut last_depth0_close: Option<usize> = None;
    let mut d: i32 = 0;
    for i in 0..scan_end {
        match bytes[i] {
            b'(' => d += 1,
            b')' => {
                d -= 1;
                if d == 0 && i > fight_pos {
                    last_depth0_close = Some(i);
                }
            }
            _ => {}
        }
    }

    match last_depth0_close {
        Some(pos) => modifier[pos + 1..mn_pos].to_string(),
        None => String::new(),
    }
}

/// Trim trailing boss-level properties from content after a fight's close paren.
///
/// Fight units use `.n.Name` (display name) but never `.mn.` — that's a
/// modifier-level property marking the boss/variant name. We trim at the LAST
/// `.mn.` at depth 0, which is always the overall boss name.
fn trim_at_boss_props(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut last_mn: Option<usize> = None;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if depth == 0 && s[i..].starts_with(".mn.") => {
                last_mn = Some(i);
            }
            _ => {}
        }
    }
    if let Some(pos) = last_mn {
        &s[..pos]
    } else {
        s
    }
}

fn extract_level(modifier: &str) -> Option<u8> {
    let marker = "ch.om";
    let mut search_from = 0;
    while let Some(pos) = modifier[search_from..].find(marker) {
        let start = search_from + pos + marker.len();
        let remaining = &modifier[start..];
        let mut end = 0;
        for ch in remaining.chars() {
            if ch.is_ascii_digit() { end += 1; } else { break; }
        }
        if end > 0 {
            if let Ok(level) = remaining[..end].parse::<u8>() {
                return Some(level);
            }
        }
        search_from = start;
    }
    None
}
