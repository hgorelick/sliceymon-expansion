# Platform Foundations Plan

## Overview

This plan delivers the IR schema, type-system, and build-API foundations that `PIPELINE_FIDELITY_PLAN.md` (parser/emitter drift fixes) and `AUTHORING_ERGONOMICS_PLAN.md` (author-from-scratch surface) both depend on. It does not own any drift-class fix and does not own the authoring builder API. It owns:

- Structured-error refactor of `CompilerError` (prerequisite for every other chunk that cites `field_path` / `suggestion`).
- `Default` + `::new(identity)` on authorable IR types where a safe default exists post-F4; explicit list of types that cannot derive `Default`.
- The empty `compiler/src/authoring/` module skeleton.
- Typed whitelist newtypes `FaceId` and `SpriteId` with their generators, and the new `Pips` newtype used by `DiceFace::Active`.
- IR field consolidation that makes sprite-bearing types self-contained (SPEC §3.3).
- `ReplicaItem` container-shape refactor (§F7) — **superseded by Chunk 9 (2026-04-23)**: Chunk 6 shipped a `ReplicaItemContainer { Capture { name }, Legendary }` enum, then Chunk 9 deleted it per chunk-impl rule 3 (zero corpus instances for `Capture`). Today `ReplicaItem` models Legendaries only; Captures route as `ItemPool` structurals at the classifier. Keep this bullet in the goal list only as a historical pointer — the live shape is the no-`container` struct.
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
    // Boxed to keep `Result<T, CompilerError>` small (clippy::result_large_err):
    // the largest ErrorKind variants reach ~100 bytes inline, which combined
    // with the three Option<String> fields below pushes the struct past the
    // lint threshold on every public API that returns `Result<T, CompilerError>`.
    // Destructure via `err.kind.as_ref()` rather than by value.
    pub kind: Box<ErrorKind>,
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

`ReplicaItemKindMismatch` is intentionally absent. Originally justified by §F7's `ReplicaItemContainer` enum making the former Capture-without-container / Legendary-with-container invariants unrepresentable. After Chunk 9 deleted the `Capture` variant entirely (chunk-impl rule 3), there is only one replica shape (Legendary) and the invariant class is vacuous — still no runtime error variant needed.

Constructor helpers (`CompilerError::build(component, message)`, `::paren(...)`, etc.) build the common fields in one call. The `Display` impl handles `field_path` / `suggestion` printing in a single shared tail. Every existing construction site migrates to the new shape in the same PR (the extractor, builder, and ops files).

This refactor is a prerequisite for every subsequent chunk that asserts errors carry `field_path` and `suggestion`. It replaces the existing type — no parallel error type.

### F1. `Default` + `::new(identity)` on authorable IR types

Types that can derive `Default` **today** (all fields already have safe defaults): `FightUnit` (already does), `Boss`, `FightDefinition`.

Types that **cannot** derive `Default` after F3/F4 because they gain `FaceId` / `SpriteId` / required-identity fields:

| Type | Blocker | Resolution |
|---|---|---|
| `Hero` | `color: char` has no safe default (any char we pick creates a false "valid hero") | No `Default`. `::new(internal_name, mn_name, color)` is the only construction. |
| `HeroBlock` | `sprite: SpriteId` post-F4 (no safe empty whitelist entry), `sd: DiceFaces` | No `Default`. `::new(template, sprite, sd)` — `tier`, `hp`, `color` default via options. |
| `ReplicaItem` | `sprite: SpriteId` post-F4. Post-Chunk-9, `ReplicaItem` models Legendaries only (no `container` field) — the former §F7 `ReplicaItemContainer::Capture` variant was deleted per chunk-impl rule 3. | No `Default` (sprite is required). `::new(name, template, sprite, sd)`. |
| `Monster` | `sprite: Option<SpriteId>` (Option — so a none-sprite monster is expressible) | `Default` allowed; `sprite` defaults to `None`. |
| `AbilityData` | `sprite: Option<SpriteId>` post-F4 | `Default` allowed. |
| `TriggerHpDef` | `sprite: Option<SpriteId>` post-F4 | `Default` allowed. |
| `DiceFaces` | `Vec<DiceFace>` — safe default is empty vec | `Default` allowed. |

`::new` signatures (strings accept `impl Into<String>`; every field not in the signature is set via `..Default::default()` for types that derive `Default`, or via an explicit per-field default list in the impl for types that don't):

- `Hero::new(internal_name: impl Into<String>, mn_name: impl Into<String>, color: char) -> Self`
- `HeroBlock::new(template: impl Into<String>, sprite: SpriteId, sd: DiceFaces) -> Self`
- `ReplicaItem::new(name: impl Into<String>, template: impl Into<String>, sprite: SpriteId, sd: DiceFaces) -> Self` (Legendary-only post-Chunk-9)
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
| Hero-bound ItemPool | `StructuralModifier` entries of kind `ItemPool` whose wrapping container-`.n.NAME` matches a hero `internal_name` — Captures currently route as `ItemPool` structurals at the classifier, not as `ReplicaItem`s (chunk-impl rule 3: no corpus instance for a Capture replica variant). | `builder::derived::generate_hero_item_pool` | **to build** |

Chunk 5 authors the two missing regenerators. Acceptance criterion for each: running it against `working-mods/sliceymon.txt` reproduces the base mod's existing PoolReplacement / hero-bound ItemPool modifiers byte-for-byte. If it doesn't, the chunk is incomplete. **Note (post-Chunk-9, 2026-04-23)**: the original plan prescribed bucketing hero-bound items by matching `ReplicaItem.container: ReplicaItemContainer::Capture { name }`. That variant was deleted per chunk-impl rule 3; Chunk 5 now buckets by parsing the `.n.NAME` out of `ItemPool` structural modifiers (the shape the classifier actually produces for Captures).

**Classification helper**: add `fn is_derived(&self) -> bool` to `StructuralModifier` in `ir/mod.rs` (not `builder/derived.rs`, because merge lives in `ir/`). Matches on `modifier_type` against the four derived kinds above.

`merge` strips any of these from both base and overlay inputs before copying content. Each strip pushes a `Finding` onto `base.warnings` at `Severity::Warning` with `rule_id: "X010"` (new rule; NOTE: X003, X016, X017 are also new — all `X*` IDs in this plan are new rules; X001 from earlier drafts has been removed because duplicate-name rejection already lives in `CompilerError::DuplicateName` + existing xref checks), `field_path` naming the modifier, and `suggestion: "Derived structurals are regenerated at build time; authoring them directly is unsupported."`

`build` and `build_complete` regenerate derived structurals from the post-merge content set unconditionally. The "when absent" gate in `build_complete` (`compiler/src/builder/mod.rs:161-168` — the `if !ir.structural.iter().any(|s| s.modifier_type == StructuralType::Selector)` and `HeroPoolBase` checks) is REMOVED: regeneration strips any pre-existing derived structural from `ir.structural` first, then appends the regenerated form. `build` (not just `build_complete`) also performs this strip-and-regenerate.

### F7. `ReplicaItemContainer` enum — SUPERSEDED by Chunk 9 (2026-04-23)

**Status**: enum existed between Chunk 6 (2026-04-21) and Chunk 9 (2026-04-23). Deleted in chunk 9 commit `77b802b` per chunk-impl rule 3 ("Every IR variant discriminator must have at least one corpus instance per variant before it ships. Zero instances for a variant means the variant is a hypothesis masquerading as a model — delete the variant rather than carry unevidenced cases"). No corpus mod contained a Capture-shaped `ReplicaItem`; the classifier never produced `ModifierType::ReplicaItem` / `ReplicaItemWithAbility`; Captures route as `ItemPool` structurals. Both variants and all Capture parsing/emitting paths were deleted.

**Current state**: `ReplicaItem` models Legendaries only. There is no `container` field. The former `Capture { name: String }` data is recovered at extraction time from the source-level `itempool.(X).n.Y` as an `ItemPool` structural modifier; its `.n.NAME` is parsed out by downstream consumers (e.g. the hero-bound ItemPool regenerator in Chunk 5) rather than carried as a typed IR variant.

**Do not resurrect**: the chunk-impl rule is explicit. If a future corpus instance appears (e.g. if `reference/textmod_guide.md` grows a case that requires a typed Capture replica), model the new variant against that instance's actual shape — not this section's pre-chunk-9 design.

X003 (no duplicate Pokemon across hero / legendary / monster buckets — SPEC §6.3) still exists and still fires; its "legendary" bucket label is now the only replica-kind bucket (`compiler/src/xref.rs:230-234`).

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

### F9. V020 restructure — remove overlap with X003 on cross-bucket Pokemon

Post-F7 (and post-Chunk-9, which deleted the `Capture` variant per chunk-impl rule 3), `X003` (SPEC §6.3) owns cross-bucket Pokemon uniqueness across `{hero, legendary, monster}`. `V020`'s `check_cross_category_names` runs the same collision pass on the same data + bosses, so any `hero↔replica`, `hero↔monster`, or `replica↔monster` duplicate emits both V020 and X003 — two findings for one defect. SPEC §3.7 forbids parallel representations; this is a parallel-rule instance.

Fix: V020's `check_cross_category_names` keeps its 4-bucket collection (hero, replica, monster, boss) for single-scan efficiency, but **skips emission when the distinct colliding bucket set is a subset of `{hero, replica_item, monster}` with cardinality ≥2** — X003 owns that slice. V020 still fires for every case X003 cannot own:

- Any collision that includes a boss (`hero↔boss`, `replica↔boss`, `monster↔boss`, `boss↔boss`). SPEC §6.3 scopes Pokemon uniqueness to `{hero, replica items, monster}`; bosses are not Pokemon, so X003 does not cover them.
- Any intra-bucket duplicate (two heroes same name, two replicas same name, two monsters same name, two bosses same name). X003 was tightened in Chunk 6 to require ≥2 distinct buckets, so intra-bucket duplicates are V020's sole territory.

X003 is unchanged. Each defect now surfaces under exactly one rule ID.

**Scope note.** The single-item CRUD checks (`check_hero_in_context`, `check_boss_in_context` at `xref.rs:593-648` and `:656-714`) validate *one new item* against a loaded IR and do not produce the double-fire (X003 is a whole-IR rule, not a per-item one). They are explicitly out of scope for F9.

### F10. Legendary-parser chain-and-ability-interior leakage — depth-aware scalar extraction

**Scope (post-Chunk-9, 2026-04-23)**: this section originally covered all three replica parsers (`parse_simple`, `parse_with_ability`, `parse_legendary`). The first two were deleted in Chunk 9 along with the `Capture` container variant (chunk-impl rule 3: no corpus instance). The leak class as it applies to live code is now Legendary-only.

Chunk 6's round-3 tribunal (PR #7) fixed three Legendary-specific parse leaks by scoping `parse_legendary`'s scalar extractors to a `before_cast` slice (now `before_ability`). One class of leak was identified but deferred and was owned here: chain sub-entries can emit free-form text (sidesc, enchant — see `compiler/src/builder/chain_emitter.rs:59-63`) that may contain literal `.hp.` / `.col.` / `.sd.` / `.img.` substrings at paren-depth 0, and ability bodies (`.abilitydata.(...)`) carry the same substrings at ability-interior depth. A non-depth-aware extractor finds matches inside either region, silently flipping `None` top-level fields to `Some(<interior value>)` at parse time.

**Why this is a §F / SPEC issue**: SPEC §3.3 ("self-contained IR — extracted IR has everything required to rebuild the mod") is the authority: any valid modifier emitted by this compiler's own emitters must round-trip, chain and ability content included. The plan's Chunk 3b lesson (item 2) already blessed this failure mode as the right hunt: *"IR-equality baselines alone are insufficient. Add at least one test whose failure mode is source-vs-IR divergence, not IR-vs-IR divergence."*

**Fix (as shipped)**: two tools in `util.rs` close the class for `parse_legendary`:

1. `util::slice_before_chain_and_cast(body: &str) -> &str` — returns the longest prefix of `body` that precedes the first depth-0 occurrence of any marker in the canonical set **§F10-MARKERS** = {`.i.`, `.sticker.`, `.abilitydata.`}. When no such marker exists at depth 0, returns the full slice. Correctness follows from emission field order: top-level scalars are always emitted before the chain and ability blocks. The ability-body marker is `.abilitydata.` per the textmod guide (`reference/textmod_guide.md` lines 747 / 857 / 975-981); `cast.TRIGGER` in the corpus is a chain keyword (guide lines 642-645), not a property marker, so `.cast.` is not a member of §F10-MARKERS.
2. `util::extract_color(content, depth_aware: bool)` — `depth_aware: bool` flag mirroring `extract_hp` / `extract_sd`. `parse_legendary` passes `depth_aware = true` (its scalar prefix is flat at depth 0, so a depth-1 `.col.` is always chain / ability drift); hero / monster / boss parsers pass `depth_aware = false` (scalars live inside nested paren groups in those shapes, and the legacy permissive semantics stay).

The original §F10 also prescribed a third helper, `util::replica_inner_body`, to scope Capture-shape parsers to the body inside `((hat.replica…))`. That helper was deleted in Chunk 9 along with the Capture parsers themselves.

**Non-scope**: this chunk does not collapse the extract path and the authoring path (SPEC §3.3 / §6.1). Both remain split. The helper and the flag bit only exist for the extract side; the authoring side has no scalar-hunt problem to solve.

**Emission-order co-requirement**: §F10 depends on `emit_legendary` placing every top-level scalar field (`.col.`, `.hp.`, `.sd.`, `.img.`) *before* the chain / `.abilitydata.` region. Without the reorder, an emit→parse cycle where the chain carries a decoy `.sd.` / `.img.` and the real value comes *after* the chain would silently lose the real value to first-match. Shipped: `emit_legendary` emits scalars before the chain (`compiler/src/builder/replica_item_emitter.rs:29-45`).

**Structural check (per the chunk-impl hook)**: the two tools are not parallel representations. `slice_before_chain_and_cast` narrows *where* to scan (prefix before chain/ability); `depth_aware` narrows *how* to scan within the slice. They compose — each guards a different leak surface (chain/sidesc text vs. paren-nested decoys), neither duplicates the other. Removing either reopens a leak surface.

---

## Implementation Plan

### Lessons from prior chunks (read before authoring a new chunk)

- **Chunk 3b (2026-04-21) — plan contradicted itself on extractor registry use.** §F4 line 236 said *"Every extractor sprite-consuming parser calls `SpriteId::owned(name, img_data)`"*; the Chunk 3b requirements block said *"Every extractor uses `SpriteId::lookup(name).cloned().unwrap_or_else(...)`"*. Implementation followed the chunk-level wording and shipped lookup-first in 8 callsites, silently replacing source `.img.` with sliceymon's registry entry on any name collision — invisible to baselines (deterministic lookup → IR equality holds on both passes of the roundtrip). Caught only by an adversarial tribunal that constructed a collision input.

  **Takeaway for future chunks:**
  1. When a chunk's "Requirements" block restates a §F-level contract, the restatement must be **strictly narrower or identical**, never contradictory. If the chunk needs different semantics, update the §F-level contract first (same PR) so there is one canonical statement.
  2. For any semantic invariant that's stable under idempotent operations (extract→build→extract, merge idempotence, signature-drop equivalence), **IR-equality baselines alone are insufficient.** Add at least one test whose failure mode is *source-vs-IR divergence*, not IR-vs-IR divergence. For the extract path this means: craft an input whose interpretation would differ if the parser consulted a derived/canonical data source, and assert the parser preserves source bytes.
  3. Path A (extract, permissive, source-preserving) and Path B (authoring, strict, registry-gated) must stay split at every callsite. If a chunk proposes collapsing them, that's a SPEC §3.3 / §6.1 amendment, not a chunk detail.
  4. Duplicating an identical 4-line incantation across N parser callsites is a plan smell. Either the incantation belongs in one helper (extract the helper as part of the chunk), or there's only one correct line to write (use that one line). Duplication encodes the incantation's wrongness N times; a helper at least concentrates it.

### Checkpoint Configuration
- Total chunks: 12 (0, 1, 2, 3a, 3b, 3c, 4, 5, 6, 7, 8, 9).
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
| 8 (V020 restructure — drop cross-bucket Pokemon overlap with X003) | — | — | — | — | **yes** (V020 emission narrowed) | — | — | — |
| 9 (replica-parser chain-and-depth-aware scalar extraction) | — | — | — | — | — | — | — | — |

**True dependency graph (after the conflict matrix):**

```
Chunk 0 (CompilerError)  ✅ COMPLETE (2026-04-21)
  ├── Chunk 1 (Default + ::new + authoring skeleton)  ✅ COMPLETE (2026-04-21)
  │     └── Chunk 2 (FaceId/Pips + IR flip)            ✅ COMPLETE (2026-04-21)
  │           └── Chunk 3a (SpriteId + registry)        ✅ COMPLETE (2026-04-21)
  │                 └── Chunk 3b (IR field consolidation) ✅ COMPLETE (2026-04-21, PR #5 merged after round-2 tribunal)
  │                       └── Chunk 3c (drop HashMap from public API) ✅ COMPLETE (2026-04-21, PR #6 merged)
  │                             ├── Chunk 4 (BuildOptions + build_with + Finding.source) ✅ COMPLETE (2026-04-22)
  │                             └── Chunk 6 (ReplicaItemContainer enum — SUPERSEDED by Chunk 9) ⚠️  Chunk 9 deleted both variants per chunk-impl rule 3
  │                                   ├── Chunk 5 (merge strips + new regenerators + unconditional regen) — dependency on Chunk 6's enum is moot; now keys off `ItemPool` structural `.n.NAME`
  │                                   ├── Chunk 8 (V020 restructure — remove cross-bucket Pokemon overlap) [needs 4; Chunk 6's X003 survives, bucketing was simplified to name-based]
  │                                   └── Chunk 9 (Legendary-parser chain-and-depth-aware scalar extraction + Capture variant deletion) ✅ COMPLETE (2026-04-23)
  └── Chunk 7 (lib-code unwrap/expect/panic elimination) [parallel from Chunk 0 completion onward; no shared files with 1..6]
```

**Parallel groups (corrected):**
- **Sequential foundation**: Chunk 0 → Chunk 1 → Chunk 2 → Chunk 3a → Chunk 3b → Chunk 3c.
- **Merge checkpoint after Chunk 3c**: Chunks 4 and 6 both touch `xref.rs`, and Chunk 7 touches `builder/hero_emitter.rs` which Chunk 3c also edits. To avoid merge-conflict gymnastics, the branches are prepared in parallel but merged **sequentially in this order**: 4 → 6 → 7. Each merge re-runs the local verification tests before the next merges. This is a **critical checkpoint** per AI-dev persona §Designing for Parallel Multi-Agent Execution rule 6.
- **Sequential after Chunk 6**: Chunk 5 (originally depended on Chunk 6's `ReplicaItemContainer` enum for `generate_hero_item_pool` bucketing; post-Chunk-9 the dependency is vacuous — bucketing now parses `.n.NAME` from `ItemPool` structural modifiers since the `Capture` variant was deleted per chunk-impl rule 3).
- **Sequential after Chunks 4 and 6**: Chunk 8 (needs X003 from Chunk 6 to defer to, and V020's `Finding.source` shape from Chunk 4 so its test assertions compile). Parallel with Chunks 5 and 7 — Chunk 8 only touches `xref.rs`; Chunk 5 writes `ir/merge.rs` / `builder/derived.rs` / `builder/mod.rs`; Chunk 7 writes extractor lib-code + post-3c `hero_emitter.rs`. No overlap.
- **Sequential after Chunk 6**: Chunk 9 (originally: depends on Chunk 6's `before_cast` scoping pattern in `parse_legendary`; generalize into a `util.rs` helper and apply to `parse_simple` / `parse_with_ability`). **As shipped (2026-04-23)**: Chunk 9 generalized the scoping for `parse_legendary` only, then deleted `parse_simple` / `parse_with_ability` / `emit_simple` / `emit_with_ability` / `ReplicaItemContainer` per chunk-impl rule 3 (no corpus instance for any Capture-shape `ReplicaItem`). Capture modifiers continue to route as `ItemPool` structurals, where they already did in production. Chunk 9 touched `compiler/src/util.rs` + `compiler/src/extractor/replica_item_parser.rs` + `compiler/src/builder/replica_item_emitter.rs` + `compiler/src/ir/mod.rs` (delete `ReplicaItemContainer`) + `compiler/src/ir/ops.rs` + `compiler/src/xref.rs` + `compiler/src/extractor/{classifier,mod}.rs` + `compiler/tests/{build_options_tests,integration_tests}.rs`. Parallel with Chunks 5, 7, 8 — Chunk 5 and 8 now have reduced dependencies on Chunk 9's output (no `ReplicaItemContainer` to match on).
- **Chunk 7 scope**: runs after Chunk 3c lands (not from round 2), because 3c touches `hero_emitter.rs` which 7 also edits. Running 7 earlier would require re-doing 7's edits after 3c rewrites the function signatures. Chunk 7's scope is therefore fixed at audit time (immediately before it starts), not at plan-write time — the "6 across 5 files" baseline at plan-write is indicative, not binding.

**Honest minimum wall-clock rounds**: 9 rounds for the main track (0 → 1 → 2 → 3a → 3b → 3c → 4 → 6 → {5, 7, 8, 9 parallel — no shared files once 3c/4/6 land}). Chunk 5 touches `ir/merge.rs`, `builder/derived.rs`, `builder/mod.rs`; Chunk 7 touches lib-code panic sites in extractor/ and the post-3c `hero_emitter.rs`; Chunk 8 touches `xref.rs` only; Chunk 9 touches `util.rs` + `extractor/replica_item_parser.rs` only. All four are truly parallel at that point.

The original plan's "Minimum wall-clock rounds: 3" was false — Chunks 2, 3, and 5/6/7 were not actually parallelizable, and Chunk 3's 11-file scope required sub-chunking.

The original plan's "parallel groups A/B/C" are false — Chunk 2 and Chunk 3 both write `ir/mod.rs`, `build.rs`, and `authoring/mod.rs`. Chunk 5 originally depended on Chunk 6's `ReplicaItemContainer` enum for `generate_hero_item_pool` bucketing; post-Chunk-9 (2026-04-23) the enum has been deleted per chunk-impl rule 3, and Chunk 5 now parses `.n.NAME` out of `ItemPool` structurals instead. These parallelism claims were wrong and are corrected here.

---

### Chunk 0: `CompilerError` structured-error refactor [PREREQUISITE] — ✅ COMPLETE (merged 2026-04-21)
**Spec**: §F0
**Files**: `compiler/src/error.rs`, every construction site across `compiler/src/extractor/*.rs`, `compiler/src/builder/*.rs`, `compiler/src/ir/ops.rs`, `compiler/src/ir/merge.rs` (sig retains `Result<_, CompilerError>`), `compiler/src/xref.rs` (only `Finding` sites — `Finding.field_path` / `.suggestion` already exist).
**Dependencies**: None. Blocks every subsequent chunk.

**Delivered** (merged via PR #1, branch `refactor/structured-errors-and-spec`):
- `compiler/src/error.rs` — `CompilerError { kind: Box<ErrorKind>, field_path, suggestion, context }` struct replaces the old enum-of-variants. 18 `ErrorKind` variants landed: `Split`, `Classify`, `HeroParse`, `Paren`, `Build`, `MergeConflict`, `SpriteNotFound`, `FaceIdInvalid`, `DerivedStructuralAuthored`, `Validation`, `DuplicateName`, `DuplicateColor`, `NotFound`, `ChainParse`, `PhaseParse`, `RewardParse`, `Io { kind, message }`, `Json { line, column, message }`. `Box<ErrorKind>` keeps `Result<T, CompilerError>` small (clippy::result_large_err).
- 18 constructor helpers; `.with_field_path(..)` / `.with_suggestion(..)` / `.with_context(..)` builders; `Display` impl prints field-path/suggestion/context tails.
- `From<io::Error>` preserves `io::ErrorKind`; `From<serde_json::Error>` preserves line/column.
- Constructor-site migrations landed in: `splitter`, `classifier`, `hero_parser`, `phase_parser`, `reward_parser`, `richtext_parser`, `hero_emitter`, `phase_emitter`, `ir/ops`, `main`.
- `compiler/src/main.rs` — three redundant `.map_err(...)` sites dropped; plain `?` routes via `From` impls.
- `compiler/src/extractor/fight_parser.rs` — dead `in_chain` variable removed.
- `compiler/src/ir/merge.rs` — verified no-op: function is `Ok`-only, signature retains `Result<_, CompilerError>` per API contract, zero `CompilerError` construction sites to migrate.
- `compiler/src/xref.rs` — verified clean: produces `Finding`s only, no `CompilerError` construction. All 8 `Finding` construction sites populate both `field_path` and `suggestion`; messages carry only semantic detail (no buried structured info to lift).

**Tests landed**:
- `error::test_display_includes_field_path`
- `error::test_display_includes_suggestion`
- `error::test_existing_variants_migrate_cleanly`
- `error::test_with_context_appears_in_display`
- 4 per-mod roundtrip baselines in `compiler/tests/baselines/roundtrip/<mod>.baseline` (heroes clean on all 4; replica items clean; monsters clean; bosses drift on all 4 — tracked by `PIPELINE_FIDELITY_PLAN.md`, out of scope here).
- Gate state at merge: `cargo build` 0 warnings, `cargo clippy` 10 lib warnings (down from 14), `cargo test` 257 pass (+4 roundtrip baselines).

**Follow-ups for subsequent chunks**:
- `ErrorKind::FaceIdInvalid` and `ErrorKind::DerivedStructuralAuthored` already exist — Chunk 2 (F3) and Chunk 5 (F6) wire them up; do not re-add.
- `Finding.field_path` currently uses `heroes[<name>].color` / `hero[<name>].mn_name` style paths (names as subscripts). Chunk 4 (F5) and later chunks that tighten xref rules may switch these to index-based paths if needed; Chunk 0 deliberately did not normalize path format.
- No `ReplicaItemKindMismatch` variant — Chunk 6 (F7) originally made the invariants unrepresentable via `ReplicaItemContainer`; Chunk 9 then deleted the `Capture` variant entirely (chunk-impl rule 3). `ReplicaItem` is Legendary-only; the invariant class is vacuous; no runtime variant needed.

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
- No unit test in `authoring/` skeleton — compilation is the gate (`pub mod authoring;` in `lib.rs` fails to build if the file is removed); Chunk 2 populates the module with real constructors and their tests.
- [ ] All 4 working mods IR-equal roundtrip.
- [ ] `cargo test` passes.

---

### Chunk 2: `FaceId` + `Pips` newtypes + whitelist + IR flip + SPEC §3.6 amendment [CHECKPOINT] — ✅ COMPLETE (2026-04-21)

**Delivered**:
- SPEC §3.6 amended: format-invariant vs corpus-whitelist distinction; `pips: u8` → `i16`.
- `compiler/build.rs` new — deterministic `.sd.` harvest into `BTreeMap<u16, FaceIdMeta>`, emits `$OUT_DIR/face_id_generated.rs` (byte-identical across rebuilds, verified).
- `compiler/src/authoring/face_id.rs` — `FaceId`, `FaceIdValue::{Known, Unknown}`, `Pips`, `FaceIdError`. `#![deny(missing_docs)]` scoped to `authoring/` and passes.
- `DiceFace::Active { face_id: FaceIdValue, pips: Pips }` — single-pass flip across parser + emitter + tests.
- `xref.rs` X016 (template-compat framework with empty production table — guide makes no per-FaceID restriction claim today) + X017 (`Severity::Warning` on `Unknown(_)`).
- `compiler/tests/spec_amendments.rs` — smoke tests pinning the SPEC wording.

**Verification (plan checklist)**:
- [x] `authoring::face_id::tests::face_id_try_new_known` / `face_id_try_new_unknown` / `pips_try_new_accepts_i16_range`
- [x] `ir::tests::diceface_roundtrip_through_newtypes` (+ `diceface_unknown_face_id_roundtrips` for Unknown)
- [x] `xref::tests::x016_flags_template_restricted_face` (injected restriction table) + `x016_silent_when_production_table_empty`
- [x] `xref::tests::x017_flags_unknown_face_id_as_warning` + `x017_silent_when_all_face_ids_known`
- [x] `spec_amendments::spec_section_3_6_mentions_unknown_variant` + `spec_section_3_6_pips_is_i16`
- [x] Build-script determinism — MD5 identical across rebuilds
- [x] All 4 working mods IR-equal roundtrip (per `tests/baselines/roundtrip/*.baseline` — unchanged shape; PIPELINE_FIDELITY_PLAN owns the known-red full-cycle fix)
- [x] `cargo rustdoc --lib` clean for `authoring` module with `deny(missing_docs)`

**Notes for follow-on chunks**:
- `KNOWN_FACE_IDS: &[u16]` is already sorted (build.rs emits via `BTreeMap` ascending); `FaceIdValue::try_new` uses `binary_search`. Chunk 3a's sprite registry can follow the same OUT_DIR pattern.
- `X016_TEMPLATE_RESTRICTIONS` stays empty until the guide explicitly documents a restriction. Chunks that add guide-derived template rules should edit this table — no other gate.
- `compiler/build.rs` uses `CARGO_MANIFEST_DIR.parent()` to locate `working-mods/`; Chunk 3a will extend the same `main()` with sprite harvesting alongside face-id harvesting (single pass, same `fs::read_to_string` calls).

### Chunk 2 (original spec below) — `FaceId` + `Pips` newtypes + whitelist + IR flip + SPEC §3.6 amendment
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

#### Chunk 3a: SpriteId newtype + registry + authoring surface — ✅ COMPLETE (2026-04-21)

**Landed**: `phf`/`phf_codegen` deps; `compiler/src/authoring/sprite.rs` (`SpriteId` with `Cow<'static, str>` fields, `lookup`/`owned`/`try_registered`/`name`/`img_data`; flat `{name, img_data}` serde via `SpriteIdSerde`); `compiler/build.rs` extended with `harvest_sprites` (single-pass read shared with face-id harvest, depth-aware `.img.`↔`.n.`/`.mn.` pairing with fewer-outer-hops → nearer-distance → `.mn.`-beats-`.n.` scoring); generator emits `OUT_DIR/sprite_registry_generated.rs` (1,395 entries, phf-backed, byte-deterministic across rebuilds); `authoring/mod.rs` + `lib.rs` re-export `SpriteId`. IR unchanged — field consolidation deferred to 3b per plan.

**Deviations from spec**:
- Generated registry lives at `$OUT_DIR/sprite_registry_generated.rs` (included from `sprite.rs`) rather than `compiler/src/authoring/sprite_registry.rs` — matches the Chunk 2 `face_id_generated.rs` pattern and keeps the generated static inside the private-field scope of `SpriteId`.
- `sprite_lookup_charmander` verification test renamed to `sprite_lookup_agumon` — the Charmander→Agumon swap is a planned Sliceymon+ authoring change, and Agumon is already present in `working-mods/sliceymon.txt`, so the test pins to the final name directly (no proxy). The byte-for-byte property is preserved: the test `include_str!`s `working-mods/sliceymon.txt`, extracts Agumon's `.img.` payload at test time, and `assert_eq!`s it against the registry value.

#### Chunk 3a (original spec below) — SpriteId newtype + registry + authoring surface
**Files**: `compiler/Cargo.toml`, `compiler/src/authoring/sprite.rs`, `compiler/src/authoring/sprite_registry.rs`, `compiler/src/authoring/mod.rs`, `compiler/build.rs`, `compiler/src/lib.rs` (re-export only, no signature change).
**Dependencies**: Chunk 2.

**Requirements**:
- Add `phf` runtime + `phf_codegen` build-deps.
- Author `SpriteId { name: Cow<'static, str>, img_data: Cow<'static, str> }` per §F4 — with `lookup`, `owned`, `try_registered`, `name()`, `img_data()` accessors.
- Extend `build.rs` to emit `sprite_registry.rs` as a `phf::Map<&'static str, SpriteId>` literal. Mod-priority: first-write-wins over forward iteration of `WORKING_MOD_ORDER = [sliceymon, pansaer, punpuns, community]`, so sliceymon sprites take precedence on name collisions.
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
- Every extractor uses `SpriteId::owned(name, img_data)` unconditionally — source bytes are preserved verbatim. **No `SpriteId::lookup` in the extract path**: the registry is first-write-wins across `sliceymon > pansaer > punpuns > community` (see `compiler/build.rs::WORKING_MOD_ORDER`), so a registry-first lookup during extract silently replaces the source's `.img.` payload with sliceymon's on any name collision (Pikachu, Amnesia, Apple, Bubble, Curry, …). That is source corruption, and it violates SPEC §3.3 (any valid textmod extracts with a self-contained IR). Registry lookup (`SpriteId::lookup` / `SpriteId::try_registered`) belongs on the **authoring path** only, where a miss is a real mistake — matches §F4 line 236 and SPEC §6.1's Path A/Path B split.
- Every emitter reads `sprite.img_data()` (and `sprite.name()` for the display field).
- xref rules that referenced sprite_name/img_data update to `sprite.name()` / `sprite.img_data()`.
- ir/ops.rs duplicate-name checks unchanged (keyed by `name`, not `sprite`).
- `TriggerHpDef::parse`: when source has no `.n.`, store `SpriteId.name == ""`. Do **not** fall back to the template name — that would store a semantic lie (template names are not sprite display names). The emitter only reads `sprite.img_data()`, so the absent name is observably absent.

**Verification — specific tests**:
- [x] `ir::heroblock_sprite_required` — compile-time: `HeroBlock { sprite: SpriteId::owned("x", "y"), ... }` compiles; missing `sprite` is a compile error.
- [x] `ir::serde_breaking_change_on_sprite_shape` — decision made (user ruling 2026-04-20: "no legacy, always choose correctness over back-compat"): **no serde compat shim**. JSON that uses the old `sprite_name` + `img_data` keys fails to deserialize. Test asserts the new flat `sprite` shape is the only accepted JSON form; no dual-representation gymnastics. Plans/PIPELINE_FIDELITY and AUTHORING_ERGONOMICS author IR in code, not from historical JSON, so there are no legacy consumers to break.
- [x] `integration_tests::extract_preserves_hero_img_data_on_registry_name_collision` — parses a hero block whose name is in the registry (`Pikachu`) with a novel `.img.` payload (`TRIBUNAL_NOVEL_IMG`); asserts `block.sprite.img_data() == "TRIBUNAL_NOVEL_IMG"`. This is the load-bearing test: it fails if any future refactor re-introduces `SpriteId::lookup` into the extract path.
- [x] `integration_tests::extract_preserves_monster_img_data_on_registry_name_collision` — analogous pin for `Monster` via the real `1-3.monsterpool.(rmon…)…img.X.n.Name` shape.
- ~~`integration_tests::extract_preserves_replica_item_img_data_on_registry_name_collision`~~ — **deleted in Chunk 9 (2026-04-23)**. The test's own comment acknowledged it was probing `parse_simple` directly "until a future caller routes here"; that caller never came. `parse_simple` and the classifier gap were deleted per chunk-impl rule 3 (no corpus instance). Chunk 3b's sprite-registry defence for `ReplicaItem` now lives solely at the `emit_legendary` + `parse_legendary` round-trip (`legendary_emit_parse_roundtrip_with_all_fields` pins that `.img.` round-trips through a realistic Legendary shape).
- [x] All 4 working mods IR-equal roundtrip (hero/item/monster IR equality preserved; `bosses.equal: false` pinned by `roundtrip_baseline` as pre-Chunk 3b known-red, PIPELINE_FIDELITY_PLAN owns the fix).

**Deviation note (2026-04-21)**: The original chunk requirement read *"Every extractor uses `SpriteId::lookup(name).cloned().unwrap_or_else(|| SpriteId::owned(name, img_data))` — registry miss falls back to owned, not error. SPEC §3.3."* That was **wrong** and contradicted §F4 line 236 (which correctly stated `SpriteId::owned` only). PR #5's first round shipped the lookup-first incantation in 8 extractor callsites; round-1 tribunal reproduced silent data loss (source `.img.WRONGDATA` + registered name `Pikachu` yielded sliceymon's Pikachu payload in the IR). Round-2 fix dropped all 8 lookups, added the 3 regression pins above, tightened the `authoring/sprite.rs` module doc to name SPEC §3.3 as the invariant forbidding registry use in extract, and rewrote the PR description. Future chunks authoring sprite-bearing parsers must copy the corrected wording above, not the original §F4 table shorthand.

#### Chunk 3c: Drop `sprites: &HashMap` from public API
**Files**: `compiler/src/lib.rs`, `compiler/src/main.rs` (if CLI passes sprite maps), `compiler/src/builder/mod.rs`, `compiler/src/builder/hero_emitter.rs` (already takes `&HashMap`; drop it), every callsite in `compiler/tests/` and `compiler/examples/`.
**Dependencies**: Chunk 3b.

**Requirements**:
- Drop `sprites: &HashMap<String, String>` from: `build`, `build_complete`, `build_hero`, `validate_hero`, `validate_hero_in_context` (5 public fns; verified list).
- Update `hero_emitter::emit(hero, sprites)` → `hero_emitter::emit(hero)`.
- Update `main.rs` CLI and every test/example that passes a sprite map.

**Verification — specific tests**:
- [x] `build_no_sprites_path_b` — Path B ModIR with an in-memory hero builds via `build_complete(&ir)`; ASCII textmod carries the sprite's `img_data` from `HeroBlock.sprite`.
- [x] `build_roundtrip_sliceymon_no_sprites` — `extract → build(&ir) → extract` on `working-mods/sliceymon.txt` yields IR-equal `heroes`, `replica_items`, and `monsters` without passing a sprite map.
- [x] `build_hero_signature` — `build_hero(&hero)` compiles with a single argument; `lib.rs` no longer imports `HashMap`.
- [x] All 4 working mods IR-equal roundtrip (`tests/roundtrip_baseline.rs` passes unchanged after the signature flip).

---

### Chunk 4: `BuildOptions` + `build_with` + `Finding.source` — ✅ COMPLETE (2026-04-22)
**Spec**: §F5

**Landed**: `compiler/src/builder/options.rs` defines `BuildOptions`, `SourceFilter`, `SourceSet`; re-exported via `builder::{BuildOptions, SourceFilter, SourceSet}` and `lib::{BuildOptions, SourceFilter, SourceSet}`. `pub fn build_with(ir, opts)` replaces the inlined emitter; `pub fn build(ir)` is a thin wrapper over `build_with(ir, &BuildOptions::default())`. Every content-emission site (heroes, replica_items, monsters, bosses, and all 19 structural-type filters via an `emit_structurals` closure) consults `opts.include.admits(entity.source)` before pushing.

`xref::Finding` gained `pub source: Option<Source>` (`#[serde(default, skip_serializing_if = "Option::is_none")]`). `xref::promote_severity(base: Severity, src: Option<Source>) -> Severity` concentrates the promotion policy in one place: `Some(Base) → Warning`, `Some(Custom|Overlay) → Error`, `None → base`. Every existing Finding-construction site applies both the source field and the helper.

**Construction-site audit** (ran at chunk start, per §F5):
- `compiler/src/xref.rs` (12 sites): X016 at `check_face_template_compat_with_table`, X017 at `check_face_unknown`, V019 in `check_hero_color_uniqueness`, V020 in `check_cross_category_names` (global — `source: None` because there is no single offending entity; severity stays Error), V016 in `check_hero_pool_refs` (source from `StructuralModifier.source`), V019 + three V020 in `check_hero_in_context`, three V020 in `check_boss_in_context`. `iter_dice_faces` extended to yield `(path, &DiceFaces, template, Source)` so face-level rules inherit provenance from the owning hero/replica_item/monster/boss. `FightUnit` has no provenance of its own → inherits from the enclosing `Boss`.
- `compiler/src/lib.rs` (1 site): `E000` in `validate_hero` — populated from `hero.source`; kept at literal `Severity::Error` (build failure is not a semantic rule subject to source promotion).

**Side effects**: `ir::Source` now derives `Copy, Eq, Hash` so filtering and bitmask lookups are cheap; existing Clone-using code is unaffected. Test helpers in `xref::tests` (`make_hero`, `make_replica_item`, `make_boss`, `make_hero_pool`) default to `Source::Base` so pre-existing rule-fire tests continue to pass their structural assertions while §F5's policy routes findings to the Warning lane; tests that want to exercise the Custom/Overlay Error path set `source` explicitly.

**Source-vs-IR divergence pin** (per Lessons takeaway #2): the severity-promotion policy *changes* what lane a finding lands in based on the offending entity's `.source`. An IR-vs-IR roundtrip test can't catch a bug where `promote_severity` silently collapses to a constant — `promote_severity_helper_table` enumerates the full 3×4 cross product so a regression trips one of twelve assertions, and `v016_finding_source_populated_{base,custom}` verifies the same rule lands in different lanes for different sources.

**Tests landed** (`compiler/tests/build_options_tests.rs`, 10 tests + new unit tests in `xref::tests`):
- [x] `build_with_only_base_omits_overlay`
- [x] `build_with_exclude_base`
- [x] `build_default_equivalent_to_build_with_default_opts` (pins `build(ir) == build_with(ir, &BuildOptions::default())`)
- [x] `source_filter_admits_const` (const-fn pin)
- [x] `promote_severity_helper_table` — full 3×4 cross product
- [x] `v016_finding_source_populated_base` — Base source → Warning, source = Some(Base)
- [x] `v016_finding_source_populated_custom` — Custom source → Error, source = Some(Custom)
- [x] `v019_finding_source_populated`
- [x] `v020_cross_category_source_is_global` (cross-category V020 stays `source: None`)
- [x] `finding_json_omits_absent_source` (serde back-compat — legacy JSON without `source` still deserializes)
- [x] All 4 working mods IR-equal roundtrip (`cargo run --example roundtrip_diag` — sliceymon / pansaer / punpuns / community all `ROUNDTRIP OK`).

---

### Chunk 5: Merge signature → `&mut` + strips derived structurals + two new derived regenerators + unconditional regeneration
**Spec**: §F6
**Files**: `compiler/src/ir/mod.rs` (add `StructuralModifier::is_derived`, add `ModIR.warnings` sidecar), `compiler/src/ir/merge.rs` (signature change + strip logic), `compiler/src/builder/mod.rs` (unconditional regeneration), `compiler/src/builder/derived.rs` (two new generators), `compiler/src/lib.rs` (merge re-export signature), `compiler/src/main.rs` (CLI merge subcommand), `compiler/tests/integration_tests.rs` or new `compiler/tests/path_c_merge_tests.rs`.
**Dependencies**: Chunk 6 (historical). Post-Chunk-9, Chunk 6's `ReplicaItemContainer::Capture { name }` variant has been deleted per chunk-impl rule 3. `generate_hero_item_pool` now buckets by parsing the `.n.NAME` out of `ItemPool` structural modifiers — the shape the classifier produces for Captures in production. No `ReplicaItem.container` field to match on.

**Requirements**:
- Add `pub warnings: Vec<Finding>` to `ModIR` with `#[serde(default, skip_serializing_if = "Vec::is_empty")]`.
- Add `pub fn is_derived(&self) -> bool` on `StructuralModifier` (in `ir/mod.rs`) matching on the four derived kinds in §F6.
- Change `merge` signature to SPEC §5's canonical form: `pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError>`. Update `lib.rs` re-export, `main.rs` CLI usage, and every test that calls `merge`. No tuple return; no parallel `merge_with_findings` function.
- `merge` strips derived structurals from both inputs before merging; each strip pushes a `Finding` onto `base.warnings` with `rule_id: "X010"`, `severity: Severity::Warning`, `field_path: Some(...)`, `suggestion: Some("Derived structurals are regenerated at build time; authoring them directly is unsupported.")`.
- Author `generate_pool_replacement(heroes)` and `generate_hero_item_pool(heroes, structural)` in `builder/derived.rs`. Byte-for-byte reproduce sliceymon's existing PoolReplacement / hero-bound ItemPool modifiers against the extracted base IR. `generate_hero_item_pool` buckets by parsing the `.n.NAME` out of each `StructuralType::ItemPool` modifier (the shape Captures route as post-Chunk-9) and matching `NAME` to a hero's `internal_name`. Legendary `ReplicaItem`s have their own emission path and are skipped for hero-bound pools.
- Remove the "when absent" gate in `build_complete` at `compiler/src/builder/mod.rs:161-168`. `build` itself also strips derived structurals from `ir.structural` before emitting and appends the regenerated forms — build-time regeneration is unconditional.
- Add a Path C integration test that does NOT rely on direct struct-literal construction (authoring layer is empty — use `ir_from_json` or roundtrip-extracted IR, then `Hero::new` from Chunk 1 to add the new hero). The test must not violate SPEC §6.1.

**Verification — specific tests**:
- [ ] `ir::is_derived_truth_table` — every `StructuralType` variant tested; only the four derived kinds return `true`.
- [ ] `merge::strips_derived_char_selection_with_warning` — base has CharacterSelection, overlay has CharacterSelection; after `merge(&mut base, overlay)?`, `base.structural` contains zero CharacterSelection entries and `base.warnings` contains two `X010` findings.
- [ ] `merge::new_signature_compiles` — `merge(&mut base, overlay)?;` compiles; `base.warnings` is readable post-merge.
- [ ] `merge::warnings_accumulate_across_calls` — a second `merge` call appends to (does not reset) `base.warnings`.
- [ ] `derived::pool_replacement_matches_sliceymon` — `generate_pool_replacement(sliceymon.heroes)` byte-matches `working-mods/sliceymon.txt`'s existing PoolReplacement modifier.
- [ ] `derived::hero_item_pool_matches_sliceymon_via_item_pool_n_name` — `generate_hero_item_pool` buckets by parsing the `.n.NAME` out of `ItemPool` structural modifiers (Captures route there; no `ReplicaItem` Capture variant exists post-Chunk-9 per chunk-impl rule 3). Byte-matches hero-bound ItemPool in sliceymon.
- [ ] `path_c_merge::adds_hero_regenerates_selector` — load sliceymon IR from JSON, `Hero::new` a new hero, append to `ir.heroes`, `build_complete`, re-extract — new hero is in the regenerated CharacterSelection.
- [ ] All 4 working mods IR-equal roundtrip after `build` strips + regenerates derived structurals.

---

### Chunk 6: `ReplicaItemContainer` enum replaces `container_name` [serial after Chunk 4, before Chunk 5] — ✅ COMPLETE (2026-04-21, branch `feat/chunk-6-replica-item-container`) — ⚠️  SUPERSEDED BY CHUNK 9 (2026-04-23)
**Spec**: §F7 (superseded — see §F7 header for current state)
**Files**: see Chunk 9's deletion commit `77b802b` for the current state of each. Historical: `compiler/src/ir/mod.rs`, `compiler/src/extractor/replica_item_parser.rs`, `compiler/src/extractor/classifier.rs`, `compiler/src/builder/replica_item_emitter.rs`, `compiler/src/xref.rs`, `compiler/src/ir/ops.rs`, `compiler/src/ir/merge.rs`.

**Chunk-9 supersession note (2026-04-23)**: Chunk 6 shipped the `ReplicaItemContainer { Capture { name }, Legendary }` enum + `container` field + X003 rule. Chunk 9 deleted the enum and the `Capture` variant per chunk-impl rule 3 — no corpus instance was ever created for a Capture `ReplicaItem` (classifier never returned `ModifierType::ReplicaItem*`; Captures route as `ItemPool` structurals). X003 survives: its `"capture"` bucket label was removed, leaving `"hero"` / `"legendary"` / `"monster"` / `"boss"` as the active buckets (`compiler/src/xref.rs:228-237`). The rest of Chunk 6's requirements / verification checklist below are archived; see Chunk 9 for the currently-shipped shape.

**Surviving artefacts from Chunk 6** (still in code):
- `ReplicaItem` struct exists (now Legendary-only; no `container` field).
- X003 xref rule exists and fires on hero + legendary name collisions. Verified by `xref::x003_duplicate_pokemon_across_kinds` + `xref::x003_finding_is_global_source_none`.
- `replica_items` in `ModIR` still holds Legendaries.

**Archived Chunk 6 requirements** (no longer live — kept for traceability):
- ~~Define `ReplicaItemContainer { Capture { name: String }, Legendary }`.~~ Deleted in Chunk 9.
- ~~Replace `ReplicaItem.container_name: String` with `pub container: ReplicaItemContainer`.~~ Field removed in Chunk 9.
- ~~Extractor classifier builds the variant from source-level modifier shape.~~ Classifier no longer returns `ModifierType::ReplicaItem*`; captures always routed as `ItemPool`.
- ~~`build_replica_item` / emitter matches `container` non-exhaustively.~~ `emit()` now dispatches straight to `emit_legendary` (single shape).
- X003 author work landed and survives — bucketing simplified to `"legendary"` label for all replica items.
- ~~Serde tagging for `ReplicaItemContainer`.~~ Type deleted.

**Archived verification tests** (deleted in Chunk 9 commit `77b802b` — exercised a type that no longer exists):
- ~~`ir::replica_item_capture_carries_container_name`~~ — deleted.
- ~~`ir::replica_item_legendary_has_no_container_by_construction`~~ — deleted.
- ~~`extractor::classifies_capture_vs_legendary_into_enum`~~ — deleted.
- [x] `xref::x003_duplicate_pokemon_across_kinds` — **still passes**; asserts `"legendary"` bucket.
- ~~`builder::replica_item_emitter_non_exhaustive_match`~~ — vacuous post-Chunk-9.
- ~~`serde::replica_item_container_json_shape`~~ — deleted.
- [x] All 4 working mods IR-equal roundtrip — **still passes**.

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

### Chunk 8: V020 restructure — remove cross-bucket Pokemon overlap with X003 [serial after Chunks 4 and 6; parallel with 5 and 7]
**Spec**: §F9
**Files**: `compiler/src/xref.rs` only (production code + in-file `#[cfg(test)]` tests).
**Dependencies**:
- Chunk 4 (V020's `Finding.source` retrofit must land so Chunk 8's test assertions compile against `finding.source`).
- Chunk 6 (X003 must exist so V020 can narrow to the slice X003 does not own).
**Parallel with**: Chunks 5 and 7 (no shared files).
**Merge ordering**: merges after 4 and 6. No constraint against 5 or 7.

**Context** — re-state of actual V-rule meanings in code (line refs are plan-write snapshots — grep the function name on the current branch before editing):
- V016 = hero pool references resolve (`check_hero_pool_refs`).
- V019 = hero color uniqueness (`check_hero_color_uniqueness`).
- V020 = cross-category name uniqueness (`check_cross_category_names`). This chunk narrows V020's emission; its 4-bucket collection (hero / replica / monster / boss) stays intact.
- X003 = SPEC §6.3 Pokemon uniqueness across `{hero, legendary, monster}` (`check_duplicate_pokemon_buckets`). Does not include bosses. Post-Chunk-9 there is no `capture` bucket — the Capture variant was deleted per chunk-impl rule 3; Captures route as `ItemPool` structurals and are not reached by X003.

**Requirements**:
- Update `check_cross_category_names` (grep for the fn — line refs in this plan are snapshots). After bucket collection, compute the distinct bucket-label set for the colliding entries. Skip emission iff that set is a subset of `{hero, replica_item, monster}` with cardinality ≥2 — X003 owns that slice. Otherwise emit V020 as today. No other V020 behavior changes.
- Do **not** introduce a new rule ID. V020's scope remains "cross-category name uniqueness"; F9 narrows its emission predicate, not its semantics.
- Do **not** modify the single-item CRUD checks (`check_hero_in_context`, `check_boss_in_context`). They validate one new item and don't produce the whole-IR double-fire.
- Update the two existing V020 tests that currently assert cross-bucket `hero↔replica` firings (`test_v020_cross_category_duplicate` and `test_v020_case_insensitive` — grep for the test names on current HEAD; the pre-F9 line refs are stale). Both already filter by `rule_id == "V020"` because X003 fires alongside after Chunk 6. Post-F9, V020 must NOT fire on those inputs; X003 is the sole owner. Rewrite the assertions accordingly.
- The `modifier_name` field on each retained V020 finding must populate from the offending entity (not `None`), for parity with X003 and other V-rules post-Chunk 4.

**Verification — specific tests**:
- [ ] `xref::v020_silent_on_cross_bucket_pokemon` — ModIR with hero "Pikachu" + legendary "Pikachu" → `report.errors.iter().filter(|f| f.rule_id == "V020").count() == 0` AND `report.errors.iter().filter(|f| f.rule_id == "X003").count() == 1`. Same for `hero+monster` and `legendary+monster` pairs. (Captures have no typed variant post-Chunk-9; authors collide via the legendary bucket.)
- [ ] `xref::v020_still_fires_on_boss_hero_collision` — hero "Pikachu" + boss "Pikachu" → V020 fires exactly once; X003 is silent (SPEC §6.3 excludes bosses). Source-vs-IR proof: invent a boss name that cannot appear in any registry lookup (e.g. "Zzzboss") to rule out a regression that routed the boss through a Pokemon-bucket.
- [ ] `xref::v020_still_fires_on_boss_replica_collision` — legendary replica + boss with same name → V020 fires; X003 does not. (The pre-Chunk-9 \"Capture + Legendary, separate sub-cases\" form is archival — Captures no longer have a typed `ReplicaItem` variant.)
- [ ] `xref::v020_still_fires_on_intra_bucket_duplicate_heroes` — two heroes with same name → V020 fires exactly once. (X003 is already tightened to require ≥2 distinct buckets in Chunk 6 and does not fire here.)
- [ ] `xref::v020_still_fires_on_intra_bucket_duplicate_replicas` — two legendary replicas with same name → V020 fires; X003 silent. (`ReplicaItem` is Legendary-only post-Chunk-9.)
- [ ] `xref::v020_still_fires_on_intra_bucket_duplicate_monsters` — two monsters with same name → V020 fires; X003 silent.
- [ ] `xref::v020_still_fires_on_intra_bucket_duplicate_bosses` — two bosses with same name → V020 fires; X003 silent.
- [ ] `xref::v020_and_x003_coexist_when_collision_spans_boss_and_pokemon_buckets` — hero + legendary + monster + boss all named "Pikachu" → V020 fires (boss involvement) AND X003 fires (≥2 distinct Pokemon buckets: hero + legendary + monster). This case deliberately keeps both findings because each describes a different invariant: V020 reports the boss-involving name collision; X003 reports the SPEC §6.3 Pokemon collision. Document this in the test body so a future "dedup everything" refactor doesn't silently collapse them.
- [ ] `xref::no_double_fire_on_working_mods` — an integration-style assertion over `check_references(&ir)` for each of the 4 working mods: for every pair `(f1, f2)` of errors where `f1.modifier_name == f2.modifier_name`, the rule IDs must differ by *invariant class*, not by redundant coverage — today, the only permitted co-fire pattern is the one in the test above.
- [ ] All 4 working mods IR-equal roundtrip. `check_references` produces the same set of errors as pre-chunk with zero added V020 findings and the same (or strictly fewer) total findings.

**Structural check (per the chunk-impl hook)**: F9 does **not** collapse two paths with different invariants. Before F9, V020 and X003 produce *identical-scope* findings on the `{hero, replica_item, monster}` cross-bucket slice — that is the parallel representation SPEC §3.7 forbids, and this chunk removes it. V020's retained scope (boss cross-category + intra-bucket) is strictly disjoint from X003's (Pokemon cross-bucket), so both rules remain single-purpose. No spec amendment is needed — SPEC §3.7 is already the authority; F9 brings the implementation back under it.

---

### Chunk 9: Legendary-parser chain-and-depth-aware scalar extraction + Capture-variant deletion [serial after Chunk 6; parallel with 5, 7, 8] — ✅ COMPLETE (2026-04-23)
**Spec**: §F10 (grounds in SPEC §3.3 self-contained IR).
**Files (as shipped)**: `compiler/src/util.rs` (add `slice_before_chain_and_cast` + `depth_aware` flag on `extract_color`), `compiler/src/extractor/replica_item_parser.rs` (rewritten as Legendary-only — deleted `parse_simple` / `parse_with_ability`), `compiler/src/builder/replica_item_emitter.rs` (rewritten as Legendary-only — deleted `emit_simple` / `emit_with_ability`; `.sd.` / `.img.` emitted before the chain per §F10 emission-order; ability emitted as `.abilitydata.(body)` per textmod guide §F10-MARKERS), `compiler/src/extractor/classifier.rs` + `compiler/src/extractor/mod.rs` (deleted `ModifierType::ReplicaItem` / `ReplicaItemWithAbility` variants + their arms — never produced by the classifier), `compiler/src/ir/mod.rs` (deleted `ReplicaItemContainer` enum + `container` field on `ReplicaItem`; deleted `chunk_6_tests` module), `compiler/src/ir/ops.rs` + `compiler/src/xref.rs` + `compiler/tests/build_options_tests.rs` (dropped `container` from every `ReplicaItem` struct literal), `compiler/tests/integration_tests.rs` (dropped self-documented dead `parse_simple` test). Signature-compatible touches in `compiler/src/extractor/{hero_parser,monster_parser}.rs` and `compiler/src/ir/mod.rs` pass `depth_aware = false` at the five non-Legendary `extract_color` callsites to preserve current behavior.
**Dependencies**: Chunk 6 (the `before_cast` scoping in `parse_legendary` is the pattern this chunk generalizes — and the enum Chunk 9 deletes post-hoc per chunk-impl rule 3).
**Parallel with**: Chunks 5, 7, 8 (no shared files).

**Context — leak class as of plan-write (historical)**:
- `util::extract_hp(modifier, false)` / `extract_sd(modifier, false)` called `content.find(marker)` — no depth tracking, no chain awareness.
- `util::extract_color(content)` had no `depth_aware` flag at all.
- `util::extract_img_data(content)` used `find_last_at_depth0` (depth-aware already) but matched the *last* depth-0 hit — a chain `sidesc` carrying `.img.X` at depth 0 could still be the last hit if the Legendary had no top-level `.img.`.
- `chain_emitter.rs:59-63` shows chain sub-entries emit `sidesc.{text}` / `enchant.{modifier}` with free-form text that can legitimately contain any `.{marker}.` substring.
- `parse_legendary`'s Chunk-6 `before_cast` slicing (now `before_ability`) only closed the cast-interior subset of the class. Chain-interior leakage at depth 0 remained.

**Requirements (as shipped)**:
- Add `pub fn slice_before_chain_and_cast(body: &str) -> &str` to `util.rs`. Semantics: scan for the first depth-0 occurrence of any marker in **§F10-MARKERS** (defined in §F10 above) and return `&body[..pos]`; when no such marker exists at depth 0, return the full `body` slice. Depth tracking follows the existing `find_at_depth0` idiom — paren balance.
- Extend `extract_color` signature to `pub fn extract_color(content: &str, depth_aware: bool) -> Option<char>`. When `depth_aware = true`, skip matches at paren depth > 0. `depth_aware = false` preserves the current behavior so non-Legendary callsites are unchanged.
- Wire the helper into `parse_legendary`: `let scalar_slice = util::slice_before_chain_and_cast(body);` (where `body` is the post-`item.`-strip content), then scalar extractors with `depth_aware = true` on `scalar_slice`. `extract_img_data(scalar_slice)` is always used — it is inherently depth-aware via `find_last_at_depth0`.
- The `before_ability` local in `parse_legendary` (renamed from `before_cast` in this chunk) is retained for the character-name lookup + chain scoping (emission places `.n.NAME` after the chain and before `.abilitydata.`; the name extractor must see that exact region). Scalar extraction moves to the broader `scalar_slice`; chain extraction scopes to `before_ability` so a stray `.i.` / `.sticker.` left inside a `.speech.` / `.doc.` value can't be misread as a chain segment.
- Chain and ability extraction themselves do **not** route through the new helper — they need to see the chain / ability regions of the body. `extract_modifier_chain` still scans for `.i.` / `.sticker.` at depth 0, `extract_nested_prop(body, ".abilitydata.")` still scans for the ability at depth 0.
- Reorder `.sd.` / `.img.` to precede the chain in `emit_legendary` (emission-order co-requirement from §F10). Without this, an emit→parse cycle where the chain carries a decoy `.sd.` and the real value comes after would silently lose the real value to first-match.
- **Delete** `ModifierType::ReplicaItem` / `ReplicaItemWithAbility`, `parse_simple` / `parse_with_ability`, `emit_simple` / `emit_with_ability`, `ReplicaItemContainer` enum + `container` field on `ReplicaItem`, `util::replica_inner_body`, `util::extract_template` per chunk-impl rule 3 — none had a corpus instance. Captures route as `ItemPool` structurals in production and always did; classifier never returned the deleted `ModifierType` variants.
- Rename the ability-body emit/read marker from `.cast.(body)` to `.abilitydata.(body)` per the textmod guide (`reference/textmod_guide.md` lines 747 / 857 / 975-981). `.cast.` in the corpus is a chain keyword, not a property marker. Affects `emit_legendary` + `parse_legendary` + §F10-MARKERS canonical set.
- Audit every other callsite of `extract_color` outside `parse_legendary` and confirm whether `depth_aware = true` is correct there too. Preserve the caller's choice — this chunk does not change non-Legendary semantics. Audit result: all six live callsites (`hero_parser.rs` ×3, `monster_parser.rs` ×1, `ir/mod.rs` ×1, `extract_color` lives inside `extract_facades_from_chain` callers implicitly) keep the non-depth-aware behavior by passing `depth_aware = false`.

**Verification — specific tests (as shipped)**:

Each test is a *source-vs-IR divergence* test by construction (per §F10 / Chunk 3b lesson item 2): the input's top-level scalar is absent, and the chain / ability contains a substring whose byte-for-byte interpretation would flip the parsed field if extraction reached for chain-interior bytes.

- [x] `extractor::legendary_hp_ignores_chain_interior_sidesc`
- [x] `extractor::legendary_color_ignores_chain_interior_sidesc`
- [x] `extractor::legendary_sd_ignores_chain_interior_sidesc` — strong form (no top-level `.sd.`; decoy inside `.sticker.sidesc`).
- [x] `extractor::legendary_img_ignores_chain_interior_sidesc`
- [x] `extractor::legendary_ignores_abilitydata_interior_hp_color_sd_img` — ability-body decoys for all four scalars.
- [x] `util::slice_before_chain_and_cast_no_markers_returns_full_body`
- [x] `util::slice_before_chain_and_cast_skips_nested_markers` — input `Alpha.abilitydata.(a.i.b)`.
- [x] `util::slice_before_chain_and_cast_returns_earliest_of_three_markers`
- [x] `util::extract_color_depth_aware_skips_parens`
- [x] `builder::legendary_emit_parse_roundtrip_with_all_fields` — every scalar field survives emit→parse (ability included as `.abilitydata.(body)`).
- [x] `builder::legendary_emit_parse_roundtrip_with_item_modifiers` — chain-emission order pinned (chain after `.sd./.img.`, before `.n.` / `.abilitydata.`).
- [x] `xref::x003_duplicate_pokemon_across_kinds` — X003 still fires on hero + legendary name collision; `"capture"` bucket removed post-deletion.
- [x] All 4 working mods IR-equal roundtrip (`cargo run --example roundtrip_diag` reports `ROUNDTRIP OK` for each). The 4 working mods contain zero replica items; this chunk must not add `Replicas ir1>0` findings spuriously — verified.
- [x] `cargo test` passes (324 tests; 21 tests deleted with their target code per chunk-impl rule 3).

**Deleted tests** (chunk-impl rule 3 — no live callers; no corpus instance):
- ~~`extractor::classifies_capture_into_enum_with_container_name_from_source`~~
- ~~`extractor::classifies_capture_with_ability_into_enum`~~ (rewritten three times across rounds 4, 7, 8; ultimately deleted with `parse_with_ability`)
- ~~Four Round-4 `capture_*_emit_parse_*` leak-probe tests~~ (deleted with the Capture parsers)
- ~~Three Round-8 `capture_*_emit_parse_*_roundtrip` tests~~ (deleted with the Capture parsers)
- ~~`util::replica_inner_body_*` (four tests)~~ (helper deleted)
- ~~`ir::chunk_6_tests` (three tests)~~ (enum deleted)
- ~~`xref::x003_distinguishes_capture_from_legendary_buckets`~~ (the distinction no longer exists at the type level)
- ~~`integration_tests::extract_preserves_replica_item_img_data_on_registry_name_collision`~~ (self-documented dead — awaiting a "future caller" that never came)

**Structural check (per the chunk-impl hook)**: Chunk 9 does **not** collapse two paths. Pre-chunk-9 there were three replica parsers (parse_simple / parse_with_ability / parse_legendary); post-chunk-9 there is one (parse_legendary), because the other two targeted types the classifier never produced. This is deletion per chunk-impl rule 3, not path-collapsing. The `depth_aware` flag on `extract_color` mirrors the flag that already exists on `extract_hp` / `extract_sd` — symmetry being restored, not a new abstraction. The single-helper Legendary scoping (`slice_before_chain_and_cast` applied to `body`) is the "one correct line to write" case — not N callsites pasting an incantation.

---

## Final Verification

- [ ] `cargo test --all` passes with no regressions.
- [ ] IR-equal roundtrip holds for all 4 working mods (`cargo run --example roundtrip_diag` is empty).
- [ ] `cargo doc` with `#![deny(missing_docs)]` on the new `authoring` module renders cleanly.
- [ ] `cargo run -- schema` produces a JSON Schema that includes `FaceId`, `FaceIdValue`, `Pips`, `SpriteId` types. (`ReplicaItemContainer` was deleted in Chunk 9 per chunk-impl rule 3 and is no longer in the schema.)
- [ ] `compiler/src/authoring/` contains only: `mod.rs`, `face_id.rs`, `face_id_generated.rs`, `sprite.rs`, `sprite_registry.rs`. No builders, no macros, no `HeroReplica` — those are owned by `AUTHORING_ERGONOMICS_PLAN.md`.
- [ ] No `std::fs` / `std::process` in `compiler/src/authoring/` or any other library file.
- [ ] Lib-code audit (`rg` with `#[cfg(test)]` stripping, enforced via `xtask`/`build.rs`) shows zero `unwrap()` / `expect()` / `panic!` / `unimplemented!` / `todo!` hits.
- [ ] Every new xref rule (X003, X010, X016, X017) populates `field_path`, `suggestion`, and `source` on its `Finding`s. Every existing V-rule (V016, V019, V020) populates `source`.
- [ ] V020 and X003 do not double-fire on cross-bucket Pokemon collisions (`{hero, replica_item, monster}` slice is X003's sole territory). V020 retains emission only for boss-involving collisions and intra-bucket duplicates. (Chunk 8, §F9.)
- [x] `parse_legendary` closes the §F10 leak class per the single-helper design in §F10 "Fix":
  - `parse_legendary` applies `util::slice_before_chain_and_cast` directly (its `item.TEMPLATE…` body is flat at depth 0), then scalar extractors with `depth_aware = true`.
  - `emit_legendary` places `.sd.` / `.img.` before the chain, per §F10's emission-order co-requirement; the ability body is emitted as `.abilitydata.(body)` per the textmod guide.
  - Chain-interior `.hp.` / `.col.` / `.sd.` / `.img.` substrings in sidesc / enchant text, and ability-interior equivalents in `.abilitydata.(body)`, do not leak into top-level fields; verified via five `legendary_*_ignores_*` source-vs-IR tests (see §F10 / Chunk 9 verification list). (Chunk 9, §F10, 2026-04-23.)
  - Capture-shape parsers (`parse_simple` / `parse_with_ability`) and the `ReplicaItemContainer` enum were deleted in the same chunk per chunk-impl rule 3 — no corpus instance ever justified them, and keeping them carried unverifiable claims about leak closure.
- [ ] `merge` signature: `pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError>` (matches SPEC §5 verbatim). Warnings surface via `ModIR.warnings: Vec<Finding>`.
- [x] `ReplicaItem` has no `container_name: String` field, no `kind: ReplicaItemKind` field, and (post-Chunk-9, 2026-04-23) no `container: ReplicaItemContainer` field either — the enum was deleted per chunk-impl rule 3 (no corpus instance for `Capture`). `ReplicaItem` models Legendaries only.
- [ ] SPEC §3.6 has been amended to name the permissive whitelist + `Unknown(raw)` variant as the spec-blessed design; SPEC §3.6's pips type annotation reads `i16`.

---

## Rulings (durable, executed as part of this plan)

All rulings below are implemented by specific chunks in this plan. Once the plan executes, the code is the source of truth (SPEC §7); these entries exist so a future reader can see *why* the code looks the way it does without reconstructing the decision history. No residual user decisions remain open.

**R1. FaceId/SpriteId permissive whitelist.**
Ruling: permissive. Any valid textmod per `reference/textmod_guide.md` must extract, even if it uses FaceIDs outside the working-mods corpus. Rationale: SPEC §1 (general-purpose mod-building backend) + SPEC §3.3 (self-contained IR) require that new mods with novel values round-trip. Implementation: `FaceIdValue::{Known(FaceId), Unknown(u16)}` + `SpriteId::owned(name, img_data)`. SPEC §3.6 is amended in Chunk 2's commit to name the `Unknown(raw)` variant as the spec-blessed design — no SPEC drift.

**R2. `ReplicaItem` container shape.**
Ruling (as of 2026-04-23, superseding the original Chunk 6 ruling): `ReplicaItem` models Legendaries only — no `container` field, no `ReplicaItemContainer` enum. Rationale: chunk-impl rule 3 ("Every IR variant discriminator must have at least one corpus instance per variant before it ships"). The original Chunk 6 ruling collapsed `kind` + `container_name` into `ReplicaItemContainer { Capture { name: String }, Legendary }`. That enum shipped in Chunk 6, but no working mod contained a Capture-shaped `ReplicaItem` (classifier never returned `ModifierType::ReplicaItem*`; Captures always routed as `ItemPool` structurals). Chunk 9 (2026-04-23) deleted both the enum and the Capture parsing/emitting paths per the rule. Captures continue to route as `ItemPool` structurals; if a future corpus instance ever demands a typed `ReplicaItem::Capture`, model the variant against the real corpus shape (`((hat.(replica.T.i.(all.(cast.TRIG.abilitydata.(body))))))` per `reference/textmod_guide.md`) rather than resurrecting this ruling's original design. No runtime `ReplicaItemKindMismatch` error variant — the invariant class is vacuous.

**R3. `build_with` introduction.**
`build_with` does not exist today. Chunk 4 introduces it; `build(ir)` becomes `build_with(ir, &BuildOptions::default())`. SPEC §5 already sketches `build_with`, so no SPEC amendment needed.

**R4. `merge` signature retained per SPEC §5.**
Ruling: `pub fn merge(base: &mut ModIR, overlay: ModIR) -> Result<(), CompilerError>` (SPEC §5 verbatim). Warnings surface through a new `ModIR.warnings: Vec<Finding>` sidecar, not via a tuple return. An earlier draft's `Result<(ModIR, Vec<Finding>), CompilerError>` was rejected because it contradicted SPEC §5; the sidecar is the spec-conforming way to carry findings. Implementation: Chunk 5.

**R5. `Pips` type annotation.**
Ruling: `Pips(i16)`. Rationale: corpus contains negative pips (`ir/mod.rs:50` comment); `DiceFace::Active.pips: i16` is already current. SPEC §3.6 amended in Chunk 2's commit (simultaneous with R1's amendment) to read `pips: i16`. No corpus-derived range bound — `try_new` accepts the full `i16` range.

**R6. `FightUnit` sprite name derivation.**
Ruling: under R1's permissive path, the name for `SpriteId::owned(name, img_data)` is sourced from the required `FightUnit.name: String` field (`ir/mod.rs:1181`). Generic unit names (e.g., `"Boss"`) flow through as novel `SpriteId::owned` entries without colliding with the registry. §F4 table reflects this.

**Memory hygiene.** After this plan executes, any memory file that still references older unresolved escalations (E1/E5/E6 wording, tuple-return `merge`, `kind`+`container_name` parallel fields) must be deleted or updated to match the ruling table above. A stale memory is worse than no memory.
