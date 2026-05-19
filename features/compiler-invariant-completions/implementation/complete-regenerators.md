# Chunk: `complete-regenerators` — `PoolReplacement` regenerator + `generate_char_selection` body fix + `DerivedKind` single-source-of-truth

**Slug:** `complete-regenerators`
**Feature:** `compiler-invariant-completions`
**PR:** —
**Depends on:** — (Foundation wave; no upstream chunk in this feature)
**Brief:** [`../brief.md`](../brief.md) · **Engineering plan:** [`../engineering-plan.md`](../engineering-plan.md) · **Decisions:** [`../decisions.md`](../decisions.md) · **Format guide:** [`../../../reference/textmod_guide.md`](../../../reference/textmod_guide.md) §SeqPhase · **SPEC anchor:** §4 (Derived-structural strip-and-regen) · §3.6 (Make invalid states unrepresentable)

> This plan is derived from the engineering plan, which is derived from the brief. If you can't restate this chunk's purpose in terms of a brief Goal or User-facing change, stop and re-read both before continuing.

## Goal

Close the derived-structural regenerator dispatch by landing the canonical-shape `PoolReplacement` regenerator alongside the canonical-form-corrected `generate_char_selection` body — both routed through a new `pub enum DerivedKind` that becomes the single source-of-truth for which `StructuralType` variants are derived, with `StructuralModifier::is_derived()` delegating to it — so the merge strip-regen cycle gains compile-time-enforced exhaustiveness over the four derived structural kinds, retiring the `_ => {}` deferred sink in `regenerate_derived_kinds`. The canonical-shape `generate_pool_replacement` emits `((heropool.<internal_name+...>))` from `&[Hero]` for the `derived: true` synthesized case; the four-mod byte-match against `working-mods/punpuns.txt`'s richer `((heropool.<list>)&Hidden).doc.<text>.mn.<name>` body — including the inline `(replica.<X>.abilitydata.(...)).n.<X>` rows — is delivered downstream by `poolreplacement-typed-payload` per decisions.md 2026-05-19, which widens `StructuralContent::PoolReplacement` to typed fields, widening this regenerator's signature accordingly.

## Brief link

- **Goal advanced (delivered by this chunk):** the complete-regenerators Goal — "Derived-structural regenerators are complete and corpus-matched."
- Today three regenerators ship; `PoolReplacement` is an explicit `_ => {}` wildcard arm in `regenerate_derived_kinds`.
- `generate_char_selection`'s current body (`1.ph.s@1<mn_name>@1<mn_name>…`) is malformed — each `@1<name>` is a free-standing SeqPhase button with no `@2` action.
- The canonical SeqPhase form per `reference/textmod_guide.md` §SeqPhase is `ph.sChoose a Party@1[<Hero1>][<Hero2>]…@2!mparty.<Hero1>+<Hero2>+…`.
- This chunk closes both regenerator residuals atomically per decisions.md 2026-05-12.
- The compile-time exhaustiveness enforcer lands per decisions.md 2026-05-13 (`DerivedKind` single source-of-truth + `is_derived()` delegation).
- The `Selector` verifier shape is bound by decisions.md 2026-05-18 (format guide is canonical authority; corpus does not evidence a Selector for round-trip).
- The `PoolReplacement` four-mod byte-match against `working-mods/punpuns.txt`'s richer corpus body (`((heropool.<list>)&Hidden).doc.<text>.mn.<name>` with inline `(replica.<X>.abilitydata.(...)).n.<X>` rows) is delivered downstream by `poolreplacement-typed-payload` per decisions.md 2026-05-19; this chunk delivers only the canonical-shape `generate_pool_replacement` (`((heropool.<list>))` from `&[Hero]`) plus the divergence proof for the `derived: true` synthesized case.
- **Non-goal honored — no re-prosecution of already-shipped platform invariants:** the chunk authors missing/malformed regenerators paralleling the three shipped ones.
- The strip-and-regen contract (SPEC §4) is unchanged.
- The brief's two enumerated complete-regenerators carve-outs (`PoolReplacement` regenerator + `generate_char_selection` body-shape correction) are exactly this chunk's scope.
- **Non-goal honored — no parser or emitter behavior changes beyond the bounded carve-outs:** the only emitter touches are the two regenerators (post-strip merge path), not extractor or top-level emitter changes.
- The `regenerate_derived_kinds` outer/inner refactor is internal to that function and changes no external behavior beyond closing the deferred wildcard arm.

## Context pack

**Read first:**
- `reference/textmod_guide.md` §SeqPhase — the canonical char-selection example is `ph.sChoose a Party@1[Scoundrel][Ruffian][Buckle][Splint][Cultist]@2!mparty.Scoundrel+Ruffian+Buckle+Splint+Cultist`. Format guide is authoritative per CLAUDE.md.
- `features/compiler-invariant-completions/decisions.md` 2026-05-18 — binds the format guide as Selector authority + acknowledges the corpus-degenerate Selector verifier shape.
- `features/compiler-invariant-completions/decisions.md` 2026-05-12 — binds atomic landing of both regenerator edits in one chunk.
- `features/compiler-invariant-completions/decisions.md` 2026-05-13 "`DerivedKind` is single source-of-truth" — binds the delegation rewrite + Path C over Path A and Path B.
- `features/compiler-invariant-completions/decisions.md` 2026-05-13 "classification disposition independent of merge order" — binds the `REACHABILITY_CARVE_OUT` extension as a cross-chunk contract with `audit-harness`.
- `features/compiler-invariant-completions/decisions.md` 2026-05-19 "`PoolReplacement` typed-payload sibling chunk; byte-match verifier delegated" — binds the typed-payload widening to `poolreplacement-typed-payload` and scopes THIS chunk's `generate_pool_replacement` to the canonical-shape regenerator (widened by the sibling).
- `features/compiler-invariant-completions/engineering-plan.md` — chunk-index row for `complete-regenerators`; Invariants entry "Derived-structural dispatch completeness"; Contract edges row `complete-regenerators ↔ audit-harness`; Manual gates "Four-mod round-trip baseline re-record".
- `compiler/src/ir/mod.rs` — `pub enum StructuralType` near `StructuralModifier`; `impl StructuralModifier` carrying `pub fn is_derived(&self) -> bool` with its 4-variant `matches!` body; `pub enum StructuralContent::PoolReplacement { body: String, hero_names: Vec<String> }`.
- `compiler/src/ir/merge.rs` — `pub fn regenerate_derived_kinds` (the dispatch function this chunk refactors); `pub fn collect_stripped_kinds` (the upstream filter that builds `kinds` via `is_derived()` — making outer `Err` structurally unreachable).
- `compiler/src/builder/derived.rs` — current `pub fn generate_char_selection`, `pub fn generate_hero_pool_base`, `pub fn generate_hero_item_pool` declarations + the inline `#[cfg(test)] mod tests` patterns this chunk's tests follow.
- `compiler/src/extractor/structural_parser.rs::parse_poolreplacement` — confirms the parser stashes `hero_names` off the source bytes; the regenerator's read-from-content contract.
- `working-mods/punpuns.txt` — the sole corpus mod carrying the `PoolReplacement` structural; its full body shape is `((heropool.<list>)&Hidden).doc.<text>.mn.<name>` with inline `(replica.<X>.abilitydata.(...)).n.<X>` rows interleaved in the `+`-joined list. This chunk's `generate_pool_replacement` emits the canonical-shape `((heropool.<list>))` subset (used for `derived: true` synthesized cases); the typed-payload sibling chunk (`poolreplacement-typed-payload` per decisions.md 2026-05-19) widens the regenerator to byte-match the full corpus body.

**Reference:**
- `features/compiler-invariant-completions/brief.md` — the complete-regenerators Goal headline + verifier prose; the no-platform-re-prosecution Non-goal's `PoolReplacement` + `generate_char_selection` carve-out enumeration.
- `CLAUDE.md` — "Retiring a public identifier" three-step; "No deferred correctness" rule; "When the parser, emitter, and the guide disagree, the guide wins" (format-guide authority binding).
- `SPEC.md` §4 (derived-structural strip-and-regen rule); §3.6 (make invalid states unrepresentable).
- `compiler/tests/audit_lib_panic_free.rs` — the existing audit's walk shape.
- `compiler/tests/roundtrip_baseline.rs` — the baseline-pinned regression suite; baseline files re-record per Manual gates row.
- `compiler/src/builder/mod.rs` — carries two `is_derived()` callsites (`needs_strip` + `emit_structurals` filter); Path C preserves return-value semantics so no edits.
- `compiler/src/builder/structural_emitter.rs` — the existing `StructuralContent::PoolReplacement { body, .. } => body.clone()` emission arm; this chunk does not touch it.
- `working-mods/{sliceymon,pansaer,punpuns,community}.txt` — corpus mods read for byte-shape verification at chunk-implementation time.
- Note: none of the four mods carry a derived char-selection Selector for round-trip per decisions.md 2026-05-18; pansaer's `1.(ph.s[…])` is a boss-fight modifier, not a Selector.
- `archive/pre-guide/` — predates `reference/textmod_guide.md`; non-authoritative.

**Conventions / patterns to follow:**

- `DerivedKind` is the single source-of-truth for which `StructuralType` variants are derived (decisions.md 2026-05-13).
- The enum body is exactly `{ Selector, HeroPoolBase, PoolReplacement, ItemPool }` — four unit variants.
- `is_derived()`'s body becomes `self.derived && DerivedKind::try_from(&self.modifier_type).is_ok()`.
- The existing 4-variant `matches!` literal in `is_derived()`'s body retires.
- Path A (drop `DerivedKind`; `_ => unreachable!()` inside inner match) is rejected per decisions.md 2026-05-13.
- Path B (panic on outer `Err` as load-bearing enforcement) is rejected per decisions.md 2026-05-13 — runtime-only detection.
- Path C (delegation) makes flipping a fifth derived `StructuralType` a one-step edit: extend `DerivedKind`.
- The compile-time forcing of the inner-exhaustive-match update on enum extension is the load-bearing property.
- `regenerate_derived_kinds` splits into outer (filter via `TryFrom`) + inner (exhaustive over `DerivedKind`).
- Outer iterates `kinds: &[StructuralType]`, calls `DerivedKind::try_from(kind)`, dispatches `Ok(dk)` to the inner function.
- Outer's `Err(_)` arm panics via `unreachable!(...)` with a message naming the broken upstream-filter discipline (`collect_stripped_kinds` builds `kinds` via `is_derived()`, and `is_derived()` returns true only when `DerivedKind::try_from` succeeds — reaching this arm means a caller bypassed `collect_stripped_kinds`-style filtering before invoking `regenerate_derived_kinds`).
- The upstream `collect_stripped_kinds` filters via `is_derived()`, which returns true only when `DerivedKind::try_from` succeeds — outer `Err` is structurally unreachable.
- decisions.md 2026-05-13's "continues to silently no-op on `Err`" prose describes today's `_ => {}` behavior surviving Path C.
- The engineering plan's `Derived-structural dispatch completeness` Invariant refines the outer-`Err` disposition to `unreachable!()` for tighter parity-break detection.
- Inner takes `DerivedKind` directly and is exhaustive over its four variants; the `_ => {}` arm retires.
- Rust's compiler-enforced exhaustiveness over `DerivedKind` is the load-bearing enforcer per decisions.md 2026-05-13.
- Panic-free audit precedent: `compiler/src/extractor/structural_parser.rs` already ships a production `unreachable!()` outside `#[cfg(test)]`.
- `compiler/tests/audit_lib_panic_free.rs` does not flag `unreachable!(` in its forbidden-primitives set — the planned panic site is precedent-aligned.
- `TryFrom<&StructuralType> for DerivedKind` succeeds for the four derived variants, errors for the other 14.
- The four success arms map by exact name (`Selector` → `Selector`, `HeroPoolBase` → `HeroPoolBase`, `PoolReplacement` → `PoolReplacement`, `ItemPool` → `ItemPool`).
- Error type is chunk-internal — implementer picks `()` or a private unit struct (e.g., `struct NotDerived;`); `.is_ok()`-invisible at every callsite. Do NOT reuse `CompilerError`, `ErrorKind`, or any other non-IR error type — `DerivedKind`'s conversion failure has no semantic relationship to compiler-error taxonomy and importing one would couple the trait signature to an unrelated module.
- Inverse `From<DerivedKind> for StructuralType` lands for symmetry — supports prospective callers that build `&[StructuralType]` arguments via `variant.into()`.
- The inverse `From` is NOT load-bearing for the outer-to-inner pass (inner takes `DerivedKind` directly).
- `generate_pool_replacement(heroes: &[Hero]) -> StructuralModifier` parallels `generate_hero_pool_base` in argument shape (canonical-shape; widened by `poolreplacement-typed-payload` per decisions.md 2026-05-19 to consume the typed `PoolReplacementEntry` payload).
- The body emits the canonical shape `((heropool.<hero1.internal_name>+<hero2.internal_name>+…))` — the typed-payload sibling chunk widens this output to byte-match `working-mods/punpuns.txt`'s richer `((heropool.<list>)&Hidden).doc.<text>.mn.<name>` body including inline `(replica.<X>.abilitydata.(...)).n.<X>` rows; this chunk delivers only the canonical shape.
- The regenerator emits from the hero list, not from a registry; the source-vs-IR divergence test proves it reads from content.
- The `derived: true` flag is set on output; `source: Source::Base` matches the three shipped regenerators.
- `StructuralModifier.name` is `None` on the canonical-shape output — the canonical `((heropool.<list>))` form carries no `.mn.<name>` suffix to plumb into the field, matching the `name: None` convention `generate_hero_pool_base` (and `generate_char_selection`) ship today. The sibling chunk `poolreplacement-typed-payload` widens this regenerator to read `.mn.<name>` from the typed payload and flips this to `Some(<extracted-name>)` per decisions.md 2026-05-19 — pinning `name: None` here defines the seam the sibling widens, so the typed-payload widening is the only place the field's source changes.
- `generate_char_selection`'s rewritten body emits the canonical SeqPhase form per `reference/textmod_guide.md` §SeqPhase.
- Target byte template: `ph.sChoose a Party@1[<Hero1.mn_name>][<Hero2.mn_name>]…[<HeroN.mn_name>]@2!mparty.<Hero1.mn_name>+<Hero2.mn_name>+…+<HeroN.mn_name>`.
- The three structural pieces are: the initial message ("Choose a Party"); the per-hero bracketed labels under one button; the `@2!mparty.<+-joined-list>` add action.
- The existing alphabetical-by-`hero.color` sort over `&[Hero]` is preserved across the body rewrite — the existing implementation builds a `sorted: Vec<&Hero>` via `sort_by_key(|h| h.color)` and iterates the sorted view to emit both `body` and `options`; the rewritten body's bracket list, `@2!mparty.<+-joined-list>` add-action list, and `content.options` all iterate the same color-sorted view so the three ordered sequences agree on hero order (the existing `generate_char_selection_alphabetical` test continues to pin color-sort on `options`).
- The implementer reads `reference/textmod_guide.md` §SeqPhase at chunk-implementation time to verify the exact byte template.
- The four reference mods do NOT carry a derived char-selection Selector for round-trip per decisions.md 2026-05-18.
- The Selector regenerator's correctness is carried by source-vs-IR divergence on synthesized heroes, not by four-mod byte-match.
- The existing malformed `1.ph.s@1<mn_name>@1<mn_name>…` body retires.
- The two assertions in `builder_auto_generates_derived_structurals` reading `output.contains("@1Alpha")` + `output.contains("@1Beta")` update to assert the new canonical-form shape.
- The `output.contains("heropool.")` assertion in the same test stays — it exercises `generate_hero_pool_base`, not `generate_char_selection`.
- Retirement-protocol guard test for the retired wildcard arm follows CLAUDE.md "Retiring a public identifier" + decisions.md 2026-05-12.
- The permanent guard test asserts the retired prose `"// PoolReplacement regenerator deferred."` returns zero hits under `compiler/src/**/*.rs` post-merge.
- The retirement comment dated to the chunk at the retired `_ => {}` site is the other retirement-protocol artifact.
- `REACHABILITY_CARVE_OUT` extension obligation is cross-chunk-contract-bound per decisions.md 2026-05-13.
- `DerivedKind` is Layer-2-unreachable from `ModIR` (helper-only, never on a `ModIR` field; only inside `regenerate_derived_kinds`).
- Whichever chunk merges second between `complete-regenerators` and `audit-harness` extends `REACHABILITY_CARVE_OUT` to include `"DerivedKind"`.
- The chunk-implementation step uses a symbol-anchored probe pinned to the path decisions.md 2026-05-13 binds: `rg -l 'REACHABILITY_CARVE_OUT' compiler/tests/audit_ir_shape_evidence.rs` (`REACHABILITY_CARVE_OUT` is unique enough as a symbol that any hit means the registry is declared in that file; file-pinned so a registry declared elsewhere doesn't false-positive; visibility-tolerant so a `static REACHABILITY_CARVE_OUT` declaration without `pub` does not false-negative — decisions.md 2026-05-13 binds the path and symbol but does not bind visibility). The probe runs at PR-open AND at PR-merge: if `audit-harness` has merged in the interim between PR-open and PR-merge, the implementer adds the `"DerivedKind"` line to audit-harness's `REACHABILITY_CARVE_OUT` in a follow-up amendment commit before merging this chunk's PR.

## Factoring Contract

**Owns (writes)** — exact paths this chunk creates or modifies. Each entry: `path` — what changes.

- `compiler/src/ir/mod.rs` — adds `pub enum DerivedKind { Selector, HeroPoolBase, PoolReplacement, ItemPool }` with `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` (no `Serialize`/`Deserialize`/`JsonSchema` — helper-only, never on `ModIR`); adds `impl TryFrom<&StructuralType> for DerivedKind` (four success arms + one fallback error); adds `impl From<DerivedKind> for StructuralType` inverse; rewrites `StructuralModifier::is_derived()` body to delegate to `DerivedKind::try_from`.
- `compiler/src/ir/merge.rs` — splits `regenerate_derived_kinds` into outer (filter via `DerivedKind::try_from`; `Err` arm panics via `unreachable!()` naming the broken invariant) + inner (exhaustive over `DerivedKind`'s four variants, no wildcard); retires the `// PoolReplacement regenerator deferred.` comment; updates the doc-comment paragraph naming `PoolReplacement` as deferred to instead name it as regenerated via `generate_pool_replacement`; adds an inline retirement comment dated to the chunk near the outer/inner split.
- `compiler/src/builder/derived.rs` — adds `pub fn generate_pool_replacement(heroes: &[Hero]) -> StructuralModifier` emitting the canonical shape `((heropool.<hero1.internal_name>+<hero2.internal_name>+…))` (widened by `poolreplacement-typed-payload` per decisions.md 2026-05-19); rewrites `generate_char_selection`'s body string assembly from the current `1.ph.s@1<mn_name>…` form to the canonical SeqPhase form per `reference/textmod_guide.md` §SeqPhase (`ph.sChoose a Party@1[<Hero1.mn_name>]…[<HeroN.mn_name>]@2!mparty.<Hero1.mn_name>+…+<HeroN.mn_name>`); the existing inline `#[cfg(test)] mod tests` updates the two assertions in `builder_auto_generates_derived_structurals` reading `output.contains("@1Alpha")` + `output.contains("@1Beta")` to assert the new canonical-form shape; the `output.contains("heropool.")` assertion stays.
- `compiler/tests/merge_tests.rs` — the `derived_char_selection` fixture's `body: "1.ph.s@1Alpha@1Beta".into()` updates to the new canonical-form body shape; the three `output.contains("@1Alpha")` / `output.contains("@1Beta")` / `output.contains("@1Gamma")` assertions in the `regenerated selector missing <name>` test update to assert the new bracketed-label form; the `non_derived_selector` fixture's arbitrary `1.ph.sAuthored selector <name>` body stays — that fixture exercises a non-derived Selector path this chunk does not touch.
- `compiler/tests/baselines/roundtrip/<mod>.baseline` (four files: `sliceymon.baseline`, `pansaer.baseline`, `punpuns.baseline`, `community.baseline`) — re-recorded per the Manual gates "Four-mod round-trip baseline re-record" row. **No baseline movement is expected from this chunk's edits**: the extractor's `make_structural` sets `derived: false` on every parsed structural (including `PoolReplacement`), so `is_derived()` returns `false`, `collect_stripped_kinds` does not collect them, the strip-regen cycle does not fire, and the existing `body.clone()` emit arm byte-preserves the source on extracted-mod round-trip; landing the canonical-shape `generate_pool_replacement` has no observable effect on extracted-mod round-trip. The four-mod byte-match baseline shifts (specifically `punpuns.baseline` flipping `roundtrip.equal` toward parity) are delivered downstream by `poolreplacement-typed-payload` per decisions.md 2026-05-19, which flips `derived: false → true` for `PoolReplacement` in the extractor and widens `generate_pool_replacement` to emit byte-equal to the source. Any unexpected baseline movement under this chunk is documented in the chunk's PR.

**Reads (no writes)** — files this chunk depends on but does not modify.

- `compiler/src/ir/mod.rs` outside `StructuralModifier`/`StructuralType`/`StructuralContent::PoolReplacement` — for context on the surrounding IR shape.
- `compiler/src/extractor/structural_parser.rs::parse_poolreplacement` — confirms the `hero_names: Vec<String>` field populated off source bytes.
- `compiler/src/builder/structural_emitter.rs` — confirms the existing `StructuralContent::PoolReplacement { body, .. } => body.clone()` arm.
- `compiler/src/builder/mod.rs` — carries two `is_derived()` callsites (`needs_strip` + `emit_structurals`); Path C preserves return-value semantics so no edits needed.
- `compiler/tests/roundtrip_baseline.rs` — the test reading `<mod>.baseline`; updating the four baseline files is sufficient.
- `working-mods/{sliceymon,pansaer,punpuns,community}.txt` — corpus mods read at chunk-implementation time; not modified.
- `reference/textmod_guide.md` §SeqPhase — read for the canonical `generate_char_selection` byte template at chunk-implementation time.

**Forbidden** — paths explicitly off-limits to this chunk.

- `compiler/src/extractor/` — no parser changes; the existing `parse_poolreplacement` is correct.
- `compiler/src/xref.rs`, `compiler/src/finding.rs`, `compiler/src/lib.rs` — out of scope; `DerivedKind` is not re-exported at the crate root.
- `compiler/src/authoring/`, `compiler/src/harvester/` — out of scope.
- `compiler/build.rs` — out of scope.
- Any new derived-structural variant beyond the four bound by decisions.md 2026-05-12 — reserved for a future chunk.
- `SPEC.md`, `personas/*.md` — this chunk does not delete a derived-structural kind, so the variant-deletion bundle's doc-surface grep does not apply.
- `compiler/tests/` audit-harness test file (path settled by `audit-harness`) — the `REACHABILITY_CARVE_OUT` extension this chunk's PR may carry is a single-line addition, not authorship.

**Single concern**

> Close the derived-structural regenerator dispatch by landing `generate_pool_replacement` + the canonical-form-corrected `generate_char_selection` body — both dispatched through a compile-time-exhaustive `DerivedKind` that `is_derived()` delegates to.

The halved-work test: halving the work in §Owns leaves either (a) `generate_pool_replacement` + the `regenerate_derived_kinds` refactor + `DerivedKind` + `is_derived()` delegation, or (b) the `generate_char_selection` body rewrite alone. The (a) half is incomplete without (b) because decisions.md 2026-05-12 explicitly bundles both regenerator edits in the same dispatch function with the same four-mod regenerator dogfood. The (b) half is incomplete without (a) for the same reason. Path A of decisions.md 2026-05-12 (split into two chunks) is rejected.

**No scaffolding**

The `DerivedKind` enum + `TryFrom` + `From` + `is_derived()` delegation + `regenerate_derived_kinds` outer/inner split form one structurally-atomic refactor.
- Inner exhaustive match takes `DerivedKind` (requires the enum exists).
- Outer arm calls `TryFrom` to filter (requires the impl exists).
- `is_derived()` delegates to the same `TryFrom` (requires the impl exists).
- Splitting any subset yields a non-compiling sub-PR, or a parallel encoding of "which `StructuralType` variants are derived" — exactly the CLAUDE.md "no parallel representations" violation Path C is sized to prevent.

- [ ] Confirmed: `compiler/src/ir/mod.rs` adds exactly the `DerivedKind` enum, its `TryFrom`, its inverse `From`, plus the `is_derived()` body delegation; no other types, impls, or public-API surface introduced.
- [ ] Confirmed: `compiler/src/ir/merge.rs`'s `regenerate_derived_kinds` outer/inner split lands as one edit; the inner function is exhaustive over `DerivedKind` with no wildcard arm; the outer function panics via `unreachable!()` on `Err`.
- [ ] Confirmed: `compiler/src/builder/derived.rs` adds exactly `generate_pool_replacement` + rewrites the body string assembly in `generate_char_selection` to the canonical SeqPhase form; no other functions added or deleted.
- [ ] Confirmed: the retirement-protocol guard test under `compiler/tests/` is one test asserting zero hits for `"// PoolReplacement regenerator deferred."` under `compiler/src/**/*.rs`.

**Abstraction earns its place**

`pub enum DerivedKind` is the chunk's load-bearing abstraction; the alternatives (per decisions.md 2026-05-13) were considered and rejected.
- Path A (drop `DerivedKind` entirely; `_ => unreachable!()` after the upstream filter) retracts compile-time enforcement of the derived-allowed-set.
- Path B (panic on outer `Err` as load-bearing mechanism) keeps `DerivedKind` without making it serve its purpose.
- Path C (delegation) makes the variant set encoded once in `DerivedKind`'s variant list, with `is_derived()` consulting it via `TryFrom`.

Consumer 1 (in-chunk): `regenerate_derived_kinds`'s outer/inner split. Consumer 2 (cross-chunk, per decisions.md 2026-05-13): `audit-harness`'s `REACHABILITY_CARVE_OUT` registry, which receives `"DerivedKind"` as a `compiler/src/ir/mod.rs`-declared pub enum that is Layer-2-unreachable from `ModIR`; the extension obligation transfers to whichever chunk merges second.

**Contracts changed** — new pub surface introduced.

- `pub enum DerivedKind { Selector, HeroPoolBase, PoolReplacement, ItemPool }` — new pub type in `compiler/src/ir/mod.rs`. Helper-only; never stored on `ModIR`; Layer-2-unreachable per decisions.md 2026-05-13. No `Serialize`/`Deserialize`/`JsonSchema` derives. The `Copy` derive is safe today because every variant is a unit variant; extending the enum with a data-bearing variant in the future requires dropping `Copy`.
- `impl TryFrom<&StructuralType> for DerivedKind` — new trait impl. Four success arms, one fallback error; chunk-internal error type.
- `impl From<DerivedKind> for StructuralType` — new trait impl. Symmetry-only; not load-bearing for `regenerate_derived_kinds`'s refactor.
- `StructuralModifier::is_derived()` — body rewrite. Signature unchanged; behavior unchanged on every `StructuralType` value. No external callsite changes — verified against `compiler/src/builder/mod.rs`'s `needs_strip` + `emit_structurals` callsites plus `compiler/src/ir/merge.rs`'s `collect_stripped_kinds` + `strip_derived_structurals` filters.
- `pub fn generate_pool_replacement(heroes: &[Hero]) -> StructuralModifier` — new pub fn in `compiler/src/builder/derived.rs`. Parallels `generate_hero_pool_base` in argument shape.
- `generate_char_selection` byte-output shape — signature unchanged; emitted body changes from `1.ph.s@1<mn_name>…` to the canonical SeqPhase form per `reference/textmod_guide.md` §SeqPhase. Downstream consumers (`compiler/tests/merge_tests.rs` fixture + assertions per §Owns) update in the same chunk per CLAUDE.md "no deferred correctness".
- `regenerate_derived_kinds` outer signature — unchanged. Inner function is module-private inside `compiler/src/ir/merge.rs`; the split is implementation-only.
- `REACHABILITY_CARVE_OUT` registry in `audit-harness`'s test file — extended with `"DerivedKind"` by whichever chunk merges second per decisions.md 2026-05-13. One-line conditional addition.

**Tests to add** — behavior + assertion shape per test bullet. Test harness layout is the implementer's call.

- **Source-vs-IR divergence test for `generate_pool_replacement`** under `compiler/src/builder/derived.rs`'s inline `#[cfg(test)] mod tests`.
- Construct `&[Hero]` with ≥2 entries having distinct `internal_name` values.
- Invoke `generate_pool_replacement(&heroes)`.
- Assert `modifier_type == StructuralType::PoolReplacement`, `derived == true`, `source == Source::Base`, `name == None` (canonical-shape carries no `.mn.<name>` suffix; the sibling chunk plumbs the typed-payload `name` per decisions.md 2026-05-19).
- Assert `StructuralContent::PoolReplacement { body, hero_names }` carries `body` matching `((heropool.<hero1.internal_name>+<hero2.internal_name>+…))`.
- Assert `hero_names` equals the constructed `internal_name` list.
- Source-vs-IR divergence: a second test constructs heroes with altered `internal_name` values; assert the regenerated `body` reflects the altered names.
- Pre-implementation gate: `rg "^pub fn generate_pool_replacement" compiler/src/builder/derived.rs` returns zero hits BEFORE the function lands.
- **`generate_char_selection` canonical-form byte-match test** under the same inline `#[cfg(test)] mod tests`.
- Construct `&[Hero]` with ≥3 entries in NON-color-sorted input order — e.g., `[make_hero("Gamma", 'c'), make_hero("Alpha", 'a'), make_hero("Beta", 'b')]` mirroring the shape `generate_char_selection_alphabetical` uses — so the test discriminates between an implementation that iterates the color-sorted view (correct) and one that iterates the raw `heroes: &[Hero]` view (a regression).
- Invoke `generate_char_selection(&heroes)`.
- Assert `modifier_type == StructuralType::Selector`, `derived == true`, `source == Source::Base`, `name == None` (unchanged from the pre-rewrite implementation).
- Assert `StructuralContent::Selector { body, options }` carries `body` byte-equal to the color-sorted canonical SeqPhase form: `ph.sChoose a Party@1[Alpha][Beta][Gamma]@2!mparty.Alpha+Beta+Gamma`. The literal byte string is what fails when an implementation iterates the unsorted `heroes` view to build either the bracket list or the `mparty.+` list — under the adversarial input above, an unsorted iteration would emit `@1[Gamma][Alpha][Beta]@2!mparty.Gamma+Alpha+Beta`, which is not byte-equal to the asserted form.
- The bracket-list cardinality matches the hero count; the `+`-joined party list cardinality matches the hero count; bracket-list order, `mparty.+`-list order, and `content.options` order all iterate the same color-sorted hero view (the order locked in `generate_char_selection_alphabetical`'s `options` assertion AND in this test's `body` byte-equal assertion).
- The existing `generate_char_selection_alphabetical` test continues to pass against the rewritten body — color-sort over `&[Hero]` is preserved, so `options[]` order is unchanged from today.
- The existing `generate_char_selection_from_heroes` test's `options[N] == "<name>"` assertions stay (they exercise the `options: Vec<String>` summary field, invariant under the body rewrite).
- Source-vs-IR divergence: a sibling test constructs `&[Hero]` with altered `mn_name` values; assert the regenerated `body` byte-shifts to reflect the altered names in both the `@1[<new_name>]` brackets AND the `@2!mparty.<new_name>+…` add-action list (proves the regenerator reads from content rather than hardcoding canonical bytes; mirrors the `generate_pool_replacement` divergence-test shape and satisfies brief Goal 5's "per-regenerator source-vs-IR divergence tests prove each regenerator reads from content").
- Pre-implementation gate: `rg -F '1.ph.s@1' compiler/src/builder/derived.rs` returns at least `1` BEFORE the chunk's edits; post-implementation, the same grep returns zero hits.
- **Exhaustiveness regression guard for `is_derived()` ↔ `DerivedKind` parity** under `compiler/src/ir/mod.rs`'s inline `#[cfg(test)] mod tests`.
- For every `StructuralType` variant, construct a `StructuralModifier` with `derived: true` + that variant.
- Assert `is_derived()` returns `true` iff `DerivedKind::try_from(&variant).is_ok()`.
- This guards against `is_derived()`'s delegation drifting away from `DerivedKind::try_from` across all 18 `StructuralType` variants.
- The existing `compiler/tests/merge_tests.rs::is_derived_truth_table` continues to coexist — it covers two slices the parity guard does not: (a) `derived: false` on each of the 4 derived kinds (proves the flag actually gates), and (b) `derived: true` on 5 non-derived kinds (a per-variant sample of the `Err` arm of `DerivedKind::try_from` under live state). The new parity guard fixes `derived: true` and proves kind-axis exhaustiveness over all 18 variants. The two prosecute complementary slices of the truth table; no retirement.
- **Per-arm correctness guard for `regenerate_derived_kinds`** under `compiler/src/ir/merge.rs`'s inline `#[cfg(test)] mod tests` (or `compiler/tests/`).
- For each `DerivedKind` variant, construct a minimum-cardinality `(heroes, replica_items)` input.
- Invoke `regenerate_derived_kinds(&mut structural, &heroes, &replica_items, &[variant.into()])`.
- Assert exactly one (or, for `ItemPool`, ≥0 per hero with matches) `StructuralModifier` appended whose `modifier_type` equals the input variant, `derived == true`, with non-empty content body.
- Per-arm fixture floor: ≥2 heroes for `Selector` and `PoolReplacement` arms; ≥1 hero for `HeroPoolBase`; ≥1 hero with ≥1 `SideUse` replica for `ItemPool`.
- The existing `regenerate_derived_kinds_rebuilds_hero_item_pool` test covers the `ItemPool` arm and stays.
- New per-arm tests cover `Selector`, `HeroPoolBase`, `PoolReplacement` so each arm has a per-variant correctness guard alongside the compile-time exhaustiveness check.
- **Retirement-protocol guard test** under `compiler/tests/` (new test file or extension of an existing `compiler/tests/audit_*` audit file; implementer picks).
- Read every file under `compiler/src/**/*.rs`, normalize by line-walking.
- Assert the literal string `"// PoolReplacement regenerator deferred."` returns zero hits across the entire library source tree.
- Mirrors the retirement-protocol pattern in `compiler/tests/audit_lib_panic_free.rs` + `compiler/tests/harvester_module_shape.rs`.
- The retired comment is unique enough that a literal-string grep is the right shape; no regex needed.
- Pre-implementation gate: `rg -F "// PoolReplacement regenerator deferred." compiler/src/` returns exactly `1` BEFORE the chunk's edits.

## Acceptance criteria

- [ ] `pub enum DerivedKind { Selector, HeroPoolBase, PoolReplacement, ItemPool }` is declared in `compiler/src/ir/mod.rs`; verify with `rg -nc "^pub enum DerivedKind " compiler/src/ir/mod.rs` returning exactly `1`.
- [ ] `impl TryFrom<&StructuralType> for DerivedKind` lands; verify with `rg -nc "^impl TryFrom<&StructuralType> for DerivedKind " compiler/src/ir/mod.rs` returning exactly `1`.
- [ ] `impl From<DerivedKind> for StructuralType` lands; verify with `rg -nc "^impl From<DerivedKind> for StructuralType " compiler/src/ir/mod.rs` returning exactly `1`.
- [ ] `StructuralModifier::is_derived()` body delegates to `DerivedKind::try_from`; verify with `rg -c "DerivedKind::try_from\(&self\.modifier_type\)\.is_ok\(\)" compiler/src/ir/mod.rs` returning at least `1`.
- [ ] The 4-variant `matches!` literal in `is_derived()`'s body retires; verify with `rg -c "StructuralType::Selector\s*\|" compiler/src/ir/mod.rs` returning zero in the `is_derived` function body.
- [ ] `pub fn generate_pool_replacement(heroes: &[Hero]) -> StructuralModifier` lands; verify with `rg -nc "^pub fn generate_pool_replacement\(heroes: &\[Hero\]\) -> StructuralModifier " compiler/src/builder/derived.rs` returning exactly `1`.
- [ ] The `// PoolReplacement regenerator deferred.` comment retires; verify with `rg -F "// PoolReplacement regenerator deferred." compiler/src/` returning zero hits.
- [ ] The outer arm of `regenerate_derived_kinds` is a two-branch match (no third wildcard arm); verify with `rg -A1 "^pub fn regenerate_derived_kinds" compiler/src/ir/merge.rs | rg "_ =>"` returning zero hits.
- [ ] `generate_char_selection`'s rewritten body matches the canonical SeqPhase form per `reference/textmod_guide.md` §SeqPhase; verify with `rg -F 'ph.sChoose a Party@1' compiler/src/builder/derived.rs` returning at least `1` (the new canonical body's prefix literal).
- [ ] The existing malformed `1.ph.s@1` prefix retires; verify with `rg -F '1.ph.s@1' compiler/src/builder/derived.rs` returning zero hits.
- [ ] The retirement comment dated to the chunk lands near the outer/inner split in `compiler/src/ir/merge.rs`; verify with `rg -c "\b2026-05-19|2026-05-2[0-9]|2026-05-3[01]|2026-06-" compiler/src/ir/merge.rs` returning at least `1`.
- [ ] `cd compiler && cargo test` passes — the new per-regenerator divergence tests, the canonical-form byte-match test, the parity guard, the per-arm correctness guards, the retirement-protocol guard test, the baseline-pinned regression suite, plus the panic-free audit.
- [ ] `cd compiler && cargo run --example roundtrip_diag` reports round-trip identity (extract → build → extract equals the initial extract) on all four reference mods.
- [ ] No four-mod baseline shift expected from this chunk: the extractor's `make_structural` sets `derived: false` so `is_derived()` returns `false` on extracted PoolReplacements, the strip-regen cycle is bypassed for them, and the existing `body.clone()` emit arm byte-preserves the source — landing the canonical-shape regenerator has no observable effect on extracted-mod round-trip. Any unexpected baseline shift documented in the chunk's PR.
- [ ] The four `compiler/tests/baselines/roundtrip/<mod>.baseline` files re-record via `UPDATE_BASELINES=1 cargo test --test roundtrip_baseline`; the updated files commit in the chunk's PR (re-records are expected to be no-ops; the test ensures the no-op claim survives).
- [ ] Per-regenerator canonical-shape check: `generate_pool_replacement(&heroes).content.body()` for a synthesized `&[Hero]` slice with ≥2 entries byte-equals the canonical shape `((heropool.<hero1.internal_name>+<hero2.internal_name>+…))`. Four-mod byte-match against `working-mods/punpuns.txt`'s richer corpus shape is delivered downstream by `poolreplacement-typed-payload` per decisions.md 2026-05-19 (the canonical-shape regenerator landed here is widened there to consume typed payload fields and emit byte-equal to the source-extracted instance).
- [ ] `generate_char_selection` canonical-form byte-match test asserts byte-equality against the format-guide form per decisions.md 2026-05-18; the sibling divergence test asserts the body reflects altered `mn_name` values per the brief Goal 5 verifier.
- [ ] `REACHABILITY_CARVE_OUT` consultation at PR-open time AND immediately pre-merge: `rg -l 'REACHABILITY_CARVE_OUT' compiler/tests/audit_ir_shape_evidence.rs` (symbol-anchored against the path decisions.md 2026-05-13 pins) determines whether `audit-harness` has merged.
- [ ] If `audit-harness` has merged before this chunk's merge, this chunk's PR adds `"DerivedKind"` to that file's `REACHABILITY_CARVE_OUT` registry in the same PR (if the probe flips between PR-open and PR-merge, a follow-up amendment commit lands the addition before merging this PR).
- [ ] If `audit-harness` has not merged, the obligation transfers to `audit-harness`'s first-run pass.

## Review checklist

- [ ] `DerivedKind`'s variant set is exactly `{ Selector, HeroPoolBase, PoolReplacement, ItemPool }` — the four derived `StructuralType` variants per decisions.md 2026-05-12; no fifth variant landed.
- [ ] `DerivedKind` carries `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` and NO `Serialize`/`Deserialize`/`JsonSchema` derives.
- [ ] `is_derived()`'s new body is exactly `self.derived && DerivedKind::try_from(&self.modifier_type).is_ok()` (or a syntactically-equivalent form); the 4-variant `matches!` literal retires.
- [ ] `regenerate_derived_kinds`'s outer arm is a two-branch match (`Ok(_) => inner(...)`, `Err(_) => unreachable!(...)`) — no third wildcard `_ =>` arm at either level.
- [ ] The `unreachable!()` message names the broken upstream-filter discipline (`collect_stripped_kinds` builds `kinds` via `is_derived()`, and `is_derived()` returns true only when `DerivedKind::try_from` succeeds), not a "parity invariant" between functions that share their source-of-truth by construction under Path C.
- [ ] `regenerate_derived_kinds`'s inner function is exhaustive over `DerivedKind`'s four variants with NO wildcard arm.
- [ ] `generate_pool_replacement` reads from the `&[Hero]` argument's `internal_name` field, NOT from a registry.
- [ ] `generate_char_selection`'s body matches the canonical SeqPhase form per `reference/textmod_guide.md` §SeqPhase.
- [ ] The malformed `1.ph.s@1<mn_name>` free-standing-buttons body retires.
- [ ] The `// PoolReplacement regenerator deferred.` comment + the `_ => {}` wildcard arm BOTH retire.
- [ ] The retirement-protocol guard test under `compiler/tests/` asserts zero hits for the comment-string across `compiler/src/**/*.rs`.
- [ ] The retirement comment dated to the chunk's merge date lands in `compiler/src/ir/merge.rs` near the outer/inner split.
- [ ] Four-mod round-trip baseline files re-record in the chunk's PR.
- [ ] `REACHABILITY_CARVE_OUT` extension obligation honored: if `audit-harness` has merged before this chunk, this chunk's PR adds `"DerivedKind"` to the registry.
- [ ] No edits to `compiler/src/extractor/`, `compiler/src/builder/structural_emitter.rs`, `compiler/src/xref.rs`, `compiler/src/finding.rs`, `compiler/src/authoring/`, `compiler/src/harvester/`, `compiler/build.rs`, `compiler/src/lib.rs`, `SPEC.md`, or `personas/*.md`.
- [ ] SPEC §4 honored — strip-and-regen contract preserved.
- [ ] SPEC §3.6 honored — `DerivedKind` encodes "which `StructuralType` variants are derived" once.
- [ ] CLAUDE.md "When the parser, emitter, and the guide disagree, the guide wins" honored — `generate_char_selection` emits the form documented in `reference/textmod_guide.md` §SeqPhase per decisions.md 2026-05-18.

## Out of scope

| Item | Where instead |
|---|---|
| Adding a fifth derived `StructuralType` variant beyond the four bound | A future chunk under a separate `decisions.md` entry |
| Renaming, reshaping, or relocating `StructuralType` or its variants | Out of scope; `StructuralType` declaration is unchanged |
| `StructuralContent::PoolReplacement` typed-payload widening (`hidden`, `doc`, `entries: Vec<PoolReplacementEntry>` fields) + `parse_poolreplacement` widening to populate the typed fields + emitter arm reconstruction from typed fields + extractor `make_structural` `derived: false → true` flip for `PoolReplacement` + `generate_pool_replacement` signature widening to consume typed fields | `poolreplacement-typed-payload` per decisions.md 2026-05-19 (delivers brief Goal 5's PoolReplacement four-mod byte-match verifier; this chunk only delivers the canonical-shape regenerator for the `derived: true` synthesized case) |
| Four-mod byte-match of `generate_pool_replacement` output against `working-mods/punpuns.txt`'s `((heropool.<list>)&Hidden).doc.<text>.mn.<name>` body including inline `(replica.<X>.abilitydata.(...)).n.<X>` rows | `poolreplacement-typed-payload` per decisions.md 2026-05-19 |
| Variant-deletion bundle (regex grep + per-variant `decisions.md` entry + retirement-protocol bundle) | `audit-harness` per decisions.md 2026-05-08; this chunk does not delete a variant |
| `REACHABILITY_CARVE_OUT` registry contents beyond the `"DerivedKind"` extension conditionally carried | `audit-harness` (the registry's declaration site) |
| Vehicle marker application on any pub enum | `audit-marker-retrofit` / each chunk introducing a new ModIR-reachable internal-state pub enum; `DerivedKind` is Layer-2-unreachable and receives no marker |
| Audit CI test (Layer-1 `syn` walk + Layer-2 `schemars` walk + closed escape-hatch registry) | `audit-harness` |
| `ItempoolItem::NonSummon` retype from `content: String` to `NonSummon(NonSummonEntry)` | `nonsummon-typed-ir` |
| `extract_from_itempool` real summon classifier, `ReplicaItem.enemy_template` nested-egg widening, `Finding.buckets` + `Finding.includes_boss` | `typed-summon-extractor` |
| V020 narrowing predicate, per-target X003 routing, `humanize_bucket`, `"legendary"` → `"replica_item"` owner-map rename | `xref-narrowing-and-rename` |
| `target_name` → `summon_name` rename on `ReplicaItem` | `summon-name-rename` |
| `::new(identity)` constructors on `HeroBlock`, `ReplicaItem`, `AbilityData`, `TriggerHpDef` | `authorable-new-constructors` |
| Harvester pure-core extraction + byte-determinism CI test | `harvester-pure-core` + `harvester-determinism-audit` |
| Author-content cross-IR rule changes beyond the dedupe pair | The no-author-content-rule-expansion Non-goal |
| Game-balance, roster, or corpus changes | The no-game-balance Non-goal; this chunk touches no `working-mods/*.txt` content |
| Drift-class parser/emitter fixes (open `PIPELINE_FIDELITY` classes; `HiddenModifierType::Skip` placeholder) | The no-drift-class-fixes Non-goal; owned by future `features/pipeline-fidelity/` feature |
