//! Chunk 4 verification tests for `BuildOptions` + `build_with` + `Finding.source`.
//! Spec: PLATFORM_FOUNDATIONS_PLAN §F5.

use textmod_compiler::authoring::SpriteId;
use textmod_compiler::ir::{
    DiceFace, DiceFaces, Hero, HeroBlock, HeroFormat, ModIR, Source,
};
use textmod_compiler::xref::{promote_severity, Severity};
use textmod_compiler::{build_with, BuildOptions, SourceFilter, SourceSet};

fn minimal_hero(name: &str, color: char, source: Source) -> Hero {
    Hero {
        internal_name: name.to_lowercase(),
        mn_name: name.to_string(),
        color,
        format: HeroFormat::default(),
        blocks: vec![HeroBlock {
            template: "Slime".to_string(),
            tier: Some(0),
            hp: Some(4),
            sd: DiceFaces { faces: vec![DiceFace::Blank] },
            bare: false,
            color: None,
            sprite: SpriteId::owned(name.to_lowercase(), ""),
            speech: String::new(),
            name: name.to_string(),
            doc: None,
            abilitydata: None,
            triggerhpdata: None,
            hue: None,
            modifier_chain: None,
            facades: vec![],
            items_inside: None,
            items_outside: None,
        }],
        removed: false,
        source,
    }
}

// -- build_with filtering --------------------------------------------------

#[test]
fn build_with_only_base_omits_overlay() {
    let mut ir = ModIR::empty();
    ir.heroes.push(minimal_hero("BaseHero", 'a', Source::Base));
    ir.heroes.push(minimal_hero("OverlayHero", 'b', Source::Overlay));

    let opts = BuildOptions {
        include: SourceFilter::Only(SourceSet::single(Source::Base)),
    };
    let textmod = build_with(&ir, &opts).expect("build_with should succeed");

    let lower = textmod.to_lowercase();
    assert!(
        lower.contains("basehero"),
        "Base hero should be emitted; got:\n{}",
        textmod
    );
    assert!(
        !lower.contains("overlayhero"),
        "Overlay hero must be filtered out when Only(Base) is requested; got:\n{}",
        textmod
    );
}

#[test]
fn build_with_exclude_base() {
    let mut ir = ModIR::empty();
    ir.heroes.push(minimal_hero("BaseHero", 'a', Source::Base));
    ir.heroes.push(minimal_hero("OverlayHero", 'b', Source::Overlay));

    let opts = BuildOptions {
        include: SourceFilter::Exclude(SourceSet::single(Source::Base)),
    };
    let textmod = build_with(&ir, &opts).expect("build_with should succeed");

    let lower = textmod.to_lowercase();
    assert!(
        !lower.contains("basehero"),
        "Base hero must be filtered out when Exclude(Base) is requested; got:\n{}",
        textmod
    );
    assert!(
        lower.contains("overlayhero"),
        "Overlay hero should be emitted; got:\n{}",
        textmod
    );
}

#[test]
fn build_default_equivalent_to_build_with_default_opts() {
    // build(ir) must be exactly build_with(ir, &BuildOptions::default()).
    let mut ir = ModIR::empty();
    ir.heroes.push(minimal_hero("A", 'a', Source::Base));
    ir.heroes.push(minimal_hero("B", 'b', Source::Custom));
    ir.heroes.push(minimal_hero("C", 'c', Source::Overlay));

    let via_default = textmod_compiler::build(&ir).expect("build ok");
    let via_with = build_with(&ir, &BuildOptions::default()).expect("build_with ok");
    assert_eq!(via_default, via_with);
}

// -- const-fn pin ----------------------------------------------------------

#[test]
fn source_filter_admits_const() {
    // Compile-time pin: `const _` items force const-evaluation, so if `admits`
    // were downgraded to a non-const fn these lines fail to compile and the
    // test binary does not build.
    const _: bool = SourceFilter::All.admits(Source::Base);
    const _: bool = SourceFilter::All.admits(Source::Custom);
    const _: bool = SourceFilter::Only(SourceSet::empty()).admits(Source::Base);

    // Runtime truth: exercises `admits` on real values (not const-eval'd) so
    // the test body does work even on a successful compile.
    let only_empty = SourceFilter::Only(SourceSet::empty());
    let all = SourceFilter::All;
    assert!(!only_empty.admits(Source::Base));
    assert!(all.admits(Source::Overlay));
}

// -- promote_severity helper cross-product ---------------------------------

#[test]
fn promote_severity_helper_table() {
    // (input_severity, input_source) → expected_severity
    let cases: &[(Severity, Option<Source>, Severity)] = &[
        // Error × {None, Base, Custom, Overlay}
        (Severity::Error, None, Severity::Error),
        (Severity::Error, Some(Source::Base), Severity::Warning),
        (Severity::Error, Some(Source::Custom), Severity::Error),
        (Severity::Error, Some(Source::Overlay), Severity::Error),
        // Warning × {None, Base, Custom, Overlay}
        (Severity::Warning, None, Severity::Warning),
        (Severity::Warning, Some(Source::Base), Severity::Warning),
        (Severity::Warning, Some(Source::Custom), Severity::Error),
        (Severity::Warning, Some(Source::Overlay), Severity::Error),
        // Info × {None, Base, Custom, Overlay}
        (Severity::Info, None, Severity::Info),
        (Severity::Info, Some(Source::Base), Severity::Warning),
        (Severity::Info, Some(Source::Custom), Severity::Error),
        (Severity::Info, Some(Source::Overlay), Severity::Error),
    ];
    for (base, src, expected) in cases {
        assert_eq!(
            &promote_severity(base.clone(), *src),
            expected,
            "promote_severity({:?}, {:?}) mismatch",
            base,
            src
        );
    }
}

// -- V-rule Finding.source population -------------------------------------

#[test]
fn v019_finding_source_populated() {
    // Two heroes with the same color → V019 fires.
    let mut ir = ModIR::empty();
    ir.heroes.push(minimal_hero("First", 'a', Source::Base));
    ir.heroes.push(minimal_hero("Second", 'a', Source::Custom));

    let report = textmod_compiler::check_references(&ir);
    let v019: Vec<_> = report
        .errors
        .iter()
        .chain(report.warnings.iter())
        .filter(|f| f.rule_id == "V019")
        .collect();
    assert_eq!(v019.len(), 1, "expected one V019 finding");
    assert!(
        v019[0].source.is_some(),
        "V019 finding must carry Finding.source"
    );
    // The second hero is the reporter (Custom). Severity: Error per §F5.
    assert_eq!(v019[0].source, Some(Source::Custom));
    assert_eq!(v019[0].severity, Severity::Error);
}

#[test]
fn v020_cross_category_source_is_global() {
    // V020 in `check_cross_category_names` is a global finding (no single
    // offender) — source=None, severity stays Error.
    use textmod_compiler::ir::{ReplicaItem, ReplicaItemContainer};
    let mut ir = ModIR::empty();
    ir.heroes.push(minimal_hero("Pikachu", 'a', Source::Base));
    ir.replica_items.push(ReplicaItem {
        name: "Pikachu".to_string(),
        container: ReplicaItemContainer::Capture { name: "Pikachu".to_string() },
        template: "Slime".to_string(),
        hp: Some(4),
        sd: DiceFaces { faces: vec![DiceFace::Blank] },
        sprite: SpriteId::owned("pikachu", ""),
        color: None,
        tier: None,
        doc: None,
        speech: None,
        abilitydata: None,
        item_modifiers: None,
        sticker: None,
        toggle_flags: None,
        source: Source::Base,
    });

    let report = textmod_compiler::check_references(&ir);
    let v020: Vec<_> = report
        .errors
        .iter()
        .chain(report.warnings.iter())
        .filter(|f| f.rule_id == "V020")
        .collect();
    assert_eq!(v020.len(), 1);
    assert_eq!(v020[0].source, None, "global V020 finding carries source=None");
    assert_eq!(v020[0].severity, Severity::Error);
}

#[test]
fn v016_finding_source_populated_base() {
    use textmod_compiler::ir::{StructuralContent, StructuralModifier, StructuralType};
    let mut ir = ModIR::empty();
    ir.heroes.push(minimal_hero("Charmander", 'a', Source::Base));
    ir.structural.push(StructuralModifier {
        modifier_type: StructuralType::HeroPoolBase,
        name: Some("heropool".to_string()),
        content: StructuralContent::HeroPoolBase {
            body: String::new(),
            hero_refs: vec!["charmander".to_string(), "nonexistent".to_string()],
        },
        derived: false,
        source: Source::Base,
    });

    let report = textmod_compiler::check_references(&ir);
    let v016: Vec<_> = report
        .errors
        .iter()
        .chain(report.warnings.iter())
        .filter(|f| f.rule_id == "V016")
        .collect();
    assert_eq!(v016.len(), 1);
    assert_eq!(v016[0].source, Some(Source::Base));
    assert_eq!(v016[0].severity, Severity::Warning);
}

#[test]
fn v016_finding_source_populated_custom() {
    use textmod_compiler::ir::{StructuralContent, StructuralModifier, StructuralType};
    let mut ir = ModIR::empty();
    ir.heroes.push(minimal_hero("Charmander", 'a', Source::Base));
    ir.structural.push(StructuralModifier {
        modifier_type: StructuralType::HeroPoolBase,
        name: Some("heropool".to_string()),
        content: StructuralContent::HeroPoolBase {
            body: String::new(),
            hero_refs: vec!["charmander".to_string(), "nonexistent".to_string()],
        },
        derived: false,
        source: Source::Custom,
    });

    let report = textmod_compiler::check_references(&ir);
    let v016: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "V016").collect();
    assert_eq!(v016.len(), 1);
    assert_eq!(v016[0].source, Some(Source::Custom));
    assert_eq!(v016[0].severity, Severity::Error);
}

// -- Finding serde shape ---------------------------------------------------

#[test]
fn finding_json_omits_absent_source() {
    // Global findings (V020 cross-category) emit `source: None`; the serde
    // skip_serializing_if keeps the emitted JSON clean — no null `source`
    // key for rules that don't bind to a single sourced entity.
    use textmod_compiler::Finding;
    let f = Finding {
        rule_id: "V020".to_string(),
        severity: Severity::Error,
        message: "duplicate".to_string(),
        source: None,
        ..Default::default()
    };
    let json = serde_json::to_string(&f).expect("serialize");
    assert!(!json.contains("\"source\""), "source=None must be skipped; got {}", json);
}
