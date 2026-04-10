use textmod_compiler::ir::*;
use textmod_compiler::ir::HeroFormat;
use textmod_compiler::error::CompilerError;

#[test]
fn ir_types_serialize_roundtrip() {
    let ir = ModIR {
        heroes: vec![Hero {
            internal_name: "gible".to_string(),
            mn_name: "Gible".to_string(),
            color: 'b',
            format: HeroFormat::default(),
            blocks: vec![HeroBlock {
                template: "Eccentric".to_string(),
                tier: Some(1),
                hp: Some(4),
                sd: DiceFaces::parse("34-1:30-1:0:0:30-1:0"),
                color: None,
                sprite_name: "Gible".to_string(),
                speech: "Gib!~Gible!".to_string(),
                name: "Gible".to_string(),
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
            }],
            removed: false,
            source: Source::Base,
        }],
        replica_items: vec![],
        monsters: vec![],
        bosses: vec![],
        structural: vec![],
    };

    let json = serde_json::to_string_pretty(&ir).unwrap();
    let deserialized: ModIR = serde_json::from_str(&json).unwrap();
    assert_eq!(ir, deserialized);
}

#[test]
fn hero_block_required_fields() {
    // Verify HeroBlock serialization includes all required fields
    let tier = HeroBlock {
        template: "Thief".to_string(),
        tier: Some(2),
        hp: Some(6),
        sd: DiceFaces::parse("93-2:93-2:56-1:56-1:0:0"),
        color: None,
        sprite_name: "Snorunt".to_string(),
        speech: "Snor!~Runt!".to_string(),
        name: "Snorunt".to_string(),
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
    };

    let json = serde_json::to_string(&tier).unwrap();
    // All required fields must be present
    assert!(json.contains("\"template\":\"Thief\""));
    assert!(json.contains("\"hp\":6"));
    assert!(json.contains("\"sd\":{\"faces\":"), "sd should serialize as DiceFaces struct");
    assert!(json.contains("\"sprite_name\":\"Snorunt\""));
    assert!(json.contains("\"speech\":\"Snor!~Runt!\""));
    assert!(json.contains("\"name\":\"Snorunt\""));
    // Optional None fields should be skipped
    assert!(!json.contains("\"doc\""));
    assert!(!json.contains("\"abilitydata\""));
}

#[test]
fn compiler_error_display() {
    let err = CompilerError::HeroParseError {
        modifier_index: 5,
        hero_name: "torchic".to_string(),
        tier_index: Some(2),
        position: 847,
        expected: ".tier.".to_string(),
        found: ".col.".to_string(),
    };
    let display = format!("{}", err);
    assert!(display.contains("torchic"));
    assert!(display.contains("tier 2"));
    assert!(display.contains("position 847"));
    assert!(display.contains(".tier."));
    assert!(display.contains(".col."));
    assert!(display.contains("hint:"));

    let paren_err = CompilerError::ParenError {
        modifier_index: 3,
        position: 120,
        depth: -1,
        context: "...some context here...".to_string(),
    };
    let display2 = format!("{}", paren_err);
    assert!(display2.contains("position 120"));
    assert!(display2.contains("depth -1"));
    assert!(display2.contains("some context here"));

    let sprite_err = CompilerError::SpriteNotFound {
        sprite_name: "MissingMon".to_string(),
        hero_name: "fakemon".to_string(),
        tier_index: 0,
    };
    let display3 = format!("{}", sprite_err);
    assert!(display3.contains("MissingMon"));
    assert!(display3.contains("fakemon"));
    assert!(display3.contains("sprite_encodings.json"));
}

#[test]
fn empty_mod_ir_serializes() {
    let ir = ModIR::empty();
    let json = serde_json::to_string_pretty(&ir).unwrap();
    let deserialized: ModIR = serde_json::from_str(&json).unwrap();
    assert_eq!(ir, deserialized);
    assert!(ir.heroes.is_empty());
    assert!(ir.replica_items.is_empty());
    assert!(ir.monsters.is_empty());
    assert!(ir.bosses.is_empty());
    assert!(ir.structural.is_empty());
    // ModIR no longer has original_modifiers field
    assert!(!json.contains("original_modifiers"));
}
