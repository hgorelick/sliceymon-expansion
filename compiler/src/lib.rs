pub mod error;
pub mod ir;
pub mod extractor;
pub mod builder;
pub mod util;
pub mod validator;

pub use validator::{validate, validate_cross_references, validate_ir, Finding, Severity, ValidationReport};
pub use ir::Source;

use std::collections::HashMap;

use error::CompilerError;
use ir::ModIR;

/// Extract a textmod string into a structured ModIR.
pub fn extract(textmod: &str) -> Result<ModIR, CompilerError> {
    extractor::extract(textmod)
}

/// Build a textmod string from a ModIR and sprite mappings.
pub fn build(ir: &ModIR, sprites: &HashMap<String, String>) -> Result<String, CompilerError> {
    builder::build(ir, sprites)
}

/// Build a complete textmod, auto-generating derived structurals (character selection,
/// hero pool base) if not explicitly present in the IR.
pub fn build_complete(ir: &ModIR, sprites: &HashMap<String, String>) -> Result<String, CompilerError> {
    builder::build_complete(ir, sprites)
}

/// Merge a base ModIR with an overlay ModIR.
pub fn merge(base: ModIR, overlay: ModIR) -> Result<ModIR, CompilerError> {
    ir::merge::merge(base, overlay)
}

// -- Single-item build functions --

/// Build a single hero into a modifier string.
pub fn build_hero(hero: &ir::Hero, sprites: &HashMap<String, String>) -> Result<String, CompilerError> {
    builder::hero_emitter::emit(hero, sprites)
}

/// Build a single replica item into a modifier string.
pub fn build_replica_item(item: &ir::ReplicaItem) -> Result<String, CompilerError> {
    builder::replica_item_emitter::emit(item)
}

/// Build a single monster into a modifier string.
pub fn build_monster(monster: &ir::Monster) -> Result<String, CompilerError> {
    builder::monster_emitter::emit_monster(monster)
}

/// Build a single boss into a modifier string.
pub fn build_boss(boss: &ir::Boss) -> Result<String, CompilerError> {
    builder::boss_emitter::emit_boss(boss)
}

// -- Single-item validate functions --

/// Validate a single hero in isolation.
pub fn validate_hero(hero: &ir::Hero, sprites: &HashMap<String, String>) -> ValidationReport {
    let mut report = ValidationReport::default();
    match build_hero(hero, sprites) {
        Ok(modifier) => {
            let single_mod = format!("{},", modifier);
            match validate(&single_mod) {
                Ok(sub_report) => {
                    report.errors.extend(sub_report.errors);
                    report.warnings.extend(sub_report.warnings);
                }
                Err(e) => {
                    report.errors.push(Finding {
                        rule_id: "E000".to_string(),
                        severity: Severity::Error,
                        modifier_index: Some(0),
                        modifier_name: Some(hero.mn_name.clone()),
                        position: None,
                        context: None,
                        message: format!("Validation failed: {}", e),
                        ..Default::default()
                    });
                }
            }
        }
        Err(e) => {
            report.errors.push(Finding {
                rule_id: "E000".to_string(),
                severity: Severity::Error,
                modifier_index: Some(0),
                modifier_name: Some(hero.mn_name.clone()),
                position: None,
                context: None,
                message: format!("Build failed: {}", e),
                ..Default::default()
            });
        }
    }
    report
}

/// Validate a hero in the context of an existing IR (checks color conflicts, name duplicates).
pub fn validate_hero_in_context(hero: &ir::Hero, ir: &ModIR, sprites: &HashMap<String, String>) -> ValidationReport {
    let mut report = validate_hero(hero, sprites);

    // Check color conflict
    if let Some(existing) = ir.heroes.iter().find(|h| h.color == hero.color && h.mn_name != hero.mn_name) {
        report.errors.push(Finding {
            rule_id: "E019".to_string(),
            severity: Severity::Error,
            modifier_index: None,
            modifier_name: Some(hero.mn_name.clone()),
            position: None,
            context: Some(format!("color '{}' used by '{}'", hero.color, existing.mn_name)),
            message: format!(
                "Hero '{}' uses color '{}' which is already used by '{}'",
                hero.mn_name, hero.color, existing.mn_name
            ),
            ..Default::default()
        });
    }

    // Check cross-category name duplicate
    let lower = hero.mn_name.to_lowercase();
    if ir.replica_items.iter().any(|r| r.name.to_lowercase() == lower) {
        report.errors.push(Finding {
            rule_id: "E020".to_string(),
            severity: Severity::Error,
            modifier_index: None,
            modifier_name: Some(hero.mn_name.clone()),
            position: None,
            context: None,
            message: format!("Hero name '{}' conflicts with an existing replica item", hero.mn_name),
            ..Default::default()
        });
    }

    report
}

/// Serialize a ModIR to JSON.
pub fn ir_to_json(ir: &ModIR) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(ir)
}

/// Deserialize a ModIR from JSON.
pub fn ir_from_json(json: &str) -> Result<ModIR, serde_json::Error> {
    serde_json::from_str(json)
}
