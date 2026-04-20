//! Parse chain segment content into typed ChainEntry variants.
//!
//! Chain content comes from `.i.` or `.sticker.` segments. Within a segment:
//! - `#` at paren depth 0 separates entries.
//! - Within one `#`-piece, MULTIPLE entries may also be chained by `.` at depth 0
//!   when a new entry prefix begins (e.g., `ritemx.8bfc.splice.self.(X)` is
//!   an EntityRef followed by a Splice).
//!
//! Entry types recognized: Parenthesized, Keyword (k.), Hat (hat.), TogItem
//! (togX), Facade (facade.), Cast (cast.), Splice (splice.), Enchant (enchant.),
//! Sidesc (sidesc.), Learn ("Learn "), EntityRef (ritemx/rmod/rmon), Memory,
//! and a fallback ItemRef for bare item names.

use crate::ir::{ChainEntry, RefKind, TogType};

/// Split a string by `#` at parenthesis depth 0.
fn split_hash_depth0(s: &str) -> Vec<&str> {
    let mut pieces = Vec::new();
    let mut depth: i32 = 0;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            '#' if depth == 0 => {
                pieces.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    if start <= s.len() {
        pieces.push(&s[start..]);
    }
    pieces
}

/// Find the index of the closing ')' that matches an opening '(' at the start.
fn find_matching_close(s: &str) -> Option<usize> {
    if !s.starts_with('(') {
        return None;
    }
    let mut depth: i32 = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Known position prefixes, ordered longest-first to avoid substring false matches.
const POSITION_PREFIXES: &[&str] = &[
    "rightmost.", "bottom.", "middle.", "topbot.", "right5.", "right3.",
    "right2.", "left2.", "right.", "mid4.", "mid2.", "left.", "self.",
    "top.", "mid.", "bot.", "row.", "all.",
];

/// Strip a known position prefix from the beginning of an entry string.
/// Returns `(position, rest)` where position is Some if a prefix was found.
fn strip_position_prefix(s: &str) -> (Option<String>, &str) {
    for prefix in POSITION_PREFIXES {
        if s.starts_with(prefix) {
            let name = &prefix[..prefix.len() - 1]; // remove trailing dot
            return (Some(name.to_string()), &s[prefix.len()..]);
        }
    }
    (None, s)
}

/// Tog item names, ordered longest-first so e.g. "togresa" is matched before "togres".
const TOG_ITEMS_LONGEST_FIRST: &[&str] = &[
    "togresa", "togresm", "togresn", "togreso", "togress", "togresx",
    "togtime", "togtarg", "togtime", "togres",
    "togfri", "togvis", "togeft", "togpip", "togkey", "togunt",
];

/// Parse a TogType from a tog item name string.
fn parse_tog_type(name: &str) -> Option<TogType> {
    match name {
        "togtime" => Some(TogType::Time),
        "togtarg" => Some(TogType::Targ),
        "togfri" => Some(TogType::Fri),
        "togvis" => Some(TogType::Vis),
        "togeft" => Some(TogType::Eft),
        "togpip" => Some(TogType::Pip),
        "togkey" => Some(TogType::Key),
        "togunt" => Some(TogType::Unt),
        "togres" => Some(TogType::Res),
        "togresm" => Some(TogType::ResM),
        "togresa" => Some(TogType::ResA),
        "togreso" => Some(TogType::ResO),
        "togresx" => Some(TogType::ResX),
        "togresn" => Some(TogType::ResN),
        "togress" => Some(TogType::ResS),
        _ => None,
    }
}

/// Test whether `s` begins with a recognized entry marker (optionally after a
/// position prefix). Used to decide whether a `.` at depth 0 delimits the end
/// of the current entry and the start of the next one.
fn starts_with_entry(s: &str) -> bool {
    let after_pos = strip_position_prefix(s).1;
    if after_pos.starts_with("k.") || after_pos.starts_with("hat.") {
        return true;
    }
    if tog_match_prefix(after_pos).is_some() {
        return true;
    }
    if after_pos.starts_with("facade.")
        || after_pos.starts_with("cast.")
        || after_pos.starts_with("splice.")
        || after_pos.starts_with("enchant.")
        || after_pos.starts_with("sidesc.")
        || after_pos.starts_with("Learn ")
    {
        return true;
    }
    let lower = after_pos.to_ascii_lowercase();
    lower.starts_with("ritemx.") || lower.starts_with("rmod.") || lower.starts_with("rmon.")
}

/// If `s` starts with a tog item name (optionally followed by `.` or end),
/// return the matched name.
fn tog_match_prefix(s: &str) -> Option<&'static str> {
    for name in TOG_ITEMS_LONGEST_FIRST {
        if s == *name {
            return Some(name);
        }
        if s.starts_with(name) {
            let after = &s[name.len()..];
            if after.starts_with('.') || after.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

/// Find the boundary of a facade value. Facade values are alphanumeric codes
/// with optional numeric `:DIGITS:DIGITS:...` parameters. Stops at the regular
/// entry boundary OR at `&`/`@`/`+` at depth 0 (top-level modifier separators).
fn find_facade_boundary(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'&' | b'@' | b'+' if depth == 0 => return i,
            b'.' if depth == 0 => {
                if starts_with_entry(&s[i + 1..]) {
                    return i;
                }
            }
            _ => {}
        }
        i += 1;
    }
    bytes.len()
}

/// Find the boundary in `s` where the current entry ends and the next one
/// begins. Returns the byte index of the boundary (just before the leading `.`),
/// or `s.len()` if no boundary is found.
fn find_entry_boundary(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'.' if depth == 0 => {
                if starts_with_entry(&s[i + 1..]) {
                    return i;
                }
            }
            _ => {}
        }
        i += 1;
    }
    bytes.len()
}

/// Peel off the first ChainEntry from `piece`, returning (entry, remainder).
/// Remainder does NOT include the separating `.` — the caller will handle that.
fn peel_one_entry(piece: &str) -> (ChainEntry, &str) {
    // 1. Parenthesized group at the very start
    if piece.starts_with('(') {
        if let Some(close) = find_matching_close(piece) {
            let inner = &piece[1..close];
            let remainder = &piece[close + 1..];
            let entries = parse_chain_entries(inner);
            return (ChainEntry::Parenthesized { entries }, remainder);
        }
    }

    // 2. Strip position prefix
    let (position, after_pos) = strip_position_prefix(piece);
    let pos_consumed = piece.len() - after_pos.len();

    // Empty after position: bare position reference
    if after_pos.is_empty() {
        return (
            ChainEntry::ItemRef { name: piece.to_string(), position: None },
            "",
        );
    }

    // 3. Keyword: k.NAME (name runs until next entry boundary)
    if let Some(kw_rest) = after_pos.strip_prefix("k.") {
        let boundary = find_entry_boundary(kw_rest);
        let (keyword, remainder) = (&kw_rest[..boundary], &kw_rest[boundary..]);
        return (
            ChainEntry::Keyword { keyword: keyword.to_string(), position },
            remainder,
        );
    }

    // 4. Hat: hat.ENTITY (entity may contain nested parens; runs to next boundary)
    if let Some(hat_rest) = after_pos.strip_prefix("hat.") {
        let boundary = find_entry_boundary(hat_rest);
        let (entity, remainder) = (&hat_rest[..boundary], &hat_rest[boundary..]);
        return (
            ChainEntry::Hat { entity: entity.to_string(), position },
            remainder,
        );
    }

    // 5. Tog items — name with optional `.remainder`
    if let Some(tog_name) = tog_match_prefix(after_pos) {
        let after_tog = &after_pos[tog_name.len()..];
        // after_tog is either "" or starts with "."
        let remainder = if let Some(stripped) = after_tog.strip_prefix('.') {
            // Preserve the leading `.` so the caller treats it as a boundary
            // Actually: peel loop strips the leading `.` itself; so pass without `.`
            // Wait — our contract is "remainder does NOT include the separating `.`"
            stripped
        } else {
            after_tog
        };
        if let Some(tt) = parse_tog_type(tog_name) {
            return (ChainEntry::TogItem { tog_type: tt, position }, remainder);
        }
    }

    // Position-free patterns (only if no position was stripped)
    if position.is_none() {
        // 6. Facade: facade.CODE[:PARAMS] — runs to next entry boundary or
        // to a top-level modifier separator (`&` / `@`) at depth 0. Facade
        // params are numeric-only (e.g., `0`, `0:10:10`, `0:-50:10`), so a
        // `&` or `@` means we've run off the end of the facade value.
        if let Some(rest) = after_pos.strip_prefix("facade.") {
            let boundary = find_facade_boundary(rest);
            let (facade_str, remainder) = (&rest[..boundary], &rest[boundary..]);
            let (entity_code, parameter) = match facade_str.find(':') {
                Some(c) => (facade_str[..c].to_string(), facade_str[c + 1..].to_string()),
                None => (facade_str.to_string(), String::new()),
            };
            return (ChainEntry::Facade { entity_code, parameter }, remainder);
        }

        // 7. Cast: cast.EFFECT — runs to next entry boundary
        if let Some(rest) = after_pos.strip_prefix("cast.") {
            let boundary = find_entry_boundary(rest);
            let (effect, remainder) = (&rest[..boundary], &rest[boundary..]);
            return (
                ChainEntry::Cast { effect: effect.to_string() },
                remainder,
            );
        }

        // 8. Splice: splice.ITEM — consumes everything to end of piece
        //    (Splice item content may contain further chain-like syntax but
        //    it's game-level parsed differently; preserving verbatim is safest.)
        if let Some(rest) = after_pos.strip_prefix("splice.") {
            return (ChainEntry::Splice { item: rest.to_string() }, "");
        }

        // 9. Enchant: enchant.MODIFIER — consumes rest
        if let Some(rest) = after_pos.strip_prefix("enchant.") {
            return (ChainEntry::Enchant { modifier: rest.to_string() }, "");
        }

        // 10. Sidesc: sidesc.TEXT — consumes rest (text may contain dots)
        if let Some(rest) = after_pos.strip_prefix("sidesc.") {
            return (ChainEntry::Sidesc { text: rest.to_string() }, "");
        }

        // 11. Learn: "Learn ABILITY" — consumes rest
        if let Some(rest) = after_pos.strip_prefix("Learn ") {
            return (ChainEntry::Learn { ability: rest.to_string() }, "");
        }

        // 12. Entity references — hash = hex only, optional .part.N / .m.N at depth 0
        let lower = after_pos.to_ascii_lowercase();
        if lower.starts_with("ritemx.") {
            return peel_entity_ref(after_pos, RefKind::Item, 7);
        }
        if lower.starts_with("rmod.") {
            return peel_entity_ref(after_pos, RefKind::Modifier, 5);
        }
        if lower.starts_with("rmon.") {
            return peel_entity_ref(after_pos, RefKind::Monster, 5);
        }

        // 13. Memory
        if after_pos == "Memory" || after_pos == "memory" {
            return (ChainEntry::Memory, "");
        }
    }

    // 14. Default: ItemRef consuming the entire remaining piece
    //     (restores the stripped position prefix into the name field for round-trip)
    let _ = pos_consumed; // unused
    (
        ChainEntry::ItemRef {
            name: after_pos.to_string(),
            position,
        },
        "",
    )
}

/// Peel an entity reference. Hash is hex-only chars from the start.
/// `.part.N` and `.m.N` suffixes only apply at depth 0 immediately after the hash.
fn peel_entity_ref(s: &str, kind: RefKind, prefix_len: usize) -> (ChainEntry, &str) {
    let after = &s[prefix_len..];
    let hash_end = after
        .bytes()
        .take_while(|b| b.is_ascii_hexdigit())
        .count();
    let hash = after[..hash_end].to_string();
    let mut remaining = &after[hash_end..];
    let mut part = None;
    let mut multiplier = None;

    if let Some(rest) = remaining.strip_prefix(".part.") {
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(p) = digits.parse::<u8>() {
            // Only accept if the digits terminate at `.` or end of string.
            let after_digits = &rest[digits.len()..];
            if after_digits.is_empty() || after_digits.starts_with('.') {
                part = Some(p);
                remaining = after_digits;
            }
        }
    }

    if let Some(rest) = remaining.strip_prefix(".m.") {
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(m) = digits.parse::<u8>() {
            let after_digits = &rest[digits.len()..];
            if after_digits.is_empty() || after_digits.starts_with('.') {
                multiplier = Some(m);
                remaining = after_digits;
            }
        }
    }

    (
        ChainEntry::EntityRef { kind, hash, part, multiplier },
        remaining,
    )
}

/// Parse one `#`-piece into a list of entries (usually one, but may be multiple
/// when entries are chained by `.` at depth 0, e.g. `ritemx.HASH.splice.SELF.(X)`).
fn parse_piece(piece: &str) -> Vec<ChainEntry> {
    let mut entries = Vec::new();
    let mut remaining = piece;

    loop {
        if remaining.is_empty() {
            break;
        }
        let before_len = remaining.len();
        let (entry, rest) = peel_one_entry(remaining);
        entries.push(entry);
        // Strip the separator `.` if any
        let next = rest.strip_prefix('.').unwrap_or(rest);
        if next.len() >= before_len {
            // Defensive: no progress — stop to avoid infinite loop
            break;
        }
        remaining = next;
    }

    entries
}

/// Parse chain segment content into typed ChainEntry variants.
/// Content is the string after `.i.` or `.sticker.` in a segment.
pub fn parse_chain_entries(content: &str) -> Vec<ChainEntry> {
    if content.is_empty() {
        return vec![];
    }
    let pieces = split_hash_depth0(content);
    pieces
        .iter()
        .filter(|p| !p.is_empty())
        .flat_map(|p| parse_piece(p).into_iter())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keyword_entry() {
        let entries = parse_chain_entries("k.scared");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::Keyword {
            keyword: "scared".to_string(), position: None,
        });
    }

    #[test]
    fn test_parse_keyword_with_position() {
        let entries = parse_chain_entries("left.k.scared");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::Keyword {
            keyword: "scared".to_string(), position: Some("left".to_string()),
        });
    }

    #[test]
    fn test_parse_facade_entry() {
        let entries = parse_chain_entries("facade.bas170:55");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::Facade {
            entity_code: "bas170".to_string(), parameter: "55".to_string(),
        });
    }

    #[test]
    fn test_parse_hat_entry() {
        let entries = parse_chain_entries("hat.Ace");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::Hat {
            entity: "Ace".to_string(), position: None,
        });
    }

    #[test]
    fn test_parse_tog_entry() {
        let entries = parse_chain_entries("togres");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::TogItem {
            tog_type: TogType::Res, position: None,
        });
    }

    #[test]
    fn test_parse_tog_with_position() {
        let entries = parse_chain_entries("left.togtime");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::TogItem {
            tog_type: TogType::Time, position: Some("left".to_string()),
        });
    }

    #[test]
    fn test_parse_memory() {
        let entries = parse_chain_entries("Memory");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::Memory);
    }

    #[test]
    fn test_parse_sidesc() {
        let entries = parse_chain_entries("sidesc.Add [pink]dejavu[cu]");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::Sidesc {
            text: "Add [pink]dejavu[cu]".to_string(),
        });
    }

    #[test]
    fn test_parse_entity_ref_item() {
        let entries = parse_chain_entries("ritemx.dae9");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::EntityRef {
            kind: RefKind::Item, hash: "dae9".to_string(), part: None, multiplier: None,
        });
    }

    #[test]
    fn test_parse_entity_ref_with_part() {
        let entries = parse_chain_entries("ritemx.132fb.part.1");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::EntityRef {
            kind: RefKind::Item, hash: "132fb".to_string(), part: Some(1), multiplier: None,
        });
    }

    #[test]
    fn test_parse_entity_ref_modifier() {
        let entries = parse_chain_entries("rmod.1270");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::EntityRef {
            kind: RefKind::Modifier, hash: "1270".to_string(), part: None, multiplier: None,
        });
    }

    #[test]
    fn test_parse_entity_ref_monster() {
        let entries = parse_chain_entries("rmon.8");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::EntityRef {
            kind: RefKind::Monster, hash: "8".to_string(), part: None, multiplier: None,
        });
    }

    #[test]
    fn test_parse_entity_ref_with_multiplier() {
        let entries = parse_chain_entries("ritemx.22c42be4.part.0.m.2");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::EntityRef {
            kind: RefKind::Item, hash: "22c42be4".to_string(),
            part: Some(0), multiplier: Some(2),
        });
    }

    #[test]
    fn test_parse_hash_delimited_chain() {
        let entries = parse_chain_entries("k.scared#facade.bas170:55");
        assert_eq!(entries.len(), 2);
        assert!(matches!(&entries[0], ChainEntry::Keyword { .. }));
        assert!(matches!(&entries[1], ChainEntry::Facade { .. }));
    }

    #[test]
    fn test_parse_item_ref() {
        let entries = parse_chain_entries("Blindfold");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::ItemRef {
            name: "Blindfold".to_string(), position: None,
        });
    }

    #[test]
    fn test_parse_parenthesized_entry() {
        let entries = parse_chain_entries("(togkey)");
        assert_eq!(entries.len(), 1);
        if let ChainEntry::Parenthesized { entries: inner } = &entries[0] {
            assert_eq!(inner.len(), 1);
            assert!(matches!(&inner[0], ChainEntry::TogItem { tog_type: TogType::Key, .. }));
        } else {
            panic!("Expected Parenthesized");
        }
    }

    #[test]
    fn test_parse_parenthesized_with_hash() {
        let entries = parse_chain_entries("(k.death#Eye of Horus.m.3)");
        assert_eq!(entries.len(), 1);
        if let ChainEntry::Parenthesized { entries: inner } = &entries[0] {
            assert_eq!(inner.len(), 2);
            assert!(matches!(&inner[0], ChainEntry::Keyword { .. }));
        } else {
            panic!("Expected Parenthesized");
        }
    }

    #[test]
    fn test_parse_learn() {
        let entries = parse_chain_entries("Learn Wings");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::Learn { ability: "Wings".to_string() });
    }

    #[test]
    fn test_parse_cast() {
        let entries = parse_chain_entries("cast.crush");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ChainEntry::Cast { effect: "crush".to_string() });
    }

    #[test]
    fn test_parse_entity_ref_case_insensitive() {
        let entries = parse_chain_entries("Ritemx.62e8");
        assert_eq!(entries.len(), 1);
        assert!(matches!(&entries[0], ChainEntry::EntityRef { kind: RefKind::Item, .. }));
    }

    #[test]
    fn test_empty_content() {
        let entries = parse_chain_entries("");
        assert!(entries.is_empty());
    }

    // --- New tests for recursive entry peeling (the bug-fix case) ---

    #[test]
    fn test_entity_ref_with_trailing_splice() {
        // The exact punpuns Forest Baby case: hash must be hex-only,
        // and `.splice.self.(X)` becomes a separate Splice entry.
        let entries = parse_chain_entries("ritemx.8bfc.splice.self.(unpack.crumbling castle.part.1)");
        assert_eq!(entries.len(), 2, "should peel into EntityRef + Splice");
        assert_eq!(entries[0], ChainEntry::EntityRef {
            kind: RefKind::Item, hash: "8bfc".to_string(), part: None, multiplier: None,
        });
        assert_eq!(entries[1], ChainEntry::Splice {
            item: "self.(unpack.crumbling castle.part.1)".to_string(),
        });
    }

    #[test]
    fn test_entity_ref_hash_hex_only() {
        // Non-hex char after hash must terminate the hash.
        let entries = parse_chain_entries("rmod.abcf.splice.xyz");
        assert_eq!(entries.len(), 2);
        if let ChainEntry::EntityRef { hash, .. } = &entries[0] {
            assert_eq!(hash, "abcf");
        } else { panic!(); }
        assert_eq!(entries[1], ChainEntry::Splice { item: "xyz".to_string() });
    }

    #[test]
    fn test_entity_ref_part_inside_nested_paren_is_not_consumed() {
        // `.part.1` inside the nested `(...)` of a following splice must NOT
        // be captured by the entity ref's part-suffix matching.
        let entries = parse_chain_entries("ritemx.8bfc.splice.(unpack.part.1)");
        assert_eq!(entries.len(), 2);
        if let ChainEntry::EntityRef { hash, part, .. } = &entries[0] {
            assert_eq!(hash, "8bfc");
            assert_eq!(*part, None, "part inside nested parens must not be consumed");
        } else { panic!(); }
    }
}
