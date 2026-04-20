//! Emit typed ChainEntry variants back to #-delimited chain content strings.

use crate::ir::{ChainEntry, RefKind, TogType};

/// Emit a list of chain entries back to a #-delimited content string.
pub fn emit_chain_entries(entries: &[ChainEntry]) -> String {
    entries.iter()
        .map(|e| emit_single_entry(e))
        .collect::<Vec<_>>()
        .join("#")
}

fn emit_position(pos: &Option<String>) -> String {
    match pos {
        Some(p) => format!("{}.", p),
        None => String::new(),
    }
}

fn emit_tog_name(tt: &TogType) -> &'static str {
    match tt {
        TogType::Time => "togtime",
        TogType::Targ => "togtarg",
        TogType::Fri => "togfri",
        TogType::Vis => "togvis",
        TogType::Eft => "togeft",
        TogType::Pip => "togpip",
        TogType::Key => "togkey",
        TogType::Unt => "togunt",
        TogType::Res => "togres",
        TogType::ResM => "togresm",
        TogType::ResA => "togresa",
        TogType::ResO => "togreso",
        TogType::ResX => "togresx",
        TogType::ResN => "togresn",
        TogType::ResS => "togress",
    }
}

/// Emit a single chain entry to its textmod string representation.
pub fn emit_single_entry(entry: &ChainEntry) -> String {
    match entry {
        ChainEntry::Parenthesized { entries } => {
            format!("({})", emit_chain_entries(entries))
        }
        ChainEntry::Keyword { keyword, position } => {
            format!("{}k.{}", emit_position(position), keyword)
        }
        ChainEntry::Hat { entity, position } => {
            format!("{}hat.{}", emit_position(position), entity)
        }
        ChainEntry::Facade { entity_code, parameter } => {
            if parameter.is_empty() {
                format!("facade.{}", entity_code)
            } else {
                format!("facade.{}:{}", entity_code, parameter)
            }
        }
        ChainEntry::Cast { effect } => format!("cast.{}", effect),
        ChainEntry::Splice { item } => format!("splice.{}", item),
        ChainEntry::Enchant { modifier } => format!("enchant.{}", modifier),
        ChainEntry::Learn { ability } => format!("Learn {}", ability),
        ChainEntry::Sidesc { text } => format!("sidesc.{}", text),
        ChainEntry::TogItem { tog_type, position } => {
            format!("{}{}", emit_position(position), emit_tog_name(tog_type))
        }
        ChainEntry::EntityRef { kind, hash, part, multiplier } => {
            let prefix = match kind {
                RefKind::Item => "ritemx",
                RefKind::Modifier => "rmod",
                RefKind::Monster => "rmon",
            };
            let mut s = format!("{}.{}", prefix, hash);
            if let Some(p) = part {
                s.push_str(&format!(".part.{}", p));
            }
            if let Some(m) = multiplier {
                s.push_str(&format!(".m.{}", m));
            }
            s
        }
        ChainEntry::Memory => "Memory".to_string(),
        ChainEntry::ItemRef { name, position } => {
            format!("{}{}", emit_position(position), name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::TogType;

    #[test]
    fn emit_keyword() {
        let entry = ChainEntry::Keyword {
            keyword: "scared".to_string(),
            position: Some("left".to_string()),
        };
        assert_eq!(emit_single_entry(&entry), "left.k.scared");
    }

    #[test]
    fn emit_facade() {
        let entry = ChainEntry::Facade {
            entity_code: "bas170".to_string(),
            parameter: "55".to_string(),
        };
        assert_eq!(emit_single_entry(&entry), "facade.bas170:55");
    }

    #[test]
    fn emit_tog_item() {
        let entry = ChainEntry::TogItem {
            tog_type: TogType::Key,
            position: None,
        };
        assert_eq!(emit_single_entry(&entry), "togkey");
    }

    #[test]
    fn emit_parenthesized() {
        let entry = ChainEntry::Parenthesized {
            entries: vec![
                ChainEntry::TogItem { tog_type: TogType::Key, position: None },
            ],
        };
        assert_eq!(emit_single_entry(&entry), "(togkey)");
    }

    #[test]
    fn emit_entity_ref_with_part_and_multiplier() {
        let entry = ChainEntry::EntityRef {
            kind: RefKind::Item,
            hash: "22c42be4".to_string(),
            part: Some(0),
            multiplier: Some(2),
        };
        assert_eq!(emit_single_entry(&entry), "ritemx.22c42be4.part.0.m.2");
    }

    #[test]
    fn emit_multiple_entries() {
        let entries = vec![
            ChainEntry::Keyword { keyword: "scared".to_string(), position: Some("left".to_string()) },
            ChainEntry::Facade { entity_code: "bas170".to_string(), parameter: "55".to_string() },
        ];
        assert_eq!(emit_chain_entries(&entries), "left.k.scared#facade.bas170:55");
    }

    #[test]
    fn emit_item_ref() {
        let entry = ChainEntry::ItemRef {
            name: "Blindfold".to_string(),
            position: None,
        };
        assert_eq!(emit_single_entry(&entry), "Blindfold");
    }
}
