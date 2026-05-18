# Chunk: `harvester-pure-core` — Extract the harvester pure core into `compiler/src/harvester/` library-reachable without `std::fs`

**Slug:** `harvester-pure-core`
**Feature:** `compiler-invariant-completions`
**PR:** —
**Depends on:** — (Foundation wave; no upstream chunk in this feature)
**Brief:** [`../brief.md`](../brief.md) · **Engineering plan:** [`../engineering-plan.md`](../engineering-plan.md) · **Decisions:** [`../decisions.md`](../decisions.md) · **SPEC anchor:** §3.4 (Library first, CLI second, WASM-ready)

> This plan is derived from the engineering plan, which is derived from the brief. If you can't restate this chunk's purpose in terms of a brief Goal or User-facing change, stop and re-read both before continuing.

## Goal

Extract the harvester pure core (today inlined in `compiler/build.rs` as `harvest_face_ids` + `harvest_sprites` + their support functions) into a new top-level crate module `compiler/src/harvester/` reachable from library code without `std::fs`, leaving `compiler/build.rs` as the thin I/O shell (`fs::read_to_string` for corpus bytes, `fs::write` to `OUT_DIR`, `phf_codegen::Map::build()` for sprite registry assembly); the refactor's byte-equivalence is verified by a one-time pre-vs-post SHA-256 ledger on `face_id_generated.rs` + `sprite_registry_generated.rs`.

## Brief link

- **Goal advanced (foundational only):** "Generated build artifacts are byte-deterministic, asserted by CI." This chunk lands the structural enabler — a pure core invokable from `compiler/tests/` via `include_str!`-baked corpus per decisions.md 2026-05-08 "Determinism audit requires harvester core invokable outside `cargo build`" — and the one-time refactor-time byte-identity check per decisions.md 2026-05-14 "Harvester SHA ledger consolidation". The Goal's recurring CI verifier is delivered by sibling `harvester-determinism-audit`, not this chunk.
- **Non-goal honored — the no-new-author-ergonomics Non-goal:** "No new author-ergonomics surface, with bounded carve-outs." The new `compiler/src/harvester/` module is build-time code-generation infrastructure, not an authoring surface. It carries no `::new` constructors, no chainable builders. Mod authors never import from `textmod_compiler::harvester::`; consumers are `compiler/build.rs` plus `harvester-determinism-audit`'s integration test.
- **Non-goal honored — the no-parser-emitter-changes Non-goal:** "No parser or emitter behavior changes beyond what the four bounded carve-outs require." The harvester is build-time code generation feeding `compiler/src/authoring/{face_id.rs, sprite.rs}` via `include!(concat!(env!("OUT_DIR"), "/..."))`. The chunk preserves byte-identity of `face_id_generated.rs` + `sprite_registry_generated.rs`, so the `include!`-consuming files compile against identical input post-refactor. No parser, emitter, extractor, builder, or xref source is edited.

## Context pack

**Read first:**
- `features/compiler-invariant-completions/decisions.md` — four load-bearing entries: 2026-05-08 "Determinism audit requires harvester core invokable outside `cargo build`"; 2026-05-08 "Harvester core module placement and reach mechanism" (durable promotion at the engineering-plan Decisions closure row of the same name); 2026-05-11 entries on `phf_codegen` confinement + lexical denylist scope; 2026-05-14 "Harvester SHA ledger consolidation".
- `features/compiler-invariant-completions/engineering-plan.md` — chunk-index row for `harvester-pure-core`; Decisions closure rows "Harvester core invocability and input shape", "Harvester core module placement and reach mechanism", "Determinism audit detection criterion", "`phf_codegen` confinement", "Lexical denylist for harvester pure-core determinism audit", "Pre-vs-post byte-identity SHA-256 ledger", "`SPRITE_REGISTRY` access-path stability"; Invariants entry "Byte-deterministic generated source"; §Build determinism; §Manual gates row "Byte-determinism re-run after harvester edits"; Risks bullet on the SHA-256 divergence check ownership; Contract edges rows `harvester-pure-core` → `audit-harness` + `harvester-pure-core` → future browser/mobile mod-builder app feature.
- `compiler/build.rs` — the file this chunk guts-and-refactors. Today carries `main` orchestration, the `KNOWN_FACE_NAMES` curated table, `read_working_mods` + `workspace_root` I/O helpers, pure data structures `FaceIdMeta` + `SpriteEntry`, pure harvest functions (`harvest_face_ids`, `scan_sd_face_ids`, `find_sd_value_end`, `parse_face_id_chunk`, `harvest_sprites`, `scan_entity_sprites`, `find_img_val_end`, `find_name_end`), and emitter helpers `emit_generated` (pure) + `emit_sprite_registry` (mixed; the inner phf_codegen glue retains in build.rs, the surrounding scaffolding moves).
- `compiler/src/lib.rs` — the existing module declaration block (the contiguous `pub mod <name>;` lines at the file head: `audit_markers`, `authoring`, `constants`, `error`, `finding`, `ir`, `extractor`, `builder`, `util`, `xref`). This chunk adds `pub mod harvester;` to that block. No crate-root re-export.
- `compiler/Cargo.toml` — confirmation that `phf_codegen = "0.11"` lives in `[build-dependencies]` and `phf = "0.11"` lives in `[dependencies]`. The pure-core's `phf_codegen` ban is enforceable because the library does not declare `phf_codegen` as a runtime dependency.

**Reference:**
- `features/compiler-invariant-completions/brief.md` — Goal 3 headline, the scope-clarifier prose "the build-time code-generation step that produces compiled-in source from the reference corpus — not `build(ir) → textmod` whole-mod emission", the SPEC §3.4 binding "the resulting harvester core is reachable from library code without importing `std::fs` or `std::process`".
- `compiler/src/authoring/sprite.rs` — the `include!(concat!(env!("OUT_DIR"), "/sprite_registry_generated.rs"));` macro call near the module's tail + the `SPRITE_REGISTRY.get(name)` lookup inside `SpriteId::lookup`. Background only.
- `compiler/src/authoring/face_id.rs` — the `include!(concat!(env!("OUT_DIR"), "/face_id_generated.rs"));` macro call. The `impl FaceId { ... }` block in the generated file lives inside the `authoring::face_id` module's privacy scope via the `include!`. Background only.
- `compiler/tests/audit_lib_panic_free.rs` — sibling integration test using a `walk_rs` recursive-walk helper over `compiler/src/**/*.rs`; the new `compiler/src/harvester/` files are automatically covered by its panic-free walk.
- `compiler/tests/audit_markers_smoke.rs` — sibling reference for the module-shape guard test pattern (read source, `strip_comments_and_blanks`, scan residue for banned items).
- `CLAUDE.md` — "Don't add features, refactor, or introduce abstractions beyond what the task requires." The pure-core is a structural extraction of existing code; rule shape, sort order, paired-name selection logic, and emitter formatting stay identical.

**Conventions / patterns to follow:**

*Pure-core signature shape:*
- **Input shape:** entry-point functions accept `&[(&'static str, String)]` slices in memory — the exact element-type shape today's `compiler/build.rs` callsites pass via `mod_contents: Vec<(&'static str, String)>` from `read_working_mods`.
- **Input rationale:** the pin is minimum-blast-radius. Post-relocation `compiler/build.rs` callsite is textually unchanged modulo the `harvester::` prefix. Per decisions.md 2026-05-08 "Determinism audit requires harvester core invokable outside `cargo build`" (slice shape `&[(name, contents)]` bound; element types deliberately unbound to leave chunk-plan specialization room).
- **No path arguments, no `std::fs`:** the pure-core takes bytes in memory; corpus reading lives in `build.rs`.
- **Return shape — `harvest_face_ids`:** returns `Vec<(String, String)>` pairs sufficient for `compiler/build.rs` glue to assemble byte-identical `face_id_generated.rs`. The first element is the `u16`-as-string FaceID; the second element is the opaque per-entry text block (doc-comment line + `pub const` line) the glue splices verbatim into the `impl FaceId { ... }` body.
- **Return shape — `harvest_sprites`:** returns `Vec<(String, String, String)>` triples sufficient for `compiler/build.rs` glue to assemble byte-identical `sprite_registry_generated.rs`. The first element is the sprite-name key; the second element is the opaque `SpriteId` value-expression string the glue feeds to `phf_codegen::Map.entry(name, expr)`; the third element is the `<mod_name>:<line_no>` provenance suffix string the glue emits into the `// Sprite provenance (stable order):` comment block ABOVE the phf static (per today's `emit_sprite_registry` two-position byte shape).
- **Return-shape asymmetry rationale:** face IDs encode their per-entry provenance as a `///` doc-comment line inside the per-entry block (single-position emission, fits a pair); sprite entries emit provenance in a separate block above the phf static (two-position emission requiring a second opaque-text column distinct from the value-expression). The triple is the minimum-blast-radius widening that preserves Property (a) opaque-text splice without licensing a side-channel `pub` helper. Per the engineering plan's §Build determinism + Decisions closure "`phf_codegen` confinement".

*Per-tuple encoding properties (architectural; constrain implementer's encoding choice within `Vec<(String, String)>`):*
- **(a) Opaque-text splice:** `compiler/build.rs` glue treats every non-key opaque-text element of each tuple as text spliced verbatim into the generated file — the second element of each `harvest_face_ids` pair, AND the second and third elements of each `harvest_sprites` triple. The glue performs zero parsing or string transformation on opaque-text columns; column boundaries are the bound integration contract.
- **(b) phf_codegen feed:** `compiler/build.rs` glue performs the `phf_codegen::Map::build()` assembly for the sprite registry by feeding the pure-core's triples into `phf_codegen::Map.entry(key, expr)` — first-element-as-key, second-element-as-value-expression. The third element (provenance suffix) bypasses phf and is emitted directly into the `// Sprite provenance (stable order):` comment block above the phf static. The SpriteId value-expression string `SpriteId { name: ::std::borrow::Cow::Borrowed(...), img_data: ::std::borrow::Cow::Borrowed(...) }` is constructed by pure-core and consumed as opaque text by build.rs.
- **(c) No SpriteId/FaceId import in build.rs:** neither pre-refactor nor post-refactor `compiler/build.rs` imports `SpriteId` or `FaceId` (build.rs is a separate compile unit from the library and cannot `use crate::authoring::sprite::SpriteId`). The pure-core textually constructs the value-expressions using `SpriteId`'s private field names (`name`, `img_data`) and `FaceId`'s tuple-newtype shape, identical to today's mechanism in `emit_sprite_registry`.
- **(c) license source:** the textual access to those private fields is licensed by the `include!`-into-privacy-scope at the macro calls near the tail of `compiler/src/authoring/sprite.rs` and `compiler/src/authoring/face_id.rs`. The relocation preserves this pre-existing coupling; it does not introduce a new coupling class.
- **(c) failure mode preserved:** a future contributor renaming `SpriteId.name` or reshaping `FaceId`'s newtype fails the `cargo build` of `compiler/src/authoring/{sprite.rs, face_id.rs}` at the `include!` site — identical to today's failure mode.
- **(d) Provenance data produced by pure-core:** today's `face_id_generated.rs` doc-comment lines (`/// FaceID {id} — first seen <mod>:<line>...`) and `sprite_registry_generated.rs` provenance block lines (`//   <name> ← <mod>:<line>` above the phf static) are produced from pure-core data, not by build.rs's own scan. For face IDs the provenance is embedded inside the second-element opaque-text block. For sprites the provenance is carried in the third element as the `<mod>:<line>` suffix string (build.rs glue prefixes `//   <name> ← ` and emits per-entry in the provenance block). Build.rs performs zero parsing of the corpus for provenance recovery — Property (a) opaque-text splice plus column bounds is sufficient.
- **Implementer's residual choice:** the implementer picks the precise per-tuple semantic within the column bounds set above. For face IDs the second element carries today's exact per-entry byte sequence — `    /// FaceID <id> — first seen <provenance>. See reference/textmod_guide.md.\n    pub const <const_name>: FaceId = FaceId(<id>);\n` — i.e., the four-space-indented `///` line terminated by `\n` followed by the four-space-indented `pub const` line terminated by `\n`. `compiler/build.rs` splices the second-element string verbatim into the `impl FaceId { ... }` body without adding indentation, separator, or trailing newline (the per-entry trailing newline IS in the second-element bytes). For sprites the second element carries the SpriteId value-expression string (Property b consumption) and the third element carries the `<mod>:<line>` provenance suffix (no surrounding whitespace, no trailing newline — build.rs prefixes `//   <name> ← ` and suffixes `\n` at splice time).
- **Sub-module helper visibility:** any such helper is `mod`-private or `pub(super)` within a sub-module under `compiler/src/harvester/`, reached only from the two entry-point functions. `pub(crate)` is banned — the bounded pub surface per §Contracts changed is the two entry-point functions only, so sub-module helpers are an implementation detail; `pub(crate)` is a misleading signal for a helper that isn't intended for in-library reach.

*Determinism + collection discipline:*
- **Vec ordering preservation:** both the `Vec<(String, String)>` and `Vec<(String, String, String)>` returns are built via `BTreeMap.into_iter().collect()` (or equivalent BTreeMap-keyed iteration), with the per-entry tuple shape derived inside the `.map(|(k, v)| ...)` adapter from `BTreeMap`'s key + value. `HashMap` is banned anywhere under `compiler/src/harvester/**/*.rs`, transitively.
- **harvest_sprites ordering — stricter:** the sprite return Vec MUST come from direct `BTreeMap.into_iter().collect()` (preserving today's `emit_sprite_registry` iteration form).
- **harvest_sprites no post-collect reordering:** no `.sort_by_key` even on a tautologically-equivalent key. `phf_codegen::Map::build()`'s output bytes depend on `.entry()` call order; a future contributor extending a tautological sort to a non-tautological form silently breaks byte-identity that no recurring CI gate catches.
- **harvest_face_ids ordering — looser:** the face-ID return Vec retains permission to derive ordering from any BTreeMap-keyed iteration. `Vec::sort_by_key` is admitted only when the key is totally ordered (e.g., the BTreeMap's own key type), because its consumption is order-preserved const-slice emit (no phf assembly). Pair shape `Vec<(String, String)>` is unchanged from prior rounds; the widening discussed for `harvest_sprites` does NOT propagate here because face-ID provenance is embedded in the second element rather than requiring a third column.
- **Hash-map iteration order is a regression class:** enumerated in decisions.md 2026-05-08 "process-distinguishable-input criterion".
- **Deterministic-iteration collection:** the harvester's internal aggregation uses `BTreeMap` (as today). The pure-core retains the existing `BTreeMap<u16, _>` / `BTreeMap<String, _>` aggregation shape.

*Module type universe + reach:*
- **No third-party crate imports:** harvester pure core compiles in `compiler/build.rs` (via `#[path]`) AND `compiler/src/lib.rs` (via `pub mod harvester;`). It depends ONLY on `core::*` + `std::*` + types it declares itself.
- **Implied bans:** the existing `[dependencies]` crates (`serde`, `serde_json`, `schemars`, `phf`, `clap`) and `[build-dependencies]` crate (`phf_codegen`) are off-limits inside `compiler/src/harvester/**/*.rs`.
- **Type universe:** primitives + module-local types defined inside `compiler/src/harvester/**/*.rs`. No imports of types from other compiler modules (no `use crate::`, no `use super::` reaching outside the module, no `use textmod_compiler::`). Per decisions.md 2026-05-11 lexical denylist positive-boundary specification.

*phf_codegen confinement (enforced by three layers; per decisions.md 2026-05-11 "phf_codegen confinement"):*
- **Layer 1 — library-side `cargo build` (strictly earliest gate):** a `phf_codegen::` reference inside `compiler/src/harvester/**/*.rs` fails the library compile because `phf_codegen` is in `[build-dependencies]` only, not `[dependencies]`. The pure-core reached via `pub mod harvester;` cannot resolve the symbol.
- **Layer 2 — module-shape guard test:** the introduction-side guard test under `compiler/tests/` performs a full-file scan and runs as an integration test (compiles after the library compile, so Layer 1 catches symbol-resolvable violations first).
- **Layer 2 — what it catches + diagnostics:** Layer 2 catches the Layer 3 gap below + dead-code reaches unreachable from `pub mod harvester;`, and produces better diagnostics — the test's failure message names the file + offending line.
- **Layer 3 — build-script `#[path]` reach does NOT enforce:** `phf_codegen` IS available to `compiler/build.rs`'s compile unit, so a `phf_codegen::` reference inside `compiler/src/harvester/**/*.rs` compiles cleanly when reached only via the `#[path]` declaration. Layers 1 and 2 together close this gap.

*Reach mechanisms (per decisions.md 2026-05-08 "Harvester core module placement and reach mechanism"; durable promotion landed at the engineering-plan Decisions closure row of the same name):*
- **Build-script reach:** `compiler/build.rs` reaches the pure core via a `#[path = "src/harvester/mod.rs"] mod harvester;` declaration at the top of the file. NOT via `use crate::harvester` or `use textmod_compiler::harvester` — both would create a build-script-on-library cyclic dependency per Cargo's compilation model.
- **Library-side reach:** `compiler/src/lib.rs` carries `pub mod harvester;` (added by this chunk). External consumers import as `textmod_compiler::harvester::{harvest_face_ids, harvest_sprites}`. The access path is forward-binding for `harvester-determinism-audit`'s integration test, the future browser/mobile mod-builder app feature, and any later audit consumer.
- **Entry-point name preservation:** pure-core entry points are named `harvest_face_ids` and `harvest_sprites` (the current `compiler/build.rs` function names, retained). The chunk relocates; it does not rename.
- **`SPRITE_REGISTRY` access-path stability:** `textmod_compiler::authoring::sprite::SPRITE_REGISTRY` stays as the import path consumed by `audit-harness`'s unknown-sprite escape-hatch detection. The chunk does not touch `compiler/src/authoring/sprite.rs`; byte-identity of `sprite_registry_generated.rs` (verified by the SHA ledger) ensures registry contents are unchanged.

*Byte-identity criterion:*
- **SHA capture:** the chunk captures SHA-256 of `face_id_generated.rs` + `sprite_registry_generated.rs` (in cargo's `OUT_DIR` after a clean build) BEFORE any pure-core extraction edits, runs the refactor, and captures the post-refactor SHA after another clean build.
- **REJECTED-on-divergence:** if the SHAs differ on either file, the chunk is REJECTED (divergence indicates either pre-existing latent non-determinism in current `compiler/build.rs` code or an out-of-scope structural change). The two SHA captures are the chunk's PR-attached test evidence. Per decisions.md 2026-05-11 round-1 resolution + 2026-05-14 "Harvester SHA ledger consolidation".

## Factoring Contract

**Owns (writes)** — exact paths this chunk creates or modifies.

- `compiler/src/harvester/mod.rs` — NEW file. Module root for the harvester pure core. Declares `pub fn harvest_face_ids(mods: &[(&'static str, String)]) -> Vec<(String, String)>` and `pub fn harvest_sprites(mods: &[(&'static str, String)]) -> Vec<(String, String, String)>` (signatures fully bound at the chunk-plan layer per §Conventions "Input shape" + "Return shape"; the return shapes are asymmetric to reflect the differing per-entry provenance emission shapes of `face_id_generated.rs` vs `sprite_registry_generated.rs`, see §Conventions "Return-shape asymmetry rationale").
- `compiler/src/harvester/mod.rs` — hosts (directly or via sub-modules under `compiler/src/harvester/`) the support functions relocated from `compiler/build.rs`: the `KNOWN_FACE_NAMES` curated table; the `FaceIdMeta` + `SpriteEntry` data structures; the scan helpers (`scan_sd_face_ids`, `find_sd_value_end`, `parse_face_id_chunk`, `scan_entity_sprites`, `find_img_val_end`, `find_name_end`); the pure-emitter helpers (`emit_generated` and the non-phf scaffolding of `emit_sprite_registry`).
- `compiler/src/harvester/mod.rs` — all types module-local; all dependencies primitives + `std::collections::BTreeMap` + `std::fmt::Write`. No `std::fs`, no `std::process`, no `std::env`, no `std::time::*`, no `OnceLock`/`OnceCell`/`lazy_static!`/`once_cell::*`, no `use crate::`/`use super::`/`use textmod_compiler::`, no `phf_codegen::*`. Per decisions.md 2026-05-11 lexical denylist positive-boundary specification + phf_codegen confinement.
- `compiler/build.rs` — refactored into a thin I/O shell. Retains `main()` orchestration (reads corpus from `working-mods/` via `fs::read_to_string`, calls `harvester::harvest_face_ids(&mod_contents)` + `harvester::harvest_sprites(&mod_contents)` via the `#[path]` declaration passing the corpus directly — no callsite conversion required because the bound input element shape matches `read_working_mods`'s return shape — iterates the `harvest_sprites` triples once to emit the provenance comment block (`//   <name> ← <provenance_suffix>` per entry) and a second time to feed `(name, value_expr)` into `phf_codegen::Map.entry(...)` for `phf_codegen::Map::build()`, writes both generated files to `OUT_DIR` via `fs::write`).
- `compiler/build.rs` — retains `read_working_mods` + `workspace_root` I/O helpers (signature unchanged); retains the `WORKING_MOD_ORDER` constant pinning the deterministic ingest order (sliceymon > pansaer > punpuns > community) for the pure core to consume.
- `compiler/build.rs` — adds the `#[path = "src/harvester/mod.rs"] mod harvester;` declaration at the top of the file (the build-script reach mechanism into the pure-core's module tree).
- `compiler/build.rs` — drops the inlined pure-harvester logic: `harvest_face_ids` / `harvest_sprites` function bodies; the `KNOWN_FACE_NAMES` curated table; the `FaceIdMeta` + `SpriteEntry` data-structure declarations; the scan helpers (`scan_sd_face_ids`, `find_sd_value_end`, `parse_face_id_chunk`, `scan_entity_sprites`, `find_img_val_end`, `find_name_end`); the pure-emitter helpers (`emit_generated` and the non-phf scaffolding of `emit_sprite_registry`). All now relocated to `compiler/src/harvester/`.
- `compiler/src/lib.rs` — add `pub mod harvester;` to the existing module declaration block (the contiguous `pub mod <name>;` lines at the file head). Placement order within the block is the implementer's call (the existing block's ordering is not strictly alphabetical — `ir` precedes `extractor`, `builder` precedes `util` — so the chunk plan does not bind a position). No crate-root re-export is added (the bound access-path is the longer-form `textmod_compiler::harvester::{harvest_face_ids, harvest_sprites}` per decisions.md "Harvester core module placement and reach mechanism", symmetric to `textmod_compiler::authoring::sprite::SPRITE_REGISTRY`).

**Reads (no writes)** — files this chunk depends on but does not modify.

- `compiler/src/authoring/face_id.rs` — the `include!(concat!(env!("OUT_DIR"), "/face_id_generated.rs"));` macro call consumes the harvester's face-ID output. Byte-identity of the generated file ensures this `include!` lands at the same byte content post-refactor.
- `compiler/src/authoring/sprite.rs` — the `include!(concat!(env!("OUT_DIR"), "/sprite_registry_generated.rs"));` macro call consumes the harvester's sprite-registry output. Byte-identity ensures this `include!` lands at the same byte content post-refactor.
- `compiler/Cargo.toml` — for confirmation of `[build-dependencies] phf_codegen = "0.11"` + `[dependencies] phf = "0.11"`. The pure-core's phf_codegen ban is enforced by the three layers in §Conventions.
- `working-mods/{sliceymon,pansaer,punpuns,community}.txt` — the corpus the harvester scans at build time. Not edited (per the no-game-balance Non-goal). The four mods are the byte-identity verification anchor.

**Forbidden** — paths explicitly off-limits to this chunk.

- `compiler/src/authoring/`, `compiler/src/extractor/`, `compiler/src/builder/`, `compiler/src/ir/`, `compiler/src/xref.rs`, `compiler/src/finding.rs`, `compiler/src/audit_markers.rs`, `compiler/src/error.rs`, `compiler/src/constants.rs`, `compiler/src/util.rs` — no edits. The chunk's blast radius is three source files (`compiler/src/harvester/mod.rs` new, `compiler/build.rs` refactored, `compiler/src/lib.rs` one-line addition) plus the new `compiler/tests/` smoke + shape-guard test files.
- `compiler/tests/audit_harvester_determinism.rs` (or analogously named) — the recurring CI determinism gate is `harvester-determinism-audit`'s scope.
- Lexical denylist test on `compiler/src/harvester/**/*.rs` as a permanent CI test — `harvester-determinism-audit`'s scope. This chunk's `compiler/tests/harvester_module_shape.rs` enforces introduction-side hygiene at this chunk's PR; the broader recurring-CI lexical denylist with the `compiler/build.rs` allowlist is the audit chunk's load-bearing artifact.
- Any new pub IR enum or change to existing pub enums in the harvester module. The pure-core uses no pub enums (`FaceIdMeta` + `SpriteEntry` are private `struct`s; their visibility stays `mod`-private or `pub(super)` within their sub-module). The pure-core's API surface is the two entry-point functions plus their per-function return types (`Vec<(String, String)>` from `harvest_face_ids`; `Vec<(String, String, String)>` from `harvest_sprites`, the third column carrying the `<mod_name>:<line_no>` provenance suffix).
- `archive/pre-guide/`, `archive/platform-foundations/`, `archive/plans/` — predate this feature; not authoritative.

**Single concern**

> Relocate the harvester pure core from `compiler/build.rs` to a library-reachable `compiler/src/harvester/` module under the bound input/output shape, with `compiler/build.rs` reduced to the I/O shell, such that `face_id_generated.rs` + `sprite_registry_generated.rs` are byte-identical across the refactor (verified by the one-time SHA-256 ledger).

**No scaffolding**

The relocation, the I/O-shell reduction, the library reach-in (`pub mod harvester;`), and the one-time SHA-256 ledger form one mutually-load-bearing unit:

- The relocation without the I/O-shell reduction would leave the harvester logic inlined inside `compiler/build.rs` AND duplicated under `compiler/src/harvester/` — a SPEC §3.7 parallel-representation hazard. Both copies would drift on any future harvester edit.
- The I/O-shell reduction without the relocation has no destination — the I/O shell must reach the relocated pure core via `#[path]`; without the relocated module there is nothing to reach.
- The library reach-in (`pub mod harvester;`) without the relocation has no module to register; without the I/O-shell reduction the registered module's symbols are unused.
- The SHA-256 ledger without the relocation has nothing to verify. Per decisions.md 2026-05-14 "Harvester SHA ledger consolidation", the one-time check is bound to live entirely inside this chunk's PR; splitting it to `harvester-determinism-audit` would invert rejection ownership.

The smoke + module-shape guard test additions under `compiler/tests/` are bundled with the src/ + build.rs edits — not by structural-atomicity of Rust's module mechanism (each `compiler/tests/*.rs` integration test is its own compilation unit and could ship independently), but because the smoke test's job is to prove the pure-core resolves at the bound crate-external path AND is invocable from a non-build context with `include_str!`-baked corpus *in this chunk*, before `harvester-determinism-audit` can rely on those properties.

- [ ] `compiler/src/harvester/mod.rs` (and any sub-modules) contains only the relocated pure harvester logic + support data structures + `KNOWN_FACE_NAMES` — no I/O helpers, no `phf_codegen` invocations, no imports beyond `std::collections::BTreeMap` + `std::fmt::Write` (or equivalent primitive-and-module-local-only set).
- [ ] `compiler/build.rs` post-refactor contains exactly the bound I/O-shell surface — `main` orchestration + `read_working_mods` + `workspace_root` + `WORKING_MOD_ORDER` + the `#[path]` `mod harvester;` declaration + the `phf_codegen::Map::build()` glue assembling the sprite registry's static from the pure-core's returned `Vec<(String, String, String)>` triples (iterating the triples once for the `// Sprite provenance (stable order):` comment block emission, second time for `phf_codegen::Map.entry((name, value_expr))` feed) + the `fs::write` calls. No inlined harvest/scan/emit logic; no `KNOWN_FACE_NAMES`; no `FaceIdMeta` / `SpriteEntry` struct definitions.
- [ ] `compiler/src/lib.rs` adds exactly one new line — `pub mod harvester;` — to the module declaration block. No new crate-root re-export.
- [ ] Pre-vs-post SHA-256 ledger captured in this chunk's PR shows zero divergence on both `face_id_generated.rs` and `sprite_registry_generated.rs`.

**Abstraction earns its place**

The new `compiler/src/harvester/` module is a structural relocation of existing pure code, not a new abstraction. Alternatives considered per decisions.md 2026-05-08 — duplicate the harvester logic into a test helper (parallel implementation drift), add a separate cargo binary target that re-implements the harvester (same drift hazard plus workspace ceremony), spawn `cargo build` twice from a test and diff `OUT_DIR` files (slow on CI, fragile across cargo versions / target dirs / incremental-build cache state) — were rejected at the brief-tribunal layer.

Two consumers depend on the relocation landing within this feature's chunk DAG:

- **Consumer 1 (Second-wave sibling per the engineering plan's DAG):** `harvester-determinism-audit`. Its `audit_harvester_determinism.rs` integration test calls `textmod_compiler::harvester::harvest_face_ids` and `harvest_sprites` against `include_str!`-baked corpus, invokes them twice in-process, and asserts the two `Vec<(String, String)>` outputs from `harvest_face_ids` AND the two `Vec<(String, String, String)>` outputs from `harvest_sprites` are byte-equal per invocation pair (per-shape byte-equality preserves the asymmetric per-entry contract this chunk binds). Without the pure-core extraction, the integration test cannot reach the harvester logic without re-implementing it.
- **Consumer 2 (Third-wave audit harness sibling per the engineering plan's DAG):** `audit-harness`. Its unknown-sprite escape-hatch detection consults `textmod_compiler::authoring::sprite::SPRITE_REGISTRY`. This chunk's byte-identity guarantee on `sprite_registry_generated.rs` keeps `SPRITE_REGISTRY` contents unchanged, so `audit-harness`'s walk reaches the same registry post-refactor.

Forward-binding to one open-ended future consumer: the future browser/mobile mod-builder app feature (SPEC §1 vision pillar 4) requires the harvester pure core to be reachable from a WASM-targeted compile of `textmod_compiler`. This chunk's `compiler/src/harvester/` extraction is the structural enabler; the app feature's WASM compile is out of scope here.

**Contracts changed** — new pub surface introduced.

- New `pub mod harvester;` in `compiler/src/lib.rs` (new top-level crate module).
- New `pub fn textmod_compiler::harvester::harvest_face_ids(mods: &[(&'static str, String)]) -> Vec<(String, String)>` (signature fully bound at the chunk-plan layer per §Conventions "Input shape" + "Return shape — `harvest_face_ids`").
- New `pub fn textmod_compiler::harvester::harvest_sprites(mods: &[(&'static str, String)]) -> Vec<(String, String, String)>` (signature fully bound at the chunk-plan layer per §Conventions "Return shape — `harvest_sprites`"; the third tuple element carries the per-entry `<mod_name>:<line_no>` provenance suffix string that `compiler/build.rs` emits into the provenance comment block above the phf static — required for byte-identity of `sprite_registry_generated.rs`).

The new pub surface is exactly those three items. No `pub mod` declarations inside `compiler/src/harvester/**/*.rs` (sub-modules MUST be `mod`-private; a `pub mod` surfaces sub-module-level helpers at the crate-external path, exceeding the bounded pub surface). No `pub fn` other than `harvest_face_ids` and `harvest_sprites`. No `pub struct`, `pub enum`, `pub const`, `pub static`, or `pub trait` anywhere under `compiler/src/harvester/**/*.rs`. The relocated data structures (`FaceIdMeta`, `SpriteEntry`, `KNOWN_FACE_NAMES`) are module-local (`mod`-private or `pub(super)` within their sub-module); the relocated scan/parse/emit helpers are module-local under the same triple. Two `pub use` re-export shapes are licensed from `compiler/src/harvester/mod.rs` (one decomposition per file, not both): (a) the single-sub-module curly-set form `pub use <sub_module>::{harvest_face_ids, harvest_sprites};` if the implementer places both function bodies in one `mod`-private sub-module; or (b) the per-function two-line form `pub use <sub_face>::harvest_face_ids;` plus `pub use <sub_sprites>::harvest_sprites;` if the implementer places each function's body in its own `mod`-private sub-module. Picking one decomposition shape rather than mixing them is the implementer's discipline; the dual-import error mode that would arise from re-exporting the same identifier twice from different sub-modules (e.g., `pub use combined::{harvest_face_ids, harvest_sprites};` plus a stray `pub use other::harvest_face_ids;`) is caught structurally by `rustc`'s duplicate-import error (E0252), not by the module-shape guard test. The guard test enforces per-line shape conformance to one of the two regexes; cross-line mixed-import is `rustc`'s enforcement layer.

### Tests to add

Test harness layout (one combined file vs separate smoke + module-shape files; fixture corpus layout) is the implementer's call.

#### Library-reachability + invocability smoke test

Suggested file: `compiler/tests/harvester_invocability_smoke.rs`.

*Corpus construction.* Include the four reference mods via `include_str!`:
- `include_str!("../../working-mods/sliceymon.txt")`
- `include_str!("../../working-mods/pansaer.txt")`
- `include_str!("../../working-mods/punpuns.txt")`
- `include_str!("../../working-mods/community.txt")`

Build the `Vec<(&'static str, String)>` corpus in `WORKING_MOD_ORDER` priority (sliceymon > pansaer > punpuns > community) by allocating an owned `String` per mod from the `include_str!`-returned `&'static str` via `.to_string()`. The `.to_string()` per-mod allocation is the cost of matching today's `compiler/build.rs` callsite element-type shape (4 mods × kB-scale strings; negligible in a test).

Example construction:

```rust
let mods: Vec<(&'static str, String)> = vec![
    ("sliceymon", include_str!("../../working-mods/sliceymon.txt").to_string()),
    ("pansaer", include_str!("../../working-mods/pansaer.txt").to_string()),
    ("punpuns", include_str!("../../working-mods/punpuns.txt").to_string()),
    ("community", include_str!("../../working-mods/community.txt").to_string()),
];
```

*Invocations.* Call `textmod_compiler::harvester::harvest_face_ids(&mods)` and `textmod_compiler::harvester::harvest_sprites(&mods)` ONCE each. Multi-invocation byte-determinism is `harvester-determinism-audit`'s recurring CI gate scope.

*Runtime assertions:*
- `harvest_face_ids` returns `Vec<(String, String)>`; `harvest_sprites` returns `Vec<(String, String, String)>` (the third element carries the per-entry `<mod_name>:<line_no>` provenance suffix the build-script glue feeds into the provenance comment block).
- `harvest_face_ids` length ≥ 18.
- The 18-floor's mechanism: the `KNOWN_FACE_NAMES` curated table is unconditionally seeded by today's belt-and-suspenders loop.
- The 18-floor's catch surface: a regression dropping corpus scanning would still emit at least the curated names, so the 18-floor catches a regression that drops the curated seeding.
- `harvest_sprites` returns at least one tuple (non-empty).
- Why no numeric floor on `harvest_sprites`: an unverified numeric floor would either ship the smoke test RED on count-mismatch unrelated to harvester correctness, or admit a silent regression below the hand-estimated floor.
- Where content-regression is caught instead: the SHA-256 ledger under §Acceptance criteria is the byte-identity gate; any majority-drop regression on sprite content surfaces as byte-divergence on `sprite_registry_generated.rs`.
- This smoke test is scoped to invocability + signature + ordering, not per-entry count audit.
- `harvest_sprites`'s return Vec has first elements lex-ascending AND unique (deterministic-ordering smoke); the third element of each triple is non-empty (provenance suffix populated for every entry) and matches the byte-shape `<mod_name>:<line_no>` (e.g., regex `^[a-z][a-z_]*:[1-9][0-9]*$` over the per-tuple third element).
- Why lex-ascending is automatic: source aggregation is `BTreeMap<String, _>` per §Conventions "Vec ordering preservation"; `BTreeMap::into_iter` over String keys produces lex-ascending output and the first-element of each tuple IS the String key.
- Why the third-element shape assertion: the provenance suffix is consumed byte-verbatim by `compiler/build.rs` glue into the provenance comment block; an empty third element would silently corrupt that block (single-space gap after `←`), and a malformed shape (missing colon, non-numeric line number) would land bytes in the generated file that do not match today's `emit_sprite_registry` output, REJECTing via the SHA ledger. The regex floor is a runtime check that fails the smoke test BEFORE the SHA ledger fires, giving a sharper failure mode than ledger divergence on this column.
- `harvest_face_ids`'s return Vec has first elements unique (deterministic-presence smoke).
- Why uniqueness rather than lex-monotone: source aggregation is `BTreeMap<u16, _>`; the implementer's first-element encoding may derive from the u16 key OR from `FaceIdMeta`'s name. A lex-monotone assertion would underspecify the encoding because `"107"` lex-sorts before `"13"` on decimal-prefix forms.
- Uniqueness still rules out a `HashMap` intermediate that collapsed two distinct u16 keys to the same first-element string, without binding the implementer's encoding choice.

*Compile-time assertions (failure surfaces as `cargo test` build-phase errors, not test-runtime failures):*
- The smoke test does not compile if `textmod_compiler::harvester::harvest_face_ids` or `harvest_sprites` fails to resolve at the bound crate-external path (a `use textmod_compiler::harvester::*` site in the test file fails).
- The smoke test does not compile if the function signatures cannot be invoked from `compiler/tests/` against the `include_str!`-baked compile-time corpus. The smoke test compiles against `harvest_face_ids: &[(&'static str, String)] -> Vec<(String, String)>` AND `harvest_sprites: &[(&'static str, String)] -> Vec<(String, String, String)>`; any signature drift away from either bound shape fails the test file's compile phase.

*Pre-implementation gate.* `rg "include_str!.*working-mods" compiler/tests/` returns zero hits BEFORE this test lands (the regex catches both `include_str!("../../working-mods/...")` and `include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../working-mods/..."))` patterns).

#### Module-shape guard test

Suggested file: `compiler/tests/harvester_module_shape.rs`.

Read every `.rs` file under `compiler/src/harvester/` recursively (mirroring the `compiler/tests/audit_lib_panic_free.rs:walk_rs` pattern). The guard splits the banned-pattern set into two scopes mirroring `compiler/tests/audit_markers_smoke.rs`.

*Residue scope.* Scan the comment-and-blanks-stripped residue per the sibling `strip_comments_and_blanks` helper. Apply banned patterns at start-of-non-whitespace on the residue using explicit allow-list predicates rather than bare `starts_with` (which cannot discriminate `use std::` from `use crate::`).

- Bans a residue line whose trimmed start matches `use ` UNLESS its trimmed start also matches `use std::` or `use core::` (the only two prelude-equivalent stdlib roots permitted).
- Bans `pub use ` UNLESS the line matches one of two licensed re-export regexes (per §Contracts changed, both shapes admitted; mixing one function via curly-set and the other via single-target is banned).
- Licensed regex (a) — single-sub-module curly-set form: `^pub use [a-zA-Z_][a-zA-Z0-9_]*::\{(harvest_face_ids,\s?harvest_sprites|harvest_sprites,\s?harvest_face_ids)\};$`. Either identifier ordering of the function-name pair is admitted within the curly set.
- Licensed regex (b) — per-function single-target form: `^pub use [a-zA-Z_][a-zA-Z0-9_]*::(harvest_face_ids|harvest_sprites);$`. Each line re-exports one function from its own `mod`-private sub-module; the two licensed lines may use different sub-module names (e.g., `pub use face_ids::harvest_face_ids;` plus `pub use sprites::harvest_sprites;`).
- Banned deviations: any deviation in identifier set; multi-line curly-set shape; trailing-comma forms not matching either regex; `pub use` of any identifier other than the two entry-point function names. Per-line shape conformance is the guard test's enforcement scope; cross-line mixed-import (the same function re-exported via both shape (a) and shape (b)) is structurally caught by `rustc`'s duplicate-import error (E0252) rather than by this guard test, per §Contracts changed.
- The sub-module name `[a-zA-Z_][a-zA-Z0-9_]*` follows the standard Rust identifier rule — e.g., `pub use inner::{harvest_face_ids, harvest_sprites};` if bodies live in `compiler/src/harvester/inner.rs`, OR `pub use face_ids::harvest_face_ids;` plus `pub use sprites::harvest_sprites;` if bodies split per-function across `compiler/src/harvester/face_ids.rs` + `compiler/src/harvester/sprites.rs`.
- Bans `use crate::` and `use super::` outright (the engineering-plan-bound denylist forbids `use super::` reach across the harvester pure-core's tree; sub-modules that need types from a sibling or parent fully-qualify with `super::Foo` inline at each callsite).
- Bans `use textmod_compiler::` outright.
- Bans `extern crate ` (Rust 2015-edition external-crate reach; admissible in Rust 2021 but bypasses the `phf_codegen::` symbol-prefix ban because `extern crate <crate>;` introduces a name into scope without a `use` line). This closes the Layer 3 gap by catching `extern crate phf_codegen;` as a textual residue match.
- Bans `pub mod ` (sub-modules MUST be `mod`-private per §Contracts changed; `pub mod` would surface sub-module helpers at the crate-external import path).
- Bans `pub fn ` other than `pub fn harvest_face_ids` / `pub fn harvest_sprites`.
- Bans `pub struct `, `pub enum `, `pub const `, `pub static `, `pub trait ` (none licensed by §Contracts changed).

*Full-file scope.* Scan the unstripped source. String-literal hits are accepted as deliberate strictness (today's `emit_generated` output strings do not name `std::fs` / `std::process` / `std::env` / `phf_codegen` symbols, so the full-file scan does not false-positive on emitted byte content).

The bound denylist + this chunk's introduction-side floors (the latter are added on top of the engineering-plan-bound denylist):
- `std::time::` (engineering-plan-bound denylist).
- `std::env::` (engineering-plan-bound denylist).
- `std::fs::` (engineering-plan-bound denylist).
- `std::process::` (newly added — SPEC §3.4 binds 'no `std::fs` or `std::process`' as library-side hygiene).
- `std::path::Path`, `std::path::PathBuf`, `AsRef<Path>`, `Path::new(` (newly added — decisions.md 2026-05-08 binds 'no `Path` arguments').
- `OnceLock`, `OnceCell`, `lazy_static!`, `once_cell::` (engineering-plan-bound denylist).
- `fs::read_dir` (engineering-plan-bound denylist).
- `crate::` as path-prefix (catches `crate::ir::Foo` reach attempts; a `///` doc-comment mentioning `crate::ir` IS scanned at full-file scope but is the deliberate strictness floor, per the sibling `audit_markers_smoke.rs:audit_markers_full_file_carries_no_disallowed_std_imports`).
- **Doc-comment authoring constraint from the full-file `crate::` ban:** any `///` or `//!` doc-comment in `compiler/src/harvester/**/*.rs` that needs to describe the build-script's reach mechanism must paraphrase rather than write literal `crate::` substrings. Licensed phrasings: "the build script reaches this via `#[path]` rather than a library-internal `use`-path"; "reached as `harvester::harvest_sprites` from `compiler/build.rs` after the `#[path]` declaration"; "not consumed via a library-internal `use`-path." Banned phrasings (each trips the full-file scan): "NOT reached via `use crate::harvester`"; "build.rs does not declare `use crate::harvester::*`"; "see `crate::ir::Source` for the analogous pattern." The strictness floor is preserved; the constraint is on doc-comment prose only. The implementer who needs to cross-reference a library-internal type uses the type's bare name and lets the reader find it via `rg` rather than embedding a `crate::` path.
- `phf_codegen::` (engineering-plan-bound denylist).
- `env!(`, `option_env!(` (newly added — macros that the `std::env::*` symbol-prefix scan misses; compile-time environment access via `env!("OUT_DIR")` would break library-reachability).
- `unsafe ` / `unsafe{` (newly added — harvester pure core uses no `unsafe` today; ban prevents future microoptimization drift).
- `std::thread::`, `std::sync::mpsc::`, `std::net::` (newly added — WASM-readiness floors).
- `std::io::Stdin`, `std::io::Stdout`, `std::io::Stderr`, `std::io::stdin`, `std::io::stdout`, `std::io::stderr` (newly added — WASM-readiness floors; a future contributor adding a debug-print scaffold via `std::io::stdin()` would land cleanly through the existing denylist's `std::(time|env|fs|process|path)::` set but break the forward-binding WASM target).
- `extern crate ` (newly added — closes the same Layer 3 gap that the residue-scope `extern crate ` ban also closes).

*Why two scopes:* the WASM-readiness OS-services group and `extern crate ` are introduction-side floors that no upstream artifact binds; the §Abstraction earns its place forward-binding to the future browser/mobile mod-builder app's WASM compile motivates them. They extend the discipline; the broader recurring-CI lexical denylist (with `compiler/build.rs` allowlist) is `harvester-determinism-audit`'s scope.

*Assertion shape.* Standard `#[test]` panic on any hit. Each panic message names the file path + matched pattern + offending line for diagnosis. Once landed, any future drift on `compiler/src/harvester/**/*.rs` trips this guard in the chunk that introduces it, not three rounds later.

## Acceptance criteria

- [ ] `compiler/src/harvester/mod.rs` exists; verify with `rg -nc "^pub fn harvest_face_ids" compiler/src/harvester/mod.rs` AND `rg -nc "^pub fn harvest_sprites" compiler/src/harvester/mod.rs` each returning at least `1`. Lower bound — the implementer may declare additional sub-module-level pub re-exports.
- [ ] Full pure-core signatures `pub fn harvest_face_ids(mods: &[(&'static str, String)]) -> Vec<(String, String)>` AND `pub fn harvest_sprites(mods: &[(&'static str, String)]) -> Vec<(String, String, String)>` (the latter's third tuple element carries the per-entry `<mod_name>:<line_no>` provenance suffix string consumed by `compiler/build.rs`'s provenance comment block emission, per §Conventions "Return shape — `harvest_sprites`") are verified by the smoke test's successful compilation at the bound crate-external path.
- [ ] `pub mod harvester;` lands in `compiler/src/lib.rs`; verify with `rg -nc "^pub mod harvester;\s*$" compiler/src/lib.rs` returning exactly `1`.
- [ ] `compiler/build.rs` reaches the pure core via `#[path]` declaration; verify with `rg -nc '#\[path = "src/harvester/mod\.rs"\]\s*mod harvester;' compiler/build.rs` returning exactly `1`.
- [ ] `compiler/build.rs` no longer hosts the harvester functions; verify with `rg "^fn (scan_sd_face_ids|find_sd_value_end|parse_face_id_chunk|emit_generated|scan_entity_sprites|find_img_val_end|find_name_end|emit_sprite_registry)" compiler/build.rs` returning zero hits.
- [ ] `compiler/build.rs` no longer hosts the harvester data structures; verify with `rg "^(struct FaceIdMeta|struct SpriteEntry|const KNOWN_FACE_NAMES)" compiler/build.rs` returning zero hits.
- [ ] Property (c) mechanization — no parallel struct: `rg -nE '^(pub )?struct (SpriteId|FaceId)\b' compiler/build.rs` returns zero hits. Catches a workaround where build.rs declares a parallel local struct (a SPEC §3.7 / CLAUDE.md no-parallel-representations violation).
- [ ] Property (c) mechanization — no `use` import: `rg -nE '^use [^;]*\b(SpriteId|FaceId)\b' compiler/build.rs` returns zero hits. Structurally guaranteed to fail at build.rs's compile site, but the mechanical check anchors Property (c) at land time.
- [ ] Property (c) scope clarification: textual mentions of `SpriteId` / `FaceId` in doc-comments or in phf_codegen value-expression strings are licensed by Property (a) and not caught by either regex above; the regexes target Rust-level imports and local-struct declarations only.
- [ ] Property (c) recurring-CI scope: this AC is the chunk-author's one-time land-time hand-verification; the recurring-CI mirror is `harvester-determinism-audit`'s scope (see §Out of scope).
- [ ] Type-universe enforcement — `std::` services group: `rg -E "std::(time|env|fs|process|path|thread|net)::|std::sync::mpsc::|std::io::(Stdin|Stdout|Stderr|stdin|stdout|stderr)" compiler/src/harvester/` returns zero hits.
- [ ] Type-universe enforcement — lazy-init group: `rg -E "OnceLock|OnceCell|lazy_static!|once_cell::|fs::read_dir" compiler/src/harvester/` returns zero hits.
- [ ] Type-universe enforcement — cross-module reach group: `rg -E "use crate::|use super::|use textmod_compiler::|phf_codegen::|AsRef<Path>|Path::new\(|env!\(|option_env!\(|\bunsafe[[:space:]{]|^extern crate " compiler/src/harvester/` returns zero hits.
- [ ] Type-universe enforcement — `crate::` path-prefix hand-verification: `rg -E "(^|[^a-zA-Z0-9_])crate::" compiler/src/harvester/` returns zero hits. Boundary-anchored to avoid false-matching identifiers like `iterate::`. Operates at full-file scope; today's relocated source carries no `crate::` in emitted strings or doc-comments, so any hit indicates a real violation.

### Pre-vs-post SHA-256 ledger procedure

Captured in the PR as test evidence. Run sequentially:

1. **Pre-capture (on a clean checkout of `main` before any chunk edits):** `cd compiler && cargo clean && cargo build` (debug profile, the cargo default; if your CI runs release builds, substitute `target/release/build/` in path patterns below and ensure both pre and post captures use the same profile).
2. **Ground-truth the wildcard expansion:** `find target/debug/build -type f -name face_id_generated.rs | wc -l` returns exactly `1` AND `find target/debug/build -type f -name sprite_registry_generated.rs | wc -l` returns exactly `1`. If either returns `0` the build did not produce the file; if either returns `>1` the build directory has stale hash-dirs from prior incremental builds and `cargo clean` did not collapse to one match — re-run `cargo clean && cargo build` before proceeding.
3. **Capture pre digests:** `shasum -a 256 $(find target/debug/build -type f -name face_id_generated.rs) $(find target/debug/build -type f -name sprite_registry_generated.rs)` to `pre.sha`.
4. **Post-capture (on the chunk's branch after all edits):** repeat steps 1-3 to produce `post.sha`.
5. **Compare digests, NOT filenames.** The digest (column 1 of `shasum -a 256` output, the hex string preceding the two-space separator and filename) on each generated file MUST match between `pre.sha` and `post.sha`. The filename column (column 2) WILL differ if the cargo build-hash directory changed between pre and post; that column-2 difference is expected and ignored.
6. **Attach `pre.sha` + `post.sha` to the PR.** Divergence on either generated file REJECTS the chunk per decisions.md 2026-05-11 round-1 resolution + 2026-05-14 "Harvester SHA ledger consolidation".

- [ ] Pre-vs-post SHA-256 ledger captured per the procedure above shows zero digest divergence on both `face_id_generated.rs` and `sprite_registry_generated.rs`.
- [ ] SHA-256 ledger is the sole land-time byte-equality gate. The roundtrip baseline suite at `compiler/tests/roundtrip_baseline.rs` exercises textmod IR roundtrip but does NOT observe the bytes of either generated file.
- [ ] The panic-free audit `compiler/tests/audit_lib_panic_free.rs` walks `compiler/src/**/*.rs` for panic patterns and is orthogonal to byte-identity; the `cargo test` checkbox below is a regression check on pre-existing test outcomes, not a byte-equality observer.
- [ ] `cd compiler && cargo test` passes. Runs the new library-reachability smoke test, the new module-shape guard test, the baseline-pinned regression suite, the panic-free audit (which now also covers `compiler/src/harvester/**/*.rs` automatically), the audit-marker smoke test, and every other integration test.
- [ ] Correctness bar: every pre-existing test outcome unchanged + the two new tests passing.
- [ ] `cd compiler && cargo run --example roundtrip_diag` shows `extract(build(extract(mod))) == extract(mod)` on all four reference mods (per CLAUDE.md "After changing the compiler"). The chunk does not touch extractor or builder logic; any drift indicates an unintended edit outside the chunk's owned files.
- [ ] No new pub IR enum is introduced in `compiler/src/harvester/`; verify with `rg "^pub enum" compiler/src/harvester/` returning zero hits. The cross-chunk vehicle-classification obligation does not apply because no in-scope pub enum lands here.
- [ ] `compiler/src/harvester/**/*.rs` honors the bounded pub-surface from §Contracts changed; verify with `rg -nE "^pub (mod|fn|struct|enum|const|static|trait|use) " compiler/src/harvester/`.
- [ ] Confirm allowed hits are limited to (i) `pub fn harvest_face_ids` and `pub fn harvest_sprites` declarations, (ii) optional `pub use` re-exports of those two function names from `compiler/src/harvester/mod.rs` if the implementer placed bodies in `mod`-private sub-modules.
- [ ] Confirm zero hits of `pub mod`, `pub struct`, `pub enum`, `pub const`, `pub static`, or `pub trait` in `compiler/src/harvester/`.

## Review checklist

- [ ] Pure-core entry points named `harvest_face_ids` + `harvest_sprites`. Signatures are asymmetric per the bound return shapes: `harvest_face_ids: &[(&'static str, String)] -> Vec<(String, String)>` and `harvest_sprites: &[(&'static str, String)] -> Vec<(String, String, String)>` (no `Path` arguments, no `std::fs::read_to_string` inside pure-core; the third element of the sprite tuples is the `<mod_name>:<line_no>` provenance suffix per §Conventions "Return shape — `harvest_sprites`"). The post-relocation `compiler/build.rs` callsite is textually unchanged modulo the `harvester::` prefix.
- [ ] Per-tuple encoding honors all four architectural properties pinned in §Conventions: (a) opaque-text splice; (b) phf_codegen feed; (c) no SpriteId/FaceId import in build.rs; (d) provenance comments produced by pure-core. A reviewer reads the implementer's chosen per-tuple semantic against these four properties.
- [ ] No `HashMap` reference anywhere under `compiler/src/harvester/**/*.rs`; `Vec<(String, String)>` returns derive their ordering from a `BTreeMap.into_iter().collect()` path. Hand-verify with `rg "HashMap" compiler/src/harvester/` returning zero hits.
- [ ] `harvest_sprites` return Vec is `BTreeMap.into_iter().collect()` with NO post-collect reordering — no `.sort` / `.sort_by` / `.sort_by_key` / `.sort_unstable*` / `.reverse` call appears between the BTreeMap collection and the function return.
- [ ] Stricter than `harvest_face_ids` because `phf_codegen::Map::build()`'s output bytes depend on `.entry()` call order; a tautological re-sort normalizes a pattern future contributors may extend to a non-tautological form.
- [ ] Sub-module helpers under `compiler/src/harvester/**/*.rs` carry visibility from the triple {`mod`-private, `pub(super)`, `pub` within the sub-module}. `pub(crate)` is NOT permitted. Hand-verify with `rg -nE "^pub\(crate\) (fn|struct|enum|const|static|trait|use) " compiler/src/harvester/` returning zero hits.
- [ ] Pure-core types limited to primitives + module-local types defined inside `compiler/src/harvester/**/*.rs`. Internal aggregation uses `BTreeMap` not `HashMap`.
- [ ] `phf_codegen::Map::build()` invocation stays in `compiler/build.rs`; no `phf_codegen::*` reference in any `compiler/src/harvester/**/*.rs` file.
- [ ] `compiler/build.rs` reaches the pure core via `#[path = "src/harvester/mod.rs"] mod harvester;` declaration (NOT `use crate::harvester` or `use textmod_compiler::harvester`).
- [ ] `compiler/src/lib.rs` adds `pub mod harvester;`. No crate-root re-export of `harvest_face_ids` / `harvest_sprites` to the bare `textmod_compiler::` namespace — consumers import the longer-form `textmod_compiler::harvester::{...}`.
- [ ] `SPRITE_REGISTRY` access-path stability honored. `textmod_compiler::authoring::sprite::SPRITE_REGISTRY` is unchanged (the chunk does not edit `compiler/src/authoring/sprite.rs`; byte-identity of `sprite_registry_generated.rs` ensures registry contents are unchanged per the SHA ledger).
- [ ] Pre-vs-post SHA-256 ledger captured in the PR shows zero divergence on both files. Divergence REJECTS the chunk.
- [ ] No `compiler/src/harvester/**/*.rs` file imports `std::fs`, `std::process`, `std::env`, `std::time::*`, `std::thread::*`, `std::sync::mpsc::*`, `std::net::*`, `std::io::{Stdin, Stdout, Stderr, stdin, stdout, stderr}`, `OnceLock`, `OnceCell`, `lazy_static!`, `once_cell::*`, or `phf_codegen::*`. No `use crate::`, `use textmod_compiler::`, `crate::` path-prefix, or `extern crate ` declaration anywhere.
- [ ] No edit to `compiler/src/extractor/`, `compiler/src/builder/`, `compiler/src/ir/`, `compiler/src/xref.rs`, `compiler/src/finding.rs`, `compiler/src/audit_markers.rs`, `compiler/src/error.rs`, `compiler/src/constants.rs`, `compiler/src/util.rs`, or `compiler/src/authoring/`. The chunk's write set is three source files (`compiler/src/harvester/mod.rs` new, `compiler/build.rs` refactored, `compiler/src/lib.rs` one-line addition) plus the two new test files.
- [ ] No `audit_harvester_determinism.rs` integration test, no permanent-CI lexical-denylist test scoped to all of `compiler/src/harvester/**/*.rs`, no explicit allowlist on `compiler/build.rs`. Those are `harvester-determinism-audit`'s scope.
- [ ] SPEC §3.4 honored. `compiler/src/harvester/**/*.rs` uses no `std::fs`, no `std::process`, no wall-clock, no environment access. The library compiles cleanly to WASM.
- [ ] No introduced public IR enum (per decisions.md 2026-05-14 "Vehicle marker semantics"). The pure-core introduces no such enum.

## Out of scope

| Item | Where instead |
|---|---|
| `audit_harvester_determinism.rs` integration test (in-process double-invocation against `include_str!`-baked corpus; recurring CI determinism gate) | `harvester-determinism-audit` |
| Lexical denylist on `compiler/src/harvester/**/*.rs` as a permanent CI test enforcing the full bound denylist on every CI run | `harvester-determinism-audit` |
| Explicit allowlist on `compiler/build.rs` I/O shell (the audit-chunk artifact naming the bound `compiler/build.rs` surface as legitimate I/O scope) | `harvester-determinism-audit` |
| Recurring-CI guard scanning `compiler/build.rs` for parallel `struct SpriteId` / `struct FaceId` declarations (the no-parallel-representation invariant per CLAUDE.md / SPEC §3.7 applied to the build-script compile unit) | `harvester-determinism-audit` (extends its `compiler/build.rs` allowlist mechanism to assert zero local-struct declarations of `SpriteId` / `FaceId` in the build-script) |
| Future browser/mobile mod-builder app WASM compile of `textmod_compiler` | A future feature (out of scope per the engineering plan's Contract edges row "harvester-pure-core → future browser/mobile mod-builder app feature"; SPEC §1 vision pillar 4) |
| `compiler/src/authoring/sprite.rs` — the `SPRITE_REGISTRY.get(name)` lookup + `include!` macro call | Untouched; byte-identity of `sprite_registry_generated.rs` ensures the read-side is unaffected |
| `compiler/src/authoring/face_id.rs` — the `include!` macro call | Untouched; byte-identity of `face_id_generated.rs` ensures the read-side is unaffected |
| `working-mods/{sliceymon,pansaer,punpuns,community}.txt` content changes | No-game-balance Non-goal |
| Author-content cross-IR semantic check changes | No-author-content-rule-expansion Non-goal |
| Re-prosecution of already-shipped platform invariants beyond the two enumerated brief carve-outs | The no-platform-re-prosecution Non-goal |
| Parser / emitter behavior changes | The no-parser-emitter-changes Non-goal |
| Drift-class parser/emitter fixes (open `PIPELINE_FIDELITY` drift classes; `HiddenModifierType::Skip` placeholder) | The no-drift-class-fixes Non-goal; future `features/pipeline-fidelity/` feature |
| `InternalStateEnum` marker classification for any new pub enum in `compiler/src/harvester/` | Not applicable — this chunk introduces no pub enum in `compiler/src/harvester/` |
| Closed escape-hatch registry contents (`FaceIdValue::Unknown` enum variant; unknown-sprite escape hatch) | `audit-harness` (per decisions.md 2026-05-08 "Escape-hatch registry is closed at two entries") |
