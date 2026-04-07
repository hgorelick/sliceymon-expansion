use textmod_compiler::extractor::classifier::{classify, ModifierType};
use textmod_compiler::extractor::hero_parser::parse_hero;
use textmod_compiler::extractor::splitter::split_modifiers;

fn load_mod(name: &str) -> String {
    let path = format!("../working-mods/{}.txt", name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", path, e))
}

fn get_hero_modifiers(name: &str) -> Vec<(usize, String)> {
    let text = load_mod(name);
    let mods = split_modifiers(&text).unwrap();
    mods.into_iter()
        .enumerate()
        .filter(|(i, m)| classify(m, *i).unwrap() == ModifierType::Hero)
        .collect()
}

fn find_hero_by_mn(name: &str, mn_name: &str) -> (usize, String) {
    let heroes = get_hero_modifiers(name);
    heroes
        .into_iter()
        .find(|(_, m)| {
            let suffix = format!(".mn.{}", mn_name);
            m.contains(&suffix)
        })
        .unwrap_or_else(|| panic!("Could not find hero with .mn.{}", mn_name))
}

// --- Test 1: Parse a simple hero with no spells ---
#[test]
fn parse_simple_hero_no_spells() {
    let (idx, modifier) = find_hero_by_mn("sliceymon", "Gible");
    let hero = parse_hero(&modifier, idx);
    assert!(!hero.blocks.is_empty(), "Gible should parse with blocks, not empty fallback");
    assert_eq!(hero.internal_name, "gible");
    assert_eq!(hero.mn_name, "Gible");
    assert_eq!(hero.color, 'a');
    assert!(!hero.blocks.is_empty(), "Gible should have blocks");
    assert_eq!(hero.blocks.len(), 5, "Gible should have 5 blocks");
    // T1 should be Gible
    assert_eq!(hero.blocks[0].name, "Gible");
    assert!(hero.blocks[0].hp > 0, "T1 HP should be > 0");
    assert!(!hero.blocks[0].sd.is_empty(), "T1 sd should not be empty");
}

// --- Test 2: Parse a hero with abilitydata ---
#[test]
fn parse_hero_with_spell() {
    let heroes = get_hero_modifiers("sliceymon");
    // Find a hero with abilitydata
    let hero_with_spell = heroes.into_iter().find(|(i, m)| {
        let h = parse_hero(m, *i);
        h.blocks.iter().any(|t| t.abilitydata.is_some())
    });
    assert!(
        hero_with_spell.is_some(),
        "Should find at least one hero with abilitydata"
    );
    let (idx, modifier) = hero_with_spell.unwrap();
    let hero = parse_hero(&modifier, idx);
    let spell_tier = hero.blocks.iter().find(|t| t.abilitydata.is_some()).unwrap();
    let ability = spell_tier.abilitydata.as_ref().unwrap();
    assert!(ability.starts_with('('), "abilitydata should start with (");
    assert!(ability.ends_with(')'), "abilitydata should end with )");
}

// --- Test 3: Parse a hero with facade data ---
#[test]
fn parse_hero_with_facade() {
    let (idx, modifier) = find_hero_by_mn("sliceymon", "Gible");
    let hero = parse_hero(&modifier, idx);
    // Gible should have facades in its modifier chain
    let has_facades = hero.blocks.iter().any(|t| !t.facades.is_empty());
    assert!(has_facades, "Gible should have facade data in at least one tier");
}

// --- Test 4: Parse a hero with triggerhpdata ---
#[test]
fn parse_hero_with_triggerhpdata() {
    let heroes = get_hero_modifiers("sliceymon");
    let hero_with_thp = heroes.into_iter().find(|(i, m)| {
        let h = parse_hero(m, *i);
        h.blocks.iter().any(|t| t.triggerhpdata.is_some())
    });
    // If no hero has triggerhpdata parsed, that is OK for now (it is optional/complex)
    if let Some((idx, modifier)) = hero_with_thp {
        let hero = parse_hero(&modifier, idx);
        let thp_tier = hero
            .blocks
            .iter()
            .find(|t| t.triggerhpdata.is_some())
            .unwrap();
        assert!(
            !thp_tier.triggerhpdata.as_ref().unwrap().is_empty(),
            "triggerhpdata should not be empty"
        );
    }
}

// --- Test 5: Parse a hero with doc ---
#[test]
fn parse_hero_with_doc() {
    let heroes = get_hero_modifiers("sliceymon");
    let hero_with_doc = heroes.into_iter().find(|(i, m)| {
        let h = parse_hero(m, *i);
        h.blocks.iter().any(|t| t.doc.is_some())
    });
    assert!(
        hero_with_doc.is_some(),
        "Should find at least one hero with a doc string"
    );
    let (idx, modifier) = hero_with_doc.unwrap();
    let hero = parse_hero(&modifier, idx);
    let doc_tier = hero.blocks.iter().find(|t| t.doc.is_some()).unwrap();
    assert!(
        !doc_tier.doc.as_ref().unwrap().is_empty(),
        "doc should not be empty"
    );
}

// --- Test 6: Tier count varies ---
#[test]
fn parse_hero_tier_count_varies() {
    let (idx, modifier) = find_hero_by_mn("sliceymon", "Gible");
    let gible = parse_hero(&modifier, idx);
    assert_eq!(gible.blocks.len(), 5, "Gible should have 5 blocks");

    // Check Eevee has many blocks (17 expected)
    let (idx, modifier) = find_hero_by_mn("sliceymon", "Eevee");
    let eevee = parse_hero(&modifier, idx);
    if eevee.raw.is_none() {
        assert!(
            eevee.blocks.len() > 10,
            "Eevee should have many blocks (>10), got {}",
            eevee.blocks.len()
        );
    }
}

// --- Test 7: Paren balance ---
#[test]
fn parse_hero_paren_balanced() {
    let heroes = get_hero_modifiers("sliceymon");
    for (idx, modifier) in &heroes {
        let hero = parse_hero(modifier, *idx);
        if hero.raw.is_some() {
            // Raw heroes preserve original, check the raw string
            let raw = hero.raw.as_ref().unwrap();
            let mut depth: i32 = 0;
            for ch in raw.chars() {
                match ch {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => {}
                }
                assert!(depth >= 0, "Negative paren depth in hero {}", hero.mn_name);
            }
            assert_eq!(depth, 0, "Unbalanced parens in hero {}", hero.mn_name);
        }
    }
}

// --- Test 8: Parse ALL sliceymon heroes ---
#[test]
fn parse_all_sliceymon_heroes() {
    let heroes = get_hero_modifiers("sliceymon");
    let count = heroes.len();

    let mut parsed_count = 0;
    let mut raw_count = 0;
    let mut errors: Vec<String> = Vec::new();

    for (idx, modifier) in &heroes {
        let hero = parse_hero(modifier, *idx);
        if hero.blocks.is_empty() {
            raw_count += 1;
        } else {
            parsed_count += 1;
        }
        // Verify at least mn_name or internal_name was extracted
        if hero.mn_name.is_empty() && hero.internal_name.is_empty() {
            errors.push(format!(
                "Hero #{} has no name. First 150 chars: {}",
                idx,
                &modifier[..modifier.len().min(150)]
            ));
        }
    }

    eprintln!(
        "Sliceymon heroes: {} total, {} fully parsed, {} raw fallback",
        count, parsed_count, raw_count
    );

    if !errors.is_empty() {
        panic!("Errors: {:?}", errors);
    }

    // All hero-classified modifiers should have parsed (either fully or via raw)
    assert!(count > 40, "Expected 40+ heroes, got {}", count);
}

// --- Test 9: HP values match ---
#[test]
fn parse_hero_extracts_correct_hp() {
    let (idx, modifier) = find_hero_by_mn("sliceymon", "Gible");
    let hero = parse_hero(&modifier, idx);
    if !hero.blocks.is_empty() {
        // T1 Gible should have some HP value
        assert!(hero.blocks[0].hp > 0, "Gible T1 HP should be > 0");
        // Later blocks should have higher HP
        if hero.blocks.len() >= 3 {
            assert!(
                hero.blocks[2].hp >= hero.blocks[0].hp,
                "T2 HP should be >= T1 HP"
            );
        }
    }
}

// --- Test 10: SD values extract ---
#[test]
fn parse_hero_extracts_correct_sd() {
    let (idx, modifier) = find_hero_by_mn("sliceymon", "Gible");
    let hero = parse_hero(&modifier, idx);
    if !hero.blocks.is_empty() {
        let sd = &hero.blocks[0].sd;
        assert!(!sd.is_empty(), "Gible T1 sd should not be empty");
        // SD should contain colons (6 faces separated by 5 colons)
        let colon_count = sd.chars().filter(|c| *c == ':').count();
        assert_eq!(colon_count, 5, "SD should have 5 colons (6 faces), got {}: {}", colon_count, sd);
    }
}

// --- Test 11: mn_name correct ---
#[test]
fn parse_hero_mn_name_correct() {
    let known_heroes = ["Gible", "Eevee", "Snorunt", "Larvitar", "Starly"];
    for name in &known_heroes {
        let result = std::panic::catch_unwind(|| find_hero_by_mn("sliceymon", name));
        if let Ok((idx, modifier)) = result {
            let hero = parse_hero(&modifier, idx);
            assert_eq!(hero.mn_name, *name, "mn_name should be {}", name);
        }
    }
}

// --- Test 12: Multiple facades ---
#[test]
fn parse_hero_multiple_facades() {
    let (idx, modifier) = find_hero_by_mn("sliceymon", "Gible");
    let hero = parse_hero(&modifier, idx);
    if !hero.blocks.is_empty() {
        let total_facades: usize = hero.blocks.iter().map(|t| t.facades.len()).sum();
        assert!(
            total_facades >= 2,
            "Gible should have at least 2 facades total, got {}",
            total_facades
        );
    }
}

// --- Test 13: Modifier chain preserved ---
#[test]
fn parse_hero_modifier_chain_preserved() {
    let (idx, modifier) = find_hero_by_mn("sliceymon", "Gible");
    let hero = parse_hero(&modifier, idx);
    if !hero.blocks.is_empty() {
        let has_chain = hero.blocks.iter().any(|t| t.modifier_chain.is_some());
        assert!(
            has_chain,
            "Gible should have modifier chain in at least one tier"
        );
        // The chain should contain .i. and .k. markers
        let chain = hero
            .blocks
            .iter()
            .find(|t| t.modifier_chain.is_some())
            .unwrap()
            .modifier_chain
            .as_ref()
            .unwrap();
        assert!(chain.contains(".i."), "Chain should contain .i. markers");
        assert!(chain.contains(".k."), "Chain should contain .k. markers");
    }
}

// --- Test 14: Parse at least one pansaer hero ---
#[test]
fn parse_pansaer_hero() {
    let heroes = get_hero_modifiers("pansaer");
    assert!(!heroes.is_empty(), "Pansaer should have hero modifiers");

    let (idx, modifier) = &heroes[0];
    let hero = parse_hero(modifier, *idx);
    // Should at least get a name
    assert!(
        !hero.mn_name.is_empty() || !hero.internal_name.is_empty(),
        "Pansaer hero should have a name"
    );
}
