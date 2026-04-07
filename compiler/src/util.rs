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
/// NAME ends at `@`, `&`, `,`, or end of string.
pub fn extract_mn_name(modifier: &str) -> Option<String> {
    let marker = ".mn.";
    let pos = modifier.rfind(marker)?;
    let start = pos + marker.len();
    let remaining = &modifier[start..];
    let end = remaining.find(['@', '&', ','])
        .unwrap_or(remaining.len());
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
pub fn extract_color(content: &str) -> Option<char> {
    let bytes = content.as_bytes();
    let marker = b".col.";
    for i in 0..bytes.len() {
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
    extract_color(modifier) == Some(target)
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

/// Extract a simple property value, ending at the next known property marker.
pub fn extract_simple_prop(content: &str, marker: &str) -> Option<String> {
    let pos = content.find(marker)?;
    let val_start = pos + marker.len();
    let remaining = &content[val_start..];
    let val_end = find_next_prop_boundary(remaining);
    let val = &remaining[..val_end];
    if val.is_empty() { None } else { Some(val.to_string()) }
}

/// Find the boundary of the next property marker in a string.
pub fn find_next_prop_boundary(remaining: &str) -> usize {
    let markers = [
        ".col.", ".tier.", ".hp.", ".sd.", ".img.", ".abilitydata.", ".triggerhpdata.",
        ".doc.", ".hue.", ".speech.", ".n.", ".i.", ".k.", ".facade.", ".sticker.",
        ".mn.", ".part.", ".bal.",
    ];
    let bytes = remaining.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] != b'.' {
            continue;
        }
        for marker in &markers {
            if i + marker.len() <= bytes.len() && &remaining[i..i + marker.len()] == *marker {
                return i;
            }
        }
    }
    remaining.len()
}

/// Extract the modifier chain (.i./.k./.facade./.sticker. sequences) from replica content.
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
            let is_chain = (i + 3 <= bytes.len() && &content[i..i + 3] == ".i.")
                || (i + 9 <= bytes.len() && &content[i..i + 9] == ".sticker.");

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
                        _ => {}
                    }
                    if d == 0 && bytes[j] == b'.' {
                        let is_non_chain = non_chain_markers.iter().any(|m| {
                            j + m.len() <= bytes.len() && &content[j..j + m.len()] == *m
                        });
                        if is_non_chain { break; }
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
