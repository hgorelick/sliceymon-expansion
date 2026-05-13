# Platform Foundations — Engineering Plan

**Brief:** [`./brief.md`](./brief.md)
**Status:** Active
**Created:** 2026-05-07
**Last updated:** 2026-05-07

## Brief mapping

### Goals

| Goal | Chunks |
|---|---|
| Typed authoring path refuses hallucinated values | `typed-face-identifiers`, `typed-sprite-identifiers` |
| IR is self-contained | `typed-sprite-identifiers` |
| Errors point at the field that failed and suggest a fix | `structured-error` |
| Library never crashes the host process | `panic-free-library` |
| Mod composition is deterministic and lossless | `merge-derived-contract` |
| Build emission can filter by source | `build-source-filter` |
| Each cross-IR defect surfaces under exactly one rule | `cross-kind-name-uniqueness` |
| IR variants reflect the corpus, not hypotheses | `corpus-evidenced-variants` |

### User-facing changes

| Change | Verified by |
|---|---|
| Typed authoring surface refuses unknown face / sprite identifiers at compile time | `typed-face-identifiers`, `typed-sprite-identifiers` |
| Extracted mod with unknown face identifier produces extraction warning and round-trips | `typed-face-identifiers` |
| `build` requires no external sprite map | `typed-sprite-identifiers` |
| Errors carry field path + suggestion; `Display` includes both | `structured-error` |
| Library audit-test refuses any new `panic!`/`unwrap`/`expect` outside test gates | `panic-free-library` |
| `merge(&mut base, overlay)` mutates base in place; warnings surface on `ModIR.warnings` | `merge-derived-contract` |
| `BuildOptions::include` filters emitted modifiers by `Source` | `build-source-filter` |
| `xref::Finding` carries the offending entity's `Source`; severity-promotes by source | `finding-provenance` |
| Cross-kind name collisions across `{hero, replica_item, monster}` surface under exactly one rule | `cross-kind-name-uniqueness` |
| Every shipping IR variant has a corpus instance; an audit-test enforces it going forward | `corpus-evidenced-variants` |

### Supporting infrastructure

- **`ir-defaults-and-constructors`** — programmatic IR construction primitives (`Default` where safe, `::new(identity)` otherwise) that the authoring layer (downstream feature) builds on. Not a brief Goal directly; the typed authoring surface needs a concrete way to construct authorable IR types without hand-spelling every field.
- **`finding-provenance`** — `xref::Finding.source: Option<Source>` + a single `promote_severity(base, source)` helper. Pairs with `build-source-filter` (both make `Source` first-class) but is its own concern: an authored modifier that would emit at `Severity::Error` must emit at `Severity::Warning` when its source is `Base`, regardless of whether the build filter is engaged.

### Non-goals enforcement

| Non-goal | Enforcement |
|---|---|
| No drift-class parser/emitter fixes | No chunk modifies parser or emitter behavior beyond what type-shape changes force; round-trip drift on corpus mods stays the parser-fidelity feature's territory |
| No author-ergonomics layer (chainable builders, macros) | `compiler/src/authoring/` ships only the typed identifier primitives (`FaceId`, `Pips`, `FaceIdValue`, `SpriteId`); no builder, no macro, no `HeroReplica` |
| No new IR sums for unobserved corpus classes | `corpus-evidenced-variants` audits and deletes; never adds |
| No CLI changes | No chunk modifies `compiler/src/main.rs` subcommand surface |
| No game-balance or roster changes | No chunk edits `working-mods/*.txt` content |

## Architecture summary

The compiler exposes a typed library surface that authors a textmod, extracts one back to IR, and composes two IR values into one. Today every public path takes a separate sprite-payload map alongside the IR, every error is a flat-message string, and the library can panic on bad input. The work here reshapes the library along three axes:

1. **Make every identifier the corpus uses (face IDs, sprite names) a typed value.** Two distinct constructors per identifier class — a strict registry-gated form for the authoring path (`SpriteId::lookup`, `FaceId::try_new`), and a permissive owned form for the extract path (`SpriteId::owned`, `FaceIdValue::Unknown(raw)`). The authoring path is the "no hallucination" path; the extract path round-trips any valid textmod the game accepts. Path A (extract) and Path B (authoring) stay split at every call site — collapsing them is a SPEC §3.3 / §6.1 amendment, not an implementation detail.
2. **Make the library never crash and always be informative.** `CompilerError` carries `field_path` + `suggestion` on every construction site; one `ErrorKind` enum carries the per-class data (`HeroParse { modifier_index, … }`, `FaceIdInvalid { raw, template }`, `DerivedStructuralAuthored { modifier_type }`, …). An in-tree audit walks `compiler/src/**/*.rs`, strips `#[cfg(test)]` / `#[test]` items via brace-counting, and fails the test suite if any `panic!`/`unwrap`/`expect`/`unimplemented`/`todo` token survives in library code.
3. **Make `Source` first-class in build, merge, and xref.** A `BuildOptions { include: SourceFilter }` filter elides modifiers by source before emission. `merge(&mut base, overlay)` mutates the base in place and writes findings onto a `ModIR.warnings` sidecar instead of returning a tuple. `xref::Finding` carries the offending entity's `Source`; `promote_severity(base, source)` renders `Source::Base` violations at `Severity::Warning` and `Custom`/`Overlay` at `Severity::Error` everywhere a finding is constructed. Derived structurals (character selection, hero-pool base, pool replacement, hero-bound itempool) are stripped by `merge` and regenerated by `build` from content; an authored modifier of a derived kind is `CompilerError::DerivedStructuralAuthored`.

Cross-chunk contracts the engineering plan introduces — `CompilerError`, `ErrorKind`, `FaceId`, `FaceIdValue`, `Pips`, `SpriteId`, `BuildOptions`, `SourceFilter`, `SourceSet`, `build_with`, `ModIR.warnings`, `Finding.source`, `promote_severity`, `compiler/src/authoring/` — are the durable surface every other compiler-consuming feature builds against.

## Decisions closure

| Decision | Resolution | Citation |
|---|---|---|
| Permissive vs strict for unknown face / sprite identifiers | bound — extract permissive (`FaceIdValue::Unknown(raw)`, `SpriteId::owned`) emits `Severity::Warning` finding and round-trips byte-for-byte; authoring strict (`FaceId::try_new`, `SpriteId::try_registered`) returns `CompilerError` | SPEC §1 (general-purpose backend) + §3.3 (self-contained IR) demand permissive extract; SPEC §6.1 demands strict authoring |
| `SPEC.md` §3.6 wording for the typed-identifier discipline | bound — amend in the same commit as `typed-face-identifiers` to name the format-invariant vs corpus-whitelist distinction; `Pips` annotation reads `i16` | SPEC §3.6; `typed-face-identifiers` |
| `Pips` numeric type | bound — `Pips(i16)`; full `i16` range; corpus contains negative pips so no narrower bound | `compiler/src/ir/mod.rs` `DiceFace::Active.pips` doc-comment + `reference/textmod_guide.md` |
| Sprite-registry priority on collision | bound — first-write-wins across `[sliceymon, pansaer, punpuns, community]`; emitted from a deterministic generator (`BTreeMap` collection, `phf_codegen` emission) so two `cargo build`s yield byte-identical output | `typed-sprite-identifiers`; `compiler/build.rs` `WORKING_MOD_ORDER` |
| `merge` signature | bound — `pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError>` per SPEC §5; warnings surface via `ModIR.warnings: Vec<Finding>` sidecar (no tuple return) | SPEC §5 |
| Severity promotion policy | bound — `promote_severity(base, source)`: `Some(Base) → Warning`; `Some(Custom \| Overlay) → Error`; `None → base`; applied at every `Finding` construction site | `finding-provenance` |
| Path A (extract) vs Path B (authoring) at sprite-bearing callsites | bound — extractor uses `SpriteId::owned(name, img_data)` unconditionally (source bytes preserved verbatim); registry lookups (`SpriteId::lookup` / `try_registered`) live only on the authoring path | SPEC §3.3 / §6.1; `typed-sprite-identifiers` |
| `FightUnit` sprite name source | bound — under permissive path, `SpriteId::owned(name, img_data)` uses the required `FightUnit.name` field; generic names (`"Boss"`) flow through as novel owned sprites | `compiler/src/ir/mod.rs` `FightUnit.name`; `typed-sprite-identifiers` |
| Audit enforcement mechanism for panic-free library | bound — integration test `compiler/tests/audit_lib_panic_free.rs` runs under `cargo test`; no `xtask` / `build.rs` gate, no separate CI pipeline | `panic-free-library` |
| Selector / HeroPoolBase / PoolReplacement / hero-bound ItemPool — derived-flag invariant during transition | bound — no chunk merged before `merge-derived-contract` may flip `derived: true` on these four kinds; the chunk lands the first regenerator that exercises the strip-and-regen path | `merge-derived-contract` |
| Hero-bound ItemPool bucketing key | bound — bucket by parsing `.n.NAME` out of each `ItemPool` structural modifier and matching `NAME` to a hero's `internal_name`; `ReplicaItem` (Legendary-shaped) skipped (it has its own emission path) | `merge-derived-contract`; corpus shape per `working-mods/sliceymon.txt` |
| Cross-kind name-uniqueness rule ownership | bound — introduce X003 (SPEC §6.3) covering `{hero, replica_item, monster}`; narrow V020 to skip emission when the colliding bucket set is a subset of those three with cardinality ≥2; V020 retains all boss-involving and intra-bucket cases | SPEC §6.3 + §3.7; `cross-kind-name-uniqueness` |
| Are corpus-unevidenced IR variants kept? | bound — no; any IR sum-type variant that *claims a corpus shape* without at least one working-mods instance is deleted along with the rules and tests authored against it; an audit-test enforces this for every shipping shape variant going forward | brief Goal "IR variants reflect the corpus"; `corpus-evidenced-variants` |
| Escape-hatch variants under the corpus-evidence rule | bound — escape-hatch variants (e.g. `FaceIdValue::Unknown(raw)`, `SpriteId` constructed via `owned` against a name absent from the registry) are not corpus-shape claims; they exist explicitly to model the absence of corpus knowledge so the permissive extract path round-trips any valid textmod. The audit excludes them. The discipline applies to variants whose semantics are "this is a distinct shape the parser produces from corpus bytes" — not to fallbacks that handle the absence of corpus matches | SPEC §3.3 (self-contained IR for any valid textmod); R1 permissive-whitelist ruling; `corpus-evidenced-variants` |
| `ReplicaItemKindMismatch` runtime error variant | bound — not introduced; `ReplicaItem` models the single corpus-evidenced shape, the invariant class is vacuous | `corpus-evidenced-variants`; `structured-error` |

No `open` or `deferred-to-<chunk>` rows: every cross-chunk wiring decision is bound at engineering-plan time.

## Invariants

### Path A / Path B split
The extract path consumes any valid textmod the game accepts; the authoring path refuses unknown identifiers at compile time. Every sprite-bearing extractor calls `SpriteId::owned(name, img_data)`; registry lookups live on the authoring path only. Collapsing the two paths is a SPEC §3.3 / §6.1 amendment, not an implementation detail. Enforced by the source-vs-IR divergence test in `typed-sprite-identifiers` (a hero whose name is in the registry but whose source `.img.` payload differs from the registry's must extract with the source's bytes, not the registry's).

### Self-contained IR
A `ModIR` value alone is sufficient input to `build`. No public build entry point takes ambient inputs (`HashMap`, file path, network handle). Sprite payload travels inside the typed IR (`SpriteId.img_data`); a saved IR file alone reconstructs the textmod. Enforced by `typed-sprite-identifiers`.

### Field-path-aware errors
Every `CompilerError` construction site populates `field_path` and `suggestion`; `Display` renders both. Enforced by the `structured-error` chunk's `error::test_display_includes_field_path` and `test_display_includes_suggestion`, plus migration of every existing construction site in the same chunk.

### Crash-free library
No `panic!`/`unwrap`/`expect`/`unimplemented!`/`todo!` token survives in `compiler/src/**/*.rs` outside `#[cfg(test)]` or `#[test]` gates. Enforced by the `panic-free-library` chunk's audit-test under `cargo test`.

### Composition is content-preserving
`merge` carries author-written content unmodified between IRs. Derived structurals (`Selector`, `HeroPoolBase`, `PoolReplacement`, hero-bound `ItemPool`) are stripped by `merge` and regenerated by `build`; an attempt to author a derived-kind modifier is `CompilerError::DerivedStructuralAuthored`. Regeneration is scoped to kinds present-and-stripped — `build` does not invent a `Selector` for a mod whose source had none. Enforced by `merge-derived-contract`.

### One finding per defect
No two cross-IR rules fire for the same reason on the same input. Where two rules previously overlapped on cross-kind name collisions across `{hero, replica_item, monster}`, the narrower rule (X003, SPEC §6.3) owns that slice; the broader rule (V020, cross-category name uniqueness) retains only what X003 cannot cover (any boss-involving collision; any intra-bucket duplicate). Enforced by `cross-kind-name-uniqueness`.

### Corpus-evidenced variants
Every IR shape claim the library ships has at least one working-mods instance. The discipline applies along four axes:

1. **Sum-type variants** — each variant of every IR `enum` has at least one corpus instance; unevidenced variants are deleted along with the rules and tests authored against them.
2. **Optional fields whose `None` case has no corpus instance** — collapse to a required field; an `Option<T>` with no observed `None` is a hypothesis that the field is optional.
3. **String-token unions with closed observed sets** — a `String` field whose observed values form a closed finite set collapses to a typed enum; a single-variant enum collapses to a direct field or `bool`.
4. **Multi-shape collisions under one variant** — when two distinct corpus shapes route through the same IR variant, the variant splits pre-ship into the typed sub-variants; collapsing distinct corpus shapes into one discriminator forces the emitter to reconstruct the missing distinction from a side channel.

Reintroducing a previously-deleted shape is a same-PR landing event: the corpus anchor (`<mod>:<verbatim line>`), the typed variant, and a per-variant test ship together.

The discipline does not apply to escape-hatch variants that exist explicitly to model the absence of corpus knowledge (`FaceIdValue::Unknown(raw)`, `SpriteId::owned` against an unregistered name) — those are not corpus-shape claims, they are deliberate fallbacks for the permissive extract path. Nor does it require type-system encoding for invariants that are already enforced at runtime soundly; the discipline is "delete unevidenced shape claims," not "rewrite every runtime gate as a type." Enforced by `corpus-evidenced-variants`.

## Source precedence and finding lanes

`Source::{Base, Custom, Overlay}` is the IR's provenance enum. Every authorable IR entity carries one. Three downstream uses:

| Use | Behavior |
|---|---|
| Build emission filter (`BuildOptions::include`) | `SourceFilter::All` emits everything (default); `Only(set)` emits only entities whose `source ∈ set`; `Exclude(set)` emits entities whose `source ∉ set`. Derived structurals are regenerated from the post-filter content set; they do not carry their own filter. |
| Finding severity promotion (`promote_severity`) | `source = Some(Source::Base)` → `Severity::Warning`; `source = Some(Custom \| Overlay)` → `Severity::Error`; `source = None` → keep the rule's base severity. The intent: the corpus mods are the trusted starting point; an author's own modifications get a stricter lane. |
| Merge strip path (`merge-derived-contract`) | Author-written derived structural (`Source::Custom`) → `CompilerError::DerivedStructuralAuthored` (refuse to merge). `Source::Base` / `Overlay` derived structural → strip with an `X010` warning on `base.warnings`. |

This is the source-of-truth for any chunk that consults `Source`.

## Build determinism

`compiler/build.rs` runs the extractor over `working-mods/*.txt` to produce two generated artifacts: `face_id_generated.rs` (one `pub const NAME: FaceId = FaceId(N);` per harvested face ID) and `sprite_registry_generated.rs` (a `phf::Map<&'static str, SpriteId>` literal). Both must be byte-identical across rebuilds — `BTreeMap` collection, `phf_codegen` emission, no wall-clock or environment input. `cargo build` twice yields identical bytes; an MD5-equality test under `cargo test` would catch a regression. `phf` (runtime) and `phf_codegen` (build-dep) are added to `compiler/Cargo.toml` in `typed-sprite-identifiers`.

## Manual gates

The compiler has no remote infrastructure; every gate is a `cargo test` invocation or a corpus inspection command. Operator-facing budgets are accordingly thin:

- **Round-trip roundtrip on each corpus mod.** `cargo run --example roundtrip_diag` after every chunk merge; `extract(build(extract(mod))) == extract(mod)` is the correctness bar across all four `working-mods/*.txt`. Chunks that change extractor or emitter outputs (sprite consolidation, derived strip-and-regen) re-record baselines under `compiler/tests/baselines/roundtrip/<mod>.baseline`.
- **Determinism check on generated files.** After `typed-face-identifiers` and `typed-sprite-identifiers` land, two consecutive `cargo build`s must produce byte-identical `face_id_generated.rs` and `sprite_registry_generated.rs`. Run on first merge of each generator chunk and on any change to `compiler/build.rs`.
- **Audit-test enforcement.** After `panic-free-library` lands, `cargo test` includes the `audit_no_lib_panic_or_unwrap` integration test; any subsequent chunk that introduces a forbidden token in library code fails the gate. After `corpus-evidenced-variants` lands, the same gate covers IR-variant evidence.

## Chunk index

| Slug | Description | Depends on |
|---|---|---|
| `structured-error` | Reshape `CompilerError` into a struct carrying `kind: Box<ErrorKind>`, `field_path`, `suggestion`, `context`; migrate every construction site in `extractor/`, `builder/`, `ir/ops.rs`, `xref.rs`; constructor helpers + `Display` tail render both fields | — |
| `typed-face-identifiers` | Typed `FaceId(u16)` + `Pips(i16)` newtypes; permissive `FaceIdValue::{Known, Unknown}`; deterministic `compiler/build.rs` face-id harvest into `face_id_generated.rs`; flip `DiceFace::Active { face_id: FaceIdValue, pips: Pips }`; new xref rules X016 (template-restricted face) and X017 (unknown face warning); SPEC §3.6 amendment in same commit | `structured-error` |
| `typed-sprite-identifiers` | `SpriteId { name, img_data }` newtype with `lookup` (registry-gated) + `owned` (source-preserving) + `try_registered` (strict) constructors; `phf_codegen`-backed `sprite_registry_generated.rs`; consolidate every IR sprite-bearing type to a single `sprite: SpriteId` field; drop `sprites: &HashMap<String, String>` from every public build entry point | `structured-error` |
| `ir-defaults-and-constructors` | `#[derive(Default)]` on IR types with safe defaults; `::new(identity)` on types where identity fields preclude `Default`; sprite-bearing constructors take their final `SpriteId` shape | `typed-sprite-identifiers` |
| `merge-derived-contract` | `merge(&mut ModIR, ModIR) -> Result<(), CompilerError>` in-place signature; `ModIR.warnings: Vec<Finding>` sidecar; `StructuralModifier::is_derived()` gate; merge strips derived structurals + emits X010 warnings; `build` regenerates kinds present-and-stripped; `generate_pool_replacement` and `generate_hero_item_pool` regenerators (byte-match the working-mods corpus); `generate_char_selection` body-shape fix against `reference/textmod_guide.md` + the working-mods corpus; `ErrorKind::DerivedStructuralAuthored` for `Source::Custom` derived modifiers | `structured-error` |
| `build-source-filter` | `BuildOptions { include: SourceFilter }`; `SourceFilter::{All, Only(SourceSet), Exclude(SourceSet)}`; bitmask `SourceSet` + `FromIterator<Source>`; `pub fn build_with(ir, opts) -> Result<String, CompilerError>`; `build(ir)` becomes thin wrapper over `build_with(ir, &Default::default())`; every content-emission site consults `opts.include.admits(entity.source)` | `structured-error` |
| `finding-provenance` | `xref::Finding.source: Option<Source>` (`#[serde(default, skip_if_none)]`); `promote_severity(base, source)` helper; every existing `Finding` construction site populates source + applies the helper; `iter_dice_faces` extended to yield owning entity's `Source` | — |
| `cross-kind-name-uniqueness` | New xref rule X003 covering SPEC §6.3 name uniqueness across the `{hero, replica_item, monster}` kinds; narrow V020's `check_cross_category_names` emission to skip cases where the colliding bucket set is a subset of those three with cardinality ≥2 (X003's territory); V020 retains all boss-involving and intra-bucket cases | `finding-provenance` |
| `panic-free-library` | Replace every `panic!`/`unwrap`/`expect`/`unimplemented!`/`todo!` in `compiler/src/**/*.rs` outside `#[cfg(test)]`/`#[test]` gates with `Result`-propagating equivalents; integration test `compiler/tests/audit_lib_panic_free.rs::audit_no_lib_panic_or_unwrap` walks the source tree, strips test-gated items via brace-counting, asserts zero forbidden tokens; runs under `cargo test` | `structured-error`, `typed-face-identifiers`, `typed-sprite-identifiers`, `merge-derived-contract` |
| `corpus-evidenced-variants` | Audit every IR sum-type variant for at least one `working-mods/*.txt` instance; delete any unevidenced variant along with the rules and tests that referenced it; install a doc-invariant guard test (or extend `audit_lib_panic_free.rs`-style audit) that asserts each shipping variant resolves to a corpus example | `typed-face-identifiers`, `typed-sprite-identifiers`, `merge-derived-contract` |

## Risks and unknowns

- **`ir-defaults-and-constructors` sprite-bearing constructor shape couples to `typed-sprite-identifiers`.** If `typed-sprite-identifiers` lands a `SpriteId` field shape different from what its plan declares (e.g. `Cow<'static, str>` swapped for `Arc<str>`), `ir-defaults-and-constructors` rebases. Mitigation: the `typed-sprite-identifiers` chunk plan freezes the `SpriteId` field shape in its Contracts section before `ir-defaults-and-constructors` opens.
- **`merge-derived-contract`'s regenerator byte-match is corpus-keyed.** `generate_pool_replacement` and `generate_hero_item_pool` must reproduce the existing hero-bound itempool and pool-replacement modifiers in the working-mods corpus byte-for-byte. If the corpus shape differs from `reference/textmod_guide.md` (the guide is the format authority, but the corpus is what the game accepts), the chunk plan resolves the conflict by following the guide; a corpus-vs-guide divergence is a parser-fidelity feature concern, not platform-foundations.
- **`cross-kind-name-uniqueness` depends on the IR having `replica_items` populated.** If the extractor classifier never produces replica items (a parser-fidelity concern), X003 has nothing to fire on across the `replica_item` bucket and the chunk's tests against that bucket cannot exercise it. Mitigation: tests construct `ModIR` values directly with literal `ReplicaItem`s, not through the extractor.
- **`corpus-evidenced-variants` may surface variants the brief did not name.** The audit is mechanical; if a sum-type variant has zero corpus instances, it is deleted regardless of how the chunk plan was scoped at authoring time. Each deletion forces a same-chunk update of any rule, test, or doc that referenced the variant. Mitigation: the chunk plan opens with a current-state audit listing every IR sum and its variants, surfacing the deletion list before the chunk authors any code.
- **`panic-free-library` competes with every chunk that introduces a new error path.** A chunk authored after `panic-free-library` lands that ships a new `unwrap()` fails the audit. Mitigation: every subsequent chunk's review checklist includes "run `cargo test --test audit_lib_panic_free`."

## Rollout plan

The compiler ships as a single Rust crate consumed by `compiler/src/main.rs` (CLI) and the (downstream) author-ergonomics feature. There is no flag system, no migration window, no telemetry. Each chunk lands as one PR; each PR re-runs the four-mod round-trip baseline (`cargo run --example roundtrip_diag`) and `cargo test`. Failures block the merge.

The four corpus mods (`working-mods/{sliceymon,pansaer,punpuns,community}.txt`) are the rollback authority — any chunk whose merge degrades round-trip equality on any of the four reverts. No "behind a flag" path; the chunks change library shapes that the CLI and downstream features link against directly.

## Dependency graph

```
   structured-error
   ├── typed-face-identifiers
   ├── typed-sprite-identifiers
   │     └── ir-defaults-and-constructors
   ├── build-source-filter
   └── merge-derived-contract

   finding-provenance
   └── cross-kind-name-uniqueness

   structured-error          ──┐
   typed-face-identifiers    ──┤
   typed-sprite-identifiers  ──┼──> panic-free-library
   merge-derived-contract    ──┘

   typed-face-identifiers    ──┐
   typed-sprite-identifiers  ──┼──> corpus-evidenced-variants
   merge-derived-contract    ──┘
```

Parallelism: once `structured-error` lands, `typed-face-identifiers`, `typed-sprite-identifiers`, `build-source-filter`, and `merge-derived-contract` open in parallel. `finding-provenance` is independent of the entire structured-error subgraph and can land at any time. `cross-kind-name-uniqueness` opens once `finding-provenance` lands. `panic-free-library` and `corpus-evidenced-variants` land last (after their respective dependency sets); both can run in parallel with each other since they touch disjoint files.
