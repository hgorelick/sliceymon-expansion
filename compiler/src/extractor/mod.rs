pub mod splitter;
pub mod classifier;
pub mod hero_parser;
pub mod replica_item_parser;
pub mod monster_parser;
pub mod boss_parser;
pub mod structural_parser;

use crate::error::CompilerError;
use crate::ir::*;
use classifier::ModifierType;

/// Build a StructuralModifier with parsed name and content.
fn make_structural(stype: StructuralType, raw: String) -> StructuralModifier {
    let name = structural_parser::extract_structural_name(&raw);
    let content = structural_parser::parse_structural_content(&stype, &raw);
    StructuralModifier {
        modifier_type: stype,
        name,
        content,
        derived: false,
        source: Source::Base,
    }
}

/// Extract a textmod string into a structured ModIR.
pub fn extract(textmod: &str) -> Result<ModIR, CompilerError> {
    let modifier_strings = splitter::split_modifiers(textmod)?;

    let mut heroes = Vec::new();
    let mut replica_items = Vec::new();
    let mut monsters = Vec::new();
    let mut bosses = Vec::new();
    let mut structural = Vec::new();

    for (i, modifier) in modifier_strings.iter().enumerate() {
        let mtype = classifier::classify(modifier, i)?;

        match mtype {
            ModifierType::Hero => {
                let hero = hero_parser::parse_hero(modifier, i);
                heroes.push(hero);
            }
            ModifierType::Monster => {
                monsters.push(monster_parser::parse_monster(modifier, i));
            }
            ModifierType::Boss => {
                bosses.push(boss_parser::parse_boss(modifier, i));
            }
            ModifierType::BossEncounter => {
                bosses.push(boss_parser::parse_encounter(modifier, i));
            }
            ModifierType::ReplicaItem => {
                replica_items.push(replica_item_parser::parse_simple(modifier, i));
            }
            ModifierType::ReplicaItemWithAbility => {
                replica_items.push(replica_item_parser::parse_with_ability(modifier, i));
            }
            ModifierType::HeroPoolBase => {
                structural.push(make_structural(StructuralType::HeroPoolBase, modifier.clone()));
            }
            ModifierType::ItemPool => {
                structural.push(make_structural(StructuralType::ItemPool, modifier.clone()));
            }
            ModifierType::PartyConfig => {
                structural.push(make_structural(StructuralType::PartyConfig, modifier.clone()));
            }
            ModifierType::EventModifier => {
                structural.push(make_structural(StructuralType::EventModifier, modifier.clone()));
            }
            ModifierType::Dialog => {
                structural.push(make_structural(StructuralType::Dialog, modifier.clone()));
            }
            ModifierType::Selector => {
                structural.push(make_structural(StructuralType::Selector, modifier.clone()));
            }
            ModifierType::GenSelect => {
                structural.push(make_structural(StructuralType::GenSelect, modifier.clone()));
            }
            ModifierType::BossModifier => {
                structural.push(make_structural(StructuralType::BossModifier, modifier.clone()));
            }
            ModifierType::LevelUpAction => {
                structural.push(make_structural(StructuralType::LevelUpAction, modifier.clone()));
            }
            ModifierType::PoolReplacement => {
                structural.push(make_structural(StructuralType::PoolReplacement, modifier.clone()));
            }
            ModifierType::Difficulty => {
                structural.push(make_structural(StructuralType::Difficulty, modifier.clone()));
            }
            ModifierType::ArtCredits => {
                structural.push(make_structural(StructuralType::ArtCredits, modifier.clone()));
            }
            ModifierType::EndScreen => {
                structural.push(make_structural(StructuralType::EndScreen, modifier.clone()));
            }
            ModifierType::Unknown => {
                structural.push(make_structural(StructuralType::Unknown, modifier.clone()));
            }
        }
    }

    Ok(ModIR {
        heroes,
        replica_items,
        monsters,
        bosses,
        structural,
    })
}
