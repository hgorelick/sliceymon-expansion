use crate::ir::{ReplicaItem, StructuralContent, StructuralModifier};

/// Emit a structural modifier from its typed content.
///
/// Most variants store a `body` field containing the full modifier text;
/// the emitter dispatches on the variant and returns the body. The
/// `ItemPool` variant is the exception: post-8A it no longer carries a
/// `body` field — the emitter rebuilds the pool body from the typed
/// `items: Vec<ItempoolItem>` via `replica_item_emitter::emit_itempool`,
/// which also needs a `&[ReplicaItem]` slice so `Summon(i)` entries can
/// dereference into the flat replica list.
pub fn emit(s: &StructuralModifier, replica_items: &[ReplicaItem]) -> String {
    match &s.content {
        StructuralContent::HeroPoolBase { body, .. } => body.clone(),
        StructuralContent::ItemPool { items } => crate::builder::replica_item_emitter::emit_itempool(
            items,
            replica_items,
            s.name.as_deref().unwrap_or(""),
        ),
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
        assert_eq!(emit(&s, &[]), "1.ph.4 Hello.mn.Intro");
    }
}
