//! Emit RewardTag back to textmod string form.
//!
//! Since RewardTag stores the full original string in its content field,
//! emission is a simple clone — guaranteeing lossless round-trip.

use crate::ir::RewardTag;

/// Emit a RewardTag to its textmod string representation.
pub fn emit_reward_tag(tag: &RewardTag) -> String {
    tag.content.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::RewardTagType;

    #[test]
    fn test_emit_modifier_tag() {
        let tag = RewardTag {
            tag_type: RewardTagType::Modifier,
            content: "m(skip&hidden)".to_string(),
        };
        assert_eq!(emit_reward_tag(&tag), "m(skip&hidden)");
    }

    #[test]
    fn test_emit_skip_tag() {
        let tag = RewardTag {
            tag_type: RewardTagType::Skip,
            content: "s".to_string(),
        };
        assert_eq!(emit_reward_tag(&tag), "s");
    }

    #[test]
    fn test_emit_preserves_content_exactly() {
        let tag = RewardTag {
            tag_type: RewardTagType::Or,
            content: "om(A)@4m(B)".to_string(),
        };
        assert_eq!(emit_reward_tag(&tag), "om(A)@4m(B)");
    }
}
