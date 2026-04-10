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
            hp: Some(5),
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            color: None,
            sprite_name: name.to_string(),
            bare: false,
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
            img_data: None,
        }],
        removed: false,
        source: Source::Base,
    }
}

fn make_replica(name: &str) -> ReplicaItem {
    ReplicaItem {
        name: name.into(),
        container_name: "Ball".into(),
        template: "Hat".into(),
        hp: None,
        sd: DiceFaces::parse("0:0:0:0:0:0"),
        sprite_name: name.into(),
        color: None,
        tier: None,
        doc: None,
        speech: None,
        abilitydata: None,
        item_modifiers: None,
        sticker: None,
        toggle_flags: None,
        img_data: None,
        source: Source::Base,
    }
}

fn make_monster(name: &str) -> Monster {
    Monster {
        name: name.into(),
        base_template: "Slimelet".into(),
        floor_range: "1-3".into(),
        hp: Some(3),
        sd: Some(DiceFaces::parse("0:0:0:0:0:0")),
        sprite_name: Some(name.into()),
        color: None,
        doc: None,
        modifier_chain: None,
        balance: None,
        img_data: None,
        source: Source::Base,
    }
}

fn make_boss(name: &str) -> Boss {
    Boss {
        name: name.into(),
        level: Some(4),
        format: BossFormat::Standard,
        encounter_id: None,
        variants: vec![],
        doc: None,
        modifier_chain: None,
        source: Source::Base,
        event_body: None,
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
    replacement.blocks[0].hp = Some(99);
    let overlay = ModIR {
        heroes: vec![replacement],
        ..empty_ir()
    };
    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.heroes.len(), 1);
    assert_eq!(merged.heroes[0].color, 'b');
    assert_eq!(merged.heroes[0].blocks[0].hp, Some(99));
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
        replica_items: vec![ReplicaItem {
            name: "Ivysaur".to_string(),
            container_name: "Poke Ball".to_string(),
            tier: None,
            template: "Thief".to_string(),
            hp: None,
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            sprite_name: "Ivysaur".to_string(),
            color: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            img_data: None,
            doc: None,
            speech: None,
            abilitydata: None,
            source: Source::Base,
        }],
        ..empty_ir()
    };
    let overlay = ModIR {
        heroes: vec![make_hero("Torchic", 'k')],
        ..empty_ir()
    };
    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.heroes.len(), 3); // Gible + Eevee + Torchic
    assert_eq!(merged.replica_items.len(), 1); // Ivysaur preserved
    assert_eq!(merged.replica_items[0].name, "Ivysaur");
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
            img_data: None,
            source: Source::Base,
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
                img_data: None,
                source: Source::Base,
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
                img_data: None,
                source: Source::Base,
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
            StructuralModifier::new(StructuralType::PartyConfig, "old-party".to_string()),
            StructuralModifier::new(StructuralType::Dialog, "old-dialog".to_string()),
        ],
        ..empty_ir()
    };
    let overlay = ModIR {
        structural: vec![StructuralModifier::new(
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
    assert_eq!(party.body(), "new-party");
    // Dialog preserved
    let dialog = merged
        .structural
        .iter()
        .find(|s| s.modifier_type == StructuralType::Dialog)
        .unwrap();
    assert_eq!(dialog.body(), "old-dialog");
}

// --- Test 6b: Merge structural replaces by (type, name) pair ---
#[test]
fn merge_structural_replaces_by_type_and_name() {
    let mut base_party_a = StructuralModifier::new(
        StructuralType::PartyConfig,
        "old-party-a".to_string(),
    );
    base_party_a.name = Some("TeamA".to_string());

    let mut base_party_b = StructuralModifier::new(
        StructuralType::PartyConfig,
        "old-party-b".to_string(),
    );
    base_party_b.name = Some("TeamB".to_string());

    let base = ModIR {
        structural: vec![base_party_a, base_party_b],
        ..empty_ir()
    };

    let mut overlay_party_b = StructuralModifier::new(
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
    assert_eq!(team_a.body(), "old-party-a");

    // TeamB replaced with new content
    let team_b = merged
        .structural
        .iter()
        .find(|s| s.name.as_deref() == Some("TeamB"))
        .unwrap();
    assert_eq!(team_b.body(), "new-party-b");
}

// --- Test 8: Full expansion build test ---
#[test]
fn expansion_build_from_base_plus_overlay() {
    let base = ModIR {
        heroes: vec![make_hero("Gible", 'a')],
        structural: vec![StructuralModifier::new(
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
        replica_items: vec![ReplicaItem {
            name: "Raichu".to_string(),
            container_name: "Ball".to_string(),
            tier: None,
            template: "Thief".to_string(),
            hp: None,
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            sprite_name: "Raichu".to_string(),
            color: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            img_data: None,
            doc: None,
            speech: None,
            abilitydata: None,
            source: Source::Base,
        }],
        ..empty_ir()
    };

    let merged = textmod_compiler::merge(base, empty_ir()).unwrap();

    // Verify no name appears in both heroes and replica_items
    let hero_names: Vec<&str> = merged.heroes.iter().map(|h| h.mn_name.as_str()).collect();
    let replica_names: Vec<&str> = merged.replica_items.iter().map(|r| r.name.as_str()).collect();

    for name in &hero_names {
        assert!(
            !replica_names.contains(name),
            "'{}' appears in both heroes and replica_items",
            name
        );
    }
}

// ---------------------------------------------------------------------------
// Overlay + Provenance tests (Chunk 12)
// ---------------------------------------------------------------------------

#[test]
fn overlay_replaces_hero_by_name() {
    let mut base = empty_ir();
    base.heroes.push(make_hero("Gible", 'a'));

    let mut overlay = empty_ir();
    let mut new_gible = make_hero("Gible", 'a');
    new_gible.blocks[0].hp = Some(99);
    overlay.heroes.push(new_gible);

    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.heroes.len(), 1);
    assert_eq!(merged.heroes[0].blocks[0].hp, Some(99));
}

#[test]
fn overlay_adds_new_hero() {
    let mut base = empty_ir();
    base.heroes.push(make_hero("Gible", 'a'));
    base.heroes.push(make_hero("Axew", 'b'));
    base.heroes.push(make_hero("Deino", 'c'));

    let mut overlay = empty_ir();
    overlay.heroes.push(make_hero("Dratini", 'd'));

    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.heroes.len(), 4);
}

#[test]
fn overlay_accepts_json() {
    let mut ir = empty_ir();
    ir.heroes.push(make_hero("Gible", 'a'));
    let json = textmod_compiler::ir_to_json(&ir).unwrap();
    let deserialized = textmod_compiler::ir_from_json(&json).unwrap();
    assert_eq!(deserialized.heroes.len(), 1);
    assert_eq!(deserialized.heroes[0].mn_name, "Gible");
}

#[test]
fn overlay_handles_all_types() {
    let mut base = empty_ir();
    base.heroes.push(make_hero("Gible", 'a'));

    let mut overlay = empty_ir();
    overlay.heroes.push(make_hero("Axew", 'b'));
    overlay.replica_items.push(make_replica("Pikachu"));
    overlay.monsters.push(make_monster("Wooper"));
    overlay.bosses.push(make_boss("Mewtwo"));

    let merged = textmod_compiler::merge(base, overlay).unwrap();
    assert_eq!(merged.heroes.len(), 2);
    assert_eq!(merged.replica_items.len(), 1);
    assert_eq!(merged.monsters.len(), 1);
    assert_eq!(merged.bosses.len(), 1);
}

#[test]
fn provenance_tracks_base_vs_custom() {
    // Extracted items have Source::Base (default)
    let mut ir = empty_ir();
    ir.heroes.push(make_hero("Gible", 'a'));
    assert_eq!(ir.heroes[0].source, Source::Base);

    // CRUD adds set Source::Custom
    ir.add_hero(make_hero("Axew", 'b')).unwrap();
    assert_eq!(ir.heroes[1].source, Source::Custom);

    // Overlay merge sets Source::Overlay
    let mut overlay = empty_ir();
    overlay.heroes.push(make_hero("Deino", 'c'));
    let merged = textmod_compiler::merge(ir, overlay).unwrap();
    assert_eq!(merged.heroes[2].source, Source::Overlay);
}

#[test]
fn provenance_survives_serialization() {
    let mut ir = empty_ir();
    let mut hero = make_hero("Gible", 'a');
    hero.source = Source::Custom;
    ir.heroes.push(hero);

    let json = textmod_compiler::ir_to_json(&ir).unwrap();
    let restored = textmod_compiler::ir_from_json(&json).unwrap();
    assert_eq!(restored.heroes[0].source, Source::Custom);
}
