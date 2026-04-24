use crate::error::CompilerError;
use crate::ir::{ReplicaItem, ReplicaItemContainer};

/// Emit a ReplicaItem as a modifier string.
///
/// Dispatch is driven by `item.container`:
/// - `Capture { name }`   → `itempool.((...)).n.{name}.mn.{name}` (double- or
///   triple-paren depending on whether `abilitydata` is present).
/// - `Legendary`          → top-level `item.TEMPLATE...n.NAME[.cast.ABILITY]`.
///
/// The match is exhaustive by variant — no `_` arm — so adding a future
/// `ReplicaItemContainer` variant is a compile error here, not a silent
/// fallthrough (SPEC §3.6).
pub fn emit(item: &ReplicaItem) -> Result<String, CompilerError> {
    match &item.container {
        ReplicaItemContainer::Capture { name } => {
            if item.abilitydata.is_some() {
                emit_with_ability(item, name)
            } else {
                emit_simple(item, name)
            }
        }
        ReplicaItemContainer::Legendary => emit_legendary(item),
    }
}

/// Emit a simple Capture replica item (no ability).
/// Format: itempool.((hat.replica.Template...)).n.ContainerName.mn.Name
fn emit_simple(item: &ReplicaItem, container_name: &str) -> Result<String, CompilerError> {
    let mut out = String::new();

    out.push_str("itempool.((hat.replica.");
    out.push_str(&item.template);

    if let Some(c) = item.color {
        out.push_str(".col.");
        out.push(c);
    }

    if let Some(hp) = item.hp {
        out.push_str(".hp.");
        out.push_str(&hp.to_string());
    }

    if let Some(ref chain) = item.item_modifiers {
        out.push_str(&chain.emit());
    }

    out.push_str(".sd.");
    out.push_str(&item.sd.emit());

    if !item.sprite.img_data().is_empty() {
        out.push_str(".img.");
        out.push_str(item.sprite.img_data());
    }

    // Inner character name
    out.push_str(".n.");
    out.push_str(&item.name);

    out.push_str("))");

    // Container name outside
    out.push_str(".n.");
    out.push_str(container_name);

    if let Some(tier) = item.tier {
        out.push_str(".tier.");
        out.push_str(&tier.to_string());
    }

    if let Some(ref sticker) = item.sticker {
        out.push_str(".sticker.");
        out.push_str(sticker);
    }

    if let Some(ref tog) = item.toggle_flags {
        out.push_str(tog);
    }

    // .mn. suffix
    out.push_str(".mn.");
    out.push_str(&item.name);

    Ok(out)
}

/// Emit a Capture replica item with ability (cast block).
/// Format: itempool.((hat.(replica.Template...cast.ABILITY...))).n.ContainerName.mn.Name
fn emit_with_ability(item: &ReplicaItem, container_name: &str) -> Result<String, CompilerError> {
    let mut out = String::new();

    out.push_str("itempool.((hat.(replica.");
    out.push_str(&item.template);

    if let Some(c) = item.color {
        out.push_str(".col.");
        out.push(c);
    }

    if let Some(hp) = item.hp {
        out.push_str(".hp.");
        out.push_str(&hp.to_string());
    }

    if let Some(ref chain) = item.item_modifiers {
        out.push_str(&chain.emit());
    }

    out.push_str(".sd.");
    out.push_str(&item.sd.emit());

    if !item.sprite.img_data().is_empty() {
        out.push_str(".img.");
        out.push_str(item.sprite.img_data());
    }

    // Inner character name
    out.push_str(".n.");
    out.push_str(&item.name);

    // Abilitydata (cast)
    if let Some(ref ability) = item.abilitydata {
        out.push_str(".cast.");
        out.push_str(&ability.emit());
    }

    if let Some(ref speech) = item.speech {
        out.push_str(".speech.");
        out.push_str(speech);
    }

    if let Some(ref doc) = item.doc {
        out.push_str(".doc.");
        out.push_str(doc);
    }

    out.push_str(")))");

    // Container name outside
    out.push_str(".n.");
    out.push_str(container_name);

    // .mn. suffix
    out.push_str(".mn.");
    out.push_str(&item.name);

    Ok(out)
}

/// Emit a Legendary replica item — top-level `item.` shape, no container name.
/// Format: `item.TEMPLATE[.col.C][.hp.N][chain].sd.SD[.img.IMG].n.NAME[.cast.ABILITY]`.
fn emit_legendary(item: &ReplicaItem) -> Result<String, CompilerError> {
    let mut out = String::new();

    out.push_str("item.");
    out.push_str(&item.template);

    if let Some(c) = item.color {
        out.push_str(".col.");
        out.push(c);
    }

    if let Some(hp) = item.hp {
        out.push_str(".hp.");
        out.push_str(&hp.to_string());
    }

    if let Some(ref chain) = item.item_modifiers {
        out.push_str(&chain.emit());
    }

    out.push_str(".sd.");
    out.push_str(&item.sd.emit());

    if !item.sprite.img_data().is_empty() {
        out.push_str(".img.");
        out.push_str(item.sprite.img_data());
    }

    out.push_str(".n.");
    out.push_str(&item.name);

    if let Some(ref ability) = item.abilitydata {
        out.push_str(".cast.");
        out.push_str(&ability.emit());
    }

    if let Some(ref speech) = item.speech {
        out.push_str(".speech.");
        out.push_str(speech);
    }

    if let Some(ref doc) = item.doc {
        out.push_str(".doc.");
        out.push_str(doc);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authoring::SpriteId;
    use crate::ir::{DiceFaces, Source};

    #[test]
    fn emit_simple_replica_item() {
        let item = ReplicaItem {
            name: "Pikachu".into(),
            container: ReplicaItemContainer::Capture { name: "Ball".into() },
            tier: None,
            template: "Hat".into(),
            hp: None,
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            sprite: SpriteId::owned("Pikachu", ""),
            color: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            doc: None,
            speech: None,
            abilitydata: None,
            source: Source::Base,
        };
        let output = emit(&item).unwrap();
        assert!(output.contains("itempool."), "Should produce valid item");
        assert!(output.contains(".n.Pikachu"), "Should have character name");
        assert!(output.contains("hat.replica"), "Simple format uses hat.replica");
        assert!(!output.contains("hat.(replica"), "Simple format should NOT use triple parens");
    }

    #[test]
    fn emit_simple_with_tier_and_sticker() {
        let item = ReplicaItem {
            name: "Pikachu".into(),
            container: ReplicaItemContainer::Capture { name: "PokeBall".into() },
            tier: Some(2),
            template: "Hat".into(),
            hp: Some(5),
            sd: DiceFaces::parse("34-1:0:0:0:0:0"),
            sprite: SpriteId::owned("Pikachu", ""),
            color: Some('y'),
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            doc: None,
            speech: None,
            abilitydata: None,
            source: Source::Base,
        };
        let output = emit(&item).unwrap();
        assert!(output.contains("itempool."));
        assert!(output.contains("hat.replica"));
        assert!(output.contains(".n.Pikachu"));
        assert!(output.contains(".n.PokeBall"));
        assert!(output.contains(".tier.2"));
    }

    #[test]
    fn emit_with_ability() {
        let item = ReplicaItem {
            name: "Mewtwo".into(),
            container: ReplicaItemContainer::Capture { name: "MasterBall".into() },
            tier: None,
            template: "Alpha".into(),
            hp: Some(20),
            sd: DiceFaces::parse("34-3:34-3:34-3:0:0:0"),
            sprite: SpriteId::owned("Mewtwo", ""),
            color: Some('p'),
            doc: None,
            speech: None,
            abilitydata: Some(crate::ir::AbilityData::parse("(Fey.sd.34-1:0.img.spark.n.Psychic)")),
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            source: Source::Base,
        };
        let output = emit(&item).unwrap();
        assert!(output.contains("itempool."));
        assert!(output.contains("hat.(replica"), "With-ability format uses hat.(replica");
        assert!(output.contains("cast."));
        assert!(output.contains(".n.Mewtwo"));
        assert!(output.contains(".n.MasterBall"));
    }

    #[test]
    fn emit_legendary_uses_top_level_item() {
        let item = ReplicaItem {
            name: "Mew".into(),
            container: ReplicaItemContainer::Legendary,
            tier: None,
            template: "Alpha".into(),
            hp: Some(12),
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            sprite: SpriteId::owned("Mew", ""),
            color: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            doc: None,
            speech: None,
            abilitydata: None,
            source: Source::Base,
        };
        let output = emit(&item).unwrap();
        assert!(output.starts_with("item."), "Legendary emits top-level item.*, got: {}", output);
        assert!(!output.contains("itempool."), "Legendary must NOT wrap in itempool.");
        assert!(!output.contains(".mn."), "Legendary must NOT emit the `.mn.` container-side suffix");
        assert!(output.contains(".n.Mew"));
    }

    #[test]
    fn legendary_emit_parse_roundtrip_with_all_fields() {
        // End-to-end roundtrip: build a fully-populated Legendary, emit,
        // re-parse, and assert every scalar field survives. No working-mod
        // in `working-mods/*.txt` contains a top-level `item.*` modifier
        // (verified: `grep -E '(^|[,(])\s*item\.' working-mods/*.txt`
        // returns 0 matches), so `roundtrip_diag` cannot exercise this
        // path — this test is the only thing pinning emit/parse parity
        // for Legendary.
        use crate::extractor::replica_item_parser::parse_legendary;

        let item = ReplicaItem {
            name: "Mew".into(),
            container: ReplicaItemContainer::Legendary,
            tier: None,
            template: "Alpha".into(),
            hp: Some(12),
            sd: DiceFaces::parse("15-1:0:0:0:0:0"),
            sprite: SpriteId::owned("Mew", "bas170:55"),
            color: Some('c'),
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            doc: Some("Psychic Legendary".into()),
            speech: Some("Mew!".into()),
            abilitydata: Some(crate::ir::AbilityData::parse(
                "(Spell.sd.150-1:0:0:0:0:0.n.Psychic)",
            )),
            source: Source::Base,
        };

        let emitted = emit(&item).unwrap();
        let parsed = parse_legendary(&emitted, 0).expect("emit(Legendary) round-trips through parse_legendary");

        assert_eq!(parsed.container, item.container, "container variant");
        assert_eq!(parsed.name, item.name, "name");
        assert_eq!(parsed.template, item.template, "template");
        assert_eq!(parsed.hp, item.hp, "hp");
        assert_eq!(parsed.color, item.color, "color");
        assert_eq!(parsed.sd, item.sd, "sd");
        assert_eq!(parsed.doc, item.doc, "doc");
        assert_eq!(parsed.speech, item.speech, "speech");
        assert_eq!(
            parsed.sprite.img_data(),
            item.sprite.img_data(),
            "sprite img_data",
        );
        assert_eq!(
            parsed.abilitydata.as_ref().map(|a| a.emit()),
            item.abilitydata.as_ref().map(|a| a.emit()),
            "abilitydata (compared via re-emit)",
        );
    }

    #[test]
    fn legendary_emit_parse_roundtrip_with_item_modifiers() {
        // Separate from `legendary_emit_parse_roundtrip_with_all_fields` so the
        // chain emission branch in `emit_legendary` isn't dead-weight coverage:
        // no working mod contains a top-level `item.*`, so without this test
        // any regression in chain ordering (e.g. emitting the chain after
        // `.sd.` instead of before) would pass every other gate.
        use crate::extractor::replica_item_parser::parse_legendary;
        use crate::ir::ModifierChain;

        let chain_src = ".i.left.k.scared#facade.bas170:55";
        let item = ReplicaItem {
            name: "Mew".into(),
            container: ReplicaItemContainer::Legendary,
            tier: None,
            template: "Alpha".into(),
            hp: Some(9),
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            sprite: SpriteId::owned("Mew", ""),
            color: None,
            item_modifiers: Some(ModifierChain::parse(chain_src)),
            sticker: None,
            toggle_flags: None,
            doc: None,
            speech: None,
            abilitydata: None,
            source: Source::Base,
        };

        let emitted = emit(&item).unwrap();
        let parsed = parse_legendary(&emitted, 0).expect("emit(Legendary) round-trips through parse_legendary");

        assert_eq!(parsed.container, item.container, "container variant");
        assert_eq!(parsed.name, item.name, "name");
        assert_eq!(parsed.template, item.template, "template");
        assert_eq!(parsed.hp, item.hp, "hp");
        assert_eq!(parsed.sd, item.sd, "sd");
        assert_eq!(
            parsed.item_modifiers.as_ref().map(|c| c.emit()),
            item.item_modifiers.as_ref().map(|c| c.emit()),
            "item_modifiers (compared via re-emit)",
        );
    }
}
