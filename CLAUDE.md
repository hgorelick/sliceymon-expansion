# Sliceymon+ — Claude Code Configuration

> **Project**: Slice & Dice textmod compiler + Sliceymon+ Pokemon expansion
> **Stack**: Rust compiler (`compiler/`) — extract → IR → build → cross-check.
> **Game**: Slice & Dice (mobile roguelike deck-builder by tann).

## Project state

The primary artifact is the **Rust compiler in `compiler/`**, which extracts a textmod into structured IR and rebuilds it back to a pasteable textmod. Four reference mods (`working-mods/{sliceymon,pansaer,punpuns,community}.txt`) roundtrip cleanly; `extract(build(extract(mod))) == extract(mod)` is the correctness bar.

Validation is **not a separate pass** — structural validity is "extract succeeded"; cross-IR semantic checks (uniqueness, hero-pool ref resolution) live in `compiler/src/xref.rs` and need the full IR.

Sliceymon+ (the ~100-Pokemon expansion) is authored *through* the compiler — IR construction, not hand-edits to a textmod. The `archive/pre-guide/` directory holds the older hand-edit pipeline; do not cite anything there as authoritative.

`reference/textmod_guide.md` (converted from Thunder's Undocumented Textmod Guide v3.2) is the **format spec**. When the parser, emitter, and the guide disagree, the guide wins.

## Source of truth

| Purpose | File / Path |
|---------|-------------|
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

### Verify before delegating (non-negotiable)
Do NOT ask the user questions answerable by reading the repo. The answer to format/behavior questions lives in `reference/textmod_guide.md`, the compiler source, or the four reference mods — read it yourself.

"Needs user input" is reserved for genuine design ambiguity, never for factual questions about code or specs.

### Correctness over convenience (non-negotiable)
Don't weaken a design for speed, complexity, or effort. *"Too complicated"*, *"too much overhead"*, and *"good enough"* are invalid justifications.

**No deferred correctness.** No parallel representations, no "alongside existing" fields, no `new_field` next to `old_field`. If the right abstraction replaces an existing one, replace it — update every callsite, fix every test, handle the full blast radius. Effort is not a reason to defer.

If a design encodes a real game invariant (damage curve, tier progression, keyword budget, paste-correctness), the invariant must be preserved or strengthened.

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
