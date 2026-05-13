# Platform Foundations — Product Brief

**Status:** Active
**Created:** 2026-05-07
**Last updated:** 2026-05-07

## Problem

The compiler is being built to serve as the typed mod-building backend for any author — a human modder, an LLM, or the future browser / mobile mod-builder app. Several gaps in today's compiler block that vision:

- A mod author can write a die face or a sprite name that doesn't correspond to any real game value, and the compiler silently accepts it. Mistakes that should fail at authoring time leak through to a paste-broken textmod the game rejects in-game.
- The result of extracting a mod is not self-contained: the build step requires the caller to supply a separate sprite payload map. A saved IR alone is not enough to rebuild the original mod, which contradicts what the IR is supposed to be.
- When something goes wrong, the error tells the author *that* something failed but not *where* in the mod or *what to try next*. An LLM acting as the author cannot recover from such errors — the diagnostic is unreadable.
- The compiler library still contains code paths that crash the host process on bad input. That is acceptable in a one-off CLI but unacceptable for a library that the future mod-builder app embeds and runs against arbitrary user input.
- The mod-composition path (combining a base mod with an overlay) does not conform to its own contract: two callers of merge can produce different output for the same inputs, and the operation can quietly drop modifiers the author wrote on purpose.
- The cross-IR rule set has overlapping rules — one mistake fires under two rule IDs — making it impossible for an author to tell whether they are seeing one defect or two, and making rule additions risky.
- The IR has historically carried variant shapes that no working mod actually uses. Carrying such a shape is a hypothesis: rules and tests written against it describe behavior that never executes in production, and any later chunk that builds on the unevidenced variant compounds the defect.

These are foundation defects. Until they are fixed, every other plan that builds on the compiler — the parser/emitter drift work, the author-ergonomics work, the eventual mod-builder app — accumulates the same risk.

## Solution

Reshape the compiler's library surface so the typed authoring path is hallucination-free by construction, the IR is genuinely self-contained, errors are actionable, the library never crashes its host, and the mod-composition operations behave deterministically and conform to the spec. Throw away IR shapes that have no corpus example. Where two rules cover the same defect, make exactly one of them own that case.

Authors continue to use the same CLI subcommands. The change is in *how the library behaves underneath* — the typed surface refuses invalid input, the build step needs no external inputs, and errors point at the offending field. The Sliceymon+ expansion is the proving ground; the future mod-builder app is the long-term consumer.

## Goals

- **The typed authoring path refuses hallucinated values.** Constructing a die face or a sprite reference through the typed surface against an unrecognized identifier is a compile-time error. An extracted mod that uses an unrecognized identifier still extracts and round-trips, and surfaces a warning so the author sees the unknown value. *Verified by:* a hero authored with an out-of-corpus face identifier fails to compile; an extracted mod containing the same identifier produces an extraction warning and round-trips byte-for-byte.
- **The IR is self-contained.** Building a mod from its IR requires no external sprite map, no companion files, and no network. *Verified by:* every working reference mod extracts, then rebuilds with no inputs other than the IR, and the rebuild round-trips.
- **Errors point at the field that failed and suggest a fix.** Every error returned by the library names the field where the failure occurred and a concrete next step the author can take. *Verified by:* every error construction site populates both fields; rendering an error includes both in the displayed message.
- **The library never crashes the host process.** No path in the library code can panic, regardless of input. Enforced by an automated audit that fails the test suite on regression. *Verified by:* an in-tree audit walks the library source, excludes test-gated items, and fails if any forbidden crash form remains.
- **Mod composition is deterministic and lossless.** Combining a base mod with an overlay alters the base in place; the same inputs produce the same output across runs; modifiers the author wrote on purpose are never silently dropped; modifiers the build derives from content can never be authored directly. *Verified by:* a base-plus-overlay composition of any two working mods produces byte-identical output across runs; an attempt to author a derived modifier returns an error rather than passing through.
- **Build emission can filter by source.** A caller can ask the build step to emit only modifiers from a chosen source (extracted, custom-authored, overlay), exclude a chosen source, or emit everything. Default behavior is unchanged. *Verified by:* building with each filter mode against an IR containing all three sources produces the expected subset.
- **Each cross-IR defect surfaces under exactly one rule.** No two rules fire on the same input for the same reason. Where two rules previously overlapped on cross-bucket name collisions, the narrower rule owns that slice and the broader rule retains only the cases the narrower one cannot cover. *Verified by:* cross-bucket collision tests assert one finding per defect; a regression that re-introduces the duplicate emission fails the cross-rule audit test.
- **IR variants reflect the corpus, not hypotheses.** Any variant of an IR sum must have at least one corpus example before it ships; variants with no example are deleted along with the rules and tests that referenced them. *Verified by:* every live variant discriminator can be traced to at least one occurrence in the working reference mods; rules and tests authored against unevidenced variants are removed in the same chunk that deletes the variant.

## Non-goals

- **No drift-class fixes to the parser or emitter.** Round-trip drift on existing reference mods is owned by the parser/emitter fidelity plan. This work changes type shapes and library surfaces, not parser or emitter behavior beyond what the type changes force.
- **No author-ergonomics layer.** Chainable builders, dice macros, hero-replica composition, and the final author-facing API live in the author-ergonomics plan. This work delivers only the empty authoring module skeleton plus the typed identifier primitives the ergonomic layer will sit on top of.
- **No new IR sums for as-yet-unobserved corpus classes.** This work covers the existing IR surface; new shapes the parser does not yet model are owned by the parser-fidelity plan.
- **No CLI changes for the author.** The CLI subcommands stay the same. The library surface underneath is what changes.
- **No game-balance or roster changes.** This is platform work; the working-mods corpus and any downstream mod's content are unaffected.

## User-facing changes

The changes here are seen by anyone consuming the compiler library — the Sliceymon+ author, the future mod-builder app, and any LLM authoring through the compiler. End-users of a published mod see no immediate difference; the change shows up at authoring or rebuild time.

- **Typed authoring surface.** Constructing a die face or a hero through the typed entry points refuses unrecognized face identifiers and unrecognized sprite names at compile time.
- **Extraction warnings for unknown corpus values.** A mod that uses a face identifier outside the working corpus extracts and round-trips, and produces a warning naming the unknown value.
- **No external sprite map needed for build.** Building a mod from IR requires only the IR. The previous parameter that took an external sprite payload map is gone from every public build entry point.
- **Field-path-aware errors.** Errors carry the field path where the failure occurred and a suggestion the author can act on.
- **Crash-free library.** The library refuses to crash on any input. The test suite fails if a regression introduces a new crash path.
- **In-place composition.** Combining a base mod with an overlay alters the base instead of returning a new copy. Findings produced by composition surface on the mod itself, not as a return-value tuple.
- **Source-filtered build.** Build accepts a filter so the caller can emit a chosen subset of sources or the full set.
- **One rule per defect.** Cross-IR diagnostics no longer fire two findings on the same input for the same reason.
- **IR shapes match what the corpus actually contains.** Variants without a corpus example are deleted from the IR — together with the rules and tests authored against them — rather than carried as design hypotheses.

## Open questions

None. Every product-level decision was resolved during plan execution. Outstanding cross-chunk technical decisions carry forward into the engineering plan.
