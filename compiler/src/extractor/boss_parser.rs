use crate::ir::{Boss, BossFightUnit, BossFightVariant, BossFormat, Source};
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

    let doc = extract_depth0_doc(modifier);
    let modifier_chain = util::extract_modifier_chain(modifier)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let level = extract_level(modifier);

    // Split into variants by finding all .fight. blocks
    let variants = extract_variants(modifier);

    Boss {
        name,
        level,
        format: BossFormat::Standard,
        encounter_id: None,
        variants,
        event_body: None,
        doc,
        modifier_chain,
        source: Source::Base,
    }
}

/// Parse a ch.om( event boss — stores the full event body and extracts level + name.
///
/// Event bosses have complex structure extending beyond the initial ch.om(...):
/// `ch.om(initial_body)&Hidden.mn.A@4m(branch_A)...&Hidden.mn.G)&Hidden.mn.OverallName`
///
/// The event_body captures everything between `ch.om` and the final `.mn.Name`.
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

    Boss {
        name,
        level,
        format: BossFormat::Event,
        encounter_id: None,
        variants: vec![],
        event_body: Some(body),
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
    let (level, fight_units) = if let Some(im) = im_pos {
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
        let fight_units = if let Some(fight_pos) = inner.find(".fight.") {
            let fight_content = &inner[fight_pos + ".fight.".len()..];
            // Strip trailing &hidden if present
            let fight_content = fight_content.strip_suffix("&hidden")
                .unwrap_or(fight_content);
            parse_flat_fight_units(fight_content)
        } else {
            vec![]
        };

        (level, fight_units)
    } else {
        (None, vec![])
    };

    let variant = BossFightVariant {
        name: name.clone(),
        trigger: None,
        fight_units,
    };

    Boss {
        name,
        level,
        format: BossFormat::Encounter,
        encounter_id,
        variants: vec![variant],
        event_body: None,
        doc: None,
        modifier_chain: None,
        source: Source::Base,
    }
}

/// Extract fight variants from a standard ch.om modifier.
///
/// Multi-variant modifiers have the pattern:
/// `.fight.(units...).mn.Name@trigger.fight.(units...)...mn.OverallName`
///
/// Single-variant: one `.fight.(...)` block.
fn extract_variants(modifier: &str) -> Vec<BossFightVariant> {
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
        return vec![BossFightVariant {
            name: String::new(),
            trigger: None,
            fight_units: vec![],
        }];
    }

    let mut variants = Vec::new();

    for (vi, &fight_pos) in fight_positions.iter().enumerate() {
        let after_fight = &modifier[fight_pos + ".fight.".len()..];

        // Parse fight units from the paren-wrapped block
        let fight_units = if after_fight.starts_with('(') {
            let abs_start = fight_pos + ".fight.".len();
            if let Some(close) = util::find_matching_close_paren(modifier, abs_start) {
                let content = &modifier[abs_start + 1..close];
                let unit_strs = util::split_at_depth0(content, '+');
                unit_strs.iter().filter_map(|s| parse_fight_unit(s)).collect()
            } else {
                vec![]
            }
        } else {
            // Flat fight content (non-paren-wrapped) — shouldn't happen for ch.om but handle gracefully
            parse_flat_fight_units(after_fight)
        };

        // Extract variant name and trigger from content BETWEEN this fight block's close
        // and the next .fight. (or end of modifier)
        let (variant_name, trigger) = if vi + 1 < fight_positions.len() {
            // There's a next fight — extract .mn.Name@trigger between the two fights
            let next_fight = fight_positions[vi + 1];
            let between = &modifier[fight_pos..next_fight];
            extract_variant_name_and_trigger(between)
        } else {
            // Last (or only) fight — no variant-level name
            (String::new(), None)
        };

        variants.push(BossFightVariant {
            name: variant_name,
            trigger,
            fight_units,
        });
    }

    variants
}

/// Extract variant name and trigger from content between two .fight. blocks.
/// Looks for `.mn.Name@trigger` pattern.
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
        (String::new(), None)
    }
}

/// Parse fight units from flat (non-paren-wrapped) content.
/// Used by encounter (ph.b+fight) format where units are +separated at depth 0.
fn parse_flat_fight_units(content: &str) -> Vec<BossFightUnit> {
    let unit_strs = util::split_at_depth0(content, '+');
    unit_strs.iter().filter_map(|s| parse_fight_unit(s)).collect()
}

/// Parse a single fight unit from a +separated block.
fn parse_fight_unit(unit_str: &str) -> Option<BossFightUnit> {
    let trimmed = unit_str.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Unwrap outer parens for nested blocks like ((Slimelet...))
    let unwrapped = {
        let mut s = trimmed;
        while s.starts_with('(') && s.ends_with(')') {
            s = &s[1..s.len()-1];
        }
        s
    };

    let template = {
        let content = unwrapped.trim_start_matches('(');
        let content = content.strip_prefix("replica.").unwrap_or(content);
        let end = content.find('.').unwrap_or(content.len().min(30));
        content[..end].to_string()
    };

    let name = util::extract_last_n_name(unwrapped)
        .unwrap_or_else(|| template.clone());
    let hp = util::extract_hp(unwrapped, true);
    let sd = util::extract_sd(unwrapped, true).map(|s| crate::ir::DiceFaces::parse(&s));
    let sprite_data = extract_fight_unit_img(unwrapped);
    let doc = util::extract_simple_prop(unwrapped, ".doc.");
    let template_override = util::extract_simple_prop(unwrapped, ".t.");
    let modifier_chain = util::extract_modifier_chain(unwrapped)
        .map(|s| crate::ir::ModifierChain::parse(&s));

    Some(BossFightUnit {
        template,
        name,
        hp,
        sd,
        sprite_data,
        template_override,
        doc,
        modifier_chain,
    })
}

/// Extract .img. data from a fight unit.
fn extract_fight_unit_img(content: &str) -> Option<String> {
    let pos = util::find_last_at_depth0(content, ".img.")?;
    let val_start = pos + ".img.".len();
    let remaining = &content[val_start..];
    let end = util::find_next_prop_boundary(remaining);
    let val = &remaining[..end];
    if val.is_empty() { None } else { Some(val.to_string()) }
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
