pub mod splitter;
pub mod classifier;
pub mod hero_parser;
pub mod replica_item_parser;
pub mod monster_parser;
pub mod boss_parser;
pub mod fight_parser;
pub mod structural_parser;
pub mod chain_parser;
pub mod level_scope_parser;
pub mod phase_parser;
pub mod reward_parser;
pub mod richtext_parser;

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
            ModifierType::HeroPoolBase => {
                structural.push(make_structural(StructuralType::HeroPoolBase, modifier.clone()));
            }
            ModifierType::ItemPool => {
                // Route ItemPool through the trigger-IR stub extractor.
                // The stub returns zero new `ReplicaItem`s plus a single
                // `NonSummon { name: "", tier: None, content: <whole body> }`
                // sentinel whose emitter-side sentinel path re-emits the body
                // verbatim for byte-equal round-trip. 8b replaces the stub
                // body with the real per-entry classifier.
                let extraction = replica_item_parser::extract_from_itempool(
                    modifier,
                    i,
                    replica_items.len(),
                )?;
                replica_items.extend(extraction.new_replica_items);
                let name = structural_parser::extract_structural_name(modifier);
                structural.push(StructuralModifier {
                    modifier_type: StructuralType::ItemPool,
                    name,
                    content: StructuralContent::ItemPool {
                        items: extraction.items,
                    },
                    derived: false,
                    source: Source::Base,
                });
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
            ModifierType::PhaseModifier => {
                structural.push(make_structural(StructuralType::PhaseModifier, modifier.clone()));
            }
            ModifierType::Choosable => {
                structural.push(make_structural(StructuralType::Choosable, modifier.clone()));
            }
            ModifierType::ValueModifier => {
                structural.push(make_structural(StructuralType::ValueModifier, modifier.clone()));
            }
            ModifierType::HiddenModifier => {
                structural.push(make_structural(StructuralType::HiddenModifier, modifier.clone()));
            }
            ModifierType::FightModifier => {
                structural.push(make_structural(StructuralType::FightModifier, modifier.clone()));
            }
        }
    }

    Ok(ModIR {
        heroes,
        replica_items,
        monsters,
        bosses,
        structural,
        warnings: Vec::new(),
    })
}
