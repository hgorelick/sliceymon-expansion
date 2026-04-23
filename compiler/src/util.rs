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
/// chain / cast / ability marker: `.i.`, `.sticker.`, `.cast.`,
/// `.abilitydata.`. When none exist at depth 0, returns the full `body` slice.
///
/// Replica parsers feed the result to scalar extractors for `hp` / `color` /
/// `sd` / `img` so chain sub-entries (`.i.` / `.sticker.` — free-form
/// `.sidesc.` / `.enchant.` text) and cast / ability effect bodies cannot
/// leak interior `.hp.N` / `.col.X` / `.sd.FACES` / `.img.DATA` substrings
/// into top-level fields. Emission places every scalar field before the
/// chain / cast region, so the prefix is sufficient.
///
/// `.abilitydata.` is included as a fourth marker because the Capture
/// parsers read the outer cast region under that name (the Capture emitter
/// writes `.cast.` inside the replica parens, so the outer cast-like region
/// parsed as `.abilitydata.` is at depth 0 in the raw modifier). This is a
/// strict superset of the plan's three markers — no callsite becomes less
/// restrictive, and the Capture-cast leak class is closed.
pub fn slice_before_chain_and_cast(body: &str) -> &str {
    let mut earliest: Option<usize> = None;
    for marker in [".i.", ".sticker.", ".cast.", ".abilitydata."] {
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

/// Return the content of the innermost `(…)` that wraps the first `replica.`
/// token in `modifier`, stripped of its outer parens. For the Capture shapes
/// `itempool.((hat.replica.TEMPLATE…))…` and
/// `itempool.((hat.(replica.TEMPLATE…)))…`, this yields the body inside which
/// scalars and the chain live at body-relative depth 0 — the shape
/// `slice_before_chain_and_cast` is designed to scan.
///
/// Returns `None` when there is no `replica.` token or no paren wraps it; the
/// caller is expected to fall back to the raw modifier in that case.
///
/// Without this scoping, `slice_before_chain_and_cast` applied to the raw
/// modifier would see the Capture's chain at raw paren depth ≥ 2 and its
/// `find_at_depth0`-based scan would skip it, leaving chain-interior
/// `.hp.` / `.col.` substrings free to leak into top-level scalars via
/// non-depth-aware `content.find(…)` — the §F10 leak class Chunk 9 set out
/// to close for replica parsers.
pub fn replica_inner_body(modifier: &str) -> Option<&str> {
    let r_pos = modifier.find("replica.")?;
    let open = modifier[..r_pos].rfind('(')?;
    let close = find_matching_close_paren(modifier, open)?;
    Some(&modifier[open + 1..close])
}

/// Extract `replica.TEMPLATE` — the template name after `replica.`.
pub fn extract_template(content: &str) -> Option<String> {
    let marker = "replica.";
    let pos = content.find(marker)?;
    let start = pos + marker.len();
    let remaining = &content[start..];
    let end = remaining.find('.').unwrap_or(remaining.len());
    let name = &remaining[..end];
    if name.is_empty() { None } else { Some(name.to_string()) }
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
        // No `.i.` / `.sticker.` / `.cast.` anywhere → the full body is
        // returned verbatim. Pins the no-op path so a future change can't
        // silently truncate legendary bodies that have no chain or cast.
        let body = "Alpha.hp.5.sd.0:0:0:0:0:0.n.Mew";
        assert_eq!(slice_before_chain_and_cast(body), body);
    }

    #[test]
    fn slice_before_chain_and_cast_skips_nested_markers() {
        // `.cast.(a.i.b)`: `.i.` is at depth 1 (inside the cast parens),
        // `.cast.` is at depth 0. The slice must end at `.cast.`, not at
        // the nested `.i.`. Pins the depth-aware scan.
        let body = "Alpha.cast.(a.i.b)";
        let cast_pos = body.find(".cast.").unwrap();
        assert_eq!(slice_before_chain_and_cast(body), &body[..cast_pos]);
    }

    #[test]
    fn slice_before_chain_and_cast_returns_earliest_of_three_markers() {
        // Of `.i.` / `.sticker.` / `.cast.`, the earliest depth-0 occurrence
        // wins — the slice cuts at whichever appears first.
        let body = "Alpha.sticker.Foo.i.hat.pad.cast.(x)";
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

    #[test]
    fn replica_inner_body_simple_shape_strips_outer_parens() {
        // `((hat.replica.TEMPLATE…))` — inner `(` is the second char in `((`,
        // matching inner `)` is the first `)` in `))`. Body is everything
        // between, so the chain's `.i.` lives at body-relative depth 0.
        let m = "itempool.((hat.replica.Hat.sd.0:0:0:0:0:0.i.hat.statue#sidesc.foo.n.Pika)).n.Ball.mn.Pika";
        assert_eq!(
            replica_inner_body(m),
            Some("hat.replica.Hat.sd.0:0:0:0:0:0.i.hat.statue#sidesc.foo.n.Pika"),
        );
    }

    #[test]
    fn replica_inner_body_with_ability_shape_strips_to_innermost_replica() {
        // `((hat.(replica.TEMPLATE…)))` — the `(` immediately before
        // `replica.` wraps the scalar-bearing body. The surrounding
        // `hat.(…)` paren is outside it.
        let m = "itempool.((hat.(replica.Alpha.sd.0:0:0:0:0:0.n.Mew.cast.(X)))).n.Ball.mn.Mew";
        assert_eq!(
            replica_inner_body(m),
            Some("replica.Alpha.sd.0:0:0:0:0:0.n.Mew.cast.(X)"),
        );
    }

    #[test]
    fn replica_inner_body_no_replica_token_returns_none() {
        // Callers fall back to the raw modifier — this pins the None path.
        assert_eq!(replica_inner_body("item.Alpha.sd.0:0:0:0:0:0.n.Mew"), None);
    }

    #[test]
    fn replica_inner_body_replica_without_enclosing_paren_returns_none() {
        // `replica.X` appearing without a wrapping `(` — the caller must
        // fall back rather than extract a misleading slice.
        assert_eq!(replica_inner_body("replica.Alpha.sd.0:0:0:0:0:0"), None);
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
