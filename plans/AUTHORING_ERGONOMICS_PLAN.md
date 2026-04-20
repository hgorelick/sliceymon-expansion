# Authoring Ergonomics — Spec & Plan

## Overview

The compiler can parse and roundtrip the four working mods cleanly. The next limiting factor for both fresh mods and the Sliceymon+ expansion is **authoring cost**: constructing IR values today requires 15–30-line struct literals per unit, with every `Option` field, every shape flag (`head_paren`, `outer_paren`, `nested_single_paren`, `part`, `post_override_keywords`, `body_order`), and every raw `u16` Face ID written by hand. That cost compounds across ~100 heroes, their replicas, bosses, and monsters.

This plan adds a thin, opinionated authoring layer on top of the existing IR: defaults, constructors, chainable setters, typed Face ID constants, sprite lookup, and dice macros. **No IR changes.** The roundtrip guarantee is preserved because the authoring layer produces the same IR values the parser does — it's a different front door to the same house.

Downstream consumers (webapp, CLI JSON input) are unaffected: they construct IR from serde and bypass the authoring layer entirely.

---

## Goals

1. Make "add a Pokemon hero from a design doc" a one-screen task, not a one-hour task.
2. Make hallucinated Face IDs / sprite data a **compile error**, not a review-checklist item.
3. Zero change to existing parse/emit behavior — all four working mods must still roundtrip byte-stable.
4. Support the Sliceymon+ roster (~100 heroes, captures, monsters, bosses) at design-doc → code velocity.

## Non-Goals

- No new IR fields, no IR refactors, no emit changes.
- No authoring DSL / macro gymnastics beyond one small `dice![]` macro.
- No persistence layer — this is in-memory construction ergonomics only. Serde/JSON I/O stays as-is.
- No auto-balance checks. Structural validation is the extract/build pipeline; cross-IR semantic checks live in `compiler/src/xref.rs`. Balance heuristics are out of scope here.

---

## Spec

### S1. `Default` impls

Add `#[derive(Default)]` (or hand-written `impl Default`) to:

- `FightUnit` — `template: ""`, `name: ""`, all `Option`s `None`, all bool flags `false`, `Vec`s empty.
- `HeroBlock` — all optional fields `None`, `sd: DiceFaces { faces: vec![] }`, `sprite_name: ""`, etc.
- `Hero` — `blocks: vec![]`, `removed: false`, `source: Source::Base`, `format: HeroFormat::Unknown`.
- `ReplicaItem` — analogous.
- `Monster` — analogous.
- `Boss` — `fights: vec![]`, `format: BossFormat::Standard`, all optionals `None`.
- `FightDefinition` — `enemies: vec![]`, others `None`.
- `DiceFaces` — `faces: vec![]`.

Types that already derive `Default` (`Source`, `HeroFormat`, `BossFormat`) are untouched.

### S2. Typed constructors

Every authorable type gets a `new(...)` constructor taking the **required identity fields only**, defaulting the rest:

```rust
impl FightUnit {
    pub fn new(template: impl Into<String>, name: impl Into<String>) -> Self {
        Self { template: template.into(), name: name.into(), ..Default::default() }
    }
}

impl HeroBlock {
    pub fn new(
        template: impl Into<String>,
        name: impl Into<String>,
        sprite_name: impl Into<String>,
        sd: DiceFaces,
    ) -> Self { ... }
}

impl Hero {
    pub fn new(internal_name: impl Into<String>, mn_name: impl Into<String>, color: char) -> Self { ... }
}

impl ReplicaItem { pub fn new(name, container_name, template, sprite_name, sd) -> Self { ... } }
impl Monster     { pub fn new(name, base_template, floor_range) -> Self { ... } }
impl Boss        { pub fn new(name, level) -> Self { ... } }
impl FightDefinition { pub fn new() -> Self { Self::default() } }
```

The required arguments are the **minimum coordinates needed to identify the entity** — everything else is set via builder methods.

### S3. Builder-pattern setters

Chainable `with_*` setters for every optional field, moving `self` by value so calls compose:

```rust
FightUnit::new("Sniper", "Wooper")
    .with_hp(6)
    .with_sd(dice![34-1, 30-1, _, _, 30-1, _])
    .with_sprite_name("wooper")
    .with_color('a')
    .with_doc("A mud fish");
```

Setters return `Self`, so calls chain without intermediate bindings. Optional fields wrap inputs in `Some(...)` internally — the caller never writes `Some(...)`.

Setter coverage per type:

- **FightUnit**: `with_hp`, `with_sd`, `with_sprite`, `with_template_override`, `with_doc`, `with_chain`, `with_color`, `with_hsv`, `with_nested_units`, `with_part`, `with_post_override_keywords`, `wrap_outer`, `wrap_head`, `single_paren_nested`.
- **HeroBlock**: `with_hp`, `with_tier`, `with_color`, `with_doc`, `with_abilitydata`, `with_triggerhpdata`, `with_hue`, `with_chain`, `with_img_data`, `with_speech`, `bare()`.
- **Hero**: `with_format`, `with_block`, `with_blocks`, `removed()`, `from_custom()`.
- **ReplicaItem**: `with_hp`, `with_tier`, `with_color`, `with_doc`, `with_speech`, `with_abilitydata`, `with_item_modifiers`, `with_sticker`, `with_toggle_flags`, `with_img_data`.
- **Monster**: `with_hp`, `with_sd`, `with_sprite`, `with_color`, `with_doc`, `with_chain`, `with_balance`.
- **Boss**: `with_format`, `with_encounter_id`, `with_fight`, `with_fights`, `with_doc`, `with_chain`, `with_event_phases`.
- **FightDefinition**: `with_level`, `with_enemy`, `with_enemies`, `with_name`, `with_trigger`.

### S4. `DiceFaces` ergonomic constructors

Three layers, increasing convenience:

```rust
// Layer 1 — explicit typed pairs (useful for programmatic construction)
DiceFaces::from_pairs(&[(34, 1), (30, 1), (0, 0), (0, 0), (30, 1), (0, 0)])

// Layer 2 — explicit DiceFace vec (for when you have them already)
DiceFaces::from_faces(vec![DiceFace::Active { face_id: 34, pips: 1 }, DiceFace::Blank, ...])

// Layer 3 — the dice![] macro (for hand-authoring)
dice![34-1, 30-1, _, _, 30-1, _]
// expands to DiceFaces::from_pairs(&[(34,1),(30,1),(0,0),(0,0),(30,1),(0,0)])
```

The `dice!` macro supports:
- `ID-PIPS` for active faces (positive or negative pips).
- `_` for blank faces.
- Exactly 6 entries per die (checked at macro expansion).
- `dice![34, 30, _, _, 30, _]` — bare ID defaults to `pips: 0`.

### S5. Typed Face ID table

A `FaceId` module (not enum — enums with ~80 variants are awkward) exposing named constants derived from `reference/textmod_guide.md` (semantics) and corroborated by every `.sd.` occurrence in `working-mods/*.txt` (corpus). Grouped by mechanic:

```rust
pub mod face_id {
    // Damage
    pub const DAMAGE: u16 = 34;
    pub const DAMAGE_PHYS: u16 = 30;
    pub const DAMAGE_ARCANE: u16 = 40;
    // ...

    // Shield
    pub const SHIELD: u16 = 42;
    // ...

    // Heal, status, combo, etc.
}
```

Callers use them with the macro:

```rust
use slice_and_dice::ir::face_id::*;
dice![DAMAGE-1, SHIELD-2, _, _, DAMAGE-1, _]
```

Macro accepts identifiers resolving to `u16` as well as integer literals — `dice![34-1]` and `dice![DAMAGE-1]` both work.

**Source of truth**: `reference/textmod_guide.md` (face-mechanic semantics) + `working-mods/*.txt` (corpus of actual usage). Every constant doc-comment cites either (a) the guide section that defines the face mechanic, or (b) a working-mods unit where the face appears in `.sd.` (template name + line). A unit test enforces that every Face ID constant appears in ≥1 working mod's `.sd.` field OR has an explicit guide-section citation.

### S6. Sprite lookup

The four mods in `working-mods/` are the authoritative sprite source — every `.img.` payload in their hero, replica, monster, and boss lines, keyed by the `.mn.`/`.n.` name on the same line. A `build.rs` extracts and emits a compile-time lookup table:

```rust
// compiler/src/sprite.rs
pub fn lookup(name: &str) -> Option<&'static str> { ... }
pub fn lookup_expect(name: &str) -> &'static str { ... } // panics if missing
```

Setters that take sprite names (`.with_sprite_name("pikachu")`) use `lookup_expect` to resolve the base64 payload. Unknown names fail loudly at construction time instead of silently producing broken `.img.` data.

Optional convenience: `HeroBlock::with_sprite("pikachu")` sets BOTH `sprite_name` and `img_data` via lookup. `.with_sprite_name(name)` alone sets just the name field (existing behavior preserved for cases where img_data comes from elsewhere).

### S7. Hero-replica convenience

The Sliceymon three-tier replica is repetitive enough to deserve its own builder:

```rust
HeroReplica::new("Pikachu", 'a')
    .tier1(HeroBlock::new("t1tmpl", "Pichu", "pichu", dice![...]).with_hp(6))
    .tier2a(HeroBlock::new("t2tmpl", "Pikachu", "pikachu", dice![...]).with_hp(12))
    .tier2b(HeroBlock::new("t2tmpl", "Pikachu Alt", ...))
    .tier3a(HeroBlock::new("t3tmpl", "Raichu", "raichu", dice![...]).with_hp(20))
    .tier3b(HeroBlock::new("t3tmpl", "Raichu Alt", ...))
    .build()   // -> Hero
```

`build()` assembles the 5 blocks in the canonical Sliceymon order (T1, T2A, T2B, T3A, T3B), sets `format: HeroFormat::Sliceymon`, fills `internal_name`/`mn_name`/`color`, and marks `source: Source::Custom`.

This is a separate type (`HeroReplica`), not a method on `Hero`, because it encodes a specific shape assumption. `Hero::new(...).with_block(...)` stays available for arbitrary shapes.

### S8. Determinism & Stability

**Critical invariant**: authored IR must emit **identically** to a parsed equivalent. Concretely:

- If you build `FightUnit::new("Sniper", "Wooper").with_hp(3).with_sd(sd)` and emit, the output must be byte-identical to what the parser produces from that same source text.
- Default `body_order` (empty) triggers the canonical emit order in `fight_emitter.rs:60-76` — verified stable.
- Default shape flags (`outer_paren: false`, `head_paren: false`) emit the flat `Template.props` shape.

Verification: a test suite that builds IR via the authoring layer, emits it, re-parses, and asserts IR equality.

---

## Implementation Plan

### Checkpoint Configuration
- Total chunks: 7
- Checkpoint frequency: Every 2 chunks
- Critical checkpoints: After Chunk 3 (Face ID table correctness), Chunk 7 (roundtrip validation)

### Parallel Execution Map
- **Foundation (sequential)**: Chunk 1 (Default impls)
- **Parallel Group A** (after Chunk 1): Chunks 2, 3, 4 (Constructors, Face IDs, Sprite lookup — no cross-dependencies; each touches different modules)
- **Parallel Group B** (after Group A): Chunks 5, 6 (Builder setters, DiceFaces constructors + `dice!` macro — setters depend on Defaults, macro depends on Face IDs but not on setters)
- **Integration (sequential, after Group B)**: Chunk 7 (HeroReplica builder + roundtrip test suite)
- Minimum wall-clock rounds: **4** (vs 7 sequential)

---

### Chunk 1: `Default` impls for authorable IR types
**Scope**: Add `Default` to every type an author constructs.
**Files**: `compiler/src/ir/mod.rs`
**Dependencies**: None

**Requirements**:
- Add `#[derive(Default)]` where all fields already have `Default` impls (FightDefinition, DiceFaces).
- Add hand-written `impl Default` for types with required-looking fields that should default to empty (`FightUnit`, `HeroBlock`, `Hero`, `ReplicaItem`, `Monster`, `Boss`).
- Preserve existing `#[derive(...)]` ordering; add `Default` to the derive list where possible.
- No changes to serde behavior — all `#[serde(...)]` attributes unchanged.

**Verification**:
- [ ] `cargo build --all-targets` passes.
- [ ] `cargo test` — all existing tests pass.
- [ ] All 4 working mods still roundtrip 0-diff (run `cargo run --example roundtrip_diag`).
- [ ] `FightUnit::default()` produces a unit with empty template/name and all Options/flags unset.

---

### Chunk 2: Constructors (`::new`) [PARALLEL GROUP A]
**Scope**: Typed `new()` constructors taking required identity fields.
**Files**: `compiler/src/ir/mod.rs` (impl blocks next to each struct)
**Dependencies**: Chunk 1
**Parallel with**: Chunks 3, 4

**Requirements**:
- One `new()` per authorable type (see Spec §S2 for signatures).
- Accept `impl Into<String>` for string fields to allow `&str` or `String` callers.
- Every field not in the `new()` signature is set via `..Default::default()`.

**Verification**:
- [ ] Each constructor has a unit test constructing a value and asserting default fields.
- [ ] Doc comment on each `new()` shows a minimal usage example.
- [ ] No regressions in existing tests.

---

### Chunk 3: Face ID constants [PARALLEL GROUP A] [CRITICAL CHECKPOINT]
**Scope**: Typed `face_id::*` constants derived from `reference/textmod_guide.md` (semantics) and `working-mods/*.txt` (corpus of actual usage).
**Files**: `compiler/src/ir/face_id.rs` (generated), `compiler/src/ir/mod.rs` (add `pub mod face_id`), `compiler/build.rs` or `examples/derive_face_ids.rs` (the generator)
**Dependencies**: None
**Parallel with**: Chunks 2, 4

**Requirements**:
- Generator harvests every distinct Face ID from `.sd.` fields across the four working mods using the existing extractor; emits one `pub const` per ID.
- Each constant's doc-comment cites either (a) the guide section that defines its mechanic, or (b) the template-unit line(s) in working mods where it appears (if the guide is silent on that ID).
- Group by mechanic with section comments (Damage, Shield, Heal, Status, Combo, Utility, etc.) — categorization comes from the guide; uncategorized IDs go in an `// UNCLASSIFIED — appears in corpus, no guide entry` section.
- Constants are `u16` to match `DiceFace::Active::face_id`.
- Naming convention: derive from guide's mechanic name where possible; fall back to `FACE_<id>` for unclassified.

**Verification**:
- [ ] Every constant has either a guide-section citation or a working-mods corpus citation.
- [ ] `cargo doc` renders cleanly.
- [ ] Unit test: every constant appears in ≥1 working mod's `.sd.` field OR has an explicit guide-section citation.
- [ ] No values appear that are NOT in either source.
- [ ] Checkpoint: present the full constant list for user review before proceeding.

---

### Chunk 4: Sprite lookup table [PARALLEL GROUP A]
**Scope**: Compile-time lookup harvested from `working-mods/*.txt`.
**Files**: `compiler/src/sprite.rs` (generated), `compiler/src/lib.rs` (re-export), `compiler/build.rs` (generator)
**Dependencies**: None
**Parallel with**: Chunks 2, 3

**Requirements**:
- `build.rs` runs the extractor over each file in `working-mods/`, collects every `.img.` payload keyed by the entity's `.mn.` (or `.n.` if `.mn.` absent) name, deduplicates by name (last-write-wins per mod-priority order: sliceymon > pansaer > punpuns > community).
- Emits `compiler/src/sprite.rs` containing a `phf::Map<&'static str, &'static str>` (or `once_cell` HashMap if `phf` not already present) and the `lookup` / `lookup_expect` functions.
- `lookup_expect` panic message: `"unknown sprite: '{name}' — check working-mods/*.txt for entities exposing this name in .mn. or .n."`.
- Generator is deterministic — re-running on unchanged inputs produces byte-identical `sprite.rs`.

**Verification**:
- [ ] `lookup("Charmander")` returns the `.img.` payload of the Charmander entity in `working-mods/sliceymon.txt`.
- [ ] `lookup("nonexistent")` returns `None`.
- [ ] `lookup_expect` panics with an informative message on missing names.
- [ ] At least 3 sample entities (one each from sliceymon/pansaer/community) byte-match their working-mod `.img.` field.
- [ ] Re-running `build.rs` on unchanged inputs produces an unchanged `sprite.rs` (no spurious diff).

---

### Chunk 5: Builder-pattern setters [PARALLEL GROUP B]
**Scope**: Chainable `with_*` setters covering every optional field per Spec §S3.
**Files**: `compiler/src/ir/mod.rs` (impl blocks)
**Dependencies**: Chunks 1, 2
**Parallel with**: Chunk 6

**Requirements**:
- One setter per `Option<T>` field, named `with_{field}` (not `set_{field}`), taking `T` and wrapping in `Some` internally.
- For `Vec<T>` fields: `with_{field}s(Vec<T>)` and a singular `with_{field}(T)` that pushes.
- For bool flags: use descriptive verbs not `with_` — `wrap_outer()`, `wrap_head()`, `single_paren_nested()`, `removed()`, `bare()`.
- Every setter takes `mut self` and returns `Self` for chaining.
- Sprite-aware setters on `HeroBlock` / `Monster` / `ReplicaItem`: `with_sprite(name)` resolves via `sprite::lookup_expect` and sets both `sprite_name` and `img_data`.

**Verification**:
- [ ] Unit test per type demonstrating chained construction matches an equivalent struct literal.
- [ ] Emit-after-build for a FightUnit produces flat `Template.props` output (body_order empty → canonical fallback).
- [ ] Emit-after-build for a boss with `wrap_outer()` on each unit matches source wrapping style.

---

### Chunk 6: `DiceFaces::from_pairs` + `dice![]` macro [PARALLEL GROUP B]
**Scope**: Ergonomic dice construction.
**Files**: `compiler/src/ir/mod.rs` (impl), `compiler/src/ir/dice_macro.rs` (new) OR top-level `macro_rules!` in `lib.rs`
**Dependencies**: Chunks 1, 3 (macro accepts Face ID consts)
**Parallel with**: Chunk 5

**Requirements**:
- `DiceFaces::from_pairs(&[(u16, i16)])` — `(0, 0)` maps to `DiceFace::Blank`, else `Active`.
- `DiceFaces::from_faces(Vec<DiceFace>)` — direct vec wrapper.
- `dice!` macro expands `dice![ID-PIPS, _, ID-PIPS, _, _, ID-PIPS]` to `from_pairs`.
- Macro supports both integer literals AND ident constants (`DAMAGE-1` uses Chunk 3 constants).
- Macro enforces exactly 6 entries via a `compile_error!` on mismatch.
- Must be exportable: `pub use dice;` from `lib.rs` so callers `use slice_and_dice::dice;`.

**Verification**:
- [ ] `dice![34-1, _, _, _, _, _]` produces `faces: [Active(34,1), Blank, Blank, Blank, Blank, Blank]`.
- [ ] `dice![DAMAGE-1, SHIELD-2, _, _, _, _]` compiles when `face_id::{DAMAGE, SHIELD}` are in scope.
- [ ] `dice![34-1]` (5 entries) fails with a compile error, not a runtime panic.
- [ ] Negative pips: `dice![13--1, _, _, _, _, _]` → `Active(13, -1)`. (Verify emit: `"13--1:0:0:0:0:0"`.)
- [ ] Roundtrip: `DiceFaces::parse(&dice![...].emit()) == dice![...]`.

---

### Chunk 7: `HeroReplica` builder + authoring roundtrip tests [INTEGRATION]
**Scope**: Three-tier hero builder + the determinism test suite that certifies authored IR is indistinguishable from parsed IR.
**Files**: `compiler/src/ir/hero_replica.rs` (new), `compiler/tests/authoring_tests.rs` (new)
**Dependencies**: Chunks 1–6

**Requirements**:
- `HeroReplica::new(name, color)` → `.tier1(HeroBlock)` → `.tier2a/2b(HeroBlock)` → `.tier3a/3b(HeroBlock)` → `.build() -> Hero`.
- `build()` sets `format: HeroFormat::Sliceymon`, `source: Source::Custom`, assembles blocks in canonical order (T1, T2A, T2B, T3A, T3B).
- `build()` returns `Result<Hero, CompilerError>` — missing tiers are an error (all 5 required for Sliceymon shape).
- Authoring roundtrip test: for a representative hand-authored FightUnit, HeroBlock, Hero, Boss:
  1. Construct via authoring layer.
  2. Emit to textmod string.
  3. Re-parse that string.
  4. Assert the re-parsed IR equals the original authored IR.
- Verify against ONE real hero from `working-mods/sliceymon.txt` — extract it to IR, then re-author it via `HeroReplica`, and confirm the resulting IR is field-equal to the extracted version (and re-emits to the same textmod line).

**Verification**:
- [ ] `cargo test authoring` passes.
- [ ] All 4 working mods still roundtrip 0-diff (regression check).
- [ ] A one-screen Charmander example lives in `compiler/examples/author_hero.rs` showing the whole API in use.
- [ ] Checkpoint: present the example + test output for user review.

---

### Final Verification (After All Chunks)
- [ ] `cargo test --all` passes with no regressions.
- [ ] `cargo run --example roundtrip_diag -- ../working-mods/{pansaer,punpuns,sliceymon,community}.txt` — all 4 ROUNDTRIP OK, 0 diff.
- [ ] `cargo run --example author_hero` compiles and emits a valid hero line.
- [ ] A manually written Pokemon hero (≤30 lines of authoring code) emits a modifier that passes the Rust validator.
- [ ] One hero is written end-to-end using the new API as a proof-of-concept — presented to user for sign-off before we scale to the full roster. Pokemon choice and design come from the user, not from archived design docs.

---

## Out of Scope (future work)

- **Automatic balance validation during construction** — the validator already owns this; authoring layer only refuses obviously-invalid inputs (unknown sprite name, wrong dice arity).
- **JSON authoring schema for webapp** — the webapp will construct IR via serde from a separate schema. The authoring layer is Rust-callers-only.
- **Macro-based hero DSL** (`hero! { name: "Pikachu", tier1: { ... } }`) — the `.with_*()` chain is already terse enough. Reconsider only if the chain proves too verbose in practice.
- **Auto-generation from design doc markdown** — parsing `hero_designs_batch*.md` into `HeroReplica` calls. Possible future automation once the authoring API is stable.

## Risk & Mitigation

| Risk | Mitigation |
|------|-----------|
| Face ID constants drift from audit | Chunk 3 is a critical checkpoint; every constant cites its audit line. |
| Sprite JSON changes silently break builds | `include_str!` → compile fails if file is removed; unknown name panics loudly. |
| Builder produces IR that emits differently than parsed IR | Chunk 7 determinism suite catches this — fails loudly on any divergence. |
| Default values leak into parsed IR and mask bugs | `Default` is additive; parser still sets every field it always set. A parser regression stays visible because the roundtrip suite compares full IR. |
| `with_sprite` adds a second way to populate `img_data`, confusing readers | Doc comment + a single example in `compiler/examples/` establishing the canonical pattern. |
