//! Dump raw source text of every monster, boss, and Unknown structural
//! from each working mod to /tmp files for offline analysis during refactor.

use std::fs;
use std::path::PathBuf;

use textmod_compiler::extractor::classifier::{classify, ModifierType};
use textmod_compiler::extractor::splitter::split_modifiers;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("working-mods");
    for name in &["sliceymon", "punpuns", "pansaer", "community"] {
        let text = fs::read_to_string(root.join(format!("{}.txt", name))).unwrap();
        let mods = match split_modifiers(&text) {
            Ok(m) => m,
            Err(e) => { eprintln!("{}: split err: {}", name, e); continue; }
        };

        let mut monsters: Vec<String> = Vec::new();
        let mut bosses: Vec<String> = Vec::new();
        let mut boss_enc: Vec<String> = Vec::new();
        let mut structurals: Vec<String> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        for (i, m) in mods.iter().enumerate() {
            match classify(m, i) {
                Ok(c) => {
                    let line = format!("[idx={}, class={:?}]\n{}\n", i, c, m);
                    match c {
                        ModifierType::Monster => monsters.push(line),
                        ModifierType::Boss => bosses.push(line),
                        ModifierType::BossEncounter => boss_enc.push(line),
                        ModifierType::HeroPoolBase | ModifierType::ItemPool |
                        ModifierType::PartyConfig | ModifierType::EventModifier |
                        ModifierType::Dialog | ModifierType::Selector |
                        ModifierType::GenSelect | ModifierType::BossModifier |
                        ModifierType::LevelUpAction | ModifierType::PoolReplacement |
                        ModifierType::Difficulty | ModifierType::ArtCredits |
                        ModifierType::EndScreen | ModifierType::PhaseModifier |
                        ModifierType::Choosable | ModifierType::ValueModifier |
                        ModifierType::HiddenModifier | ModifierType::FightModifier => structurals.push(line),
                        _ => {}
                    }
                }
                Err(e) => {
                    errors.push(format!("[idx={}, classify error: {}]\n{}\n", i, e, m));
                }
            }
        }

        let out_dir = PathBuf::from("/tmp").join("mod_bodies").join(name);
        fs::create_dir_all(&out_dir).unwrap();
        fs::write(out_dir.join("monsters.txt"), monsters.join("\n---\n")).unwrap();
        fs::write(out_dir.join("bosses_standard.txt"), bosses.join("\n---\n")).unwrap();
        fs::write(out_dir.join("bosses_encounter.txt"), boss_enc.join("\n---\n")).unwrap();
        fs::write(out_dir.join("structurals.txt"), structurals.join("\n---\n")).unwrap();
        fs::write(out_dir.join("classify_errors.txt"), errors.join("\n---\n")).unwrap();
        println!("{}: wrote to {}/", name, out_dir.display());
    }
}
