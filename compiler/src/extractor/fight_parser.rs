//! Shared fight unit parsing — extracts FightUnit structs from textmod fight content.
//!
//! Used by boss_parser and (in future) phase parsers for any fight-containing context.

use crate::ir::FightUnit;
use crate::util;

/// Parse fight units from flat (non-paren-wrapped) content.
/// Units are +separated at depth 0.
pub fn parse_fight_units(content: &str) -> Vec<FightUnit> {
    let unit_strs = util::split_at_depth0(content, '+');
    unit_strs.iter().filter_map(|s| parse_fight_unit(s)).collect()
}

/// Parse a single fight unit from a +separated block.
pub fn parse_fight_unit(unit_str: &str) -> Option<FightUnit> {
    // Only strip leading whitespace; trailing whitespace may be a meaningful
    // part of the last property value (e.g., `.n.NAME\n` where `\n` is part
    // of the name, not a separator artifact).
    let trimmed = unit_str.trim_start();
    if trimmed.trim().is_empty() {
        return None;
    }

    // Unwrap outer parens ONLY if the entire string is one balanced paren group.
    // `(Wolf.n.Seviper).i.X` is NOT fully wrapped (ends after props, not `)`).
    // Track whether we performed the unwrap — this tells us the source used
    // `(unit)` shape and the emitter must re-wrap on output.
    let mut outer_paren_wrapped = false;
    let unwrapped = {
        let mut s = trimmed;
        while s.starts_with('(') && s.ends_with(')') {
            // Verify the first `(` matches the last `)` — otherwise unwrap is wrong.
            if let Some(close) = util::find_matching_close_paren(s, 0) {
                if close == s.len() - 1 {
                    s = &s[1..s.len()-1];
                    outer_paren_wrapped = true;
                    continue;
                }
            }
            break;
        }
        s
    };

    // Detect `(Template.n.Name).rest` or `(Template.((nested))).rest` head-paren.
    let (head_template, head_name, body_content) = extract_head_paren(unwrapped);
    let head_paren_flag = head_template.is_some();
    // `head_nested_in_paren` is true when the head paren wraps template+nested
    // (Case B). The emitter then places the nested group inside the head parens.
    let mut head_nested_in_paren = false;

    let template = if let Some(t) = head_template {
        t
    } else {
        let content = unwrapped.trim_start_matches('(');
        let content = content.strip_prefix("replica.").unwrap_or(content);
        // Multi-segment template: read segments separated by `.` until we hit
        // a known property marker. Handles `rmon.ded`, `egg`, `Alpha`, etc.
        extract_template_name(content)
    };

    // Detect nested fight units: template.((child1+child2)).properties
    // The `((` immediately after the template's `.` signals a nested group.
    // When the head-paren wraps template+nested (Case B), the nested group
    // lives inside the head paren — scan `unwrapped` normally; the resulting
    // `nested_props` string is what sits after the close of the head paren.
    let (nested_units, nested_props, nested_single_paren) =
        extract_nested_and_props_with_style(unwrapped, &template);
    if head_paren_flag && head_name.is_none() && nested_units.is_some() {
        head_nested_in_paren = true;
    }

    // Choose the prop source: if head paren had a name, properties come from body_content.
    // Otherwise if nested group was found, use nested_props. Otherwise use unwrapped.
    let prop_source = nested_props.as_deref()
        .or(body_content.as_deref())
        .unwrap_or(unwrapped);

    let name = head_name.clone()
        .or_else(|| util::extract_last_n_name(prop_source))
        .unwrap_or_else(|| template.clone());
    let hp = util::extract_hp(prop_source, true);
    let sd = util::extract_sd(prop_source, true).map(|s| crate::ir::DiceFaces::parse(&s));
    let img_data = extract_fight_unit_img(prop_source);
    let sprite = img_data.map(|img| {
        crate::authoring::SpriteId::lookup(&name)
            .cloned()
            .unwrap_or_else(|| crate::authoring::SpriteId::owned(name.clone(), img))
    });
    let doc = util::extract_simple_prop(prop_source, ".doc.");
    let template_override = extract_real_template_override(prop_source);
    let modifier_chain = util::extract_modifier_chain(prop_source)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    // Standalone `.k.KEYWORD` segments at depth 0 AFTER a `.t.X` override are
    // unit-level keywords (structured — each is a single bare keyword word).
    // Example: `...img.X.t.jinx.allitem.k.wither` → one entry, `"wither"`.
    let post_override_keywords = extract_post_override_keywords(prop_source);
    let part = util::extract_simple_prop(prop_source, ".part.")
        .and_then(|v| v.parse::<u16>().ok());
    // When nested is emitted inside the head paren (Case B), omit `Nested`
    // from the body order — the head paren handles it.
    let nested_in_body = nested_units.is_some() && !head_nested_in_paren;
    let body_order = compute_body_order(prop_source, nested_in_body);

    Some(FightUnit {
        template,
        name,
        hp,
        sd,
        sprite,
        template_override,
        doc,
        modifier_chain,
        color: None,
        hsv: None,
        nested_units,
        nested_single_paren,
        head_paren: head_paren_flag,
        outer_paren: outer_paren_wrapped,
        part,
        post_override_keywords,
        body_order,
    })
}

/// Read a (possibly multi-segment) template name like `rmon.ded` or `Alpha`.
/// Reads segments separated by `.` until we hit a known property marker or
/// non-template character.
fn extract_template_name(content: &str) -> String {
    let mut out = String::new();
    let bytes = content.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '.' {
            // Check if this `.` starts a known property marker
            let markers = [
                ".col.", ".tier.", ".hp.", ".sd.", ".img.", ".hsv.",
                ".doc.", ".speech.", ".n.", ".i.", ".k.", ".facade.", ".sticker.",
                ".mn.", ".part.", ".bal.", ".t.", ".abilitydata.", ".triggerhpdata.",
                ".hue.",
            ];
            if markers.iter().any(|m| content[i..].starts_with(m)) {
                break;
            }
            // Next char must be alphanumeric to continue the template name.
            // `egg.(` → the `.` begins a nested paren group, stop before it.
            let next = bytes.get(i + 1).copied().unwrap_or(0);
            if !(next.is_ascii_alphanumeric()) {
                break;
            }
            // Unknown `.segment` — include it as part of template name
            out.push('.');
            i += 1;
            continue;
        }
        if "&@()+\n".contains(c) {
            break;
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Extract a leading paren group that wraps the head of a unit, returning
/// `(template, optional_name, rest-after-close)`.
///
/// Recognised shapes:
///   `(Template.n.Name).rest`        — classic head-paren, name inside parens
///   `(Template.((nested))).rest`    — template + nested fight group, name in rest
///
/// Returns (None, None, None) when no leading paren or no matching close.
fn extract_head_paren(content: &str) -> (Option<String>, Option<String>, Option<String>) {
    if !content.starts_with('(') {
        return (None, None, None);
    }
    let close = match util::find_matching_close_paren(content, 0) {
        Some(c) => c,
        None => return (None, None, None),
    };
    // Must have content after the close to be a head-paren (vs fully wrapped unit)
    if close + 1 >= content.len() {
        return (None, None, None);
    }
    let inner = &content[1..close];
    let template = extract_template_name(inner);
    if template.is_empty() {
        return (None, None, None);
    }
    let rest = content[close + 1..].to_string();
    // Case A: inner starts with `Template.n.Name`
    if let Some(n_pos) = util::find_at_depth0(inner, ".n.") {
        if n_pos == template.len() {
            let name_start = n_pos + ".n.".len();
            let name = inner[name_start..].to_string();
            return (Some(template), Some(name), Some(rest));
        }
    }
    // Case B: inner has a nested group — head wraps template + nested.
    // Matches both `.((nested))` (double-paren) and `.(nested)` (single-paren).
    // The caller fetches name from the rest string via `extract_last_n_name`.
    let after_template = &inner[template.len()..];
    if after_template.starts_with(".(") {
        return (Some(template), None, Some(rest));
    }
    (None, None, None)
}

/// Walk the content recording property markers in source order, producing a
/// `body_order` vector that drives deterministic re-emission. Includes every
/// field the emitter will produce so the output matches source structure.
fn compute_body_order(
    content: &str,
    has_nested: bool,
) -> Vec<crate::ir::FightUnitMarker> {
    use crate::ir::FightUnitMarker as M;
    let bytes = content.as_bytes();
    let mut order: Vec<(usize, M)> = Vec::new();
    let mut depth: i32 = 0;
    let mut chain_seg_idx: usize = 0;
    let non_chain_markers: &[&str] = &[
        ".col.", ".tier.", ".hp.", ".sd.", ".img.", ".hsv.", ".abilitydata.",
        ".triggerhpdata.", ".doc.", ".hue.", ".speech.", ".n.", ".mn.",
        ".part.", ".bal.", ".t.",
    ];
    // Nested group appears immediately at position 0 when template has `.((`.
    if has_nested {
        order.push((0, M::Nested));
    }
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            if i + 3 <= bytes.len() && &content[i..i + 3] == ".i." {
                order.push((i, M::Chain(chain_seg_idx)));
                chain_seg_idx += 1;
                i += 3;
                continue;
            }
            if i + 9 <= bytes.len() && &content[i..i + 9] == ".sticker." {
                order.push((i, M::Chain(chain_seg_idx)));
                chain_seg_idx += 1;
                i += 9;
                continue;
            }
            // Non-chain marker ends chain
            let mut matched_non_chain = false;
            for m in non_chain_markers {
                if i + m.len() <= bytes.len() && &content[i..i + m.len()] == *m {
                    let marker = match *m {
                        ".col." => Some(M::Col),
                        ".hp." => Some(M::Hp),
                        ".sd." => Some(M::Sd),
                        ".img." => Some(M::Img),
                        ".hsv." => Some(M::Hsv),
                        ".doc." => Some(M::Doc),
                        ".n." => Some(M::Name),
                        ".part." => Some(M::Part),
                        ".t." => {
                            // Suppress `.t.` inside `.i.t.X` — those were handled via chain
                            if i >= 2 {
                                let check = &content[i - 2..i + 1];
                                if check == ".i." || check == ".k." {
                                    None
                                } else {
                                    Some(M::TemplateOverride)
                                }
                            } else {
                                Some(M::TemplateOverride)
                            }
                        }
                        _ => None,
                    };
                    if let Some(mk) = marker {
                        // Dedup: only first occurrence per marker, except Chain/Keyword
                        let already = order.iter().any(|(_, existing)| std::mem::discriminant(existing) == std::mem::discriminant(&mk));
                        if !already {
                            order.push((i, mk));
                        }
                    }
                    i += m.len();
                    matched_non_chain = true;
                    break;
                }
            }
            if matched_non_chain { continue; }
        }
        i += 1;
    }
    order.sort_by_key(|(pos, _)| *pos);
    order.into_iter().map(|(_, m)| m).collect()
}

/// Scan for `.k.KEYWORD` segments at depth 0 that appear AFTER a top-level
/// `.t.X` override. These are fight-unit-level keyword segments (parsed as
/// `ChainSegment::Keyword`), not item sub-entries or value boundaries.
fn extract_post_override_keywords(content: &str) -> Vec<String> {
    let t_pos = match find_top_level_override_start(content) {
        Some(p) => p,
        None => return Vec::new(),
    };
    // Skip past the `.t.VALUE` to find keywords that follow it.
    let after_t = t_pos + ".t.".len();
    let remaining = &content[after_t..];
    let val_end = util::find_next_prop_boundary(remaining);
    let mut scan_from = after_t + val_end;

    let bytes = content.as_bytes();
    let mut depth: i32 = 0;
    // Initialize depth by pre-walking from 0 to scan_from so we respect parens.
    for &b in &bytes[..scan_from] {
        match b {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
    }
    // Track chain state so `.k.X` inside an `.i.` or `.sticker.` segment is
    // treated as a sub-entry, not a unit-level keyword. The chain is terminated
    // by any known non-chain property marker at depth 0.
    let non_chain_markers: &[&str] = &[
        ".col.", ".tier.", ".hp.", ".sd.", ".img.", ".hsv.", ".abilitydata.",
        ".triggerhpdata.", ".doc.", ".hue.", ".speech.", ".n.", ".mn.",
        ".part.", ".bal.", ".t.",
    ];
    let mut in_chain = false;
    let mut out: Vec<String> = Vec::new();
    while scan_from < bytes.len() {
        match bytes[scan_from] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            // Entering a chain segment
            if scan_from + 3 <= bytes.len() && &content[scan_from..scan_from + 3] == ".i." {
                in_chain = true;
                scan_from += 3;
                continue;
            }
            if scan_from + 9 <= bytes.len() && &content[scan_from..scan_from + 9] == ".sticker." {
                in_chain = true;
                scan_from += 9;
                continue;
            }
            // Non-chain marker terminates any open chain
            let mut hit_non_chain = false;
            for m in non_chain_markers {
                if scan_from + m.len() <= bytes.len()
                    && &content[scan_from..scan_from + m.len()] == *m
                {
                    in_chain = false;
                    hit_non_chain = true;
                    break;
                }
            }
            // Standalone `.k.X` — only record when NOT inside a chain segment
            if !in_chain
                && !hit_non_chain
                && scan_from + 3 <= bytes.len()
                && &content[scan_from..scan_from + 3] == ".k."
            {
                let kw_start = scan_from + 3;
                let mut k = kw_start;
                while k < bytes.len() {
                    let b = bytes[k];
                    if b == b'.' || b == b'&' || b == b'@' || b == b')' || b == b'+' || b == b'#' {
                        break;
                    }
                    k += 1;
                }
                let kw = &content[kw_start..k];
                if !kw.is_empty() {
                    out.push(kw.to_string());
                }
                scan_from = k;
                continue;
            }
        }
        scan_from += 1;
    }
    out
}

/// Locate the position of the first top-level `.t.X` template override —
/// one at depth 0 that is NOT preceded by `.i.`/`.k.` (which would make it
/// an item/keyword sub-entry).
fn find_top_level_override_start(content: &str) -> Option<usize> {
    let bytes = content.as_bytes();
    let mut depth: i32 = 0;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0 && i + 3 <= bytes.len() && &content[i..i + 3] == ".t." {
            let preceded_by_chain = i >= 2 && {
                let pre = &content[i - 2..i + 1];
                pre == ".i." || pre == ".k."
            };
            if !preceded_by_chain {
                return Some(i);
            }
        }
    }
    None
}

/// Extract a REAL template override `.t.X` — one that's NOT inside an `.i.t.X` chain item.
/// Returns the override value, or None.
fn extract_real_template_override(content: &str) -> Option<String> {
    let bytes = content.as_bytes();
    let mut depth: i32 = 0;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0 && content[i..].starts_with(".t.") {
            // Skip if preceded by `.i.` or `.k.` — in `.i.t.X` the two `.`s
            // overlap: positions [i-2, i-1, i] spell `.i.` while [i, i+1, i+2]
            // spell `.t.`. Check the 3 chars ENDING at `i` (inclusive).
            if i >= 2 {
                let check = &content[i - 2..i + 1];
                if check == ".i." || check == ".k." {
                    continue;
                }
            }
            // Extract value — stops at next prop boundary
            let val_start = i + ".t.".len();
            let remaining = &content[val_start..];
            let val_end = util::find_next_prop_boundary(remaining);
            let val = &remaining[..val_end];
            if !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}

/// Detect and extract nested fight units from `template.((child1+child2))...props`.
///
/// Returns `(nested_units, props_content)`:
/// - `nested_units`: parsed child FightUnits if `((..))` was found, else None.
/// - `props_content`: the property content AFTER the nested group (for extracting
///   name, hp, img, etc. from the parent unit), or None if no nesting.
fn extract_nested_and_props_with_style(
    content: &str,
    template: &str,
) -> (Option<Vec<FightUnit>>, Option<String>, bool) {
    let (nested, props) = extract_nested_and_props(content, template);
    let single = nested.is_some() && {
        // Determine style from source: `.((` → double, `.(` → single
        let stripped = content.trim_start_matches('(');
        let after_template = if stripped.starts_with(template) {
            &stripped[template.len()..]
        } else { "" };
        after_template.starts_with(".(") && !after_template.starts_with(".((")
    };
    (nested, props, single)
}

fn extract_nested_and_props(content: &str, template: &str) -> (Option<Vec<FightUnit>>, Option<String>) {
    // Look for a paren group after `template.` — either `.((child+child))` for
    // multi-unit group or `.(child)` for a single nested unit transform.
    let stripped = content.trim_start_matches('(');
    let after_template = if stripped.starts_with(template) {
        &stripped[template.len()..]
    } else {
        return (None, None);
    };

    let is_double = after_template.starts_with(".((");
    let is_single = after_template.starts_with(".(") && !is_double;

    if !(is_double || is_single) {
        return (None, None);
    }

    // Position of the first `(` (in the original `content`, accounting for any
    // leading `(` that was skipped by `trim_start_matches`).
    let prefix = if is_double { ".((" } else { ".(" };
    let nested_start_in_content = content.find(prefix).unwrap() + 1; // position of first `(`
    let close = util::find_matching_close_paren(content, nested_start_in_content);
    let close = match close {
        Some(c) => c,
        None => return (None, None),
    };

    let (inner_start, inner_end) = if is_double {
        // `((child...))` — skip outer `(`, inner ends before outer `)`
        (nested_start_in_content + 1, close)
    } else {
        // `.(child)` — everything between the single parens
        (nested_start_in_content + 1, close)
    };
    let inner = &content[inner_start..inner_end];

    let children: Vec<FightUnit> = if is_double {
        // Inner may be wrapped: `(child1+child2)` — unwrap; or flat `child1+child2`
        if inner.starts_with('(') {
            if let Some(inner_close) = util::find_matching_close_paren(inner, 0) {
                let child_content = &inner[1..inner_close];
                let unit_strs = util::split_at_depth0(child_content, '+');
                unit_strs.iter().filter_map(|s| parse_fight_unit(s)).collect()
            } else {
                vec![]
            }
        } else {
            parse_fight_units(inner)
        }
    } else {
        // Single-paren: one nested unit
        parse_fight_unit(inner).map(|u| vec![u]).unwrap_or_default()
    };

    if children.is_empty() {
        return (None, None);
    }

    // Properties come after the close paren `)` of the nested group.
    let props = &content[close + 1..];
    let props = props.strip_prefix(')').unwrap_or(props);

    (Some(children), Some(props.to_string()))
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
