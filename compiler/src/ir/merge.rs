use crate::error::CompilerError;
use crate::finding::{Finding, Severity};
use crate::ir::{ModIR, Source, StructuralModifier, StructuralType};

/// X010 — derived structural (`CharacterSelection`, `HeroPoolBase`,
/// `PoolReplacement`, hero-bound `ItemPool`) present in IR. Authoring derived
/// structurals is unsupported; they are regenerated from content. For
/// `Source::Base` / `Source::Overlay` input we strip + warn. For
/// `Source::Custom` input it is a [`CompilerError::DerivedStructuralAuthored`]
/// (see SPEC §4 derived-structural rule).
pub const X010: &str = "X010";

const X010_SUGGESTION: &str = "Derived structurals are regenerated at build time; authoring them directly is unsupported.";

/// Strip derived structurals from `ir.structural` according to SPEC §4's
/// provenance-gated rule:
///
/// - `Source::Custom` → returns `Err(CompilerError::DerivedStructuralAuthored)`.
/// - `Source::Base` / `Source::Overlay` → strip and append an `X010` `Warning`
///   to `ir.warnings`.
///
/// `label` names the input side ("base" / "overlay") and is used in the
/// finding's `field_path` so downstream tools can attribute the strip.
///
/// The hero list comes from `heroes_ref` so hero-bound `ItemPool` modifiers
/// (whose derived-ness depends on whether the pool name matches a hero) can
/// be classified correctly. Pass the post-merge hero set when merging so both
/// sides see the same classification.
pub fn strip_derived_structurals(
    structural: &mut Vec<StructuralModifier>,
    warnings: &mut Vec<Finding>,
    label: &str,
) -> Result<(), CompilerError> {
    let mut i = 0;
    while i < structural.len() {
        if structural[i].is_derived() {
            let s = &structural[i];
            if s.source == Source::Custom {
                return Err(CompilerError::derived_structural_authored(format!(
                    "{:?}{}",
                    s.modifier_type,
                    s.name.as_deref().map(|n| format!(" ({})", n)).unwrap_or_default()
                ))
                .with_field_path(format!(
                    "{}.structural[{}]",
                    label, i
                ))
                .with_suggestion(X010_SUGGESTION));
            }
            let name = s.name.clone();
            let modifier_type = s.modifier_type.clone();
            structural.remove(i);
            warnings.push(Finding {
                rule_id: X010.to_string(),
                severity: Severity::Warning,
                message: format!(
                    "stripped derived structural {:?}{} (regenerated at build time)",
                    modifier_type,
                    name.as_deref().map(|n| format!(" \"{}\"", n)).unwrap_or_default()
                ),
                field_path: Some(format!("{}.structural[{}]", label, i)),
                suggestion: Some(X010_SUGGESTION.to_string()),
                modifier_name: name,
                ..Default::default()
            });
        } else {
            i += 1;
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

    // Strip overlay derived structurals into an intermediate vec so the strip
    // validates provenance and warnings land on `base.warnings` with an
    // "overlay"-labeled field_path.
    let mut overlay_structural = overlay.structural;
    // Overlay items picked up Source::Overlay when they were authored or
    // during an earlier merge; ensure it here so stripping classifies them
    // correctly.
    for s in overlay_structural.iter_mut() {
        // Preserve Source::Custom (author-added) — `strip_derived_structurals`
        // treats Custom as an error. Bump anything else to Overlay so the
        // X010 path fires on Base-origin overlays too.
        if s.source != Source::Custom {
            s.source = Source::Overlay;
        }
    }
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
    regenerate_derived_kinds(base, &stripped_kinds);

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

/// Regenerate derived structurals for the listed kinds from `ir` content.
///
/// Used by both `merge` (post-strip, pre-return) and `build_with`. Only
/// regenerates kinds that were present-and-stripped — this preserves
/// format-specific roundtrip (sliceymon's inline `!mheropool.` encoding has
/// no separate char-selection Selector, and nothing in this function adds
/// one unless the input already had one marked derived).
///
/// The two additional derived kinds (PoolReplacement, hero-bound ItemPool)
/// will route here once Chunk 5b lands them. Today they hit the `_ => {}`
/// arm — a stripped derived PoolReplacement is dropped without regeneration.
/// This is acceptable today because no regenerator exists yet; it is the
/// exact ticket for Chunk 5b.
pub fn regenerate_derived_kinds(ir: &mut ModIR, kinds: &[StructuralType]) {
    if ir.heroes.is_empty() || kinds.is_empty() {
        return;
    }
    for kind in kinds {
        let already_regenerated = ir
            .structural
            .iter()
            .any(|s| &s.modifier_type == kind && s.derived);
        if already_regenerated {
            continue;
        }
        match kind {
            StructuralType::Selector => {
                ir.structural
                    .push(crate::builder::derived::generate_char_selection(&ir.heroes));
            }
            StructuralType::HeroPoolBase => {
                ir.structural
                    .push(crate::builder::derived::generate_hero_pool_base(&ir.heroes));
            }
            // PoolReplacement + ItemPool regenerators deferred to Chunk 5b.
            _ => {}
        }
    }
}
