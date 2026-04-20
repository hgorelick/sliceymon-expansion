//! Parser and emitter for richtext strings.
//!
//! Richtext in Slice & Dice textmods uses square brackets for formatting tags:
//! `[orange]hello[cu]` applies orange color to "hello" then resets.
//!
//! This parser validates bracket balance and preserves the exact content.
//! Unknown tags (not in `constants::RICHTEXT_TAGS`) are accepted — we only
//! enforce structural correctness (balanced brackets).
//!
//! WASM-safe: no std::fs, no unwrap().

use crate::error::CompilerError;
use crate::ir::RichText;

/// Parse a richtext string, validating bracket balance.
///
/// Returns `Ok(RichText)` if brackets are balanced, or
/// `Err(CompilerError::PhaseParseError)` if they are not.
pub fn parse_richtext(input: &str) -> Result<RichText, CompilerError> {
    let mut depth: i32 = 0;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth < 0 {
                    return Err(CompilerError::PhaseParseError {
                        phase_code: None,
                        content: input.to_string(),
                        expected: "balanced brackets".to_string(),
                        found: format!(
                            "unexpected closing ']' at position {} (no matching '[')",
                            i
                        ),
                    });
                }
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err(CompilerError::PhaseParseError {
            phase_code: None,
            content: input.to_string(),
            expected: "balanced brackets".to_string(),
            found: format!(
                "{} unclosed '[' bracket(s) at end of string",
                depth
            ),
        });
    }

    Ok(RichText::new(input))
}

/// Emit a richtext back to its string representation.
///
/// Richtext preserves exact content, so this is a simple extraction.
pub fn emit_richtext(rt: &RichText) -> String {
    rt.as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_richtext_balanced() {
        let result = parse_richtext("[orange]hello[cu]");
        assert!(result.is_ok());
        let rt = result.expect("should be Ok");
        assert_eq!(rt.as_str(), "[orange]hello[cu]");
    }

    #[test]
    fn test_richtext_unbalanced() {
        // Missing closing bracket — "[orange" has depth 1 at end
        let result = parse_richtext("[orangehello");
        assert!(result.is_err());
        match result {
            Err(CompilerError::PhaseParseError { expected, .. }) => {
                assert_eq!(expected, "balanced brackets");
            }
            _ => panic!("expected PhaseParseError"),
        }

        // Extra closing bracket
        let result2 = parse_richtext("hello]world");
        assert!(result2.is_err());
    }

    #[test]
    fn test_richtext_no_tags() {
        let result = parse_richtext("plain text");
        assert!(result.is_ok());
        let rt = result.expect("should be Ok");
        assert_eq!(rt.as_str(), "plain text");
    }

    #[test]
    fn test_richtext_roundtrip() {
        let input = "[orange]hello [blue]world[cu][cu]";
        let rt = parse_richtext(input).expect("should parse");
        let emitted = emit_richtext(&rt);
        assert_eq!(emitted, input);
    }
}
