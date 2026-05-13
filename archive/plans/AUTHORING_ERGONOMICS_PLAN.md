# Authoring Ergonomics — Spec & Plan

## Overview

The compiler can parse and roundtrip the four working mods; the next limiting factor is authoring cost. Constructing IR values today requires 15–30-line struct literals per unit with every `Option` field, every shape flag (`head_paren`, `outer_paren`, `nested_single_paren`, `part`, `post_override_keywords`, `body_order`), and every hand-managed default written out. That cost compounds across ~100 heroes, their replicas, bosses, and monsters.

This plan delivers the author-facing surface of `compiler/src/authoring/` — the **only supported path** from human or LLM intent to a `ModIR` value (SPEC §4, §5, §6.1). It is a typed frontend over the IR: chainable builders for every authorable type, a `dice![]` macro for hand-authored dice, a three-tier Sliceymon hero-replica builder, and a Path B determinism test suite that proves authored IR is indistinguishable from parsed IR.

The roundtrip guarantee holds because the authoring layer and the extractor produce the same IR values: builders set the same typed fields the extractor populates, using the same `FaceId` / `SpriteId` newtypes. Downstream consumers (webapp, CLI JSON input via serde) deserialize into the same typed IR.

Foundational prerequisites — `Default` impls, `::new(identity)` constructors, `FaceId` / `SpriteId` newtypes, self-contained IR, the `authoring/` module skeleton — are delivered by `PLATFORM_FOUNDATIONS_PLAN.md` and are assumed in place before this plan begins.

---

## Goals

1. Stand up the author-facing contents of `compiler/src/authoring/` — builders, `dice!` macro, `HeroReplica` — atop the foundations-supplied skeleton and newtypes (SPEC §4, §5, §6.1).
2. Make "add a Pokemon hero from a design doc" a one-screen task, not a one-hour task.
3. Unlock Path B (author-from-scratch → build → re-extract → equal) as a first-class test (SPEC §4).
4. Cover every new IR variant introduced by `PIPELINE_FIDELITY_PLAN.md` Phase 1 with an authoring constructor in the same PR.
5. Preserve IR-equivalence roundtrip (SPEC §3.1) for all four working mods.

## Non-Goals

- No authoring DSL / macro gymnastics beyond one small `dice![]` macro.
- No persistence layer — this is in-memory construction ergonomics only. Serde/JSON I/O is already typed via foundations' newtype `try_from` deserializers.
- No auto-balance checks. Structural validation is the extract/build pipeline; cross-IR semantic checks live in `compiler/src/xref.rs`. Balance heuristics are out of scope.
- Not a replacement for `PIPELINE_FIDELITY_PLAN.md`: drift-class fixes and roundtrip regressions live there. This plan owns the authoring surface.
- Not a replacement for `PLATFORM_FOUNDATIONS_PLAN.md`: newtype definitions, IR defaults, module skeleton, and sprite/face-id generators live there.

---

## Spec

### S1. Builder-pattern setters

Chainable `with_*` setters for every optional field on every authorable IR type. Setters consume `self` and return `Self` so calls compose without intermediate bindings. Optional fields wrap inputs in `Some(...)` internally — the caller never writes `Some(...)`.

```rust
FightUnit::new("Sniper", "Wooper")
    .with_hp(6)
    .with_sd(dice![DAMAGE-1, DAMAGE_PHYS-1, _, _, DAMAGE_PHYS-1, _])
    .with_sprite("wooper")?
    .with_color('a')
    .with_doc("A mud fish");
```

Naming:

- One setter per `Option<T>` field, named `with_{field}`, taking `T` and wrapping internally.
- For `Vec<T>` fields: `with_{field}s(Vec<T>)` and a singular `with_{field}(T)` that pushes.
- For bool flags, use descriptive verbs instead of `with_`: `wrap_outer()`, `wrap_head()`, `single_paren_nested()`, `removed()`, `bare()`.
- Sprite-aware setters resolve via `SpriteId::try_new` and return `Result<Self, CompilerError>`: `.with_sprite(name)` on `HeroBlock`, `Monster`, `ReplicaItem`, `AbilityData`, `TriggerHpDef`, `FightUnit` builders. There is no `with_sprite_name(raw)` or `with_img_data(raw)` escape hatch.

Setter coverage per type:

- **FightUnit**: `with_hp`, `with_sd`, `with_sprite`, `with_template_override`, `with_doc`, `with_chain`, `with_color`, `with_hsv`, `with_nested_units`, `with_part`, `with_post_override_keywords`, `wrap_outer`, `wrap_head`, `single_paren_nested`.
- **HeroBlock**: `with_hp`, `with_tier`, `with_color`, `with_doc`, `with_abilitydata`, `with_triggerhpdata`, `with_hue`, `with_chain`, `with_sprite`, `with_speech`, `bare()`.
- **Hero**: `with_format`, `with_block`, `with_blocks`, `removed()`, `from_custom()`.
- **ReplicaItem**: `with_hp`, `with_tier`, `with_color`, `with_doc`, `with_speech`, `with_abilitydata`, `with_item_modifiers`, `with_sticker`, `with_toggle_flags`, `with_sprite`.
- **Monster**: `with_hp`, `with_sd`, `with_sprite`, `with_color`, `with_doc`, `with_chain`, `with_balance`.
- **Boss**: `with_format`, `with_encounter_id`, `with_fight`, `with_fights`, `with_doc`, `with_chain`, `with_event_phases`.
- **FightDefinition**: `with_level`, `with_enemy`, `with_enemies`, `with_name`, `with_trigger`.

### S2. `DiceFaces` ergonomic constructors

Two layers, both `FaceId`-typed:

```rust
// Layer 1 — explicit typed pairs (useful for programmatic construction)
DiceFaces::from_pairs(&[
    (FaceId::DAMAGE, Pips::from(1)),
    (FaceId::SHIELD, Pips::from(2)),
]);

// Layer 2 — explicit DiceFace vec (when you have them already)
DiceFaces::from_faces(vec![
    DiceFace::Active { face_id: FaceId::DAMAGE, pips: Pips::from(1) },
    DiceFace::Blank,
    // ...
]);
```

The `FaceId` whitelist is already enforced by the newtype; no sentinel-based blank detection. Blanks are explicit via `DiceFace::Blank` or the macro's `_` syntax.

### S3. `dice![]` macro

Ergonomic hand-authoring:

```rust
dice![DAMAGE-1, SHIELD-2, _, _, DAMAGE-1, _]
// expands to DiceFaces::from_pairs-equivalent
```

Rules:

- `PATH-PIPS` for active faces. `PATH` must resolve to `FaceId` (e.g., `FaceId::DAMAGE`, or `DAMAGE` via a prelude glob). Positive or negative pips.
- `PATH` bare (no `-PIPS`) defaults to pips `0`.
- `_` for blank faces.
- Exactly 6 entries per die — `compile_error!` on mismatch.
- **Bare integer literals are rejected at macro expansion** (`dice![34-1]` → `compile_error!`). Accepting them would bypass the whitelist. Any code that needs to build a `FaceId` from a parsed `u16` must call `FaceId::try_new(raw)?` explicitly; the macro never admits it implicitly.

Exported from `lib.rs` as `pub use authoring::dice;` so callers `use slice_and_dice::dice;`.

### S4. `HeroReplica` — three-tier Sliceymon builder

The Sliceymon three-tier replica shape is repetitive enough to deserve its own type:

```rust
HeroReplica::new("Pikachu", 'a')
    .tier1(HeroBlock::new("t1tmpl", "Pichu", sprite!("pichu"), dice![...]).with_hp(6))
    .tier2a(HeroBlock::new("t2tmpl", "Pikachu", sprite!("pikachu"), dice![...]).with_hp(12))
    .tier2b(HeroBlock::new(...))
    .tier3a(HeroBlock::new(...).with_hp(20))
    .tier3b(HeroBlock::new(...))
    .build()?   // -> Result<Hero, CompilerError>
```

`build()` assembles the 5 blocks in canonical Sliceymon order (T1, T2A, T2B, T3A, T3B), sets `format: HeroFormat::Sliceymon`, fills `internal_name` / `mn_name` / `color`, and marks `source: Source::Custom`. Returns `Err(CompilerError)` on missing tiers with `field_path` + `suggestion`.

`HeroReplica` is a separate type, not a method on `Hero`, because it encodes a specific shape assumption. `Hero::new(...).with_block(...)` remains available for arbitrary shapes.

### S5. New-variant constructor coverage

For every IR variant introduced by `PIPELINE_FIDELITY_PLAN.md` Phase 1, this plan ships an authoring constructor in the same PR as the variant:

- `HeropoolEntry::{BareName, ReplicaBlock}`
- `ItemPoolEntry::EntityRef`
- `MonsterPoolEntry::{EntityRef, BareTemplate, ReplicaBlock}`
- `AddModifier`
- `HeroBlock::BlockWrapper` variants
- `FightUnitEntry` shape

No IR variant merges without its authoring constructor. Direct struct-literal construction of these variants is unsupported per SPEC §6.1.

### S6. Determinism & stability

Authored IR must emit identically to a parsed equivalent:

- `FightUnit::new("Sniper", "Wooper").with_hp(3).with_sd(sd)` emitted and compared byte-wise must match the parser's output on that same source text.
- Default `body_order` (empty) triggers the canonical emit order in `fight_emitter.rs`.
- Default shape flags (`outer_paren: false`, `head_paren: false`) emit the flat `Template.props` shape.

Verification is the Path B test suite (Chunk 3): build IR via the authoring layer, emit, re-parse, assert IR equality.

---

## Module layout

```
compiler/src/authoring/
├── mod.rs                 # prelude + re-exports (skeleton from foundations)
├── face_id.rs             # FaceId + whitelist consts (from foundations)
├── sprite.rs              # SpriteId + phf registry (from foundations)
├── dice.rs                # dice![] macro + DiceFaces ergonomic ctors  [THIS PLAN]
├── builders.rs            # chainable with_* setters per IR type       [THIS PLAN]
└── hero_replica.rs        # Sliceymon three-tier builder                [THIS PLAN]
```

Public surface re-exported from `lib.rs`:

```rust
pub use authoring::{dice, HeroReplica};
pub use authoring::prelude::*;   // FaceId associated-const glob, builder traits
```

Direct struct-literal construction of `Hero`, `HeroBlock`, `ReplicaItem`, `Monster`, `Boss`, `DiceFace` remains possible in Rust but is unsupported per SPEC §6.1. Each gets a module-level doc comment pointing at the authoring API.

---

## Implementation Plan

### Checkpoint Configuration
- Total chunks: 3
- Checkpoint frequency: after Chunk 3 (final).

### Parallel Execution Map
- **Parallel Group A**: Chunk 1 (builder setters) and Chunk 2 (`dice!` macro + `DiceFaces` ctors) — different files, no cross-deps.
- **Sequential integration**: Chunk 3 (`HeroReplica` + Path B tests) after Group A.

Minimum wall-clock rounds: 2.

---

### Chunk 1: Builder-pattern setters [PARALLEL GROUP A]
**Spec**: §S1
**Files**: `compiler/src/authoring/builders.rs` (new), `compiler/src/authoring/mod.rs` (re-export)
**Dependencies**: `PLATFORM_FOUNDATIONS_PLAN.md` Chunks 1–4 complete (Defaults, `::new`, `FaceId`, `SpriteId`).
**Parallel with**: Chunk 2.

**Requirements**:
- Implement every setter per §S1 coverage list.
- All setters take `mut self` and return `Self`.
- Sprite-aware setters (`with_sprite`) return `Result<Self, CompilerError>`; no raw-name / raw-img-data escape hatches.
- Constructor coverage for every `PIPELINE_FIDELITY_PLAN.md` Phase 1 variant listed in §S5 — ships alongside the IR variant in the same PR.

**Verification**:
- [ ] Unit test per authorable type demonstrating chained construction produces the same IR as a reference struct literal.
- [ ] Emit-after-build for a FightUnit with default shape flags produces the flat `Template.props` form.
- [ ] Emit-after-build for a boss with `wrap_outer()` on each unit matches source wrapping style.
- [ ] All 4 working mods IR-equal roundtrip (regression check).

---

### Chunk 2: `DiceFaces` ctors + `dice![]` macro [PARALLEL GROUP A]
**Spec**: §S2, §S3
**Files**: `compiler/src/authoring/dice.rs` (new), `compiler/src/authoring/mod.rs` (re-export), `compiler/src/lib.rs` (`pub use authoring::dice;`)
**Dependencies**: `PLATFORM_FOUNDATIONS_PLAN.md` Chunk 2 complete (`FaceId` newtype).
**Parallel with**: Chunk 1.

**Requirements**:
- `DiceFaces::from_pairs(&[(FaceId, Pips)])`, `DiceFaces::from_faces(Vec<DiceFace>)`.
- `dice![PATH-PIPS, _, PATH-PIPS, _, _, PATH-PIPS]` expansion per §S3 rules.
- Macro rejects bare integer literals with `compile_error!`.
- Macro enforces exactly 6 entries with `compile_error!`.
- Macro exported as `pub use authoring::dice;` from `lib.rs`.

**Verification**:
- [ ] `dice![DAMAGE-1, _, _, _, _, _]` produces `faces: [Active(DAMAGE, 1), Blank×5]`.
- [ ] `dice![FaceId::DAMAGE-1, FaceId::SHIELD-2, _, _, _, _]` compiles without any prelude import.
- [ ] `dice![DAMAGE-1, SHIELD-2, _, _, _, _]` compiles with `use slice_and_dice::authoring::prelude::*;`.
- [ ] `dice![34-1]` (bare integer) → `compile_error!`.
- [ ] `dice![DAMAGE-1]` (5 entries) → `compile_error!`.
- [ ] Negative pips: `dice![FACE_13--1, _, _, _, _, _]` → `Active(FACE_13, -1)`; emit produces `"13--1:0:0:0:0:0"`.
- [ ] Roundtrip: `DiceFaces::parse(&emit(&dice![...])) == dice![...]`.

---

### Chunk 3: `HeroReplica` + Path B test suite [INTEGRATION]
**Spec**: §S4, §S5, §S6
**Files**: `compiler/src/authoring/hero_replica.rs` (new), `compiler/tests/path_b/` (new directory)
**Dependencies**: Chunks 1, 2.

**Requirements**:
- `HeroReplica::new(name, color)` → `.tier1/2a/2b/3a/3b(HeroBlock)` → `.build() -> Result<Hero, CompilerError>`.
- `build()` sets `format: HeroFormat::Sliceymon`, `source: Source::Custom`, assembles blocks in canonical order (T1, T2A, T2B, T3A, T3B).
- `build()` returns `Err(CompilerError)` on any missing tier, with `field_path` + `suggestion`.
- Path B test suite: for a representative hand-authored `FightUnit`, `HeroBlock`, `Hero`, `Boss`, `Monster`, `ReplicaItem` (each `ReplicaItemKind`), plus every Phase 1 IR variant:
  1. Construct via authoring layer (no `extract` call).
  2. `build(&ir)` → textmod string.
  3. `extract(&text)` → re-parsed IR.
  4. Assert `re_parsed == original` as semantic IR equality (SPEC §3.1).
- Verify against one real hero from `working-mods/sliceymon.txt`: extract → re-author via `HeroReplica` → assert the re-authored IR is field-equal to the extracted IR.
- One-screen Charmander example at `compiler/examples/author_hero.rs` exercising the full API.

**Verification**:
- [ ] `cargo test --test path_b` passes.
- [ ] All 4 working mods IR-equal roundtrip (shared gate with `PIPELINE_FIDELITY_PLAN.md` Phase 4.1).
- [ ] `cargo run --example author_hero` emits a valid hero line; re-extracting produces IR equal to the authored input.
- [ ] Present the example + test output for user review.

---

### Final Verification

- [ ] `cargo test --all` passes.
- [ ] `cargo test --test path_b` passes.
- [ ] IR-equal roundtrip holds for all 4 working mods.
- [ ] No `std::fs` / `std::process` in `compiler/src/authoring/`.
- [ ] No `unwrap()` / `expect()` / `panic!` in `compiler/src/authoring/`.
- [ ] A manually written Pokemon hero (≤30 lines of authoring code) emits a modifier that passes `cargo run -- check --round-trip`.
- [ ] One hero is written end-to-end using the new API as a proof-of-concept — presented to user for sign-off before scaling to the full roster. Pokemon choice and design come from the user, not from archived design docs.

---

## Out of Scope (future work)

- **Automatic balance validation during construction** — SPEC §6.4 game-design rules are enforced in `xref`, not in the authoring layer. Authoring refuses only structurally-invalid inputs (unknown `FaceId` / `SpriteId`, wrong dice arity, missing required Sliceymon tiers).
- **Webapp JSON authoring schema** — the webapp constructs IR via serde + `schemars`-derived schema. Foundations' typed newtype `try_from` keeps webapp input equally safe; no separate schema is needed.
- **Macro-based hero DSL** (`hero! { name: "Pikachu", tier1: { ... } }`) — the `.with_*()` chain is terse enough. Reconsider only if the chain proves too verbose in practice.
- **Auto-generation from design-doc markdown** — possible future automation once the authoring API is stable.

## Risk & Mitigation

| Risk | Mitigation |
|------|-----------|
| Builder produces IR that emits differently than parsed IR | Chunk 3 Path B suite catches this — fails loudly on any `extract(build(ir)) != ir` divergence (SPEC §4). |
| Callers bypass authoring layer via struct literals, reintroducing hallucination | Module-level docs on IR types point at authoring API; foundations' newtypes (`FaceId` / `SpriteId`) already reject invalid values through private tuple inner fields and serde `try_from`. |
| A new Phase 1 IR variant merges without its authoring constructor | §S5 coverage list is enforced in code review; Path B fixture for the variant fails to compile until its authoring constructor exists. |
