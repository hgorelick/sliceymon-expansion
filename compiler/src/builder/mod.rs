pub mod hero_emitter;
pub mod structural_emitter;
pub mod capture_emitter;
pub mod monster_emitter;
pub mod boss_emitter;

use std::collections::HashMap;

use crate::error::CompilerError;
use crate::ir::{ModIR, StructuralType};

/// Build a textmod string from a ModIR and sprite mappings.
///
/// Assembly order matches sliceymon convention:
/// party, events, dialogs, selectors, heropool base, heroes, level-up,
/// items, captures, legendaries, monsters, bosses, boss modifiers,
/// gen select, difficulty, end screen, art credits
///
/// Output format: one modifier per line, comma-terminated, with blank spacer lines.
pub fn build(ir: &ModIR, sprites: &HashMap<String, String>) -> Result<String, CompilerError> {
    // If original modifier order is preserved (from extraction), emit in that order.
    // This ensures the game sees modifiers in the exact same sequence as the original.
    if let Some(ref originals) = ir.original_modifiers {
        let output = originals.join(",\n\n");
        return Ok(if output.is_empty() {
            output
        } else {
            format!("{},", output)
        });
    }

    // Otherwise, use type-based assembly for hand-authored or merged mods.
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
        modifiers.push(hero_emitter::emit(hero, sprites)?);
    }

    // 7. Level-up action
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::LevelUpAction) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 8. Item pools
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::ItemPool) {
        modifiers.push(structural_emitter::emit(s));
    }

    // 9. Captures
    for cap in &ir.captures {
        modifiers.push(capture_emitter::emit_capture(cap)?);
    }

    // 10. Legendaries
    for leg in &ir.legendaries {
        modifiers.push(capture_emitter::emit_legendary(leg)?);
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

    // 18. Unknown structural (appended at end)
    for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::Unknown) {
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
