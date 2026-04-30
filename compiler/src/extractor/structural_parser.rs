use crate::ir::{StructuralContent, StructuralType};
use crate::util;

/// Extract the .mn. name from a structural modifier.
pub fn extract_structural_name(raw: &str) -> Option<String> {
    util::extract_mn_name(raw)
}

/// Parse structured content from a structural modifier based on its type.
/// Each variant stores the full modifier text as `body` for emission,
/// plus typed summary fields for introspection.
///
/// `StructuralType::ItemPool` is intentionally absent — itempool extraction
/// goes through `replica_item_parser::extract_from_itempool` directly from
/// `extractor/mod.rs`, bypassing this dispatcher. Routing an ItemPool here
/// is a contract violation and panics.
pub fn parse_structural_content(stype: &StructuralType, raw: &str) -> StructuralContent {
    match stype {
        StructuralType::HeroPoolBase => parse_heropoolbase(raw),
        StructuralType::ItemPool => unreachable!(
            "ItemPool is parsed via replica_item_parser::extract_from_itempool, \
             not parse_structural_content"
        ),
        StructuralType::BossModifier => parse_bossmodifier(raw),
        StructuralType::PartyConfig => parse_partyconfig(raw),
        StructuralType::EventModifier => parse_eventmodifier(raw),
        StructuralType::Dialog => parse_dialog(raw),
        StructuralType::ArtCredits => parse_artcredits(raw),
        StructuralType::Selector => parse_selector(raw),
        StructuralType::Difficulty => parse_difficulty(raw),
        StructuralType::GenSelect => parse_genselect(raw),
        StructuralType::LevelUpAction => parse_levelupaction(raw),
        StructuralType::PoolReplacement => parse_poolreplacement(raw),
        StructuralType::EndScreen => parse_endscreen(raw),
        StructuralType::PhaseModifier => StructuralContent::PhaseModifier {
            body: raw.to_string(), level_scope: None, phase: None,
        },
        StructuralType::Choosable => StructuralContent::Choosable {
            body: raw.to_string(), level_scope: None, tag: None,
        },
        StructuralType::ValueModifier => StructuralContent::ValueModifier {
            body: raw.to_string(), level_scope: None, value_ref: None,
        },
        StructuralType::HiddenModifier => StructuralContent::HiddenModifier {
            body: raw.to_string(),
            modifier_type: crate::ir::HiddenModifierType::Skip, // placeholder until reward parsing lands
        },
        StructuralType::FightModifier => StructuralContent::FightModifier {
            body: raw.to_string(), level_scope: None, fight: None,
        },
    }
}

fn parse_heropoolbase(raw: &str) -> StructuralContent {
    let mut hero_refs = Vec::new();
    if let Some(pos) = raw.to_lowercase().find("heropool.") {
        let content = &raw[pos + "heropool.".len()..];
        for part in content.split('.') {
            let trimmed = part.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("o0")
                || trimmed == "0"
                || trimmed == "part"
                || trimmed == "1"
                || trimmed.starts_with("mn")
                || trimmed.starts_with('&')
            {
                continue;
            }
            if trimmed.parse::<u32>().is_ok() {
                continue;
            }
            hero_refs.push(trimmed.to_string());
        }
    }
    StructuralContent::HeroPoolBase { body: raw.to_string(), hero_refs }
}

fn parse_bossmodifier(raw: &str) -> StructuralContent {
    let mut flags = Vec::new();
    if raw.contains("noflee") || raw.contains("no flee") {
        flags.push("noflee".to_string());
    }
    if raw.contains("horde") {
        flags.push("horde".to_string());
    }
    if let Some(pos) = raw.find("levelstart.") {
        let remaining = &raw[pos..];
        let end = remaining.find([',', '&', ' '])
            .unwrap_or(remaining.len());
        flags.push(remaining[..end].to_string());
    }
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
    StructuralContent::BossModifier { body: raw.to_string(), flags }
}

fn parse_partyconfig(raw: &str) -> StructuralContent {
    let party_name = if let Some(rest) = raw.strip_prefix("=party.") {
        let end = rest.find(['+', '(', '.', ','])
            .unwrap_or(rest.len());
        rest[..end].to_string()
    } else {
        String::new()
    };

    let mut members = Vec::new();
    let blocks = util::split_at_depth0(raw, '+');
    for block in blocks.iter().skip(1) {
        if let Some(name) = util::extract_simple_prop(block, ".n.") {
            members.push(name);
        } else if let Some(name) = util::extract_last_n_name(block) {
            members.push(name);
        }
    }

    StructuralContent::PartyConfig { body: raw.to_string(), party_name, members }
}

fn parse_eventmodifier(raw: &str) -> StructuralContent {
    let event_name = if let Some(rest) = raw.strip_prefix('=') {
        rest.to_string()
    } else {
        raw.to_string()
    };
    StructuralContent::EventModifier { body: raw.to_string(), event_name }
}

fn parse_dialog(raw: &str) -> StructuralContent {
    let phase = if let Some(pos) = raw.find(".ph.") {
        let remaining = &raw[pos + 4..];
        let end = remaining.find(['.', ',', '&'])
            .unwrap_or(remaining.len());
        remaining[..end].to_string()
    } else {
        String::new()
    };
    StructuralContent::Dialog { body: raw.to_string(), phase }
}

fn parse_artcredits(raw: &str) -> StructuralContent {
    StructuralContent::ArtCredits { body: raw.to_string() }
}

fn parse_selector(raw: &str) -> StructuralContent {
    let options = extract_options(raw);
    StructuralContent::Selector { body: raw.to_string(), options }
}

fn parse_difficulty(raw: &str) -> StructuralContent {
    let options = extract_options(raw);
    StructuralContent::Difficulty { body: raw.to_string(), options }
}

fn parse_genselect(raw: &str) -> StructuralContent {
    let options = extract_options(raw);
    StructuralContent::GenSelect { body: raw.to_string(), options }
}

fn parse_levelupaction(raw: &str) -> StructuralContent {
    StructuralContent::LevelUpAction { body: raw.to_string() }
}

fn parse_poolreplacement(raw: &str) -> StructuralContent {
    let mut hero_names = Vec::new();

    let content = raw.trim_start_matches('(').trim_end_matches(')');

    let lower = content.to_lowercase();
    if let Some(pos) = lower.find("heropool.") {
        let after = &content[pos + "heropool.".len()..];
        let parts = util::split_at_depth0(after, '+');
        for part in &parts {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.contains("replica.") {
                if let Some(name) = util::extract_last_n_name(trimmed) {
                    hero_names.push(name);
                } else if let Some(name) = util::extract_mn_name(trimmed) {
                    hero_names.push(name);
                }
            } else {
                let name = trimmed.trim_matches(|c: char| c == '(' || c == ')');
                let end = name.find(['.', '&', ',', '@'])
                    .unwrap_or(name.len());
                let clean = &name[..end];
                if !clean.is_empty() {
                    hero_names.push(clean.to_string());
                }
            }
        }
    }

    StructuralContent::PoolReplacement { body: raw.to_string(), hero_names }
}

fn parse_endscreen(raw: &str) -> StructuralContent {
    StructuralContent::EndScreen { body: raw.to_string() }
}

/// Extract option labels from @N-delimited selector text.
fn extract_options(raw: &str) -> Vec<String> {
    let mut options = Vec::new();
    let mut search_from = 0;

    while let Some(pos) = raw[search_from..].find('@') {
        let abs = search_from + pos;
        let after_at = &raw[abs + 1..];

        if let Some(first_char) = after_at.chars().next() {
            if first_char.is_ascii_digit() {
                let label_start = abs + 2;
                if label_start < raw.len() {
                    let remaining = &raw[label_start..];
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
                    if let Some(mn) = util::extract_mn_name(option_text) {
                        options.push(mn);
                    } else if let Some(n) = util::extract_last_n_name(option_text) {
                        options.push(n);
                    } else {
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

    options
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_name_extracted() {
        let name = extract_structural_name("something.mn.MyName@2!m(skip)");
        assert_eq!(name, Some("MyName".to_string()));
    }

    #[test]
    fn parse_eventmodifier_extracts_name() {
        let content = parse_structural_content(&StructuralType::EventModifier, "=battle.event1");
        if let StructuralContent::EventModifier { event_name, body } = content {
            assert!(!event_name.is_empty());
            assert_eq!(body, "=battle.event1");
        } else {
            panic!("Expected EventModifier");
        }
    }

    #[test]
    fn parse_dialog_extracts_phase() {
        let content = parse_structural_content(&StructuralType::Dialog, "something.ph.4.text");
        if let StructuralContent::Dialog { phase, body } = content {
            assert_eq!(phase, "4");
            assert_eq!(body, "something.ph.4.text");
        } else {
            panic!("Expected Dialog");
        }
    }

    #[test]
    fn parse_difficulty_as_own_variant() {
        let content = parse_structural_content(
            &StructuralType::Difficulty,
            ".ph.s@1Heaven@2!m(diff.heaven)@3Easy@4!m(diff.Easy)"
        );
        if let StructuralContent::Difficulty { options, .. } = content {
            assert!(!options.is_empty());
        } else {
            panic!("Expected Difficulty");
        }
    }

    #[test]
    fn parse_art_credits_as_own_variant() {
        let content = parse_structural_content(
            &StructuralType::ArtCredits,
            "something.ph.4.credits.mn.Art Credits"
        );
        if let StructuralContent::ArtCredits { body } = content {
            assert_eq!(body, "something.ph.4.credits.mn.Art Credits");
        } else {
            panic!("Expected ArtCredits");
        }
    }
}
