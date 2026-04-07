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
/// Checks counts and key fields, not exact string equality.
fn assert_ir_equal(ir1: &ModIR, ir2: &ModIR, label: &str) {
    assert_eq!(
        ir1.heroes.len(),
        ir2.heroes.len(),
        "{}: hero count mismatch ({} vs {})",
        label,
        ir1.heroes.len(),
        ir2.heroes.len()
    );
    assert_eq!(
        ir1.captures.len(),
        ir2.captures.len(),
        "{}: capture count mismatch",
        label
    );
    assert_eq!(
        ir1.legendaries.len(),
        ir2.legendaries.len(),
        "{}: legendary count mismatch",
        label
    );
    assert_eq!(
        ir1.monsters.len(),
        ir2.monsters.len(),
        "{}: monster count mismatch",
        label
    );
    assert_eq!(
        ir1.bosses.len(),
        ir2.bosses.len(),
        "{}: boss count mismatch",
        label
    );
    assert_eq!(
        ir1.structural.len(),
        ir2.structural.len(),
        "{}: structural count mismatch ({} vs {})",
        label,
        ir1.structural.len(),
        ir2.structural.len()
    );

    // Compare hero names and key fields
    for (h1, h2) in ir1.heroes.iter().zip(ir2.heroes.iter()) {
        assert_eq!(
            h1.mn_name, h2.mn_name,
            "{}: hero mn_name mismatch",
            label
        );
        assert_eq!(
            h1.internal_name, h2.internal_name,
            "{}: hero internal_name mismatch for {}",
            label, h1.mn_name
        );
        // If both have parsed blocks, compare block counts
        if h1.raw.is_none() && h2.raw.is_none() {
            assert_eq!(
                h1.blocks.len(),
                h2.blocks.len(),
                "{}: block count mismatch for hero {}",
                label,
                h1.mn_name
            );
        }
    }

    // Compare monster names
    for (m1, m2) in ir1.monsters.iter().zip(ir2.monsters.iter()) {
        assert_eq!(m1.name, m2.name, "{}: monster name mismatch", label);
    }

    // Compare boss names
    for (b1, b2) in ir1.bosses.iter().zip(ir2.bosses.iter()) {
        assert_eq!(b1.name, b2.name, "{}: boss name mismatch", label);
    }

}

/// Full round-trip: extract -> build -> extract -> compare IRs.
/// Heroes use raw passthrough since we do not have a full emit pipeline for
/// mods we did not generate (we need sprites for non-raw emit).
fn roundtrip(name: &str) {
    let text = load_mod(name);
    let sprites = load_sprites();

    // First extraction
    let ir1 = textmod_compiler::extract(&text).unwrap();

    // Build from IR
    let rebuilt = textmod_compiler::build(&ir1, &sprites).unwrap();

    // Second extraction
    let ir2 = textmod_compiler::extract(&rebuilt).unwrap();

    // Compare
    assert_ir_equal(&ir1, &ir2, name);
}

// --- Test 1: Sliceymon round-trip ---
#[test]
fn roundtrip_sliceymon() {
    roundtrip("sliceymon");
}

// --- Test 2: Punpuns round-trip ---
#[test]
fn roundtrip_punpuns() {
    roundtrip("punpuns");
}

// --- Test 3: Pansaer round-trip ---
#[test]
fn roundtrip_pansaer() {
    roundtrip("pansaer");
}

// --- Test 4: Hero count preserved ---
#[test]
fn roundtrip_sliceymon_hero_count() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    let hero_count = ir.heroes.len();
    eprintln!(
        "Sliceymon: {} heroes, captures={}, monsters={}, bosses={}",
        hero_count,
        ir.captures.len(),
        ir.monsters.len(),
        ir.bosses.len()
    );
    assert!(
        hero_count >= 40 && hero_count <= 48,
        "Expected 40-48 heroes, got {}",
        hero_count
    );
}

// --- Test 5: Capture count preserved ---
#[test]
fn roundtrip_sliceymon_capture_count() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    let sprites = load_sprites();
    let rebuilt = textmod_compiler::build(&ir, &sprites).unwrap();
    let ir2 = textmod_compiler::extract(&rebuilt).unwrap();
    assert_eq!(ir.captures.len(), ir2.captures.len());
}

// --- Test 6: Boss count preserved ---
#[test]
fn roundtrip_sliceymon_boss_count() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    let sprites = load_sprites();
    let rebuilt = textmod_compiler::build(&ir, &sprites).unwrap();
    let ir2 = textmod_compiler::extract(&rebuilt).unwrap();
    assert_eq!(ir.bosses.len(), ir2.bosses.len());
}

// --- Test 7: Built output validity ---
#[test]
fn build_output_is_valid_text() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    let sprites = load_sprites();
    let output = textmod_compiler::build(&ir, &sprites).unwrap();

    // No non-ASCII
    for (i, ch) in output.char_indices() {
        assert!(
            ch.is_ascii(),
            "Non-ASCII char {} (U+{:04X}) at position {}",
            ch,
            ch as u32,
            i
        );
    }

    // Globally balanced parens
    let mut depth: i32 = 0;
    for (i, ch) in output.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        assert!(
            depth >= 0,
            "Negative paren depth at position {}",
            i
        );
    }
    assert_eq!(depth, 0, "Unbalanced parens: depth {} at end of output", depth);
}
