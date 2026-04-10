use crate::error::CompilerError;
use crate::ir::ReplicaItem;

/// Emit a ReplicaItem as a modifier string.
///
/// Format is derived from fields:
/// - If abilitydata is present → triple-paren format: itempool.((hat.(replica...cast...)))
/// - Otherwise → double-paren format: itempool.((hat.replica...))
pub fn emit(item: &ReplicaItem) -> Result<String, CompilerError> {
    if item.abilitydata.is_some() {
        emit_with_ability(item)
    } else {
        emit_simple(item)
    }
}

/// Emit a simple replica item (no ability).
/// Format: itempool.((hat.replica.Template...)).n.ContainerName.mn.Name
fn emit_simple(item: &ReplicaItem) -> Result<String, CompilerError> {
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

    if let Some(ref img) = item.img_data {
        out.push_str(".img.");
        out.push_str(img);
    }

    // Inner character name
    out.push_str(".n.");
    out.push_str(&item.name);

    out.push_str("))");

    // Container name outside
    out.push_str(".n.");
    out.push_str(&item.container_name);

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

/// Emit a replica item with ability (cast block).
/// Format: itempool.((hat.(replica.Template...cast.ABILITY...))).n.ContainerName.mn.Name
fn emit_with_ability(item: &ReplicaItem) -> Result<String, CompilerError> {
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

    if let Some(ref img) = item.img_data {
        out.push_str(".img.");
        out.push_str(img);
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
    out.push_str(&item.container_name);

    // .mn. suffix
    out.push_str(".mn.");
    out.push_str(&item.name);

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{DiceFaces, Source};

    #[test]
    fn emit_simple_replica_item() {
        let item = ReplicaItem {
            name: "Pikachu".into(),
            container_name: "Ball".into(),
            tier: None,
            template: "Hat".into(),
            hp: None,
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            sprite_name: "Pikachu".into(),
            color: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            img_data: None,
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
            container_name: "PokeBall".into(),
            tier: Some(2),
            template: "Hat".into(),
            hp: Some(5),
            sd: DiceFaces::parse("34-1:0:0:0:0:0"),
            sprite_name: "Pikachu".into(),
            color: Some('y'),
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            img_data: None,
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
            container_name: "MasterBall".into(),
            tier: None,
            template: "Alpha".into(),
            hp: Some(20),
            sd: DiceFaces::parse("34-3:34-3:34-3:0:0:0"),
            sprite_name: "Mewtwo".into(),
            color: Some('p'),
            doc: None,
            speech: None,
            abilitydata: Some(crate::ir::AbilityData::parse("(Fey.sd.34-1:0.img.spark.n.Psychic)")),
            item_modifiers: None,
            img_data: None,
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
}
