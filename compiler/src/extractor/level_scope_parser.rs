//! Parser and emitter for level scope prefixes.
//!
//! Level scopes are optional numeric prefixes on modifier strings that restrict
//! when a modifier is active based on the current floor/level.
//!
//! Patterns:
//! - `5.`    → floor 5 only (start=5)
//! - `3-7.`  → floors 3 through 7 (start=3, end=7)
//! - `e2.`   → every 2 floors (interval=2)
//! - `e3.1.` → every 3 floors starting at floor 1 (interval=3, offset=1)
//!
//! WASM-safe: no std::fs, no unwrap().

use crate::ir::LevelScope;

/// Parse a level scope prefix from the beginning of `input`.
///
/// Returns `(Some(scope), remaining)` if a scope prefix was found,
/// or `(None, input)` if the input does not start with a scope prefix.
pub fn parse_level_scope(input: &str) -> (Option<LevelScope>, &str) {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return (None, input);
    }

    // Case 1: starts with 'e' followed by a digit → interval pattern
    if bytes[0] == b'e' {
        return parse_interval_scope(input);
    }

    // Case 2: starts with a digit → single floor or range
    if bytes[0].is_ascii_digit() {
        return parse_numeric_scope(input);
    }

    // No scope prefix
    (None, input)
}

/// Parse `eN.` or `eN.M.` interval patterns.
fn parse_interval_scope(input: &str) -> (Option<LevelScope>, &str) {
    let bytes = input.as_bytes();
    // Skip the 'e'
    let mut i = 1;

    // Read the interval number
    let interval_start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == interval_start {
        // 'e' not followed by digits — not a scope
        return (None, input);
    }
    let interval = match input[interval_start..i].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return (None, input),
    };

    // Expect a dot after the interval
    if i >= bytes.len() || bytes[i] != b'.' {
        return (None, input);
    }
    i += 1; // skip the dot

    // Check if there's an offset: another digit sequence followed by a dot
    if i < bytes.len() && bytes[i].is_ascii_digit() {
        let offset_start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        // Must be followed by a dot to be a valid offset
        if i < bytes.len() && bytes[i] == b'.' {
            let offset = match input[offset_start..i].parse::<u8>() {
                Ok(v) => v,
                Err(_) => return (None, input),
            };
            i += 1; // skip the dot after offset
            let scope = LevelScope {
                start: 0,
                end: None,
                interval: Some(interval),
                offset: Some(offset),
            };
            return (Some(scope), &input[i..]);
        }
        // Digits not followed by a dot — those digits are part of the remaining string.
        // Back up: the offset_start position digits are the remaining content.
        // The scope is just eN. with no offset.
        let scope = LevelScope {
            start: 0,
            end: None,
            interval: Some(interval),
            offset: None,
        };
        return (Some(scope), &input[offset_start..]);
    }

    // No offset — just eN.
    let scope = LevelScope {
        start: 0,
        end: None,
        interval: Some(interval),
        offset: None,
    };
    (Some(scope), &input[i..])
}

/// Parse `N.` or `N-M.` numeric scope patterns.
fn parse_numeric_scope(input: &str) -> (Option<LevelScope>, &str) {
    let bytes = input.as_bytes();
    let mut i = 0;

    // Read the first number
    let start_begin = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == start_begin {
        return (None, input);
    }
    let start_val = match input[start_begin..i].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return (None, input),
    };

    if i >= bytes.len() {
        return (None, input);
    }

    // Check for range: N-M.
    if bytes[i] == b'-' {
        i += 1; // skip '-'
        let end_begin = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == end_begin {
            // '-' not followed by digits — not a valid scope
            return (None, input);
        }
        let end_val = match input[end_begin..i].parse::<u8>() {
            Ok(v) => v,
            Err(_) => return (None, input),
        };
        // Must be followed by a dot
        if i >= bytes.len() || bytes[i] != b'.' {
            return (None, input);
        }
        i += 1; // skip the dot
        let scope = LevelScope {
            start: start_val,
            end: Some(end_val),
            interval: None,
            offset: None,
        };
        return (Some(scope), &input[i..]);
    }

    // Single floor: N.
    if bytes[i] == b'.' {
        // Disambiguate: we need the character after the dot.
        // If the next char is a digit, this could be ambiguous (e.g. "5.4" is it scope "5" + "4"
        // or just data?). The rule: the prefix ends at the first '.' that transitions from
        // digits to non-digit content. So if next char after '.' is a digit, that's NOT a scope
        // separator — it's part of a dotted numeric sequence (like a version or ID).
        // BUT: level scopes like "5.ph.4" are valid — after "5." comes "ph.4" which starts with
        // a non-digit. So check if the char after '.' is NOT a digit.
        let after_dot = i + 1;
        if after_dot < bytes.len() && bytes[after_dot].is_ascii_digit() {
            // Next char is a digit — this dot is NOT a scope separator
            return (None, input);
        }
        i += 1; // skip the dot
        let scope = LevelScope {
            start: start_val,
            end: None,
            interval: None,
            offset: None,
        };
        return (Some(scope), &input[i..]);
    }

    // Digit(s) not followed by '.' or '-' — not a scope
    (None, input)
}

/// Emit a `LevelScope` back to textmod format (no trailing dot).
///
/// Examples:
/// - `LevelScope { start: 5, .. }` → `"5"`
/// - `LevelScope { start: 3, end: Some(7), .. }` → `"3-7"`
/// - `LevelScope { interval: Some(2), .. }` → `"e2"`
/// - `LevelScope { interval: Some(3), offset: Some(1), .. }` → `"e3.1"`
pub fn emit_level_scope(scope: &LevelScope) -> String {
    if let Some(interval) = scope.interval {
        if let Some(offset) = scope.offset {
            format!("e{}.{}", interval, offset)
        } else {
            format!("e{}", interval)
        }
    } else if let Some(end) = scope.end {
        format!("{}-{}", scope.start, end)
    } else {
        format!("{}", scope.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_floor() {
        let (scope, remaining) = parse_level_scope("5.ph.4");
        let scope = scope.expect("should parse single floor");
        assert_eq!(scope.start, 5);
        assert_eq!(scope.end, None);
        assert_eq!(scope.interval, None);
        assert_eq!(scope.offset, None);
        assert_eq!(remaining, "ph.4");
    }

    #[test]
    fn test_parse_floor_range() {
        let (scope, remaining) = parse_level_scope("3-7.ph.!");
        let scope = scope.expect("should parse floor range");
        assert_eq!(scope.start, 3);
        assert_eq!(scope.end, Some(7));
        assert_eq!(scope.interval, None);
        assert_eq!(scope.offset, None);
        assert_eq!(remaining, "ph.!");
    }

    #[test]
    fn test_parse_every_n() {
        let (scope, remaining) = parse_level_scope("e2.ph.4");
        let scope = scope.expect("should parse every-N");
        assert_eq!(scope.start, 0);
        assert_eq!(scope.end, None);
        assert_eq!(scope.interval, Some(2));
        assert_eq!(scope.offset, None);
        assert_eq!(remaining, "ph.4");
    }

    #[test]
    fn test_parse_every_n_offset() {
        let (scope, remaining) = parse_level_scope("e3.1.ph.4");
        let scope = scope.expect("should parse every-N with offset");
        assert_eq!(scope.start, 0);
        assert_eq!(scope.end, None);
        assert_eq!(scope.interval, Some(3));
        assert_eq!(scope.offset, Some(1));
        assert_eq!(remaining, "ph.4");
    }

    #[test]
    fn test_parse_no_scope() {
        let (scope, remaining) = parse_level_scope("ph.4");
        assert!(scope.is_none());
        assert_eq!(remaining, "ph.4");
    }

    #[test]
    fn test_level_scope_roundtrip() {
        // Single floor
        let scope1 = LevelScope {
            start: 5,
            end: None,
            interval: None,
            offset: None,
        };
        let emitted1 = emit_level_scope(&scope1);
        let input1 = format!("{}.rest", emitted1);
        let (parsed1, rem1) = parse_level_scope(&input1);
        assert_eq!(parsed1, Some(scope1));
        assert_eq!(rem1, "rest");

        // Floor range
        let scope2 = LevelScope {
            start: 3,
            end: Some(7),
            interval: None,
            offset: None,
        };
        let emitted2 = emit_level_scope(&scope2);
        let input2 = format!("{}.rest", emitted2);
        let (parsed2, rem2) = parse_level_scope(&input2);
        assert_eq!(parsed2, Some(scope2));
        assert_eq!(rem2, "rest");

        // Every-N
        let scope3 = LevelScope {
            start: 0,
            end: None,
            interval: Some(2),
            offset: None,
        };
        let emitted3 = emit_level_scope(&scope3);
        let input3 = format!("{}.rest", emitted3);
        let (parsed3, rem3) = parse_level_scope(&input3);
        assert_eq!(parsed3, Some(scope3));
        assert_eq!(rem3, "rest");

        // Every-N with offset
        let scope4 = LevelScope {
            start: 0,
            end: None,
            interval: Some(3),
            offset: Some(1),
        };
        let emitted4 = emit_level_scope(&scope4);
        let input4 = format!("{}.rest", emitted4);
        let (parsed4, rem4) = parse_level_scope(&input4);
        assert_eq!(parsed4, Some(scope4));
        assert_eq!(rem4, "rest");
    }
}
