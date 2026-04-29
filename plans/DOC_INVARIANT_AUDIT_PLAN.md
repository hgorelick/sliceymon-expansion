# Documentation ↔ Invariant Audit

> **Why this plan exists.** PR #14 (chunk-8A) generated 23 tribunal rounds; rounds 5/8/10/12/14/16/18/20/22 each overturned a "verdict CLEAN" by widening the grep to a sibling-class hit the prior round had not enumerated. The work is real — every finding traced to a documentation claim that contradicted a current code invariant — but the loop pattern is the failure mode: each round scoped its grep to the file the previous round happened to fix, never to the *invariant being protected*. This plan replaces that pattern with a single invariant-driven audit pass + CI guards that prevent the class from re-opening.
>
> **Scope boundary (declared up front).** This is a **documentation-only** PR. Zero source-code behavior changes. No IR shape changes, no builder/extractor logic changes, no test logic changes (only added guard tests). The 67-hit cumulative-class table from PR #14 round-23 is the seed; the plan extends it by deriving the grep from the invariant rather than the reverse, so any siblings the table missed will surface in the audit pass below — not in a future round-N.
>
> **Authority rule.** Every retired identifier or shape this plan declares "stale" must be anchored by a verbatim quote from a current code-side invariant (a negative-test guard, a retirement comment dated to a specific commit, a `compile_error!`, a SPEC §F-series ruling, or a retired enum variant gated by `#[deprecated]`). "I remember it was retired" is not authority. Greps must be reproducible in CI; the plan's success criterion is that every grep used to find a doc violation can also be expressed as a CI guard that fails if the violation re-appears.

## 1. Scope

Rewrite or retire every live-doc reference (`*.md`, prose comments in source) that contradicts a current code-side invariant in `compiler/`. "Live doc" means: SPEC.md, CLAUDE.md, every file under `personas/`, and inline doc-comments on `pub` items. Excludes `archive/`, `plans/`, baseline fixtures, **`HANDOFF.md` (ephemeral local handoff file — gitignored at `.gitignore:13`, untracked per `git ls-files HANDOFF.md` returning empty in this session; should be removed from the working tree after use, never persisted as audit material)**, and intentional historical-contrast notes (with carve-out rationale).

**In scope** — the cumulative class set surfaced rounds 19-23, *normalized to the protecting invariant*:

| Class | Protecting invariant (code side) | Round of discovery |
|-------|----------------------------------|--------------------|
| Capture / Legendary IR vocabulary as kind-discriminator | `compiler/src/ir/mod.rs` ReplicaItem shape; no `Capture`/`Legendary` enum variant exists; `ModifierType::Legendary` retired in chunk-8A | 18, 19 |
| `parse_legendary` / `legendary_*` test fns / `parse_simple` | Deleted in chunk-8A; `compiler/tests/retirements.rs:13` T13 guard (`grep_crate_for_parse_legendary`) | 18, 19 |
| `compiler/src/{validator,sprite,capture_emitter,capture_parser}.rs` | Phantom files — `ls` confirms non-existence; real surface is `xref.rs` + `authoring/sprite.rs` + `replica_item_*` | 19, 22 |
| `Validator` as pipeline stage / `textmod-compiler validate` CLI | `compiler/src/main.rs:15-51` Subcommand list (`Extract`, `Build`, `Check`, `Schema`, `Overlay` — no `Validate`); CLAUDE.md:12 (`Validation is **not a separate pass**`) + SPEC.md:59 negation; round-trip lives on `Check` | 20 |
| External `sprites.json` / `SpriteMap` / `build(ir, sprites)` 2-arg signature | `compiler/src/lib.rs:26` `pub fn build(ir: &ModIR) -> Result<String, CompilerError>` (single arg); no `SpriteMap` type | 20, 21, 22 |
| Phantom WASM API `validate(input)` | No `pub fn validate` on `lib.rs`; semantic checks are `xref::check_references` (`lib.rs:12`) | 22 |
| Retired pre-§F4 top-level `img_data` / `sprite_name` fields on IR types | `compiler/src/ir/mod.rs:1908-1920` `serde_breaking_change_on_sprite_shape` negative-test guard; real shape is `sprite: SpriteId` accessed via `sprite.img_data()` | 23 |
| TDD-progression `parse_hero(line).unwrap()` shorthand vs real `parse_hero(modifier, modifier_index) -> Hero` signature | `compiler/src/extractor/hero_parser.rs:7` real signature | 21 (resolved during planning — rewrite as workflow prose, see §3.4) |
| `§F<N>` source citations referencing plan-only sections (both phantom-SPEC and PLATFORM_FOUNDATIONS_PLAN refs) | Per CLAUDE.md "plans go stale once executed" — source code MUST NOT cite plan-section IDs as durable references. The protecting invariant is structural (citations must be stable handles, not coupled to a plan that will rot). Real discriminators are datestamped ruling-names per §3.3. | Round-1 audit of this plan (added 2026-04-27) |

**Also in scope** (added during planning per user decision §6):
- Replacing **every `§F<N>` source citation** in `compiler/` (i.e. `compiler/src/` for sub-commits 1a–1e plus `compiler/tests/` for sub-commit 1f, per the partition documented at §3.3 line 116 and §4.2 chunk 1's per-sub-commit scope at line 291) with datestamped-ruling-name prose (full callsite catalog + ruling-name table at §3.3). Per CLAUDE.md, plans/ are "roadmaps for in-flight work, not durable sources of truth" — therefore source code MUST NOT depend on plan-section IDs. This applies whether the cited §F-N is a phantom SPEC section (e.g. `SPEC §F4`, `SPEC §F8`) or a real PLATFORM_FOUNDATIONS_PLAN section (e.g. `PLATFORM_FOUNDATIONS_PLAN §F5`); both are equally fragile under the CLAUDE.md rule.
- Rewriting `personas/testing.md`'s TDD-progression chapter as workflow prose (red-green-refactor as a process), deleting Rust snippets that pin **any** API signature — applies to all five Phases (1-5), not just the `parse_hero` examples (full line catalog at §3.4). The lesson is the workflow, not the API call.

**Out of scope** (enumerated, not "future work"):
- Bucket-label asymmetry between xref's `"thief"` lowercase lookup key and emitter's `"Thief"` literal — chunk-8B obligation per `plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md`.
- Plan files under `plans/` — explicitly carved out by CLAUDE.md as roadmaps for in-flight work, not durable sources of truth.
- `archive/pre-guide/` — predates `reference/textmod_guide.md` (added 2026-04-10) and is not authoritative per CLAUDE.md.
- New persona files. Authoring-ergonomics persona, security persona — out of scope; the audit fixes existing files only.
- Updating the global `~/.claude/skills/review-pr` skill. The skill is shared across projects; baking project-specific carve-out paths into it would break for projects that don't have one. Review-time enforcement comes from project-local hooks (§5.5), not from the global skill.

## 2. Pre-conditions

- PR #14 (chunk-8A) merged to `main`. The audit baselines off the merged code state, not off a moving branch.
- `cargo test` clean on `main` post-merge (the negative-test guards at `compiler/src/ir/mod.rs:1908-1920` and the retirement-grep guards at `compiler/tests/retirements.rs` must pass — they are this plan's correctness anchor).

## 3. Method — invariant-driven, not finding-driven

This is the structural change versus rounds 19-23. Every prior round started from the previous round's fix-line and asked *"what other lines look like this?"*. This plan starts from the protected invariant in code and asks *"what doc lines contradict this invariant, anywhere in the repo?"*.

### 3.1 Enumerate every code-side invariant that a doc could contradict

Run **once**, save output to `plans/.doc_audit_invariants.txt`. The `.gitignore` (verified by Read in this session) does not auto-cover dotfiles; implementation must add `plans/.doc_audit_*.txt` to `.gitignore` in the same commit that produces the file, or place the file under `compiler/target/` (already gitignored). Pick at implementation start.

Each grep below is a **separate** rg invocation — joining them with `|` makes the multi-line context clause `[^}]*` apply only to the first alternative, dropping the negative-test scoping for the rest:

```bash
# Negative-test guards (assertions that a retired shape MUST FAIL).
# Multi-line context with -U requires the regex to NOT branch with | at the top
# level — otherwise [^}]* binds only to the first arm. Run two greps:
rg -nU 'fn \w+\s*\(\s*\)\s*\{[^}]*MUST NOT' compiler/src/ -g '!target' -g '!*.rs.bk'
rg -nU 'fn \w+\s*\(\s*\)\s*\{[^}]*MUST FAIL' compiler/src/ -g '!target' -g '!*.rs.bk'
# These two scan-anywhere; not body-scoped — that's intentional, the strings
# are themselves load-bearing identifiers wherever they appear:
rg -n 'legacy.*deserialize|retired.*field' compiler/src/ -g '!target' -g '!*.rs.bk'

# Retirement comments dated to a commit
rg -n '§F[0-9]+|retired in chunk|post-Chunk-[0-9]+|pre-Chunk-[0-9]+|Post-8A|pre-8A' compiler/src/

# Retired enum variants / compile_error / deprecated
rg -n '#\[deprecated\]|compile_error!|/// retired|/// deleted' compiler/src/

# Real surface of public API — the doc-side reference set
rg -n '^pub (fn|struct|enum|use|type|const) ' compiler/src/lib.rs

# Real subcommand list
rg -n 'Commands::|Subcommand' compiler/src/main.rs

# Negative-test test names (these are the contracts a doc violation would break)
rg -n '#\[test\][^a-z]+(fn )?(serde_breaking|retirement|guard|must_not|should_fail)' compiler/
```

The output of this run is the **full invariant catalog**. Every doc fix in §4 below must cite one row of this catalog as authority. No fix lands without an invariant cite.

### 3.2 Derive the grep set from the invariant catalog, not from a previous fix

For each invariant in §3.1, write the grep that would find a doc claim contradicting it. Examples:

| Invariant (from §3.1) | Doc-violation grep |
|-----------------------|--------------------|
| `serde_breaking_change_on_sprite_shape` retires top-level `img_data`/`sprite_name` | `rg -inE '\bimg_data\b\|\bsprite_name\b' personas/ SPEC.md CLAUDE.md` (excluding the accessor pattern `sprite.img_data()`) |
| `ModifierType::Legendary` retired in chunk-8A | `rg -in '\b(Legendary\|Capture)\b' personas/ SPEC.md CLAUDE.md` (excluding game-design vocabulary callouts in `personas/slice-and-dice-design.md` + the SPEC §10 explicit-NOT-IR carve-out) |
| `compiler/src/main.rs` Subcommand list = `Extract`/`Build`/`Check`/`Schema`/`Overlay` | `rg -in 'textmod-compiler (validate\|verify\|run)\b' personas/ SPEC.md CLAUDE.md` |
| `pub fn build(ir: &ModIR) -> Result<...>` (single-arg) | `rg -in 'build\(ir, sprites\|build_textmod\(.*sprites\|fn build.*sprites:' personas/ SPEC.md CLAUDE.md` |
| Phantom files (`validator.rs`, `sprite.rs` at top-level, `capture_*.rs`) | `rg -in 'compiler/src/(validator\.rs\|sprite\.rs\|capture_)' personas/ SPEC.md CLAUDE.md` |
| Real semantic-check fn = `xref::check_references` | `rg -in 'fn validate\(\|validate_textmod\b' personas/ SPEC.md CLAUDE.md` |

Run **all** greps in one pass against the **full repo** (not just files modified by a prior round). Save the consolidated hit list to `plans/.doc_audit_violations.txt`. This is the punch list. Nothing gets fixed before the punch list is complete.

### 3.3 §F-series source-citation rewrite (resolved per round-1 user decision (b))

**Decision (per user, 2026-04-27 — round-1 audit of this plan):** the §3.3 rewrite covers **every `§F<N>` citation in `compiler/src/`**, not just the four `§F4` callsites the original plan listed. Per CLAUDE.md "plans go stale once executed", source code MUST NOT depend on plan-section IDs as durable references — and `§F4` is structurally identical to `§F5`/`§F8`/`§F10` in this respect. The rewrite uses **datestamped-ruling-name prose** that survives plan rot and SPEC restructuring alike. The original §F4-only scope was the same anti-pattern §0 condemns: scoping the grep narrower than the protected invariant.

**Rewrite pattern.** Each `§F<N>` citation becomes `<YYYY-MM-DD> "<short ruling name>" ruling`. The combined `<date> "<ruling-name>"` is the stable handle. The ruling name itself is the load-bearing identifier (the date alone repeats — multiple rulings landed on 2026-04-20 / 2026-04-22); the §5.1 guard test asserts each ruling-name string appears nowhere else in the doc surface that doesn't relate to its protected invariant.

**Ruling-name table** (canonical — every rewrite cites one row of this table):

| §F-N | Protected invariant / ruling subject | Authoring date (verified in this session via `grep -nE '^### F[0-9]+' plans/PLATFORM_FOUNDATIONS_PLAN.md` + the dated `Chunk` headers below each section) | Canonical datestamped name |
|------|--------------------------------------|--------------------------------------------------------------------------------------------------|----------------------------|
| §F3  | Permissive Face IDs + `Pips` newtype — `FaceIdValue::{Known, Unknown}`, extraction succeeds with `Severity::Warning` | 2026-04-20 (`user ruling 2026-04-20: permissive (b)` at PLATFORM_FOUNDATIONS_PLAN.md:148) | `2026-04-20 "permissive face IDs" ruling` |
| §F4  | Sprite shape consolidation — `sprite: SpriteId`, no legacy back-compat, no serde shim | 2026-04-20 (`user ruling 2026-04-20: "no legacy, always choose correctness over back-compat"` at PLATFORM_FOUNDATIONS_PLAN.md:651) | `2026-04-20 "no legacy back-compat" ruling on sprite shape` |
| §F5  | `BuildOptions { include: SourceFilter }` + `build_with` + provenance-aware `Finding.source` severity promotion | 2026-04-22 (Chunk 4 complete header at PLATFORM_FOUNDATIONS_PLAN.md:676) | `2026-04-22 "BuildOptions + provenance-aware findings" ruling` |
| §F8  | No `unwrap`/`expect`/`panic!`/`unimplemented!`/`todo!` in `compiler/src/**/*.rs` outside `#[cfg(test)]`; enforced by `audit_lib_panic_free.rs` | 2026-04-22 (Chunk 7 re-audit header at PLATFORM_FOUNDATIONS_PLAN.md:379, PR #11) | `2026-04-22 "library code panic-free" ruling` |
| §F10 | Depth-aware scalar extraction in `parse_legendary` + emission-order requirement (`.sd.`/`.img.`/`.col.` before chain) | 2026-04-23 (Chunk 9 complete header at PLATFORM_FOUNDATIONS_PLAN.md:467) | `2026-04-23 "depth-aware scalar extraction" ruling` |

`§F0`/`§F1`/`§F2`/`§F6`/`§F7`/`§F9` have **zero hits in `compiler/src/`** per the round-1 census (re-verified for §F0 in this session by `grep -rn '§F0' compiler/ SPEC.md CLAUDE.md personas/` — zero hits in any in-scope file; HANDOFF.md hits are out of scope per §1's ephemeral-file exclusion). They do not need ruling-name entries until a future source citation appears.

**Project-internal token `§F10-MARKER`.** The `util.rs:228, :247, :507` references define and use a *named token* for the canonical marker set `{.i., .sticker., .abilitydata.}` — not a SPEC reference. Rewrite as a renamed token: `INNER_BODY_MARKERS` (the set names the body region the markers terminate). All three callsites flip together; the token has no consumers outside `util.rs`.

**Concrete callsites** (verified across `compiler/src/` for sub-commits 1a–1e via `grep -nE '§F[0-9]+'` and `grep -rn '§F4'` — both runs are quoted in the round-1 audit transcript above — and across `compiler/tests/` for sub-commit 1f via PR #15's round-1 tribunal extension generalized class-search; commit `5a5f86d` enumerates the 4-file / 10-callsite test-side scope. Per the chunk-implementation discipline "name the set once, never re-list its members", subsequent references to the §F-N callsite list — e.g. in §4.1 — point to this section by name instead of re-enumerating).

§F4 — sprite shape (6 callsites in 4 files: 5 src callsites in 3 files for sub-commit 1a + 1 test callsite in 1 file for sub-commit 1f):

**Rewrite rule.** Substitute every `§F4` token with `the 2026-04-20 "no legacy back-compat" ruling on sprite shape`. At first occurrence per file, expand to the full canonical name; subsequent occurrences in the same file may use the short form `the 2026-04-20 "no legacy back-compat" ruling`. Drop preceding `SPEC ` / `Per SPEC ` / `post-` qualifiers — the ruling name is self-identifying. Two callsites have idiosyncratic surrounding context that the rewrite preserves (`(2026-04-20 ruling + round-1 tribunal fix)` at monster_parser.rs:15 keeps the date-only short-short form because the surrounding parenthetical is already two phrases joined by `+`); per-site notes on these two:

Sites (file:line):
- Sub-commit 1a (`compiler/src/`):
  - `compiler/src/ir/mod.rs:1858, :1888, :1919` (line 1858 first-occurrence-in-file → long form; :1919 is the assertion message inside `serde_breaking_change_on_sprite_shape`, rewrite preserves the `must NOT deserialize into the post-...HeroBlock` shape with the date substituted for `§F4`. **Line :1919 was originally missed by the §F4-only enumeration — surfaced by round-1 audit's repo-wide grep.**)
  - `compiler/src/builder/hero_emitter.rs:309` (first-occurrence-in-file → long form; **keep `SPEC §3.3`** — that section exists in SPEC.md and is the live cross-reference, drop only `§F4`.)
  - `compiler/src/extractor/monster_parser.rs:15` (date-only short-short form per the per-site note above: `(2026-04-20 ruling + round-1 tribunal fix)`.)
- Sub-commit 1f (`compiler/tests/`; commit `5a5f86d`):
  - `compiler/tests/integration_tests.rs:373` (date-only short-short form per the `monster_parser.rs:15` precedent above — surrounding parenthetical `(§F4 follow-up)` is rewritten to `(2026-04-20 sprite-shape ruling follow-up)`.)

§F5 — BuildOptions / provenance-aware findings (19 callsites: 15 src callsites in 4 files for sub-commit 1b — `ir/merge.rs`, `builder/mod.rs`, `builder/options.rs`, `xref.rs` — plus 4 test callsites in 2 files for sub-commit 1f; src count and sites verified by `grep -nE '§F[0-9]+' compiler/src/` quoted in this session; test sites verified by commit `5a5f86d`'s body):

**Rewrite rule** (one line, applied at every site below — defined once per the chunk-implementation discipline "extract a helper or write the single correct line, never paste an incantation N times"):
- Substitute every `§F5` token with: `the 2026-04-22 "provenance-aware findings" ruling`.
- At the *first* occurrence per file, expand the short form to the full canonical name once: `the 2026-04-22 "BuildOptions + provenance-aware findings" ruling` — this anchors the file to the long form so a grep for the long name lands at least once per file.
- Drop any preceding `(plan ` / `Per PLATFORM_FOUNDATIONS_PLAN ` / `Per ` qualifier; the ruling name is self-identifying.
- Preserve all surrounding prose (parens, trailing punctuation, accompanying clauses) verbatim.

Sites (file:line, drawn from the §F-N census above; no per-site replacement strings — the rule above is the single source of truth):
- Sub-commit 1b (`compiler/src/`):
  - `compiler/src/ir/merge.rs:276`
  - `compiler/src/builder/mod.rs:39, :60, :95` (line 39 is first-occurrence-in-file → use long form)
  - `compiler/src/builder/options.rs:3` (first-occurrence-in-file → long form)
  - `compiler/src/xref.rs:49, :243, :776, :851, :1016, :1098, :1156, :1183, :1259, :1314` (line 49 is first-occurrence-in-file → long form)
- Sub-commit 1f (`compiler/tests/`; commit `5a5f86d`):
  - `compiler/tests/build_options_tests.rs:2, :177` (line 2 first-occurrence-in-file → long form; line 177 short form)
  - `compiler/tests/path_c_merge_tests.rs:1, :463` (line 1 also dropped a `plan §F6` qualifier — §F6 has zero entries in this ruling-name table per line 112's "zero hits in compiler/src/" census, so the qualifier is dropped without coining a new ruling name; line 463 first-occurrence-in-file uses long form per the rewrite rule)

§F3 — permissive face IDs (1 callsite):

**Rewrite rule.** Substitute `§F3` with `the 2026-04-20 "permissive face IDs" ruling`. Drop preceding `(PLATFORM_FOUNDATIONS_PLAN.md ` qualifier; preserve trailing `.` and surrounding parens.
- `compiler/src/xref.rs:35`

§F8 — library panic-free (6 callsites: 1 src callsite in 1 file for sub-commit 1d + 5 test callsites in 1 file for sub-commit 1f):

**Rewrite rule.** Substitute `§F8` with `the 2026-04-22 "library code panic-free" ruling`. Drop preceding `SPEC ` / `plan ` qualifier — the ruling name is self-identifying. At first-occurrence-in-file expand to the long form; subsequent occurrences in the same file may use the short form. Includes a user-visible assertion-message rewrite at `audit_lib_panic_free.rs:219` (formerly `"SPEC §F8 violation:"` — phantom SPEC ref since SPEC.md has zero §F-series sections).

Sites (file:line):
- Sub-commit 1d (`compiler/src/`):
  - `compiler/src/extractor/fight_parser.rs:563` (first-occurrence-in-file → long form per the rule above)
- Sub-commit 1f (`compiler/tests/`; commit `5a5f86d`):
  - `compiler/tests/audit_lib_panic_free.rs:1, :43, :97, :134, :219` (line 1 first-occurrence-in-file → long form; lines :43, :97, :134, :219 use the short form per the rewrite rule; the `:11` site that the round-1 audit briefly listed was dropped per the rewrite when the surrounding date-only short-short context was removed.)

§F10-MARKER — internal token rename (3 callsites in `util.rs`):

**Rewrite rule.** Flip token name from `§F10-MARKER` to `INNER_BODY_MARKERS` everywhere it appears (the new name describes the body region the markers terminate, the canonical set being `{.i., .sticker., .abilitydata.}`). The token has zero hits outside `util.rs` per the round-1 census (re-verified in this session by `grep -rn 'F10-MARKER\|INNER_BODY_MARKERS'` over `compiler/`, `personas/`, `SPEC.md`, `CLAUDE.md` — three util.rs hits, no others).
- `compiler/src/util.rs:228, :247, :507`

**Doc-side §F<N> citations** (also in scope per §1's row "live doc"):
- `personas/backend.md:134` — `// Carries name + inline .img. payload (per SPEC §F4)` → `// Carries name + inline .img. payload (per the 2026-04-20 "no legacy back-compat" ruling on sprite shape)`.
- `personas/backend.md:248` — `# SPEC §F8 — no unwrap/expect in lib code` → `# 2026-04-22 "library code panic-free" ruling — no unwrap/expect in lib code`.

**No §F-series gets added to SPEC.md.** Datestamped ruling names are forward-stable: when a future ruling lands, it gets a row in this table and gets cited as `<date> "<name>" ruling` at the affected callsite — no SPEC section number to thread through, no plan-coupling to rot.

### 3.4 testing.md TDD-progression rewrite (resolved)

Decision (per user): the testing persona's TDD chapter teaches a **workflow**, not API shapes. Rewrite the chapter as workflow prose; delete Rust snippets that pin signatures.

**Pattern**:
- Keep prose like *"Write a failing test that constructs a sample modifier line, calls `parse_hero`, and asserts the resulting `Hero` has the expected `name` field. Watch it fail. Implement the parser until it passes. Refactor."*
- Delete code blocks like:
  ```rust
  let hero = parse_hero(SAMPLE_HERO_LINE).unwrap();
  assert_eq!(hero.name, "Charmander");
  ```
- Code samples remain ONLY where they teach a Rust-specific pattern that prose can't carry (`if let Some(x) = ...`, `?` operator usage, etc.). API-call examples go.

**Affected lines** (verified in this session via Read on `personas/testing.md` lines 1-205 + grep for `parse_hero|classify(line)|build_charselect|build_ditto|extract|build`):

The principle "API-call examples go" applies to ALL five Phases, not just the parse_hero ones the original draft enumerated. Phase labels are corrected: `parse_hero` examples live in **Phase 2** (lines 53-90) and **Phase 3** (lines 92-136), not "Phases 3-5" as the original draft claimed.

| Phase | Lines (verified by Read) | API-pinning content to retire |
|-------|--------------------------|------------------------------|
| Phase 1 — Modifier Classification | 31-51 | `classify(line)` calls + `ModifierType::Hero` / `ModifierType::Unknown` enum-variant pins (lines 37, 45-48) |
| Phase 2 — Hero Parser | 53-90 | `parse_hero(SAMPLE_HERO_LINE).unwrap()` calls (58, 66); `hero.tiers[i].hp` / `hero.tiers[i].name` / `hero.mn_name` / `hero.tiers.len()` field pins (59-61, 67-69, 77-87) |
| Phase 3 — Builder / Emitter | 92-136 | `parse_hero(SAMPLE_HERO_LINE).unwrap()` (97, 107, 124); `build_hero(&hero).unwrap()` (98, 108, 125); `output.char_indices()` / `.rfind(".n.")` pattern pins |
| Phase 4 — Character Selection / Ditto / ReplicaItems / Monsters | 138-176 | `extract(&sliceymon_text()).unwrap()` (143, 155); `extract(&text).unwrap()` (167 — uses local `text` binding, not the `sliceymon_text()` helper, but same parser-API retirement target); `build_charselect(&ir.heroes)` (144); `build_ditto(&ir.heroes)` (156); `build(&ir1).unwrap()` (168); `assert_eq!(a.target_name, b.target_name)` (172); `a.trigger.dice_faces()` (173) — these pin the post-Chunk-9 `ReplicaItem.target_name` rename and the `SummonTrigger::dice_faces()` accessor signature |
| Phase 5 — Full Round-Trip | 178-198 | Four `roundtrip_*` test bodies: `extract(&original).unwrap()` / `build(&ir_a).unwrap()` / `assert_ir_equal(&ir_a, &ir_b)` (184-187, 191-197) |
| `assert_ir_equal` description | 200-205 | Bullet list pinning `replica_items` field name + tier ops |

**Rewrite shape** (per Phase): replace the fenced ```rust block with a single prose paragraph that describes the *workflow* (what failing test to write, what to assert at the abstract level, what shape passing implies) without citing function names, struct field names, or method signatures. A reader new to the codebase still understands the discipline; a reader who wants the API navigates to `compiler/src/lib.rs` and `compiler/src/ir/mod.rs`.

The `.doc_audit_carveouts` registry from §3.5 below does NOT carry these as carve-outs after the rewrite; they're deleted, not preserved.

### 3.5 Carve-out rule (explicit, not implicit)

Every hit in `.doc_audit_violations.txt` is either a **fix** or a **carve-out**. A carve-out requires:
- A one-line stable rationale that survives hostile re-prosecution (a future reviewer running the same grep lands on the same rationale).
- An entry in the carve-out registry (path picked at implementation start per §6 decision 3) with the file:line, the rationale, and the invariant the carve-out doesn't violate.

**Registry format** (resolved during round-1 audit). The registry is **TOML** (`compiler/tests/doc_invariants_carveouts.toml` is the working path; finalize at implementation start):

```toml
[[carveout]]
path = "compiler/src/ir/mod.rs"
line = 622
pattern = "captures"
rationale = "English-verb usage, not the retired Capture type name"
invariant_not_violated = "Capture / Legendary IR vocabulary as kind-discriminator (§1 row 1)"
```

TOML is chosen because (a) the schema is rigid enough that the §5.1 guard tests can deserialize it with `serde::Deserialize` rather than hand-parsing markdown, (b) line-number drift is detectable (a `pattern` field that no longer matches at the recorded `line` triggers a guard-test failure that prompts the carve-out be re-verified or dropped). The `toml` crate is **not** currently in `[dev-dependencies]` per `compiler/Cargo.toml` (verified in this session by Read; current dev-deps are `assert_cmd = "2"` + `proptest = "1"` only); implementation must add `toml = "0.8"` to `[dev-dependencies]` as part of the §5.1 commit. Adding a dev-dep is **outside the doc-only PR boundary** (§0/§8) — it's the one exception, justified by the §5.1 guard tests being load-bearing for the audit's structural-enforcement story. The §8 anti-pattern "no source-code behavior changes" stands; a `[dev-dependencies]` addition does not change runtime behavior.

Examples of stable rationales (each becomes a `[[carveout]]` entry):
- "English-verb usage" — `compiler/src/ir/mod.rs:622` "captures" as a verb in a comment, not the retired type name.
- "Negation, deliberate" — `SPEC.md:59` "There is no separate validator pass to bolt on later" — the word "validator" is the very thing being negated. Same pattern at `personas/architecture.md:72` (`"validator pass" exists` inside an ASCII-art negation), `personas/ai-development.md:628, :656` (both phrase "no separate validator pass"). Each gets a `[[carveout]]` entry; the rationale text is identical, the file:line differs.
- "Game-design vocabulary, NOT IR identifier" — `personas/slice-and-dice-design.md:137-138` (literal `Capture:` / `Legendary:` headings naming textmod patterns) and `personas/slice-and-dice-design.md:232` ("pseudo-legendary" Pokemon classification term). Each line gets its own entry; the file-wide blanket carve-out the original draft proposed at §3.2 row 2 is **withdrawn** — too coarse, would silently absorb future genuine violations.
- "Retired-vs-current contrast note, dated to commit" — `compiler/src/builder/mod.rs:23-27` "Post-8A: a single `replica_items` loop replaces the pre-rewrite capture / legendary stages" — historical contrast that documents the migration (verbatim text from Read above).
- "Bucket label preserved per chunk-8B obligation" — `compiler/src/xref.rs:179-214` literal `"legendary"` retained until chunk-8B unifies the bucket name (`compiler/src/xref.rs:184-187, :205-210` show the wired-in carve-out comment).
- "Error-payload field name, not IR field" — `compiler/src/error.rs:48` `ErrorKind::SpriteNotFound { sprite_name: String, ... }` — the name of the missing sprite, used for error display.
- "Glossary explicit-NOT-IR carve-out" — `SPEC.md:353` "Capturable / Legendary — Game-design vocabulary ... NOT IR identifiers — the IR discriminator is `SummonTrigger::{SideUse, Cast}`." The glossary line is the very thing protecting the invariant; carving it out preserves the protection.
- "Proposed WASM wrapper, not phantom claim" — `personas/frontend.md:65` `pub fn build_textmod(ir_json: &str)` and `personas/frontend.md:71-77` `pub fn validate_textmod(input)` — both are *proposed* WASM bindings around real library functions (`build`, `extract` + `check_references`); the body comment at line 73 already self-corrects ("There is no single validate(textmod) entry point — the pipeline IS validation"). Carve-out preserves the design proposal.
If a hit can't be paired with a stable rationale, it must be fixed.

## 4. Fix execution

### 4.0 Checkpoint configuration

- **Total chunks**: 10 main chunks (1, 2, 3, 4, 5, 6, 7a–7g aggregated as chunk 7, 8, 9, 10) plus chunk 1.5 (added between chunks 1 and 2 by PR #15's round-2 tribunal per §6 row 6 — runs in parallel with chunk 2 per §4.1's parallel-execution map). Chunk 1 itself has 6 sub-commits 1a–1f; sub-commit 1f added during PR #15's round-1 tribunal extended chunk 1's scope from `compiler/src/` only to `compiler/src/ + compiler/tests/` per §3.3 line 116's partition.
- **Checkpoint frequency**: after every chunk (after every sub-commit for chunk 1)
- **Critical checkpoints** (require explicit user approval before proceeding):
  - **After 1f** — all `§F<N>` source citations gone across `compiler/`; verify with `grep -rn '§F[0-9]\+' compiler/` returning zero hits (excluding `compiler/target/`)
  - **After 6** — `personas/testing.md` TDD-chapter rewrite is the largest single divergence; review on its own
  - **After 8** — guard tests landed; `cargo test --test doc_invariants` must run green with the registry populated by 3–7
  - **After 9** — `.claude/settings.json` hook change verifiable only in next session

### 4.1 Parallel execution map

```
Foundation (sequential):
  Chunk 1 (sub-commits 1a → 1b → 1c → 1d → 1e → 1f — `§F<N>` source
           citations in `compiler/src/` + `compiler/tests/`; sub-commit
           1f added during PR #15's round-1 tribunal extension to cover
           test-file siblings)
  Chunk 1.5 (`§Chunk Na` / `plan §N` / direct-plan-filename source
             citations — added by PR #15's round-2 tribunal per §6 row 6;
             7 sites in 6 files; replacement-strategy table inside §4.2's Chunk 1.5 entry)
  Chunk 2 (carve-out registry scaffold)

Parallel Group A (after Chunk 2; SPEC must land before personas can quote it):
  Chunk 3 (SPEC.md)
    └── Chunk 4 (CLAUDE.md — quotes SPEC; also adds §5.3 rule + source-of-truth row)
        └── Parallel Group B (after Chunk 4):
              Chunk 5 (personas/architecture.md)
              Chunk 6 (personas/testing.md — TDD rewrite, large)
              Chunk 7a (personas/backend.md)
              Chunk 7b (personas/frontend.md)
              Chunk 7c (personas/code-reviewer.md)
              Chunk 7d (personas/ai-development.md)
              Chunk 7e (personas/slice-and-dice-design.md)
              Chunk 7f (personas/security.md)
              Chunk 7g (personas/README.md)

Integration (sequential, after Group B):
  Chunk 8 (guard tests + Cargo.toml dev-dep + retirements.rs refactor)
    └── Chunk 9 (.claude/settings.json PreToolUse hook)
        └── Chunk 10 (inline /// doc comments — class-only)
```

Critical-path depth: 8 rounds (1 → 2 → 3 → 4 → {5, 6, 7a–7g in parallel} → 8 → 9 → 10; chunk 1.5 runs in parallel with chunk 2 on the same off-critical branch — both depend only on chunk 1) versus 22 sequential commits (6 sub-commits 1a–1f + 1.5 + 2 + 3 + 4 + 5 + 6 + 7a–7g + 8 + 9 + 10 = 22). Group B chunks share zero state — each persona file is independent — so a multi-agent runner can ship them concurrently.

### 4.2 Chunks

Each chunk uses the persona-required template: Scope / Files / Dependencies / Consumer / Dogfood / Verification. Per-chunk evidence requirements (§4.3) and commit-message format (§4.4) apply to every chunk.

---

#### Chunk 1: `compiler/` §F-series prose conversion

Sub-commits 1a–1f share scope, dependencies, consumer, and verification — only the site list and rewrite rule differ (per §3.3). Listed once here; per-sub-commit site lists live at §3.3.

**Scope**: Replace every `§F<N>` source citation in `compiler/src/` (sub-commits 1a–1e) and `compiler/tests/` (sub-commit 1f) with the canonical datestamped ruling-name prose from §3.3's table. Doc-comment-only changes; zero behavior delta. Sub-commit 1f was added during PR #15's round-1 tribunal (commit `5a5f86d`) after a generalized class-search surfaced 10 test-file callsites the original `compiler/src/`-scoped grep missed; 1f shares the §3.3 rewrite pattern with 1a–1e (long-form/short-form/date-only-short-short discipline).
**Dependencies**: None (foundation).
**Consumer**: All subsequent chunks reference the resulting citation pattern; chunk 8's ruling-name uniqueness guard tests assert each canonical name appears only at sanctioned sites.

**1a — §F4** (sprite-shape ruling). Sites + rewrite rule at §3.3 §F4. 3 files in `compiler/src/`.
**1b — §F5** (provenance-aware findings ruling). Sites + rewrite rule at §3.3 §F5. 4 files in `compiler/src/` / 15 callsites.
**1c — §F3** (permissive face IDs ruling). Site + rewrite rule at §3.3 §F3. 1 callsite in `compiler/src/`.
**1d — §F8** (library panic-free ruling). Site + rewrite rule at §3.3 §F8. 1 callsite in `compiler/src/`.
**1e — §F10-MARKER → `INNER_BODY_MARKERS` token rename**. Sites + rewrite rule at §3.3 §F10-MARKER. 3 callsites in `util.rs`.
**1f — `compiler/tests/` §F<N> source-citation siblings** (added during PR #15's round-1 tribunal extension; commit `5a5f86d`). Sites + rewrite rule at §3.3's per-§F-N "Sub-commit 1f (`compiler/tests/`)" subsections (under §F4, §F5, §F8). 4 files / 10 callsites.

**Dogfood (per sub-commit)**:
- `~/.cargo/bin/cargo build` succeeds (the rewrite touches doc-comments only — compilation proves no token was rewritten inside a code identifier).
- `~/.cargo/bin/cargo test` passes (no behavior delta).

**Verification (per sub-commit)**:
- [ ] `grep -rn '§F<N>' compiler/` (excluding `compiler/target/`) for the sub-commit's §F-N returns zero hits.
- [ ] Long form of the ruling name appears at least once per touched file (asserts the §3.3 first-occurrence-per-file rule).
- [ ] `~/.cargo/bin/cargo run --example roundtrip_diag` reports 4× ROUNDTRIP OK.

**Critical checkpoint after 1f**: `grep -rn '§F[0-9]\+' compiler/` (excluding `compiler/target/`) returns zero hits across both `compiler/src/` (closed by 1a–1e) and `compiler/tests/` (closed by 1f).

---

#### Chunk 1.5: `§Chunk Na` / `plan §N` / direct-plan-filename source citations

**Context.** Round 2 of PR #15's tribunal (2026-04-28) ran a generalized class-search after chunk 1 closed `§F[0-9]+` in `compiler/`. Six files / seven sites surfaced where source code cites a *plan-section ID under a different spelling* than `§F<N>` — `§Chunk Na`, `plan §N`, `parent plan §N`, or a direct `PLATFORM_FOUNDATIONS_PLAN.md` / `AUTHORING_ERGONOMICS_PLAN.md` filename in prose. The protected invariant (CLAUDE.md *"plans go stale once executed"*) is identical to the one §F<N> defended; the spelling differs. Per user decision §6 row 6, these are addressed as their own chunk rather than mixed into chunk 1's `§F<N>` rewrite or deferred to chunk 10.

**Scope (declared up front).** This chunk is **strictly narrower** than chunk 1: it covers only the seven enumerated sites below, all in `compiler/src/**/*.rs` and `compiler/tests/**/*.rs`. Out-of-scope (with stable carve-out rationales):
- Bare `Chunk Na` references without a plan-name qualifier (e.g. `Chunk 2 populates...`, `Chunk 8A retirement greps`, `// Chunk 8A: trigger-IR shape`). These are *commit-anchored timeline references* — once a chunk lands, its identity is durable in git history, so the cite isn't fragile under plan rot. Carve-out rationale: *commit-anchored chunk IDs are not plan-section IDs*. (A future review running the same generalized grep lands on this rationale and accepts.)
- Hits in `personas/*.md`, `SPEC.md`, `CLAUDE.md` — covered by chunks 3 / 4 / 7a-g.
- Hits inside `plans/`, `archive/`, `compiler/target/` — out of plan §1 by construction.

**Authoritative site list** (verified by Read of every line at branch tip `5a5f86d`, this session — 7 hits, no others under the chunk 1.5 pattern):

| File | Line | Current cite | Replacement strategy |
|------|------|--------------|----------------------|
| `compiler/src/util.rs` | 245 | `retired per plan §3.1 (zero corpus instances of top-level item.<…>)` | **(b) verbatim rule, no plan ref** — the `(zero corpus instances...)` parenthetical *is* the rule. Drop `per plan §3.1`; the surrounding prose already states the authority. Final: `retired (zero corpus instances of top-level item.<…>) — see compiler/tests/retirements.rs T13`. |
| `compiler/src/authoring/mod.rs` | 11 | `PLATFORM_FOUNDATIONS_PLAN.md / AUTHORING_ERGONOMICS_PLAN.md.` | **(c) deletion + replacement pointer** — the cited plans are roadmaps that go stale; the *real* authority for "what comes after Chunk 3a" is the next chunk that lands. Final: `Subsequent chunks add chainable builders per the live authoring layer in compiler/src/authoring/.` (Reader follows the code, not the plan.) |
| `compiler/src/authoring/replica_item.rs` | 11 | `Cast carries no ability-payload field (corpus has zero depth-0 .n.<spell_name> inside the spell-cast envelope's inner body; parent plan §1.1).` | **(b) verbatim rule, no plan ref** — the `(corpus has zero depth-0 ...)` parenthetical *is* the rule. Drop `; parent plan §1.1`; final ends `inner body).`. |
| `compiler/src/authoring/sprite.rs` | 144 | `Plan spec (PLATFORM_FOUNDATIONS_PLAN.md §Chunk 3a): img_data() must match working-mods/sliceymon.txt byte-for-byte.` | **(b) verbatim rule, no plan ref** — drop `Plan spec (PLATFORM_FOUNDATIONS_PLAN.md §Chunk 3a):` prefix; the byte-for-byte rule against `working-mods/sliceymon.txt` *is* the authority. Final starts `img_data() must match working-mods/sliceymon.txt byte-for-byte.` |
| `compiler/tests/spec_amendments.rs` | 4 | `These tests exist to prevent silent rollback of SPEC wording that Chunk 2 of plans/PLATFORM_FOUNDATIONS_PLAN.md required — the permissive-whitelist ruling (SPEC §3.6) and the Pips: i16 annotation.` | **(a) real SPEC anchor** — the surviving authority is `SPEC §3.6` (already cited later in the same sentence). Rewrite: `These tests exist to prevent silent rollback of SPEC §3.6's permissive-whitelist ruling and the Pips: i16 annotation.` Drop `Chunk 2 of plans/...`. |
| `compiler/tests/retirements.rs` | 2 | `Per parent plan §5, retirement greps live in an integration test file, not build.rs ...` | **(b) verbatim rule, no plan ref** — the `live in an integration test file, not build.rs (which is forbidden — coupling cargo build success to retirement absence drifts the WASM build surface)` IS the rule, self-justifying. Drop `Per parent plan §5,`; final starts `Retirement greps live in an integration test file, ...`. |
| `compiler/tests/retirements.rs` | 172 | `cast.sthief.abilitydata bodies have zero depth-0 .n.<spell_name> (parent plan §1.1).` | **(b) verbatim rule, no plan ref** — same pattern as `authoring/replica_item.rs:11`. Drop `(parent plan §1.1)`; the corpus-bytes rule self-justifies. Final ends `<spell_name>.`. |

**Replacement-strategy keys** (named once in the table column 4 above; this list documents the keys without re-asserting their per-site assignment):
- **(a) real SPEC anchor** — replace the plan-section cite with the underlying SPEC § / `compiler/src/...:line` cite that was the actual authority. Used when the plan was repeating a SPEC rule.
- **(b) verbatim rule, no plan ref** — drop the plan-section qualifier; the surrounding prose already states the rule the plan was citing. Used when the plan was just naming the rule the comment already explains.
- **(c) deletion + replacement pointer** — drop the plan-section cite and replace with a pointer to the *live* code surface. Used when the plan-section was a roadmap forward, not a rule.

**Why no new ruling-name table entries.** Chunk 1's `§F<N>` rewrite needed canonical datestamped names because each `§F-N` pointed at a *user ruling with verbatim wording* (e.g. `2026-04-20 "no legacy back-compat"`). Chunk 1.5's seven sites cite plan-section IDs whose underlying authority is already reachable without coining a new ruling name — see the per-site dispatch in the table column 4 above. This keeps chunk 1.5 strictly narrower than chunk 1 (no new shared identifiers); the §3.3 ruling-name table is unchanged.

**Dogfood (per chunk)**:
- `~/.cargo/bin/cargo build` succeeds (doc-comment-only changes — compilation proves no token was rewritten inside a code identifier).
- `~/.cargo/bin/cargo test` reports 364 passed / 0 failed (matches pre-chunk-1.5 baseline).
- `~/.cargo/bin/cargo run --example roundtrip_diag` reports 4× ROUNDTRIP OK.

**Verification (per chunk)**:
- [ ] `grep -rnE '§Chunk [0-9]+|plan §[0-9]+\.[0-9]+|plan §[0-9]+\b|parent plan §[0-9]+|PLATFORM_FOUNDATIONS_PLAN\.md|AUTHORING_ERGONOMICS_PLAN\.md' compiler/src/ compiler/tests/` returns zero hits.
- [ ] Bare-`Chunk Na` references (without plan-name qualifier) are NOT touched — `git diff main..HEAD -- compiler/src/ compiler/tests/ | grep -E '^[+-].*\bChunk [0-9]+[a-z]?\b' | grep -v -E 'PLATFORM_FOUNDATIONS_PLAN|AUTHORING_ERGONOMICS_PLAN|§Chunk|parent plan|plan §[0-9]'` returns zero hits (i.e. only the plan-qualified chunk cites moved).
- [ ] Each touched file's prose still parses as English without the dropped cite (Read of each touched line, post-edit).

**Critical checkpoint after 1.5**: chunk-1.5 grep returns zero; bare-`Chunk Na` carve-out is intact (the §6 row 6 rationale survives a re-grep).

---

#### Chunk 2: Carve-out registry scaffold

**Scope**: Create `compiler/tests/doc_invariants_carveouts.toml` as an empty-but-parseable TOML file (zero `[[carveout]]` entries, header comment only).
**Files**: `compiler/tests/doc_invariants_carveouts.toml` (new).
**Dependencies**: None.
**Consumer**: Chunks 3–7 append entries during their fix decisions; chunk 8's guard tests deserialize the registry via `serde::Deserialize`.

**Dogfood**:
- `python3 -c 'import tomllib; tomllib.load(open("compiler/tests/doc_invariants_carveouts.toml","rb"))'` (or equivalent) succeeds — file parses as valid TOML.
- File contains a top-level comment naming the §3.5 schema (path, line, pattern, rationale, invariant_not_violated).

**Verification**:
- [ ] File parses as valid TOML.
- [ ] Comment header matches §3.5 schema.

---

#### Chunk 3: SPEC.md

**Scope**: Apply every §3.2 grep finding scoped to `SPEC.md`; add carve-out entries to the registry for any deliberate-negation/glossary-NOT-IR sites per §3.5.
**Files**: `SPEC.md`, `compiler/tests/doc_invariants_carveouts.toml` (append carve-outs).
**Dependencies**: Chunk 2 (registry must exist to append to).
**Consumer**: Chunks 4–7 quote SPEC sections in their fix justifications.

**Dogfood**:
- Re-run §3.2 grep set against `SPEC.md`; every remaining hit has a matching `[[carveout]]` entry in the registry.

**Verification**:
- [ ] §3.2 grep set on `SPEC.md`, filtered against the registry, returns zero hits.
- [ ] Every appended `[[carveout]]` entry includes path, line, pattern, rationale, invariant_not_violated (per §3.5 schema).

---

#### Chunk 4: CLAUDE.md

**Scope**: Apply every §3.2 grep finding scoped to `CLAUDE.md`. Also add: §5.3 retirement-discipline rule under `## Working principles`; source-of-truth-table row for the carve-out registry per §5.2 layer 2.
**Files**: `CLAUDE.md`, `compiler/tests/doc_invariants_carveouts.toml` (append carve-outs).
**Dependencies**: Chunk 3 (CLAUDE.md may quote SPEC sections in justification).
**Consumer**: Group B persona chunks reference CLAUDE.md authority.

**Dogfood**:
- Re-run §3.2 grep set against `CLAUDE.md`; every remaining hit is registry-carve-out.
- `## Source of truth` table contains the row `Doc-class carve-outs | compiler/tests/doc_invariants_carveouts.toml`.
- `## Working principles` contains the §5.3 retirement-discipline rule (three-step commitment).

**Verification**:
- [ ] §3.2 grep set on `CLAUDE.md`, filtered against the registry, returns zero hits.
- [ ] Both new sections present and correctly cited.

---

#### Chunk 5: personas/architecture.md (Group B)

**Scope**: Apply every §3.2 grep finding scoped to `personas/architecture.md`; append carve-outs (the deliberate-negation `"validator pass" exists` ASCII-art at line 72 per §3.5).
**Files**: `personas/architecture.md`, `compiler/tests/doc_invariants_carveouts.toml`.
**Dependencies**: Chunk 4. **Parallel with**: 6, 7a–7g.
**Consumer**: None downstream within this PR; chunk 8 guard test asserts file is clean.

**Dogfood**: §3.2 grep set on the file, filtered against the registry, returns zero hits.

**Verification**:
- [ ] Grep clean modulo carve-outs.
- [ ] Each carve-out cites the protecting invariant.

---

#### Chunk 6: personas/testing.md (Group B, large)

**Scope**: Apply every §3.2 grep finding scoped to the file. **Plus**: rewrite the TDD-progression chapter (Phases 1–5, lines 31–198 per §3.4) as workflow prose, deleting all API-pinning Rust snippets per §3.4. The `assert_ir_equal` description (lines 200–205) loses its field-pinning bullets.
**Files**: `personas/testing.md`, `compiler/tests/doc_invariants_carveouts.toml`.
**Dependencies**: Chunk 4. **Parallel with**: 5, 7a–7g.
**Consumer**: None downstream within this PR; the §7 acceptance gate `awk '/^## TDD Progression/,/^## Test Design Principles/' personas/testing.md | grep -c '^```rust'` returning 0 verifies the rewrite landed.

**Dogfood**:
- §3.2 grep set on the file, filtered against the registry, returns zero hits.
- `awk '/^## TDD Progression/,/^## Test Design Principles/' personas/testing.md | grep -c '^```rust'` returns 0.
- Skim the rewritten chapter — workflow prose still teaches red-green-refactor without naming any function/field/method signature.

**Verification**:
- [ ] Grep clean modulo carve-outs.
- [ ] Zero ```rust fences in TDD-progression chapter.
- [ ] Code samples remain only where teaching a Rust-specific pattern prose can't carry (per §3.4 carve-out rule).

---

#### Chunk 7 (Group B parallel): persona files

7a `personas/backend.md` — also picks up the two doc-side §F-N rewrites at lines 134, 248 per §3.3.
7b `personas/frontend.md` — applies §3.5 carve-outs for the proposed `build_textmod` / `validate_textmod` WASM bindings at lines 65, 71-77.
7c `personas/code-reviewer.md`.
7d `personas/ai-development.md` — applies §3.5 carve-outs for the deliberate-negation phrases at lines 628, 656.
7e `personas/slice-and-dice-design.md` — applies per-line §3.5 carve-outs for game-design-vocabulary headings (lines 137-138, 232).
7f `personas/security.md`.
7g `personas/README.md`.

**Scope (per file)**: Apply every §3.2 grep finding scoped to the file; append carve-outs per §3.5.
**Files (per chunk)**: the persona file + `compiler/tests/doc_invariants_carveouts.toml`.
**Dependencies (per chunk)**: Chunk 4. **Parallel with**: each other + 5 + 6.
**Consumer**: None downstream within this PR; chunk 8 guard tests assert each is clean.

**Dogfood (per chunk)**: §3.2 grep set on the file, filtered against the registry, returns zero hits.

**Verification (per chunk)**:
- [ ] Grep clean modulo carve-outs.
- [ ] Carve-outs cite protecting invariants.

---

#### Chunk 8: Guard tests + dev-dep + retirements.rs refactor

**Scope**: Land the §5.1 guard tests. Adds `compiler/tests/doc_invariants.rs` (per-invariant tests + ruling-name uniqueness tests) + `compiler/tests/common/mod.rs` (shared `recursive_grep` with extension-list and skip-list parameters, `has_word_boundary_match`, `filter_carveouts` per §5.1). Refactors `compiler/tests/retirements.rs` to call the shared helper. Adds `toml = "0.8"` to `[dev-dependencies]` in `compiler/Cargo.toml` (the only `Cargo.toml` change in this PR — justified at §3.5 / §5.1).
**Files**: `compiler/tests/doc_invariants.rs` (new), `compiler/tests/common/mod.rs` (new), `compiler/tests/retirements.rs` (refactor), `compiler/Cargo.toml` (dev-dep).
**Dependencies**: Chunks 2 (registry exists) + 3, 4, 5, 6, 7a–7g (registry populated). Cannot land before all of Group B.
**Consumer**: CI runs the new tests on every commit forever; chunk 9 hook references the registry the tests load.

**Dogfood**:
- `~/.cargo/bin/cargo test --test doc_invariants` passes — every per-invariant test green, every ruling-name uniqueness test green.
- `~/.cargo/bin/cargo test --test retirements` passes (refactor preserves behavior).
- `~/.cargo/bin/cargo test` full suite passes — no regressions vs. pre-PR baseline (recorded in PR description per §7).

**Verification**:
- [ ] One test per row in §3.1's invariant table.
- [ ] One ruling-name uniqueness test per row of §3.3's ruling-name table.
- [ ] `retirements.rs` calls shared `recursive_grep` (no duplicate copy).
- [ ] Single `const CARVEOUT_REGISTRY: &str = "tests/doc_invariants_carveouts.toml";` declaration; all tests load the registry through it.
- [ ] `compiler/Cargo.toml` `[dev-dependencies]` has `toml = "0.8"`; no other Cargo.toml changes.

**Critical checkpoint after 8**: full test suite green; registry-driven enforcement is the audit's load-bearing artifact going forward.

---

#### Chunk 9: `.claude/settings.json` PreToolUse hook

**Scope**: Add the §5.2 layer 3 PreToolUse hook on `Edit`/`Write` for `personas/*.md`, `SPEC.md`, `CLAUDE.md` (HANDOFF.md excluded per §1). Hook surfaces a **summary** of the carve-out registry — count + one-line index entries (`<file>:<line> — <pattern> — <one-line rationale>`) + pointer to the TOML file. Follows the same `jq -n '{hookSpecificOutput:{hookEventName:"PreToolUse",additionalContext:"..."}}'` shape as the existing Evidence-rule hook.
**Files**: `.claude/settings.json`.
**Dependencies**: Chunks 2 + 8 (registry exists, tests reference it).
**Consumer**: Future doc-edit operations in next sessions; not directly testable in this PR.

**Dogfood**:
- `jq . .claude/settings.json` succeeds (file parses).
- The new hook entry mirrors the existing Evidence-rule hook's shape verbatim except for matcher and command body.
- Hand-execute the hook command against the current registry; output is a count + one-line index, **not** the full registry contents (per §5.2 context-bound discipline).

**Verification**:
- [ ] `.claude/settings.json` parses as valid JSON.
- [ ] Hook matcher covers `personas/*.md`, `SPEC.md`, `CLAUDE.md`; excludes `HANDOFF.md`.
- [ ] Hook output is a summary (count + index + pointer), not the full registry.

**Critical checkpoint after 9**: hook only verifiable in a future session — confirm with user the shape is right before merging.

---

#### Chunk 10: Inline `///` doc comments

**Scope**: Class-only fixes in `compiler/src/**/*.rs`'s `///` doc comments — touch only when the comment makes a claim that contradicts a §3.1 invariant (per §6 decision 4).
**Files**: TBD — driven by the §3.2 grep set re-run against `compiler/src/**/*.rs` after chunk 8 lands.
**Dependencies**: Chunk 8 (guard tests now grep `compiler/src/**/*.rs` too — the test failures are this chunk's punch list).
**Consumer**: CI; the chunk 8 cross-tree test going green is the success signal.

**Dogfood**:
- `~/.cargo/bin/cargo test --test doc_invariants` passes against the full tree (markdown + `compiler/src/**/*.rs`).
- `~/.cargo/bin/cargo test` full suite still passes.
- `~/.cargo/bin/cargo run --example roundtrip_diag` reports 4× ROUNDTRIP OK.

**Verification**:
- [ ] All §5.1 guard tests green against full tree.
- [ ] No behavior delta.

### 4.3 Per-fix evidence requirement

Every edit cites one row from `.doc_audit_invariants.txt` (the §3.1 catalog) in the commit message. Format:

```
docs(<file>): retire <retired thing> per <invariant>

Invariant: <verbatim quote from §3.1 catalog, with file:line>
Doc violation: <verbatim quote from the doc, with file:line>
Replacement: <verbatim new text>
Why this is a class fix not a line fix: <enumeration of every other hit
in the same class addressed in the same commit>
```

### 4.4 Commit granularity

One commit per chunk (and per sub-commit for chunk 1). Multiple class fixes in the same file ship together within that file's chunk. Total commits: 22 per the §4.1 parallel-execution map's chunk enumeration above (canonical chunk set named once at §4.1; this section references the count by name without re-listing members). Each commit is independently reviewable. Group B and chunk 1.5's parallel-safety claims live at §4.1 (referenced here by name).

## 5. CI guards (the structural change)

The previous 23 rounds all surfaced **the same kind of defect** — a doc reference to a retired or non-existent code identifier. The doc audit alone fixes every current instance, but it doesn't prevent the class from re-opening when chunk-8B (or any future chunk) retires another identifier.

### 5.1 Per-invariant guard tests

Add `compiler/tests/doc_invariants.rs` with a test per invariant class. Each test runs the §3.2 grep against the live-doc set **AND `compiler/src/**/*.rs`** (so a stale `///` comment in code triggers the same failure as a stale persona claim) and asserts zero hits (excluding entries in the carve-out registry from §3.5). The test name names the invariant and the protecting code-side guard.

**Implementation discipline** (resolved during round-1 audit, anchored to `compiler/tests/retirements.rs` lines 1-95 read in this session):

- **Search primitive.** Generalize the `recursive_grep` helper pattern from `retirements.rs:24-94` into a shared `compiler/tests/common/mod.rs`. The current helper is purpose-built and has three properties that need to change for cross-extension use:
  - (i) **Path resolution.** Uses `crate_dir() = PathBuf::from(env!("CARGO_MANIFEST_DIR"))` (line 24-26) to anchor relative paths — so `Path::new("../personas")` resolves to `<repo_root>/personas` regardless of test-run CWD. **More robust than depending on Cargo's CWD behavior** — this is the form to keep.
  - (ii) **Extension filter is hardcoded to `.rs`** (lines 48, 76 — `path.extension() != Some("rs")` skips). Grepping `.md` files through the current helper silently returns zero hits. The shared helper takes an extension list parameter: `recursive_grep(root, pattern, &["rs", "md"])` or `&["md"]` for doc-only sweeps.
  - (iii) **Self-skip is hardcoded to `retirements.rs`** (lines 51-53, 80-82 — the file containing the retirement-pattern string literals must be skipped to avoid self-match). The shared helper takes a skip-list: `recursive_grep(root, pattern, &["rs", "md"], &["retirements.rs", "doc_invariants.rs"])`. Refactor `retirements.rs` to call the new shared helper in the same commit so it doesn't carry the duplicate.
- **Word-boundary handling.** Substring search can't express `\b...\b` directly. For invariants where word boundaries matter (e.g., distinguishing `img_data` the retired field from `sprite.img_data()` the current accessor), implement a small `has_word_boundary_match(line: &str, needle: &str) -> bool` helper using `char::is_alphanumeric` (or `_`) checks on the byte before/after the substring match. The helper lives in the same shared `tests/common/` module.
- **Carve-out registry.** `filter_carveouts(hits, registry_path)` deserializes the TOML registry (§3.5) into `Vec<CarveOut>` via `serde::Deserialize`, then drops any `Hit` where `(hit.path, hit.line)` matches a registry entry. The registry path is a single `const CARVEOUT_REGISTRY: &str = "tests/doc_invariants_carveouts.toml";` declared once at the top of `doc_invariants.rs`. Like the search helper, the path is resolved via `crate_dir().join(CARVEOUT_REGISTRY)` so it works regardless of CWD.
- **Dependency.** Add `toml = "0.8"` to `[dev-dependencies]` (justified at §3.5 above; one-line addition, contained to the §5.1 commit).

Example test (rewritten against the discipline above):

```rust
mod common;
use common::{recursive_grep, has_word_boundary_match, filter_carveouts, Hit};
use std::path::Path;

const CARVEOUT_REGISTRY: &str = "tests/doc_invariants_carveouts.toml";

const DOC_SURFACE: &[&str] = &[
    "../personas",
    "../SPEC.md",
    "../CLAUDE.md",
    // HANDOFF.md is excluded — ephemeral local file, gitignored at .gitignore:13,
    // never persisted as audit material per §1's ephemeral-file exclusion.
    "src",  // CWD is compiler/, so src/ == compiler/src/
];

#[test]
fn no_doc_references_to_retired_img_data_field() {
    let mut hits: Vec<Hit> = Vec::new();
    for needle in &["img_data", "sprite_name"] {
        for root in DOC_SURFACE {
            for hit in recursive_grep(Path::new(root), needle) {
                // Word-boundary post-filter: drop the current-accessor pattern
                // `sprite.img_data()` and the error-payload field
                // `ErrorKind::SpriteNotFound { sprite_name: ... }` (see carve-outs).
                if has_word_boundary_match(&hit.line, needle) {
                    hits.push(hit);
                }
            }
        }
    }
    let live = filter_carveouts(hits, CARVEOUT_REGISTRY);
    let violations: Vec<_> = live.into_iter()
        .filter(|h| !h.line.contains("sprite.img_data()"))
        .filter(|h| !h.line.contains("SpriteId.img_data()"))
        .collect();
    assert!(
        violations.is_empty(),
        "Doc reference to the retired top-level img_data/sprite_name field. \
         Real shape: sprite: SpriteId per compiler/src/ir/mod.rs:574; accessor \
         is sprite.img_data() per :1885. Negative-test guard at :1908-1920 \
         requires legacy JSON to fail deserialization.\n\n\
         Violations:\n{:#?}",
        violations,
    );
}
```

The cross-tree grep is the structural fix for the §4 scope-vs-coverage tension. The audit fixes only markdown today (keeping the PR reviewable), but the guard greps the full tree forever — so a future chunk that retires an identifier without updating its `///` comments fails CI immediately, in that chunk, not three rounds of tribunal later.

One such test per row in §3.1. The test catalog is the audit's load-bearing artifact.

**Ruling-name uniqueness guard.** Per §3.3 the canonical datestamped name (e.g., `"no legacy back-compat" ruling on sprite shape`) is the load-bearing identifier. Add one guard test per ruling-name string asserting it appears nowhere in the doc surface OR `compiler/src/**/*.rs` outside of citations of its protected invariant (the carve-out registry whitelists the legitimate citations). This catches the failure mode where a future ruling reuses the same name — date alone repeats across rulings (multiple decisions landed 2026-04-20 and 2026-04-22 per the §3.3 table sources), so the name + date together are the stable handle, and the name itself must stay unique.

### 5.2 Enforcement layers (additive)

Carve-out registry location is incidental; what matters is mechanical enforcement so a reviewer (human or agent) can't forget the registry exists. Three layers, all project-local (no global skill changes):

1. **CI guard tests** (§5.1 above) — load the carve-out registry as a fixture. Already enforced by `cargo test`. Strongest layer; runs on every commit.
2. **CLAUDE.md source-of-truth table row** — add a line under `## Source of truth`: `Doc-class carve-outs | <path>`. Anyone reading CLAUDE.md before doing review work sees it.
3. **Claude Code PreToolUse hook on `Edit`/`Write` for `personas/*.md`, `SPEC.md`, `CLAUDE.md`** (HANDOFF.md is explicitly excluded per §1's ephemeral-file exclusion) — when an agent attempts a doc edit, the hook surfaces a **summary** of the carve-out registry's contents into the conversation: the count of carve-outs, a one-line index (`<file>:<line> — <pattern> — <one-line rationale>`), and a pointer to the full TOML at `compiler/tests/doc_invariants_carveouts.toml`. **Do not paste the full registry contents** — verified in this session by Read of `.claude/settings.json` that the existing PreToolUse hooks already inject ~2-3KB of text per invocation (Evidence rule reminder + plans/ chunk-implementation checklist). A growing registry pasted on every doc edit would compound context cost; the index keeps the load bounded. Configure in `.claude/settings.json` (project-local, not user-global). The hook follows the same `jq -n '{hookSpecificOutput:{hookEventName:"PreToolUse",additionalContext:"..."}}'` shape as the existing Evidence-rule hook (verified in this session by Read of `.claude/settings.json`).

The `/review-pr` skill is **not** updated. The skill is shared across projects; baking a project-specific carve-out path into it would break for projects that don't have one. The three layers above are project-local and self-contained.

### 5.3 Retirement-discipline rule (CLAUDE.md amendment)

Class-only audit scope today + full-tree CI guards forever closes the *current* class. The structural fix for *future* classes is a CLAUDE.md rule that makes "retire something" a multi-step commitment, not an aspiration. Add to CLAUDE.md under `## Working principles`:

> **Retiring a public identifier (function, type, field, enum variant, file path, CLI subcommand) is a three-step commitment in the same chunk:**
> 1. Add a retirement comment dated to the chunk (e.g., `// retired in chunk-8B (2026-04-29) — replaced by ReplicaItem trigger IR`).
> 2. Add a guard test in `compiler/tests/doc_invariants.rs` that greps the retired identifier across both the markdown surface AND `compiler/src/**/*.rs`, asserting zero hits (modulo carve-outs).
> 3. Update every doc reference (markdown + `///` comments + persona claims) to the replacement, in the same commit. "Follow-up to clean up docs" is forbidden — it's how the rounds-19-23 doc-drift loop opened.

This rule + the §5.1 guard tests mean a future identifier retirement either ships clean or fails CI immediately. There is no "next round of tribunal will catch it".

## 6. Decisions resolved during planning

All originally-open items resolved in planning conversations. Recorded here so the rationale survives:

1. **§F4 / SPEC-section-citation pattern** → option **(c) — delete the §F4 citations, replace with datestamped-ruling-name prose.** Rationale (per user): SPEC restructuring shouldn't break source-code citations. Datestamped ruling names (e.g., `the 2026-04-20 "no legacy back-compat" ruling`) are stable handles that survive section renumbers and stay greppable. Implementation lives in §3.3.
2. **TDD-progression `parse_hero(line).unwrap()` shorthand** → **rewrite testing.md TDD chapter as workflow prose; delete signature-pinning Rust snippets.** Rationale (per user): the persona teaches workflow (red-green-refactor), not the parser API. Code samples remain only where they teach Rust-specific patterns prose can't carry. Implementation lives in §3.4.
3. **Carve-out registry location and format** → **TOML at `compiler/tests/doc_invariants_carveouts.toml` (working path); enforcement layered, not located.** Rationale (per user): location only matters if the file is forgotten; if multiple layers always surface it, the path is incidental. Three project-local layers in §5.2 (CI guard tests, CLAUDE.md source-of-truth row, PreToolUse hook). The TOML format (resolved in round-1 audit, §3.5) lets the §5.1 guard tests deserialize the registry with `serde::Deserialize` rather than hand-parsing markdown, and supports a `pattern` field that detects line-number drift.
4. **Inline `///` doc-comment scope** → **class-only at write-time, full-tree at CI-test-time.** Rationale (per user concern that class-only might not catch enough): the §5.1 guard tests grep both markdown AND `compiler/src/**/*.rs`, so future drift in `///` comments fails CI even though *this* audit only fixes markdown. Combined with the §5.3 retirement-discipline rule in CLAUDE.md, the long tail is structurally closed without exploding this PR's scope.
5. **Source citing plan-section IDs (added round-1 audit, 2026-04-27)** → option **(b) — extend §3.3's rewrite pattern to ALL `§F<N>` source citations**, not just `§F4`. Rationale (per user, this conversation): per CLAUDE.md "plans go stale once executed", source code MUST NOT depend on plan-section IDs as durable references; `§F4` (a phantom SPEC ref) and `§F5`/`§F8`/`§F10` (real PLATFORM_FOUNDATIONS_PLAN refs) are structurally identical under that rule. The §3.3 ruling-name table covers both; the §5.1 cross-tree guard test asserts no new `§F<N>` source citation appears outside the carve-out registry.
6. **`§Chunk Na` / `plan §N` / direct-plan-filename source citations (added round-2 audit of PR #15, 2026-04-28)** → option **(b) — defer to a separate chunk (chunk 1.5)**, not bolted onto chunk 1 mid-PR. Rationale (per user, this conversation): the seven hits surfaced by Round-2's generalized grep cite plan-section IDs under a different spelling than `§F<N>` (e.g. `§Chunk 3a`, `parent plan §1.1`, `plan §3.1`, direct `PLATFORM_FOUNDATIONS_PLAN.md` filenames). They violate the same protected invariant — but extending chunk 1's `§F<N>`-only ruling-name table to invent canonical names for these would require new user-blessed ruling names and was not the user decision in §6 row 5. Chunk 1.5 (in §4.2) handles them with the **strategy-key** approach defined there, keeping the §3.3 ruling-name table unchanged. Bare `Chunk Na` references (without plan-name qualifier) remain carved out as commit-anchored timeline references — chunk identity is durable in git history once the chunk lands, so the cite isn't fragile under plan rot.

The plan ships ready to execute — no remaining ambiguity blocks §3-§5.

## 7. Acceptance criteria

- The §3.2 grep set, run across the live-doc surface AND `compiler/src/**/*.rs` and filtered against the carve-out registry, returns **zero hits**.
- `cargo test --test doc_invariants` passes — every §5.1 guard test green (one test per §3.1 invariant class + one ruling-name uniqueness test per row of §3.3's ruling-name table).
- `cargo test` (full suite) passes — no regressions versus the pre-PR baseline. (Pre-PR test count is established at implementation start by `~/.cargo/bin/cargo test 2>&1 | tail` on `main` and recorded in the PR description; the round-1 audit could not run `cargo test` from this session's bash because `cargo` is not on `PATH` here. The full path `~/.cargo/bin/cargo` is the project convention per `personas/ai-development.md:725` — `Use ~/.cargo/bin/cargo (not bare cargo) — PATH may not include it.`)
- `cargo run --example roundtrip_diag` passes (4× ROUNDTRIP OK) — proves the audit didn't accidentally touch behavior.
- `grep -rn '§F[0-9]\+' compiler/ SPEC.md CLAUDE.md personas/` returns **zero hits** (HANDOFF.md is out of scope per §1's ephemeral-file exclusion; the `§F10-MARKER` token is renamed and so leaves no `§F10` residue). The original draft's §F4-only criterion is broadened to all §F-N classes per §6 decision 5.
- `grep -rnE '§Chunk [0-9]+|plan §[0-9]+\.[0-9]+|plan §[0-9]+\b|parent plan §[0-9]+|PLATFORM_FOUNDATIONS_PLAN\.md|AUTHORING_ERGONOMICS_PLAN\.md' compiler/src/ compiler/tests/` returns **zero hits** post chunk 1.5 (per §6 decision 6; bare `Chunk Na` references without plan-name qualifier are the documented carve-out, not a violation).
- Every retired claim in personas has been replaced with the current shape, cited by `compiler/src/...:line` in the commit message.
- The carve-out registry (`compiler/tests/doc_invariants_carveouts.toml` per §3.5 working path) exists, lists every carve-out with stable rationale + invariant cite + (file, line, pattern) triple, and is referenced by every guard test through one constant.
- `CLAUDE.md` carries the §5.3 retirement-discipline rule under `## Working principles` and a row in the source-of-truth table for the carve-out registry.
- `.claude/settings.json` has a PreToolUse hook on `Edit`/`Write` for the doc surface (per §5.2 layer 3) that surfaces a **summary** of the carve-out registry (count + one-line index + pointer to the TOML file) into the conversation — not the full file contents per the §5.2 context-bound discipline.
- `compiler/Cargo.toml` `[dev-dependencies]` includes `toml = "0.8"` (justified at §3.5 / §5.1 — needed by the carve-out-registry parser; the only Cargo.toml change in this PR).
- `compiler/tests/common/mod.rs` exists and exposes the shared `recursive_grep` (with extension-list and skip-list parameters) + `has_word_boundary_match` + `filter_carveouts` per §5.1; `compiler/tests/retirements.rs` is refactored to call the shared helper rather than carrying its own copy.
- `personas/testing.md` has zero fenced ```rust blocks in Phases 1-5 of the TDD-progression chapter — `awk '/^## TDD Progression/,/^## Test Design Principles/' personas/testing.md | grep -c '^```rust'` returns 0 (per §3.4 "delete API-call examples"). Code samples elsewhere (Test Design Principles + downstream sections) remain.

## 8. Anti-pattern explicitly forbidden

- **No reactive grep widening.** If a violation surfaces during fix-execution that wasn't in `.doc_audit_violations.txt`, the audit (§3.2) was incomplete — re-run §3.1 + §3.2 from scratch and rebuild the punch list. Do not fix the new hit and continue. The whole point of this plan is to enumerate the class once.
- **No "fixed in this PR, follow-up for the rest".** Every hit on the punch list is closed in this PR or carved out in this PR. Follow-ups are how the multi-round loop opened in the first place.
- **No "the test passes so it's fine"** as a closing argument. The §5 guard tests close the class going forward; they do not retroactively prove the audit caught everything. The §3.1 invariant catalog + §3.2 grep set is what proves the audit was complete.
- **No source-code behavior changes.** This PR is doc-only. If a fix surfaces a real code defect (e.g., a `pub fn` that's actually missing), file a separate issue and continue with the doc audit. Mixing scopes re-creates the round-22 "PR scope expanded" pattern.

