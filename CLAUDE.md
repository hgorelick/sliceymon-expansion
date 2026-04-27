# Sliceymon+ — Claude Code Configuration

> **Project**: Slice & Dice textmod compiler + Sliceymon+ Pokemon expansion
> **Stack**: Rust compiler (`compiler/`) — extract → IR → build → cross-check.
> **Game**: Slice & Dice (mobile roguelike deck-builder by tann).
> **Spec**: [`SPEC.md`](SPEC.md) — read this first for vision, invariants, and quality bar.

## Project state

The primary artifact is the **Rust compiler in `compiler/`**, which extracts a textmod into structured IR and rebuilds it back to a pasteable textmod. Four reference mods (`working-mods/{sliceymon,pansaer,punpuns,community}.txt`) roundtrip cleanly; `extract(build(extract(mod))) == extract(mod)` is the correctness bar.

Validation is **not a separate pass** — structural validity is "extract succeeded"; cross-IR semantic checks (uniqueness, hero-pool ref resolution) live in `compiler/src/xref.rs` and need the full IR.

Sliceymon+ (the ~100-Pokemon expansion) is authored *through* the compiler — IR construction, not hand-edits to a textmod. The `archive/pre-guide/` directory holds the older hand-edit pipeline; do not cite anything there as authoritative.

`reference/textmod_guide.md` (converted from Thunder's Undocumented Textmod Guide v3.2) is the **format spec**. When the parser, emitter, and the guide disagree, the guide wins.

## Source of truth

| Purpose | File / Path |
|---------|-------------|
| Project spec (vision, invariants, quality bar) | `SPEC.md` |
| Format spec (authoritative) | `reference/textmod_guide.md` |
| IR schema | `compiler/src/ir/mod.rs` |
| Pipeline | `compiler/src/{extractor,builder}/` |
| Cross-IR semantic checks | `compiler/src/xref.rs` |
| Reference mods (roundtrip target + sprite/face-id corpus) | `working-mods/{sliceymon,pansaer,punpuns,community}.txt` |
| Compiler tests | `compiler/tests/`, `cargo test` |
| Game-balance reference | `personas/slice-and-dice-design.md` |
| AI workflow reference | `personas/ai-development.md` |

Active plans live in `plans/`. Treat them as **roadmaps for in-flight work**, not durable sources of truth — they go stale once executed.

Anything under `archive/pre-guide/` predates `reference/textmod_guide.md` (added 2026-04-10) and is not authoritative. Don't cite it.

## Working principles

### Communication style
- Don't say "you're right" or validate without analysis.
- When presenting options, state which you recommend and why.
- Be warm and collaborative while remaining precise.
- In summaries, surface incomplete work, deviations, simplifications, and assumptions explicitly.
- **Never include size, effort, time, or duration estimates** in plans, summaries, status updates, PR descriptions, comments, commit messages, or chat replies. No "~30 minutes", no "~2 hours", no "one focused day", no "small/medium/large", no "quick", no "easy", no "should take a while". Estimates are noise: I can't predict wall-clock time, the user doesn't need my guess, and they bias the user's prioritization with made-up numbers. Describe what work is involved (steps, files, dependencies), not how long it takes.

### Verify before delegating (non-negotiable)
Do NOT ask the user questions answerable by reading the repo. The answer to format/behavior questions lives in `reference/textmod_guide.md`, the compiler source, or the four reference mods — read it yourself.

"Needs user input" is reserved for genuine design ambiguity, never for factual questions about code or specs.

### Correctness over convenience (non-negotiable)
Don't weaken a design for speed, complexity, or effort. *"Too complicated"*, *"too much overhead"*, and *"good enough"* are invalid justifications.

**No deferred correctness.** No parallel representations, no "alongside existing" fields, no `new_field` next to `old_field`. If the right abstraction replaces an existing one, replace it — update every callsite, fix every test, handle the full blast radius. Effort is not a reason to defer.

If a design encodes a real game invariant (damage curve, tier progression, keyword budget, paste-correctness), the invariant must be preserved or strengthened.

### Evidence rule (non-negotiable, applies to every file)
No edit — code, test, plan, spec, doc, config, hook — lands without tool-output evidence from THIS conversation that the edit is correct. "Looks right" / "I remember the signature" / "standard pattern" is not evidence. Every edit must trace to one of:

1. **Corpus bytes.** Parser, emitter, IR shape, round-trip logic — quote the `working-mods/*.txt` lines the change is modeled on. Grep the corpus *before* changing extractor/emitter behavior.
2. **`reference/textmod_guide.md`.** Format semantics, modifier chain rules, nesting, sticker semantics, vase/hat/replica/cast structure, dice encoding — cite section + line. When parser/emitter/guide disagree, the guide wins.
3. **Current code state.** For rename/move/refactor/signature change — Read the file being changed AND every callsite, at its *current* state. Don't recall signatures, line numbers, or field types from memory. Grep for every caller before changing a type or signature.
4. **A failing test or reproduced bug.** Reproduce before fixing. "This could break X" without a reproduction is speculation.
5. **A named user decision.** Design choices, scope calls, trade-offs the user explicitly made.

**Forbidden shortcuts**: "verified at implementation start", "re-grep before authoring", "check the exact line count" are deferred verification — run the check *now*. "Probably" / "should" / "standard pattern" are guesses — Read the file. "The plan says X" as sole basis — plans can be wrong; re-verify against code and corpus. Writing against training-data memory of a library/API is forbidden — Read the local version.

If evidence is missing, stop and gather it before editing. If the change is genuinely trivial (typo, whitespace, lint) say so — triviality is its own evidence.

### Plans
- When implementing a plan, start promptly. Don't re-read plans extensively if you've seen them recently.
- In plan mode, follow `personas/ai-development.md`.
- Plans are instructions for future implementation, not changelogs. Rewrite in place; only keep historical context if it changes how the next reader will *act*.
- Plans go in `plans/` in the project root, not `.claude/plans/`.
- Plans are not sources of truth — once a plan is executed, the code becomes the truth. Don't cite plans in persona/spec tables.

### After changing the compiler
- `cargo test` (lib + integration + proptest) must pass.
- All four mods in `working-mods/` must still roundtrip — `extract(build(extract(mod))) == extract(mod)`.
- New parser/emitter behavior must be defensible against `reference/textmod_guide.md`. If the guide is silent, prefer normalization only for shapes the guide shows in multiple equivalent forms.

### Authoring Sliceymon+ content
- Build IR through the compiler's authoring layer, not hand-written struct literals or hand-edited textmod lines.
- Face IDs, sprites, and dice patterns must be typed/looked-up so hallucination is a compile error.
- The user picks Pokemon. Don't suggest additions unsolicited.
- ASCII only — game rejects em-dashes and smart quotes.

### Fix pre-existing issues
If you encounter a format error, broken parens, wrong Face ID, or structural drift while doing other work, **fix it before continuing**. "It was already broken" is not an excuse.

## Commands

```bash
# All from compiler/
cargo test                                         # run all tests
cargo run -- extract <mod.txt> -o <ir-dir>         # textmod → IR
cargo run -- build <ir-dir> -o <out.txt>           # IR → textmod
cargo run -- check <mod.txt>                       # extract (structural) + xref (semantic)
cargo run -- check <mod.txt> --round-trip          # also verify extract/build/extract is stable
cargo run --example roundtrip_diag                 # roundtrip diff for a mod
cargo run --example drift_audit                    # drift-class audit across mods
```

## Git conventions
- Branch from `main` for new features.
- Commit format: `type: description` (feat, fix, refactor, docs).
- **Never** add `Co-Authored-By` lines or any AI attribution to commits or PRs.
- Don't push while the user is on the work git account — ask first. User's GitHub: `hgorelick` (not `hgorelick-scala`).

## Personas

| Task | Persona |
|------|---------|
| Game balance, dice/hero/monster/boss design | `personas/slice-and-dice-design.md` |
| Plan structure, chunked tasks, one-shot design | `personas/ai-development.md` |
| Architecture review | `personas/architecture.md` |
| Code review | `personas/code-reviewer.md` |
| Testing strategy | `personas/testing.md` |

## When stuck
1. **Format question?** → `reference/textmod_guide.md` first, then `compiler/src/extractor/` for how we parse it.
2. **Drift after a change?** → `cargo run --example roundtrip_diag` and inspect the diff.
3. **Balance question?** → `personas/slice-and-dice-design.md`.
4. **Roundtrip failing?** → bisect against the four `working-mods/*.txt` to find which mod and which entity broke.
