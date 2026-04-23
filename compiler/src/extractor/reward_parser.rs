//! Parse reward tag strings into typed RewardTag IR.
//!
//! Reward tags appear in SCPhase/ChoicePhase rewards. Each tag starts with a
//! single-letter prefix that determines the tag type:
//!   m(content)  → Modifier
//!   i(content)  → Item
//!   l(content)  → Levelup
//!   g(content)  → Hero
//!   r...        → Random
//!   q...        → RandomRange
//!   o...        → Or
//!   e(content)  → Enu
//!   v...        → Value
//!   p...        → Replace
//!   s           → Skip
//!
//! The content field stores the FULL original string for round-trip emission.
//! The tag_type provides typed access for semantic operations.

use crate::error::CompilerError;
use crate::ir::{RewardTag, RewardTagType};

/// Parse a reward tag string into a RewardTag.
///
/// The input is the raw tag string (e.g., `"m(skip&hidden)"`, `"r1~3~i"`, `"s"`).
/// Returns a RewardTag with tag_type set from the first character and content
/// storing the full original string for lossless round-trip.
pub fn parse_reward_tag(s: &str) -> Result<RewardTag, CompilerError> {
    let Some(first) = s.chars().next() else {
        return Err(CompilerError::reward_parse(
            s.to_string(),
            "non-empty reward tag string",
            "empty string",
        )
        .with_field_path("reward_tag.raw")
        .with_suggestion("tag must start with one of: m i l g r q o e v p s"));
    };

    let tag_type = match first {
        'm' => RewardTagType::Modifier,
        'i' => RewardTagType::Item,
        'l' => RewardTagType::Levelup,
        'g' => RewardTagType::Hero,
        'r' => RewardTagType::Random,
        'q' => RewardTagType::RandomRange,
        'o' => RewardTagType::Or,
        'e' => RewardTagType::Enu,
        'v' => RewardTagType::Value,
        'p' => RewardTagType::Replace,
        's' => RewardTagType::Skip,
        _ => {
            return Err(CompilerError::reward_parse(
                s.to_string(),
                "reward tag letter (m, i, l, g, r, q, o, e, v, p, s)",
                format!("'{}'", first),
            ));
        }
    };

    Ok(RewardTag {
        tag_type,
        content: s.to_string(),
    })
}

/// Emit a RewardTag back to its original string form.
///
/// Since the content field stores the full original string, this is a
/// simple clone — guaranteeing lossless round-trip.
pub fn emit_reward_tag(tag: &RewardTag) -> String {
    tag.content.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_modifier_tag() {
        let tag = parse_reward_tag("m(skip&hidden)").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Modifier);
        assert_eq!(tag.content, "m(skip&hidden)");
    }

    #[test]
    fn test_parse_item_tag() {
        let tag = parse_reward_tag("i(Sword)").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Item);
        assert_eq!(tag.content, "i(Sword)");
    }

    #[test]
    fn test_parse_levelup_tag() {
        let tag = parse_reward_tag("l(levelup)").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Levelup);
        assert_eq!(tag.content, "l(levelup)");
    }

    #[test]
    fn test_parse_hero_tag() {
        let tag = parse_reward_tag("g(hero)").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Hero);
        assert_eq!(tag.content, "g(hero)");
    }

    #[test]
    fn test_parse_random_tag() {
        let tag = parse_reward_tag("r1~3~i").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Random);
        assert_eq!(tag.content, "r1~3~i");
    }

    #[test]
    fn test_parse_random_range_tag() {
        let tag = parse_reward_tag("q1~2~3~m").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::RandomRange);
        assert_eq!(tag.content, "q1~2~3~m");
    }

    #[test]
    fn test_parse_value_tag() {
        let tag = parse_reward_tag("vscoreV50").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Value);
        assert_eq!(tag.content, "vscoreV50");
    }

    #[test]
    fn test_parse_or_tag() {
        let tag = parse_reward_tag("om(A)@4m(B)").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Or);
        assert_eq!(tag.content, "om(A)@4m(B)");
    }

    #[test]
    fn test_parse_enu_tag() {
        let tag = parse_reward_tag("e(template)").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Enu);
        assert_eq!(tag.content, "e(template)");
    }

    #[test]
    fn test_parse_skip_tag() {
        let tag = parse_reward_tag("s").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Skip);
        assert_eq!(tag.content, "s");
    }

    #[test]
    fn test_parse_replace_tag() {
        let tag = parse_reward_tag("pm(old)~(new)").unwrap();
        assert_eq!(tag.tag_type, RewardTagType::Replace);
        assert_eq!(tag.content, "pm(old)~(new)");
    }

    #[test]
    fn test_parse_empty_string_errors() {
        let result = parse_reward_tag("");
        assert!(result.is_err());
    }

    #[test]
    fn reward_parser_malformed_propagates_error() {
        // Empty tag is the pathological input previously reaching
        // `s.chars().next().unwrap()` (now replaced by the
        // `let Some(first) = s.chars().next() else { ... }` arm above). It must now
        // return `Err(RewardParse)` whose `content` equals the original
        // source string — source-vs-IR proof: a regression that derived
        // `content` from a default or placeholder would fail the exact
        // equality check below.
        let err = parse_reward_tag("").expect_err("empty reward tag must not parse");
        match err.kind.as_ref() {
            crate::error::ErrorKind::RewardParse { content, expected, found } => {
                assert_eq!(content, "");
                assert_eq!(expected, "non-empty reward tag string");
                assert_eq!(found, "empty string");
            }
            other => panic!("expected RewardParse, got {:?}", other),
        }
        assert_eq!(err.field_path.as_deref(), Some("reward_tag.raw"));
        assert!(err.suggestion.is_some());
    }

    #[test]
    fn test_parse_unknown_letter_errors() {
        let result = parse_reward_tag("x(something)");
        assert!(result.is_err());
    }

    #[test]
    fn test_reward_tag_roundtrip() {
        let test_cases = vec![
            "m(skip&hidden)",
            "i(Sword)",
            "l(levelup)",
            "g(hero)",
            "r1~3~i",
            "q1~2~3~m",
            "om(A)@4m(B)",
            "e(template)",
            "vscoreV50",
            "pm(old)~(new)",
            "s",
        ];

        for input in test_cases {
            let tag = parse_reward_tag(input).unwrap();
            let emitted = emit_reward_tag(&tag);
            assert_eq!(
                emitted, input,
                "round-trip failed for input: {}",
                input
            );
        }
    }
}
