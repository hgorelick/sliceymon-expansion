# Sliceymon+ — Project Specification

> **Status**: Source of truth for what we are building and why.
> **Scope**: The Rust textmod compiler and the Sliceymon+ Pokemon expansion authored through it.
> **Audience**: Future contributors (human and AI). Read this before opening any plan.

---

## 1. Vision

Build a **Rust textmod compiler** for [Slice & Dice](https://tann.fun/games/dice/) (mobile roguelike deck-builder by tann) that:

1. **Extracts** any valid textmod into a structured, self-contained intermediate representation (IR).
2. **Builds** an IR back into a pasteable textmod that the game accepts byte-for-byte cleanly.
3. **Authors** new mods (specifically: the Sliceymon+ Pokemon expansion) entirely through typed IR construction — never by hand-editing textmod text.
4. **Powers a future browser/mobile mod-builder app** as a WASM library, with the CLI as a thin wrapper over the same `lib.rs` surface.

The compiler is not a one-off script for a single mod. It is a **mod-building backend** that exposes CRUD, single-item operations, semantic validation, and JSON Schema for any textmod conforming to the format described in `reference/textmod_guide.md`.

The Sliceymon+ expansion (~100 Pokemon authored as heroes / capturables / monsters / bosses) is the proving ground that exercises every feature.

---

## 2. Non-Goals

- A textmod editor UI. (Future app; the compiler is its backend.)
- Game-engine reimplementation, simulation, or balance auto-solving.
- Support for textmod features not documented in `reference/textmod_guide.md` (Thunder's Undocumented Textmod Guide v3.2). The guide is authoritative — if the parser, the emitter, and the guide disagree, **the guide wins**.
- Backwards compatibility with the pre-guide pipeline in `archive/pre-guide/`. That work predates the format spec and must not be cited.

---

## 3. Architectural Invariants

These are the load-bearing properties of the system. They are non-negotiable.

### 3.1 Round-trip fidelity

```
extract(build(extract(mod))) == extract(mod)
```

Compared as **semantic IR equality**, not string diff. Tested against all four reference mods (`working-mods/{sliceymon,pansaer,punpuns,community}.txt`). Any change that breaks this for any mod is a regression.

### 3.2 No raw passthrough

Every IR field that the extractor populates must be consumed by the emitter, and every byte the emitter writes must come from a typed field. There is no `raw: String` escape hatch. If extraction can't represent a construct as fields, the IR schema is wrong — extend the schema, don't passthrough.

### 3.3 Self-contained IR

An extracted IR has everything required to rebuild the mod, including `.img.` sprite payloads stored as `img_data` on the relevant types. The builder requires no external sprite map, no companion files, no network. Path B (author IR from scratch and build) is the proof.

### 3.4 Library first, CLI second, WASM-ready

All operations live in `lib.rs` as `pub fn`. `main.rs` is argument parsing and file I/O only. Library code uses no `std::fs` or `std::process` so it compiles cleanly to WASM for the future app frontend.

### 3.5 Validation lives in the pipeline, not beside it

Structural validity = "the extractor succeeded." Cross-IR semantic checks (uniqueness, hero-pool reference resolution, color conflicts) live in `compiler/src/xref.rs` and operate on a fully extracted `ModIR`. There is no separate validator pass to bolt on later.

### 3.6 Make invalid states unrepresentable

Use Rust enums and newtypes to encode constraints at compile time wherever the constraint is a **format invariant** (e.g., `DiceFace::{Blank, Active{face_id: FaceIdValue, pips: Pips}}` not `String`). For **corpus-derived whitelists** (Face IDs, sprite registry), the typed layer is the authoring path and the source-of-truth for correctness, and unknown values are surfaced as `Unknown(raw)` variants that extract successfully and emit an `xref` `Finding` at `Severity::Warning`. This preserves SPEC §1 / §3.3 (any valid textmod extracts, self-contained IR) while keeping the authoring layer hallucination-free (SPEC §6.1). `Pips` is a newtype around `i16` (the corpus contains negative pips, e.g. `13--1`).

### 3.7 Correctness over convenience

"Too complicated", "too much overhead", "good enough", and "we'll fix it in a follow-up" are invalid justifications. No parallel representations, no `new_field` next to `old_field`, no deferred replacement. If the right design replaces an existing one, replace it everywhere — update every callsite, fix every test, handle the full blast radius.

---

## 4. Pipeline

```
                ┌─────────────────────────────────────────────────────────┐
                │                  Compiler library                        │
                │                                                          │
mod.txt ────────┤  Extractor                                              │
                │   classifier → type parsers (hero/capture/monster/      │
                │   boss/structural/chain/fight/phase/reward/level)        │
                │                       ↓                                  │
                │                    ModIR                                 │
                │   (typed fields only — self-contained, no raw)           │
                │                                                          │
                │  Operations (CRUD)                                       │
                │   add/remove/update per type                             │
                │   cross-category uniqueness, provenance tracking         │
                │                                                          │
                │  Cross-reference checks (xref)                           │
                │   uniqueness, hero-pool resolution, color conflicts      │
                │   single-item-in-context AND full-mod                    │
                │                                                          │
                │  Builder                                                 │
                │   type emitters → derived structurals → assembler        │
                │   (parens balanced by construction)                      │
                │                       ↓                                  │
                │                  textmod string                          │
                └─────────────────────────────────────────────────────────┘
```

**Canonical emission order** (enforced by the assembler, load-bearing for roundtrip fidelity):

```
structural (non-derived) → derived structural (char selection, hero pools)
  → heroes → items → replica items (captures, legendaries)
  → monsters → bosses
```

Any reordering is a roundtrip regression, not a builder-internal detail.

### Module boundaries

| Module                 | Knows about                                         | Does NOT know about              |
| ---------------------- | --------------------------------------------------- | -------------------------------- |
| `ir/`                  | IR types, CRUD ops, merge logic, provenance        | Parsing, emission, files, CLI    |
| `authoring/`           | Typed `FaceId` / `SpriteId` constructors, dice macros, roster lookups | Parsing, emission, files, CLI |
| `extractor/`           | Raw text → IR types                                 | Emission, file layout            |
| `builder/`             | IR types → raw text, derived structurals, canonical emission order | Parsing, file discovery |
| `xref.rs`              | IR types, cross-reference rules                     | Parsing, emission                |
| `lib.rs`               | Public API (extract / build / merge / xref / CRUD / authoring) | File I/O, CLI arg parsing |
| `main.rs`              | CLI args, file I/O, orchestration                   | Parser/emitter internals         |

### Build paths

| Path | Flow                                                                   | Why it matters                                       |
| ---- | ---------------------------------------------------------------------- | ---------------------------------------------------- |
| A    | `extract(mod.txt)` → CRUD edits → `build` → textmod                    | Modify an existing mod (Sliceymon+ on Sliceymon)     |
| B    | Author IR JSON from scratch → `build` → textmod                        | Proves the builder works without the extractor       |
| C    | `extract(base.txt)` → load overlay IR → `merge` → `build` → textmod    | Composable expansions over a base mod                |

All three paths use the same builder. Path B is the regression test that no builder logic secretly depends on extractor metadata.

#### Path C merge semantics

`merge(base: &mut ModIR, overlay: ModIR)` is **additive by identity key**:

- Content items (heroes, replica items, monsters, bosses) are keyed by `mn_name`. If the overlay carries a key not in base, it is **added** with `Source::Overlay`. If the key already exists in base, the overlay **replaces** it in place (overlay wins) and retains `Source::Overlay`.
- Derived structurals are **never merged** — they are regenerated from the merged content during `build`.
- Non-derived structurals are keyed by their structural kind + discriminator; overlay entries replace same-key base entries and are appended where no key collides.
- Merging is deterministic: the same `(base, overlay)` pair produces byte-identical output.

A merge that cannot resolve a key collision unambiguously is a `CompilerError`, not a silent pick.

### Derived structural modifiers

Some structurals are **computed from IR content**, never authored or stored independently:

| Structural          | Derived from | Reason                                                            |
| ------------------- | ------------ | ----------------------------------------------------------------- |
| Character Selection | Hero list    | Must list every hero color/label — drift = broken character pick  |
| HeroPoolBase        | Hero list    | The pool the game loads from                                      |
| PoolReplacement     | Hero list    | Pool overrides                                                    |
| Hero-bound ItemPool | Hero data    | Items tied to specific heroes (e.g., PorygonItem)                 |

Adding a hero auto-updates these. Their handling is **provenance-gated** (`Source::{Base, Custom, Overlay}`):

- `Source::Custom` — a human hand-authored a derived structural. This is a category error: `build` and `merge` reject it with `CompilerError::DerivedStructuralAuthored`. Authors never write derived structurals directly.
- `Source::Base` — came out of `extract` on a source textmod, which legitimately contains derived structurals. `build` and `merge` strip these before emission. `merge` additionally appends an `X010` `Severity::Warning` `Finding` to `base.warnings` per strip so downstream tools see what was dropped and why; `build` strips on a local clone (its `&ModIR` signature makes writing to the caller's `warnings` impossible) — callers that need the X010 sidecar run `merge` (which takes `&mut base`) or diff `ir.warnings` before and after a merge.
- `Source::Overlay` — came from an overlay that was either authored or loaded independently (per the Overlay definition in §4). We cannot distinguish hand-authored from extract-then-load at runtime, so `Source::Overlay` is treated the same as `Source::Base` — strip, and (in `merge`) X010 warn.

Regeneration after a strip is scoped to **kinds present-and-stripped**, not to all four kinds unconditionally. This preserves format-specific roundtrip: `sliceymon` encodes its character picker inline and has no top-level `Selector` flagged `derived: true`, so build emits no char-selection Selector; a mod that did carry a derived Selector gets a freshly-regenerated one against the post-merge content set.

Derived structurals are not "ignored-if-present"; their presence in input is either an error (Custom) or a strip (Base/Overlay) — never a passthrough.

---

## 5. IR Design

The IR is the contract between extractor, builder, CLI, app frontend, and LLM authors. It is the **schema of the mod itself**, not a serialization detail.

### Properties

1. **JSON-serializable** via serde, with `schemars` deriving a JSON Schema for editor validation.
2. **Human-readable**: meaningful field names, no positional encoding leaks.
3. **Self-contained**: `img_data` lives on the type that needs it; nothing depends on an external file at build time.
4. **Provenance-tracked and consumed**: every item carries `Source::{Base, Custom, Overlay}`. Provenance is not decorative — `build` accepts a `BuildOptions { include: SourceFilter }` that can restrict emission to (e.g.) `Custom | Overlay` for diff-style exports; `xref` uses provenance to scope rules (e.g., a `Base` item that violates a rule is a warning about the source mod, a `Custom` violation is an author error); and `merge` stamps provenance on the result. An item with no consumer for its provenance is a schema bug.
5. **Lossless across all types**: heroes, replica items (captures, legendaries), monsters, bosses, structurals, chains, fights, phases, rewards, level scopes — all roundtrip through fields.
6. **Authorable**: a user (or an LLM) can write valid IR JSON without ever opening a textmod.

### Public library surface (sketch)

```rust
// Whole-mod
pub fn extract(textmod: &str)              -> Result<ModIR, CompilerError>;
pub fn build(ir: &ModIR)                   -> Result<String, CompilerError>;
pub fn build_with(ir: &ModIR, opts: &BuildOptions)
                                           -> Result<String, CompilerError>;
pub fn merge(base: &mut ModIR, overlay: ModIR)
                                           -> Result<(), CompilerError>;
pub fn xref(ir: &ModIR)                    -> Vec<Finding>;

// Single-item
pub fn build_hero(h: &Hero)                                   -> Result<String, CompilerError>;
pub fn check_hero_in_context(h: &Hero, ir: &ModIR)            -> Vec<Finding>;
pub fn add_hero(ir: &mut ModIR, h: Hero)                      -> Result<(), CompilerError>;
pub fn remove_hero(ir: &mut ModIR, mn_name: &str)             -> Result<(), CompilerError>;
// ... same for Monster, Boss, ReplicaItem (captures and legendaries are both ReplicaItem kinds)

// Authoring layer — the only supported way to construct content
pub fn face(id: FaceId, pips: Pips)        -> DiceFace;                 // typed, whitelisted
pub fn sprite(name: &str)                  -> Result<SpriteId, CompilerError>; // registry lookup
pub fn die(faces: [DiceFace; 6])           -> Die;                      // macro-friendly
pub fn hero(template: Template, color: Color, hp: Hp, die: Die, ...)
                                           -> Result<Hero, CompilerError>;
```

The authoring layer (`authoring/` module) is the **only** supported path from human/LLM intent to an IR value. Direct struct-literal construction of `Hero`, `DiceFace`, etc., is possible in Rust but unsupported — it bypasses the typed whitelists and reintroduces the hallucination class `FaceId` / `SpriteId` exist to eliminate.

The CLI subcommands (`extract`, `build`, `merge`, `check`, `check --round-trip`, `schema`) are thin wrappers over the library surface.

### Failure modes

- `extract` returns `Err(CompilerError)` for inputs not conforming to `reference/textmod_guide.md`. It never panics, never silently discards bytes, and never produces a partial `ModIR`.
- `build` returns `Err(CompilerError)` for structurally impossible IR (e.g., a user-authored derived structural, a `FaceId` whitelisted for no template, a duplicate `mn_name`).
- `xref` never returns `Err` — every finding is a `Finding` so that tools can render partial reports.
- `merge` returns `Err(CompilerError)` on unresolvable key collisions (see §4 Path C).
- Errors must be actionable for an LLM author: `field_path` tells them *where*, `suggestion` tells them *what to try next*. An error readable only by the compiler author is a bug.

### Errors

Errors are **structured**, never flat strings:

```rust
struct Finding {
    rule_id: String,           // "X016", "X019", "X020", ... (X = xref; historical V-prefixed IDs are retained but new rules use X)
    severity: Severity,        // Error | Warning
    message: String,
    field_path: Option<String>,    // "heroes[3].blocks[2].sd"
    suggestion: Option<String>,    // "Valid Face IDs for Fey: 15, 32, 34, ..."
    modifier_index: Option<usize>,
    modifier_name: Option<String>,
}
```

Errors that lack a field path or fix suggestion are bugs. Errors that cannot be acted on by an LLM author (no concrete next step) are also bugs — the `suggestion` field is non-optional in spirit even where its type allows `None`.

---

## 6. Sliceymon+ Authoring

The Pokemon expansion is the compiler's first real customer. Constraints:

### 6.1 Author through the compiler, not around it

- Build IR through the authoring layer (typed Face IDs, sprite registry lookup, dice macros). Hand-written struct literals that bypass the typed lookups are forbidden — they reintroduce the hallucination class the typed layer eliminates.
- Hand-edits to a textmod string are forbidden. If you find yourself opening `working-mods/sliceymon.txt` to fix a comma, the authoring layer is missing something. Fix the layer.
- ASCII only. The game rejects em-dashes and smart quotes.

### 6.2 The user picks Pokemon

Don't suggest Pokemon to add. Don't preemptively design rosters. The user names a Pokemon; you provide role, color, template, and dice design grounded in the design persona.

### 6.3 No duplicate Pokemon

A Pokemon may exist in **at most one** of: heroes, replica items (captures / legendaries), monsters. CRUD operations enforce this; the author cannot accidentally bypass it.

### 6.4 Game-design rules (from `personas/slice-and-dice-design.md`)

These are project invariants, not just persona advice:

| Rule                                                                                                                        | Where it bites                                |
| --------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| Tier budgets — T1: 2-3 blanks / 1-2 pips / HP 3-6; T2: 1-2 / 2-3 / 6-9; T3: 0-1 / 3-6 / 8-13                                | Hero design                                   |
| Damage : Shield ≈ 1 : 1.4 in balance value; shields don't persist; heals cap at max HP and resolve after damage            | All face design                               |
| Monsters never reroll → **no Cantrip on monsters**, ever; use enemy-style Face IDs (170 / 171), not hero-style (15 / 36)   | Monster design                                |
| Boss difficulty curve — F4 simple, F8 multi-body, F12 lieutenant, F16 phase / regen, F20 escalating                         | Boss design                                   |
| Source fidelity serves fun — Pokemon competitive role informs but does not dictate; if strict fidelity is unplayable, fun wins | Pokemon → S&D translation                  |
| No power creep — a new hero must not strictly dominate an existing hero of the same color/role                              | Roster review                                 |

The Face ID reference, type → keyword mapping, and template selection table live in the design persona; this spec just enforces that designs comply.

### 6.4.1 Pokemon → S&D translation protocol

When designing any Pokemon as a hero, the design persona's protocol is mandatory:

1. **Establish Pokemon identity** — type(s), competitive role, signature moves/abilities, stat profile.
2. **Map to S&D role** — physical attacker → DPS, special attacker → spellcaster, wall → tank, support → healer, etc. (full table in `personas/slice-and-dice-design.md`).
3. **Apply type → keyword mapping** — Fire → Damage + Mana Pain; Grass → Heal + Growth; Steel → Steel Shield + Redirect; etc.
4. **Design the evolution curve** — T1 hints at the final identity, T2 makes role clear, T3 is the full power fantasy. Branching T3s (Poliwrath vs Politoed) must fill different roles.
5. **Verify against the roster** — no duplicates, role uniqueness within color, power-level parity with same-tier heroes, spell balance, draft impact ("would I always pick this? would I never pick this?" — both are problems).

A hero design that skips any of these steps is incomplete, regardless of how good it looks in isolation.

### 6.5 Fix issues you encounter

If you trip over a pre-existing format error, broken parens, wrong Face ID, or structural drift while doing other work, fix it before continuing. "It was already broken" is not an excuse.

---

## 7. Working with AI

Per `personas/ai-development.md`, AI work on this project is structured for **autonomous one-shot completion** with chunked checkpoints for larger tasks.

- **Complete context up front**: every task references the IR types, the format guide, and any pattern files needed before generation begins.
- **Self-verification**: AI validates its output against `reference/textmod_guide.md` and `cargo test` before declaring completion.
- **Anti-hallucination by construction**: typed Face IDs, typed sprite lookups, JSON Schema. A hallucinated face is a compile error, not a runtime bug.
- **Chunked plans for large work**: see the chunked-plan template in the AI development persona. Each chunk is one-shot; checkpoints happen between chunks.
- **Plans are roadmaps, not sources of truth**: once executed, the code is the truth. Plans in `plans/` go stale on purpose; do not cite them in long-lived spec or persona tables.

---

## 8. Quality Bar (CI gates)

A change is only "done" when **all** of these hold:

- [ ] `cargo test` passes (lib + integration + proptest).
- [ ] All four `working-mods/*.txt` round-trip cleanly (`cargo run --example roundtrip_diag` is empty).
- [ ] `cargo run -- check <mod> --round-trip` succeeds for any mod in scope.
- [ ] No new `unwrap()` / `expect()` / `panic!` in library code.
- [ ] No `std::fs` or `std::process` introduced in `lib.rs` or its modules.
- [ ] No raw passthrough fields (`raw: String` used to bypass field-based emission).
- [ ] Any new IR type ships with an extractor, an emitter, a round-trip test, and an entry in the JSON Schema.
- [ ] Any new behavior is defensible against `reference/textmod_guide.md`. Where the guide is silent, prefer minimal normalization (only for shapes the guide shows in multiple equivalent forms).
- [ ] Errors include `field_path` and a `suggestion` where applicable.

---

## 9. Source-of-Truth Map

| Concern                                       | File / path                                                      |
| --------------------------------------------- | ---------------------------------------------------------------- |
| Textmod format spec (authoritative)           | `reference/textmod_guide.md`                                     |
| IR schema                                     | `compiler/src/ir/mod.rs`                                         |
| Extraction pipeline                           | `compiler/src/extractor/`                                        |
| Build pipeline                                | `compiler/src/builder/`                                          |
| Cross-reference / semantic checks             | `compiler/src/xref.rs`                                           |
| Public library surface                        | `compiler/src/lib.rs`                                            |
| CLI                                           | `compiler/src/main.rs`                                           |
| Round-trip + integration tests                | `compiler/tests/`                                                |
| Reference mods (roundtrip + sprite corpus)    | `working-mods/{sliceymon,pansaer,punpuns,community}.txt`         |
| Game design / dice / balance reference        | `personas/slice-and-dice-design.md`                              |
| AI workflow reference                         | `personas/ai-development.md`                                     |
| Architecture / pipeline review                | `personas/architecture.md`                                       |
| Rust implementation patterns                  | `personas/backend.md`                                            |
| Working principles, commands, git conventions | `CLAUDE.md`                                                      |
| In-flight roadmaps (NOT durable truth)        | `plans/`                                                         |
| Pre-guide history (do not cite)               | `archive/pre-guide/`                                             |

---

## 10. Glossary

- **Textmod** — The plain-text mod format Slice & Dice ingests (paste into in-game ledger).
- **Modifier** — One comma-separated entry in a textmod (one hero, capture, monster, structural, etc.).
- **IR** — Intermediate Representation. The typed Rust structure produced by the extractor and consumed by the builder.
- **ModIR** — The full mod as IR — the root struct of a parsed textmod.
- **Replica** — Game term for a unit (hero / monster / boss). Templates are `replica.NAME`.
- **Face ID** — Numeric ID for a die face's behavior (e.g., 15 = basic damage, 126 = Cantrip, 170 = enemy damage). Encoded in `.sd.` as `FaceID-Pips:...`.
- **Pips** — The numeric magnitude on a face (damage amount, shield amount, heal amount).
- **Tier** — Evolution stage of a hero (T1 → T2 → T3). Separated by `+` at depth 0 in the modifier.
- **ReplicaItem** — IR type for items that summon a Pokemon as a unit. Two kinds: **Capture** (one-shot, mid-fight, via ball-style item) and **Legendary** (persistent ally with spell). Both share the same IR struct with a kind discriminant; "capturable" and "legendary" are *kinds*, not separate IR types.
- **Capturable / Legendary** — Kinds of `ReplicaItem` (see above). User-facing vocabulary only.
- **Authoring layer** — The `authoring/` module: typed `FaceId` / `SpriteId` constructors, dice macros, and roster-aware hero/monster/boss builders. The only supported path from human/LLM intent to IR values. Hand-written struct literals bypass it and are unsupported.
- **Overlay / Merge** — Path C flow. An overlay is a `ModIR` authored or loaded independently of a base mod; `merge(base, overlay)` composes them additively by identity key, with overlay winning on collisions and `Source::Overlay` stamped on merged items. Derived structurals are regenerated, never merged.
- **Structural modifier** — Non-content modifier: character selection, hero pools, party config, dialog, difficulty.
- **Derived structural** — A structural the builder generates from IR content rather than the user authoring it (e.g., character selection).
- **Roundtrip** — `extract(build(extract(mod))) == extract(mod)`. The correctness invariant.
- **Path A / B / C** — Build paths from §4: modify-existing, author-from-scratch, base-plus-overlay.
- **Provenance** — `Source::{Base, Custom, Overlay}` tag on each IR item — extracted vs added vs merged.
- **xref** — Cross-IR semantic checks operating on a fully populated `ModIR` (uniqueness, color conflicts, hero-pool resolution).
