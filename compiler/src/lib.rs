pub mod authoring;
pub mod constants;
pub mod error;
pub mod ir;
pub mod extractor;
pub mod builder;
pub mod util;
pub mod xref;

pub use xref::{check_references, check_hero_in_context, check_boss_in_context, Finding, Severity, ValidationReport};
pub use ir::Source;
pub use authoring::SpriteId;
pub use builder::{BuildOptions, SourceFilter, SourceSet};

use error::CompilerError;
use ir::ModIR;

/// Extract a textmod string into a structured ModIR.
pub fn extract(textmod: &str) -> Result<ModIR, CompilerError> {
    extractor::extract(textmod)
}

/// Build a textmod string from a ModIR.
pub fn build(ir: &ModIR) -> Result<String, CompilerError> {
    builder::build(ir)
}

/// Build a textmod string from a ModIR, honoring the supplied [`BuildOptions`].
///
/// `build(ir)` is equivalent to `build_with(ir, &BuildOptions::default())`.
pub fn build_with(ir: &ModIR, opts: &BuildOptions) -> Result<String, CompilerError> {
    builder::build_with(ir, opts)
}

/// Build a complete textmod, auto-generating derived structurals (character selection,
/// hero pool base) if not explicitly present in the IR.
pub fn build_complete(ir: &ModIR) -> Result<String, CompilerError> {
    builder::build_complete(ir)
}

/// Merge a base ModIR with an overlay ModIR.
pub fn merge(base: ModIR, overlay: ModIR) -> Result<ModIR, CompilerError> {
    ir::merge::merge(base, overlay)
}

// -- Single-item build functions --

/// Build a single hero into a modifier string.
pub fn build_hero(hero: &ir::Hero) -> Result<String, CompilerError> {
    builder::hero_emitter::emit(hero)
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

// -- Single-item validation wrappers (build = structural check, xref = semantic) --

/// Validate a single hero by attempting to build it. If the build succeeds,
/// the hero is structurally valid. Returns an empty report on success,
/// or a report with a build error on failure.
pub fn validate_hero(hero: &ir::Hero) -> ValidationReport {
    let mut report = ValidationReport::default();
    if let Err(e) = build_hero(hero) {
        report.errors.push(Finding {
            rule_id: "E000".to_string(),
            severity: Severity::Error,
            modifier_name: Some(hero.mn_name.clone()),
            message: format!("Build failed: {}", e),
            field_path: Some(format!("hero[{}]", hero.mn_name)),
            suggestion: Some("Fix the hero definition so it can be built into a valid modifier".to_string()),
            source: Some(hero.source),
            ..Default::default()
        });
    }
    report
}

/// Validate a hero in the context of an existing IR (checks color conflicts, name duplicates).
/// Combines build validation with cross-reference checks.
pub fn validate_hero_in_context(hero: &ir::Hero, ir: &ModIR) -> ValidationReport {
    let mut report = validate_hero(hero);
    let context_report = xref::check_hero_in_context(hero, ir);
    report.merge(context_report);
    report
}

// -- Single-item phase functions --

/// Parse a phase string into a Phase IR struct.
pub fn parse_phase(content: &str) -> Result<ir::Phase, CompilerError> {
    extractor::phase_parser::parse_phase(content)
}

/// Emit a Phase IR struct back to its textmod string representation.
pub fn emit_phase(phase: &ir::Phase) -> Result<String, CompilerError> {
    builder::phase_emitter::emit_phase(phase)
}

/// Parse a reward tag string into a RewardTag struct.
pub fn parse_reward_tag(content: &str) -> Result<ir::RewardTag, CompilerError> {
    extractor::reward_parser::parse_reward_tag(content)
}

/// Emit a RewardTag back to its textmod string representation.
pub fn emit_reward_tag(tag: &ir::RewardTag) -> String {
    builder::reward_emitter::emit_reward_tag(tag)
}

/// Serialize a ModIR to JSON.
pub fn ir_to_json(ir: &ModIR) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(ir)
}

/// Deserialize a ModIR from JSON.
pub fn ir_from_json(json: &str) -> Result<ModIR, serde_json::Error> {
    serde_json::from_str(json)
}
