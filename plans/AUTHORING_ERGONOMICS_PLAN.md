# Authoring Ergonomics — Spec & Plan

## Overview

The compiler can parse and roundtrip the four working mods; the next limiting factors are (a) SPEC §3.6's "make invalid states unrepresentable" invariant — hallucinated Face IDs and sprite names compile cleanly today — and (b) authoring cost: constructing IR values requires 15–30-line struct literals per unit, with every `Option` field, every shape flag (`head_paren`, `outer_paren`, `nested_single_paren`, `part`, `post_override_keywords`, `body_order`), and every raw `u16` Face ID written by hand. That cost compounds across ~100 heroes, their replicas, bosses, and monsters.

This plan delivers the `authoring/` module SPEC §4 and §5 require: the **only supported path** from human/LLM intent to a `ModIR` value. It introduces `FaceId` and `SpriteId` **newtypes** (not bare `pub const u16` constants) with fallible whitelist constructors, typed builders for every authorable IR type, a `dice![]` macro, and a three-tier hero-replica builder. Because SPEC §3.6 requires invalid states to be unrepresentable, IR types also change: `DiceFace::Active { face_id: FaceId, pips: Pips }` and `sprite_name: SpriteId` replace the current raw-typed fields. This is not a "thin layer" — it is a typed frontend plus the IR changes that make the typing load-bearing.

The roundtrip guarantee is preserved because the authoring layer and the extractor produce the same IR values: the extractor calls the same `FaceId::try_new(raw)` and `SpriteId::lookup(name)` constructors on its way into the IR. A mod that contains a face ID absent from the whitelist is a `CompilerError` at extract time — SPEC §5 says extraction never produces a partial `ModIR`.

Downstream consumers (webapp, CLI JSON input via serde) deserialize into the same typed IR; serde_with + custom deserializers on `FaceId`/`SpriteId` validate at deserialize time. There is no path into a `ModIR` that skips whitelist validation.

---

## Goals

1. Make invalid states unrepresentable (SPEC §3.6): hallucinated Face IDs, sprite names, and dice arities are compile errors or `CompilerError`s — never silent runtime bugs.
2. Stand up the `compiler/src/authoring/` module as the only supported path from intent to `ModIR` (SPEC §4, §5, §6.1).
3. Make "add a Pokemon hero from a design doc" a one-screen task, not a one-hour task.
4. Preserve IR-equivalence roundtrip (SPEC §3.1) for all four working mods, and unlock Path B (author-from-scratch → build → re-extract → equal) as a first-class test (SPEC §4).
5. Cover every new IR variant introduced by `PIPELINE_FIDELITY_PLAN.md` Phase 1 with an authoring constructor in the same PR (that plan's §1b.1 points here).

## Non-Goals

- No authoring DSL / macro gymnastics beyond one small `dice![]` macro.
- No persistence layer — this is in-memory construction ergonomics only. Serde/JSON I/O stays as-is except for the typed deserializers `FaceId` / `SpriteId` need.
- No auto-balance checks. Structural validation is the extract/build pipeline; cross-IR semantic checks live in `compiler/src/xref.rs`. Balance heuristics are out of scope here.
- Not a replacement for `PIPELINE_FIDELITY_PLAN.md`: the drift-class fixes and roundtrip regressions live there. This plan owns the authoring surface; that plan owns parser/emitter correctness. Shared obligations are called out by cross-reference.

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

### S5. `FaceId` newtype (replaces raw `u16` in IR)

SPEC §3.6 requires `FaceId` to be a newtype with a whitelist constructor — "constructing an invalid `FaceId` is a compile error, not a runtime validation failure." Bare `pub const DAMAGE: u16 = 34` does not satisfy this: nothing stops a caller from passing `3400`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "u16", into = "u16")]
pub struct FaceId(u16);

impl FaceId {
    pub const fn try_new(raw: u16) -> Result<Self, CompilerError> { /* whitelist check */ }
    pub const fn raw(self) -> u16 { self.0 }

    // Named associated constants, whitelist-backed. These are `Self`, not `u16`.
    pub const DAMAGE: Self        = Self(34);
    pub const DAMAGE_PHYS: Self   = Self(30);
    pub const SHIELD: Self        = Self(42);
    // ... grouped by mechanic, generated in Chunk 3.
}
```

`DiceFace::Active` uses the newtype directly (IR change, shared with `PIPELINE_FIDELITY_PLAN.md` Phase 0.6):

```rust
pub enum DiceFace {
    Blank,
    Active { face_id: FaceId, pips: Pips },
}
```

Callers:

```rust
use slice_and_dice::authoring::face_id::FaceId;
dice![FaceId::DAMAGE-1, FaceId::SHIELD-2, _, _, FaceId::DAMAGE-1, _]
// or, with a glob import of the associated consts re-exported from the authoring prelude:
dice![DAMAGE-1, SHIELD-2, _, _, DAMAGE-1, _]
```

The `dice!` macro accepts paths that resolve to `FaceId` (associated consts or local bindings). Bare integer literals are **rejected at macro expansion** — accepting `34` would bypass the whitelist. Extractor internals that need to build a `FaceId` from a parsed u16 call `FaceId::try_new(raw)?` explicitly; the macro never admits it implicitly.

The extractor's `.sd.` parser calls `FaceId::try_new(raw)?` on every face; an ID absent from the whitelist is a `CompilerError` (SPEC §5 forbids partial `ModIR`). Serde's `try_from = "u16"` enforces the same on JSON input — there is no path into a `ModIR` that skips whitelist validation.

**Source of truth**: `reference/textmod_guide.md` (face-mechanic semantics) + `working-mods/*.txt` (corpus of actual usage). Every associated constant's doc-comment cites either (a) the guide section that defines the face mechanic, or (b) a working-mods unit where the face appears in `.sd.` (template name + line). A unit test enforces that every `FaceId::*` constant appears in ≥1 working mod's `.sd.` field OR has an explicit guide-section citation.

**Cross-plan obligation**: the IR flip from `face_id: u16` → `face_id: FaceId` lands in the same PR as `PIPELINE_FIDELITY_PLAN.md` Phase 0.6. Neither change ships alone.

### S6. `SpriteId` newtype + registry

SPEC §3.3 requires the IR to be self-contained (every sprite owns its `img_data`), and §3.6 requires typed construction. SPEC §5 sketches `sprite(name: &str) -> Result<SpriteId, CompilerError>`.

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String", into = "String")]
pub struct SpriteId {
    name: &'static str,    // interned from the registry
    img_data: &'static str, // base64 payload from the registry
}

impl SpriteId {
    pub fn try_new(name: &str) -> Result<Self, CompilerError> { /* registry lookup */ }
    pub fn name(&self) -> &str { self.name }
    pub fn img_data(&self) -> &str { self.img_data }
}
```

`HeroBlock`, `ReplicaItem`, `Monster`, and every fight-unit / ability-data type that carries a sprite use `sprite: SpriteId` instead of the current `sprite_name: String` + `img_data: Option<String>`. One typed field replaces two untyped ones — per SPEC §3.7, no parallel representations (the `sprite_name: String` field and the `img_data: Option<String>` field both go away in the same PR; see `PIPELINE_FIDELITY_PLAN.md` Phase 0.1).

The registry is a `build.rs`-generated `phf::Map<&'static str, &'static str>` harvested from `working-mods/*.txt`. Unknown names are `CompilerError` at construction time — both from authoring (`SpriteId::try_new`) and from extractor / serde deserialization. Path B can construct a `SpriteId` via the registry without any other files.

Builders expose `.with_sprite("pikachu")` as the sole path; there is no `with_sprite_name(raw)` escape hatch — that would reintroduce the hallucination class.

**Cross-plan obligation**: IR field consolidation (`sprite_name + img_data` → `sprite: SpriteId`) lands in the same PR as `PIPELINE_FIDELITY_PLAN.md` Phase 0.1.

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

## Module layout

All authoring code lives under `compiler/src/authoring/`, per SPEC §4 module boundaries. `ir/mod.rs` stays focused on data definitions; IR types may gain `#[derive(Default)]` and small `new()` constructors that take identity fields only, but **chainable builders, `dice!`, `FaceId`/`SpriteId` newtype definitions, the sprite registry, and `HeroReplica` live in `authoring/`**, not `ir/`.

```
compiler/src/
├── ir/
│   ├── mod.rs                 # data types + minimal ::new(identity) + Default
│   └── ...
├── authoring/
│   ├── mod.rs                 # prelude + re-exports
│   ├── face_id.rs             # `FaceId` newtype + whitelist consts (generated)
│   ├── sprite.rs              # `SpriteId` newtype + phf registry (generated by build.rs)
│   ├── dice.rs                # `dice![]` macro + DiceFaces ergonomic ctors
│   ├── builders.rs            # chainable with_* setters per IR type
│   └── hero_replica.rs        # Sliceymon three-tier builder
```

Public surface is re-exported from `lib.rs`:

```rust
pub mod authoring;
pub use authoring::{FaceId, SpriteId, dice, HeroReplica};
pub use authoring::prelude::*;   // consts, builder methods, face_id::*
```

Direct struct-literal construction of `Hero`, `HeroBlock`, `ReplicaItem`, `Monster`, `Boss`, `DiceFace`, `FaceId`, `SpriteId` remains possible in Rust but is **unsupported** per SPEC §6.1. Each of these types gets a module-level doc comment pointing at the authoring API and noting that raw struct literals bypass whitelist validation. Where feasible without breaking the extractor's own construction path, make fields `pub(crate)` and only expose them via `authoring::`.

## Implementation Plan

### Checkpoint Configuration
- Total chunks: 7
- Checkpoint frequency: Every 2 chunks
- Critical checkpoints: After Chunk 3 (FaceId newtype + whitelist), Chunk 7 (roundtrip validation)

### Parallel Execution Map
- **Foundation (sequential)**: Chunk 1 (Default impls + `authoring/` module skeleton)
- **Parallel Group A** (after Chunk 1): Chunks 2, 3, 4 (Constructors, `FaceId` newtype, `SpriteId` newtype + registry — different files; Chunks 3 and 4 each include IR-field flips coordinated with `PIPELINE_FIDELITY_PLAN.md` Phase 0.1/0.6)
- **Parallel Group B** (after Group A): Chunks 5, 6 (Builder setters, `dice!` macro + DiceFaces ctors — setters depend on Defaults, macro depends on `FaceId` newtype)
- **Integration (sequential, after Group B)**: Chunk 7 (HeroReplica builder + Path B roundtrip test suite)
- Minimum wall-clock rounds: **4** (vs 7 sequential)

---

### Chunk 1: `Default` impls for authorable IR types + `authoring/` module skeleton
**Scope**: Add `Default` to every type an author constructs, and stand up the empty `compiler/src/authoring/` module that subsequent chunks populate.
**Files**: `compiler/src/ir/mod.rs`, `compiler/src/authoring/mod.rs` (new, mostly empty), `compiler/src/lib.rs` (`pub mod authoring;`)
**Dependencies**: None

**Requirements**:
- Add `#[derive(Default)]` where all fields already have `Default` impls (FightDefinition, DiceFaces).
- Add hand-written `impl Default` for types with required-looking fields that should default to empty (`FightUnit`, `HeroBlock`, `Hero`, `ReplicaItem`, `Monster`, `Boss`). Types whose fields become newtypes in Chunks 3/4 (`SpriteId`, `FaceId`) do not get `Default` — there is no safe empty whitelist entry.
- Preserve existing `#[derive(...)]` ordering; add `Default` to the derive list where possible.
- No changes to serde behavior — all `#[serde(...)]` attributes unchanged.
- Create `compiler/src/authoring/mod.rs` with a module doc comment citing SPEC §6.1 ("only supported path from human/LLM intent to an IR value"). Re-exports populate in later chunks.

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

### Chunk 3: `FaceId` newtype + whitelist [PARALLEL GROUP A] [CRITICAL CHECKPOINT]
**Scope**: `FaceId` newtype with fallible constructor, whitelist-backed associated consts, IR flip from `u16` → `FaceId`. Coordinates with `PIPELINE_FIDELITY_PLAN.md` Phase 0.6 — ships in one PR.
**Files**: `compiler/src/authoring/face_id.rs` (generated), `compiler/src/authoring/mod.rs` (re-export), `compiler/src/ir/mod.rs` (`DiceFace::Active { face_id: FaceId, pips: Pips }`), `compiler/src/lib.rs` (re-export), `compiler/build.rs` (generator), every extractor/emitter/xref callsite (flip raw u16 ↔ FaceId)
**Dependencies**: None (generator), but lands with IR change
**Parallel with**: Chunks 2, 4

**Requirements**:
- Define `pub struct FaceId(u16)` with `try_new(raw: u16) -> Result<Self, CompilerError>`, `raw(self) -> u16`, `const fn` where feasible, and `#[serde(try_from = "u16", into = "u16")]`.
- Generator harvests every distinct Face ID from `.sd.` fields across the four working mods using the existing extractor; emits one `pub const NAME: Self = Self(N);` per ID on the `FaceId` inherent impl.
- Each constant's doc-comment cites either (a) the guide section that defines its mechanic, or (b) the template-unit line(s) in working mods where it appears (if the guide is silent on that ID).
- Group by mechanic with section comments (Damage, Shield, Heal, Status, Combo, Utility, etc.) — categorization comes from the guide; uncategorized IDs go in an `// UNCLASSIFIED — appears in corpus, no guide entry` section.
- Flip every IR field and extractor/emitter/xref callsite from `u16` to `FaceId` in the same PR (SPEC §3.7: no parallel `face_id_raw` alongside `face_id`). The extractor calls `FaceId::try_new(parsed_u16)?`; unknown IDs become `CompilerError` with `field_path` + `suggestion` (e.g., "Valid Face IDs for Fey: 15, 32, 34, ...").
- `xref` rule `X016` (face-template compatibility) operates on `FaceId` values; message lists allowed consts by name.
- Naming convention: derive from guide's mechanic name where possible; fall back to `FACE_<id>` for unclassified.

**Verification**:
- [ ] Every associated const has either a guide-section citation or a working-mods corpus citation.
- [ ] `cargo doc` renders cleanly.
- [ ] Unit test: every `FaceId::*` const appears in ≥1 working mod's `.sd.` field OR has an explicit guide-section citation.
- [ ] No values appear that are NOT in either source.
- [ ] Compile-fail test: `FaceId(3400)` outside the whitelist is unreachable from safe code; `FaceId::try_new(3400)` returns `CompilerError`.
- [ ] All 4 working mods still extract + IR-equal roundtrip after the flip (Path A regression; shared gate with `PIPELINE_FIDELITY_PLAN.md` Phase 4.1).
- [ ] Checkpoint: present the full associated-const list for user review before proceeding.

---

### Chunk 4: `SpriteId` newtype + registry [PARALLEL GROUP A]
**Scope**: `SpriteId` newtype that bundles name + img_data, replacing raw `sprite_name: String` + `img_data: Option<String>`. Coordinates with `PIPELINE_FIDELITY_PLAN.md` Phase 0.1 — ships in one PR.
**Files**: `compiler/src/authoring/sprite.rs` (generated), `compiler/src/authoring/mod.rs` (re-export), `compiler/src/ir/mod.rs` (flip `HeroBlock`/`ReplicaItem`/`Monster`/`AbilityData`/`TriggerHpDef`/fight-unit sprite fields), `compiler/src/lib.rs` (re-export), `compiler/build.rs` (generator), every extractor/emitter callsite
**Dependencies**: None (generator), but lands with IR change
**Parallel with**: Chunks 2, 3

**Requirements**:
- Define `pub struct SpriteId { name: &'static str, img_data: &'static str }` backed by a compile-time `phf::Map<&'static str, SpriteId>`. Accessors `name()`, `img_data()`. No `Default`.
- `SpriteId::try_new(name: &str) -> Result<SpriteId, CompilerError>` returns the interned entry or `CompilerError` with actionable `suggestion` ("Known sprite names include: …").
- `#[serde(try_from = "String", into = "String")]` so JSON authoring validates at deserialize time.
- `build.rs` runs the extractor over each file in `working-mods/`, collects every `.img.` payload keyed by the entity's `.mn.` (or `.n.` if `.mn.` absent) name, deduplicates by name (last-write-wins per mod-priority order: sliceymon > pansaer > punpuns > community). Emits a deterministic `phf::Map`.
- Generator is deterministic — re-running on unchanged inputs produces byte-identical `sprite.rs`.
- Flip every IR field: `sprite_name: String` + `img_data: Option<String>` → `sprite: SpriteId` (one typed field replaces two untyped — SPEC §3.7). Every extractor/emitter/xref callsite updates in one pass.
- Drop the external sprite `HashMap` parameter from `build` / `build_with` / `build_complete` (same PR as `PIPELINE_FIDELITY_PLAN.md` Phase 0.1). The IR is self-contained after this chunk lands.
- Typed setters on builders expose `.with_sprite("pikachu")` only; there is no `with_sprite_name(raw)` escape hatch.

**Verification**:
- [ ] `SpriteId::try_new("Charmander").unwrap().img_data()` returns the `.img.` payload of the Charmander entity in `working-mods/sliceymon.txt`.
- [ ] `SpriteId::try_new("nonexistent")` returns `CompilerError` with an informative `suggestion`.
- [ ] At least 3 sample entities (one each from sliceymon/pansaer/community) byte-match their working-mod `.img.` field.
- [ ] `build(ir)` compiles and runs without any external sprite map.
- [ ] All 4 working mods still extract + IR-equal roundtrip after the flip (Path A regression; shared gate with `PIPELINE_FIDELITY_PLAN.md` Phase 4.1).
- [ ] Re-running `build.rs` on unchanged inputs produces an unchanged `sprite.rs` (no spurious diff).

---

### Chunk 5: Builder-pattern setters [PARALLEL GROUP B]
**Scope**: Chainable `with_*` setters covering every optional field per Spec §S3, plus constructors for **every IR variant introduced by `PIPELINE_FIDELITY_PLAN.md` Phase 1** (`HeropoolEntry::{BareName, ReplicaBlock}`, `ItemPoolEntry::EntityRef`, `MonsterPoolEntry::{EntityRef, BareTemplate, ReplicaBlock}`, `AddModifier`, `FightUnitEntry`, boss phase fields, `ReplicaItemKind::{Capture, Legendary}`). No new variant merges without its authoring constructor.
**Files**: `compiler/src/authoring/builders.rs` (new — setters live here, not in `ir/mod.rs`), `compiler/src/authoring/mod.rs` (re-export)
**Dependencies**: Chunks 1, 2; synchronous with `PIPELINE_FIDELITY_PLAN.md` Phase 1 for new-variant coverage
**Parallel with**: Chunk 6

**Requirements**:
- One setter per `Option<T>` field, named `with_{field}` (not `set_{field}`), taking `T` and wrapping in `Some` internally.
- For `Vec<T>` fields: `with_{field}s(Vec<T>)` and a singular `with_{field}(T)` that pushes.
- For bool flags: use descriptive verbs not `with_` — `wrap_outer()`, `wrap_head()`, `single_paren_nested()`, `removed()`, `bare()`.
- Every setter takes `mut self` and returns `Self` for chaining.
- Sprite-aware setter `with_sprite(name: &str) -> Result<Self, CompilerError>` on `HeroBlock`, `Monster`, `ReplicaItem`, `AbilityData`, `TriggerHpDef`, fight-unit builders — resolves via `SpriteId::try_new` and stores a single typed `sprite` field. There is no separate `with_sprite_name` or `with_img_data`.
- For every IR variant the fidelity plan introduces, ship an authoring constructor alongside it (closes `PIPELINE_FIDELITY_PLAN.md` §1b.1).

**Verification**:
- [ ] Unit test per type demonstrating chained construction matches an equivalent struct literal.
- [ ] Emit-after-build for a FightUnit produces flat `Template.props` output (body_order empty → canonical fallback).
- [ ] Emit-after-build for a boss with `wrap_outer()` on each unit matches source wrapping style.

---

### Chunk 6: `DiceFaces::from_pairs` + `dice![]` macro [PARALLEL GROUP B]
**Scope**: Ergonomic dice construction, all `FaceId`-typed.
**Files**: `compiler/src/authoring/dice.rs` (new), `compiler/src/authoring/mod.rs` (re-export), `compiler/src/lib.rs` (`pub use authoring::dice;`)
**Dependencies**: Chunks 1, 3 (macro requires `FaceId` newtype)
**Parallel with**: Chunk 5

**Requirements**:
- `DiceFaces::from_pairs(&[(FaceId, Pips)])` — the `FaceId` whitelist and `Pips` clamp are already enforced by the types; no sentinel-based blank detection. Blanks use `DiceFace::Blank` explicitly via `from_faces` or the macro's `_` syntax.
- `DiceFaces::from_faces(Vec<DiceFace>)` — direct vec wrapper.
- `dice!` macro expands `dice![FACE-PIPS, _, FACE-PIPS, _, _, FACE-PIPS]` to a `DiceFaces` literal. `FACE` is a path resolving to `FaceId` (e.g., `DAMAGE` from a prelude import, or `FaceId::DAMAGE`).
- Macro **rejects** bare integer literals (`dice![34-1]` → `compile_error!`) — accepting them would bypass the whitelist, reintroducing the hallucination class `FaceId` exists to eliminate. Any raw-`u16` pathway must go through `FaceId::try_new` explicitly.
- Macro enforces exactly 6 entries via a `compile_error!` on mismatch.
- Must be exportable: `pub use authoring::dice;` from `lib.rs` so callers `use slice_and_dice::dice;`.

**Verification**:
- [ ] `dice![34-1, _, _, _, _, _]` produces `faces: [Active(34,1), Blank, Blank, Blank, Blank, Blank]`.
- [ ] `dice![DAMAGE-1, SHIELD-2, _, _, _, _]` compiles when `face_id::{DAMAGE, SHIELD}` are in scope.
- [ ] `dice![34-1]` (5 entries) fails with a compile error, not a runtime panic.
- [ ] Negative pips: `dice![13--1, _, _, _, _, _]` → `Active(13, -1)`. (Verify emit: `"13--1:0:0:0:0:0"`.)
- [ ] Roundtrip: `DiceFaces::parse(&dice![...].emit()) == dice![...]`.

---

### Chunk 7: `HeroReplica` builder + authoring roundtrip tests [INTEGRATION]
**Scope**: Three-tier hero builder + the Path B determinism test suite that certifies authored IR is indistinguishable from parsed IR (SPEC §4 Path B).
**Files**: `compiler/src/authoring/hero_replica.rs` (new), `compiler/tests/path_b/` (new test directory; also referenced by `PIPELINE_FIDELITY_PLAN.md` Phase 4.2)
**Dependencies**: Chunks 1–6

**Requirements**:
- `HeroReplica::new(name, color)` → `.tier1(HeroBlock)` → `.tier2a/2b(HeroBlock)` → `.tier3a/3b(HeroBlock)` → `.build() -> Result<Hero, CompilerError>`.
- `build()` sets `format: HeroFormat::Sliceymon`, `source: Source::Custom`, assembles blocks in canonical order (T1, T2A, T2B, T3A, T3B).
- `build()` returns `Err(CompilerError)` on missing tiers (all 5 required for Sliceymon shape) with `field_path` + `suggestion`.
- Path B roundtrip test: for a representative hand-authored FightUnit, HeroBlock, Hero, Boss, Monster, ReplicaItem (each kind), plus every IR variant introduced by `PIPELINE_FIDELITY_PLAN.md` Phase 1:
  1. Construct via authoring layer (no `extract` call).
  2. `build(&ir)` → textmod string.
  3. `extract(&text)` → re-parsed IR.
  4. Assert `re_parsed == original` as **semantic IR equality** (SPEC §3.1), not byte diff.
- Verify against ONE real hero from `working-mods/sliceymon.txt` — extract it to IR, then re-author it via `HeroReplica`, and confirm the resulting IR is field-equal to the extracted version.

**Verification**:
- [ ] `cargo test --test path_b` passes.
- [ ] All 4 working mods still IR-equal roundtrip after the authoring layer is wired in (regression check; shared gate with `PIPELINE_FIDELITY_PLAN.md` Phase 4.1).
- [ ] A one-screen Charmander example lives in `compiler/examples/author_hero.rs` showing the whole API in use.
- [ ] Checkpoint: present the example + test output for user review.

---

### Final Verification (After All Chunks)
- [ ] `cargo test --all` passes with no regressions.
- [ ] `cargo test --test path_b` passes — Path B from SPEC §4 is exercised (not just the Path A extract+rebuild).
- [ ] IR-equal roundtrip holds for all 4 working mods (shared with `PIPELINE_FIDELITY_PLAN.md` Phase 4.1).
- [ ] `cargo run --example author_hero` compiles and emits a valid hero line; the emitted line, when re-extracted, equals the authored IR.
- [ ] `cargo run -- schema` produces a JSON Schema that validates `FaceId` and `SpriteId` against their whitelists.
- [ ] No `std::fs` / `std::process` in `compiler/src/authoring/` (SPEC §3.4, WASM-clean).
- [ ] No `unwrap()` / `expect()` / `panic!` in `compiler/src/authoring/` library code (SPEC §8).
- [ ] A manually written Pokemon hero (≤30 lines of authoring code) emits a modifier that passes `cargo run -- check --round-trip`.
- [ ] One hero is written end-to-end using the new API as a proof-of-concept — presented to user for sign-off before we scale to the full roster. Pokemon choice and design come from the user, not from archived design docs.

---

## Out of Scope (future work)

- **Automatic balance validation during construction** — SPEC §6.4 game-design rules are enforced in `xref`, not in the authoring layer. Authoring refuses only structurally-invalid inputs (unknown `FaceId`/`SpriteId`, wrong dice arity, missing required Sliceymon tiers).
- **Webapp JSON authoring schema** — the webapp constructs IR via serde + `schemars`-derived schema. The typed newtypes' serde `try_from` keeps webapp input equally safe; no separate authoring schema is needed.
- **Macro-based hero DSL** (`hero! { name: "Pikachu", tier1: { ... } }`) — the `.with_*()` chain is already terse enough. Reconsider only if the chain proves too verbose in practice.
- **Auto-generation from design doc markdown** — parsing `hero_designs_batch*.md` into `HeroReplica` calls. Possible future automation once the authoring API is stable.

## Risk & Mitigation

| Risk | Mitigation |
|------|-----------|
| `FaceId` whitelist drifts from the working-mods corpus + guide | Chunk 3 is a critical checkpoint; generator re-runs as a `build.rs` step; unit test enforces every associated const has a citation. |
| IR flips to `FaceId` / `SpriteId` break the extractor on a working mod | All 4 mods gated by IR-equal roundtrip in Chunks 3 and 4; any unknown ID/sprite in the corpus is fixed by adding to the whitelist (or is a bug in the source mod — flagged as `CompilerError` with actionable suggestion, not silently admitted). |
| `SpriteId` registry changes silently break builds | `build.rs` re-runs on any `working-mods/*.txt` change; `phf::Map` keys are stable; `SpriteId::try_new` returns `CompilerError` on unknown, never panics. |
| Builder produces IR that emits differently than parsed IR | Chunk 7 Path B suite catches this — fails loudly on any `extract(build(ir)) != ir` divergence (SPEC §4). |
| Default values leak into parsed IR and mask bugs | `Default` is additive; the extractor continues to populate every field it populates today. IR-equal roundtrip compares full `ModIR`, so a parser regression stays visible. |
| Callers bypass authoring layer via struct literals, reintroducing hallucination | Module-level docs + `pub(crate)` visibility on IR fields where feasible; `FaceId` / `SpriteId` tuple inner fields are private. Serde `try_from` guards JSON input. |
