use crate::error::CompilerError;

/// Split raw textmod text into individual modifier strings.
///
/// Strategy: split on commas at parenthesis depth 0.
/// This handles all three known formats uniformly:
/// - pansaer: single line, comma-separated (76 modifiers)
/// - punpuns: alternating content/comma lines (75 modifiers)
/// - sliceymon: one comma-terminated modifier per line, blank spacers (92 modifiers)
pub fn split_modifiers(text: &str) -> Result<Vec<String>, CompilerError> {
    let mut modifiers = Vec::new();
    let mut depth: i32 = 0;
    let mut start = 0;

    for (pos, byte) in text.bytes().enumerate() {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth < 0 {
                    return Err(CompilerError::SplitError {
                        raw_position: pos,
                        message: format!(
                            "Unmatched closing parenthesis at position {} (depth went to {})",
                            pos, depth
                        ),
                    });
                }
            }
            b',' if depth == 0 => {
                let trimmed = text[start..pos].trim();
                if !trimmed.is_empty() {
                    modifiers.push(trimmed.to_string());
                }
                start = pos + 1;
            }
            _ => {}
        }
    }

    // Handle last modifier (may not be comma-terminated)
    let trimmed = text[start..].trim();
    if !trimmed.is_empty() {
        modifiers.push(trimmed.to_string());
    }

    Ok(modifiers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_simple_commas() {
        let result = split_modifiers("a,b,c").unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn split_respects_paren_depth() {
        let result = split_modifiers("a(b,c),d").unwrap();
        assert_eq!(result, vec!["a(b,c)", "d"]);
    }

    #[test]
    fn split_trims_whitespace_and_newlines() {
        let result = split_modifiers("  a  ,\n  b  ,\n  c  ").unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn split_filters_empty() {
        let result = split_modifiers("a,,b,,,c,").unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn split_nested_parens() {
        let result = split_modifiers("a((b,c),(d,e)),f").unwrap();
        assert_eq!(result, vec!["a((b,c),(d,e))", "f"]);
    }

    #[test]
    fn split_unmatched_close_paren_errors() {
        assert!(split_modifiers("a),b").is_err());
    }
}
