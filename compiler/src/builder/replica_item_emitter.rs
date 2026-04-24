use crate::error::CompilerError;
use crate::ir::ReplicaItem;

/// Emit a ReplicaItem as a modifier string.
///
/// Only the Legendary shape is modelled — Capture-shaped items route as
/// `ItemPool` structurals because no corpus instance exists for a
/// Capture-specific `ReplicaItem` variant (chunk-impl rule 3).
pub fn emit(item: &ReplicaItem) -> Result<String, CompilerError> {
    emit_legendary(item)
}

/// Emit a Legendary replica item — top-level `item.` shape, no container name.
/// Format: `item.TEMPLATE[.col.C][.hp.N][.sd.SD][.img.IMG][chain].n.NAME[.abilitydata.ABILITY]`.
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

    // Scalars precede the chain so `parse_legendary`'s `scalar_slice` +
    // depth-aware extract has a chain-and-ability-free prefix to scan
    // (§F10, Chunk 9). A chain-interior `.sd.` / `.img.` decoy therefore
    // cannot appear before the legit top-level value.
    out.push_str(".sd.");
    out.push_str(&item.sd.emit());

    if !item.sprite.img_data().is_empty() {
        out.push_str(".img.");
        out.push_str(item.sprite.img_data());
    }

    if let Some(ref chain) = item.item_modifiers {
        out.push_str(&chain.emit());
    }

    out.push_str(".n.");
    out.push_str(&item.name);

    // Ability body — `.abilitydata.(body)` per the textmod guide
    // (reference/textmod_guide.md lines 747 / 857 / 975-981). `cast.X` is
    // a chain keyword (guide lines 642-645), not a property marker.
    if let Some(ref ability) = item.abilitydata {
        out.push_str(".abilitydata.");
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
    fn emit_legendary_uses_top_level_item() {
        let item = ReplicaItem {
            name: "Mew".into(),
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
