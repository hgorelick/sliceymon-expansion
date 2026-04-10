use crate::error::CompilerError;
use crate::ir::{Boss, BossFormat};

/// Emit a Boss struct as a modifier string.
/// Dispatches on format: Standard (ch.om), Event (ch.om(...)), or Encounter (ph.b+fight).
pub fn emit_boss(boss: &Boss) -> Result<String, CompilerError> {
    match boss.format {
        BossFormat::Standard => emit_standard(boss),
        BossFormat::Event => emit_event(boss),
        BossFormat::Encounter => emit_encounter(boss),
    }
}

/// Emit an event boss: `ch.om{event_body}.mn.Name`
///
/// The event_body contains the full event content (initial body + branches).
fn emit_event(boss: &Boss) -> Result<String, CompilerError> {
    let mut out = String::from("ch.om");
    if let Some(ref body) = boss.event_body {
        out.push_str(body);
    }
    out.push_str(".mn.");
    out.push_str(&boss.name);
    Ok(out)
}

/// Emit a standard ch.om boss.
///
/// Single-variant: `ch.omN.fight.(units...).doc.X.mn.Name`
/// Multi-variant:  `ch.omN.fight.(units_A...).mn.VarA@trigger.fight.(units_B...).doc.X.mn.Name`
fn emit_standard(boss: &Boss) -> Result<String, CompilerError> {
    let mut out = String::new();

    if let Some(level) = boss.level {
        out.push_str("ch.om");
        out.push_str(&level.to_string());
    }

    for variant in &boss.variants {
        out.push_str(".fight.(");

        for (ui, unit) in variant.fight_units.iter().enumerate() {
            if ui > 0 {
                out.push('+');
            }
            emit_fight_unit(&mut out, unit);
        }

        out.push(')');

        // Variant name and trigger (for multi-variant bosses with labeled alternatives)
        if !variant.name.is_empty() {
            out.push_str(".mn.");
            out.push_str(&variant.name);
        }
        if let Some(ref trigger) = variant.trigger {
            out.push_str(trigger);
        }
    }

    // Doc and modifier chain (after all variants)
    if let Some(ref doc) = boss.doc {
        out.push_str(".doc.");
        out.push_str(doc);
    }
    if let Some(ref chain) = boss.modifier_chain {
        out.push_str(&chain.emit());
    }

    // Overall boss name
    out.push_str(".mn.");
    out.push_str(&boss.name);

    Ok(out)
}

/// Emit an encounter (ph.b+fight) boss.
///
/// Format: `1.ph.b{id};1;!m({level}.fight.{units}+...&hidden).mn.{name}@2!m(skip&hidden&temporary)`
fn emit_encounter(boss: &Boss) -> Result<String, CompilerError> {
    let mut out = String::new();

    out.push_str("1.ph.b");
    out.push(boss.encounter_id.unwrap_or('X'));
    out.push_str(";1;!m(");

    if let Some(level) = boss.level {
        out.push_str(&level.to_string());
    }

    // Fight content (flat, not paren-wrapped)
    out.push_str(".fight.");

    if let Some(variant) = boss.variants.first() {
        for (ui, unit) in variant.fight_units.iter().enumerate() {
            if ui > 0 {
                out.push('+');
            }
            emit_fight_unit(&mut out, unit);
        }
    }

    out.push_str("&hidden)");
    out.push_str(".mn.");
    out.push_str(&boss.name);
    out.push_str("@2!m(skip&hidden&temporary)");

    Ok(out)
}

/// Emit a single fight unit's properties.
fn emit_fight_unit(out: &mut String, unit: &crate::ir::BossFightUnit) {
    out.push_str(&unit.template);

    // Template override (.t.)
    if let Some(ref t) = unit.template_override {
        out.push_str(".t.");
        out.push_str(t);
    }

    // Modifier chain (items, equipment, facades)
    if let Some(ref chain) = unit.modifier_chain {
        out.push_str(&chain.emit());
    }

    if let Some(hp) = unit.hp {
        out.push_str(".hp.");
        out.push_str(&hp.to_string());
    }

    if let Some(ref sd) = unit.sd {
        out.push_str(".sd.");
        out.push_str(&sd.emit());
    }

    if let Some(ref img) = unit.sprite_data {
        out.push_str(".img.");
        out.push_str(img);
    }

    if let Some(ref doc) = unit.doc {
        out.push_str(".doc.");
        out.push_str(doc);
    }

    out.push_str(".n.");
    out.push_str(&unit.name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BossFightUnit, BossFightVariant, DiceFaces, Source};

    #[test]
    fn emit_single_variant_boss() {
        let boss = Boss {
            name: "TestBoss".into(),
            level: Some(4),
            format: BossFormat::Standard,
            encounter_id: None,
            variants: vec![BossFightVariant {
                name: String::new(),
                trigger: None,
                fight_units: vec![BossFightUnit {
                    template: "Sniper".into(),
                    name: "Wooper".into(),
                    hp: Some(3),
                    sd: Some(DiceFaces::parse("0:0:0:0:0:0")),
                    sprite_data: None,
                    template_override: None,
                    doc: None,
                    modifier_chain: None,
                }],
            }],
            doc: None,
            modifier_chain: None,
            source: Source::Base,
            event_body: None,
        };
        let out = emit_boss(&boss).unwrap();
        assert!(out.starts_with("ch.om4"), "Should start with ch.om4: {}", out);
        assert!(out.contains(".fight."), "Should contain .fight.");
        assert!(out.contains(".mn.TestBoss"), "Should contain .mn.TestBoss");
        assert!(out.contains(".n.Wooper"), "Should contain .n.Wooper");
    }

    #[test]
    fn emit_encounter_boss() {
        let boss = Boss {
            name: "TestEncounter".into(),
            level: Some(12),
            format: BossFormat::Encounter,
            encounter_id: Some('X'),
            variants: vec![BossFightVariant {
                name: "TestEncounter".into(),
                trigger: None,
                fight_units: vec![BossFightUnit {
                    template: "Basalt".into(),
                    name: "Necrozma".into(),
                    hp: Some(25),
                    sd: None,
                    sprite_data: None,
                    template_override: None,
                    doc: Some("The Pillager".into()),
                    modifier_chain: None,
                }],
            }],
            doc: None,
            modifier_chain: None,
            source: Source::Base,
            event_body: None,
        };
        let out = emit_boss(&boss).unwrap();
        assert!(out.starts_with("1.ph.bX;1;!m("), "Should start with encounter prefix: {}", out);
        assert!(out.contains("12.fight."), "Should contain level.fight.");
        assert!(out.contains("&hidden)"), "Should contain &hidden)");
        assert!(out.contains(".mn.TestEncounter@2!m(skip&hidden&temporary)"), "Should have encounter suffix");
    }

    #[test]
    fn boss_empty_variants() {
        let boss = Boss {
            name: "Empty".into(),
            level: Some(1),
            format: BossFormat::Standard,
            encounter_id: None,
            variants: vec![],
            doc: None,
            modifier_chain: None,
            source: Source::Base,
            event_body: None,
        };
        let out = emit_boss(&boss).unwrap();
        assert!(out.contains(".mn.Empty"), "Should contain name even with no variants");
    }
}
