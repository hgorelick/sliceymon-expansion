use std::collections::HashMap;

use textmod_compiler::builder::hero_emitter;
use textmod_compiler::ir::*;
use textmod_compiler::ir::HeroFormat;

fn test_sprites() -> HashMap<String, String> {
    let mut sprites = HashMap::new();
    sprites.insert("Gible".to_string(), "GIBLE_SPRITE_DATA".to_string());
    sprites.insert("Gabite".to_string(), "GABITE_SPRITE_DATA".to_string());
    sprites.insert("Garchomp".to_string(), "GARCHOMP_SPRITE_DATA".to_string());
    sprites.insert("Torchic".to_string(), "TORCHIC_SPRITE_DATA".to_string());
    sprites.insert("Combusken".to_string(), "COMBUSKEN_SPRITE_DATA".to_string());
    sprites.insert("Blaziken".to_string(), "BLAZIKEN_SPRITE_DATA".to_string());
    sprites
}

fn simple_hero() -> Hero {
    Hero {
        internal_name: "gible".to_string(),
        mn_name: "Gible".to_string(),
        color: 'a',
        format: HeroFormat::Sliceymon,
        blocks: vec![
            HeroBlock {
                template: "Lost".to_string(),
                tier: None,
                hp: Some(5),
                sd: DiceFaces::parse("170-3:158-1:158-1:158-1:43:0"),
                color: None,
                sprite_name: "Gible".to_string(),
                speech: "Gib!~Gible!".to_string(),
                name: "Gible".to_string(),
                doc: None,
                abilitydata: None,
                triggerhpdata: None,
                hue: None,
                modifier_chain: Some(ModifierChain::parse(".i.left.k.scared#facade.bas170:55")),
                facades: vec!["bas170:55".to_string()],
                items_inside: None,
                items_outside: None,
                img_data: None,
                bare: false,
            },
            HeroBlock {
                template: "Lost".to_string(),
                tier: Some(2),
                hp: Some(7),
                sd: DiceFaces::parse("170-3:158-2:158-2:43-1:0:0"),
                color: None,
                sprite_name: "Gabite".to_string(),
                speech: "Gab!~Gabite!".to_string(),
                name: "Gabite".to_string(),
                doc: None,
                abilitydata: None,
                triggerhpdata: None,
                hue: None,
                modifier_chain: None,
                facades: vec![],
                items_inside: None,
                items_outside: None,
                img_data: None,
                bare: false,
            },
            HeroBlock {
                template: "Lost".to_string(),
                tier: Some(3),
                hp: Some(9),
                sd: DiceFaces::parse("170-4:158-3:158-3:43-2:0:0"),
                color: None,
                sprite_name: "Garchomp".to_string(),
                speech: "GARCHOMP!".to_string(),
                name: "Garchomp".to_string(),
                doc: Some("Rough Skin".to_string()),
                abilitydata: None,
                triggerhpdata: None,
                hue: None,
                modifier_chain: None,
                facades: vec![],
                items_inside: None,
                items_outside: None,
                img_data: None,
                bare: false,
            },
        ],
        removed: false,
        source: Source::Base,
    }
}

// --- Test 1: Emitted hero has balanced parens ---
#[test]
fn emit_hero_paren_balanced() {
    let hero = simple_hero();
    let sprites = test_sprites();
    let output = hero_emitter::emit(&hero, &sprites).unwrap();
    hero_emitter::verify_paren_balance(&output).unwrap();
}

// --- Test 2: Tier separators at depth 0 ---
#[test]
fn emit_hero_tier_separators_at_depth_0() {
    let hero = simple_hero();
    let sprites = test_sprites();
    let output = hero_emitter::emit(&hero, &sprites).unwrap();
    hero_emitter::verify_tier_separators(&output).unwrap();
}

// --- Test 3: .n.NAME is last before each + ---
#[test]
fn emit_hero_name_last() {
    let hero = simple_hero();
    let sprites = test_sprites();
    let output = hero_emitter::emit(&hero, &sprites).unwrap();
    hero_emitter::verify_name_last(&output).unwrap();
}

// --- Test 4: .img. contains actual encoding, not sprite_name ---
#[test]
fn emit_hero_sprite_resolved() {
    let hero = simple_hero();
    let sprites = test_sprites();
    let output = hero_emitter::emit(&hero, &sprites).unwrap();
    assert!(output.contains(".img.GIBLE_SPRITE_DATA"), "T1 sprite should be resolved");
    assert!(output.contains(".img.GABITE_SPRITE_DATA"), "T2 sprite should be resolved");
    assert!(output.contains(".img.GARCHOMP_SPRITE_DATA"), "T3 sprite should be resolved");
    // Sprite name should NOT appear as a raw value
    assert!(!output.contains(".img.Gible"), "Should not have .img.Gible (unresolved)");
}

// --- Test 5: Missing sprite gives SpriteNotFound error ---
#[test]
fn emit_hero_sprite_not_found_error() {
    let hero = simple_hero(); // blocks have no img_data
    let empty_sprites = HashMap::new();
    let result = hero_emitter::emit(&hero, &empty_sprites);
    assert!(result.is_err());
    match result.unwrap_err() {
        textmod_compiler::error::CompilerError::SpriteNotFound {
            sprite_name,
            hero_name,
            ..
        } => {
            assert_eq!(sprite_name, "Gible");
            assert_eq!(hero_name, "gible");
        }
        e => panic!("Expected SpriteNotFound, got {:?}", e),
    }
}

// --- Test 6: img_data fallback when no sprite map entry ---
#[test]
fn emit_hero_img_data_fallback() {
    let mut hero = simple_hero();
    // Set img_data on all blocks
    for block in &mut hero.blocks {
        block.img_data = Some(format!("{}_FROM_IMGDATA", block.sprite_name));
    }
    // Empty sprite map — should fall back to img_data
    let empty_sprites = HashMap::new();
    let output = hero_emitter::emit(&hero, &empty_sprites).unwrap();
    assert!(output.contains(".img.Gible_FROM_IMGDATA"), "Should use img_data fallback");
    assert!(output.contains(".img.Gabite_FROM_IMGDATA"), "Should use img_data fallback");
}

// --- Test 6b: sprite map overrides img_data ---
#[test]
fn emit_hero_sprite_map_overrides_img_data() {
    let mut hero = simple_hero();
    for block in &mut hero.blocks {
        block.img_data = Some("OLD_DATA".to_string());
    }
    let sprites = test_sprites();
    let output = hero_emitter::emit(&hero, &sprites).unwrap();
    // Sprite map should win over img_data
    assert!(output.contains(".img.GIBLE_SPRITE_DATA"), "Sprite map should override img_data");
    assert!(!output.contains("OLD_DATA"), "img_data should not appear when sprite map has entry");
}

// --- Test 7: .part.1&hidden suffix and .mn. suffix ---
#[test]
fn emit_hero_part1_suffix() {
    let hero = simple_hero();
    let sprites = test_sprites();
    let output = hero_emitter::emit(&hero, &sprites).unwrap();
    assert!(
        output.contains(".part.1&hidden"),
        "Should have .part.1&hidden suffix"
    );
    assert!(
        output.contains(".mn.Gible@2!m(skip&hidden&temporary)"),
        "Should have .mn. suffix"
    );
    // .part.1 should come after the last tier's .n.Name
    let part_pos = output.find(".part.1").unwrap();
    let last_n = output.rfind(".n.Garchomp").unwrap();
    assert!(
        part_pos > last_n,
        ".part.1 should come after last .n.Name"
    );
}

// --- Test 8: Modifier chain emitted ---
#[test]
fn emit_hero_modifier_chain() {
    let hero = simple_hero();
    let sprites = test_sprites();
    let output = hero_emitter::emit(&hero, &sprites).unwrap();
    assert!(
        output.contains(".i.left.k.scared#facade.bas170:55"),
        "Modifier chain should be preserved"
    );
}

// --- Test 9: ASCII only ---
#[test]
fn emit_hero_ascii_only() {
    let hero = simple_hero();
    let sprites = test_sprites();
    let output = hero_emitter::emit(&hero, &sprites).unwrap();
    hero_emitter::verify_ascii_only(&output).unwrap();
}

// --- Test 10: Build minimal textmod ---
#[test]
fn build_minimal_textmod() {
    let ir = ModIR {
        heroes: vec![simple_hero()],
        replica_items: vec![],
        monsters: vec![],
        bosses: vec![],
        structural: vec![StructuralModifier::new(
            StructuralType::PartyConfig,
            "=party.Roulette.img.ABC123".to_string(),
        )],
    };
    let sprites = test_sprites();
    let output = textmod_compiler::build(&ir, &sprites).unwrap();
    // Should contain both the party config and the hero
    assert!(output.contains("=party.Roulette"));
    assert!(output.contains("heropool."));
    // Should be comma-separated
    assert!(output.contains(",\n\n"));
}

// --- Test 11: Assembly order ---
#[test]
fn build_assembly_order() {
    let ir = ModIR {
        heroes: vec![simple_hero()],
        replica_items: vec![],
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
        bosses: vec![Boss {
            name: "Quagsire".to_string(),
            level: Some(4),
            format: BossFormat::Standard,
            encounter_id: None,
            variants: vec![BossFightVariant {
                name: String::new(),
                trigger: None,
                fight_units: vec![BossFightUnit {
                    template: "Sniper".into(),
                    name: "Wooper".into(),
                    hp: Some(3),
                    sd: None,
                    sprite_data: None,
                    template_override: None,
                    doc: None,
                    modifier_chain: None,
                }],
            }],
            doc: None,
            modifier_chain: None,
            source: Source::Base,
            event_body: None,
        }],
        structural: vec![
            StructuralModifier::new(
                StructuralType::PartyConfig,
                "=party.content".to_string(),
            ),
            StructuralModifier::new(
                StructuralType::Dialog,
                "dialog-content".to_string(),
            ),
        ],
    };
    let sprites = test_sprites();
    let output = textmod_compiler::build(&ir, &sprites).unwrap();

    // Verify ordering: party before dialog before hero before monster before boss
    let party_pos = output.find("=party.content").unwrap();
    let dialog_pos = output.find("dialog-content").unwrap();
    let hero_pos = output.find("heropool.").unwrap();
    let monster_pos = output.find("monsterpool.").unwrap();
    let boss_pos = output.find(".fight.").unwrap();

    assert!(party_pos < dialog_pos, "Party should come before dialog");
    assert!(dialog_pos < hero_pos, "Dialog should come before hero");
    assert!(hero_pos < monster_pos, "Hero should come before monster");
    assert!(monster_pos < boss_pos, "Monster should come before boss");
}
