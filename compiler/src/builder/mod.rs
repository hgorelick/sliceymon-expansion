pub mod hero_emitter;
pub mod structural_emitter;
pub mod replica_item_emitter;
pub mod monster_emitter;
pub mod boss_emitter;
pub mod fight_emitter;
pub mod chain_emitter;
pub mod reward_emitter;
pub mod phase_emitter;
pub mod derived;

use crate::error::CompilerError;
use crate::ir::{ModIR, StructuralType};

/// Build a textmod string from a ModIR.
///
/// Assembly order matches sliceymon convention:
/// party, events, dialogs, selectors, heropool base, heroes, level-up,
/// items, captures, legendaries, monsters, bosses, boss modifiers,
/// gen select, difficulty, end screen, art credits
///
/// Output format: one modifier per line, comma-terminated, with blank spacer lines.
pub fn build(ir: &ModIR) -> Result<String, CompilerError> {
    // Type-based assembly for all mods.
    let mut modifiers: Vec<String> = Vec::new();

    // 1. Party config
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::PartyConfig) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 2. Event modifiers
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::EventModifier) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 3. Dialogs
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::Dialog) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 4. Selectors
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::Selector) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 5. HeroPool base (template list)
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::HeroPoolBase) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 6. Heroes
    for hero in &ir.heroes {
        modifiers.push(hero_emitter::emit(hero)?);
    }

    // 7. Level-up action
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::LevelUpAction) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 8. Item pools
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::ItemPool) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 9. Replica items (captures, legendaries, etc.)
    for item in &ir.replica_items {
        modifiers.push(replica_item_emitter::emit(item)?);
    }

    // 11. Monsters
    for mon in &ir.monsters {
        modifiers.push(monster_emitter::emit_monster(mon)?);
    }

    // 12. Bosses
    for boss in &ir.bosses {
        modifiers.push(boss_emitter::emit_boss(boss)?);
    }

    // 12b. Pool replacements (structural)
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::PoolReplacement) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 13. Boss modifiers
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::BossModifier) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 14. Gen select
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::GenSelect) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 15. Difficulty
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::Difficulty) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 16. End screen
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::EndScreen) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 17. Art credits
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::ArtCredits) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 18. Phase modifiers
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::PhaseModifier) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 19. Choosables
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::Choosable) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 20. Value modifiers
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::ValueModifier) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 21. Hidden modifiers
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::HiddenModifier) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 22. Fight modifiers
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::FightModifier) {
        modifiers.push(structural_emitter::emit(s));
    }

    // Join with comma + blank line (sliceymon format)
    let output = modifiers.join(",\n\n");

    // Trailing comma
    let output = if output.is_empty() {
        output
    } else {
        format!("{},", output)
    };

    Ok(output)
}

/// Build a complete textmod from an IR, auto-generating derived structurals
/// (character selection, hero pool base) if not explicitly present.
///
/// Use this for programmatic IR creation where the IR may not have
/// explicit structural modifiers. For round-trip builds from existing mods,
/// use `build()` instead.
pub fn build_complete(ir: &ModIR) -> Result<String, CompilerError> {
    let mut ir = ir.clone();

    if !ir.heroes.is_empty() {
        if !ir.structural.iter().any(|s| s.modifier_type == StructuralType::Selector) {
            ir.structural.push(derived::generate_char_selection(&ir.heroes));
        }
        if !ir.structural.iter().any(|s| s.modifier_type == StructuralType::HeroPoolBase) {
            ir.structural.push(derived::generate_hero_pool_base(&ir.heroes));
        }
    }

    build(&ir)
}
