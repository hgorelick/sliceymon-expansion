# Platform Foundations Plan

## Overview

This plan delivers the IR schema, type-system, and build-API foundations that `PIPELINE_FIDELITY_PLAN.md` (parser/emitter drift fixes) and `AUTHORING_ERGONOMICS_PLAN.md` (author-from-scratch surface) both depend on. It does not own any drift-class fix and does not own the authoring builder API. It owns:

- Structured-error refactor of `CompilerError` (prerequisite for every other chunk that cites `field_path` / `suggestion`).
- `Default` + `::new(identity)` on authorable IR types where a safe default exists post-F4; explicit list of types that cannot derive `Default`.
- The empty `compiler/src/authoring/` module skeleton.
- Typed whitelist newtypes `FaceId` and `SpriteId` with their generators, and the new `Pips` newtype used by `DiceFace::Active`.
- IR field consolidation that makes sprite-bearing types self-contained (SPEC §3.3).
- `ReplicaItem` container-shape refactor: replace the stringly-typed `container_name: String` with a `ReplicaItemContainer { Capture { name: String }, Legendary }` enum, making invalid states unrepresentable (SPEC §3.6, §3.7). No separate `kind` discriminator.
- The provenance-filtered build API (`BuildOptions` / `SourceFilter` / `build_with`) and xref-finding provenance.
- `merge` retains SPEC §5's `pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError>` shape. Warnings emitted when merge strips derived structurals are written into a new `ModIR.warnings: Vec<Finding>` sidecar field so the signature stays as SPEC §5 specifies.
- Authoring of new xref rules `X003`, `X010`, `X016`, `X017` (these do not exist today — current xref uses `V016`/`V019`/`V020` only). No X001 demo rule.
- Merge semantics that strip derived structurals so `build` regenerates them unconditionally, plus the two missing derived generators (`pool_replacement`, `hero_item_pool`).
- `panic!`/`unwrap`/`expect` elimination in library code (SPEC §8) — scoped to the 6 lib-code occurrences across 5 files that a verified audit found, NOT the `ir/mod.rs:284` site (which is inside `#[cfg(test)]`).

All items here satisfy SPEC §3.7 (no parallel representations, no deferred replacement). Each is implementable against today's codebase, with explicit prerequisites where the current type shape does not yet support the target design.

## Goals

1. Make invalid states unrepresentable (SPEC §3.6): hallucinated Face IDs and sprite names become `CompilerError`, not silent runtime bugs.
2. Make the IR self-contained (SPEC §3.3): `build(ir)` produces a textmod with no ambient inputs.
3. Use `Source::{Base, Custom, Overlay}` as a first-class filter and severity signal (SPEC §5 #4).
4. Give merge and build a clean separation: merge carries only content; build regenerates derived structurals.
5. Eliminate `panic!`/`unwrap`/`expect` from library code (SPEC §8).

## Non-Goals

- Drift-class fixes — `PIPELINE_FIDELITY_PLAN.md` owns these.
- Chainable builders, `dice!` macro, `HeroReplica` — `AUTHORING_ERGONOMICS_PLAN.md` owns these.
- New IR variants for classes B/F/H/J/K/L/Q/R/S/T/U/V/W/X/Y — those land as part of pipeline fidelity.

## Spec

### F0. `CompilerError` structured-error refactor (PREREQUISITE)

Every chunk below cites `CompilerError` carrying `field_path` and `suggestion`. The current type (`compiler/src/error.rs`) has no such fields on any variant — each variant has bespoke fields (`position`, `hero_name`, `tier_index`, etc.).

Replace the current per-variant shape with:

```rust
pub struct CompilerError {
    pub kind: ErrorKind,
    pub field_path: Option<String>,
    pub suggestion: Option<String>,
    pub context: Option<String>,
}

pub enum ErrorKind {
    Split { raw_position: usize, message: String },
    Classify { modifier_index: usize, preview: String, message: String },
    HeroParse { modifier_index: usize, hero_name: String, tier_index: Option<usize>, position: usize, expected: String, found: String },
    Paren { modifier_index: usize, position: usize, depth: i32 },   // per-variant `context: String` on the old `ParenError` is dropped; existing sites move their text to the outer `CompilerError.context` via `.with_context(...)`.
    Build { component: String, message: String },
    MergeConflict { key: String, base_value: String, overlay_value: String },
    SpriteNotFound { sprite_name: String, hero_name: Option<String>, tier_index: Option<usize> },
    FaceIdInvalid { raw: u16, template: Option<String> },            // new, used by F3
    DerivedStructuralAuthored { modifier_type: String },             // new, used by F6
    Validation { message: String },
    DuplicateName { name: String, existing_category: String, new_category: String },
    DuplicateColor { color: char, existing_hero: String },
    NotFound { type_name: String, key: String },
    ChainParse { content: String, position: usize, expected: String, found: String },
    PhaseParse { phase_code: Option<char>, content: String, expected: String, found: String },
    RewardParse { content: String, expected: String, found: String },
    Io(String),
}
```

`ReplicaItemKindMismatch` is intentionally absent: §F7 replaces `container_name: String` with a `ReplicaItemContainer` enum, so the former Capture-without-container / Legendary-with-container invariants are unrepresentable at the type level and need no runtime error variant.

Constructor helpers (`CompilerError::build(component, message)`, `::paren(...)`, etc.) build the common fields in one call. The `Display` impl handles `field_path` / `suggestion` printing in a single shared tail. Every existing construction site migrates to the new shape in the same PR (the extractor, builder, and ops files).

This refactor is a prerequisite for every subsequent chunk that asserts errors carry `field_path` and `suggestion`. It replaces the existing type — no parallel error type.

### F1. `Default` + `::new(identity)` on authorable IR types

Types that can derive `Default` **today** (all fields already have safe defaults): `FightUnit` (already does), `Boss`, `FightDefinition`.

Types that **cannot** derive `Default` after F3/F4 because they gain `FaceId` / `SpriteId` / required-identity fields:

| Type | Blocker | Resolution |
|---|---|---|
| `Hero` | `color: char` has no safe default (any char we pick creates a false "valid hero") | No `Default`. `::new(internal_name, mn_name, color)` is the only construction. |
| `HeroBlock` | `sprite: SpriteId` post-F4 (no safe empty whitelist entry), `sd: DiceFaces` | No `Default`. `::new(template, sprite, sd)` — `tier`, `hp`, `color` default via options. |
| `ReplicaItem` | `sprite: SpriteId` post-F4, `container: ReplicaItemContainer` post-F7 (non-`Default`-able enum with no inherently safe variant) | No `Default`. `::new(name, container, template, sprite, sd)`. |
| `Monster` | `sprite: Option<SpriteId>` (Option — so a none-sprite monster is expressible) | `Default` allowed; `sprite` defaults to `None`. |
| `AbilityData` | `sprite: Option<SpriteId>` post-F4 | `Default` allowed. |
| `TriggerHpDef` | `sprite: Option<SpriteId>` post-F4 | `Default` allowed. |
| `DiceFaces` | `Vec<DiceFace>` — safe default is empty vec | `Default` allowed. |

`::new` signatures (strings accept `impl Into<String>`; every field not in the signature is set via `..Default::default()` for types that derive `Default`, or via an explicit per-field default list in the impl for types that don't):

- `Hero::new(internal_name: impl Into<String>, mn_name: impl Into<String>, color: char) -> Self`
- `HeroBlock::new(template: impl Into<String>, sprite: SpriteId, sd: DiceFaces) -> Self`
- `ReplicaItem::new(name: impl Into<String>, container: ReplicaItemContainer, template: impl Into<String>, sprite: SpriteId, sd: DiceFaces) -> Self`
- `Monster::new(name: impl Into<String>, base_template: impl Into<String>, floor_range: impl Into<String>) -> Self`
- `Boss::new(name: impl Into<String>, level: Option<u8>) -> Self`
- `FightDefinition::new() -> Self` (via `Default`)
- `FightUnit::new(template: impl Into<String>, name: impl Into<String>) -> Self`

Chunk ordering: Chunk 1 lands `::new` for types that can have safe defaults **today** (Hero without a FaceId/SpriteId dependency — color is still required). Sprite-bearing constructors (`HeroBlock`, `ReplicaItem`, `AbilityData`, `TriggerHpDef`, `Monster`) take their final `SpriteId`-based shape in the same PR as §F4.

### F2. `compiler/src/authoring/` module skeleton

Create `compiler/src/authoring/mod.rs` with a module doc comment citing SPEC §6.1 ("only supported path from human/LLM intent to an IR value"). The module is otherwise empty at this plan's conclusion — it is populated by `AUTHORING_ERGONOMICS_PLAN.md` chunks. `lib.rs` declares `pub mod authoring;` and nothing more.

### F3. `FaceId` + `Pips` newtypes + whitelist + IR flip

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "u16", into = "u16")]
pub struct FaceId(u16);

impl FaceId {
    pub const fn try_new(raw: u16) -> Result<Self, FaceIdError>;
    pub const fn raw(self) -> u16;
    // Generated associated consts (see below).
}

// Generated consts — emitted into the SAME file as the newtype definition so
// `Self(N)` can access the tuple field. The field stays private at module level;
// only the `face_id_generated.rs` (included via `include!`) constructs values
// from raw u16.
pub const DAMAGE: FaceId = FaceId(34);
// ...

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "i16", into = "i16")]
pub struct Pips(i16);
// `Pips::try_new` accepts the full `i16` range — no corpus-derived bound.
// Negative pips exist in the corpus (see `ir/mod.rs:50` "pips may be
// negative (e.g. `13--1`)"). Corpus-derived bounds would overfit the
// working-mods set and break future valid mods. If `reference/textmod_guide.md`
// declares an explicit pip range, use that bound; otherwise accept all `i16`.
// Pip magnitude is a numeric field, not a whitelist — no `Unknown` variant.
```

Both newtypes live at `compiler/src/authoring/face_id.rs`. Re-exported from `lib.rs`. The consts are emitted **into the same file** (or `include!("face_id_generated.rs")`-style — not a submodule with a private tuple field that would break `Self(N)` access).

**Escape hatch — user ruling 2026-04-20: permissive (b).** Any valid textmod (per `reference/textmod_guide.md`) must extract, even if it uses a FaceID outside the working-mods corpus. `pub enum FaceIdValue { Known(FaceId), Unknown(u16) }` where `Unknown` carries the raw ID and emits via the `u16` back-channel; whitelist is advisory, extraction of unknown IDs succeeds with a `Finding` (`Severity::Warning`) and can still round-trip byte-for-byte.

This ruling **requires a SPEC §3.6 amendment in the same PR as Chunk 2**. SPEC §3.6 currently reads: *"constructing an invalid `FaceId` is a compile error, not a runtime validation failure."* That wording predates the general-purpose-backend framing and is incompatible with a permissive whitelist. Amend to:

> Use Rust enums and newtypes to encode constraints at compile time wherever the constraint is a **format invariant** (e.g., `DiceFace::{Blank, Active{..}}`). For **corpus-derived whitelists** (Face IDs, sprite registry), the typed layer is the authoring path and the source-of-truth for correctness, and unknown values are surfaced as `Unknown(raw)` variants that extract successfully and emit a xref `Finding` at `Severity::Warning`. This preserves SPEC §1 / §3.3 (any valid textmod extracts, self-contained IR) while keeping the authoring layer hallucination-free (SPEC §6.1).

SPEC §3.6's `Pips` type annotation is likewise amended from `u8` to `i16` (E5). The SPEC amendment lands as a file edit in the same commit as the Chunk 2 newtype introduction; it is not a separate chunk.

Build.rs generator (`compiler/build.rs`, new file): runs the current extractor over `working-mods/*.txt`, collects every distinct `face_id` into a `BTreeMap<u16, FaceIdMeta>` (BTreeMap, not HashMap — deterministic iteration is load-bearing), emits one `pub const NAME: FaceId = FaceId(N);` per ID into `compiler/src/authoring/face_id_generated.rs`. The generator writes consts in ascending `u16` order. Each const's doc-comment cites a `reference/textmod_guide.md` section where the ID is documented OR a working-mods template-unit line (sorted by `(mod_name, line_number)`) where it occurs. No wall-clock, PID, or environment input enters the generator — `cargo build` twice yields a byte-identical file.

IR flip in the same PR: `DiceFace::Active { face_id: FaceIdValue, pips: Pips }` replaces the current `{ face_id: u16, pips: i16 }`. Every `.sd.` parser callsite calls `FaceIdValue::try_new(raw)` and `Pips::try_new(raw)?`. Unknown IDs become `FaceIdValue::Unknown(raw)` and emit a `Finding` from xref. Unknown pip values (outside observed range by >1 order of magnitude) become `CompilerError`.

New xref rule **X016 (face-template compatibility)** — NOTE: this rule **does not exist today**. Current xref uses V016/V019/V020 only. Chunk 2 authors X016 as a new rule, operating on `FaceIdValue::Known(FaceId)` values; `FaceIdValue::Unknown` emits a separate `X017` warning at xref time.

No parallel `face_id_raw` field. Single pass across extractor/emitter/xref.

### F4. `SpriteId` newtype + registry + IR consolidation

```rust
// Cow-backed. Registry returns borrowed &'static strs; runtime constructions
// (e.g., an extractor parsing a novel sprite) own their data. `&'static str`
// alone is unsound — deserialization from owned `String` has no lifetime to
// borrow from without leaking or global interning.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "SpriteIdSerde", into = "SpriteIdSerde")]
pub struct SpriteId {
    name: Cow<'static, str>,
    img_data: Cow<'static, str>,
}

// Flat serde representation: `{ "name": "...", "img_data": "..." }`.
// Chosen over `try_from = "String"` because a sprite is not losslessly
// round-trippable via its name alone when the registry doesn't know it —
// the img_data must serialize.
#[derive(Serialize, Deserialize, JsonSchema)]
struct SpriteIdSerde { name: String, img_data: String }

impl SpriteId {
    /// Registry lookup — returns a zero-allocation borrowed value.
    pub fn lookup(name: &str) -> Option<&'static SpriteId>;
    /// Construct from owned data — used by the extractor when `.img.` is
    /// present for a name not in the registry. Always succeeds.
    pub fn owned(name: impl Into<String>, img_data: impl Into<String>) -> Self;
    /// Strict registry-only constructor — fails if `name` isn't registered.
    pub fn try_registered(name: &str) -> Result<&'static SpriteId, CompilerError>;
    pub fn name(&self) -> &str;
    pub fn img_data(&self) -> &str;
}
```

Rationale for dual constructors: the registry (Path B / authoring) needs zero-allocation lookup; the extractor (Path A) needs to accept any `.img.` payload the game ingested. Making `SpriteId` registry-only would re-break SPEC §3.3 (self-contained IR) because a new mod with a new sprite couldn't extract.

**Escape hatch for novel sprites**: `SpriteId::owned(...)` is the escape hatch; its existence is the design answer to Count O for sprites. The registry is advisory, not gating.

`SpriteId` lives at `compiler/src/authoring/sprite.rs`. Re-exported from `lib.rs`. The generated registry lives at `compiler/src/authoring/sprite_registry.rs` (via `include!`), populated at build time via `phf_codegen`.

Cargo dep addition: `phf = { version = "0.11", features = ["macros"] }` (runtime dep) + `phf_codegen = "0.11"` (build-dep). Both added to `compiler/Cargo.toml` in the same PR as this chunk.

Registry generator (`compiler/build.rs`, same file as F3's generator): runs the extractor over `working-mods/*.txt`, collects every `.img.` payload keyed by the entity's `.mn.` (falling back to `.n.` when absent) into a `BTreeMap<String, SpriteId>`, dedupes last-write-wins in mod-priority order (sliceymon > pansaer > punpuns > community; mods iterated in that fixed order), and emits via `phf_codegen::Map::build()` which itself emits entries in a deterministic hash order. `cargo build` twice yields a byte-identical `sprite_registry.rs`. No wall-clock or environment input.

IR consolidation in the same PR: every IR type that references a sprite replaces the split fields with a single sprite field. Enumerated (corrected against actual IR):

| Type | Current fields | Post-F4 field |
|---|---|---|
| `HeroBlock` | `sprite_name: String` + `img_data: Option<String>` | `sprite: SpriteId` (required — `SpriteId::owned` if not in registry) |
| `ReplicaItem` | `sprite_name: String` + `img_data: Option<String>` | `sprite: SpriteId` (required) |
| `Monster` | `sprite_name: Option<String>` + `img_data: Option<String>` | `sprite: Option<SpriteId>` |
| `FightUnit` | `sprite_data: Option<String>` (NOT `sprite_name`) | `sprite: Option<SpriteId>` (name derived from required `fight_unit.name: String` at extract — confirmed `ir/mod.rs:1181`; generic names like `"Boss"` flow to `SpriteId::owned(name, img_data)` as novel owned sprites per E1(b)) |
| `AbilityData` | no `sprite_name` field currently; `img_data: Option<String>` | `sprite: Option<SpriteId>` (name derived from parent ability name at extract) |
| `TriggerHpDef` | no `sprite_name` field currently; `img_data: Option<String>` | `sprite: Option<SpriteId>` (name derived from parent entity name at extract) |

Two fields → one, per SPEC §3.7.

Build-API signature change in the same PR: drop `sprites: &HashMap<String, String>` from every public fn in `lib.rs` that currently takes it. Verified list against `lib.rs` today:

| Function | Takes `&HashMap<String, String>` today? | Post-F4 |
|---|---|---|
| `extract` | no | no |
| `build` | yes | drop |
| `build_complete` | yes | drop |
| `merge` | no | no |
| `build_hero` | yes | drop |
| `build_replica_item` | no | no |
| `build_monster` | no | no |
| `build_boss` | no | no |
| `validate_hero` | yes | drop |
| `validate_hero_in_context` | yes | drop |

Every extractor sprite-consuming parser calls `SpriteId::owned(name, img_data)` (registry miss becomes an owned value, not an error). Merge carries `sprite` through unmodified.

### F5. `BuildOptions { include: SourceFilter }` + `build_with` + provenance-aware findings

`build_with` does not exist today — this chunk introduces it and makes `build` a thin wrapper.

```rust
#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub include: SourceFilter,
}

impl Default for BuildOptions {
    fn default() -> Self { Self { include: SourceFilter::All } }
}

#[derive(Debug, Clone)]
pub enum SourceFilter {
    All,
    Only(SourceSet),
    Exclude(SourceSet),
}

// Enum defaults require a #[default] attribute (Rust 1.62+).
// BuildOptions::default() uses the hand-written impl above, NOT #[derive(Default)]
// on SourceFilter, so this is fine. If any callsite wants SourceFilter::default(),
// annotate All with #[default]. Not required for this plan.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSet(u8); // bitmask: bit 0=Base, bit 1=Custom, bit 2=Overlay

impl SourceSet {
    pub const fn empty() -> Self { Self(0) }
    pub const fn all() -> Self { Self(0b111) }
    pub const fn single(s: Source) -> Self { Self(1 << Self::bit(s)) }
    pub const fn contains(self, s: Source) -> bool { self.0 & (1 << Self::bit(s)) != 0 }
    pub const fn union(self, other: Self) -> Self { Self(self.0 | other.0) }
    const fn bit(s: Source) -> u8 {
        match s {
            Source::Base => 0,
            Source::Custom => 1,
            Source::Overlay => 2,
        }
    }
}

impl FromIterator<Source> for SourceSet {
    fn from_iter<I: IntoIterator<Item = Source>>(iter: I) -> Self {
        iter.into_iter().fold(Self::empty(), |acc, s| acc.union(Self::single(s)))
    }
}

impl SourceFilter {
    pub const fn admits(&self, s: Source) -> bool {
        match self {
            SourceFilter::All => true,
            SourceFilter::Only(set) => set.contains(s),
            SourceFilter::Exclude(set) => !set.contains(s),
        }
    }
}

pub fn build_with(ir: &ModIR, opts: &BuildOptions) -> Result<String, CompilerError>;
```

`build(ir)` is `build_with(ir, &BuildOptions::default())`. `Source::Base` default matches the pre-existing `Source::is_base` shortcut (ir/mod.rs:23 — extend to `pub(crate)` if needed to reuse).

No external crate dependency — the bitflag is local.

Every content-emission site in `builder/` checks `opts.include.admits(entity.source)` before emitting. Derived structurals are regenerated from the post-filter content set; they do not carry their own `Source` filter.

`xref::Finding` gains `pub source: Option<Source>`. `Option` because some findings are global (e.g., "no heroes defined") and don't bind to a single entity. Every xref rule that visits a sourced entity populates it. Severity promotion: violations on `Source::Base` emit at `Severity::Warning`; `Source::Custom`/`Overlay` emit at `Severity::Error`.

**Finding construction-site audit**: adding a field to `Finding` breaks every struct-literal construction site. `lib.rs:68` uses `..Default::default()` — safe. Audit target for Chunk 4: every other construction site in `xref.rs` and `ir/ops.rs`. Finding currently derives `Default` (xref.rs:38), so `..Default::default()` tails work; adding `source: Option<Source>` with `#[serde(default)]` preserves JSON compat. All existing V-rules (V016, V019, V020) keep their IDs unchanged — the plan does NOT rename them.

### F6. Merge strips derived structurals; build regenerates unconditionally

**Merge signature change (PREREQUISITE).** Current signature:

```rust
pub fn merge(base: ModIR, overlay: ModIR) -> Result<ModIR, CompilerError>
```

SPEC §5 public-surface sketch mandates the in-place form:

```rust
pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError>
```

To emit warnings when merge strips derived structurals without violating SPEC §5's signature, warnings are written into a new `ModIR.warnings: Vec<Finding>` sidecar field rather than returned as a tuple. Rationale:

- SPEC §5 is authoritative; the tuple-return form considered earlier contradicted it.
- A sidecar on `ModIR` generalizes beyond merge — `build` can also append findings (e.g., when a build-time derived-structural regeneration discovers a stale author-supplied copy), and downstream tools see a single channel.
- `ModIR.warnings` is serde `#[serde(default, skip_serializing_if = "Vec::is_empty")]` — zero JSON-schema churn for mods without findings.

Schema addition (applied in the same chunk):

```rust
pub struct ModIR {
    // ... existing fields ...
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Finding>,
}
```

Every callsite updates in the same PR:
- `compiler/src/lib.rs::merge` re-export — surface matches SPEC §5 exactly.
- `compiler/src/main.rs` CLI merge command — uses `&mut base`.
- All tests in `compiler/tests/` that call `merge` — switch from `let merged = merge(base, overlay)?;` to `merge(&mut base, overlay)?;` and read `base.warnings` for stripping notices.

No parallel `merge_with_findings` function. Replace the signature.

Derived structural inventory (current state in `builder/derived.rs`):

| Structural kind | Derived from | Regenerator | Status |
|---|---|---|---|
| Character Selection (`ch.`/`cs.`) | Heroes sorted by color | `builder::derived::generate_char_selection` | **exists** |
| HeroPoolBase (bare-name `heropool.`) | Hero internal_names | `builder::derived::generate_hero_pool_base` | **exists** |
| PoolReplacement (tier-constrained heropool) | Heroes grouped by color + tier | `builder::derived::generate_pool_replacement` | **to build** |
| Hero-bound ItemPool | `ReplicaItem` entries whose `container` is `ReplicaItemContainer::Capture { name }` where `name` matches a hero | `builder::derived::generate_hero_item_pool` | **to build** |

Chunk 5 authors the two missing regenerators. Acceptance criterion for each: running it against `working-mods/sliceymon.txt` (post-F7, where `ReplicaItem.container: ReplicaItemContainer` — the `Capture { name }` variant carries what was the old `container_name`) reproduces the base mod's existing PoolReplacement / hero-bound ItemPool modifiers byte-for-byte. If it doesn't, the chunk is incomplete.

**Classification helper**: add `fn is_derived(&self) -> bool` to `StructuralModifier` in `ir/mod.rs` (not `builder/derived.rs`, because merge lives in `ir/`). Matches on `modifier_type` against the four derived kinds above.

`merge` strips any of these from both base and overlay inputs before copying content. Each strip pushes a `Finding` onto `base.warnings` at `Severity::Warning` with `rule_id: "X010"` (new rule; NOTE: X003, X016, X017 are also new — all `X*` IDs in this plan are new rules; X001 from earlier drafts has been removed because duplicate-name rejection already lives in `CompilerError::DuplicateName` + existing xref checks), `field_path` naming the modifier, and `suggestion: "Derived structurals are regenerated at build time; authoring them directly is unsupported."`

`build` and `build_complete` regenerate derived structurals from the post-merge content set unconditionally. The "when absent" gate in `build_complete` (`compiler/src/builder/mod.rs:161-168` — the `if !ir.structural.iter().any(|s| s.modifier_type == StructuralType::Selector)` and `HeroPoolBase` checks) is REMOVED: regeneration strips any pre-existing derived structural from `ir.structural` first, then appends the regenerated form. `build` (not just `build_complete`) also performs this strip-and-regenerate.

### F7. `ReplicaItemContainer` — collapse kind + container_name into one enum

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ReplicaItemContainer {
    /// Source-level `itempool.(X).n.Y` — the capturable ball shape. The `name`
    /// is the outer container's `.n.` value (`X` above). By construction a
    /// Capture cannot exist without it.
    Capture { name: String },
    /// Source-level top-level `item.` — persistent ally with spell. By
    /// construction a Legendary carries no container name.
    Legendary,
}
```

**Schema replacement.** Current IR: `ReplicaItem.container_name: String` (ir/mod.rs:570). Replace with a single field `pub container: ReplicaItemContainer`. No separate `kind` discriminator — the variant IS the discriminator.

Why a single enum instead of `kind: ReplicaItemKind` + `container_name: Option<String>`:
- **Makes invalid states unrepresentable (SPEC §3.6)**. Capture-without-container and Legendary-with-container are type errors, not runtime `CompilerError` paths. No `ReplicaItemKindMismatch` variant needed on `ErrorKind`.
- **SPEC §3.7 — no parallel representations.** `kind` and `container_name.is_some()` encoded the same source-level distinction; keeping them separate was exactly the parallel-field pattern SPEC §3.7 forbids.
- **Forward compatibility is cheap.** If `reference/textmod_guide.md` ever documents a named Legendary, extend the enum (`Legendary { name: Option<String> }`) at that time; this is a one-line schema change with `#[serde(default)]`, not a type-system redesign.

Callsite blast radius: every emitter / extractor / xref site touching `container_name`. Enumerated via grep: `compiler/src/extractor/replica_item_parser.rs`, `compiler/src/builder/replica_item_emitter.rs`, `compiler/src/xref.rs`, `compiler/src/ir/ops.rs` (constructor sites), `compiler/src/ir/merge.rs` (match-by-key logic). Each updates in the same PR.

Extractor classifier reads the source-level modifier kind directly and constructs the enum: `itempool.(X).n.Y` → `ReplicaItemContainer::Capture { name: X }`; top-level `item.` → `ReplicaItemContainer::Legendary`. Builder dispatches emission with a `match` on `container` — a non-exhaustive match is a compile error, so new variants cannot silently fail to emit.

No build-time invariant check is needed; the type system already enforces Capture⇔container-name and Legendary⇔no-container.

New xref rule **X003** (no duplicate Pokemon across heroes / captures / legendaries / monsters — SPEC §6.3) is authored as part of this chunk. It does **not** exist today; current xref uses V-prefixed rules. X003 matches on `ReplicaItem.container` to route items into the per-kind buckets (Capture vs Legendary).

### F8. `panic!`/`unwrap`/`expect` elimination in library code

**Correction — the original plan cited `compiler/src/ir/mod.rs:284` as a `panic!("Expected Item segment")` in `ModifierChain::split_at_segment()`. That line is inside a `#[cfg(test)]` module (test `typed_entries_populated` at line 274) and is not library code.** Library-code `panic!` is not at this location.

**Verified lib-code audit (with `#[cfg(test)]` blocks removed — 2026-04-20):**

| File | Lib-code `unwrap/expect/panic` hits |
|---|---|
| `compiler/src/builder/hero_emitter.rs` | 2 |
| `compiler/src/extractor/fight_parser.rs` | 1 |
| `compiler/src/extractor/reward_parser.rs` | 1 |
| `compiler/src/extractor/hero_parser.rs` | 1 |
| `compiler/src/extractor/phase_parser.rs` | 1 |
| **Total** | **6 across 5 files** |

Replace each with structured error propagation using the new `CompilerError` shape from §F0. Callers that previously could not fail now return `Result`; update call chains.

Before starting Chunk 7, re-run the audit (`rg '\.unwrap\(\)|\.expect\(|panic!\(|unimplemented!|todo!\(' compiler/src/` with test-block stripping) to confirm the count — the baseline may drift as §F0 and earlier chunks land and add new error paths. SPEC §8.

---

## Implementation Plan

### Checkpoint Configuration
- Total chunks: 10 (0, 1, 2, 3a, 3b, 3c, 4, 5, 6, 7).
- Checkpoint frequency: After Chunk 0 (error type lands), Chunk 2 (SPEC §3.6 amendment + newtypes + IR flip), Chunk 3c (HashMap dropped + sprite consolidation), and Chunk 7 (final). Chunk 2's checkpoint is load-bearing because it lands a SPEC amendment; a missed amendment here invalidates every later chunk that cites the permissive whitelist.

### Parallel Execution Map — conflict-verified

File-level conflict matrix (every chunk's primary writes):

| Chunk | Touches `ir/mod.rs` | Touches `lib.rs` | Touches `build.rs` | Touches `authoring/mod.rs` | Touches `xref.rs` | Touches `error.rs` | Touches `builder/derived.rs` | Touches `ir/merge.rs` |
|---|---|---|---|---|---|---|---|---|
| 0 (error refactor) | — | yes (re-export) | — | — | yes (Finding ctor sites) | **yes** | — | yes (ctor sites) |
| 1 (Default + ::new + authoring skel) | **yes** | yes (mod decl) | — | **yes** | — | — | — | — |
| 2 (FaceId + Pips + IR flip + SPEC §3.6 amendment) | **yes** | yes (re-export) | **yes** | yes (submod decl) | yes (X016, X017 new) | yes (FaceIdInvalid) | — | — |
| 3 (SpriteId + registry + IR consolidation + drop HashMap + ModIR.warnings) | **yes** | **yes** (sig change) | **yes** | yes (submod decl) | yes | yes (SpriteNotFound shape) | yes (sprite access) | yes (sprite passthrough) |
| 4 (BuildOptions + build_with + Finding.source + V-rule source retrofit) | — | **yes** (build_with new) | — | — | **yes** (Finding.source on V016/V019/V020) | — | — | — |
| 5 (merge sig → `&mut`, strips derived, new regenerators, is_derived, unconditional regen) | yes (`is_derived` helper) | yes (merge sig) | — | — | — | — | **yes** | **yes** |
| 6 (`ReplicaItemContainer` enum replaces `container_name`) | **yes** | — | — | — | yes (X003 new) | — | — | yes (merge match-by-key) |
| 7 (lib-code unwrap/expect/panic elimination) | no (hits are in extractor/builder only) | no | — | — | — | — | — | — |

**True dependency graph (after the conflict matrix):**

```
Chunk 0 (CompilerError)
  ├── Chunk 1 (Default + ::new + authoring skeleton)
  │     └── Chunk 2 (FaceId/Pips + IR flip)            [shares ir/mod.rs + build.rs + authoring/mod.rs with 3a]
  │           └── Chunk 3a (SpriteId + registry)        [extends build.rs; new authoring submod]
  │                 └── Chunk 3b (IR field consolidation) [rewrites 6 IR types + their parsers/emitters]
  │                       └── Chunk 3c (drop HashMap from public API)
  │                             ├── Chunk 4 (BuildOptions + build_with + Finding.source)
  │                             └── Chunk 6 (ReplicaItemContainer enum replaces container_name)
  │                                   └── Chunk 5 (merge strips + new regenerators + unconditional regen)
  └── Chunk 7 (lib-code unwrap/expect/panic elimination) [parallel from Chunk 0 completion onward; no shared files with 1..6]
```

**Parallel groups (corrected):**
- **Sequential foundation**: Chunk 0 → Chunk 1 → Chunk 2 → Chunk 3a → Chunk 3b → Chunk 3c.
- **Merge checkpoint after Chunk 3c**: Chunks 4 and 6 both touch `xref.rs`, and Chunk 7 touches `builder/hero_emitter.rs` which Chunk 3c also edits. To avoid merge-conflict gymnastics, the branches are prepared in parallel but merged **sequentially in this order**: 4 → 6 → 7. Each merge re-runs the local verification tests before the next merges. This is a **critical checkpoint** per AI-dev persona §Designing for Parallel Multi-Agent Execution rule 6.
- **Sequential after Chunk 6**: Chunk 5 (depends on Chunk 6's `ReplicaItemContainer` enum, which `generate_hero_item_pool` matches on).
- **Chunk 7 scope**: runs after Chunk 3c lands (not from round 2), because 3c touches `hero_emitter.rs` which 7 also edits. Running 7 earlier would require re-doing 7's edits after 3c rewrites the function signatures. Chunk 7's scope is therefore fixed at audit time (immediately before it starts), not at plan-write time — the "6 across 5 files" baseline at plan-write is indicative, not binding.

**Honest minimum wall-clock rounds**: 9 rounds for the main track (0 → 1 → 2 → 3a → 3b → 3c → 4 → 6 → {5, 7 parallel — no shared files once 3c/4/6 land}). Chunk 5 touches `ir/merge.rs`, `builder/derived.rs`, `builder/mod.rs`; Chunk 7 touches only lib-code panic sites in extractor/ and the post-3c `hero_emitter.rs`. They are truly parallel at that point.

The original plan's "Minimum wall-clock rounds: 3" was false — Chunks 2, 3, and 5/6/7 were not actually parallelizable, and Chunk 3's 11-file scope required sub-chunking.

The original plan's "parallel groups A/B/C" are false — Chunk 2 and Chunk 3 both write `ir/mod.rs`, `build.rs`, and `authoring/mod.rs`. Chunk 5 depends on Chunk 6's `ReplicaItemContainer` enum (needed for `generate_hero_item_pool` bucketing). These parallelism claims were wrong and are corrected here.

---

### Chunk 0: `CompilerError` structured-error refactor [PREREQUISITE]
**Spec**: §F0
**Files**: `compiler/src/error.rs`, every construction site across `compiler/src/extractor/*.rs`, `compiler/src/builder/*.rs`, `compiler/src/ir/ops.rs`, `compiler/src/ir/merge.rs` (sig retains `Result<_, CompilerError>`), `compiler/src/xref.rs` (only `Finding` sites — `Finding.field_path` / `.suggestion` already exist).
**Dependencies**: None. Blocks every subsequent chunk.

**Requirements**:
- Replace `CompilerError` enum with `{ kind: ErrorKind, field_path: Option<String>, suggestion: Option<String>, context: Option<String> }` struct per §F0.
- Add constructor helpers and update `Display` impl to print `field_path` / `suggestion` tail.
- Update every `CompilerError::{Variant}` construction site to the new shape (`CompilerError { kind: ErrorKind::Variant { .. }, field_path: ..., suggestion: ..., context: None }`).
- Add new `ErrorKind` variants `FaceIdInvalid`, `DerivedStructuralAuthored` (used by later chunks — introducing them here prevents adding them piecemeal). No `ReplicaItemKindMismatch` variant is added: §F7's `ReplicaItemContainer` enum makes the former Capture/Legendary invariants unrepresentable at the type level.

**Verification — specific tests**:
- [ ] `error::test_display_includes_field_path` — construct each `ErrorKind` variant, assert `Display` output contains `field_path: "..."` line when set.
- [ ] `error::test_display_includes_suggestion` — same for `suggestion`.
- [ ] `error::test_existing_variants_migrate_cleanly` — every `ErrorKind::*` variant constructable via its matching helper.
- [ ] All 4 working mods IR-equal roundtrip (no regression).
- [ ] `cargo test` passes.

---

### Chunk 1: `Default` + `::new(identity)` + `authoring/` skeleton
**Spec**: §F1, §F2
**Files**: `compiler/src/ir/mod.rs`, `compiler/src/authoring/mod.rs` (new), `compiler/src/lib.rs`
**Dependencies**: Chunk 0.

**Requirements**:
- Add `#[derive(Default)]` only to types that have safe defaults today per the §F1 table: `Boss`, `FightDefinition`, `DiceFaces`. `FightUnit` already derives it. `Monster` awaits Chunk 3's sprite consolidation (adding `Default` to it with a pre-consolidation `sprite_name: Option<String>` is safe — do so).
- Add `::new(identity)` to `Hero`, `Monster`, `Boss`, `FightDefinition`, `FightUnit`. Sprite-bearing constructors (`HeroBlock::new`, `ReplicaItem::new`, etc.) land in Chunk 3 in their final form — omit them here.
- Accept `impl Into<String>` for string fields.
- Create `compiler/src/authoring/mod.rs` with a module doc comment citing SPEC §6.1. Declare `pub mod authoring;` in `lib.rs`. The module is otherwise empty.
- Do not change serde attributes.

**Verification — specific tests**:
- [ ] `ir::hero_new_defaults_empty_blocks` — `Hero::new("x", "X", 'r')` has `blocks.is_empty()`, `removed == false`, `source == Source::Base`.
- [ ] `ir::monster_new_defaults_optional_fields_none` — `Monster::new("Foo", "bas", "1-5")` has `sd.is_none()` and the sprite-shaped optional field is `None`. The exact field name (`sprite_name` pre-Chunk 3b, `sprite` post-Chunk 3b) is updated by Chunk 3b when the field is renamed; this test moves with the rename in the same PR as the consolidation.
- [ ] `ir::boss_default_roundtrip` — `Boss::default()` round-trips through `ir_to_json`/`ir_from_json` identically.
- [ ] `authoring::module_doc_comment_present` — compile-time doc test referencing SPEC §6.1.
- [ ] All 4 working mods IR-equal roundtrip.
- [ ] `cargo test` passes.

---

### Chunk 2: `FaceId` + `Pips` newtypes + whitelist + IR flip + SPEC §3.6 amendment [CHECKPOINT]
**Spec**: §F3
**Files**: `SPEC.md` (amend §3.6 per §F3), `compiler/src/authoring/face_id.rs` (new), `compiler/src/authoring/face_id_generated.rs` (generated by build.rs), `compiler/src/authoring/mod.rs`, `compiler/src/lib.rs`, `compiler/src/ir/mod.rs`, `compiler/build.rs` (new), `compiler/src/xref.rs` (new X016 + X017 rules), plus the two source files that touch `face_id`: `compiler/src/ir/mod.rs` (DiceFace parsing/emitting), `compiler/src/constants.rs`.
**Dependencies**: Chunk 0, Chunk 1. **Not parallel with Chunk 3** (shares `ir/mod.rs`, `build.rs`, `authoring/mod.rs`, `lib.rs`).

**Requirements**:
- **Amend SPEC §3.6** with the wording in §F3 — permissive whitelist is the spec-blessed design, not an override. `Pips` type annotation in §3.6 changes from `u8` to `i16` (E5). This edit lands in the same commit as the newtype introduction; no SPEC drift.
- Define `FaceId(u16)` newtype and `FaceIdValue { Known(FaceId), Unknown(u16) }` per §F3 (permissive, user ruling 2026-04-20).
- Define `Pips(i16)` newtype accepting the full `i16` range (no corpus-derived bound — see §F3 code comment). Negative pips supported per `ir/mod.rs:50`.
- Emit generated consts into `face_id_generated.rs` (same-file equivalent via `include!`) so `FaceId(N)` can construct values within the newtype's module.
- `build.rs` (new — file does not exist today) harvests `.sd.` Face IDs from `working-mods/*.txt` via the existing extractor. Deterministic output using `BTreeMap<u16, FaceIdMeta>` (never `HashMap`). Grouped by mechanic with section comments. Unclassified IDs under `// UNCLASSIFIED`.
- Each associated const's doc-comment cites a `reference/textmod_guide.md` section (with anchor / line number) or a working-mods template-unit line.
- Flip `DiceFace::Active { face_id: FaceIdValue, pips: Pips }`. Every callsite (`ir/mod.rs` DiceFaces::parse + emit, `constants.rs`) updates in one pass.
- Author new xref rule **X016** (face-template compatibility) — new `rule_id` constant in xref.rs; the rule body uses `FaceIdValue::Known` only. Before writing the rule body, read `reference/textmod_guide.md` to confirm which FaceIDs are template-restricted (e.g., 170/171 = enemy-only). **If the guide does not make a template-restriction claim for a given FaceID, X016 does not flag that FaceID** — no hardcoded lists based on game-design persona claims.
- Author new xref rule **X017** (unknown FaceID) — fires at `Severity::Warning` when a `DiceFace::Active { face_id: FaceIdValue::Unknown(_), .. }` is encountered.

**Verification — specific tests**:
- [ ] `authoring::face_id_try_new_known` — `FaceIdValue::try_new(34)` → `Known(FaceId::DAMAGE)`.
- [ ] `authoring::face_id_try_new_unknown` — `FaceIdValue::try_new(9999)` → `Unknown(9999)` (permissive path).
- [ ] `authoring::pips_try_new_accepts_i16_range` — `Pips::try_new(i16::MIN)`, `Pips::try_new(-1)`, `Pips::try_new(i16::MAX)` all succeed.
- [ ] `ir::diceface_roundtrip_through_newtypes` — `"34-1:0:170-5:0:0:13--1"` parses → emits byte-identical.
- [ ] `xref::x016_flags_template_restricted_face` — a hero using a FaceID the **guide** declares as enemy-only surfaces Finding with `rule_id == "X016"`. Test cites the guide section verbatim in a code comment.
- [ ] `xref::x017_flags_unknown_face_id_as_warning` — an extracted mod containing a `FaceIdValue::Unknown(9999)` produces Finding with `rule_id == "X017"` and `severity == Severity::Warning`. Roundtrip still succeeds.
- [ ] `spec::spec_section_3_6_mentions_unknown_variant` — a smoke test (or doc-test) reads `SPEC.md` and asserts the amended wording is present; prevents silent SPEC rollback.
- [ ] Build-script determinism: `cargo build` twice without touching working-mods produces a byte-identical `face_id_generated.rs`.
- [ ] All 4 working mods IR-equal roundtrip.
- [ ] `cargo doc` with `#![deny(missing_docs)]` on the new `authoring` module renders cleanly — bare `cargo doc` is insufficient as it exits 0 on missing doc-comments.

---

### Chunk 3: `SpriteId` newtype + registry + IR consolidation + drop HashMap
**Spec**: §F4
**Files**:
- `compiler/Cargo.toml` (add `phf` + `phf_codegen` deps)
- `compiler/src/authoring/sprite.rs` (new — newtype)
- `compiler/src/authoring/sprite_registry.rs` (generated by build.rs)
- `compiler/src/authoring/mod.rs` (add submod)
- `compiler/src/lib.rs` (drop HashMap from 5 public fns)
- `compiler/src/ir/mod.rs` (field consolidation on 6 types)
- `compiler/build.rs` (extend from Chunk 2)
- Every extractor sprite-consuming file: `compiler/src/extractor/hero_parser.rs`, `compiler/src/extractor/replica_item_parser.rs`, `compiler/src/extractor/monster_parser.rs`
- Every emitter: `compiler/src/builder/hero_emitter.rs`, `compiler/src/builder/monster_emitter.rs`, `compiler/src/builder/replica_item_emitter.rs`, `compiler/src/builder/derived.rs`
- `compiler/src/util.rs`, `compiler/src/xref.rs`, `compiler/src/ir/ops.rs`

This exceeds the 5-file rule. **Sub-chunk split required** — this chunk breaks into 3a/3b/3c:

#### Chunk 3a: SpriteId newtype + registry + authoring surface
**Files**: `compiler/Cargo.toml`, `compiler/src/authoring/sprite.rs`, `compiler/src/authoring/sprite_registry.rs`, `compiler/src/authoring/mod.rs`, `compiler/build.rs`, `compiler/src/lib.rs` (re-export only, no signature change).
**Dependencies**: Chunk 2.

**Requirements**:
- Add `phf` runtime + `phf_codegen` build-deps.
- Author `SpriteId { name: Cow<'static, str>, img_data: Cow<'static, str> }` per §F4 — with `lookup`, `owned`, `try_registered`, `name()`, `img_data()` accessors.
- Extend `build.rs` to emit `sprite_registry.rs` as a `phf::Map<&'static str, SpriteId>` literal. Mod-priority last-write-wins.
- Serde via `SpriteIdSerde` helper — flat `{name, img_data}` JSON.
- Do NOT change any IR field types yet; IR still uses `sprite_name`/`img_data` strings.

**Verification — specific tests**:
- [ ] `authoring::sprite_lookup_charmander` — `SpriteId::lookup("Charmander")` is `Some` and its `img_data()` matches `working-mods/sliceymon.txt` byte-for-byte.
- [ ] `authoring::sprite_lookup_miss` — `SpriteId::lookup("NoSuchPokemon")` is `None`.
- [ ] `authoring::sprite_owned_roundtrip` — `SpriteId::owned("X", "abcd").name() == "X"` and `img_data() == "abcd"`.
- [ ] `authoring::sprite_serde_roundtrip` — serialize to JSON, parse back, equal.
- [ ] Build-script determinism: `cargo build` twice without touching working-mods produces a byte-identical `sprite_registry.rs`.

#### Chunk 3b: IR field consolidation
**Files**: `compiler/src/ir/mod.rs`, and each of the parsers/emitters listed in the parent chunk (the full set; they must update in lockstep because the IR shape changes).
**Dependencies**: Chunk 3a.

**Requirements**:
- Consolidate fields on `HeroBlock`, `ReplicaItem`, `Monster`, `FightUnit` (renaming `sprite_data` → `sprite`), `AbilityData`, `TriggerHpDef` per the §F4 table.
- Every extractor uses `SpriteId::lookup(name).cloned().unwrap_or_else(|| SpriteId::owned(name, img_data))` — registry miss falls back to owned, not error. SPEC §3.3.
- Every emitter reads `sprite.img_data()` (and `sprite.name()` for the display field).
- xref rules that referenced sprite_name/img_data update to `sprite.name()` / `sprite.img_data()`.
- ir/ops.rs duplicate-name checks unchanged (keyed by `name`, not `sprite`).

**Verification — specific tests**:
- [ ] `ir::heroblock_sprite_required` — compile-time: `HeroBlock { sprite: SpriteId::owned("x", "y"), ... }` compiles; missing `sprite` is a compile error.
- [ ] `ir::serde_breaking_change_on_sprite_shape` — decision made (user ruling 2026-04-20: "no legacy, always choose correctness over back-compat"): **no serde compat shim**. JSON that uses the old `sprite_name` + `img_data` keys fails to deserialize. Test asserts the new flat `sprite` shape is the only accepted JSON form; no dual-representation gymnastics. Plans/PIPELINE_FIDELITY and AUTHORING_ERGONOMICS author IR in code, not from historical JSON, so there are no legacy consumers to break.
- [ ] All 4 working mods IR-equal roundtrip.

#### Chunk 3c: Drop `sprites: &HashMap` from public API
**Files**: `compiler/src/lib.rs`, `compiler/src/main.rs` (if CLI passes sprite maps), `compiler/src/builder/mod.rs`, `compiler/src/builder/hero_emitter.rs` (already takes `&HashMap`; drop it), every callsite in `compiler/tests/` and `compiler/examples/`.
**Dependencies**: Chunk 3b.

**Requirements**:
- Drop `sprites: &HashMap<String, String>` from: `build`, `build_complete`, `build_hero`, `validate_hero`, `validate_hero_in_context` (5 public fns; verified list).
- Update `hero_emitter::emit(hero, sprites)` → `hero_emitter::emit(hero)`.
- Update `main.rs` CLI and every test/example that passes a sprite map.

**Verification — specific tests**:
- [ ] `lib::build_no_sprites_path_b` — build an IR constructed entirely in-memory (Path B) with no sprite map, emits a valid textmod.
- [ ] `lib::build_roundtrip_sliceymon_no_sprites` — roundtrip sliceymon without passing a sprite HashMap.
- [ ] `lib::build_hero_signature` — `build_hero(&hero)` compiles (no HashMap arg).
- [ ] All 4 working mods IR-equal roundtrip.

---

### Chunk 4: `BuildOptions` + `build_with` + `Finding.source` [serial after Chunk 3c, before Chunk 6]
**Spec**: §F5
**Files**: `compiler/src/builder/mod.rs`, `compiler/src/lib.rs`, `compiler/src/xref.rs` (every existing V-rule Finding construction site), every content-emission site in `builder/*.rs`.
**Dependencies**: Chunk 3c.
**Merge ordering**: merges before Chunk 6 to keep `xref.rs` conflicts linear.

**Requirements**:
- Define `BuildOptions`, `SourceFilter`, `SourceSet` per §F5. Hand-written `impl Default for BuildOptions`. `SourceFilter::admits` is `const fn`.
- Introduce `pub fn build_with(ir: &ModIR, opts: &BuildOptions) -> Result<String, CompilerError>`. `build(ir)` is `build_with(ir, &BuildOptions::default())`.
- Every content-emission site in `builder/mod.rs` checks `opts.include.admits(entity.source)` before emitting. Derived structurals are regenerated from post-filter content.
- `Finding.source: Option<Source>` added with `#[serde(default, skip_serializing_if = "Option::is_none")]`. Existing tests that parse Finding JSON continue to work.
- **Retrofit `Finding.source` on every existing V-rule**: V016 (sprite-template compat), V019 (hero pool ref), V020 (color conflict). Plan author runs `rg 'Finding \{ rule_id: "V01[69]"' compiler/src/xref.rs` at chunk start and lists every construction site in the chunk-open note; each site populates `source` from the offending entity's `.source` field. The retrofit is a load-bearing part of this chunk, not a follow-up.
- No X001 demo rule. Duplicate-name detection already lives in `CompilerError::DuplicateName` (ops.rs) and existing xref checks; inventing a parallel rule violates SPEC §3.7.
- Severity promotion: violations on `Source::Base` emit `Severity::Warning`; `Source::Custom`/`Overlay` emit `Severity::Error`. Promotion logic lives in a single helper `fn promote_severity(base: Severity, src: Option<Source>) -> Severity` so every rule applies it identically.

**Verification — specific tests**:
- [ ] `builder::build_with_only_base_omits_overlay` — ModIR with one `Base` hero + one `Overlay` hero, `build_with(ir, &BuildOptions { include: SourceFilter::Only(SourceSet::single(Source::Base)) })` emits only the Base hero.
- [ ] `builder::build_with_exclude_base` — same ModIR, `Exclude(SourceSet::single(Source::Base))` emits only the Overlay hero.
- [ ] `builder::source_filter_admits_const` — `const _: bool = SourceFilter::All.admits(Source::Base);` compiles.
- [ ] `xref::v016_finding_carries_source_for_base` — craft a mod where a `Source::Base` hero trips V016 → Finding has `source == Some(Source::Base)` and promoted `Severity::Warning`.
- [ ] `xref::v016_finding_carries_source_for_custom` — same trigger on `Source::Custom` → Finding has `severity == Severity::Error`.
- [ ] `xref::v019_finding_source_populated`, `xref::v020_finding_source_populated` — same for the other two retrofitted rules.
- [ ] `xref::promote_severity_helper_table` — unit test of the severity-promotion helper across the full cross product (Error × {None, Base, Custom, Overlay}).
- [ ] All 4 working mods IR-equal roundtrip with `BuildOptions::default()`.

---

### Chunk 5: Merge signature → `&mut` + strips derived structurals + two new derived regenerators + unconditional regeneration
**Spec**: §F6
**Files**: `compiler/src/ir/mod.rs` (add `StructuralModifier::is_derived`, add `ModIR.warnings` sidecar), `compiler/src/ir/merge.rs` (signature change + strip logic), `compiler/src/builder/mod.rs` (unconditional regeneration), `compiler/src/builder/derived.rs` (two new generators), `compiler/src/lib.rs` (merge re-export signature), `compiler/src/main.rs` (CLI merge subcommand), `compiler/tests/integration_tests.rs` or new `compiler/tests/path_c_merge_tests.rs`.
**Dependencies**: Chunk 6 (requires `ReplicaItemContainer::Capture { name }` variant — `generate_hero_item_pool` matches on `container` to bucket items per hero).

**Requirements**:
- Add `pub warnings: Vec<Finding>` to `ModIR` with `#[serde(default, skip_serializing_if = "Vec::is_empty")]`.
- Add `pub fn is_derived(&self) -> bool` on `StructuralModifier` (in `ir/mod.rs`) matching on the four derived kinds in §F6.
- Change `merge` signature to SPEC §5's canonical form: `pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError>`. Update `lib.rs` re-export, `main.rs` CLI usage, and every test that calls `merge`. No tuple return; no parallel `merge_with_findings` function.
- `merge` strips derived structurals from both inputs before merging; each strip pushes a `Finding` onto `base.warnings` with `rule_id: "X010"`, `severity: Severity::Warning`, `field_path: Some(...)`, `suggestion: Some("Derived structurals are regenerated at build time; authoring them directly is unsupported.")`.
- Author `generate_pool_replacement(heroes)` and `generate_hero_item_pool(heroes, replica_items)` in `builder/derived.rs`. Byte-for-byte reproduce sliceymon's existing PoolReplacement / hero-bound ItemPool modifiers against the extracted base IR. `generate_hero_item_pool` matches on each `ReplicaItem.container` — `Capture { name }` routes the item into the hero's pool keyed by `name`; `Legendary` is skipped for hero-bound pools (legendaries have their own emission path).
- Remove the "when absent" gate in `build_complete` at `compiler/src/builder/mod.rs:161-168`. `build` itself also strips derived structurals from `ir.structural` before emitting and appends the regenerated forms — build-time regeneration is unconditional.
- Add a Path C integration test that does NOT rely on direct struct-literal construction (authoring layer is empty — use `ir_from_json` or roundtrip-extracted IR, then `Hero::new` from Chunk 1 to add the new hero). The test must not violate SPEC §6.1.

**Verification — specific tests**:
- [ ] `ir::is_derived_truth_table` — every `StructuralType` variant tested; only the four derived kinds return `true`.
- [ ] `merge::strips_derived_char_selection_with_warning` — base has CharacterSelection, overlay has CharacterSelection; after `merge(&mut base, overlay)?`, `base.structural` contains zero CharacterSelection entries and `base.warnings` contains two `X010` findings.
- [ ] `merge::new_signature_compiles` — `merge(&mut base, overlay)?;` compiles; `base.warnings` is readable post-merge.
- [ ] `merge::warnings_accumulate_across_calls` — a second `merge` call appends to (does not reset) `base.warnings`.
- [ ] `derived::pool_replacement_matches_sliceymon` — `generate_pool_replacement(sliceymon.heroes)` byte-matches `working-mods/sliceymon.txt`'s existing PoolReplacement modifier.
- [ ] `derived::hero_item_pool_matches_sliceymon_via_container_enum` — `generate_hero_item_pool` uses `ReplicaItem.container` (the `Capture { name }` variant) to bucket items; byte-matches hero-bound ItemPool in sliceymon.
- [ ] `path_c_merge::adds_hero_regenerates_selector` — load sliceymon IR from JSON, `Hero::new` a new hero, append to `ir.heroes`, `build_complete`, re-extract — new hero is in the regenerated CharacterSelection.
- [ ] All 4 working mods IR-equal roundtrip after `build` strips + regenerates derived structurals.

---

### Chunk 6: `ReplicaItemContainer` enum replaces `container_name` [serial after Chunk 4, before Chunk 5]
**Spec**: §F7
**Files**: `compiler/src/ir/mod.rs` (replace `container_name: String` with `container: ReplicaItemContainer`), `compiler/src/extractor/replica_item_parser.rs` (build the enum variant from source shape), `compiler/src/extractor/classifier.rs` (if modifier-kind classification lives there), `compiler/src/builder/replica_item_emitter.rs` (match on `container` for emission), `compiler/src/xref.rs` (new X003 rule), `compiler/src/ir/ops.rs` (constructor updates + duplicate-name checks), `compiler/src/ir/merge.rs` (match-by-`name` logic already keys on `ReplicaItem.name`, not `container_name` — verify during this chunk that no merge code path reaches into `container_name` directly), serde test fixtures.
**Dependencies**: Chunk 3c, Chunk 4.
**Merge ordering**: merges after Chunk 4; Chunk 5 depends on it.

**Requirements**:
- Define `ReplicaItemContainer { Capture { name: String }, Legendary }` with `#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]`. No default — forcing explicit construction prevents silent misclassification.
- Replace `ReplicaItem.container_name: String` with `pub container: ReplicaItemContainer`. No separate `kind` field. Every callsite handles the enum via `match`, not via string inspection.
- Extractor classifier builds the variant from source-level modifier shape: `itempool.(X).n.Y` → `ReplicaItemContainer::Capture { name: X }`; top-level `item.` → `ReplicaItemContainer::Legendary`.
- `build_replica_item` / emitter matches `container` non-exhaustively (no `_` arm) so adding a future variant is a compile error, not a runtime fallthrough. No `ReplicaItemKindMismatch` runtime check — the type system already rules out the invalid combinations.
- Author new xref rule **X003** (no duplicate Pokemon across heroes / captures / legendaries / monsters — SPEC §6.3) matching on `container` to route items into per-kind buckets (Capture vs Legendary).
- Serde representation is the default enum tagging — `{"Capture": {"name": "PokeBall"}}` or `"Legendary"`. No custom `try_from` needed.

**Verification — specific tests**:
- [ ] `ir::replica_item_capture_carries_container_name` — `let c = ReplicaItemContainer::Capture { name: "PokeBall".into() };` is constructible; `ReplicaItemContainer::Capture` without `name` is a compile error (structural enforcement).
- [ ] `ir::replica_item_legendary_has_no_container_by_construction` — compile-time proof: there is no variant shape that lets a `Legendary` carry a name; trying to pattern-match `ReplicaItemContainer::Legendary { name }` fails to compile.
- [ ] `extractor::classifies_capture_vs_legendary_into_enum` — sliceymon capture `itempool.(X).n.Y` extracts to `ReplicaItemContainer::Capture { name: "Y" }`; a legendary extracts to `ReplicaItemContainer::Legendary`.
- [ ] `xref::x003_duplicate_pokemon_across_kinds` — a ModIR with Pokemon "Pikachu" as both Hero and Capture surfaces `Finding { rule_id: "X003", .. }`; `source` field populated from the offending entity.
- [ ] `builder::replica_item_emitter_non_exhaustive_match` — a doc-test or `// SAFETY: non-exhaustive` comment + a test that adding a hypothetical new variant produces a `cargo check` failure at the emitter callsite.
- [ ] `serde::replica_item_container_json_shape` — `Capture { name: "X" }` serializes as `{"Capture":{"name":"X"}}`; `Legendary` as `"Legendary"`; round-trips both.
- [ ] All 4 working mods IR-equal roundtrip (every existing ReplicaItem correctly classified).

---

### Chunk 7: `unwrap`/`expect`/`panic` elimination in lib code [after Chunk 3c; parallel with Chunk 5]
**Spec**: §F8
**Files** (indicative baseline audited 2026-04-20; re-audit immediately before starting):
- `compiler/src/builder/hero_emitter.rs` (2 — post-Chunk 3c, signatures have already dropped the `sprites` arg)
- `compiler/src/extractor/fight_parser.rs` (1)
- `compiler/src/extractor/reward_parser.rs` (1)
- `compiler/src/extractor/hero_parser.rs` (1)
- `compiler/src/extractor/phase_parser.rs` (1)

Callers in the same files or their module roots that previously could not fail may now return `Result`; update the call chain in this chunk.

**Dependencies**: Chunk 3c (shares `builder/hero_emitter.rs` with 3c — running earlier would force 7's edits to be redone after 3c lands).
**Parallel with**: Chunk 5 only (5 writes `ir/merge.rs`, `builder/derived.rs`, `builder/mod.rs`; 7 writes extractor files + the post-3c `hero_emitter.rs`; no overlap).

**Note**: `compiler/src/ir/mod.rs:284`'s `panic!("Expected Item segment")` is inside `#[cfg(test)]` (test `typed_entries_populated`). It is NOT lib code and is NOT in scope for this chunk. The original plan's citation was wrong.

**Requirements**:
- **Re-audit at chunk start**, not at plan-write time. Run `rg '\.unwrap\(\)|\.expect\(|panic!\(|unimplemented!|todo!\(' compiler/src/` with `#[cfg(test)]` stripping (use a small helper script or `cargo xtask`) and record the actual count in the chunk-open note. Fix every hit found — the "6 across 5 files" figure is indicative only. Running Chunk 7 after Chunk 3c fixes the baseline-drift problem: 3c is the last chunk that may introduce new error paths in the files 7 touches.
- Replace each lib-code `unwrap()` / `expect(...)` / `panic!(...)` / `unimplemented!(...)` / `todo!(...)` with a `?` propagation returning `CompilerError` with `field_path` + `suggestion` populated via `ErrorKind::{Build, Paren, HeroParse, PhaseParse, RewardParse, ChainParse}` from §F0.
- `ir::DiceFaces::parse` and friends currently use `.unwrap_or` patterns (no panic) — out of scope; do not touch.

**Verification — specific tests**:
- [ ] `audit_no_lib_panic_or_unwrap` — an `xtask` / `build.rs` check (not a unit test, which can't meaningfully grep the workspace) that greps `compiler/src/**/*.rs` with test-module stripping and fails CI if any hit is found. SPEC §8.
- [ ] `extractor::hero_parser_malformed_propagates_error` — the exact input that previously panicked now returns `Err(CompilerError { kind: ErrorKind::HeroParse { .. }, .. })`.
- [ ] `extractor::fight_parser_malformed_propagates_error` — same for `fight_parser.rs`.
- [ ] `extractor::reward_parser_malformed_propagates_error` — same for `reward_parser.rs`.
- [ ] `extractor::phase_parser_malformed_propagates_error` — same for `phase_parser.rs`.
- [ ] `builder::hero_emitter_pathological_input_propagates_error` — covers both hits in `hero_emitter.rs`.
- [ ] All 4 working mods IR-equal roundtrip.

---

## Final Verification

- [ ] `cargo test --all` passes with no regressions.
- [ ] IR-equal roundtrip holds for all 4 working mods (`cargo run --example roundtrip_diag` is empty).
- [ ] `cargo doc` with `#![deny(missing_docs)]` on the new `authoring` module renders cleanly.
- [ ] `cargo run -- schema` produces a JSON Schema that includes `FaceId`, `FaceIdValue`, `Pips`, `SpriteId`, and `ReplicaItemContainer` types.
- [ ] `compiler/src/authoring/` contains only: `mod.rs`, `face_id.rs`, `face_id_generated.rs`, `sprite.rs`, `sprite_registry.rs`. No builders, no macros, no `HeroReplica` — those are owned by `AUTHORING_ERGONOMICS_PLAN.md`.
- [ ] No `std::fs` / `std::process` in `compiler/src/authoring/` or any other library file.
- [ ] Lib-code audit (`rg` with `#[cfg(test)]` stripping, enforced via `xtask`/`build.rs`) shows zero `unwrap()` / `expect()` / `panic!` / `unimplemented!` / `todo!` hits.
- [ ] Every new xref rule (X003, X010, X016, X017) populates `field_path`, `suggestion`, and `source` on its `Finding`s. Every existing V-rule (V016, V019, V020) populates `source`.
- [ ] `merge` signature: `pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError>` (matches SPEC §5 verbatim). Warnings surface via `ModIR.warnings: Vec<Finding>`.
- [ ] `ReplicaItem` has no `container_name: String` field and no `kind: ReplicaItemKind` field; the only container-related field is `container: ReplicaItemContainer`.
- [ ] SPEC §3.6 has been amended to name the permissive whitelist + `Unknown(raw)` variant as the spec-blessed design; SPEC §3.6's pips type annotation reads `i16`.

---

## Rulings (durable, executed as part of this plan)

All rulings below are implemented by specific chunks in this plan. Once the plan executes, the code is the source of truth (SPEC §7); these entries exist so a future reader can see *why* the code looks the way it does without reconstructing the decision history. No residual user decisions remain open.

**R1. FaceId/SpriteId permissive whitelist.**
Ruling: permissive. Any valid textmod per `reference/textmod_guide.md` must extract, even if it uses FaceIDs outside the working-mods corpus. Rationale: SPEC §1 (general-purpose mod-building backend) + SPEC §3.3 (self-contained IR) require that new mods with novel values round-trip. Implementation: `FaceIdValue::{Known(FaceId), Unknown(u16)}` + `SpriteId::owned(name, img_data)`. SPEC §3.6 is amended in Chunk 2's commit to name the `Unknown(raw)` variant as the spec-blessed design — no SPEC drift.

**R2. `ReplicaItem` container shape.**
Ruling: collapse into `ReplicaItemContainer { Capture { name: String }, Legendary }` rather than `kind: ReplicaItemKind` + `container_name: Option<String>`. Rationale: SPEC §3.6 (invalid states unrepresentable) + §3.7 (no parallel representations). Implementation: Chunk 6. No runtime `ReplicaItemKindMismatch` error variant — the type system enforces the invariant.

**R3. `build_with` introduction.**
`build_with` does not exist today. Chunk 4 introduces it; `build(ir)` becomes `build_with(ir, &BuildOptions::default())`. SPEC §5 already sketches `build_with`, so no SPEC amendment needed.

**R4. `merge` signature retained per SPEC §5.**
Ruling: `pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError>` (SPEC §5 verbatim). Warnings surface through a new `ModIR.warnings: Vec<Finding>` sidecar, not via a tuple return. An earlier draft's `Result<(ModIR, Vec<Finding>), CompilerError>` was rejected because it contradicted SPEC §5; the sidecar is the spec-conforming way to carry findings. Implementation: Chunk 5.

**R5. `Pips` type annotation.**
Ruling: `Pips(i16)`. Rationale: corpus contains negative pips (`ir/mod.rs:50` comment); `DiceFace::Active.pips: i16` is already current. SPEC §3.6 amended in Chunk 2's commit (simultaneous with R1's amendment) to read `pips: i16`. No corpus-derived range bound — `try_new` accepts the full `i16` range.

**R6. `FightUnit` sprite name derivation.**
Ruling: under R1's permissive path, the name for `SpriteId::owned(name, img_data)` is sourced from the required `FightUnit.name: String` field (`ir/mod.rs:1181`). Generic unit names (e.g., `"Boss"`) flow through as novel `SpriteId::owned` entries without colliding with the registry. §F4 table reflects this.

**Memory hygiene.** After this plan executes, any memory file that still references older unresolved escalations (E1/E5/E6 wording, tuple-return `merge`, `kind`+`container_name` parallel fields) must be deleted or updated to match the ruling table above. A stale memory is worse than no memory.
