use crate::error::CompilerError;
use crate::ir::{Capture, Legendary};

/// Emit a Capture struct as a modifier string.
///
/// If the capture has a `raw` field, emits that directly.
/// Otherwise, reconstructs the modifier from parsed fields.
pub fn emit_capture(capture: &Capture) -> Result<String, CompilerError> {
    if let Some(ref raw) = capture.raw {
        return Ok(raw.clone());
    }

    let mut out = String::new();

    // itempool.((hat.replica.Template...)).n.BallName
    out.push_str("itempool.((hat.replica.");
    out.push_str(&capture.template);

    if let Some(c) = capture.color {
        out.push_str(".col.");
        out.push(c);
    }

    if let Some(hp) = capture.hp {
        out.push_str(".hp.");
        out.push_str(&hp.to_string());
    }

    if let Some(ref chain) = capture.item_modifiers {
        out.push_str(chain);
    }

    out.push_str(".sd.");
    out.push_str(&capture.sd);

    // Sprite data is inline in captures — use .n. for pokemon name inside replica
    out.push_str(".n.");
    out.push_str(&capture.pokemon);

    out.push_str("))");

    // Ball name outside
    out.push_str(".n.");
    out.push_str(&capture.ball_name);

    if let Some(tier) = capture.ball_tier {
        out.push_str(".tier.");
        out.push_str(&tier.to_string());
    }

    if let Some(ref sticker) = capture.sticker {
        out.push_str(".sticker.");
        out.push_str(sticker);
    }

    if let Some(ref tog) = capture.toggle_flags {
        out.push_str(tog);
    }

    // .mn. suffix
    out.push_str(".mn.");
    out.push_str(&capture.pokemon);

    Ok(out)
}

/// Emit a Legendary struct as a modifier string.
pub fn emit_legendary(legendary: &Legendary) -> Result<String, CompilerError> {
    if let Some(ref raw) = legendary.raw {
        return Ok(raw.clone());
    }

    let mut out = String::new();

    // itempool.((hat.(replica.Template...cast.abilitydata...))).n.SummoningItem
    out.push_str("itempool.((hat.(replica.");
    out.push_str(&legendary.template);

    if let Some(c) = legendary.color {
        out.push_str(".col.");
        out.push(c);
    }

    if let Some(hp) = legendary.hp {
        out.push_str(".hp.");
        out.push_str(&hp.to_string());
    }

    if let Some(ref chain) = legendary.item_modifiers {
        out.push_str(chain);
    }

    out.push_str(".sd.");
    out.push_str(&legendary.sd);

    // Pokemon name inside replica
    out.push_str(".n.");
    out.push_str(&legendary.pokemon);

    // Abilitydata (cast)
    if let Some(ref ability) = legendary.abilitydata {
        out.push_str(".cast.");
        out.push_str(ability);
    }

    if let Some(ref speech) = legendary.speech {
        out.push_str(".speech.");
        out.push_str(speech);
    }

    if let Some(ref doc) = legendary.doc {
        out.push_str(".doc.");
        out.push_str(doc);
    }

    out.push_str(")))");

    // Summoning item name outside
    out.push_str(".n.");
    out.push_str(&legendary.summoning_item);

    // .mn. suffix
    out.push_str(".mn.");
    out.push_str(&legendary.pokemon);

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_raw_fallback() {
        let cap = Capture {
            pokemon: "Pikachu".into(),
            ball_name: "Ball".into(),
            ball_tier: None,
            template: "Hat".into(),
            hp: None,
            sd: "0:0:0:0:0:0".into(),
            sprite_name: "Pikachu".into(),
            color: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            raw: Some("original-raw-content".into()),
        };
        assert_eq!(emit_capture(&cap).unwrap(), "original-raw-content");
    }

    #[test]
    fn emit_capture_produces_valid_output() {
        let cap = Capture {
            pokemon: "Pikachu".into(),
            ball_name: "PokeBall".into(),
            ball_tier: Some(2),
            template: "Hat".into(),
            hp: Some(5),
            sd: "34-1:0:0:0:0:0".into(),
            sprite_name: "Pikachu".into(),
            color: Some('y'),
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            raw: None,
        };
        let output = emit_capture(&cap).unwrap();
        assert!(output.contains("itempool."));
        assert!(output.contains("hat.replica"));
        assert!(output.contains(".n.Pikachu"));
        assert!(output.contains(".n.PokeBall"));
        assert!(output.contains(".tier.2"));
    }

    #[test]
    fn emit_legendary_produces_valid_output() {
        let leg = Legendary {
            pokemon: "Mewtwo".into(),
            summoning_item: "MasterBall".into(),
            template: "Alpha".into(),
            hp: Some(20),
            sd: "34-3:34-3:34-3:0:0:0".into(),
            sprite_name: "Mewtwo".into(),
            color: Some('p'),
            doc: None,
            speech: None,
            abilitydata: Some("(Fey.sd.34-1:0.img.spark.n.Psychic)".into()),
            item_modifiers: None,
            raw: None,
        };
        let output = emit_legendary(&leg).unwrap();
        assert!(output.contains("itempool."));
        assert!(output.contains("cast."));
        assert!(output.contains(".n.Mewtwo"));
        assert!(output.contains(".n.MasterBall"));
    }
}
