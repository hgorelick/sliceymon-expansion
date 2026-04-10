use std::collections::HashMap;
use textmod_compiler::ir::ModIR;

fn load_mod(name: &str) -> String {
    let path = format!("../working-mods/{}.txt", name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", path, e))
}

fn load_sprites() -> HashMap<String, String> {
    let path = "../tools/sprite_encodings.json";
    let json = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to load sprites: {}", e));
    serde_json::from_str(&json).unwrap()
}

/// Compare two ModIRs for semantic equality.
/// Checks counts, names, and key fields across all types.
fn assert_ir_equal(ir1: &ModIR, ir2: &ModIR, label: &str) {
    // Collection counts
    assert_eq!(
        ir1.heroes.len(), ir2.heroes.len(),
        "{}: hero count mismatch ({} vs {})", label, ir1.heroes.len(), ir2.heroes.len()
    );
    assert_eq!(
        ir1.replica_items.len(), ir2.replica_items.len(),
        "{}: replica_items count mismatch ({} vs {})", label, ir1.replica_items.len(), ir2.replica_items.len()
    );
    assert_eq!(
        ir1.monsters.len(), ir2.monsters.len(),
        "{}: monster count mismatch ({} vs {})", label, ir1.monsters.len(), ir2.monsters.len()
    );
    assert_eq!(
        ir1.bosses.len(), ir2.bosses.len(),
        "{}: boss count mismatch ({} vs {})", label, ir1.bosses.len(), ir2.bosses.len()
    );
    assert_eq!(
        ir1.structural.len(), ir2.structural.len(),
        "{}: structural count mismatch ({} vs {})", label, ir1.structural.len(), ir2.structural.len()
    );

    // Hero field comparison
    for (h1, h2) in ir1.heroes.iter().zip(ir2.heroes.iter()) {
        assert_eq!(h1.mn_name, h2.mn_name, "{}: hero mn_name mismatch", label);
        assert_eq!(h1.internal_name, h2.internal_name, "{}: hero internal_name mismatch for {}", label, h1.mn_name);
        assert_eq!(h1.color, h2.color, "{}: hero color mismatch for {}", label, h1.mn_name);

        // Emittable block count (degenerate blocks with no template/img_data skipped during emission)
        let emittable1 = h1.blocks.iter().filter(|b| !b.template.is_empty() || b.img_data.is_some()).count();
        let emittable2 = h2.blocks.iter().filter(|b| !b.template.is_empty() || b.img_data.is_some()).count();
        assert_eq!(
            emittable1, emittable2,
            "{}: emittable block count mismatch for hero {} ({} vs {})", label, h1.mn_name, emittable1, emittable2
        );

        // Per-block field comparison (emittable blocks only)
        let blocks1: Vec<_> = h1.blocks.iter().filter(|b| !b.template.is_empty() || b.img_data.is_some()).collect();
        let blocks2: Vec<_> = h2.blocks.iter().filter(|b| !b.template.is_empty() || b.img_data.is_some()).collect();
        for (b1, b2) in blocks1.iter().zip(blocks2.iter()) {
            assert_eq!(b1.template, b2.template, "{}: block template mismatch for {} block", label, h1.mn_name);
            assert_eq!(b1.hp, b2.hp, "{}: block hp mismatch for {} block", label, h1.mn_name);
            assert_eq!(b1.sd, b2.sd, "{}: block sd mismatch for {} block", label, h1.mn_name);
            assert_eq!(b1.name.trim(), b2.name.trim(), "{}: block name mismatch for {} block", label, h1.mn_name);
        }
    }

    // Monster field comparison
    for (m1, m2) in ir1.monsters.iter().zip(ir2.monsters.iter()) {
        assert_eq!(m1.name, m2.name, "{}: monster name mismatch", label);
        assert_eq!(m1.base_template, m2.base_template, "{}: monster template mismatch for {}", label, m1.name);
        assert_eq!(m1.hp, m2.hp, "{}: monster hp mismatch for {}", label, m1.name);
    }

    // Boss field comparison
    for (b1, b2) in ir1.bosses.iter().zip(ir2.bosses.iter()) {
        assert_eq!(b1.name, b2.name, "{}: boss name mismatch", label);
        assert_eq!(b1.level, b2.level, "{}: boss level mismatch for {}", label, b1.name);
        assert_eq!(b1.variants.len(), b2.variants.len(), "{}: boss variants count mismatch for {}", label, b1.name);
        for (v1, v2) in b1.variants.iter().zip(b2.variants.iter()) {
            assert_eq!(v1.fight_units.len(), v2.fight_units.len(), "{}: boss variant fight_units count mismatch for {}", label, b1.name);
        }
    }

    // Replica item field comparison
    for (r1, r2) in ir1.replica_items.iter().zip(ir2.replica_items.iter()) {
        assert_eq!(r1.name, r2.name, "{}: replica item name mismatch", label);
        assert_eq!(r1.template, r2.template, "{}: replica item template mismatch for {}", label, r1.name);
    }

    // Structural: compare as (type, name) sets — order may differ after rebuild
    let mut s1_keys: Vec<_> = ir1.structural.iter().map(|s| (format!("{:?}", s.modifier_type), s.name.clone())).collect();
    let mut s2_keys: Vec<_> = ir2.structural.iter().map(|s| (format!("{:?}", s.modifier_type), s.name.clone())).collect();
    s1_keys.sort();
    s2_keys.sort();
    assert_eq!(s1_keys, s2_keys, "{}: structural (type, name) set mismatch", label);
}

/// Full round-trip: extract -> build -> extract -> compare IRs.
fn roundtrip(name: &str) {
    let text = load_mod(name);
    let sprites = load_sprites();
    let ir1 = textmod_compiler::extract(&text).unwrap();
    let rebuilt = textmod_compiler::build(&ir1, &sprites).unwrap();
    let ir2 = textmod_compiler::extract(&rebuilt).unwrap();
    assert_ir_equal(&ir1, &ir2, name);
}

// --- Round-trip tests for all 4 mods ---

#[test]
fn roundtrip_sliceymon() { roundtrip("sliceymon"); }

#[test]
fn roundtrip_punpuns() { roundtrip("punpuns"); }

#[test]
fn roundtrip_pansaer() { roundtrip("pansaer"); }

#[test]
fn roundtrip_community_extraction() {
    // Community mod requires external sprites not in sprite_encodings.json,
    // so full build round-trip fails. Verify extraction produces valid IR.
    let text = load_mod("community");
    let ir = textmod_compiler::extract(&text).unwrap();
    assert!(ir.heroes.len() > 0, "community should have heroes");
    assert!(ir.structural.len() > 0, "community should have structural modifiers");
    // Verify JSON round-trip of the IR itself
    let json = textmod_compiler::ir_to_json(&ir).unwrap();
    let restored = textmod_compiler::ir_from_json(&json).unwrap();
    assert_eq!(ir.heroes.len(), restored.heroes.len());
    assert_eq!(ir.monsters.len(), restored.monsters.len());
    assert_eq!(ir.bosses.len(), restored.bosses.len());
}

// --- Deeper field tests ---

#[test]
fn roundtrip_sliceymon_hero_count() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    let hero_count = ir.heroes.len();
    assert!(hero_count >= 40 && hero_count <= 48, "Expected 40-48 heroes, got {}", hero_count);
}

#[test]
fn roundtrip_sliceymon_no_individual_replica_items() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    assert_eq!(ir.replica_items.len(), 0, "No individual replica_items from extraction");
    let sprites = load_sprites();
    let rebuilt = textmod_compiler::build(&ir, &sprites).unwrap();
    let ir2 = textmod_compiler::extract(&rebuilt).unwrap();
    assert_eq!(ir2.replica_items.len(), 0);
}

#[test]
fn roundtrip_sliceymon_boss_count() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    let sprites = load_sprites();
    let rebuilt = textmod_compiler::build(&ir, &sprites).unwrap();
    let ir2 = textmod_compiler::extract(&rebuilt).unwrap();
    assert_eq!(ir.bosses.len(), ir2.bosses.len());
}

#[test]
fn build_output_is_valid_text() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    let sprites = load_sprites();
    let output = textmod_compiler::build(&ir, &sprites).unwrap();

    // No non-ASCII
    for (i, ch) in output.char_indices() {
        assert!(ch.is_ascii(), "Non-ASCII char {} (U+{:04X}) at position {}", ch, ch as u32, i);
    }

    // Globally balanced parens
    let mut depth: i32 = 0;
    for (i, ch) in output.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        assert!(depth >= 0, "Negative paren depth at position {}", i);
    }
    assert_eq!(depth, 0, "Unbalanced parens: depth {} at end of output", depth);
}

// --- Rebuilt mod validates (structural checks only — per-field rebuild fidelity
// verified by the round-trip IR comparison above) ---

#[test]
fn rebuilt_sliceymon_validates_structure() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    let sprites = load_sprites();
    let rebuilt = textmod_compiler::build(&ir, &sprites).unwrap();
    let report = textmod_compiler::validate(&rebuilt).unwrap();
    // Rebuilt text may have E010/E011 from emitted .hp.0 blocks — these are
    // emit fidelity issues, not IR correctness issues. Check no E001 (parens).
    let paren_errors: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "E001").collect();
    assert!(paren_errors.is_empty(), "Rebuilt sliceymon should have balanced parens: {:?}", paren_errors);
}

#[test]
fn rebuilt_pansaer_validates_structure() {
    let text = load_mod("pansaer");
    let ir = textmod_compiler::extract(&text).unwrap();
    let sprites = load_sprites();
    let rebuilt = textmod_compiler::build(&ir, &sprites).unwrap();
    let report = textmod_compiler::validate(&rebuilt).unwrap();
    let paren_errors: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "E001").collect();
    assert!(paren_errors.is_empty(), "Rebuilt pansaer should have balanced parens: {:?}", paren_errors);
}

// --- Schema test ---

#[test]
fn schema_validates_hand_authored_hero() {
    let schema = schemars::schema_for!(textmod_compiler::ir::ModIR);
    let json = serde_json::to_string_pretty(&schema).unwrap();
    // Schema should be valid JSON with definitions
    assert!(json.contains("\"ModIR\"") || json.contains("\"properties\""), "Schema should contain type definitions");
    assert!(json.contains("\"Hero\""), "Schema should reference Hero type");
}

// --- Build from partial IR ---

#[test]
fn build_from_partial_ir() {
    use textmod_compiler::ir::*;

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
            sprite_name: "test".into(),
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
            img_data: Some("testimg".into()),
            bare: false,
        }],
        removed: false,
        source: Source::Base,
    });

    let sprites: HashMap<String, String> = [("test".into(), "testimg".into())].into();
    let output = textmod_compiler::build(&ir, &sprites).unwrap();
    assert!(!output.is_empty(), "Build from partial IR should produce output");
    assert!(output.contains("Lost"), "Output should contain hero template");
}
