# Documentation ↔ Invariant Audit (remaining work)

> **Status.** Chunks 1, 1.5, 2, 3, 4 landed: §F-series and plan-section citation rewrites in `compiler/src/` + `compiler/tests/`, carve-out registry scaffold + well-formedness gate (`compiler/tests/doc_invariants_carveouts.toml` + `_parses.rs`), SPEC.md sweep populated the registry (the two existing entries), CLAUDE.md sweep produced zero registry entries (no invariant-catalog patterns hit in CLAUDE.md), and CLAUDE.md's retirement-discipline rule (canonical home: CLAUDE.md `## Working principles` § "Retiring a public identifier (non-negotiable)") + source-of-truth row landed.
>
> **Out of scope.** The eight additional persona-file audit sweeps and the `///` doc-comment mop-up are not in scope. Rationale at the `Withdrawn from scope` subsection below.
>
> **Why ship the guard tests.** They are the structural lock-in. Without them, the previously-landed sweeps are a one-time scrub — the class re-opens the next time an identifier retires without sweeping docs. With them, future drift fails CI in the chunk that introduced it, not three tribunal rounds later. That changes the long-run cost curve.
>
> **Authority rule.** Every retired identifier or shape this plan declares "stale" must be anchored by a verbatim quote from a current code-side invariant (a negative-test guard, a retirement comment dated to a specific commit, a `compile_error!`, a SPEC §F-series ruling, or a retired enum variant gated by `#[deprecated]`). Greps must be reproducible in CI; the plan's success criterion is that every grep used to find a doc violation can also be expressed as a CI guard that fails if the violation re-appears.

## Scope

**In scope:**
- **Chunk 6** — Rewrite `personas/testing.md`'s TDD-progression chapter as workflow prose (per the TDD-progression-rewrite section below); delete Rust snippets that pin parser/builder API signatures. The lesson is the workflow (red-green-refactor); API-call examples rot every time a signature changes. This chunk's rationale is independent of the doc-audit class motivation — it's a separate active footgun.
- **Chunk 8** — Land `compiler/tests/doc_invariants.rs` + `compiler/tests/common/mod.rs`. One test per invariant-catalog row + one ruling-name uniqueness test per ruling-name-table row. **Test surface is `SPEC.md`, `CLAUDE.md`, and `compiler/src/`** (the helper's extension parameter `["rs", "md"]` per the search-primitive subsection owns what files match under `compiler/src/`) — not the persona files; rationale at the `Withdrawn from scope` subsection below.
- **Chunk 9** — Add the layer-3 PreToolUse hook (per the enforcement-layers subsection) on `Edit`/`Write` for `personas/*.md`, `SPEC.md`, `CLAUDE.md` (HANDOFF.md excluded per ephemeral-file exclusion). Surfaces a summary (count + one-line index + pointer) of the carve-out registry; never the full TOML.

**Withdrawn from scope:**
- **Chunks 5, 7a–7g** — `personas/{architecture,backend,frontend,code-reviewer,ai-development,slice-and-dice-design,security,README}.md` audit sweeps. The cumulative persona sweeps already shipped on `main` covered these surfaces hard, so the marginal find rate over them is low. The chunk 8 cross-tree grep targets `SPEC.md` + `CLAUDE.md` + `compiler/src/`, where retirements actually happen — persona drift not anchored to a `compiler/src/` retirement is rare. CLAUDE.md's retirement-discipline rule (canonical home: CLAUDE.md `## Working principles` § "Retiring a public identifier (non-negotiable)", landed in chunk 4) makes future retirements a multi-step commitment that includes doc updates, so new persona drift is structurally bounded going forward.
- **Chunk 10 (`///` mop-up)** — Subsumed by chunk 8's cross-tree grep. The guard tests fire on `compiler/src/**/*.rs` `///` comments alongside markdown; any drift surfaces as a CI failure in the chunk that retires the identifier, not a separate manual sweep.

**Tradeoff acknowledged.** A stale claim in `personas/{architecture,backend,frontend,code-reviewer,ai-development,slice-and-dice-design,security,README}.md` could slip into Claude's suggestions on a future task and cost a tribunal round before chunk 8 catches it on a code-side retirement. That cost is real but bounded; lower than the cost of auditing eight more persona files preemptively when the audit grep + carve-out registry already exist as reactive tools.

**Out of scope (unchanged from original plan):**
- Bucket-label asymmetry between xref's `"thief"` and emitter's `"Thief"` — tracked separately.
- Plan files under `plans/` — explicit CLAUDE.md carve-out.
- `archive/pre-guide/`, `HANDOFF.md`.
- The global `~/.claude/skills/review-pr` skill — project-local enforcement only.

## Pre-conditions

- Chunks 1, 1.5, 2, 3, 4 landed on `main`.
- `cargo test` clean on `main`.
- `compiler/tests/doc_invariants_carveouts.toml` exists with entries from the SPEC.md and CLAUDE.md sweeps; `compiler/tests/doc_invariants_carveouts_parses.rs` (well-formedness gate) green.
- `toml = "0.8"` already in `compiler/Cargo.toml` `[dev-dependencies]` per the well-formedness-gate justification.

## Method

### testing.md TDD-progression rewrite (chunk 6 content)

The testing persona's TDD chapter teaches a **workflow**, not API shapes. Rewrite the chapter as workflow prose; delete Rust snippets that pin signatures.

**Pattern**:
- Keep prose like *"Write a failing test that constructs a sample modifier line, calls `parse_hero`, and asserts the resulting `Hero` has the expected `name` field. Watch it fail. Implement the parser until it passes. Refactor."*
- Delete code blocks like:
  ```rust
  let hero = parse_hero(SAMPLE_HERO_LINE).unwrap();
  assert_eq!(hero.name, "Charmander");
  ```
- Code samples remain ONLY where they teach a Rust-specific pattern that prose can't carry (`if let Some(x) = ...`, `?` operator usage, etc.). API-call examples go.

**Affected lines** (verified by Read on `personas/testing.md` lines 1-205):

The principle "API-call examples go" applies to ALL five Phases.

| Phase | Lines | API-pinning content to retire |
|-------|-------|------------------------------|
| Phase 1 — Modifier Classification | 31-51 | `classify(line)` calls + `ModifierType::Hero` / `ModifierType::Unknown` enum-variant pins (lines 37, 45-48) |
| Phase 2 — Hero Parser | 53-90 | `parse_hero(SAMPLE_HERO_LINE).unwrap()` calls (58, 66); `hero.tiers[i].hp` / `hero.tiers[i].name` / `hero.mn_name` / `hero.tiers.len()` field pins (59-61, 67-69, 77-87) |
| Phase 3 — Builder / Emitter | 92-136 | `parse_hero(SAMPLE_HERO_LINE).unwrap()` (97, 107, 124); `build_hero(&hero).unwrap()` (98, 108, 125); `output.char_indices()` / `.rfind(".n.")` pattern pins |
| Phase 4 — Character Selection / Ditto / ReplicaItems / Monsters | 138-176 | `extract(&sliceymon_text()).unwrap()` (143, 155); `extract(&text).unwrap()` (167); `build_charselect(&ir.heroes)` (144); `build_ditto(&ir.heroes)` (156); `build(&ir1).unwrap()` (168); `assert_eq!(a.target_name, b.target_name)` (172); `a.trigger.dice_faces()` (173) — these pin the post-Chunk-9 `ReplicaItem.target_name` rename and the `SummonTrigger::dice_faces()` accessor signature |
| Phase 5 — Full Round-Trip | 178-198 | Four `roundtrip_*` test bodies: `extract(&original).unwrap()` / `build(&ir_a).unwrap()` / `assert_ir_equal(&ir_a, &ir_b)` (184-187, 191-197) |
| `assert_ir_equal` description | 200-205 | Bullet list pinning `replica_items` field name + tier ops |

**Rewrite shape** (per Phase): replace the fenced ```rust block with a single prose paragraph that describes the *workflow* (what failing test to write, what to assert at the abstract level, what shape passing implies) without citing function names, struct field names, or method signatures. A reader new to the codebase still understands the discipline; a reader who wants the API navigates to `compiler/src/lib.rs` and `compiler/src/ir/mod.rs`.

The carve-out registry does not carry these as carve-outs after the rewrite — they're deleted, not preserved.

### Invariant catalog (chunk 8 input)

Chunk 8's per-invariant guard tests are derived from this catalog. One test per row, asserting zero hits across the re-scoped doc surface (`SPEC.md` + `CLAUDE.md` + `compiler/src/`, with `recursive_grep`'s extension parameter `["rs", "md"]` per the search-primitive subsection owning what files match under `compiler/src/`) modulo carve-outs from the registry.

| Class | Protecting invariant (code side) | Doc-violation grep |
|-------|----------------------------------|--------------------|
| Capture / Legendary IR vocabulary as kind-discriminator | `compiler/src/ir/mod.rs` ReplicaItem shape; `ModifierType` (`compiler/src/extractor/classifier.rs`) has no `Capture`/`Legendary` variant | `rg -in '\b(Legendary\|Capture)\b'` (excluding registered carve-outs) |
| `parse_legendary` / `legendary_*` / `parse_simple` | The exact identifiers `parse_legendary`, any `legendary_<word>`, and `parse_simple` (with word-boundary closure on both sides) do not exist as defined symbols in `compiler/src/`; Rust's compile-time enforcement prevents silent reintroduction. The unrelated `parse_simple_choice` at `compiler/src/extractor/phase_parser.rs:141` falls outside the grep's word-boundary closure (`_` is a word character, so `\bparse_simple\b` does not match `parse_simple_choice`) and is not in scope | `rg -in '\bparse_legendary\b\|\blegendary_\w+\b\|\bparse_simple\b'` |
| Phantom files (`validator.rs`, top-level `sprite.rs`, `capture_*.rs`) | `ls` confirms non-existence; real surface is `xref.rs` + `authoring/sprite.rs` + `replica_item_*` | `rg -in 'compiler/src/(validator\.rs\|sprite\.rs\|capture_)'` |
| `Validator` as pipeline stage / `textmod-compiler validate` CLI | `compiler/src/main.rs:15-51` Subcommand list; CLAUDE.md:12 negation; round-trip lives on `Check` | `rg -in 'textmod-compiler (validate\|verify\|run)\b'` |
| External `sprites.json` / `SpriteMap` / `build(ir, sprites)` 2-arg signature | `compiler/src/lib.rs:26` `pub fn build(ir: &ModIR) -> Result<String, CompilerError>` (single arg); no `SpriteMap` | `rg -in 'build\(ir, sprites\|build_textmod\(.*sprites\|fn build.*sprites:'` |
| Phantom WASM API `validate(input)` | No `pub fn validate` on `lib.rs`; semantic checks are `xref::check_references` | `rg -in 'fn validate\(\|validate_textmod\b'` |
| Retired pre-§F4 top-level `img_data` / `sprite_name` fields | `compiler/src/ir/mod.rs:1895` `serde_breaking_change_on_sprite_shape` negative-test guard; real shape is `sprite: SpriteId` accessed via `sprite.img_data()` | `rg -inE '\bimg_data\b\|\bsprite_name\b'` (excluding accessor pattern `sprite.img_data()` — pre-grep filter at the test grep site, not registry carve-out) |
| `xref` IS the validator — no separate pass | `compiler/src/lib.rs:12` exports `xref::check_references`; CLAUDE.md:12 | `rg -in '\bvalidator pass\b\|no separate validator'` (every hit is a deliberate-negation registry carve-out) |
| §F-series source citations referencing plan-only sections | CLAUDE.md "plans go stale once executed" — source code MUST NOT cite plan-section IDs as durable references | `rg -nE '§F[0-9]+'` (zero hits expected post chunk 1; the test asserts the post-rewrite zero-hit state holds — surface is `DOC_SURFACE` per the search-primitive subsection, named once at the chunk-8 entry above, not re-listed per row) |
| `§Chunk Na` / `plan §N` / direct-plan-filename source citations | Same protected invariant as §F-N; spelling differs | `rg -nE '§Chunk [0-9]+\|plan §[0-9]+\.[0-9]+\|plan §[0-9]+\b\|parent plan §[0-9]+\|PLATFORM_FOUNDATIONS_PLAN\.md\|AUTHORING_ERGONOMICS_PLAN\.md'` (zero hits expected post chunk 1.5 — surface is `DOC_SURFACE` per the search-primitive subsection, same as the §F-N row above) |

### Ruling-name table (chunk 8 input)

Chunk 8 asserts each ruling-name string appears nowhere in the doc surface that doesn't relate to its protected invariant — date alone repeats across rulings, so the name is the load-bearing identifier.

| §F-N | Protected invariant | Authoring date | Canonical datestamped name |
|------|---------------------|----------------|----------------------------|
| §F3  | Permissive Face IDs + `Pips` newtype — `FaceIdValue::{Known, Unknown}`, extraction succeeds with `Severity::Warning` | 2026-04-20 | `2026-04-20 "permissive face IDs" ruling` |
| §F4  | Sprite shape consolidation — `sprite: SpriteId`, no legacy back-compat, no serde shim | 2026-04-20 | `2026-04-20 "no legacy back-compat" ruling on sprite shape` |
| §F5  | `BuildOptions { include: SourceFilter }` + `build_with` + provenance-aware `Finding.source` severity promotion | 2026-04-22 | `2026-04-22 "BuildOptions + provenance-aware findings" ruling` |
| §F8  | No `unwrap`/`expect`/`panic!`/`unimplemented!`/`todo!` in `compiler/src/**/*.rs` outside `#[cfg(test)]`; enforced by `audit_lib_panic_free.rs` | 2026-04-22 | `2026-04-22 "library code panic-free" ruling` |

### Carve-out registry (already exists)

`compiler/tests/doc_invariants_carveouts.toml` was populated by chunk 3's SPEC.md sweep (the two existing entries); chunk 4's CLAUDE.md sweep produced zero registry entries. The well-formedness gate at `compiler/tests/doc_invariants_carveouts_parses.rs` enforces structural-and-addressing contracts (per-file `pattern`-uniqueness keyed on canonical path).

Chunk 8's `filter_carveouts` deserializes the registry via `serde::Deserialize` and treats it as known-sound (the gate runs first; CI ordering guarantees gate-asserted invariants hold before chunk 8 runs).

The registry is **not extended** by the work in this plan — the persona-file sweeps and `///` mop-up are withdrawn, so no additional persona-side carve-outs append. If chunk 6's TDD rewrite leaves a residual hit (it shouldn't — the rewrite deletes API-pinning lines, not preserves them), surface it to the user before adding a carve-out.

## Chunks

Each chunk uses the persona-required template: Scope / Files / Dependencies / Consumer / Requirements / Dogfood / Verification.

---

### Chunk 6: personas/testing.md TDD-progression rewrite

**Scope.** Rewrite the TDD-progression chapter per the TDD-progression-rewrite section above as workflow prose. Delete API-pinning Rust snippets per that section's affected-lines table — every row of the table, including the closing `assert_ir_equal` description, loses its API-pinning content.

**Files.** `personas/testing.md`.

**Dependencies.** None (independent of chunk 8).

**Consumer.** None downstream within this PR. The acceptance gate (the chunk-6 TDD-rewrite gate, defined once in the Acceptance criteria section) verifies the rewrite landed: zero ` ```rust ` fence opens in the TDD-progression chapter (the lines between the `## TDD Progression` heading and the `## Test Design Principles` heading in `personas/testing.md`, verified by Read).

**Requirements.**
- Each Phase (1-5) replaces its rust-fenced code block with workflow prose: what failing test to write, what shape passing implies, what diagnostic message a failure surfaces.
- Workflow prose names no parser/builder function, IR field, or method signature; readers seeking the API navigate to `compiler/src/lib.rs` and `compiler/src/ir/mod.rs`.
- The `assert_ir_equal` description (final row of the TDD-progression-rewrite section's affected-lines table) loses its API-pinning bullet list in the same pass.
- The "Test Design Principles" rust-fenced examples that pin the hallucinated `hero.tiers.len() == 5` universal are rewritten as prose, and the false "5 tiers per hero" row in the Key Invariants table is dropped — extended scope per user decision in PR #19 invocation 4.
- Code samples remain only where teaching a Rust-specific pattern prose can't carry.

**Dogfood.**
- The chunk-6 TDD-rewrite gate (defined in the Acceptance criteria section) holds: Read `personas/testing.md`, locate the lines between the `## TDD Progression` heading and the `## Test Design Principles` heading, count occurrences of ` ```rust ` fence opens; the count must be 0. (Awk-based section extraction is a globally banned form per user's `~/.claude/CLAUDE.md`.)
- Skim the rewritten chapter — workflow prose still teaches red-green-refactor without naming any function/field/method signature.

**Verification.**
- [ ] Zero ```rust fences in TDD-progression chapter.
- [ ] Code samples remain only where teaching a Rust-specific pattern prose can't carry.
- [ ] No registry append needed (chapter is rewritten, not carved out).

---

### Chunk 8: Guard tests (re-scoped surface)

**Scope.** Land `compiler/tests/doc_invariants.rs` + `compiler/tests/common/mod.rs`. One test per invariant-catalog row; one ruling-name uniqueness test per ruling-name-table row. The test surface is `SPEC.md`, `CLAUDE.md`, and `compiler/src/` (extensions filtered to `["rs", "md"]` by `recursive_grep` per the search-primitive subsection) — **personas excluded** per the `Withdrawn from scope` subsection. The `recursive_grep` helper takes parameterized roots, so a future scope expansion to `personas/` is a one-line change in `DOC_SURFACE`.

**Files.** `compiler/tests/doc_invariants.rs` (new), `compiler/tests/common/mod.rs` (new).

**Dependencies.** Chunks 2, 3, 4 (registry exists and populated for SPEC.md + CLAUDE.md surfaces). Independent of chunk 6.

**Consumer.** CI runs the new tests on every commit forever; chunk 9's hook references the registry the tests load.

**Requirements.**
- One test per invariant-catalog row, asserting zero hits across `DOC_SURFACE` modulo carve-outs from the registry.
- One ruling-name uniqueness test per ruling-name-table row, asserting the canonical datestamped name appears nowhere in the doc surface outside its protected-invariant citation site.
- `DOC_SURFACE` constant scopes to `["../SPEC.md", "../CLAUDE.md", "src"]` with extension filter `["rs", "md"]`. Site comment cites the `Withdrawn from scope` subsection so a future scope flip to personas is a one-line change.
- `recursive_grep` lives in `compiler/tests/common/mod.rs` with parameterized roots, extension list, and skip list (per the search-primitive subsection).
- `has_word_boundary_match(line, needle)` helper for substring searches that need `\b...\b` semantics.
- `filter_carveouts(hits, registry_path)` deserializes the registry via `serde::Deserialize` and drops hits whose canonicalized path matches an entry's canonicalized path AND whose line content contains the entry's `pattern`.
- Single `const CARVEOUT_REGISTRY: &str = "tests/doc_invariants_carveouts.toml";` declaration; every test loads the registry through it.
- Per-row parenthetical exclusions (e.g. `sprite.img_data()` for the retired-`img_data` row) encoded as pre-grep filters at the test grep call site, NOT as registry carve-outs.

**Dogfood.**
- `~/.cargo/bin/cargo test --test doc_invariants` passes — every per-invariant test green, every ruling-name uniqueness test green.
- `~/.cargo/bin/cargo test` full suite passes — no regressions vs. pre-chunk baseline.

**Verification.**
- [ ] One test per row in the invariant-catalog table.
- [ ] One ruling-name uniqueness test per row of the ruling-name table.
- [ ] `DOC_SURFACE` constant scopes to `["../SPEC.md", "../CLAUDE.md", "src"]` — personas excluded per the `Withdrawn from scope` subsection; the comment at the constant's site cites that subsection so a future reader who runs the same audit on personas can flip the scope by extending the array.
- [ ] Single `const CARVEOUT_REGISTRY: &str = "tests/doc_invariants_carveouts.toml";` declaration; all tests load the registry through it.
- [ ] `recursive_grep` lives in `compiler/tests/common/mod.rs` with extension-list and skip-list parameters per the search-primitive subsection.

**Critical checkpoint after 8.** Full test suite green; registry-driven enforcement is the audit's load-bearing artifact going forward. The structural fix lands here — without it, the work in chunks 1–4 is one-time scrub.

---

### Chunk 9: `.claude/settings.json` PreToolUse hook

**Scope.** Add the layer-3 PreToolUse hook (per the enforcement-layers subsection) on `Edit`/`Write` for `personas/*.md`, `SPEC.md`, `CLAUDE.md` (HANDOFF.md excluded). Hook surfaces a **summary** (count + one-line index per entry + pointer to TOML) of the carve-out registry. Follows the same `jq -n '{hookSpecificOutput:{hookEventName:"PreToolUse",additionalContext:"..."}}'` shape as the existing Evidence-rule hook.

**Files.** `.claude/settings.json`.

**Dependencies.** Chunks 2 + 8 (registry exists; tests reference it).

**Consumer.** Future doc-edit operations in next sessions; not directly testable in this PR.

**Requirements.**
- Hook matcher covers `personas/*.md`, `SPEC.md`, `CLAUDE.md`; excludes `HANDOFF.md` (ephemeral file exclusion).
- Hook output is a registry summary: count of carve-outs + one-line index per entry (`<path> — <pattern> — <one-line rationale>`) + pointer to `compiler/tests/doc_invariants_carveouts.toml`.
- Hook does NOT paste the full registry contents (context-bound discipline).
- Hook follows the same `jq -n '{hookSpecificOutput:{hookEventName:"PreToolUse",additionalContext:"..."}}'` shape as the existing Evidence-rule hook.
- Configured in `.claude/settings.json` (project-local; no global skill changes).

**Dogfood.**
- `jq . .claude/settings.json` succeeds.
- The new hook entry mirrors the existing Evidence-rule hook's shape verbatim except for matcher and command body.
- Hand-execute the hook command against the current registry; output is a count + one-line index, **not** the full registry contents.

**Verification.**
- [ ] `.claude/settings.json` parses as valid JSON.
- [ ] Hook matcher covers `personas/*.md`, `SPEC.md`, `CLAUDE.md`; excludes `HANDOFF.md`.
- [ ] Hook output is a summary, not the full registry.

**Critical checkpoint after 9.** Hook only verifiable in a future session — confirm with user the shape is right before merging.

---

## Implementation discipline (chunk 8)

### Search primitive and helpers

`recursive_grep` lives in `compiler/tests/common/mod.rs`. Three properties for cross-extension use:

- **Path resolution.** Uses `crate_dir() = PathBuf::from(env!("CARGO_MANIFEST_DIR"))` to anchor relative paths — so `Path::new("../SPEC.md")` resolves to `<repo_root>/SPEC.md` regardless of test-run CWD. More robust than depending on Cargo's CWD behavior.
- **Extension filter is parameterized.** Takes an extension list: `recursive_grep(root, pattern, &["rs", "md"])` or `&["md"]` for doc-only sweeps. The helper must NOT hardcode `.rs` — grepping `.md` files is a first-class use case.
- **Self-skip parameterized.** Takes a skip-list of basenames so a test file containing the retirement-pattern string literals doesn't self-match: `recursive_grep(root, pattern, &["rs", "md"], &["doc_invariants.rs"])`.

**Word-boundary handling.** Substring search can't express `\b...\b` directly. For invariants where word boundaries matter (e.g., distinguishing `img_data` the retired field from `sprite.img_data()` the current accessor), implement `has_word_boundary_match(line: &str, needle: &str) -> bool` using `char::is_alphanumeric` (or `_`) checks on the byte before/after the substring match.

**Pre-grep parenthetical exclusion.** Each invariant-catalog row's parenthetical exclusion clause (e.g. row "Retired pre-§F4 top-level fields"'s "excluding the accessor pattern `sprite.img_data()`") MUST be encoded as a pre-grep filter at the test's grep call site, NOT as a registry carve-out. Pre-exclusion is part of the grep specification — the row's grep was written to mean "the regex AND NOT the parenthetical". The registry carves out hits that ARE in the audit set but have a stable rationale; pre-exclusion removes hits the audit specification never considered violations.

**Carve-out filter.** `filter_carveouts(hits, registry_path)` deserializes the TOML registry into `Vec<CarveOut>` via `serde::Deserialize`, then drops any `Hit` whose canonicalized path matches an entry's canonicalized path AND whose `hit.line` contains the entry's `pattern`. Both sides canonicalize before comparison so spelling variants collapse to one identity. The filter keys on line content containing the pattern — per-file `pattern` uniqueness (asserted by the well-formedness gate at registry-load time) is what makes per-line carve-outs unambiguous. The registry path is `const CARVEOUT_REGISTRY: &str = "tests/doc_invariants_carveouts.toml";` declared once at the top of `doc_invariants.rs` and resolved via `crate_dir().join(CARVEOUT_REGISTRY)`.

**Example test:**

```rust
mod common;
use common::{recursive_grep, has_word_boundary_match, filter_carveouts, Hit};
use std::path::Path;

const CARVEOUT_REGISTRY: &str = "tests/doc_invariants_carveouts.toml";

// Persona files excluded per the plan's `Withdrawn from scope` subsection —
// extend this array to add coverage if a future audit revisits the
// persona-file sweeps.
const DOC_SURFACE: &[&str] = &[
    "../SPEC.md",
    "../CLAUDE.md",
    "src",  // CWD is compiler/, so src/ == compiler/src/
];

#[test]
fn no_doc_references_to_retired_img_data_field() {
    let mut hits: Vec<Hit> = Vec::new();
    for needle in &["img_data", "sprite_name"] {
        for root in DOC_SURFACE {
            for hit in recursive_grep(Path::new(root), needle, &["rs", "md"], &["doc_invariants.rs"]) {
                // Pre-grep exclusion: the row's grep means
                // "(\bimg_data\b|\bsprite_name\b) AND NOT sprite.img_data()".
                // The error-payload `sprite_name: ...` field on
                // `ErrorKind::SpriteNotFound` is a registry carve-out
                // (filtered after this pre-exclusion).
                if hit.line.contains("sprite.img_data()") {
                    continue;
                }
                if has_word_boundary_match(&hit.line, needle) {
                    hits.push(hit);
                }
            }
        }
    }
    let violations = filter_carveouts(hits, CARVEOUT_REGISTRY);
    assert!(
        violations.is_empty(),
        "Doc reference to the retired top-level img_data/sprite_name field. \
         Real shape: sprite: SpriteId per compiler/src/ir/mod.rs:574 \
         (accessed via sprite.img_data()). Negative-test guard at :1895 \
         requires legacy JSON to fail deserialization.\n\n\
         Violations:\n{:#?}",
        violations,
    );
}
```

One such test per row of the invariant catalog. The test catalog is the audit's load-bearing artifact.

**Ruling-name uniqueness guard.** Per the ruling-name-table section above, the canonical datestamped name (e.g., `"no legacy back-compat" ruling on sprite shape`) is the load-bearing identifier. Add one guard test per ruling-name string asserting it appears nowhere in the doc surface (`SPEC.md` + `CLAUDE.md` + `compiler/src/`, extensions filtered to `["rs", "md"]` per this subsection's helper) outside of citations of its protected invariant (the carve-out registry whitelists legitimate citations). Date alone repeats across rulings (multiple decisions landed 2026-04-20 and 2026-04-22), so the name is what makes the handle stable.

### Enforcement layers (additive)

Three layers, all project-local (no global skill changes):

1. **CI guard tests** (chunk 8) — strongest layer; runs on every commit. Loads the carve-out registry as a fixture.
2. **CLAUDE.md source-of-truth row** — landed in chunk 4. Anyone reading CLAUDE.md before doing review work sees the registry exists.
3. **PreToolUse hook on `Edit`/`Write`** (chunk 9) — when an agent attempts a doc edit, the hook surfaces a **summary** of the carve-out registry into the conversation: count of carve-outs, one-line index per entry (`<path> — <pattern> — <one-line rationale>`, where `<path>` and `<pattern>` use the scaffold's field spelling and uniquely locate the carved-out site by construction), and a pointer to the full TOML at `compiler/tests/doc_invariants_carveouts.toml`. **Do not paste the full registry** — context-bound discipline. Configure in `.claude/settings.json` (project-local). The hook follows the same `jq -n '{hookSpecificOutput:{hookEventName:"PreToolUse",additionalContext:"..."}}'` shape as the existing Evidence-rule hook.

The `/review-pr` skill is not updated. Project-local enforcement only.

### Retirement-discipline rule (canonical home: CLAUDE.md)

The rule body lives at `CLAUDE.md` `## Working principles` § "Retiring a public identifier (non-negotiable)" — that is the canonical site, named once. The plan does not restate it here; chunk 8's guard tests and chunk 9's hook are the in-test and in-tooling implementations of that rule. Together they mean a future identifier retirement either ships clean or fails CI immediately, with no "next round of tribunal will catch it" failure mode.

## Acceptance criteria

Per-chunk acceptance is owned by each chunk's `Verification` checklist in the Chunks section above (the per-chunk template's gate). This section names (a) the cross-cutting end-state gates that must hold once chunks 6, 8, and 9 have all landed, and (b) the chunk-6 TDD-rewrite gate definition referenced from chunk 6's Consumer and Dogfood entries.

### End-state gates (after chunks 6, 8, 9 all land)

- `cargo test --test doc_invariants` passes — every invariant-catalog test green, every ruling-name uniqueness test green.
- `cargo test` (full suite) passes — no regressions vs. pre-chunk-8 baseline.
- `cargo run --example roundtrip_diag` passes (4× ROUNDTRIP OK) — proves the audit didn't accidentally touch behavior.

### Chunk-6 TDD-rewrite gate (referenced from chunk 6)

Read `personas/testing.md`. Locate the lines between the `## TDD Progression` heading and the `## Test Design Principles` heading. Count occurrences of ` ```rust ` fence opens within that range; the count must be 0. Code samples elsewhere in the file remain.

Awk-based section extraction (`awk '/^## …/,/^## …/' file | grep -c …`) is a globally banned form per `~/.claude/CLAUDE.md`; use Read for the verification, not awk.

## Anti-patterns explicitly forbidden

- **No reactive grep widening.** If a violation surfaces in chunk 8 that isn't covered by the invariant catalog, the test catalog is incomplete — extend the invariant catalog and add a new test, don't add a one-off carve-out to silence the test.
- **No "fixed in this PR, follow-up for the rest".** Chunk 6's TDD rewrite deletes API-pinning snippets in one pass. Chunk 8's guard test catalog covers every invariant-catalog row in one pass.
- **No "the test passes so it's fine"** as a closing argument. The invariant catalog + ruling-name table is what proves the audit was complete. The guard tests close the class going forward; they do not retroactively prove the audit caught everything.
- **No source-code behavior changes.** This PR is doc + tests only. If a fix surfaces a real code defect, file a separate issue and continue.
- **No re-scoping `DOC_SURFACE` to include personas mid-chunk.** If a future audit decides the persona-file sweeps are worth running, that's a separate PR with its own per-persona carve-out decisions. Adding personas to `DOC_SURFACE` without sweeping them first will fail the chunk 8 tests on landing.
