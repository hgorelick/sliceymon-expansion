//! Chunk 5 / SPEC §4 / plan §F6 — derived-structural provenance gate + new
//! `merge(&mut base, overlay)` signature + warnings sidecar.
//!
//! These tests pin the behavior that:
//! - `merge` takes `&mut base` and returns `Result<(), CompilerError>`.
//! - Stripping derived structurals is provenance-gated:
//!     - `Source::Custom` → `CompilerError::DerivedStructuralAuthored`.
//!     - `Source::Base` / `Source::Overlay` → strip + `X010` Warning onto
//!       `base.warnings`.
//! - `build` applies the same provenance gate and regenerates stripped kinds.
//! - Warnings accumulate across successive `merge` calls (do not reset).

use textmod_compiler::error::ErrorKind;
use textmod_compiler::finding::Severity;
use textmod_compiler::ir::{
    DiceFaces, HeroBlock, HeroFormat, ModIR, Source, StructuralContent, StructuralModifier,
    StructuralType,
};
use textmod_compiler::{merge, Finding, SpriteId};
use textmod_compiler::ir::merge::X010;

fn make_hero(name: &str, color: char, source: Source) -> textmod_compiler::ir::Hero {
    textmod_compiler::ir::Hero {
        internal_name: name.to_lowercase(),
        mn_name: name.to_string(),
        color,
        format: HeroFormat::Sliceymon,
        blocks: vec![HeroBlock {
            template: "Lost".into(),
            tier: Some(1),
            hp: Some(5),
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            color: None,
            sprite: SpriteId::owned(name.to_string(), "test"),
            speech: "!".into(),
            name: name.into(),
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
        source,
    }
}

fn derived_char_selection(source: Source) -> StructuralModifier {
    StructuralModifier {
        modifier_type: StructuralType::Selector,
        name: None,
        content: StructuralContent::Selector {
            body: "1.ph.s@1Alpha@1Beta".into(),
            options: vec!["Alpha".into(), "Beta".into()],
        },
        derived: true,
        source,
    }
}

fn non_derived_selector(source: Source, name: &str) -> StructuralModifier {
    StructuralModifier {
        modifier_type: StructuralType::Selector,
        name: Some(name.into()),
        content: StructuralContent::Selector {
            body: format!("1.ph.sAuthored selector {}", name),
            options: vec![],
        },
        derived: false,
        source,
    }
}

// -- is_derived truth table --

#[test]
fn is_derived_truth_table() {
    // Derived kinds × derived flag:
    for (ty, body) in [
        (StructuralType::Selector, StructuralContent::Selector { body: "x".into(), options: vec![] }),
        (StructuralType::HeroPoolBase, StructuralContent::HeroPoolBase { body: "x".into(), hero_refs: vec![] }),
        (StructuralType::PoolReplacement, StructuralContent::PoolReplacement { body: "x".into(), hero_names: vec![] }),
        (StructuralType::ItemPool, StructuralContent::ItemPool { body: "x".into(), items: vec![] }),
    ] {
        let mut s = StructuralModifier {
            modifier_type: ty.clone(),
            name: None,
            content: body,
            derived: true,
            source: Source::Base,
        };
        assert!(s.is_derived(), "derived:true + {:?} should be derived", ty);
        s.derived = false;
        assert!(
            !s.is_derived(),
            "derived:false + {:?} should NOT be derived (preserves authored content)",
            ty
        );
    }

    // Non-derived kinds are never derived regardless of flag:
    for ty in [
        StructuralType::PartyConfig,
        StructuralType::Dialog,
        StructuralType::BossModifier,
        StructuralType::ArtCredits,
        StructuralType::Difficulty,
    ] {
        let content = StructuralContent::from_body(&ty, "x".into());
        let s = StructuralModifier {
            modifier_type: ty.clone(),
            name: None,
            content,
            derived: true,
            source: Source::Base,
        };
        assert!(
            !s.is_derived(),
            "{:?} with derived:true should still be non-derived (not in SPEC §4 list)",
            ty
        );
    }
}

// -- merge signature: &mut base, Result<(), _> --

#[test]
fn merge_new_signature_compiles_and_returns_unit() {
    let mut base = ModIR::empty();
    base.heroes.push(make_hero("Alpha", 'a', Source::Base));

    let mut overlay = ModIR::empty();
    overlay.heroes.push(make_hero("Beta", 'b', Source::Base));

    // Must compile against the SPEC §5 signature. If this line changes shape,
    // the re-export is drifting.
    let result: Result<(), textmod_compiler::error::CompilerError> = merge(&mut base, overlay);
    result.expect("merge should succeed with non-derived overlay content");

    assert_eq!(base.heroes.len(), 2);
    // warnings is readable post-merge, even if empty
    assert!(base.warnings.is_empty(), "no derived structurals → no warnings");
}

// -- merge strips Base-source derived structural with X010 Warning --

#[test]
fn merge_strips_base_derived_char_selection_with_x010_warning() {
    let mut base = ModIR::empty();
    base.structural.push(derived_char_selection(Source::Base));

    let mut overlay = ModIR::empty();
    overlay.structural.push(derived_char_selection(Source::Base));

    merge(&mut base, overlay).expect("strip+warn, not error, for Base/Overlay");

    assert!(
        base.structural
            .iter()
            .all(|s| !(s.modifier_type == StructuralType::Selector && s.derived)),
        "derived Selector must be stripped from base.structural"
    );
    let x010_warnings: Vec<&Finding> = base.warnings.iter().filter(|w| w.rule_id == X010).collect();
    assert_eq!(
        x010_warnings.len(),
        2,
        "expected one X010 per side (base + overlay), got {:?}",
        x010_warnings
    );
    assert!(
        x010_warnings.iter().all(|f| f.severity == Severity::Warning),
        "X010 findings must be Severity::Warning"
    );
    let has_base_label = x010_warnings
        .iter()
        .any(|f| f.field_path.as_deref().map(|p| p.starts_with("base.")).unwrap_or(false));
    let has_overlay_label = x010_warnings
        .iter()
        .any(|f| f.field_path.as_deref().map(|p| p.starts_with("overlay.")).unwrap_or(false));
    assert!(has_base_label, "one X010 should be labeled base.*");
    assert!(has_overlay_label, "one X010 should be labeled overlay.*");
}

// -- merge errors on Source::Custom derived structural (SPEC §4 category error) --

#[test]
fn merge_errors_on_custom_authored_derived_structural() {
    let mut base = ModIR::empty();

    let mut overlay = ModIR::empty();
    overlay.structural.push(derived_char_selection(Source::Custom));

    let err = merge(&mut base, overlay).expect_err("Custom derived structural must error");
    match *err.kind {
        ErrorKind::DerivedStructuralAuthored { .. } => {}
        other => panic!("expected DerivedStructuralAuthored, got {:?}", other),
    }
}

// -- merge leaves non-derived Selectors alone (source-vs-IR divergence test) --

// If the strip logic were type-only ("Selector ⇒ maybe derived") instead of
// flag-gated, a Base `Selector` with `derived: false` and `name: Some(...)`
// like "Send Teams" would be incorrectly stripped. This test proves the
// implementation uses the `derived: bool` signal, not a heuristic on type.
#[test]
fn merge_preserves_non_derived_selectors_even_when_name_matches_no_hero() {
    let mut base = ModIR::empty();
    base.heroes.push(make_hero("Alpha", 'a', Source::Base));
    base.structural.push(non_derived_selector(Source::Base, "Send Teams"));

    let overlay = ModIR::empty();

    merge(&mut base, overlay).expect("non-derived Selector must survive merge");

    assert_eq!(
        base.structural.iter().filter(|s| s.modifier_type == StructuralType::Selector).count(),
        1,
        "non-derived Selector must be preserved through merge"
    );
    assert!(
        base.warnings.iter().all(|w| w.rule_id != X010),
        "no X010 warnings should fire on non-derived Selector"
    );
}

// -- merge warnings accumulate across calls --

#[test]
fn merge_warnings_accumulate_across_calls() {
    let mut base = ModIR::empty();
    base.structural.push(derived_char_selection(Source::Base));

    let mut o1 = ModIR::empty();
    o1.structural.push(derived_char_selection(Source::Base));

    let mut o2 = ModIR::empty();
    o2.structural.push(derived_char_selection(Source::Base));

    merge(&mut base, o1).unwrap();
    let after_first = base.warnings.iter().filter(|w| w.rule_id == X010).count();
    assert!(after_first >= 2, "first merge should produce at least 2 X010 warnings");

    merge(&mut base, o2).unwrap();
    let after_second = base.warnings.iter().filter(|w| w.rule_id == X010).count();
    assert!(
        after_second > after_first,
        "second merge must append (not reset) X010 warnings: before={}, after={}",
        after_first,
        after_second
    );
}

// -- build errors on Custom-authored derived structural (SPEC §4) --

#[test]
fn build_errors_on_custom_authored_derived_structural() {
    let mut ir = ModIR::empty();
    ir.heroes.push(make_hero("Alpha", 'a', Source::Base));
    ir.structural.push(derived_char_selection(Source::Custom));

    let err = textmod_compiler::build(&ir).expect_err("Custom derived must error on build");
    match *err.kind {
        ErrorKind::DerivedStructuralAuthored { .. } => {}
        other => panic!("expected DerivedStructuralAuthored from build, got {:?}", other),
    }
}

// -- field_path carries ORIGINAL input index, not post-remove running index --

// If `strip_derived_structurals` used the running index after `Vec::remove`
// collapses, two head-of-list strips would both report `structural[0]`.
// Downstream tools would see a meaningless field_path. This test pins the
// original-index semantics so a future refactor that reintroduces the
// running-index bug fails here, not in a downstream tool.
#[test]
fn strip_findings_carry_original_index_not_running_index() {
    use textmod_compiler::ir::merge::strip_derived_structurals;

    let mut structural = vec![
        derived_char_selection(Source::Base),            // original index 0
        non_derived_selector(Source::Base, "keep me"),   // original index 1
        derived_char_selection(Source::Base),            // original index 2
        non_derived_selector(Source::Base, "and me"),    // original index 3
    ];
    let mut warnings: Vec<Finding> = Vec::new();

    strip_derived_structurals(&mut structural, &mut warnings, "base")
        .expect("Base-source derived strips are warnings, not errors");

    // The two non-derived items must survive, in original order.
    assert_eq!(structural.len(), 2);
    assert_eq!(structural[0].name.as_deref(), Some("keep me"));
    assert_eq!(structural[1].name.as_deref(), Some("and me"));

    // Exactly two X010 findings, and their field_paths must be the ORIGINAL
    // input indices [0, 2] — not [0, 0] (the running-index bug) and not
    // [0, 1] (post-remove-then-skip).
    let x010s: Vec<&Finding> = warnings.iter().filter(|w| w.rule_id == X010).collect();
    assert_eq!(x010s.len(), 2, "one X010 per derived strip");
    let paths: Vec<&str> = x010s
        .iter()
        .filter_map(|w| w.field_path.as_deref())
        .collect();
    assert!(
        paths.contains(&"base.structural[0]"),
        "missing field_path for original index 0, got: {:?}",
        paths
    );
    assert!(
        paths.contains(&"base.structural[2]"),
        "missing field_path for original index 2 (running-index regression), got: {:?}",
        paths
    );
    // And the modifier_index column mirrors it structurally.
    let indices: Vec<usize> = x010s.iter().filter_map(|w| w.modifier_index).collect();
    assert!(indices.contains(&0) && indices.contains(&2), "modifier_index drift: {:?}", indices);
}

// -- error path is transactional: base.structural is untouched on Custom-err --

// If strip_derived_structurals partially drained the vec before discovering a
// Custom-derived item, the caller's IR would be left half-stripped on the
// error path. Pin the transactional contract: pre-error structural state must
// match post-error structural state byte-for-byte.
#[test]
fn strip_custom_error_preserves_structural_vec() {
    use textmod_compiler::ir::merge::strip_derived_structurals;

    let mut structural = vec![
        derived_char_selection(Source::Base),            // would be stripped...
        non_derived_selector(Source::Base, "keep"),
        derived_char_selection(Source::Custom),          // ...but this errors first
    ];
    let snapshot = structural.clone();
    let mut warnings: Vec<Finding> = Vec::new();

    let err = strip_derived_structurals(&mut structural, &mut warnings, "base")
        .expect_err("Custom-derived must error");
    match *err.kind {
        ErrorKind::DerivedStructuralAuthored { .. } => {}
        other => panic!("expected DerivedStructuralAuthored, got {:?}", other),
    }
    assert_eq!(
        structural, snapshot,
        "structural must be unchanged on the error path (no half-strip)"
    );
    assert!(
        warnings.is_empty(),
        "no X010 warnings should be emitted when the call errored"
    );
}

// -- Path C: adding a hero then building regenerates the char selection --

// Confirms the strip-regenerate cycle completes end to end: a Base-origin
// derived Selector bound to Alpha/Beta is replaced by a fresh one that
// reflects the post-merge hero set (Alpha/Beta/Gamma).
#[test]
fn path_c_merge_adds_hero_regenerates_selector() {
    let mut base = ModIR::empty();
    base.heroes.push(make_hero("Alpha", 'a', Source::Base));
    base.heroes.push(make_hero("Beta", 'b', Source::Base));
    base.structural.push(derived_char_selection(Source::Base));

    let mut overlay = ModIR::empty();
    overlay.heroes.push(make_hero("Gamma", 'c', Source::Base));

    merge(&mut base, overlay).unwrap();

    // merge stripped the derived Selector (Base → X010 warn). build regenerates.
    let output = textmod_compiler::build(&base).expect("build should succeed");
    assert!(output.contains("@1Alpha"), "regenerated selector missing Alpha");
    assert!(output.contains("@1Beta"), "regenerated selector missing Beta");
    assert!(output.contains("@1Gamma"), "regenerated selector missing Gamma");
}
