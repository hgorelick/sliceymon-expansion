use textmod_compiler::extractor::splitter::split_modifiers;

fn load_mod(name: &str) -> String {
    let path = format!("../working-mods/{}.txt", name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", path, e))
}

#[test]
fn split_pansaer_gives_76_modifiers() {
    let text = load_mod("pansaer");
    let mods = split_modifiers(&text).unwrap();
    assert_eq!(
        mods.len(),
        76,
        "Expected 76 pansaer modifiers, got {}",
        mods.len()
    );
}

#[test]
fn split_punpuns_gives_75_modifiers() {
    let text = load_mod("punpuns");
    let mods = split_modifiers(&text).unwrap();
    assert_eq!(
        mods.len(),
        75,
        "Expected 75 punpuns modifiers, got {}",
        mods.len()
    );
}

#[test]
fn split_sliceymon_gives_92_modifiers() {
    let text = load_mod("sliceymon");
    let mods = split_modifiers(&text).unwrap();
    assert_eq!(
        mods.len(),
        92,
        "Expected 92 sliceymon modifiers, got {}",
        mods.len()
    );
}

#[test]
fn split_respects_paren_depth() {
    // Commas inside parentheses should NOT be split points
    let text = load_mod("sliceymon");
    let mods = split_modifiers(&text).unwrap();
    // Every modifier should have balanced parentheses
    for (i, m) in mods.iter().enumerate() {
        let mut depth: i32 = 0;
        for ch in m.chars() {
            match ch {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
            assert!(
                depth >= 0,
                "Modifier {} has negative paren depth: {}",
                i,
                &m[..m.len().min(80)]
            );
        }
        assert_eq!(
            depth, 0,
            "Modifier {} has unbalanced parens (depth {}): {}",
            i,
            depth,
            &m[..m.len().min(80)]
        );
    }
}
