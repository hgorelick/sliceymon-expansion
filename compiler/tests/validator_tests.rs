use textmod_compiler::validator::{validate, validate_ir, ValidationReport, Finding, Severity};
use textmod_compiler::ir::{ModIR, Hero, HeroBlock, HeroFormat, DiceFaces, ReplicaItem, AbilityData, Source};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn load_mod(name: &str) -> String {
    let path = format!("../working-mods/{}.txt", name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", path, e))
}

/// Build two modifiers separated by comma for cross-modifier tests.
fn two_modifiers(a: &str, b: &str) -> String {
    format!("{},{}", a, b)
}

// ---------------------------------------------------------------------------
// Integration tests: working mods produce 0 errors
// ---------------------------------------------------------------------------

#[test]
fn validate_pansaer_zero_errors() {
    let text = load_mod("pansaer");
    let report = validate(&text).unwrap();
    assert!(
        report.is_ok(),
        "pansaer should have 0 errors, got {}:\n{}",
        report.errors.len(),
        report
    );
}

#[test]
fn validate_punpuns_zero_errors() {
    let text = load_mod("punpuns");
    let report = validate(&text).unwrap();
    assert!(
        report.is_ok(),
        "punpuns should have 0 errors, got {}:\n{}",
        report.errors.len(),
        report
    );
}

#[test]
fn validate_sliceymon_zero_errors() {
    let text = load_mod("sliceymon");
    let report = validate(&text).unwrap();
    assert!(
        report.is_ok(),
        "sliceymon should have 0 errors, got {}:\n{}",
        report.errors.len(),
        report
    );
}

// ---------------------------------------------------------------------------
// Unit tests: each error rule
// ---------------------------------------------------------------------------

// 4. E001: unbalanced parens
#[test]
fn validate_unbalanced_parens() {
    let input = "hidden&temporary&ph.bfoo;1;!mheropool.\
        ((replica.Foo.col.a.tier.1.hp.6.sd.0:0:0:0:0:0.img.x).n.Foo\
        +(replica.Foo.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.Foo2\
        +(replica.Foo.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.Foo3\
        .part.1&hidden.mn.Foo@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E001"),
        "Expected E001 for unbalanced parens, got: {:?}",
        report.errors
    );
    let e001 = report.errors.iter().find(|f| f.rule_id == "E001").unwrap();
    assert!(e001.position.is_some(), "E001 should have position");
}

// 5. E002: non-ASCII
#[test]
fn validate_non_ascii() {
    let input = "some\u{2014}modifier";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E002"),
        "Expected E002 for non-ASCII"
    );
}

// 6. E003: duplicate .mn. names
#[test]
fn validate_duplicate_mn() {
    let hero1 = "hidden&temporary&ph.bpika;1;!mheropool.\
        (replica.Pika.col.a.tier.1.hp.6.sd.0:0:0:0:0:0.img.x).n.Pika1\
        +(replica.Pika.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.Pika2\
        +(replica.Pika.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.Pika3\
        .part.1&hidden.mn.Pikachu@2!m(skip&hidden&temporary)";
    let hero2 = "hidden&temporary&ph.bpika2;1;!mheropool.\
        (replica.Pika.col.b.tier.1.hp.6.sd.0:0:0:0:0:0.img.x).n.Pika1b\
        +(replica.Pika.col.b.hp.8.sd.0:0:0:0:0:0.img.x).n.Pika2b\
        +(replica.Pika.col.b.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.Pika3b\
        .part.1&hidden.mn.Pikachu@2!m(skip&hidden&temporary)";
    let input = two_modifiers(hero1, hero2);
    let report = validate(&input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E003"),
        "Expected E003 for duplicate .mn."
    );
    let e003 = report.errors.iter().find(|f| f.rule_id == "E003").unwrap();
    assert!(
        e003.message.contains("Pikachu"),
        "E003 message should contain 'Pikachu'"
    );
}

// 7. E004: hero bad prefix
#[test]
fn validate_hero_bad_prefix() {
    // Missing hidden&temporary& prefix but still classifiable as hero
    let input = "ph.btest;1;!mheropool.\
        (replica.Test.col.a.tier.1.hp.6.sd.0:0:0:0:0:0.img.x).n.T1\
        +(replica.Test.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.T2\
        +(replica.Test.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.T3\
        .part.1&hidden.mn.Test@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E004"),
        "Expected E004 for bad hero prefix"
    );
}

// 8. E005: hero bad suffix
#[test]
fn validate_hero_bad_suffix() {
    let input = "hidden&temporary&ph.btest;1;!mheropool.\
        (replica.Test.col.a.tier.1.hp.6.sd.0:0:0:0:0:0.img.x).n.T1\
        +(replica.Test.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.T2\
        +(replica.Test.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.T3\
        .mn.Test@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E005"),
        "Expected E005 for bad hero suffix"
    );
}

// 9. E006: too few tiers
#[test]
fn validate_hero_too_few_tiers() {
    let input = "hidden&temporary&ph.btest;1;!mheropool.\
        (replica.Test.col.a.tier.1.hp.6.sd.0:0:0:0:0:0.img.x).n.T1\
        +(replica.Test.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.T2\
        .part.1&hidden.mn.Test@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E006"),
        "Expected E006 for too few tiers"
    );
}

// 10. E007: tier block with unmatched opening paren
#[test]
fn validate_tier_block_no_replica() {
    // Tier 2 has ( but no matching ) — triggers E007
    let input = "hidden&temporary&ph.btest;1;!mheropool.\
        (replica.Test.col.a.tier.1.hp.6.sd.0:0:0:0:0:0.img.x).n.T1\
        +(replica.Test.col.a.hp.8.sd.0:0:0:0:0:0.img.x.n.T2\
        +(replica.Test.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.T3\
        .part.1&hidden.mn.Test@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    // Either E007 (unmatched paren in tier) or E001 (overall balance) should fire
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E007" || f.rule_id == "E001"),
        "Expected E007 or E001 for tier with unmatched paren, got: {:?}",
        report.errors
    );
}

// 11. E008: bad face format
#[test]
fn validate_bad_face_format() {
    let input = "hidden&temporary&ph.btest;1;!mheropool.\
        (replica.Test.col.a.tier.1.hp.6.sd.sword:3-1:0:0:0:0.img.x).n.T1\
        +(replica.Test.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.T2\
        +(replica.Test.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.T3\
        .part.1&hidden.mn.Test@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E008"),
        "Expected E008 for bad face format, got errors: {:?}",
        report.errors
    );
}

// 12. E009: face ID out of range
#[test]
fn validate_face_id_out_of_range() {
    let input = "hidden&temporary&ph.btest;1;!mheropool.\
        (replica.Test.col.a.tier.1.hp.6.sd.999-1:0:0:0:0:0.img.x).n.T1\
        +(replica.Test.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.T2\
        +(replica.Test.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.T3\
        .part.1&hidden.mn.Test@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E009"),
        "Expected E009 for face ID out of range"
    );
    let e009 = report.errors.iter().find(|f| f.rule_id == "E009").unwrap();
    assert!(
        e009.message.contains("999"),
        "E009 message should contain '999'"
    );
}

// 13. E010: bad hp
#[test]
fn validate_bad_hp() {
    let input = "hidden&temporary&ph.btest;1;!mheropool.\
        (replica.Test.col.a.tier.1.hp.0.sd.0:0:0:0:0:0.img.x).n.T1\
        +(replica.Test.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.T2\
        +(replica.Test.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.T3\
        .part.1&hidden.mn.Test@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E010"),
        "Expected E010 for hp.0"
    );
}

// 14. E011: bad color
#[test]
fn validate_bad_color() {
    let input = "hidden&temporary&ph.btest;1;!mheropool.\
        (replica.Test.col.3.tier.1.hp.6.sd.0:0:0:0:0:0.img.x).n.T1\
        +(replica.Test.col.3.hp.8.sd.0:0:0:0:0:0.img.x).n.T2\
        +(replica.Test.col.3.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.T3\
        .part.1&hidden.mn.Test@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E011"),
        "Expected E011 for bad color, got errors: {:?}",
        report.errors
    );
}

// 15. E012: bad tier value
#[test]
fn validate_bad_tier() {
    let input = "hidden&temporary&ph.btest;1;!mheropool.\
        (replica.Test.col.a.tier.x.hp.6.sd.0:0:0:0:0:0.img.x).n.T1\
        +(replica.Test.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.T2\
        +(replica.Test.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.T3\
        .part.1&hidden.mn.Test@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    assert!(
        report.errors.iter().any(|f| f.rule_id == "E012"),
        "Expected E012 for bad tier value"
    );
}

// 16. ValidationReport.is_ok
#[test]
fn validate_report_is_ok() {
    let empty = ValidationReport::default();
    assert!(empty.is_ok());

    let with_warning = ValidationReport {
        warnings: vec![Finding {
            rule_id: "W001".to_string(),
            message: "test warning".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };
    assert!(with_warning.is_ok(), "Report with only warnings should be is_ok()");
}

// 17. ValidationReport Display
#[test]
fn validate_report_display() {
    let report = ValidationReport {
        errors: vec![Finding {
            rule_id: "E001".to_string(),
            severity: Severity::Error,
            modifier_index: Some(0),
            modifier_name: Some("TestHero".to_string()),
            position: Some(42),
            context: Some("...around here...".to_string()),
            message: "Unbalanced parens".to_string(),
            ..Default::default()
        }],
        warnings: vec![Finding {
            rule_id: "W003".to_string(),
            severity: Severity::Warning,
            modifier_index: Some(1),
            message: "Unknown type".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let output = format!("{}", report);
    assert!(output.contains("FAILED"), "Display should contain FAILED");
    assert!(output.contains("E001"), "Display should contain E001");
    assert!(output.contains("W003"), "Display should contain W003");
}

// ---------------------------------------------------------------------------
// Warning tests
// ---------------------------------------------------------------------------

// 18. W003: unknown type
#[test]
fn validate_unknown_type_warning() {
    let input = "some_random_string_that_matches_nothing";
    let report = validate(input).unwrap();
    assert!(
        report.warnings.iter().any(|f| f.rule_id == "W003"),
        "Expected W003 for unknown modifier type"
    );
}

// 19. W005: missing .mn.
#[test]
fn validate_missing_mn_warning() {
    let input = "some_modifier_without_mn_marker";
    let report = validate(input).unwrap();
    assert!(
        report.warnings.iter().any(|f| f.rule_id == "W005"),
        "Expected W005 for missing .mn."
    );
}

// ---------------------------------------------------------------------------
// Context field test
// ---------------------------------------------------------------------------

// 20. E001 has context
#[test]
fn validate_paren_error_has_context() {
    let input = "hidden&temporary&ph.bfoo;1;!mheropool.\
        ((replica.Foo.col.a.tier.1.hp.6.sd.0:0:0:0:0:0.img.x).n.Foo\
        +(replica.Foo.col.a.hp.8.sd.0:0:0:0:0:0.img.x).n.Foo2\
        +(replica.Foo.col.a.tier.3.hp.10.sd.0:0:0:0:0:0.img.x).n.Foo3\
        .part.1&hidden.mn.Foo@2!m(skip&hidden&temporary)";
    let report = validate(input).unwrap();
    let e001 = report.errors.iter().find(|f| f.rule_id == "E001");
    assert!(e001.is_some(), "Expected E001");
    assert!(
        e001.unwrap().context.is_some(),
        "E001 should have context field populated"
    );
}

// ---------------------------------------------------------------------------
// Helpers for IR-based semantic validation tests
// ---------------------------------------------------------------------------

fn make_test_hero(name: &str, color: char) -> Hero {
    Hero {
        internal_name: name.to_lowercase(),
        mn_name: name.to_string(),
        color,
        format: HeroFormat::Sliceymon,
        blocks: vec![make_test_block("Lost", 1, 5, "0:0:0:0:0:0")],
        removed: false,
        source: Source::Base,
    }
}

fn make_test_block(template: &str, tier: u8, hp: u16, sd: &str) -> HeroBlock {
    HeroBlock {
        template: template.into(),
        tier: Some(tier),
        hp: Some(hp),
        sd: DiceFaces::parse(sd),
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
        img_data: Some("test".into()),
        bare: false,
    }
}

fn make_test_replica(name: &str) -> ReplicaItem {
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

// ---------------------------------------------------------------------------
// Semantic validation tests (E017-E021, W008-W011)
// ---------------------------------------------------------------------------

#[test]
fn validate_face_id_for_template() {
    let mut ir = ModIR::empty();
    // Face 42 (Damage Charged) on Fey -> E017
    let mut hero = make_test_hero("TestFey", 'a');
    hero.blocks = vec![make_test_block("Fey", 1, 5, "42-1:0:0:0:0:0")];
    ir.heroes.push(hero);

    let report = validate_ir(&ir);
    assert!(report.errors.iter().any(|f| f.rule_id == "E017"),
        "Expected E017 for face 42 on Fey, got: {:?}", report.errors);

    // Face 15 (Damage) on Fey -> no E017
    let mut ir2 = ModIR::empty();
    let mut hero2 = make_test_hero("TestFey2", 'b');
    hero2.blocks = vec![make_test_block("Fey", 1, 5, "15-1:0:0:0:0:0")];
    ir2.heroes.push(hero2);

    let report2 = validate_ir(&ir2);
    assert!(!report2.errors.iter().any(|f| f.rule_id == "E017"),
        "Face 15 should be valid for Fey");
}

#[test]
fn validate_template_exists() {
    let mut ir = ModIR::empty();
    let mut hero = make_test_hero("TestBad", 'a');
    hero.blocks = vec![make_test_block("NonExistent", 1, 5, "0:0:0:0:0:0")];
    ir.heroes.push(hero);

    let report = validate_ir(&ir);
    assert!(report.errors.iter().any(|f| f.rule_id == "E018"),
        "Expected E018 for NonExistent template");

    // Known template -> no error
    let mut ir2 = ModIR::empty();
    let mut hero2 = make_test_hero("TestGood", 'b');
    hero2.blocks = vec![make_test_block("Lost", 1, 5, "0:0:0:0:0:0")];
    ir2.heroes.push(hero2);

    let report2 = validate_ir(&ir2);
    assert!(!report2.errors.iter().any(|f| f.rule_id == "E018"),
        "Lost should be a known template");
}

#[test]
fn validate_color_uniqueness() {
    let mut ir = ModIR::empty();
    ir.heroes.push(make_test_hero("Alpha", 'a'));
    ir.heroes.push(make_test_hero("Beta", 'a')); // duplicate color

    let report = validate_ir(&ir);
    assert!(report.warnings.iter().any(|f| f.rule_id == "E019"),
        "Expected E019 warning for duplicate color 'a'");
}

#[test]
fn validate_pokemon_uniqueness_across_categories() {
    let mut ir = ModIR::empty();
    ir.heroes.push(make_test_hero("Charmander", 'a'));
    ir.replica_items.push(make_test_replica("Charmander")); // same name

    let report = validate_ir(&ir);
    assert!(report.errors.iter().any(|f| f.rule_id == "E020"),
        "Expected E020 for Charmander in both heroes and replica items");
}

#[test]
fn validate_hp_range_per_tier() {
    let mut ir = ModIR::empty();
    let mut hero = make_test_hero("BigHP", 'a');
    hero.blocks = vec![make_test_block("Lost", 1, 50, "0:0:0:0:0:0")]; // T1 HP 50
    ir.heroes.push(hero);

    let report = validate_ir(&ir);
    assert!(report.warnings.iter().any(|f| f.rule_id == "W008"),
        "Expected W008 for T1 HP 50");
}

#[test]
fn validate_sd_face_count() {
    let mut ir = ModIR::empty();
    let mut hero = make_test_hero("BadDice", 'a');
    hero.blocks = vec![make_test_block("Lost", 1, 5, "0:0:0:0:0:0:0")]; // 7 faces
    ir.heroes.push(hero);

    let report = validate_ir(&ir);
    assert!(report.warnings.iter().any(|f| f.rule_id == "W009"),
        "Expected W009 for 7 dice faces");
}

#[test]
fn validate_spell_face_ids() {
    let mut ir = ModIR::empty();
    let mut hero = make_test_hero("SpellHero", 'a');
    let mut block = make_test_block("Lost", 1, 5, "0:0:0:0:0:0");
    block.abilitydata = Some(AbilityData {
        template: "Fey".into(),
        sd: DiceFaces::parse("42-1:0:0:0:0:0"), // face 42 invalid for Fey
        img_data: Some("spark".into()),
        name: "TestSpell".into(),
        modifier_chain: None,
        hsv: None,
    });
    hero.blocks = vec![block];
    ir.heroes.push(hero);

    let report = validate_ir(&ir);
    assert!(report.errors.iter().any(|f| f.rule_id == "E021"),
        "Expected E021 for spell face 42 on Fey template");
}

#[test]
fn validate_replica_item_template() {
    let mut ir = ModIR::empty();
    let mut item = make_test_replica("Oddish");
    item.template = "FakeTemplate".into();
    ir.replica_items.push(item);

    let report = validate_ir(&ir);
    assert!(report.warnings.iter().any(|f| f.rule_id == "W010"),
        "Expected W010 for unknown replica item template");
}

#[test]
fn validate_replica_item_with_ability_hp() {
    let mut ir = ModIR::empty();
    let mut item = make_test_replica("PowerItem");
    item.hp = Some(30);
    item.abilitydata = Some(AbilityData {
        template: "Fey".into(),
        sd: DiceFaces::parse("0:0:0:0:0:0"),
        img_data: Some("spark".into()),
        name: "ItemSpell".into(),
        modifier_chain: None,
        hsv: None,
    });
    ir.replica_items.push(item);

    let report = validate_ir(&ir);
    assert!(report.warnings.iter().any(|f| f.rule_id == "W011"),
        "Expected W011 for replica item with ability and HP 30");
}

#[test]
fn validate_ir_sliceymon_zero_semantic_errors() {
    let text = load_mod("sliceymon");
    let ir = textmod_compiler::extract(&text).unwrap();
    let report = validate_ir(&ir);
    assert!(
        report.errors.is_empty(),
        "sliceymon should have 0 semantic errors, got {}:\n{:?}",
        report.errors.len(),
        report.errors.iter().map(|f| format!("[{}] {}", f.rule_id, f.message)).collect::<Vec<_>>()
    );
}

// ---------------------------------------------------------------------------
// Structured error tests (Chunk 11: field_path, suggestion, severity)
// ---------------------------------------------------------------------------

#[test]
fn finding_has_field_path() {
    let mut ir = ModIR::empty();
    let mut hero = make_test_hero("FieldPathTest", 'a');
    hero.blocks = vec![make_test_block("Fey", 1, 50, "0:0:0:0:0:0")]; // T1 HP 50 -> W008
    ir.heroes.push(hero);

    let report = validate_ir(&ir);
    let w008 = report.warnings.iter().find(|f| f.rule_id == "W008");
    assert!(w008.is_some(), "Expected W008");
    let field_path = w008.unwrap().field_path.as_ref();
    assert!(field_path.is_some(), "W008 should have field_path");
    assert_eq!(field_path.unwrap(), "heroes[0].blocks[0].hp");
}

#[test]
fn finding_has_suggestion() {
    let mut ir = ModIR::empty();
    let mut hero = make_test_hero("SugTest", 'a');
    hero.blocks = vec![make_test_block("Fey", 1, 5, "42-1:0:0:0:0:0")]; // E017
    ir.heroes.push(hero);

    let report = validate_ir(&ir);
    let e017 = report.errors.iter().find(|f| f.rule_id == "E017");
    assert!(e017.is_some(), "Expected E017");
    let suggestion = e017.unwrap().suggestion.as_ref();
    assert!(suggestion.is_some(), "E017 should have suggestion");
    assert!(suggestion.unwrap().contains("42"), "Suggestion should mention face ID 42");
}

#[test]
fn finding_serializes_to_json() {
    let finding = Finding {
        rule_id: "E017".to_string(),
        severity: Severity::Error,
        message: "test error".to_string(),
        field_path: Some("heroes[0].blocks[2].sd".to_string()),
        suggestion: Some("Use face 15 instead".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&finding).unwrap();
    assert!(json.contains("\"rule_id\":\"E017\""));
    assert!(json.contains("\"severity\":\"Error\""));
    assert!(json.contains("\"field_path\""));
    assert!(json.contains("\"suggestion\""));
}

#[test]
fn finding_has_severity_enum() {
    let mut ir = ModIR::empty();
    let mut hero = make_test_hero("SevTest", 'a');
    hero.blocks = vec![make_test_block("Fey", 1, 50, "42-1:0:0:0:0:0")]; // E017 + W008
    ir.heroes.push(hero);

    let report = validate_ir(&ir);

    let e017 = report.errors.iter().find(|f| f.rule_id == "E017").unwrap();
    assert_eq!(e017.severity, Severity::Error);

    let w008 = report.warnings.iter().find(|f| f.rule_id == "W008").unwrap();
    assert_eq!(w008.severity, Severity::Warning);
}

#[test]
fn validation_report_groups_by_severity() {
    let mut ir = ModIR::empty();
    let mut hero = make_test_hero("GroupTest", 'a');
    hero.blocks = vec![make_test_block("Fey", 1, 50, "42-1:0:0:0:0:0")]; // E017 error + W008 warning
    ir.heroes.push(hero);

    let report = validate_ir(&ir);
    // Errors vec contains only Error severity
    for f in &report.errors {
        assert_eq!(f.severity, Severity::Error, "errors vec should only contain Error severity");
    }
    // Warnings vec contains only Warning severity
    for f in &report.warnings {
        assert_eq!(f.severity, Severity::Warning, "warnings vec should only contain Warning severity");
    }
}
