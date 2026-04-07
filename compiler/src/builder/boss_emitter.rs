use crate::error::CompilerError;
use crate::ir::Boss;

/// Emit a Boss struct as a modifier string.
///
/// If the boss has a `raw` field, emits that directly.
/// Otherwise, reconstructs from parsed fields.
pub fn emit_boss(boss: &Boss) -> Result<String, CompilerError> {
    if let Some(ref raw) = boss.raw {
        return Ok(raw.clone());
    }

    // Reconstruct: ch.omN.fight.(template...fight_content)
    let mut out = String::new();

    if let Some(level) = boss.level {
        out.push_str("ch.om");
        out.push_str(&level.to_string());
    }

    out.push_str(".fight.(");

    // Emit fight units
    for (i, unit) in boss.fight_units.iter().enumerate() {
        if i > 0 {
            out.push('+');
        }

        out.push_str("replica.");
        out.push_str(&unit.template);

        if let Some(hp) = unit.hp {
            out.push_str(".hp.");
            out.push_str(&hp.to_string());
        }

        if let Some(ref sd) = unit.sd {
            out.push_str(".sd.");
            out.push_str(sd);
        }

        if let Some(ref img) = unit.sprite_data {
            out.push_str(".img.");
            out.push_str(img);
        }

        out.push_str(".n.");
        out.push_str(&unit.name);
    }

    out.push(')');

    if let Some(ref doc) = boss.doc {
        out.push_str(".doc.");
        out.push_str(doc);
    }

    if let Some(ref chain) = boss.modifier_chain {
        out.push_str(chain);
    }

    // .mn. suffix
    out.push_str(".mn.");
    out.push_str(&boss.name);

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::BossFightUnit;

    #[test]
    fn boss_raw_fallback() {
        let boss = Boss {
            name: "Boss1".into(),
            level: Some(5),
            template: None,
            hp: None,
            sd: None,
            sprite_name: None,
            doc: None,
            modifier_chain: None,
            fight_units: vec![],
            variant: None,
            raw: Some("original-boss-raw".into()),
        };
        assert_eq!(emit_boss(&boss).unwrap(), "original-boss-raw");
    }

    #[test]
    fn emit_boss_produces_valid_output() {
        let boss = Boss {
            name: "Boss1".into(),
            level: Some(5),
            template: Some("Alpha".into()),
            hp: Some(20),
            sd: Some("34-3:0:0:0:0:0".into()),
            sprite_name: Some("Boss1".into()),
            doc: None,
            modifier_chain: None,
            fight_units: vec![
                BossFightUnit {
                    template: "Alpha".into(),
                    name: "BigBoss".into(),
                    hp: Some(20),
                    sd: Some("34-3:0:0:0:0:0".into()),
                    sprite_data: Some("abc123".into()),
                },
            ],
            variant: None,
            raw: None,
        };
        let output = emit_boss(&boss).unwrap();
        assert!(output.contains(".fight."));
        assert!(output.contains("replica.Alpha"));
        assert!(output.contains(".n.BigBoss"));
        assert!(output.contains("ch.om5"));
    }
}
