# Documentation ↔ Invariant Audit (remaining work)

> **Status.** Chunks 1, 1.5, 2, 3, 4 landed: §F-series and plan-section citation rewrites in `compiler/src/` + `compiler/tests/`, carve-out registry scaffold + well-formedness gate (`compiler/tests/doc_invariants_carveouts.toml` + `_parses.rs`), SPEC.md sweep populated the registry (the two existing entries), CLAUDE.md sweep produced zero registry entries (no invariant-catalog patterns hit in CLAUDE.md), and CLAUDE.md's retirement-discipline rule (canonical home: CLAUDE.md `## Working principles` § "Retiring a public identifier (non-negotiable)") + source-of-truth row landed.
>
> **Going-forward enforcement.** The retirement-discipline rule (CLAUDE.md `## Working principles`) is the durable mechanism: any future retirement is a three-step commitment in the same chunk (dated retirement comment + guard test in `compiler/tests/` + same-commit doc updates). The first retirement that needs a guard test creates the doc-invariant guard suite using the existing test pattern. The carve-out registry scaffold (TOML + well-formedness gate) is parked but available — a future hand-written guard test can load it via `serde::Deserialize` if per-file pattern carve-outs are needed.
>
> **Why withdraw the planned guard-test infrastructure.** The originally-planned chunk 8 (guard test suite) and chunk 9 (PreToolUse hook) were CI scaffolding for ~10 doc-drift patterns. Most of those patterns are already covered by stronger mechanisms: Rust's compile-time enforcement protects retired identifiers (`parse_legendary`, `legendary_*`); `serde_breaking_change_on_sprite_shape` at `compiler/src/ir/mod.rs:1895` protects the §F4 `img_data`/`sprite_name` retirement; CLAUDE.md:76 + the retirement-discipline rule protect against §F-N / `§Chunk Na` doc drift. The CI scaffolding's marginal value over those mechanisms doesn't justify the engineering scope.

## Scope

**In scope:**
- **Chunk 6** — Rewrite `personas/testing.md`'s TDD-progression chapter as workflow prose (per the TDD-progression-rewrite section below); delete Rust snippets that pin parser/builder API signatures. The lesson is the workflow (red-green-refactor); API-call examples rot every time a signature changes. Independent of the doc-audit class motivation — a separate active footgun.

**Withdrawn from scope:**
- **Chunk 8 (guard test suite)** — `compiler/tests/doc_invariants.rs` + `compiler/tests/common/mod.rs` + per-invariant tests + ruling-name uniqueness tests. The retirement-discipline rule plus the existing type-system / negative-test guards already protect the patterns this would have covered; CI scaffolding's marginal value doesn't justify the engineering scope.
- **Chunk 9 (PreToolUse hook)** — `.claude/settings.json` hook surfacing registry summaries on doc edits. Has no consumer once chunk 8 is withdrawn.
- **Chunks 5, 7a–7g** — `personas/{architecture,backend,frontend,code-reviewer,ai-development,slice-and-dice-design,security,README}.md` audit sweeps. Cumulative persona sweeps already shipped on `main` covered these surfaces; CLAUDE.md's retirement-discipline rule bounds future persona drift.
- **Chunk 10 (`///` mop-up)** — Subsumed by the retirement-discipline rule's third step (same-commit doc updates).

**Tradeoff acknowledged.** Without mechanical CI guards, doc drift surfaces only on tribunal review or when a future contributor reads an out-of-date claim. The retirement-discipline rule is the structural backstop; review is the human one.

**Out of scope:**
- Plan files under `plans/` — explicit CLAUDE.md carve-out.
- `archive/pre-guide/`, `HANDOFF.md`.
- The global `~/.claude/skills/review-pr` skill — project-local enforcement only.

## Pre-conditions

- `cargo test` clean on `main`.

## Method

### testing.md TDD-progression rewrite (chunk 6 content)

The testing persona's TDD chapter teaches a **workflow**, not API shapes. Rewrite the chapter as workflow prose; delete Rust snippets that pin signatures.

**Pattern**:
- Keep prose like *"Write a failing test that constructs a sample modifier line, calls the hero parser, and asserts the resulting hero has the expected name. Watch it fail. Implement the parser until it passes. Refactor."*
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
| Phase 4 — Character Selection / Ditto / ReplicaItems / Monsters | 138-176 | `extract(&sliceymon_text()).unwrap()` (143, 155); `extract(&text).unwrap()` (167); `build_charselect(&ir.heroes)` (144); `build_ditto(&ir.heroes)` (156); `build(&ir1).unwrap()` (168); `assert_eq!(a.target_name, b.target_name)` (172); `a.trigger.dice_faces()` (173) |
| Phase 5 — Full Round-Trip | 178-198 | Four `roundtrip_*` test bodies: `extract(&original).unwrap()` / `build(&ir_a).unwrap()` / `assert_ir_equal(&ir_a, &ir_b)` (184-187, 191-197) |
| `assert_ir_equal` description | 200-205 | Bullet list pinning `replica_items` field name + tier ops |

**Rewrite shape** (per Phase): replace the fenced ```rust block with a single prose paragraph that describes the *workflow* (what failing test to write, what to assert at the abstract level, what shape passing implies) without citing function names, struct field names, or method signatures. A reader new to the codebase still understands the discipline; a reader who wants the API navigates to `compiler/src/lib.rs` and `compiler/src/ir/mod.rs`.

The "Test Design Principles" rust-fenced examples that pin the hallucinated `hero.tiers.len() == 5` universal are also rewritten as prose, and the false "5 tiers per hero" row in the Key Invariants table is dropped.

## Chunks

### Chunk 6: personas/testing.md TDD-progression rewrite

**Scope.** Rewrite the TDD-progression chapter per the TDD-progression-rewrite section above as workflow prose. Delete API-pinning Rust snippets per that section's affected-lines table — every row of the table, including the closing `assert_ir_equal` description, loses its API-pinning content.

**Files.** `personas/testing.md`.

**Dependencies.** None.

**Consumer.** None downstream within this PR. The acceptance gate (the chunk-6 TDD-rewrite gate, defined once in the Acceptance criteria section) verifies the rewrite landed: zero ` ```rust ` fence opens in the TDD-progression chapter (the lines between the `## TDD Progression` heading and the `## Test Design Principles` heading in `personas/testing.md`, verified by Read).

**Requirements.**
- Each Phase (1-5) replaces its rust-fenced code block with workflow prose: what failing test to write, what shape passing implies, what diagnostic message a failure surfaces.
- Workflow prose names no parser/builder function, IR field, or method signature; readers seeking the API navigate to `compiler/src/lib.rs` and `compiler/src/ir/mod.rs`.
- The `assert_ir_equal` description (final row of the TDD-progression-rewrite section's affected-lines table) loses its API-pinning bullet list in the same pass.
- The "Test Design Principles" rust-fenced examples that pin the hallucinated `hero.tiers.len() == 5` universal are rewritten as prose, and the false "5 tiers per hero" row in the Key Invariants table is dropped.
- Code samples remain only where teaching a Rust-specific pattern prose can't carry.

**Dogfood.**
- The chunk-6 TDD-rewrite gate (defined in the Acceptance criteria section) holds: Read `personas/testing.md`, locate the lines between the `## TDD Progression` heading and the `## Test Design Principles` heading, count occurrences of ` ```rust ` fence opens; the count must be 0.
- Skim the rewritten chapter — workflow prose still teaches red-green-refactor without naming any function/field/method signature.

**Verification.**
- [ ] Zero ```rust fences in TDD-progression chapter.
- [ ] Code samples remain only where teaching a Rust-specific pattern prose can't carry.
- [ ] No registry append needed (chapter is rewritten, not carved out).

## Acceptance criteria

### Chunk-6 TDD-rewrite gate

Read `personas/testing.md`. Locate the lines between the `## TDD Progression` heading and the `## Test Design Principles` heading. Count occurrences of ` ```rust ` fence opens within that range; the count must be 0. Code samples elsewhere in the file remain.

Awk-based section extraction (`awk '/^## …/,/^## …/' file | grep -c …`) is a globally banned form per `~/.claude/CLAUDE.md`; use Read for the verification, not awk.

### End-state gate

`cargo test` (full suite) passes — no regressions vs. pre-chunk-6 baseline.

## Anti-patterns explicitly forbidden

- **No "fixed in this PR, follow-up for the rest".** Chunk 6's TDD rewrite deletes API-pinning snippets in one pass — every Phase, every row of the affected-lines table, in the same chunk.
- **No source-code behavior changes.** This PR is doc only. If a fix surfaces a real code defect, file a separate issue and continue.
