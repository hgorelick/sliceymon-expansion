//! Shared utility functions for textmod parsing.
//! These are used across multiple extractor and builder modules.

/// Find the position of `needle` in `haystack` at parenthesis depth 0.
/// Returns the byte index of the first match, or None.
pub fn find_at_depth0(haystack: &str, needle: &str) -> Option<usize> {
    let bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    if needle_bytes.is_empty() || bytes.len() < needle_bytes.len() {
        return None;
    }
    let mut depth: i32 = 0;
    for i in 0..=bytes.len() - needle_bytes.len() {
        if depth == 0 && bytes[i..].starts_with(needle_bytes) {
            return Some(i);
        }
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
    }
    None
}

/// Find the last occurrence of `needle` in `haystack` at parenthesis depth 0.
pub fn find_last_at_depth0(haystack: &str, needle: &str) -> Option<usize> {
    let bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    if needle_bytes.is_empty() || bytes.len() < needle_bytes.len() {
        return None;
    }
    let mut last = None;
    let mut depth: i32 = 0;
    for i in 0..=bytes.len() - needle_bytes.len() {
        if depth == 0 && bytes[i..].starts_with(needle_bytes) {
            last = Some(i);
        }
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
    }
    last
}

/// Find the matching ')' for '(' at `open_pos` in `s`.
pub fn find_matching_close_paren(s: &str, open_pos: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth = 0;
    for (i, &byte) in bytes.iter().enumerate().skip(open_pos) {
        match byte {
            b'(' => depth += 1,
            b')' => {
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

/// Split a string at occurrences of `separator` char at parenthesis depth 0.
pub fn split_at_depth0(s: &str, separator: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth: i32 = 0;
    for ch in s.chars() {
        match ch {
            '(' => { depth += 1; current.push(ch); }
            ')' => { depth -= 1; current.push(ch); }
            c if c == separator && depth == 0 => {
                parts.push(current.clone());
                current.clear();
            }
            _ => { current.push(ch); }
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

/// Check parenthesis balance of a string. Returns Ok(()) or Err with details.
pub fn verify_paren_balance(s: &str) -> Result<(), String> {
    let mut depth: i32 = 0;
    for (i, ch) in s.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Err(format!("Negative depth at position {}", i));
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        Err(format!("Unbalanced: depth {} at end", depth))
    } else {
        Ok(())
    }
}

/// Extract the last `.mn.NAME` from a modifier string.
/// NAME ends at `@`, `&`, `,`, newline, or end of string.
/// Trailing property markers like `.modtier.N` (community mod convention) also
/// terminate the name — they are NOT part of it.
pub fn extract_mn_name(modifier: &str) -> Option<String> {
    let marker = ".mn.";
    let pos = modifier.rfind(marker)?;
    let start = pos + marker.len();
    let remaining = &modifier[start..];
    let mut end = remaining.find(['@', '&', ',', '\n'])
        .unwrap_or(remaining.len());
    // Also end at known trailing property markers (community mods put
    // `.modtier.N` / `.doc.` / `.part.` after `.mn.`).
    for m in &[".modtier.", ".doc.", ".part.", ".img.", ".tier."] {
        if let Some(p) = remaining[..end].find(m) {
            end = p;
        }
    }
    let name = remaining[..end].trim();
    if name.is_empty() { None } else { Some(name.to_string()) }
}

/// Extract the last `.n.NAME` from a modifier at depth 0.
/// NAME ends at `&`, `.`, `+`, or end of string.
pub fn extract_last_n_name(modifier: &str) -> Option<String> {
    let pos = find_last_at_depth0(modifier, ".n.")?;
    let start = pos + 3;
    let remaining = &modifier[start..];
    let end = remaining.find(['&', '.', '+'])
        .unwrap_or(remaining.len());
    let name = &remaining[..end];
    if name.is_empty() { None } else { Some(name.to_string()) }
}

/// Extract `.sd.FACES` value — sequence of `[0-9:-]` chars after `.sd.`.
/// If `depth_aware` is true, only matches at paren depth 0.
pub fn extract_sd(content: &str, depth_aware: bool) -> Option<String> {
    let marker = ".sd.";
    let pos = if depth_aware {
        find_at_depth0(content, marker)?
    } else {
        content.find(marker)?
    };
    let start = pos + marker.len();
    let bytes = content.as_bytes();
    let mut end = start;
    while end < bytes.len() && (bytes[end].is_ascii_digit() || bytes[end] == b':' || bytes[end] == b'-') {
        end += 1;
    }
    if end > start { Some(content[start..end].to_string()) } else { None }
}

/// Extract `.hp.N` value — digits after `.hp.`.
/// If `depth_aware` is true, only matches at paren depth 0 and skips `.triggerhpdata.`.
pub fn extract_hp(content: &str, depth_aware: bool) -> Option<u16> {
    let marker = ".hp.";
    let pos = if depth_aware {
        // Skip .triggerhpdata. matches
        let mut search_from = 0;
        loop {
            let p = find_at_depth0(&content[search_from..], marker)?;
            let abs = search_from + p;
            if abs >= 7 && &content[abs - 7..abs + marker.len()] == ".triggerhpdata." {
                search_from = abs + marker.len();
                continue;
            }
            break Some(abs);
        }?
    } else {
        content.find(marker)?
    };
    let start = pos + marker.len();
    let bytes = content.as_bytes();
    let mut end = start;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    if end > start { content[start..end].parse().ok() } else { None }
}

/// Extract the first `.col.X` where X is a lowercase letter, skipping `.i.col.` patterns.
///
/// When `depth_aware` is true, only matches at paren depth 0 — i.e. the scan
/// ignores any `.col.X` inside a nested paren group. Non-replica callers
/// (hero/monster/boss parsers) pass `false` to preserve legacy behavior.
pub fn extract_color(content: &str, depth_aware: bool) -> Option<char> {
    let bytes = content.as_bytes();
    let marker = b".col.";
    let mut depth: i32 = 0;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => { depth += 1; continue; }
            b')' => { depth -= 1; continue; }
            _ => {}
        }
        if depth_aware && depth != 0 {
            continue;
        }
        if i + marker.len() < bytes.len() && &bytes[i..i + marker.len()] == marker {
            if i >= 2 && &bytes[i - 2..i] == b".i" {
                continue;
            }
            let c = bytes[i + marker.len()];
            if c.is_ascii_lowercase() {
                return Some(c as char);
            }
        }
    }
    None
}

/// Check if a modifier contains a hero with the given color letter.
pub fn has_color(modifier: &str, target: char) -> bool {
    extract_color(modifier, false) == Some(target)
}

/// Return the prefix of `body` preceding the first depth-0 occurrence of any
/// INNER_BODY_MARKERS — the canonical set `{.i., .sticker., .abilitydata.}`. When
/// none exist at depth 0, returns the full `body` slice.
///
/// Scalar extractors for `hp` / `color` / `sd` / `img` feed the result
/// through here so chain sub-entries (`.i.` / `.sticker.` — free-form
/// `.sidesc.` / `.enchant.` text) and ability-effect bodies
/// (`.abilitydata.(...)`) cannot leak interior `.hp.N` / `.col.X` /
/// `.sd.FACES` / `.img.DATA` substrings into top-level fields. Emission
/// places every scalar field before the chain and ability region, so the
/// prefix is sufficient.
///
/// The ability-body marker is `.abilitydata.` per the textmod guide
/// (reference/textmod_guide.md lines 747 / 857 / 975-981); `cast.TRIGGER`
/// in the corpus is a chain keyword (guide lines 642-645), not a property
/// marker.
///
/// Post-8A this helper has no in-tree caller — the legacy top-level
/// `item.TEMPLATE…` replica-item parser was retired (zero corpus
/// instances of top-level `item.<…>`) — see `compiler/tests/retirements.rs`
/// T13. It remains as a reusable depth-0 prefix slicer for future
/// INNER_BODY_MARKERS work.
pub fn slice_before_chain_and_cast(body: &str) -> &str {
    let mut earliest: Option<usize> = None;
    for marker in [".i.", ".sticker.", ".abilitydata."] {
        if let Some(pos) = find_at_depth0(body, marker) {
            earliest = Some(match earliest {
                Some(prev) => prev.min(pos),
                None => pos,
            });
        }
    }
    match earliest {
        Some(pos) => &body[..pos],
        None => body,
    }
}

/// Extract a nested property like `.abilitydata.(...)` at depth 0.
/// Returns the content including outer parens.
pub fn extract_nested_prop(content: &str, marker: &str) -> Option<String> {
    let pos = find_at_depth0(content, marker)?;
    let val_start = pos + marker.len();
    let bytes = content.as_bytes();
    if val_start < bytes.len() && bytes[val_start] == b'(' {
        let close = find_matching_close_paren(content, val_start)?;
        Some(content[val_start..=close].to_string())
    } else {
        None
    }
}

/// Check that a string contains only ASCII characters.
pub fn verify_ascii_only(s: &str) -> Result<(), String> {
    for (i, ch) in s.char_indices() {
        if !ch.is_ascii() {
            return Err(format!("Non-ASCII '{}' (U+{:04X}) at position {}", ch, ch as u32, i));
        }
    }
    Ok(())
}

/// Extract a simple property value at depth 0, ending at the next known property marker.
/// Only matches markers at parenthesis depth 0, so values inside nested paren groups
/// (abilitydata, triggerhpdata, replica-inner) are ignored.
pub fn extract_simple_prop(content: &str, marker: &str) -> Option<String> {
    let pos = find_at_depth0(content, marker)?;
    let val_start = pos + marker.len();
    let remaining = &content[val_start..];
    let val_end = find_next_prop_boundary(remaining);
    let val = &remaining[..val_end];
    if val.is_empty() { None } else { Some(val.to_string()) }
}

/// Find the boundary of the next property marker in a string.
/// Also stops at `)` at depth 0 (which closes an outer scope, not part of the value).
pub fn find_next_prop_boundary(remaining: &str) -> usize {
    let markers = [
        ".col.", ".tier.", ".hp.", ".sd.", ".img.", ".hsv.", ".abilitydata.", ".triggerhpdata.",
        ".doc.", ".hue.", ".speech.", ".n.", ".i.", ".k.", ".facade.", ".sticker.",
        ".mn.", ".part.", ".bal.", ".t.",
    ];
    let bytes = remaining.as_bytes();
    let mut depth: i32 = 0;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth < 0 {
                    return i;
                }
            }
            b'.' if depth == 0 => {
                for marker in &markers {
                    if i + marker.len() <= bytes.len() && &remaining[i..i + marker.len()] == *marker {
                        return i;
                    }
                }
            }
            _ => {}
        }
    }
    remaining.len()
}

/// Extract the modifier chain (.i./.sticker. sequences, plus standalone `.k.X`
/// unit-level keywords) from replica or fight-unit content.
///
/// Standalone `.k.X` detection is conservative — only `.k.` at depth 0 that
/// is NOT preceded by `.i.`, `.sticker.`, `.col.`, `#`, or another `.k.` is
/// treated as a top-level keyword segment. This excludes item sub-entries
/// (`.i.k.X`, `#k.X`) and the color letter `.col.k` boundary.
pub fn extract_modifier_chain(content: &str) -> Option<String> {
    let non_chain_markers = [
        ".col.", ".tier.", ".hp.", ".sd.", ".img.", ".abilitydata.", ".triggerhpdata.",
        ".doc.", ".hue.", ".speech.", ".n.", ".mn.", ".part.", ".bal.",
    ];

    let mut chain_parts: Vec<&str> = Vec::new();
    let bytes = content.as_bytes();
    let mut depth: i32 = 0;
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }

        if depth == 0 && bytes[i] == b'.' {
            let is_dot_i = i + 3 <= bytes.len() && &content[i..i + 3] == ".i.";
            let is_sticker = i + 9 <= bytes.len() && &content[i..i + 9] == ".sticker.";
            let is_chain = is_dot_i || is_sticker;

            // Reject false `.i.` matches: when `.col.X.i.NEXT_PROPERTY`, the `.i.`
            // is really the terminator of the color value (e.g. `col.i`). Detect by
            // checking whether the next content starts with a non-chain property.
            if is_dot_i {
                let after = &content[i + 2..]; // starts with the trailing `.`
                let is_false = non_chain_markers.iter().any(|m| after.starts_with(m));
                if is_false {
                    i += 1;
                    continue;
                }
            }

            if is_chain {
                let mut j = i + 1;
                let mut d: i32 = 0;
                while j < bytes.len() {
                    match bytes[j] {
                        b'(' => d += 1,
                        b')' => {
                            d -= 1;
                            if d < 0 { break; }
                        }
                        // `&` and `@` at depth 0 mark top-level modifier flags
                        // (&Hidden, &temporary) and variant triggers (@4m, @2!m).
                        // Chain content never spans these — break the chunk.
                        b'&' | b'@' if d == 0 => break,
                        _ => {}
                    }
                    if d == 0 && bytes[j] == b'.' {
                        let is_non_chain = non_chain_markers.iter().any(|m| {
                            j + m.len() <= bytes.len() && &content[j..j + m.len()] == *m
                        });
                        if is_non_chain { break; }
                        // Top-level `.t.X` (template override) terminates chain
                        // content. Inside `.i.t.X` or `.k.t.X` it's a sub-entry,
                        // recognised by the 3-char-ending-at-j overlap with `.i.`
                        // or `.k.`.
                        if j + 3 <= bytes.len() && &content[j..j + 3] == ".t." {
                            let preceded_by_chain = j >= 2 && {
                                let pre = &content[j - 2..j + 1];
                                pre == ".i." || pre == ".k."
                            };
                            if !preceded_by_chain { break; }
                        }
                    }
                    j += 1;
                }
                chain_parts.push(&content[i..j]);
                i = j;
                continue;
            }
        }
        i += 1;
    }

    if chain_parts.is_empty() { None } else { Some(chain_parts.join("")) }
}

/// Extract `.img.DATA` value at depth 0. DATA ends at the next known property
/// marker (`.n.`, `.sd.`, etc.), a `)` that closes an outer scope, or end of string.
/// If DATA itself is a `(X)` paren group (e.g. `.img.(rdhero.hsv.0:0:0)`), the
/// full group including the closing `)` is captured.
///
/// This is the shared extraction used by heroes, replica items, monsters, etc.
/// Boss fight units use `extract_simple_prop(".img.")` which works the same way.
pub fn extract_img_data(content: &str) -> Option<String> {
    let pos = find_last_at_depth0(content, ".img.")?;
    let val_start = pos + ".img.".len();
    let remaining = &content[val_start..];
    // Paren-wrapped value: capture the whole matched group verbatim.
    if remaining.starts_with('(') {
        if let Some(close) = find_matching_close_paren(remaining, 0) {
            return Some(remaining[..=close].to_string());
        }
    }
    // End at next property boundary or closing paren
    let end = find_img_data_end(remaining);
    let val = &remaining[..end];
    if val.is_empty() { None } else { Some(val.to_string()) }
}

/// Find where img data ends — at the next known property marker, a depth-0 `)`
/// (which closes an outer scope), or end of string. Depth-aware so that
/// `)` characters inside a paren group inside the value don't terminate it.
fn find_img_data_end(remaining: &str) -> usize {
    let bytes = remaining.as_bytes();
    let mut depth: i32 = 0;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => {
                if depth == 0 {
                    return i;
                }
                depth -= 1;
            }
            b'.' if depth == 0 => {
                // Check for known property markers starting at this position
                let markers = [
                    ".col.", ".tier.", ".hp.", ".sd.", ".abilitydata.", ".triggerhpdata.",
                    ".doc.", ".hue.", ".speech.", ".n.", ".i.", ".k.", ".facade.", ".sticker.",
                    ".mn.", ".part.", ".bal.", ".img.", ".hsv.", ".t.",
                ];
                for marker in &markers {
                    if i + marker.len() <= bytes.len() && &remaining[i..i + marker.len()] == *marker {
                        return i;
                    }
                }
            }
            _ => {}
        }
    }
    remaining.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_img_data_basic() {
        let content = "replica.Lost.col.a.hp.5.sd.170-3:0:0:0:0:0.img.ABC123.n.Gible";
        assert_eq!(extract_img_data(content), Some("ABC123".to_string()));
    }

    #[test]
    fn extract_img_data_at_end() {
        let content = "replica.Lost.sd.0:0:0:0:0:0.img.LONGDATA";
        assert_eq!(extract_img_data(content), Some("LONGDATA".to_string()));
    }

    #[test]
    fn extract_img_data_before_paren() {
        let content = "replica.Lost.img.ABC123).speech.X";
        assert_eq!(extract_img_data(content), Some("ABC123".to_string()));
    }

    #[test]
    fn extract_img_data_missing() {
        let content = "replica.Lost.sd.0:0:0:0:0:0.n.Gible";
        assert_eq!(extract_img_data(content), None);
    }

    #[test]
    fn slice_before_chain_and_cast_no_markers_returns_full_body() {
        // No INNER_BODY_MARKERS at depth 0 → the full body is returned verbatim.
        // Pins the no-op path so a future change can't silently truncate
        // modifier bodies that have no chain or ability block.
        let body = "Alpha.hp.5.sd.0:0:0:0:0:0.n.Mew";
        assert_eq!(slice_before_chain_and_cast(body), body);
    }

    #[test]
    fn slice_before_chain_and_cast_skips_nested_markers() {
        // `.abilitydata.(a.i.b)`: `.i.` is at depth 1 (inside the ability
        // parens), `.abilitydata.` is at depth 0. The slice must end at
        // `.abilitydata.`, not at the nested `.i.`. Pins the depth-aware scan.
        let body = "Alpha.abilitydata.(a.i.b)";
        let pos = body.find(".abilitydata.").unwrap();
        assert_eq!(slice_before_chain_and_cast(body), &body[..pos]);
    }

    #[test]
    fn slice_before_chain_and_cast_returns_earliest_of_three_markers() {
        // Of `.i.` / `.sticker.` / `.abilitydata.`, the earliest depth-0
        // occurrence wins — the slice cuts at whichever appears first.
        let body = "Alpha.sticker.Foo.i.hat.pad.abilitydata.(x)";
        let sticker_pos = body.find(".sticker.").unwrap();
        assert_eq!(slice_before_chain_and_cast(body), &body[..sticker_pos]);
    }

    #[test]
    fn extract_color_depth_aware_skips_parens() {
        // `depth_aware = true` must skip matches inside parentheses.
        // First depth-0 `.col.` is `.col.b`; the `.col.c` inside
        // `(...)` is at depth 1 and must be ignored.
        let content = "a.col.b.(.col.c)";
        assert_eq!(extract_color(content, true), Some('b'));
        // Without depth-awareness, the legacy first-match behavior is
        // preserved: both scans walk into `.col.b` first anyway, but the
        // flag is observable on shapes where the in-paren match is the
        // FIRST match textually.
        let content2 = "(.col.c).col.b";
        assert_eq!(
            extract_color(content2, false),
            Some('c'),
            "non-depth-aware must pick the first match regardless of paren depth",
        );
        assert_eq!(
            extract_color(content2, true),
            Some('b'),
            "depth-aware must skip the depth-1 match and pick the depth-0 one",
        );
    }

    #[test]
    fn extract_img_data_depth_aware() {
        // .img. inside nested parens should NOT be found (find_last_at_depth0 skips them)
        let content = ".abilitydata.(Fey.sd.34-1:0.img.spark.n.Psychic).img.HERO_SPRITE";
        assert_eq!(extract_img_data(content), Some("HERO_SPRITE".to_string()));
    }

}

/// Extract facade values from a modifier chain string.
pub fn extract_facades_from_chain(chain: &str) -> Vec<String> {
    let mut facades = Vec::new();
    for marker in &["#facade.", ".facade."] {
        let mut search_from = 0;
        while let Some(pos) = chain[search_from..].find(marker) {
            let val_start = search_from + pos + marker.len();
            let remaining = &chain[val_start..];
            let val_end = remaining
                .find(".i.")
                .or_else(|| remaining.find(".k."))
                .or_else(|| remaining.find('#'))
                .or_else(|| remaining.find(".sticker."))
                .unwrap_or(remaining.len());
            let val = &remaining[..val_end];
            if !val.is_empty() {
                facades.push(val.to_string());
            }
            search_from = val_start + val_end;
        }
    }
    facades
}
