use crate::error::CompilerError;
use crate::ir::Monster;

/// Emit a Monster struct as a modifier string.
///
/// If the monster has a `raw` field, emits that directly.
/// Otherwise, reconstructs from parsed fields.
pub fn emit_monster(monster: &Monster) -> Result<String, CompilerError> {
    if let Some(ref raw) = monster.raw {
        return Ok(raw.clone());
    }

    // Reconstruct: floor_range.monsterpool.(replica.template.hp.N.sd.FACES...).n.Name
    let mut out = String::new();

    if !monster.floor_range.is_empty() {
        out.push_str(&monster.floor_range);
        out.push('.');
    }

    out.push_str("monsterpool.(replica.");
    out.push_str(&monster.base_template);

    if let Some(c) = monster.color {
        out.push_str(".col.");
        out.push(c);
    }

    if let Some(hp) = monster.hp {
        out.push_str(".hp.");
        out.push_str(&hp.to_string());
    }

    if let Some(ref chain) = monster.modifier_chain {
        out.push_str(chain);
    }

    if let Some(ref sd) = monster.sd {
        out.push_str(".sd.");
        out.push_str(sd);
    }

    if let Some(ref balance) = monster.balance {
        out.push_str(".bal.");
        out.push_str(balance);
    }

    out.push_str(".n.");
    out.push_str(&monster.name);

    if let Some(ref doc) = monster.doc {
        out.push_str(".doc.");
        out.push_str(doc);
    }

    out.push(')');

    // .mn. suffix
    out.push_str(".mn.");
    out.push_str(&monster.name);

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monster_raw_fallback() {
        let mon = Monster {
            name: "Wooper".into(),
            base_template: "Slimelet".into(),
            floor_range: "1-3".into(),
            hp: Some(3),
            sd: Some("0:0:0:0:0:0".into()),
            sprite_name: Some("Wooper".into()),
            color: None,
            doc: None,
            modifier_chain: None,
            balance: None,
            raw: Some("original-monster-raw".into()),
        };
        assert_eq!(emit_monster(&mon).unwrap(), "original-monster-raw");
    }

    #[test]
    fn emit_monster_produces_valid_output() {
        let mon = Monster {
            name: "Wooper".into(),
            base_template: "Slimelet".into(),
            floor_range: "1-3".into(),
            hp: Some(3),
            sd: Some("0:0:0:0:0:0".into()),
            sprite_name: Some("Wooper".into()),
            color: Some('b'),
            doc: None,
            modifier_chain: None,
            balance: None,
            raw: None,
        };
        let output = emit_monster(&mon).unwrap();
        assert!(output.contains("monsterpool."));
        assert!(output.contains("replica.Slimelet"));
        assert!(output.contains(".n.Wooper"));
        assert!(output.contains("1-3."));
    }
}
