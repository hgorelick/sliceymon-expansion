use crate::error::CompilerError;
use crate::ir::Monster;

/// Emit a Monster struct as a modifier string.
///
/// Reconstructs from parsed fields. Emits .img. when img_data is present.
pub fn emit_monster(monster: &Monster) -> Result<String, CompilerError> {
    let mut out = String::new();

    if !monster.floor_range.is_empty() {
        out.push_str(&monster.floor_range);
        out.push('.');
    }

    // Simple monster format: monsterpool.(replica.TEMPLATE.n.NAME).OUTER_PROPS.mn.NAME
    // The monsterpool paren wraps the template and name; other properties are outside.
    out.push_str("monsterpool.(replica.");
    out.push_str(&monster.base_template);

    // Name inside the paren (the display name for the monster template)
    out.push_str(".n.");
    out.push_str(&monster.name);

    // Close monsterpool paren
    out.push(')');

    // Properties outside the paren
    if let Some(c) = monster.color {
        out.push_str(".col.");
        out.push(c);
    }

    if let Some(hp) = monster.hp {
        out.push_str(".hp.");
        out.push_str(&hp.to_string());
    }

    if let Some(ref chain) = monster.modifier_chain {
        out.push_str(&chain.emit());
    }

    if let Some(ref sd) = monster.sd {
        out.push_str(".sd.");
        out.push_str(&sd.emit());
    }

    if let Some(ref balance) = monster.balance {
        out.push_str(".bal.");
        out.push_str(balance);
    }

    if let Some(ref img) = monster.img_data {
        out.push_str(".img.");
        out.push_str(img);
    }

    if let Some(ref doc) = monster.doc {
        out.push_str(".doc.");
        out.push_str(doc);
    }

    // .mn. suffix
    out.push_str(".mn.");
    out.push_str(&monster.name);

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{DiceFaces, Source};

    #[test]
    fn monster_raw_fallback() {
        let mon = Monster {
            name: "Wooper".into(),
            base_template: "Slimelet".into(),
            floor_range: "1-3".into(),
            hp: Some(3),
            sd: Some(DiceFaces::parse("0:0:0:0:0:0")),
            sprite_name: Some("Wooper".into()),
            color: None,
            doc: None,
            modifier_chain: None,
            balance: None,
            img_data: None,
            source: Source::Base,
        };
        let output = emit_monster(&mon).unwrap();
        assert!(output.contains("monsterpool."), "Should produce valid monster");
    }

    #[test]
    fn emit_monster_produces_valid_output() {
        let mon = Monster {
            name: "Wooper".into(),
            base_template: "Slimelet".into(),
            floor_range: "1-3".into(),
            hp: Some(3),
            sd: Some(DiceFaces::parse("0:0:0:0:0:0")),
            sprite_name: Some("Wooper".into()),
            color: Some('b'),
            doc: None,
            modifier_chain: None,
            balance: None,
            img_data: None,
            source: Source::Base,
        };
        let output = emit_monster(&mon).unwrap();
        assert!(output.contains("monsterpool."));
        assert!(output.contains("replica.Slimelet"));
        assert!(output.contains(".n.Wooper"));
        assert!(output.contains("1-3."));
    }
}
