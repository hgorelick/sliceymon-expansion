use crate::error::CompilerError;
use crate::finding::{Finding, Severity};
use crate::ir::{Hero, ModIR, Source, StructuralModifier, StructuralType};

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
/// - `Source::Custom` → returns `Err(CompilerError::DerivedStructuralAuthored)`
///   (see [`check_no_custom_derived`] — this function runs the same scan as a
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
/// - ReplicaItems: match by `name`, replace or add.
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

    // Replica items: replace by name, add new
    for mut item in overlay.replica_items {
        item.source = Source::Overlay;
        if let Some(pos) = base.replica_items.iter().position(|r| r.name == item.name) {
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
    // pass the full merged hero set. `build_with` applies its own filter-aware
    // regen pass on top of a local clone.
    let heroes = base.heroes.clone();
    regenerate_derived_kinds(&mut base.structural, &heroes, &stripped_kinds);

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
/// deriving from the supplied `heroes` slice.
///
/// Used by both `merge` (post-strip, pre-return — passes the full merged
/// hero set) and `build_with` (passes the post-`BuildOptions` filter hero
/// set, so plan §F5's "regenerated from the post-filter content set"
/// contract is observable). Only regenerates kinds that were
/// present-and-stripped — this preserves format-specific roundtrip
/// (sliceymon's inline `!mheropool.` encoding has no separate char-selection
/// Selector, and nothing in this function adds one unless the input already
/// had one marked derived).
///
/// The two additional derived kinds (PoolReplacement, hero-bound ItemPool)
/// will route here once Chunk 5b lands them. Today they hit the `_ => {}`
/// arm — a stripped derived PoolReplacement is dropped without regeneration.
/// This is acceptable today because no regenerator exists yet; it is the
/// exact ticket for Chunk 5b.
///
/// Contract: callers must invoke `strip_derived_structurals` on `structural`
/// before calling this — `collect_stripped_kinds` returns a deduped `kinds`
/// list from the pre-strip structural, and strip guarantees no `derived:true`
/// items remain, so each regenerated kind is pushed exactly once.
pub fn regenerate_derived_kinds(
    structural: &mut Vec<StructuralModifier>,
    heroes: &[Hero],
    kinds: &[StructuralType],
) {
    if heroes.is_empty() || kinds.is_empty() {
        return;
    }
    for kind in kinds {
        match kind {
            StructuralType::Selector => {
                structural.push(crate::builder::derived::generate_char_selection(heroes));
            }
            StructuralType::HeroPoolBase => {
                structural.push(crate::builder::derived::generate_hero_pool_base(heroes));
            }
            // PoolReplacement + ItemPool regenerators deferred to Chunk 5b.
            _ => {}
        }
    }
}
