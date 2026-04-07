use crate::ir::{Boss, BossFightUnit};
use crate::util;

pub fn parse_boss(modifier: &str, _modifier_index: usize) -> Boss {
    let name = util::extract_mn_name(modifier)
        .or_else(|| util::extract_last_n_name(modifier))
        .unwrap_or_default();
    let doc = util::extract_simple_prop(modifier, ".doc.");
    let modifier_chain = util::extract_modifier_chain(modifier);

    // Extract fight content
    let fight = extract_fight_data(modifier);

    Boss {
        name,
        level: extract_level(modifier),
        template: fight.template,
        hp: fight.hp,
        sd: fight.sd,
        sprite_name: fight.sprite_name,
        doc,
        modifier_chain,
        fight_units: fight.fight_units,
        variant: extract_variant(modifier),
        raw: Some(modifier.to_string()),
    }
}

struct FightData {
    template: Option<String>,
    hp: Option<u16>,
    sd: Option<String>,
    sprite_name: Option<String>,
    fight_units: Vec<BossFightUnit>,
}

/// Extract data from the .fight.(...) section.
fn extract_fight_data(modifier: &str) -> FightData {
    let empty = FightData { template: None, hp: None, sd: None, sprite_name: None, fight_units: vec![] };
    let fight_start = match modifier.find(".fight.") {
        Some(pos) => pos,
        None => return empty,
    };
    let after_fight = &modifier[fight_start + ".fight.".len()..];

    // Find the opening paren and its matching close
    if !after_fight.starts_with('(') {
        return empty;
    }

    let fight_close = util::find_matching_close_paren(
        &modifier[fight_start + ".fight.".len()..],
        0,
    );
    if fight_close.is_none() {
        return empty;
    }
    let fight_content = &after_fight[1..fight_close.unwrap()];

    // Split fight content at depth-0 '+' to get individual units
    let unit_strs = util::split_at_depth0(fight_content, '+');

    // Extract top-level properties from first unit
    let first_unit = unit_strs.first().map(|s| s.as_str()).unwrap_or("");
    let template = first_unit
        .strip_prefix("replica.")
        .or_else(|| first_unit.strip_prefix("(replica."))
        .and_then(|r| r.find('.').map(|end| r[..end].to_string()))
        .or_else(|| {
            // Try to get template from first word
            let trimmed = first_unit.trim_start_matches('(');
            let content = trimmed.strip_prefix("replica.").unwrap_or(trimmed);
            let end = content.find('.').unwrap_or(content.len().min(30));
            let t = &content[..end];
            if t.is_empty() { None } else { Some(t.to_string()) }
        });

    let hp = util::extract_hp(fight_content, false);
    let sd = util::extract_sd(fight_content, false);

    // Sprite name: use .n. from first unit
    let sprite_name = util::extract_simple_prop(fight_content, ".n.");

    // Parse individual fight units
    let mut fight_units = Vec::new();
    for unit_str in &unit_strs {
        if let Some(unit) = parse_fight_unit(unit_str) {
            fight_units.push(unit);
        }
    }

    FightData { template, hp, sd, sprite_name, fight_units }
}

/// Parse a single fight unit from a +separated block.
fn parse_fight_unit(unit_str: &str) -> Option<BossFightUnit> {
    let trimmed = unit_str.trim();
    if trimmed.is_empty() {
        return None;
    }

    let template = {
        let content = trimmed.trim_start_matches('(');
        let content = content.strip_prefix("replica.").unwrap_or(content);
        let end = content.find('.').unwrap_or(content.len().min(30));
        content[..end].to_string()
    };

    let name = util::extract_simple_prop(trimmed, ".n.")
        .unwrap_or_else(|| template.clone());

    let hp = util::extract_hp(trimmed, false);
    let sd = util::extract_sd(trimmed, false);

    // Inline sprite data
    let sprite_data = util::extract_simple_prop(trimmed, ".img.");

    Some(BossFightUnit {
        template,
        name,
        hp,
        sd,
        sprite_data,
    })
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

fn extract_variant(modifier: &str) -> Option<String> {
    let mn = util::extract_mn_name(modifier)?;
    let lower = mn.to_lowercase();
    if lower.contains("gen6") || lower.contains("gen7") || lower.contains("variant") {
        Some(mn)
    } else {
        None
    }
}
