use std::collections::HashMap;

use textmod_compiler::ir::*;

fn empty_ir() -> ModIR {
    ModIR::empty()
}

fn make_hero(name: &str, color: char) -> Hero {
    Hero {
        internal_name: name.to_lowercase(),
        mn_name: name.to_string(),
        color,
        format: HeroFormat::Sliceymon,
        blocks: vec![HeroBlock {
            template: "Thief".to_string(),
            tier: None,
            hp: 5,
            sd: "0:0:0:0:0:0".to_string(),
            color: None,
            sprite_name: name.to_string(),
            speech: format!("{}!", name),
            name: name.to_string(),
            doc: None,
            abilitydata: None,
            triggerhpdata: None,
            hue: None,
            modifier_chain: None,
            facades: vec![],
            items_inside: None,
            items_outside: None,
        }],
        removed: false,
        raw: None,
    }
}

// --- Test 1: Merge adds new hero ---
#[test]
fn merge_adds_new_hero() {
    let base = ModIR {
        heroes: vec![make_hero("Gible", 'a')],
        ..empty_ir()
    };
    let overlay = ModIR {
        heroes: vec![make_hero("Torchic", 'k')],
        ..empty_ir()
    };
    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.heroes.len(), 2);
    assert!(merged.heroes.iter().any(|h| h.mn_name == "Gible"));
    assert!(merged.heroes.iter().any(|h| h.mn_name == "Torchic"));
}

// --- Test 2: Merge replaces existing hero ---
#[test]
fn merge_replaces_existing_hero() {
    let base = ModIR {
        heroes: vec![make_hero("Gible", 'a')],
        ..empty_ir()
    };
    let mut replacement = make_hero("Gible", 'b'); // Different color
    replacement.blocks[0].hp = 99;
    let overlay = ModIR {
        heroes: vec![replacement],
        ..empty_ir()
    };
    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.heroes.len(), 1);
    assert_eq!(merged.heroes[0].color, 'b');
    assert_eq!(merged.heroes[0].blocks[0].hp, 99);
}

// --- Test 3: Merge removes hero ---
#[test]
fn merge_removes_hero() {
    let base = ModIR {
        heroes: vec![make_hero("Gible", 'a'), make_hero("Eevee", 'c')],
        ..empty_ir()
    };
    let mut removed = make_hero("Gible", 'a');
    removed.removed = true;
    let overlay = ModIR {
        heroes: vec![removed],
        ..empty_ir()
    };
    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.heroes.len(), 1);
    assert_eq!(merged.heroes[0].mn_name, "Eevee");
}

// --- Test 4: Merge preserves unmodified ---
#[test]
fn merge_preserves_unmodified() {
    let base = ModIR {
        heroes: vec![make_hero("Gible", 'a'), make_hero("Eevee", 'c')],
        captures: vec![Capture {
            pokemon: "Ivysaur".to_string(),
            ball_name: "Poke Ball".to_string(),
            ball_tier: None,
            template: "Thief".to_string(),
            hp: None,
            sd: "0:0:0:0:0:0".to_string(),
            sprite_name: "Ivysaur".to_string(),
            color: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            raw: Some("raw-capture".to_string()),
        }],
        ..empty_ir()
    };
    let overlay = ModIR {
        heroes: vec![make_hero("Torchic", 'k')],
        ..empty_ir()
    };
    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.heroes.len(), 3); // Gible + Eevee + Torchic
    assert_eq!(merged.captures.len(), 1); // Ivysaur preserved
    assert_eq!(merged.captures[0].pokemon, "Ivysaur");
}

// --- Test 5: Merge monsters ---
#[test]
fn merge_adds_and_replaces_monsters() {
    let base = ModIR {
        monsters: vec![Monster {
            name: "Wooper".to_string(),
            base_template: "Slimelet".to_string(),
            floor_range: "1-3".to_string(),
            hp: Some(4),
            sd: None,
            sprite_name: None,
            color: None,
            doc: None,
            modifier_chain: None,
            balance: None,
            raw: Some("wooper-raw".to_string()),
        }],
        ..empty_ir()
    };
    let overlay = ModIR {
        monsters: vec![
            Monster {
                name: "Wooper".to_string(),
                base_template: "Slimelet".to_string(),
                floor_range: "1-3".to_string(),
                hp: Some(6), // Changed HP
                sd: None,
                sprite_name: None,
                color: None,
                doc: None,
                modifier_chain: None,
                balance: None,
                raw: Some("wooper-raw-v2".to_string()),
            },
            Monster {
                name: "Rattata".to_string(),
                base_template: "Bones".to_string(),
                floor_range: "4-6".to_string(),
                hp: Some(3),
                sd: None,
                sprite_name: None,
                color: None,
                doc: None,
                modifier_chain: None,
                balance: None,
                raw: Some("rattata-raw".to_string()),
            },
        ],
        ..empty_ir()
    };
    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.monsters.len(), 2);
    assert_eq!(merged.monsters[0].hp, Some(6)); // Wooper replaced
    assert_eq!(merged.monsters[1].name, "Rattata"); // Rattata added
}

// --- Test 6: Merge structural replaces by type ---
#[test]
fn merge_structural_replaces_by_type() {
    let base = ModIR {
        structural: vec![
            StructuralModifier::new_raw(StructuralType::PartyConfig, "old-party".to_string()),
            StructuralModifier::new_raw(StructuralType::Dialog, "old-dialog".to_string()),
        ],
        ..empty_ir()
    };
    let overlay = ModIR {
        structural: vec![StructuralModifier::new_raw(
            StructuralType::PartyConfig,
            "new-party".to_string(),
        )],
        ..empty_ir()
    };
    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.structural.len(), 2);
    let party = merged
        .structural
        .iter()
        .find(|s| s.modifier_type == StructuralType::PartyConfig)
        .unwrap();
    assert_eq!(party.raw, "new-party");
    // Dialog preserved
    let dialog = merged
        .structural
        .iter()
        .find(|s| s.modifier_type == StructuralType::Dialog)
        .unwrap();
    assert_eq!(dialog.raw, "old-dialog");
}

// --- Test 6b: Merge structural replaces by (type, name) pair ---
#[test]
fn merge_structural_replaces_by_type_and_name() {
    let mut base_party_a = StructuralModifier::new_raw(
        StructuralType::PartyConfig,
        "old-party-a".to_string(),
    );
    base_party_a.name = Some("TeamA".to_string());

    let mut base_party_b = StructuralModifier::new_raw(
        StructuralType::PartyConfig,
        "old-party-b".to_string(),
    );
    base_party_b.name = Some("TeamB".to_string());

    let base = ModIR {
        structural: vec![base_party_a, base_party_b],
        ..empty_ir()
    };

    let mut overlay_party_b = StructuralModifier::new_raw(
        StructuralType::PartyConfig,
        "new-party-b".to_string(),
    );
    overlay_party_b.name = Some("TeamB".to_string());

    let overlay = ModIR {
        structural: vec![overlay_party_b],
        ..empty_ir()
    };

    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.structural.len(), 2);

    // TeamA preserved with old content
    let team_a = merged
        .structural
        .iter()
        .find(|s| s.name.as_deref() == Some("TeamA"))
        .unwrap();
    assert_eq!(team_a.raw, "old-party-a");

    // TeamB replaced with new content
    let team_b = merged
        .structural
        .iter()
        .find(|s| s.name.as_deref() == Some("TeamB"))
        .unwrap();
    assert_eq!(team_b.raw, "new-party-b");
}

// --- Test 8: Full expansion build test ---
#[test]
fn expansion_build_from_base_plus_overlay() {
    let base = ModIR {
        heroes: vec![make_hero("Gible", 'a')],
        structural: vec![StructuralModifier::new_raw(
            StructuralType::PartyConfig,
            "=party.content".to_string(),
        )],
        ..empty_ir()
    };
    let overlay = ModIR {
        heroes: vec![make_hero("Torchic", 'k')],
        ..empty_ir()
    };
    let merged = textmod_compiler::merge(base, overlay).unwrap();

    let mut sprites = HashMap::new();
    sprites.insert("Gible".to_string(), "GIBLE_DATA".to_string());
    sprites.insert("Torchic".to_string(), "TORCHIC_DATA".to_string());

    let output = textmod_compiler::build(&merged, &sprites).unwrap();
    assert!(output.contains("=party.content"));
    assert!(output.contains("heropool.")); // At least one hero emitted
}

// --- Test 9: No duplicate pokemon across pools after merge ---
#[test]
fn expansion_no_duplicate_pokemon() {
    let base = ModIR {
        heroes: vec![make_hero("Pikachu", 'e')],
        captures: vec![Capture {
            pokemon: "Raichu".to_string(),
            ball_name: "Ball".to_string(),
            ball_tier: None,
            template: "Thief".to_string(),
            hp: None,
            sd: "0:0:0:0:0:0".to_string(),
            sprite_name: "Raichu".to_string(),
            color: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            raw: Some("raichu-raw".to_string()),
        }],
        ..empty_ir()
    };

    let merged = textmod_compiler::merge(base, empty_ir()).unwrap();

    // Verify no pokemon name appears in both heroes and captures
    let hero_names: Vec<&str> = merged.heroes.iter().map(|h| h.mn_name.as_str()).collect();
    let capture_names: Vec<&str> = merged.captures.iter().map(|c| c.pokemon.as_str()).collect();

    for name in &hero_names {
        assert!(
            !capture_names.contains(name),
            "Pokemon '{}' appears in both heroes and captures",
            name
        );
    }
}
