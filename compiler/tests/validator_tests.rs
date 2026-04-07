use textmod_compiler::validator::{validate, ValidationReport, Finding};

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
            modifier_index: None,
            modifier_name: None,
            position: None,
            context: None,
            message: "test warning".to_string(),
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
            modifier_index: Some(0),
            modifier_name: Some("TestHero".to_string()),
            position: Some(42),
            context: Some("...around here...".to_string()),
            message: "Unbalanced parens".to_string(),
        }],
        warnings: vec![Finding {
            rule_id: "W003".to_string(),
            modifier_index: Some(1),
            modifier_name: None,
            position: None,
            context: None,
            message: "Unknown type".to_string(),
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
