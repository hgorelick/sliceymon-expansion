use textmod_compiler::extractor::classifier::{classify, ModifierType};
use textmod_compiler::extractor::monster_parser;
use textmod_compiler::extractor::boss_parser;
use textmod_compiler::extractor::splitter::split_modifiers;

fn load_mod(name: &str) -> String {
    let path = format!("../working-mods/{}.txt", name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", path, e))
}

fn get_modifiers_of_type(name: &str, mtype: ModifierType) -> Vec<(usize, String)> {
    let text = load_mod(name);
    let mods = split_modifiers(&text).unwrap();
    mods.into_iter()
        .enumerate()
        .filter(|(i, m)| classify(m, *i).unwrap() == mtype)
        .collect()
}

// --- Monster tests ---

#[test]
fn parse_monster_extracts_name() {
    let monsters = get_modifiers_of_type("sliceymon", ModifierType::Monster);
    assert!(!monsters.is_empty(), "Should have monsters");
    let (idx, modifier) = &monsters[0];
    let mon = monster_parser::parse_monster(modifier, *idx);
    assert!(!mon.name.is_empty(), "Monster name should not be empty");
}

#[test]
fn parse_monster_extracts_floor_range() {
    let monsters = get_modifiers_of_type("sliceymon", ModifierType::Monster);
    let has_range = monsters.iter().any(|(i, m)| {
        let mon = monster_parser::parse_monster(m, *i);
        !mon.floor_range.is_empty()
    });
    assert!(has_range, "At least one monster should have a floor range");
}

#[test]
fn parse_monster_extracts_template() {
    let monsters = get_modifiers_of_type("sliceymon", ModifierType::Monster);
    let has_template = monsters.iter().any(|(i, m)| {
        let mon = monster_parser::parse_monster(m, *i);
        !mon.base_template.is_empty()
    });
    assert!(has_template, "At least one monster should have a base template");
}

// --- Boss tests ---

#[test]
fn parse_boss_extracts_name() {
    let bosses = get_modifiers_of_type("sliceymon", ModifierType::Boss);
    assert!(!bosses.is_empty(), "Should have bosses");
    let (idx, modifier) = &bosses[0];
    let boss = boss_parser::parse_boss(modifier, *idx);
    assert!(!boss.name.is_empty(), "Boss name should not be empty");
}

#[test]
fn parse_boss_extracts_level() {
    let bosses = get_modifiers_of_type("sliceymon", ModifierType::Boss);
    let has_level = bosses.iter().any(|(i, m)| {
        let boss = boss_parser::parse_boss(m, *i);
        boss.level.is_some()
    });
    assert!(has_level, "At least one boss should have a level");
}

// --- Round-trip tests ---

#[test]
fn roundtrip_monsters_raw() {
    let monsters = get_modifiers_of_type("sliceymon", ModifierType::Monster);
    for (idx, modifier) in &monsters {
        let mon = monster_parser::parse_monster(modifier, *idx);
        assert_eq!(mon.raw.as_ref().unwrap(), modifier, "Monster raw should match original");
    }
}

#[test]
fn roundtrip_bosses_raw() {
    let bosses = get_modifiers_of_type("sliceymon", ModifierType::Boss);
    for (idx, modifier) in &bosses {
        let boss = boss_parser::parse_boss(modifier, *idx);
        assert_eq!(boss.raw.as_ref().unwrap(), modifier, "Boss raw should match original");
    }
}

#[test]
fn parse_punpuns_monsters_no_panic() {
    let monsters = get_modifiers_of_type("punpuns", ModifierType::Monster);
    assert!(!monsters.is_empty(), "Punpuns should have monsters");
    for (idx, modifier) in &monsters {
        // Should not panic
        let _mon = monster_parser::parse_monster(modifier, *idx);
    }
}
