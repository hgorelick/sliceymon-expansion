pub mod hero_emitter;
pub mod structural_emitter;
pub mod replica_item_emitter;
pub mod monster_emitter;
pub mod boss_emitter;
pub mod fight_emitter;
pub mod chain_emitter;
pub mod reward_emitter;
pub mod phase_emitter;
pub mod derived;
pub mod options;

pub use options::{BuildOptions, SourceFilter, SourceSet};

use crate::error::CompilerError;
use crate::ir::{ModIR, StructuralType};
use crate::ir::merge::{collect_stripped_kinds, regenerate_derived_kinds, strip_derived_structurals};

/// Build a textmod string from a ModIR.
///
/// Assembly order matches sliceymon convention:
/// party, events, dialogs, selectors, heropool base, heroes, level-up,
/// item pools, replica items (trigger-based summons), monsters, bosses,
/// boss modifiers, gen select, difficulty, end screen, art credits.
/// (Post-8A: a single `replica_items` loop replaces the pre-rewrite
/// capture / legendary stages — `ReplicaItem` now models trigger-based
/// summons via `SummonTrigger::SideUse` / `Cast` per `ir/mod.rs`.)
///
/// Output format: one modifier per line, comma-terminated, with blank spacer lines.
///
/// Thin wrapper over [`build_with`]; equivalent to
/// `build_with(ir, &BuildOptions::default())`.
pub fn build(ir: &ModIR) -> Result<String, CompilerError> {
    build_with(ir, &BuildOptions::default())
}

/// Build a textmod string from a ModIR, honoring the supplied [`BuildOptions`].
///
/// Per the 2026-04-22 "BuildOptions + provenance-aware findings" ruling,
/// every content-emission site consults
/// `opts.include.admits(entity.source)` before emitting. Structural modifiers
/// carry their own `source`; heroes / replica items / monsters / bosses use
/// their top-level `source`.
///
/// Derived-structural policy (SPEC §4): `build` strips any
/// `StructuralModifier` with `derived: true` from the IR before emission and
/// appends regenerated forms. `Source::Custom`-origin derived structurals are
/// rejected with [`CompilerError::derived_structural_authored`];
/// `Source::Base` / `Source::Overlay` are stripped with an `X010` warning.
/// The strip-regenerate cycle operates on a local clone so the caller's IR
/// is not mutated; callers that need the warnings should use `merge` (which
/// writes to `base.warnings`).
pub fn build_with(ir: &ModIR, opts: &BuildOptions) -> Result<String, CompilerError> {
    let filter = &opts.include;

    // SPEC §4: derived structurals are stripped and regenerated. We only
    // clone (and run the strip+regenerate cycle) when the IR actually
    // carries a derived-flagged structural — otherwise this is a no-op cost
    // on every build() call against extracted Base-origin IR.
    //
    // The 2026-04-22 "provenance-aware findings" ruling / options.rs
    // contract: "regenerated from the post-filter
    // content set." We regenerate from heroes admitted by `filter` so the
    // char-selection / hero-pool reflects the same hero set the emission
    // loop will actually emit.
    let needs_strip = ir.structural.iter().any(|s| s.is_derived());
    let mut ir_buf;
    let ir: &ModIR = if needs_strip {
        ir_buf = ir.clone();
        let kinds = collect_stripped_kinds(&ir_buf.structural);
        strip_derived_structurals(
            &mut ir_buf.structural,
            &mut ir_buf.warnings,
            "build",
        )?;
        let filtered_heroes: Vec<_> = ir_buf
            .heroes
            .iter()
            .filter(|h| filter.admits(h.source))
            .cloned()
            .collect();
        regenerate_derived_kinds(
            &mut ir_buf.structural,
            &filtered_heroes,
            &ir_buf.replica_items,
            &kinds,
        );
        &ir_buf
    } else {
        ir
    };

    let mut modifiers: Vec<String> = Vec::new();

    // Derived structurals bypass the source filter: they're regenerated from
    // post-filter content and are tagged `Source::Base` by construction, so
    // a filter that excludes `Base` would silently drop them. The
    // 2026-04-22 "provenance-aware findings" ruling /
    // options.rs: "they do not carry their own Source filter." We use
    // `is_derived()` (flag + kind) rather than the raw `derived` flag so a
    // hand-authored structural with `derived:true` on a non-derived kind
    // still honors the filter.
    let emit_structurals =
        |modifiers: &mut Vec<String>, kind: StructuralType| {
            for s in ir
                .structural
                .iter()
                .filter(|s| s.modifier_type == kind && (s.is_derived() || filter.admits(s.source)))
            {
                modifiers.push(structural_emitter::emit(s, &ir.replica_items));
            }
        };

    // 1. Party config
    emit_structurals(&mut modifiers, StructuralType::PartyConfig);

    // 2. Event modifiers
    emit_structurals(&mut modifiers, StructuralType::EventModifier);

    // 3. Dialogs
    emit_structurals(&mut modifiers, StructuralType::Dialog);

    // 4. Selectors
    emit_structurals(&mut modifiers, StructuralType::Selector);

    // 5. HeroPool base (template list)
    emit_structurals(&mut modifiers, StructuralType::HeroPoolBase);

    // 6. Heroes
    for hero in ir.heroes.iter().filter(|h| filter.admits(h.source)) {
        modifiers.push(hero_emitter::emit(hero)?);
    }

    // 7. Level-up action
    emit_structurals(&mut modifiers, StructuralType::LevelUpAction);

    // 8. Item pools
    emit_structurals(&mut modifiers, StructuralType::ItemPool);

    // 9. Replica items (trigger-based summons — 8A stub produces zero here
    //    because itempool extraction demotes every entry to a sentinel
    //    NonSummon. The future real parser surfaces SideUse / Cast entries
    //    that this loop will emit alongside their itempool envelopes.)
    for item in ir.replica_items.iter().filter(|i| filter.admits(i.source)) {
        modifiers.push(replica_item_emitter::emit_replica_item(item));
    }

    // 11. Monsters
    for mon in ir.monsters.iter().filter(|m| filter.admits(m.source)) {
        modifiers.push(monster_emitter::emit_monster(mon)?);
    }

    // 12. Bosses
    for boss in ir.bosses.iter().filter(|b| filter.admits(b.source)) {
        modifiers.push(boss_emitter::emit_boss(boss)?);
    }

    // 12b. Pool replacements (structural)
    emit_structurals(&mut modifiers, StructuralType::PoolReplacement);

    // 13. Boss modifiers
    emit_structurals(&mut modifiers, StructuralType::BossModifier);

    // 14. Gen select
    emit_structurals(&mut modifiers, StructuralType::GenSelect);

    // 15. Difficulty
    emit_structurals(&mut modifiers, StructuralType::Difficulty);

    // 16. End screen
    emit_structurals(&mut modifiers, StructuralType::EndScreen);

    // 17. Art credits
    emit_structurals(&mut modifiers, StructuralType::ArtCredits);

    // 18. Phase modifiers
    emit_structurals(&mut modifiers, StructuralType::PhaseModifier);

    // 19. Choosables
    emit_structurals(&mut modifiers, StructuralType::Choosable);

    // 20. Value modifiers
    emit_structurals(&mut modifiers, StructuralType::ValueModifier);

    // 21. Hidden modifiers
    emit_structurals(&mut modifiers, StructuralType::HiddenModifier);

    // 22. Fight modifiers
    emit_structurals(&mut modifiers, StructuralType::FightModifier);

    // Join with comma + blank line (sliceymon format)
    let output = modifiers.join(",\n\n");

    // Trailing comma
    let output = if output.is_empty() {
        output
    } else {
        format!("{},", output)
    };

    Ok(output)
}

/// Build a complete textmod from an IR, auto-generating derived structurals
/// (character selection, hero pool base) if not explicitly present.
///
/// Use this for programmatic IR creation where the IR may not have
/// explicit structural modifiers. For round-trip builds from existing mods,
/// use `build()` instead.
///
/// Invariant: any structural this function appends is marked `derived: true`,
/// so subsequent `build` calls strip + regenerate rather than carrying
/// staleness through. See SPEC §4 derived-structural rule.
pub fn build_complete(ir: &ModIR) -> Result<String, CompilerError> {
    let mut ir = ir.clone();

    if !ir.heroes.is_empty() {
        if !ir.structural.iter().any(|s| s.modifier_type == StructuralType::Selector) {
            ir.structural.push(derived::generate_char_selection(&ir.heroes));
        }
        if !ir.structural.iter().any(|s| s.modifier_type == StructuralType::HeroPoolBase) {
            ir.structural.push(derived::generate_hero_pool_base(&ir.heroes));
        }
    }

    build(&ir)
}
