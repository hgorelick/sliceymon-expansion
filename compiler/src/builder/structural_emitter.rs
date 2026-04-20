use crate::ir::{StructuralContent, StructuralModifier};

/// Emit a structural modifier from its typed content.
///
/// Each StructuralContent variant stores a `body` field containing the full modifier
/// text. The emitter dispatches on the variant and returns the body.
pub fn emit(s: &StructuralModifier) -> String {
    match &s.content {
        StructuralContent::HeroPoolBase { body, .. } => body.clone(),
        StructuralContent::ItemPool { body, .. } => body.clone(),
        StructuralContent::BossModifier { body, .. } => body.clone(),
        StructuralContent::PartyConfig { body, .. } => body.clone(),
        StructuralContent::EventModifier { body, .. } => body.clone(),
        StructuralContent::Dialog { body, .. } => body.clone(),
        StructuralContent::ArtCredits { body } => body.clone(),
        StructuralContent::Selector { body, .. } => body.clone(),
        StructuralContent::GenSelect { body, .. } => body.clone(),
        StructuralContent::Difficulty { body, .. } => body.clone(),
        StructuralContent::LevelUpAction { body } => body.clone(),
        StructuralContent::PoolReplacement { body, .. } => body.clone(),
        StructuralContent::EndScreen { body } => body.clone(),
        StructuralContent::PhaseModifier { body, .. } => body.clone(),
        StructuralContent::Choosable { body, .. } => body.clone(),
        StructuralContent::ValueModifier { body, .. } => body.clone(),
        StructuralContent::HiddenModifier { body, .. } => body.clone(),
        StructuralContent::FightModifier { body, .. } => body.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Source, StructuralContent, StructuralModifier, StructuralType};

    #[test]
    fn structural_emitter_dialog() {
        let s = StructuralModifier {
            modifier_type: StructuralType::Dialog,
            name: Some("Credits".into()),
            content: StructuralContent::Dialog {
                body: "1.ph.4 Hello.mn.Intro".into(),
                phase: "4".into(),
            },
            derived: false,
            source: Source::Base,
        };
        assert_eq!(emit(&s), "1.ph.4 Hello.mn.Intro");
    }

}
