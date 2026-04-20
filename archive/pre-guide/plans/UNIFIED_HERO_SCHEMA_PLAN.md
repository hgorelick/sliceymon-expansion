# Plan: Unified Hero Schema

## Context

All three known-working mods (sliceymon, pansaer, punpuns) use structurally identical hero content — `(replica.Template.col.X.hp.N.sd.FACES.img.DATA).n.Name` — wrapped in different injection mechanisms. The current IR, parser, emitter, and validator are all sliceymon-specific. Non-sliceymon heroes fall to raw passthrough and skip validation.

**Goal**: Replace `HeroTier` with `HeroBlock`, add `HeroFormat`, and update the full pipeline — parser, emitter, validator, all tests — so the compiler works correctly with any mod format.

## Out of Scope

- CharSelect generation from parsed phases (existing TODO in builder/mod.rs)
- Ditto generation from parsed forms (existing TODO in builder/mod.rs)
- Capture/Legendary emission from fields (existing TODO in builder/mod.rs)
- New validation rules beyond content-level block checks (E008-E012, W001-W002)
- Changes to splitter, monster_parser, boss_parser, capture_parser

## The Two Hero Formats (+ one structural)

| | Sliceymon | Grouped (Pansaer/Punpuns) |
|---|---|---|
| **Wrapper** | `hidden&temporary&ph.b{name};1;!mheropool.` | `Heropool.` (pansaer, capital H) / `heropool.` (punpuns, lowercase) |
| **`+` means** | Next tier of SAME hero | Next DIFFERENT hero |
| **Suffix** | `.part.1&hidden.mn.Name@2!m(skip&hidden&temporary)` | `.part.1&Hidden.mn.Group Name` (pansaer) or `.part.1.mn.Group Name` (punpuns, no `&Hidden`) |
| **Heroes/modifier** | 1 | Many (grouped by tier+color) |

**Pool Replacement** (punpuns `((heropool.Thief+Scoundrel+...+(replica.X...)...)&Hidden)`) is NOT a hero definition — it's a structural pool configuration. Reclassify it from `ModifierType::Hero` to a new `ModifierType::PoolReplacement` and store as structural.

## What Gets Built

### Type Changes (`src/ir/mod.rs`)

**Rename `HeroTier` to `HeroBlock`**, add `color` field:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroBlock {
    pub template: String,
    pub tier: Option<u8>,
    pub hp: u16,
    pub sd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,     // NEW: per-block color extracted from .col. in this block's replica content (None if absent)
    pub sprite_name: String,
    pub speech: String,
    pub name: String,
    // Existing optional fields (unchanged):
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abilitydata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggerhpdata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<String>,
    #[serde(default)]
    pub facades: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_inside: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_outside: Option<String>,
}
```

**Rename `Hero.tiers` to `Hero.blocks`**, add `format`:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HeroFormat {
    Sliceymon,       // ph.b prefix, one hero per modifier, + = next tier
    Grouped,         // Bare heropool prefix, multiple heroes per modifier, + = next hero
    Unknown,         // Unrecognized — raw passthrough
}

impl Default for HeroFormat {
    fn default() -> Self { HeroFormat::Unknown }
}

pub struct Hero {
    pub internal_name: String,
    pub mn_name: String,
    /// Hero-level color. For Sliceymon: the single color shared by all blocks.
    /// For Grouped: the color of the first block (each block has its own color via block.color).
    pub color: char,
    #[serde(default)]
    pub format: HeroFormat,         // NEW (replaces implicit assumption of sliceymon)
    pub blocks: Vec<HeroBlock>,     // RENAMED from tiers
    pub removed: bool,
    pub raw: Option<String>,
}
```

**Serde compatibility**: `HeroFormat` implements `Default` (returns `Unknown`) and `Hero.format` has `#[serde(default)]`. This ensures existing serialized JSON (without the `format` field) deserializes correctly as `Unknown` rather than failing.

No parallel representations. `HeroTier` and `hero.tiers` cease to exist.

### Classifier Changes (`src/extractor/classifier.rs`)

Add `PoolReplacement` variant to `ModifierType`. Insert check BEFORE the Hero check:

```
if starts_with_ci "((heropool." → PoolReplacement   (case-insensitive, uses existing starts_with_ci helper)
if contains_ci "heropool" AND contains_ci "replica." → Hero (unchanged)
```

Add `pub fn detect_hero_format(modifier: &str) -> HeroFormat`:
```
if contains_ci "ph.b" AND contains_ci "!mheropool." → Sliceymon
if contains_ci "heropool." AND contains_ci "replica." → Grouped
else → Unknown
```

### Parser Changes (`src/extractor/hero_parser.rs`)

- Rename return type from `HeroTier` to `HeroBlock` in `parse_tier_block()`
- Add `color: extract_color(content)` to block construction
- `parse_hero()` calls `detect_hero_format()` first, then dispatches:
  - **Sliceymon**: Current logic (find `heropool.` marker, extract content, split on `+` at depth 0, `separate_suffix()` on last part, each part -> `parse_tier_block()` -> `HeroBlock`). Sets `format: Sliceymon`.
  - **Grouped**: New `parse_grouped()` — find `heropool.` marker, extract body, split on `+` at depth 0, call `separate_suffix()` on last part (handles both `.part.1&Hidden.mn.` and `.part.1.mn.` suffix variants), each part -> `parse_tier_block()` -> `HeroBlock`. Sets `format: Grouped`.
  - **Unknown**: Raw passthrough (same fallback as now).

### Extractor Changes (`src/extractor/mod.rs`)

- Route `ModifierType::PoolReplacement` -> structural (new `StructuralType::PoolReplacement`)
- Ditto detection: `h.blocks.len() > 10` (renamed from `h.tiers.len()`)

### Builder Changes

**`src/builder/hero_emitter.rs`**: `emit()` dispatches on `hero.format`:

- **Raw passthrough first** (unchanged): If `hero.raw.is_some()`, emit raw directly. This remains the first check. During extraction, `raw` is always populated (even for successfully parsed heroes), so extracted-then-rebuilt heroes always round-trip via raw passthrough. The structured emit paths below serve **Path B** (hand-authored IR where `raw: None`) and future work that clears `raw` after successful parsing.
- **Sliceymon** (`emit_sliceymon`): Current logic, renamed. Uses `hero.blocks`. Color: `block.color.unwrap_or(hero.color)`.
- **Grouped** (`emit_grouped`): New function.
  ```
  Heropool.(block1_replica).n.Name1+(block2_replica).n.Name2+...part.1&Hidden.mn.{mn_name}
  ```
  **Prefix casing**: Use `Heropool.` (capital H, matching pansaer convention). The game's parser is case-insensitive, so both `Heropool.` and `heropool.` work.
  Each block emits the same replica content as sliceymon tier emission (template, color, tier, hp, chain, sd, img) + speech + name. Suffix handling: pansaer uses `.part.1&Hidden.mn.{mn_name}` (capital H, no `@2!m(...)` suffix); punpuns uses `.part.1.mn.{mn_name}` (no `&Hidden` at all). Since `emit_grouped()` reconstructs from IR (not round-tripping), use `.part.1&Hidden.mn.{mn_name}` as the default suffix (matches pansaer convention). Add a `suffix_hidden: bool` field to `Hero` if punpuns-faithful round-trip is later needed (not in this plan -- grouped heroes currently round-trip via raw passthrough).
- **Unknown**: Error if `raw` is None, passthrough if `raw` is Some (unchanged).

**Important**: `emit_grouped()` is testable in isolation (construct a `Hero` with `raw: None, format: Grouped, blocks: [...]` and verify output). It does NOT require extraction to exercise. Round-trip tests for grouped heroes continue to use raw passthrough.

**`src/builder/mod.rs`**: Add `PoolReplacement` to the structural emission loop (emit alongside other structural types, after HeroPoolBase).

### Validator Changes (`src/validator.rs`)

**New dependencies**: Add `use crate::ir::HeroBlock;`, `use crate::extractor::hero_parser;`, and `use crate::extractor::classifier::detect_hero_format;`. The validator currently depends on `classifier::classify` but NOT on `hero_parser` or `HeroBlock`. This adds those dependencies (no circular dependency -- validator -> extractor and validator -> ir are both one-way).

In `validate()`, after classifying a modifier as Hero:
1. Call `detect_hero_format()` to determine format
2. If Sliceymon: call `phase_hero()` for wrapper rules only (E004-E006, E007) -- **remove** the per-tier content checks (E008-E012, W001-W002) from `phase_hero()`, since those move to step 3
3. For ALL formats: call `hero_parser::parse_hero()` to get blocks, then call `phase_hero_blocks()` for content rules

**Refactoring `phase_hero()`**: The current `phase_hero()` (lines 328-552) does BOTH wrapper checks (E004-E007) AND per-tier content checks (E008-E012, W001-W002) via inline text parsing. After this change:
- `phase_hero()` retains: E004 (prefix), E005 (suffix), E006 (tier count), E007 (tier paren match) -- all Sliceymon-specific wrapper rules
- `phase_hero()` loses: The per-tier loop's E008-E012, W001, W002 checks (lines 414-552) -- these move to `phase_hero_blocks()`

New `phase_hero_blocks(blocks: &[HeroBlock], idx: usize, mn: &Option<String>, report: &mut ValidationReport)`:
- For each block: validate face format (E008), face ID range (E009), HP range (E010), color (E011), tier digit (E012), face count (W001), pip range (W002)
- Operates on parsed `HeroBlock` structs (not raw text), so the logic is simpler and reusable across all formats
- **E008/E009/W001/W002**: Split `block.sd` on `:`, call existing `parse_face_entry()` for each face string. This reuses the same validation logic currently in `phase_hero`.
- **E010**: Check `block.hp == 0 || block.hp > 999`. Note: "not numeric" HP is impossible here (parser already converted to u16; parse failures go to raw passthrough and skip block validation).
- **E011**: Check `block.color` if `Some(c)` -- must be `c.is_ascii_lowercase()`. If `None`, skip (no `.col.` in source).
- **E012**: Check `block.tier` if `Some(t)` -- must be `t <= 9`. If `None`, skip (no `.tier.` in source).

### Test Updates

Every file referencing `HeroTier` or `.tiers` gets updated:

| File | Change |
|------|--------|
| `tests/builder_tests.rs` | `HeroTier{...}` -> `HeroBlock{..., color: Some('a')}`, `.tiers` -> `.blocks`, add `format: HeroFormat::Sliceymon` to Hero construction |
| `tests/expansion_tests.rs` | Same rename in `make_hero()`, `.tiers[0].hp` -> `.blocks[0].hp` |
| `tests/ir_tests.rs` | Same rename, update serde test |
| `tests/hero_tests.rs` | `.tiers` -> `.blocks` throughout (28 references) |
| `tests/roundtrip_tests.rs` | `.tiers.len()` -> `.blocks.len()` |
| `tests/validator_tests.rs` | Add tests for grouped-format validation |

New test file: `tests/hero_block_tests.rs`
- `pansaer_heroes_have_format_grouped` — parse pansaer, assert `hero.format == Grouped`
- `pansaer_heroes_have_blocks` — assert `hero.blocks.len() > 0` for each hero
- `punpuns_heroes_have_blocks` — same for punpuns
- `sliceymon_heroes_have_blocks` — regression: blocks populated
- `grouped_hero_block_content_valid` — each block has name, sd, hp > 0

## Parallel Execution Map

```
Chunk 1 (Schema + Rename)
    |
    v
Chunk 2 (Parser + Classifier)  -->  Chunk 3 (Test Updates)
    |                                     |
    v                                     v
Chunk 4 (Emitter + Builder + Validator + New Tests)
```

- **Chunk 1** must complete first (all other chunks depend on the renamed types).
- **Chunks 2 and 3** can execute in parallel after Chunk 1: Chunk 2 changes parser/classifier logic, Chunk 3 does mechanical test renames. Neither writes to the other's files.
- **Chunk 4** depends on both Chunks 2 and 3 (needs parsed `HeroBlock`s to validate/emit, needs tests to compile).

## Implementation Order

### Chunk 1: Schema Rename + Mechanical Propagation

Rename `HeroTier` -> `HeroBlock` everywhere. Rename `hero.tiers` -> `hero.blocks` everywhere. Add `color: Option<char>` to `HeroBlock`. Add `HeroFormat` enum. Add `format: HeroFormat` to `Hero`. Add `PoolReplacement` to `StructuralType`. All changes in this chunk are mechanical renames and field additions — no new logic.

**Read first:**
- `src/ir/mod.rs` (current `HeroTier` and `Hero` definitions, `StructuralType` enum)
- `src/builder/hero_emitter.rs` (lines 16, 31 reference `.tiers`)
- `src/extractor/mod.rs` (line 124 references `.tiers`)

**Files:**
| Action | File |
|--------|------|
| Edit | `src/ir/mod.rs` — rename `HeroTier` -> `HeroBlock`, add `color` field, add `HeroFormat` enum, rename `Hero.tiers` -> `Hero.blocks`, add `Hero.format`, add `PoolReplacement` to `StructuralType` |
| Edit | `src/extractor/hero_parser.rs` — rename `HeroTier` import and return types to `HeroBlock` (line 2 import, line 190 return type, line 244 construction), rename `tiers` -> `blocks` in `try_parse_hero()` (lines 77, 80, 98), add `color: None` to block construction (line 244), add `format: HeroFormat::Sliceymon` to Hero construction in `try_parse_hero()` success path (line 97), add `format: HeroFormat::Unknown` to fallback Hero construction in `parse_hero()` error path (lines 15-22). **Note**: line 73 has `let last = tier_strs.last_mut().unwrap()` -- this is safe because the `is_empty()` check on line 61 guarantees at least one element, but document this with a comment. |
| Edit | `src/extractor/mod.rs` — rename `.tiers` -> `.blocks` (line 124) |
| Edit | `src/builder/hero_emitter.rs` — rename `.tiers` -> `.blocks` (line 16: `hero.tiers.is_empty()`, line 31: `hero.tiers.iter()`) |

**TDD**: No new test cases in this chunk — this is a mechanical rename. All existing tests are updated in Chunk 3 to match the new names. This chunk and Chunk 3 must both complete before `cargo test` can pass.

**Verification (enumerated, checked after Chunks 1+3 both complete):**
1. `cargo check` passes (type-level compilation) -- verifiable after Chunk 1 alone
2. `grep -rn "HeroTier" compiler/src/` returns 0 results
3. `grep -rn "\.tiers\|tiers:" compiler/src/` returns 0 results
4. `HeroFormat` enum exists with `Sliceymon`, `Grouped`, `Unknown` variants
5. `Hero` struct has `format: HeroFormat` field with `#[serde(default)]`
6. `HeroBlock` struct has `color: Option<char>` field
7. `StructuralType::PoolReplacement` variant exists

**Checkpoint**: `cargo check` passes (type-level compilation). Full `cargo test` requires Chunk 3 (test updates).

**If blocked**: If a file references `.tiers` or `HeroTier` that was missed, find it with `grep -rn "HeroTier\|\.tiers" compiler/` and fix.

### Chunk 2: Parser + Classifier Generalization

Add `PoolReplacement` variant to `ModifierType`. Add `detect_hero_format()`. Add grouped-format parsing path. Route `PoolReplacement` to structural in extractor. Add `PoolReplacement` emission in builder.

**Read first:**
- `src/extractor/classifier.rs` (current `classify()` function, `ModifierType` enum)
- `src/extractor/hero_parser.rs` (current `try_parse_hero()` and `parse_tier_block()`)
- `src/extractor/mod.rs` (current extraction routing)
- `src/builder/mod.rs` (structural emission loop)

**Files:**
| Action | File |
|--------|------|
| Edit | `src/extractor/classifier.rs` — add `PoolReplacement` to `ModifierType`, add `detect_hero_format()`, insert pool replacement check before Hero check |
| Edit | `src/extractor/hero_parser.rs` — add `parse_grouped()`, dispatch on format in `parse_hero()`. Change `color: None` to `color: util::extract_color(replica_content)` in `parse_tier_block()` (line 244 area) so all blocks get their per-block color populated. |
| Edit | `src/extractor/mod.rs` — add `ModifierType::PoolReplacement` match arm routing to `StructuralType::PoolReplacement` (insert before `ModifierType::Hero` arm, around line 29). Pattern follows existing structural routing (e.g., `HeroPoolBase` arm on line 35). |
| Edit | `src/builder/mod.rs` — add `PoolReplacement` filter in structural emission loop (insert after step 6 HeroPoolBase, before step 7 Heroes, around line 63). Pattern: `for s in ir.structural.iter().filter(|s| s.modifier_type == StructuralType::PoolReplacement) { modifiers.push(structural_emitter::emit(s)); }` |

**TDD (enumerated test cases — added in Chunk 4):**
1. `pansaer_heroes_have_format_grouped` — parse pansaer mod, assert all parsed heroes have `hero.format == HeroFormat::Grouped`
2. `punpuns_heroes_have_blocks` — parse punpuns mod, assert each hero has `hero.blocks.len() > 0`
3. `grouped_hero_block_content_valid` — for each pansaer hero block, assert: `name` non-empty, `sd` non-empty, `hp > 0`
4. `pool_replacement_classified_correctly` — feed a `((heropool.Thief+...` modifier to `classify()`, assert `ModifierType::PoolReplacement`
5. `pool_replacement_routed_to_structural` — extract punpuns mod, assert no hero has raw containing `((heropool.`; assert structural has a `PoolReplacement` entry

**Checkpoint**: `cargo check` passes. Full `cargo test` requires Chunks 3 and 4.

**If blocked:**
- If grouped block parsing hits unexpected structure: fall back to `HeroBlock` with defaults and `raw` passthrough for that block. Never crash.
- If pool replacement reclassification breaks existing tests: the pool replacement modifier was never meaningfully parsed as a Hero (always raw passthrough). Reclassifying it as structural is a no-op for the emitter.

### Chunk 3: Mechanical Test Renames

Update all existing test files: `HeroTier` -> `HeroBlock`, `.tiers` -> `.blocks`, add `color: None` (or `Some('a')` where a color is set), add `format: HeroFormat::Sliceymon` to Hero construction.

**Read first:**
- `tests/builder_tests.rs` (3 `HeroTier` constructors, lines 23/40/57)
- `tests/expansion_tests.rs` (`make_hero()` function, lines 14/61/69)
- `tests/ir_tests.rs` (serde test, lines 11/48-49)
- `tests/hero_tests.rs` (28 `.tiers` references)
- `tests/roundtrip_tests.rs` (lines 74-75)

**Files:**
| Action | File |
|--------|------|
| Edit | `tests/hero_tests.rs` — `.tiers` -> `.blocks` (28 sites). No `HeroTier` constructors or `tiers:` field inits in this file (it uses `parse_hero()` which returns the struct). |
| Edit | `tests/builder_tests.rs` — `HeroTier` -> `HeroBlock` (3 constructors, lines 23/40/57), add `color: None` to each, add `format: HeroFormat::Sliceymon` to `simple_hero()`. `tiers:` -> `blocks:` field init (lines 22, 147). Raw passthrough test (line 143): add `format: HeroFormat::Unknown` (raw hero is not Sliceymon). |
| Edit | `tests/expansion_tests.rs` — `HeroTier` -> `HeroBlock` (1 constructor, line 14), `tiers:` -> `blocks:` field init (line 14), add `color: None`, add `format: HeroFormat::Sliceymon` to `make_hero()`. `.tiers[0]` -> `.blocks[0]` (lines 61, 69). |
| Edit | `tests/ir_tests.rs` — `HeroTier` -> `HeroBlock` (3 refs: lines 11, 47 test name, 49 constructor), `tiers:` -> `blocks:` field init (line 11), add `color: None`, add `format: HeroFormat::Sliceymon` to constructed Heroes. Rename test `hero_tier_required_fields` -> `hero_block_required_fields`. |
| Edit | `tests/roundtrip_tests.rs` — `.tiers.len()` -> `.blocks.len()` |

**TDD (enumerated verification after Chunks 1+2+3 combined):**
1. All 11 builder_tests pass (emit_hero_paren_balanced through build_assembly_order)
2. All 9 expansion_tests pass (merge_adds_new_hero through expansion_no_duplicate_pokemon)
3. All 4 ir_tests pass (ir_types_serialize_roundtrip, hero_block_required_fields, compiler_error_display, empty_mod_ir_serializes)
4. All 14 hero_tests pass (parse_simple_hero_no_spells through parse_pansaer_hero)
5. All 7 roundtrip_tests pass (roundtrip_sliceymon through build_output_is_valid_text)
6. `grep -rn "HeroTier" compiler/tests/` returns 0 results
7. `grep -rn "\.tiers" compiler/tests/` returns 0 results (also check `tiers:` field initializers)
8. Raw passthrough test uses `format: HeroFormat::Unknown`, not `HeroFormat::Sliceymon`

This is purely a rename + field addition -- no assertion logic changes. Every test that passed before must pass after.

**Checkpoint**: `cargo test` passes (combined with Chunks 1 + 2), `cargo clippy -- -D warnings` clean.

**If blocked**: If a test references `HeroTier` or `.tiers` that was missed, find it with `grep -rn "HeroTier\|\.tiers" compiler/tests/` and fix.

### Chunk 4: Emitter Dispatch + Validator Generalization + New Tests

Emitter dispatches on `HeroFormat` — emit grouped format. Validator applies content rules to all blocks universally. Add all new test files.

**Read first:**
- `src/builder/hero_emitter.rs` (current `emit()` function — sliceymon-only logic)
- `src/validator.rs` (current `phase_hero()` function, rules E004-E012)
- `tests/validator_tests.rs` (existing validation tests for reference)

**Files:**
| Action | File |
|--------|------|
| Edit | `src/builder/hero_emitter.rs` — dispatch on `hero.format` after raw passthrough check, add `emit_grouped()`, rename error message "hero has no tiers" -> "hero has no blocks" (line 19). Color resolution in emit paths: `block.color.unwrap_or(hero.color)`. |
| Edit | `src/validator.rs` — add `phase_hero_blocks()`, call for ALL heroes; `phase_hero()` wrapper rules only for Sliceymon |
| Edit | `tests/builder_tests.rs` — add `emit_grouped_hero_produces_valid_output` and `emit_grouped_hero_no_sliceymon_suffix` tests |
| Edit | `tests/validator_tests.rs` — add grouped-format validation tests |
| Create | `tests/hero_block_tests.rs` — grouped format parsing tests |

**TDD (enumerated test cases):**

In `tests/hero_block_tests.rs`:
1. `pansaer_heroes_have_format_grouped` — parse pansaer, each hero has `format == Grouped`
2. `pansaer_heroes_have_blocks` — each pansaer hero has `blocks.len() > 0`
3. `punpuns_heroes_have_blocks` — each punpuns hero has `blocks.len() > 0`
4. `sliceymon_heroes_have_blocks` — regression: each sliceymon hero (non-raw) has `blocks.len() > 0`
5. `grouped_hero_block_content_valid` — each pansaer block: name non-empty, sd non-empty, hp > 0
6. `pool_replacement_classified_correctly` — `((heropool.Thief+...` -> `ModifierType::PoolReplacement`
7. `pool_replacement_routed_to_structural` — extract punpuns, no hero raw contains `((heropool.`, structural has `PoolReplacement`

In `tests/builder_tests.rs` (additions):
8. `emit_grouped_hero_produces_valid_output` — construct a `Hero` with `raw: None, format: Grouped, blocks: [2 HeroBlocks with different colors]`, call `emit()`, verify: contains `heropool.`, balanced parens, ASCII-only, contains both `.n.Name1` and `.n.Name2`
9. `emit_grouped_hero_no_sliceymon_suffix` — same hero as test 8, verify output does NOT contain `@2!m(skip&hidden&temporary)` (that suffix is sliceymon-only)

In `tests/validator_tests.rs` (additions):
10. `validate_grouped_hero_blocks_checked` — construct a grouped-format hero modifier with an invalid face (face ID 999), validate, assert E009 fires
11. `validate_grouped_hero_no_wrapper_errors` — validate pansaer mod, assert zero E004/E005/E006 errors (wrapper rules skip grouped heroes)

**Checkpoint**: All 3 working mods produce 0 errors with `validate()`. Content rules fire on pansaer/punpuns blocks. `cargo test` + `cargo clippy -- -D warnings` clean.

**If blocked:**
- If a working mod produces new false-positive errors: the mod is ground truth. Relax the rule, add TODO, report.
- If grouped emit doesn't round-trip perfectly: grouped heroes currently use raw passthrough on emit; the new `emit_grouped()` is additive. Fallback to raw if `hero.raw.is_some()` is preserved.

## Reuse

- `util::split_at_depth0` — split grouped body on `+`
- `util::find_at_depth0` — find markers
- `util::find_matching_close_paren` — extract replica blocks
- `util::extract_color` — populate `block.color`
- `parse_tier_block()` — already parses `(replica.X...).n.Name` content. Keep the name `parse_tier_block` (renaming it to `parse_block` is cosmetic and not required for correctness; the function is private to `hero_parser.rs`)

## Self-Verification Checklist

- [ ] `HeroTier` does not exist anywhere in the codebase
- [ ] `hero.tiers` does not exist anywhere in the codebase (check both `.tiers` accessors and `tiers:` field initializers)
- [ ] All 3 working mods produce 0 errors with `validate()`
- [ ] `cargo test` passes with 0 failures
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `hero.blocks` populated for sliceymon AND grouped formats
- [ ] Emitter handles both Sliceymon and Grouped formats (Grouped tested via Path B unit test with `raw: None`)
- [ ] Content rules (E008-E012) fire on all hero blocks regardless of format
- [ ] Wrapper rules (E004-E006) only fire on sliceymon heroes
- [ ] Serde roundtrip works: existing JSON without `format` field deserializes to `HeroFormat::Unknown` (via `#[serde(default)]`)
- [ ] Serde roundtrip works: JSON with `format` field round-trips correctly
- [ ] No `unwrap()` or `panic!()` in new parsing code
- [ ] `PoolReplacement` in `StructuralType` and emitted by builder
- [ ] Raw passthrough test uses `HeroFormat::Unknown`, not `HeroFormat::Sliceymon`
