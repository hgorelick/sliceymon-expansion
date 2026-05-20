use crate::error::CompilerError;
use crate::finding::{Finding, Severity};
use crate::ir::{DerivedKind, Hero, ModIR, ReplicaItem, Source, StructuralModifier, StructuralType};

/// X010 — derived structural (`CharacterSelection`, `HeroPoolBase`,
/// `PoolReplacement`, hero-bound `ItemPool`) present in IR. Authoring derived
/// structurals is unsupported; they are regenerated from content. For
/// `Source::Base` / `Source::Overlay` input we strip + warn. For
/// `Source::Custom` input it is a
/// [`CompilerError::derived_structural_authored`] (backed by
/// [`crate::error::ErrorKind::DerivedStructuralAuthored`]) — see SPEC §4
/// derived-structural rule.
pub const X010: &str = "X010";

const X010_SUGGESTION: &str = "Derived structurals are regenerated at build time; authoring them directly is unsupported.";

/// Scan `structural` for a `Source::Custom` derived modifier; if found,
/// return the SPEC §4 category error with a `{label}.structural[{orig_i}]`
/// `field_path`. Used as a preflight by `merge` so the error path is
/// transactional across the whole merge (not just the individual strip call).
///
/// Derived-ness is decided by [`StructuralModifier::is_derived`] (flag +
/// kind match); see that method's docs for the classification contract.
pub fn check_no_custom_derived(
    structural: &[StructuralModifier],
    label: &str,
) -> Result<(), CompilerError> {
    if let Some((orig_i, s)) = structural
        .iter()
        .enumerate()
        .find(|(_, s)| s.is_derived() && s.source == Source::Custom)
    {
        return Err(CompilerError::derived_structural_authored(format!(
            "{:?}{}",
            s.modifier_type,
            s.name.as_deref().map(|n| format!(" ({})", n)).unwrap_or_default()
        ))
        .with_field_path(format!("{}.structural[{}]", label, orig_i))
        .with_suggestion(X010_SUGGESTION));
    }
    Ok(())
}

/// Strip derived structurals from `structural` according to SPEC §4's
/// provenance-gated rule:
///
/// - `Source::Custom` → returns `Err(CompilerError)` carrying
///   [`crate::error::ErrorKind::DerivedStructuralAuthored`] (constructed via
///   [`CompilerError::derived_structural_authored`]; see
///   [`check_no_custom_derived`] — this function runs the same scan as a
///   preflight before any mutation).
/// - `Source::Base` / `Source::Overlay` → strip and append an `X010`
///   `Severity::Warning` `Finding` to `warnings`. The finding's `field_path`
///   uses the caller's ORIGINAL input index (via drain+enumerate); its
///   `modifier_index` mirrors it.
///
/// `label` names the input side (`"base"` / `"overlay"` / `"build"`) and is
/// used in the finding's `field_path` so downstream tools can attribute the
/// strip. Derived-ness is decided by [`StructuralModifier::is_derived`].
pub fn strip_derived_structurals(
    structural: &mut Vec<StructuralModifier>,
    warnings: &mut Vec<Finding>,
    label: &str,
) -> Result<(), CompilerError> {
    // Pass 1: error out BEFORE mutating `structural` if any Custom-authored
    // derived structural is present, so the caller's IR isn't left half-stripped
    // on the error path. The reported `field_path` is the item's original index.
    check_no_custom_derived(structural, label)?;

    // Pass 2: partition into keep/strip. We need the caller's ORIGINAL index
    // in `field_path` — a running index after `Vec::remove` collapses is
    // misleading (two head-of-list strips would both report `[0]`). Drain +
    // enumerate gives us the original index; non-derived items are pushed back
    // in order.
    let taken: Vec<(usize, StructuralModifier)> = structural.drain(..).enumerate().collect();
    for (orig_i, s) in taken {
        if s.is_derived() {
            let item_source = s.source;
            warnings.push(Finding {
                rule_id: X010.to_string(),
                severity: Severity::Warning,
                message: format!(
                    "stripped derived structural {:?}{} (regenerated at build time)",
                    s.modifier_type,
                    s.name.as_deref().map(|n| format!(" \"{}\"", n)).unwrap_or_default()
                ),
                field_path: Some(format!("{}.structural[{}]", label, orig_i)),
                suggestion: Some(X010_SUGGESTION.to_string()),
                modifier_index: Some(orig_i),
                modifier_name: s.name,
                source: Some(item_source),
                ..Default::default()
            });
        } else {
            structural.push(s);
        }
    }
    Ok(())
}

/// Merge a base ModIR with an overlay ModIR.
///
/// Merge rules:
/// - Heroes: overlay heroes REPLACE base heroes with the same `internal_name`.
///   New heroes ADDED. Heroes with `removed: true` are filtered out.
/// - ReplicaItems: match by `(target_name, trigger discriminant)`, replace
///   or add. A single `target_name` may carry one ReplicaItem per trigger
///   variant (`SummonTrigger::SideUse` and `SummonTrigger::Cast`), so the
///   merge key must include the trigger discriminant — matching by
///   `target_name` alone would silently collapse SideUse + Cast pairs into
///   the first match.
/// - Monsters: match by `name`, replace or add.
/// - Bosses: match by `name`, replace or add.
/// - Structural: overlay structural modifiers REPLACE base modifiers with
///   matching `(modifier_type, name)` pair. Unknown types are appended.
/// - Derived structurals (char selection, hero pool base, pool replacement,
///   hero-bound item pool) are stripped from both sides per SPEC §4's
///   provenance-gated rule — `Source::Custom` derived structural → error,
///   `Source::Base`/`Source::Overlay` → strip + X010 warning into
///   `base.warnings`.
///
/// All overlay items are marked with `Source::Overlay`.
///
/// Signature matches SPEC §5 canonical form: `&mut base` plus
/// `Result<(), CompilerError>`. Warnings accumulate on `base.warnings` — they
/// are NOT reset, so successive `merge` calls compose.
pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError> {
    // Preflight: scan BOTH sides for Custom-authored derived structurals BEFORE
    // any mutation. Without this, a Custom-derived item on the overlay side
    // errors only after base has been partially merged (heroes/items/monsters/
    // bosses pushed, base.structural already stripped, X010 warnings already
    // emitted). The preflight keeps `merge` transactional on the error path.
    check_no_custom_derived(&base.structural, "base")?;
    check_no_custom_derived(&overlay.structural, "overlay")?;

    // Heroes: replace by internal_name, add new, remove marked
    for mut hero in overlay.heroes {
        hero.source = Source::Overlay;
        if let Some(pos) = base
            .heroes
            .iter()
            .position(|h| h.internal_name == hero.internal_name)
        {
            base.heroes[pos] = hero;
        } else {
            base.heroes.push(hero);
        }
    }
    base.heroes.retain(|h| !h.removed);

    // Replica items: replace by (target_name, trigger discriminant), add
    // new. The IR allows one ReplicaItem per trigger variant per
    // `target_name` (e.g. a single target with both a SideUse Pokeball and a
    // Cast spell — corpus shape that the future real parser will produce).
    // Matching on target_name alone would silently collapse such pairs.
    for mut item in overlay.replica_items {
        item.source = Source::Overlay;
        let item_disc = std::mem::discriminant(&item.trigger);
        if let Some(pos) = base
            .replica_items
            .iter()
            .position(|r| {
                r.target_name == item.target_name
                    && std::mem::discriminant(&r.trigger) == item_disc
            })
        {
            base.replica_items[pos] = item;
        } else {
            base.replica_items.push(item);
        }
    }

    // Monsters: replace by name, add new
    for mut mon in overlay.monsters {
        mon.source = Source::Overlay;
        if let Some(pos) = base.monsters.iter().position(|m| m.name == mon.name) {
            base.monsters[pos] = mon;
        } else {
            base.monsters.push(mon);
        }
    }

    // Bosses: replace by name, add new
    for mut boss in overlay.bosses {
        boss.source = Source::Overlay;
        if let Some(pos) = base.bosses.iter().position(|b| b.name == boss.name) {
            base.bosses[pos] = boss;
        } else {
            base.bosses.push(boss);
        }
    }

    // Strip derived structurals from base BEFORE merging overlay structural —
    // overlay may carry its own derived copies, which we strip next. Snapshot
    // which kinds were stripped so we can regenerate after content is merged.
    let mut stripped_kinds = collect_stripped_kinds(&base.structural);
    strip_derived_structurals(
        &mut base.structural,
        &mut base.warnings,
        "base",
    )?;

    // Strip overlay derived structurals into an intermediate vec so warnings
    // land on `base.warnings` with an "overlay"-labeled field_path. We do NOT
    // pre-bump overlay item sources before strip: the preflight at the top of
    // merge already rejected any Custom-derived overlay item, and strip's
    // classifier treats Source::Base and Source::Overlay identically. The
    // post-strip merge loop below is the single authority for "overlay items
    // become Source::Overlay."
    let mut overlay_structural = overlay.structural;
    for kind in collect_stripped_kinds(&overlay_structural) {
        if !stripped_kinds.contains(&kind) {
            stripped_kinds.push(kind);
        }
    }
    strip_derived_structurals(
        &mut overlay_structural,
        &mut base.warnings,
        "overlay",
    )?;

    // Structural: replace by (modifier_type, name) pair, append otherwise.
    // This loop is where overlay items acquire Source::Overlay — no earlier
    // site should mutate `s.source` for overlay structurals.
    for mut s in overlay_structural {
        s.source = Source::Overlay;
        if let Some(pos) = base
            .structural
            .iter()
            .position(|bs| bs.modifier_type == s.modifier_type && bs.name == s.name)
        {
            base.structural[pos] = s;
        } else {
            base.structural.push(s);
        }
    }

    // Overlay warnings are accumulated onto base.warnings. The overlay's own
    // warnings (if any) also carry over, so the final ModIR sees the union.
    base.warnings.extend(overlay.warnings);

    // Regenerate the derived kinds we stripped, against the merged content.
    // Merge operates at the IR level with no build-time filter concept, so
    // pass the full merged hero set + full replica list. `build_with` applies
    // its own filter-aware regen pass on top of a local clone. Disjoint-field
    // borrow: `structural` is mut-borrowed while `heroes` and `replica_items`
    // are immut-borrowed.
    regenerate_derived_kinds(
        &mut base.structural,
        &base.heroes,
        &base.replica_items,
        &stripped_kinds,
    );

    Ok(())
}

/// Snapshot the `StructuralType`s of any derived-flagged modifier in
/// `structural`. Returned set lets callers restore the same kinds after a
/// strip-regenerate cycle.
pub fn collect_stripped_kinds(structural: &[StructuralModifier]) -> Vec<StructuralType> {
    let mut kinds = Vec::new();
    for s in structural.iter().filter(|s| s.is_derived()) {
        if !kinds.contains(&s.modifier_type) {
            kinds.push(s.modifier_type.clone());
        }
    }
    kinds
}

/// Regenerate derived structurals for the listed kinds into `structural`,
/// deriving from the supplied `heroes` and `replica_items` slices.
///
/// Used by both `merge` (post-strip, pre-return — passes the full merged
/// hero set and full replica list) and `build_with` (passes the
/// post-`BuildOptions` filter hero set + full replica list, so the
/// 2026-04-22 "BuildOptions + provenance-aware findings" ruling's
/// "regenerated from the post-filter content set" contract is observable
/// for heroes; replica indices are preserved across the filter because
/// `ItempoolItem::Summon(i)` indexes the flat `replica_items` list).
/// Only regenerates kinds that were present-and-stripped — this preserves
/// format-specific roundtrip (sliceymon's inline `!mheropool.` encoding has
/// no separate char-selection Selector, and nothing in this function adds
/// one unless the input already had one marked derived).
///
/// Every derived kind has a wired regenerator: `Selector` via
/// `derived::generate_char_selection`, `HeroPoolBase` via
/// `derived::generate_hero_pool_base`, `PoolReplacement` via
/// `derived::generate_pool_replacement` (canonical shape; widened by the
/// future typed-payload retype of `StructuralContent::PoolReplacement` per
/// decisions.md 2026-05-19), and hero-bound `ItemPool` via
/// `derived::generate_hero_item_pool` against the trigger-based replica list.
///
/// Contract: callers must invoke `strip_derived_structurals` on `structural`
/// before calling this — `collect_stripped_kinds` returns a deduped `kinds`
/// list from the pre-strip structural, and strip guarantees no `derived:true`
/// items remain, so each regenerated kind is pushed exactly once.
///
/// Dispatch shape per decisions.md 2026-05-13 "`DerivedKind` is single
/// source-of-truth; `is_derived()` delegates": this outer function filters
/// `StructuralType` to [`DerivedKind`] via `TryFrom`, delegating to
/// `regenerate_derived_kind_inner` which is exhaustive over `DerivedKind`'s
/// four variants. Rust's compiler-enforced exhaustiveness check on
/// `DerivedKind` is the load-bearing property: flipping a fifth
/// `StructuralType` variant to derived requires only extending `DerivedKind`,
/// which then forces the inner exhaustive-match update.
pub fn regenerate_derived_kinds(
    structural: &mut Vec<StructuralModifier>,
    heroes: &[Hero],
    replica_items: &[ReplicaItem],
    kinds: &[StructuralType],
) {
    // Outer/inner dispatch split (chunk complete-regenerators, 2026-05-19):
    // the `_ => {}` wildcard sink that previously deferred `PoolReplacement`
    // retired here. Upstream callers build `kinds` via `collect_stripped_kinds`,
    // which filters by `is_derived()` — and `is_derived()` returns `true` only
    // when `DerivedKind::try_from` succeeds — so the outer `Err` arm is
    // structurally unreachable; reaching it indicates a caller bypassed the
    // upstream-filter discipline.
    if heroes.is_empty() || kinds.is_empty() {
        return;
    }
    for kind in kinds {
        match DerivedKind::try_from(kind) {
            Ok(dk) => regenerate_derived_kind_inner(structural, heroes, replica_items, dk),
            Err(_) => unreachable!(
                "regenerate_derived_kinds reached a non-derived StructuralType ({:?}). \
                 collect_stripped_kinds builds `kinds` via `is_derived()`, and `is_derived()` \
                 returns true only when `DerivedKind::try_from` succeeds — reaching this arm \
                 means a caller bypassed `collect_stripped_kinds`-style filtering before \
                 invoking `regenerate_derived_kinds`.",
                kind
            ),
        }
    }
}

/// Inner dispatch — exhaustive over [`DerivedKind`]. Adding a fifth variant
/// to `DerivedKind` forces this match's update at compile time, per the
/// chunk's "compile-time enforcement of the derived-allowed-set" property.
fn regenerate_derived_kind_inner(
    structural: &mut Vec<StructuralModifier>,
    heroes: &[Hero],
    replica_items: &[ReplicaItem],
    kind: DerivedKind,
) {
    match kind {
        DerivedKind::Selector => {
            structural.push(crate::builder::derived::generate_char_selection(heroes));
        }
        DerivedKind::HeroPoolBase => {
            structural.push(crate::builder::derived::generate_hero_pool_base(heroes));
        }
        DerivedKind::PoolReplacement => {
            structural.push(crate::builder::derived::generate_pool_replacement(heroes));
        }
        DerivedKind::ItemPool => {
            structural.extend(
                crate::builder::derived::generate_hero_item_pool(heroes, replica_items),
            );
        }
    }
}

#[cfg(test)]
mod per_arm_regenerator_guards {
    //! Per-arm correctness guards for `regenerate_derived_kinds`. Each
    //! `DerivedKind` variant gets a minimum-cardinality input and asserts the
    //! dispatched arm appends a non-empty modifier of the correct type. The
    //! `ItemPool` arm is covered by
    //! `derived::tests::regenerate_derived_kinds_rebuilds_hero_item_pool` and
    //! is not duplicated here; this module covers the three other arms
    //! (`Selector`, `HeroPoolBase`, `PoolReplacement`) so each has a
    //! per-variant correctness guard alongside the compile-time
    //! exhaustiveness check that `DerivedKind`-driven dispatch provides.
    use super::*;
    use crate::ir::{DiceFaces, HeroBlock, HeroFormat, StructuralContent};

    fn make_hero(name: &str, color: char) -> Hero {
        Hero {
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
                sprite: crate::authoring::SpriteId::owned(name.to_string(), "test"),
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
            source: Source::Base,
        }
    }

    #[test]
    fn selector_arm_appends_one_derived_selector() {
        let heroes = vec![make_hero("Alpha", 'a'), make_hero("Beta", 'b')];
        let mut structural: Vec<StructuralModifier> = Vec::new();
        regenerate_derived_kinds(&mut structural, &heroes, &[], &[StructuralType::Selector]);
        assert_eq!(structural.len(), 1, "Selector arm regenerates exactly one modifier");
        assert_eq!(structural[0].modifier_type, StructuralType::Selector);
        assert!(structural[0].derived);
        match &structural[0].content {
            StructuralContent::Selector { body, options } => {
                assert_eq!(
                    body,
                    "ph.sChoose a Party@1[Alpha][Beta]@2!mparty.Alpha+Beta"
                );
                assert_eq!(options, &["Alpha", "Beta"]);
            }
            other => panic!("expected Selector content, got {:?}", other),
        }
    }

    #[test]
    fn hero_pool_base_arm_appends_one_derived_hero_pool_base() {
        let heroes = vec![make_hero("Alpha", 'a')];
        let mut structural: Vec<StructuralModifier> = Vec::new();
        regenerate_derived_kinds(&mut structural, &heroes, &[], &[StructuralType::HeroPoolBase]);
        assert_eq!(structural.len(), 1, "HeroPoolBase arm regenerates exactly one modifier");
        assert_eq!(structural[0].modifier_type, StructuralType::HeroPoolBase);
        assert!(structural[0].derived);
        match &structural[0].content {
            StructuralContent::HeroPoolBase { hero_refs, .. } => {
                assert_eq!(hero_refs, &["alpha"]);
            }
            other => panic!("expected HeroPoolBase content, got {:?}", other),
        }
    }

    #[test]
    fn pool_replacement_arm_appends_one_derived_pool_replacement() {
        let heroes = vec![make_hero("Alpha", 'a'), make_hero("Beta", 'b')];
        let mut structural: Vec<StructuralModifier> = Vec::new();
        regenerate_derived_kinds(
            &mut structural,
            &heroes,
            &[],
            &[StructuralType::PoolReplacement],
        );
        assert_eq!(
            structural.len(),
            1,
            "PoolReplacement arm regenerates exactly one modifier (no longer a deferred sink)"
        );
        assert_eq!(structural[0].modifier_type, StructuralType::PoolReplacement);
        assert!(structural[0].derived);
        match &structural[0].content {
            StructuralContent::PoolReplacement { body, hero_names } => {
                assert_eq!(body, "((heropool.alpha+beta))");
                assert_eq!(hero_names, &["alpha", "beta"]);
            }
            other => panic!("expected PoolReplacement content, got {:?}", other),
        }
    }
}
