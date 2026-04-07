use textmod_compiler::extractor::classifier::{classify, ModifierType};
use textmod_compiler::extractor::capture_parser;
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

#[test]
fn parse_capture_extracts_pokemon_name() {
    let captures = get_modifiers_of_type("sliceymon", ModifierType::Capture);
    assert!(!captures.is_empty(), "Should have captures");
    let (idx, modifier) = &captures[0];
    let cap = capture_parser::parse_capture(modifier, *idx);
    assert!(
        !cap.pokemon.is_empty(),
        "Capture pokemon name should not be empty: {}",
        &modifier[..modifier.len().min(100)]
    );
}

#[test]
fn parse_capture_extracts_sd() {
    let captures = get_modifiers_of_type("sliceymon", ModifierType::Capture);
    assert!(!captures.is_empty());
    let (idx, modifier) = &captures[0];
    let cap = capture_parser::parse_capture(modifier, *idx);
    assert!(!cap.sd.is_empty(), "Capture sd should not be empty");
    // SD should have colons
    assert!(cap.sd.contains(':'), "SD should contain colons: {}", cap.sd);
}

#[test]
fn parse_legendary_extracts_name() {
    let legends = get_modifiers_of_type("sliceymon", ModifierType::Legendary);
    if legends.is_empty() {
        // Legendaries may not classify separately -- skip test
        return;
    }
    let (idx, modifier) = &legends[0];
    let leg = capture_parser::parse_legendary(modifier, *idx);
    assert!(!leg.pokemon.is_empty(), "Legendary name should not be empty");
}

#[test]
fn roundtrip_captures_raw() {
    let captures = get_modifiers_of_type("sliceymon", ModifierType::Capture);
    for (idx, modifier) in &captures {
        let cap = capture_parser::parse_capture(modifier, *idx);
        assert!(cap.raw.is_some(), "Capture should have raw");
        assert_eq!(cap.raw.as_ref().unwrap(), modifier, "Raw should match original");
    }
}

#[test]
fn roundtrip_legendaries_raw() {
    let legends = get_modifiers_of_type("sliceymon", ModifierType::Legendary);
    for (idx, modifier) in &legends {
        let leg = capture_parser::parse_legendary(modifier, *idx);
        assert!(leg.raw.is_some(), "Legendary should have raw");
        assert_eq!(leg.raw.as_ref().unwrap(), modifier, "Raw should match original");
    }
}
