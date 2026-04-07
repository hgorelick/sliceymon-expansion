use crate::ir::{ItemPoolEntry, StructuralContent, StructuralType};
use crate::util;

/// Extract the .mn. name from a structural modifier.
pub fn extract_structural_name(raw: &str) -> Option<String> {
    util::extract_mn_name(raw)
}

/// Parse structured content from a structural modifier based on its type.
pub fn parse_structural_content(stype: &StructuralType, raw: &str) -> StructuralContent {
    match stype {
        StructuralType::HeroPoolBase => parse_heropoolbase(raw),
        StructuralType::ItemPool => parse_itempool(raw),
        StructuralType::BossModifier => parse_bossmodifier(raw),
        StructuralType::PartyConfig => parse_partyconfig(raw),
        StructuralType::EventModifier => parse_eventmodifier(raw),
        StructuralType::Dialog | StructuralType::ArtCredits => parse_dialog(raw),
        StructuralType::Selector | StructuralType::Difficulty => parse_selector(raw),
        StructuralType::GenSelect => parse_genselect(raw),
        StructuralType::LevelUpAction => parse_levelupaction(raw),
        StructuralType::PoolReplacement => parse_poolreplacement(raw),
        StructuralType::EndScreen | StructuralType::Unknown => StructuralContent::Raw,
    }
}

fn parse_heropoolbase(raw: &str) -> StructuralContent {
    // HeroPoolBase modifiers are typically: heropool.Name.Name.Name... or heropool.o0.0.part.0
    // Extract hero names by splitting the heropool content at '.' and filtering
    let mut hero_refs = Vec::new();
    if let Some(pos) = raw.to_lowercase().find("heropool.") {
        let content = &raw[pos + "heropool.".len()..];
        // Split at depth-0 '.' — but heropool refs are just dot-separated names
        // Filter out property markers and numeric entries
        for part in content.split('.') {
            let trimmed = part.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("o0")
                || trimmed == "0"
                || trimmed == "part"
                || trimmed == "1"
                || trimmed.starts_with("mn")
                || trimmed.starts_with("&")
            {
                continue;
            }
            // Skip if it looks like a property value
            if trimmed.parse::<u32>().is_ok() {
                continue;
            }
            hero_refs.push(trimmed.to_string());
        }
    }
    StructuralContent::HeroPoolBase { hero_refs }
}

fn parse_itempool(raw: &str) -> StructuralContent {
    // Item pools: itempool.((item1))#((item2))#...
    // or complex nested structures
    let mut items = Vec::new();
    let lower = raw.to_lowercase();

    // Find itempool content
    let start = if let Some(pos) = lower.find("itempool.") {
        pos + "itempool.".len()
    } else if let Some(pos) = lower.find("!mitempool.") {
        pos + "!mitempool.".len()
    } else {
        return StructuralContent::ItemPool { items };
    };

    let content = &raw[start..];

    // Split by '#' at depth 0 to get individual items
    let item_strs = util::split_at_depth0(content, '#');
    for item_str in &item_strs {
        let trimmed = item_str.trim().trim_matches(|c| c == '(' || c == ')');
        if trimmed.is_empty() {
            continue;
        }
        let name = util::extract_simple_prop(trimmed, ".n.")
            .or_else(|| util::extract_mn_name(trimmed))
            .unwrap_or_default();
        let tier = util::extract_simple_prop(trimmed, ".tier.")
            .and_then(|v| v.parse::<i8>().ok());
        items.push(ItemPoolEntry {
            name,
            tier,
            content: item_str.to_string(),
        });
    }

    StructuralContent::ItemPool { items }
}

fn parse_bossmodifier(raw: &str) -> StructuralContent {
    // Boss modifiers contain flags like: levelstart.N, ch.omN, noflee, horde, etc.
    let mut flags = Vec::new();
    if raw.contains("noflee") || raw.contains("no flee") {
        flags.push("noflee".to_string());
    }
    if raw.contains("horde") {
        flags.push("horde".to_string());
    }
    // Extract levelstart patterns
    if let Some(pos) = raw.find("levelstart.") {
        let remaining = &raw[pos..];
        let end = remaining.find([',', '&', ' '])
            .unwrap_or(remaining.len());
        flags.push(remaining[..end].to_string());
    }
    // Extract ch.omN patterns
    let mut search_from = 0;
    while let Some(pos) = raw[search_from..].find("ch.om") {
        let abs = search_from + pos;
        let remaining = &raw[abs..];
        let end = remaining.find(['.', ',', '&', ' ', ')'])
            .unwrap_or(remaining.len());
        if end > 5 {
            flags.push(remaining[..end].to_string());
        }
        search_from = abs + end.max(1);
    }
    StructuralContent::BossModifier { flags }
}

fn parse_partyconfig(raw: &str) -> StructuralContent {
    // =party.Name+(members...)
    let party_name = if let Some(rest) = raw.strip_prefix("=party.") {
        let end = rest.find(['+', '(', '.', ','])
            .unwrap_or(rest.len());
        rest[..end].to_string()
    } else {
        String::new()
    };

    // Extract member names from +() blocks
    let mut members = Vec::new();
    let blocks = util::split_at_depth0(raw, '+');
    for block in blocks.iter().skip(1) {
        // Each block is typically (content).n.Name or just a name
        if let Some(name) = util::extract_simple_prop(block, ".n.") {
            members.push(name);
        } else if let Some(name) = util::extract_last_n_name(block) {
            members.push(name);
        }
    }

    StructuralContent::PartyConfig { party_name, members }
}

fn parse_eventmodifier(raw: &str) -> StructuralContent {
    // =EventName or =event.Name
    let event_name = if let Some(rest) = raw.strip_prefix('=') {
        rest.to_string()
    } else {
        raw.to_string()
    };
    StructuralContent::EventModifier { event_name }
}

fn parse_dialog(raw: &str) -> StructuralContent {
    // Dialog: contains .ph.4
    let phase = if let Some(pos) = raw.find(".ph.") {
        let remaining = &raw[pos + 4..];
        let end = remaining.find(['.', ',', '&'])
            .unwrap_or(remaining.len());
        remaining[..end].to_string()
    } else {
        String::new()
    };
    StructuralContent::Dialog { phase }
}

fn parse_selector(raw: &str) -> StructuralContent {
    // Selector: @1[color]Label@2!m(...) patterns
    let mut options = Vec::new();
    let mut search_from = 0;

    while let Some(pos) = raw[search_from..].find('@') {
        let abs = search_from + pos;
        let after_at = &raw[abs + 1..];

        // Check if this is @N where N is a digit (option marker)
        if let Some(first_char) = after_at.chars().next() {
            if first_char.is_ascii_digit() {
                // Find the label: after the digit, skip !m(...) patterns
                let label_start = abs + 2; // skip @N
                if label_start < raw.len() {
                    let remaining = &raw[label_start..];
                    // The option text runs until next @ at depth 0 or end
                    let end = {
                        let mut d: i32 = 0;
                        let mut e = remaining.len();
                        for (i, ch) in remaining.char_indices() {
                            match ch {
                                '(' => d += 1,
                                ')' => d -= 1,
                                '@' if d == 0 && i > 0 => { e = i; break; }
                                _ => {}
                            }
                        }
                        e
                    };
                    let option_text = &remaining[..end];
                    // Extract a readable label from the option
                    if let Some(mn) = util::extract_mn_name(option_text) {
                        options.push(mn);
                    } else if let Some(n) = util::extract_last_n_name(option_text) {
                        options.push(n);
                    } else {
                        // Take first chunk as label
                        let label_end = option_text.find(['!', '(', '.'])
                            .unwrap_or(option_text.len());
                        let label = option_text[..label_end].trim();
                        if !label.is_empty() {
                            options.push(label.to_string());
                        }
                    }
                }
            }
        }
        search_from = abs + 1;
    }

    StructuralContent::Selector { options }
}

fn parse_genselect(raw: &str) -> StructuralContent {
    // GenSelect uses similar @N option patterns
    let content = parse_selector(raw);
    if let StructuralContent::Selector { options } = content {
        StructuralContent::GenSelect { options }
    } else {
        StructuralContent::GenSelect { options: vec![] }
    }
}

fn parse_levelupaction(raw: &str) -> StructuralContent {
    // Just extract the content
    StructuralContent::LevelUpAction { content: raw.to_string() }
}

fn parse_poolreplacement(raw: &str) -> StructuralContent {
    // ((heropool.Name+Name+(replica.X...).n.X+...))
    // Extract hero names from the pool replacement
    let mut hero_names = Vec::new();

    // Strip outer (( and ))
    let content = raw.trim_start_matches('(').trim_end_matches(')');

    // Find heropool. prefix
    let lower = content.to_lowercase();
    if let Some(pos) = lower.find("heropool.") {
        let after = &content[pos + "heropool.".len()..];
        // Split at depth-0 '+'
        let parts = util::split_at_depth0(after, '+');
        for part in &parts {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            // If it contains a replica block, extract .n. from it
            if trimmed.contains("replica.") {
                if let Some(name) = util::extract_last_n_name(trimmed) {
                    hero_names.push(name);
                } else if let Some(name) = util::extract_mn_name(trimmed) {
                    hero_names.push(name);
                }
            } else {
                // Plain name (no parens/replica)
                let name = trimmed.trim_matches(|c: char| c == '(' || c == ')');
                // Filter out suffix markers
                let end = name.find(['.', '&', ',', '@'])
                    .unwrap_or(name.len());
                let clean = &name[..end];
                if !clean.is_empty() {
                    hero_names.push(clean.to_string());
                }
            }
        }
    }

    StructuralContent::PoolReplacement { hero_names }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_unknown_is_raw() {
        let content = parse_structural_content(&StructuralType::Unknown, "anything");
        assert_eq!(content, StructuralContent::Raw);
    }

    #[test]
    fn structural_name_extracted() {
        let name = extract_structural_name("something.mn.MyName@2!m(skip)");
        assert_eq!(name, Some("MyName".to_string()));
    }

    #[test]
    fn parse_eventmodifier_extracts_name() {
        let content = parse_structural_content(&StructuralType::EventModifier, "=battle.event1");
        if let StructuralContent::EventModifier { event_name } = content {
            assert!(!event_name.is_empty());
        } else {
            panic!("Expected EventModifier");
        }
    }

    #[test]
    fn parse_dialog_extracts_phase() {
        let content = parse_structural_content(&StructuralType::Dialog, "something.ph.4.text");
        if let StructuralContent::Dialog { phase } = content {
            assert_eq!(phase, "4");
        } else {
            panic!("Expected Dialog");
        }
    }

    #[test]
    fn parse_difficulty_as_selector() {
        let content = parse_structural_content(
            &StructuralType::Difficulty,
            ".ph.s@1Heaven@2!m(diff.heaven)@3Easy@4!m(diff.Easy)"
        );
        if let StructuralContent::Selector { options } = content {
            assert!(!options.is_empty());
        } else {
            panic!("Expected Selector for Difficulty");
        }
    }

    #[test]
    fn parse_art_credits_as_dialog() {
        let content = parse_structural_content(
            &StructuralType::ArtCredits,
            "something.ph.4.credits.mn.Art Credits"
        );
        if let StructuralContent::Dialog { phase } = content {
            assert_eq!(phase, "4");
        } else {
            panic!("Expected Dialog for ArtCredits");
        }
    }
}
