use textmod_compiler::*;
use textmod_compiler::ir::*;

// ---------------------------------------------------------------------------
// Phase parse/emit round-trips
// ---------------------------------------------------------------------------

#[test]
fn test_phase_parse_emit_roundtrip_message() {
    let input = "ph.4Hello World";
    let phase = parse_phase(input).unwrap();
    let emitted = emit_phase(&phase).unwrap();
    assert_eq!(input, emitted);
}

#[test]
fn test_phase_parse_emit_roundtrip_with_scope() {
    let input = "5.ph.4Hello";
    let phase = parse_phase(input).unwrap();
    let emitted = emit_phase(&phase).unwrap();
    assert_eq!(input, emitted);
}

#[test]
fn test_phase_parse_emit_run_end() {
    let input = "ph.e";
    let phase = parse_phase(input).unwrap();
    let emitted = emit_phase(&phase).unwrap();
    assert_eq!(input, emitted);
}

#[test]
fn test_phase_parse_emit_position_swap() {
    let input = "ph.813";
    let phase = parse_phase(input).unwrap();
    let emitted = emit_phase(&phase).unwrap();
    assert_eq!(input, emitted);
}

#[test]
fn test_phase_parse_emit_reset() {
    let input = "ph.6";
    let phase = parse_phase(input).unwrap();
    let emitted = emit_phase(&phase).unwrap();
    assert_eq!(input, emitted);
}

#[test]
fn test_phase_parse_emit_damage() {
    let input = "ph.d";
    let phase = parse_phase(input).unwrap();
    let emitted = emit_phase(&phase).unwrap();
    assert_eq!(input, emitted);
}

#[test]
fn test_phase_parse_emit_item_combine() {
    let input = "ph.7SecondHighestToTierThrees";
    let phase = parse_phase(input).unwrap();
    let emitted = emit_phase(&phase).unwrap();
    assert_eq!(input, emitted);
}

// ---------------------------------------------------------------------------
// Chain round-trips
// ---------------------------------------------------------------------------

#[test]
fn test_chain_roundtrip_simple() {
    let input = ".i.left.k.scared#facade.bas170:55";
    let chain = ModifierChain::parse(input);
    assert_eq!(chain.emit(), input);
}

#[test]
fn test_chain_roundtrip_nested_parens() {
    let input = ".i.(left.hat.(statue.sd.15-2))";
    let chain = ModifierChain::parse(input);
    assert_eq!(chain.emit(), input);
}

#[test]
fn test_chain_roundtrip_sticker() {
    let input = ".sticker.k.dejavu#k.exert#sidesc.Add [pink]dejavu[cu] text";
    let chain = ModifierChain::parse(input);
    assert_eq!(chain.emit(), input);
}

#[test]
fn test_chain_roundtrip_mixed_segments() {
    let input = ".i.left.k.scared.sticker.k.dejavu";
    let chain = ModifierChain::parse(input);
    assert_eq!(chain.emit(), input);
}

// ---------------------------------------------------------------------------
// Reward tag round-trips
// ---------------------------------------------------------------------------

#[test]
fn test_reward_tag_roundtrip() {
    let cases = vec!["m(skip&hidden)", "i(Sword)", "s", "r1~3~i"];
    for input in cases {
        let tag = parse_reward_tag(input).unwrap();
        let emitted = emit_reward_tag(&tag);
        assert_eq!(input, emitted, "Failed for input: {}", input);
    }
}

// ---------------------------------------------------------------------------
// Validator: cross-reference checks
// ---------------------------------------------------------------------------

#[test]
fn test_check_references_empty_ir() {
    let ir = ModIR::empty();
    let report = check_references(&ir);
    assert!(report.errors.is_empty());
    assert!(report.warnings.is_empty());
}

#[test]
fn test_check_references_reports_used_but_not_defined() {
    // A boss referencing a template that doesn't exist should still pass
    // check_references (it validates cross-references between IR items,
    // not against the game's built-in templates).
    let mut ir = ModIR::empty();
    ir.bosses.push(Boss {
        name: "TestBoss".into(),
        level: Some(4),
        format: BossFormat::Standard,
        encounter_id: None,
        fights: vec![FightDefinition {
            level: None,
            enemies: vec![FightUnit {
                template: "Sniper".into(),
                name: "Enemy".into(),
                hp: Some(3),
                ..Default::default()
            }],
            name: None,
            trigger: None,
        }],
        doc: None,
        modifier_chain: None,
        source: Source::Base,
        event_phases: None,
    });
    let report = check_references(&ir);
    // No errors for basic structural references
    assert!(report.errors.is_empty(), "Errors: {:?}", report.errors);
}

// ---------------------------------------------------------------------------
// IR JSON round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_ir_json_roundtrip() {
    let mut ir = ModIR::empty();
    // Add a simple structural modifier
    ir.structural.push(StructuralModifier::new(
        StructuralType::EventModifier,
        "test content".to_string(),
    ));
    let json = ir_to_json(&ir).unwrap();
    let back = ir_from_json(&json).unwrap();
    assert_eq!(ir.structural.len(), back.structural.len());
}

#[test]
fn test_ir_json_roundtrip_with_hero() {
    let mut ir = ModIR::empty();
    ir.heroes.push(Hero {
        internal_name: "test".into(),
        mn_name: "Test".into(),
        color: 'a',
        format: HeroFormat::Sliceymon,
        blocks: vec![HeroBlock {
            template: "Lost".into(),
            tier: Some(1),
            hp: Some(5),
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            color: None,
            sprite: SpriteId::owned("test", "testimg"),
            speech: "!".into(),
            name: "Test".into(),
            doc: None,
            abilitydata: None,
            triggerhpdata: None,
            hue: None,
            modifier_chain: None,
            facades: vec![],
            items_inside: None,
            items_outside: None,
            bare: false,
        }],
        removed: false,
        source: Source::Base,
    });
    let json = ir_to_json(&ir).unwrap();
    let back = ir_from_json(&json).unwrap();
    assert_eq!(ir.heroes.len(), back.heroes.len());
    assert_eq!(ir.heroes[0].mn_name, back.heroes[0].mn_name);
    assert_eq!(ir.heroes[0].blocks[0].hp, back.heroes[0].blocks[0].hp);
}

// ---------------------------------------------------------------------------
// Schema generation
// ---------------------------------------------------------------------------

#[test]
fn test_schema_generation() {
    // Verify JSON Schema can be generated for ModIR
    let schema = schemars::schema_for!(ModIR);
    let json = serde_json::to_string_pretty(&schema).unwrap();
    assert!(json.contains("ModIR"));
    assert!(json.contains("Phase"));
    assert!(json.contains("ChainEntry"));
    assert!(json.contains("FightDefinition"));
}

#[test]
fn test_schema_contains_new_types() {
    let schema = schemars::schema_for!(ModIR);
    let json = serde_json::to_string_pretty(&schema).unwrap();
    // Verify all chunk types are present in the schema
    assert!(json.contains("RewardTag"), "Schema should contain RewardTag");
    assert!(json.contains("PhaseType"), "Schema should contain PhaseType");
    assert!(json.contains("ChainSegment"), "Schema should contain ChainSegment");
    assert!(json.contains("FightUnit"), "Schema should contain FightUnit");
    assert!(json.contains("RichText"), "Schema should contain RichText");
    assert!(json.contains("LevelScope"), "Schema should contain LevelScope");
}

// ---------------------------------------------------------------------------
// DiceFaces edge cases
// ---------------------------------------------------------------------------

#[test]
fn test_dice_faces_all_blanks() {
    let df = DiceFaces::parse("0:0:0:0:0:0");
    assert_eq!(df.faces.len(), 6);
    assert_eq!(df.emit(), "0:0:0:0:0:0");
    for face in &df.faces {
        assert_eq!(*face, DiceFace::Blank);
    }
}

#[test]
fn test_dice_faces_mixed() {
    let df = DiceFaces::parse("34-1:30-1:0:0:30-1:0");
    assert_eq!(df.faces.len(), 6);
    assert_eq!(df.emit(), "34-1:30-1:0:0:30-1:0");
}

#[test]
fn test_dice_faces_high_pips() {
    let df = DiceFaces::parse("170-9:0:0:0:0:0");
    assert_eq!(df.emit(), "170-9:0:0:0:0:0");
    if let DiceFace::Active { face_id, pips } = &df.faces[0] {
        assert_eq!(face_id.raw(), 170);
        assert_eq!(pips.raw(), 9);
    } else {
        panic!("Expected Active face");
    }
}

// ---------------------------------------------------------------------------
// ModifierChain: typed entry access
// ---------------------------------------------------------------------------

#[test]
fn test_chain_typed_entries() {
    let chain = ModifierChain::parse(".i.left.k.scared#facade.bas170:55");
    assert_eq!(chain.segments.len(), 1);
    match &chain.segments[0] {
        ChainSegment::Item { sub_entries } => {
            assert!(sub_entries.len() >= 2, "Should have at least 2 sub-entries");
        }
        _ => panic!("Expected Item segment"),
    }
}

#[test]
fn test_chain_sticker_segment() {
    let chain = ModifierChain::parse(".sticker.k.dejavu#k.exert");
    assert_eq!(chain.segments.len(), 1);
    match &chain.segments[0] {
        ChainSegment::Sticker { sub_entries } => {
            assert_eq!(sub_entries.len(), 2);
        }
        _ => panic!("Expected Sticker segment"),
    }
}

// ---------------------------------------------------------------------------
// Hero validate-then-build
// ---------------------------------------------------------------------------

#[test]
fn test_validate_hero_minimal() {
    let hero = Hero {
        internal_name: "test".into(),
        mn_name: "Test".into(),
        color: 'a',
        format: HeroFormat::Sliceymon,
        blocks: vec![HeroBlock {
            template: "Lost".into(),
            tier: Some(1),
            hp: Some(5),
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            color: None,
            sprite: SpriteId::owned("test", "testimg"),
            speech: "!".into(),
            name: "Test".into(),
            doc: None,
            abilitydata: None,
            triggerhpdata: None,
            hue: None,
            modifier_chain: None,
            facades: vec![],
            items_inside: None,
            items_outside: None,
            bare: false,
        }],
        removed: false,
        source: Source::Base,
    };

    let report = validate_hero(&hero);
    assert!(report.errors.is_empty(), "Minimal hero should validate: {:?}", report.errors);
}

// ---------------------------------------------------------------------------
// Provenance tracking
// ---------------------------------------------------------------------------

#[test]
fn test_source_default_is_base() {
    let hero = Hero {
        internal_name: "test".into(),
        mn_name: "Test".into(),
        color: 'a',
        format: HeroFormat::default(),
        blocks: vec![],
        removed: false,
        source: Source::default(),
    };
    assert_eq!(hero.source, Source::Base);
}

#[test]
fn test_source_survives_json_roundtrip() {
    let mut ir = ModIR::empty();
    let hero = Hero {
        internal_name: "test".into(),
        mn_name: "Test".into(),
        color: 'a',
        format: HeroFormat::default(),
        blocks: vec![],
        removed: false,
        source: Source::Custom,
    };
    ir.heroes.push(hero);
    let json = ir_to_json(&ir).unwrap();
    let back = ir_from_json(&json).unwrap();
    assert_eq!(back.heroes[0].source, Source::Custom);
}

// ---------------------------------------------------------------------------
// Extract must preserve source `.img.` even when the display name collides
// with the corpus-derived sprite registry. Round-1 tribunal finding
// (2026-04-20 sprite-shape ruling follow-up): the earlier
// `SpriteId::lookup(...)` fast-path in every parser
// silently replaced the source's `.img.` payload with the registry entry's
// whenever the name was known (sliceymon-seeded, so pansaer/punpuns/community
// names colliding with sliceymon — e.g. `Pikachu` — lost their own `.img.`).
// That's source corruption during extract and violates SPEC §3.3. These tests
// pin the source-preservation property so a future refactor can't re-introduce
// lookup-first semantics into the extract path.
// ---------------------------------------------------------------------------

#[test]
fn extract_preserves_hero_img_data_on_registry_name_collision() {
    // `Pikachu` is registered from sliceymon.txt. The source below supplies a
    // novel, deliberately-wrong `.img.` payload; extract must store *that*, not
    // the registry's sliceymon payload.
    let text = "!mheropool.(replica.Lost.hp.5.sd.0:0:0:0:0:0.img.TRIBUNAL_NOVEL_IMG).speech.!.n.Pikachu,";
    let ir = extract(text).expect("extract");
    let block = &ir.heroes[0].blocks[0];
    assert_eq!(block.sprite.name(), "Pikachu");
    assert_eq!(
        block.sprite.img_data(),
        "TRIBUNAL_NOVEL_IMG",
        "extract must keep source .img. verbatim even when the display name is in the registry",
    );
}

#[test]
fn extract_preserves_monster_img_data_on_registry_name_collision() {
    // Monster shape per `working-mods/punpuns.txt`: `1-3.monsterpool.(rmon...)...img.X...n.NAME`.
    let text = "1-3.monsterpool.(rmon.ded.bal.boar).hp.3.sd.30-3:30-3:30-2:30-2:19-3:19-2.img.TRIBUNAL_NOVEL_IMG.n.Pikachu,";
    let ir = extract(text).expect("extract");
    let monster = &ir.monsters[0];
    let sprite = monster.sprite.as_ref().expect("monster sprite");
    assert_eq!(sprite.name(), "Pikachu");
    assert_eq!(sprite.img_data(), "TRIBUNAL_NOVEL_IMG");
}

// ---------------------------------------------------------------------------
// Chunk 3c: `sprites: &HashMap` dropped from the public API
// ---------------------------------------------------------------------------

fn minimal_path_b_hero() -> Hero {
    Hero {
        internal_name: "pb".into(),
        mn_name: "PathB".into(),
        color: 'a',
        format: HeroFormat::Sliceymon,
        blocks: vec![HeroBlock {
            template: "Lost".into(),
            tier: Some(1),
            hp: Some(5),
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            color: None,
            sprite: SpriteId::owned("PathB", "PATHB_IMG"),
            speech: "!".into(),
            name: "PathB".into(),
            doc: None,
            abilitydata: None,
            triggerhpdata: None,
            hue: None,
            modifier_chain: None,
            facades: vec![],
            items_inside: None,
            items_outside: None,
            bare: false,
        }],
        removed: false,
        source: Source::Base,
    }
}

#[test]
fn build_no_sprites_path_b() {
    // Path B: ModIR constructed in-memory, no sprite HashMap passed anywhere.
    let mut ir = ModIR::empty();
    ir.heroes.push(minimal_path_b_hero());
    let textmod = build_complete(&ir).expect("build_complete");
    assert!(textmod.contains("ph.bpb"), "expected sliceymon ph.b prefix");
    assert!(textmod.contains(".img.PATHB_IMG"), "img_data must ride on the block");
    assert!(textmod.contains(".mn.PathB"), "expected mn suffix");
}

#[test]
fn build_roundtrip_sliceymon_no_sprites() {
    let mod_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("working-mods")
        .join("sliceymon.txt");
    let text = std::fs::read_to_string(&mod_path).expect("read sliceymon.txt");
    let ir1 = extract(&text).expect("extract1");
    let rebuilt = build(&ir1).expect("build without sprites");
    let ir2 = extract(&rebuilt).expect("extract2");
    assert_eq!(ir1.heroes, ir2.heroes, "sliceymon heroes must roundtrip IR-equal");
    assert_eq!(ir1.replica_items, ir2.replica_items, "replica items must roundtrip IR-equal");
    assert_eq!(ir1.monsters, ir2.monsters, "monsters must roundtrip IR-equal");
}

#[test]
fn build_hero_signature() {
    // Compile-time guarantee: build_hero takes only &Hero — no HashMap arg.
    let hero = minimal_path_b_hero();
    let emitted = build_hero(&hero).expect("build_hero");
    assert!(emitted.contains(".mn.PathB"));
}

#[test]
fn build_emits_block_img_data_not_registry_payload() {
    // Source-vs-IR divergence pin (per Chunk 3b tribunal lesson): the emitter
    // must read `HeroBlock.sprite.img_data()` and nothing else. If a future
    // refactor re-introduces registry lookup on the build path, a block whose
    // name collides with a registered sprite but carries novel img_data would
    // silently emit the registered payload instead of the block's own bytes.
    //
    // "Pikachu" is a registered name (see Chunk 3a), so using it as `name`
    // while carrying `CHUNK_3C_NOVEL_IMG` as `img_data` exercises the
    // collision surface.
    let mut hero = minimal_path_b_hero();
    hero.blocks[0].sprite = SpriteId::owned("Pikachu", "CHUNK_3C_NOVEL_IMG");
    let emitted = build_hero(&hero).expect("build_hero");
    assert!(
        emitted.contains(".img.CHUNK_3C_NOVEL_IMG"),
        "emitter must use the block's own img_data, got: {}",
        emitted
    );
}
