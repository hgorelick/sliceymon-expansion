# Plan: Textmod Compiler — Complete Pipeline Fix

## Vision

The textmod compiler is the backend for a web/mobile mod-building app. Users create heroes, replica items, monsters, bosses from scratch via structured data (JSON). The compiler validates, assembles, and exports a pasteable textmod. The CLI is a first-class interface to the same functionality.

Every feature must be a **library function first**, CLI as thin wrapper.

## Current State (broken)

The compiler parses textmod strings into structured IR, then **throws away the structure** and emits the raw strings verbatim. It's a parser that never uses its parse results. Specifically:

```
CURRENT PIPELINE:
  textmod ──► Extractor ──► IR (fields + raw) ──► Builder ──► textmod
                                                      │
                                                 uses raw only,
                                                 ignores fields

  No way to:
  - Build a mod from hand-authored JSON
  - Add/remove/edit a single hero
  - Preview one modifier
  - Validate a hand-authored hero
  - Auto-generate character selection from hero list
  - Know which content is base vs custom
```

## Target State

```
TARGET PIPELINE:
  textmod ──► Extractor ──► IR (fields only, self-contained)
                              │
                              ├──► build_full(ir) → complete textmod
                              ├──► build_single(hero) → one modifier string (preview)
                              ├──► validate_single(hero) → semantic errors
                              ├──► add/remove/update_hero(ir, hero) → mutated IR
                              ├──► auto-generate char selection, hero pools from hero list
                              └──► export schema → JSON Schema for editors

  hero.json ──► deserialize ──► same IR ──► same builder
                                              │
                                         (same code path
                                          for extracted and
                                          hand-authored)
```

## Problems to Fix

### P1: Raw Passthrough (3 layers)

| Layer | Where | Effect |
|-------|-------|--------|
| `original_modifiers` | `ModIR` | Builder skips all assembly, emits raw |
| `raw: Option<String>` | Hero, ReplicaItem, Monster, Boss | Each emitter short-circuits to raw |
| `raw: String` | StructuralModifier | Structural emitter always emits raw |

### P2: Missing `.img.` Data Extraction

Sprite data (`.img.ENCODED_STRING`) embedded in modifiers is not extracted to IR fields. Without raw, this data is lost. Only BossFightUnit.sprite_data does it correctly.

### P3: Emitter Gaps

| Type | Missing |
|------|---------|
| Hero | `hue`, `facades`, `items_inside` not emitted |
| ReplicaItem | No `.img.` handling (replica items have `.img.` data -- extract and emit it just like heroes) |
| Monster | No `.img.` handling |
| Boss | Redundant top-level fields duplicate fight_units |
| Structural | 0% field-based emission |

### P4: No CRUD Operations

No library functions to add/remove/update individual items in the IR. The only mutation path is merge(), which replaces entire types by name.

### P5: No Single-Item Build/Validate

`build()` produces a complete textmod. `validate()` validates a complete textmod string. No way to build or validate a single hero, replica item, etc.

### P6: No Derived Structural Modifiers

Several structural modifiers are derived from other content:
- **Character Selection** (Selector) — lists hero colors with labels, auto-generated from hero list
- **HeroPoolBase** — lists available heroes
- **PoolReplacement** — hero pool overrides
- **Hero-specific ItemPools** (PorygonItem, DittoItem) — derived from hero data

Currently these are stored as raw strings. The builder should auto-generate them from the hero/replica item lists.

### P7: Validation Is Shallow

The validator catches structural issues (parens, face count) but not semantic hand-authoring mistakes:
- Invalid Face ID for template (Face 42 valid for Statue but not Fey)
- Duplicate color assignment
- Duplicate Pokemon across hero/replica_item/monster/boss
- Invalid template name
- Missing required fields for a type

### P8: Errors Are Developer-Facing

`CompilerError` and `Finding` return flat strings. A web app needs:
- Error code (rule ID)
- Affected field path (`heroes[3].blocks[2].sd`)
- Human-readable message
- Fix suggestion

### P9: No Provenance Tracking

No way to distinguish "this hero came from the base mod" vs "the user added this." Needed for UI (show base vs custom), undo, and diff.

### P10: No JSON Schema

IR types serialize to JSON but there's no JSON Schema document for editor validation/autocomplete.

### P11: Patch Command Is Raw Text Surgery

The `patch` CLI operates on raw strings, never touches IR, still checks for `line_new_` prefix.

---

## Source of Truth Files

| File | Purpose | When to Reference |
|------|---------|-------------------|
| `compiler/src/ir/mod.rs` | IR types -- the mod schema | All chunks (IR is the central artifact) |
| `compiler/src/builder/hero_emitter.rs` | Hero emission pattern (reference for other emitters) | Chunks 2, 8 |
| `compiler/src/builder/structural_emitter.rs` | Structural emission (currently 100% raw fallback) | Chunk 6 |
| `compiler/src/extractor/structural_parser.rs` | Structural parsing (partial content extraction) | Chunk 6 |
| `compiler/src/validator.rs` | Existing rules E001-E016, W001-W007 | Chunks 8, 10, 11 |
| `compiler/src/error.rs` | CompilerError variants | Chunks 7, 11 |
| `compiler/src/lib.rs` | Public API surface | Chunks 7, 8, 13 |
| `compiler/Cargo.toml` | Dependencies (serde, clap) | Chunk 13 |
| `SLICEYMON_AUDIT.md` | Face IDs per template, property codes | Chunk 10 |
| `working-mods/` | pansaer.txt, punpuns.txt, community.txt | Chunks 2-6, 13 |
| `textmod.txt` | sliceymon base mod -- primary round-trip test | Chunks 2-6, 13 |

## Out of Scope

- **WASM bindings**: Library is WASM-safe (no std::fs in lib code), but wasm-bindgen integration is deferred
- **Web/mobile app frontend**: This plan covers the library + CLI backend only
- **Sprite encoding/downloading**: Handled by Node.js tooling, not the Rust compiler
- **Game balance validation**: Only structural/semantic validation (not "is this hero too strong?")
- **Migration tooling**: For hand-authored textmods outside the 4 known working mods
- **Performance optimization**: Correctness first; optimization is a separate concern
- **New game mechanics**: No new Face IDs, templates, or game features

---

## Checkpoint Configuration

- Total chunks: 13
- Checkpoint frequency: After each chunk
- Critical checkpoints:
  - After Chunk 4 (all `raw: Option<String>` removed from content types)
  - After Chunk 6 (all raw removed including structural `raw: String` + original_modifiers -- zero raw in IR)
  - After Chunk 9 (CRUD + single-item + derived structurals -- full library API)
  - After Chunk 13 (full integration -- JSON Schema, round-trip, overlay workflow)

## Parallel Execution Map

```
Chunk 1:  Foundation -- img_data extraction for all types
├── Chunk 2:  Remove raw from Hero + fix emitter gaps          [PARALLEL GROUP A]
├── Chunk 3:  Remove raw from ReplicaItem + fix emitters       [PARALLEL GROUP A]
└── Chunk 4:  Remove raw from Monster/Boss + fix emitters      [PARALLEL GROUP A]
    └── Chunk 5:  Remove original_modifiers from ModIR
        └── Chunk 6:  Structural field-based emission (replace raw: String)
            ├── Chunk 7:  CRUD API -- add/remove/update per type           [PARALLEL GROUP B]
            ├── Chunk 8:  Single-item build + validate                     [PARALLEL GROUP B]
            └── Chunk 9:  Derived structural modifiers (auto-gen)          [PARALLEL GROUP B]
                └── Chunk 10: Semantic validation (face IDs, templates, duplicates)
                    ├── Chunk 11: Structured errors (field paths, suggestions) [PARALLEL GROUP C]
                    └── Chunk 12: Replace patch with overlay + provenance      [PARALLEL GROUP C]
                        └── Chunk 13: JSON Schema + round-trip integration tests
```

Parallel Groups:
- **Group A** (after Chunk 1): Chunks 2, 3, 4 -- each removes raw from different IR types
- **Group B** (after Chunk 6): Chunks 7, 8, 9 -- independent library features
- **Group C** (after Chunk 10): Chunks 11, 12 -- independent enhancements

**NOTE on ir/mod.rs concurrency in Group A**: Chunks 2, 3, 4 all modify `ir/mod.rs` but touch different, non-overlapping structs (Hero, ReplicaItem, Monster/Boss respectively). Each agent removes `raw` from its assigned structs only. The changes are non-overlapping and merge cleanly. However, if Git merge conflicts are a concern, execute them sequentially (adds 2 rounds).

Minimum wall-clock rounds: 8 (1 -> {2,3,4} -> 5 -> 6 -> {7,8,9} -> 10 -> {11,12} -> 13)
Sequential alternative: 13 rounds

---

## Chunk 1: Foundation -- Extract img_data for All Types

**Scope**: Add `img_data: Option<String>` to HeroBlock, ReplicaItem, Monster. Write `extract_img_data()` utility. Wire into all extractors.
**Dependencies**: None
**Read first**: `compiler/src/ir/mod.rs`, `compiler/src/util.rs`, `compiler/src/extractor/hero_parser.rs`

**Tests FIRST**:
```rust
fn extract_img_data_basic()           // ".img.ABC123.n.X" -> Some("ABC123")
fn extract_img_data_at_end()          // ".img.ABC123" -> Some("ABC123")
fn extract_img_data_before_paren()    // ".img.ABC123).n.Y" -> Some("ABC123")
fn extract_img_data_missing()         // "no image" -> None
fn hero_blocks_have_img_data()        // Extract sliceymon.txt -> every block has img_data
fn replica_item_has_img_data()        // Replica items with sprites have img_data
fn monster_has_img_data()             // Monsters with .img. have img_data
fn boss_fight_units_have_sprite_data() // Already works -- verify not broken
```

**Implementation**:
- `util.rs`: Add `pub fn extract_img_data(s: &str) -> Option<String>`. **Note**: `hero_parser.rs` already has a local `extract_img_at_depth0()` (line 421) that does this but discards the result into `_img` (line 371). Remove that local function and replace with `util::extract_img_data()`. `boss_parser.rs` already extracts sprite data via `extract_simple_prop(".img.")` into `BossFightUnit.sprite_data` -- verify this still works.
- `ir/mod.rs`: Add `img_data: Option<String>` to HeroBlock, ReplicaItem, Monster
- `extractor/hero_parser.rs`: Replace `_img = extract_img_at_depth0()` with `img_data = util::extract_img_data()`, remove local `extract_img_at_depth0` fn
- `extractor/replica_item_parser.rs`, `monster_parser.rs`: Call `util::extract_img_data()` and store in new `img_data` field

**Files** (5 modified):
1. `compiler/src/util.rs` -- add extract_img_data()
2. `compiler/src/ir/mod.rs` -- add img_data field to HeroBlock, ReplicaItem, Monster
3. `compiler/src/extractor/hero_parser.rs` -- call extract_img_data() for each block
4. `compiler/src/extractor/replica_item_parser.rs` -- call extract_img_data() for ReplicaItem (both simple and with-ability)
5. `compiler/src/extractor/monster_parser.rs` -- call extract_img_data() for Monster

**Note**: Existing test files with constructors (e.g., `replica_item_emitter.rs` tests, `monster_emitter.rs` tests, `boss_emitter.rs` tests) will need `img_data: None` added to struct literals. These are mechanical additions within the files already listed or their test modules.

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all existing tests still pass (img_data defaults to None via serde)
- [ ] New img_extraction tests pass
- [ ] Extract sliceymon.txt -> hero blocks have img_data populated

**If blocked**: If `.img.` extraction is ambiguous (e.g., nested `.img.` inside abilitydata), extract only from the replica block content, not from nested spell data. The BossFightUnit.sprite_data pattern in boss_parser.rs is the reference implementation.

---

## Chunk 2: Remove Raw from Hero + Fix Emitter [PARALLEL GROUP A]

**Scope**: Remove `raw: Option<String>` from Hero. Fix emitter: use img_data, emit hue/facades/items_inside. Replace blob `String` fields with typed schemas: `sd` → `DiceFaces`, `abilitydata` → `AbilityData`, `modifier_chain`/`items_inside`/`items_outside` → `ModifierChain`.
**Dependencies**: Chunk 1
**Parallel with**: Chunks 3, 4
**Read first**: `compiler/src/ir/mod.rs` (Hero struct), `compiler/src/builder/hero_emitter.rs`, `compiler/src/extractor/hero_parser.rs`

**Tests FIRST**:
```rust
fn hero_emits_using_img_data()        // img_data on blocks, no sprite map -> output has .img.{data}
fn hero_sprite_map_overrides_img_data() // Sprite map wins (allows refresh)
fn hero_emits_hue()                   // hue: Some("10") -> .hue.10 in output
fn hero_emits_facades()               // facades populated -> appear in output
fn hero_emits_items_inside()          // items_inside -> inside replica block
fn hero_sd_is_typed_dice_faces()      // sd field is DiceFaces, not String -- validates face count and IDs
fn hero_abilitydata_is_typed()        // abilitydata is AbilityData struct with template/sd/img_data/name
fn extracted_hero_round_trips()       // Extract -> emit -> re-extract -> field comparison
```

**Implementation**: Remove raw from Hero struct/parser/emitter. Sprite resolution order: sprite map > img_data > error. Add hue/facades/items_inside emission. **Type upgrades on HeroBlock**: `sd: String` → `DiceFaces`, `abilitydata: Option<String>` → `Option<AbilityData>`, `modifier_chain: Option<String>` → `Option<ModifierChain>`, `items_inside: Option<String>` → `Option<ModifierChain>`, `items_outside: Option<String>` → `Option<ModifierChain>`. Parser must parse these into typed structs; emitter must emit from typed fields.

**Files** (3 modified):
1. `compiler/src/ir/mod.rs` -- remove `raw: Option<String>` from Hero (lines 56-57 only)
2. `compiler/src/extractor/hero_parser.rs` -- remove raw fallback, make parse failure a hard error
3. `compiler/src/builder/hero_emitter.rs` -- remove raw passthrough, add img_data fallback, emit hue/facades/items_inside

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all pass, no raw references in hero code paths
- [ ] Round-trip: extract(emit(extract(sliceymon.txt))) == extract(sliceymon.txt) for heroes
- [ ] `~/.cargo/bin/cargo clippy` -- no warnings

**If blocked**: If hero fields (hue, facades, items_inside) are not extracted by the parser for some heroes, the round-trip test will fail. Check the parser first -- if it extracts the field but the emitter does not use it, that is this chunk's bug. If the parser does not extract it, fix the parser in this chunk too (it touches hero_parser.rs).

---

## Chunk 3: Remove Raw from ReplicaItem [PARALLEL GROUP A]

**Scope**: Remove raw from ReplicaItem. Fix emitters: add .img. emission using img_data. Replace blob `String` fields with typed schemas: `sd` → `DiceFaces`, `item_modifiers` → `ModifierChain`, `abilitydata` → `AbilityData`.
**Dependencies**: Chunk 1
**Parallel with**: Chunks 2, 4
**Read first**: `compiler/src/ir/mod.rs` (ReplicaItem struct), `compiler/src/builder/replica_item_emitter.rs`, `compiler/src/extractor/replica_item_parser.rs`

**Tests FIRST**:
```rust
fn replica_item_emits_all_fields_including_img()  // img_data present -> .img.{data} in output
fn replica_item_with_ability_emits_img()          // with-ability variant emits .img. and .cast.
fn replica_item_sd_is_typed_dice_faces()          // sd field is DiceFaces -- validates face IDs
fn replica_item_modifiers_is_typed()              // item_modifiers is ModifierChain, not String
fn replica_item_abilitydata_is_typed()            // abilitydata is AbilityData struct
fn replica_item_field_round_trip()                // extract -> emit -> re-extract -> fields match
```

**Implementation**: Remove raw from ReplicaItem struct and parser fallbacks. Add img_data emission to replica_item_emitter. **Replica items have `.img.` data** -- verified in textmod.txt (e.g., line 105 shows itempool replica items with inline `.img.` sprite encodings). The emitter must always emit `.img.` when `img_data` is `Some`, just like heroes. **Type upgrades on ReplicaItem**: `sd: String` → `DiceFaces`, `item_modifiers: Option<String>` → `Option<ModifierChain>`, `abilitydata: Option<String>` → `Option<AbilityData>`.

**Files** (3 modified):
1. `compiler/src/ir/mod.rs` -- remove `raw: Option<String>` from ReplicaItem
2. `compiler/src/extractor/replica_item_parser.rs` -- remove `raw: Some(modifier.to_string())` from both parse functions
3. `compiler/src/builder/replica_item_emitter.rs` -- remove raw passthrough, add .img. emission using img_data (emit when `img_data` is `Some`, same as heroes)

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all pass
- [ ] Round-trip: replica items survive extract -> emit -> re-extract
- [ ] No `raw` field references remain in replica item code paths

---

## Chunk 4: Remove Raw from Monster + Boss [PARALLEL GROUP A]

**Scope**: Remove raw from Monster and Boss. Fix emitters. Clean up Boss redundant top-level fields. Replace blob `String` fields with typed schemas: `sd` → `DiceFaces`, `modifier_chain` → `ModifierChain`.
**Dependencies**: Chunk 1
**Parallel with**: Chunks 2, 3
**Read first**: `compiler/src/ir/mod.rs` (Monster/Boss structs), `compiler/src/builder/monster_emitter.rs`, `compiler/src/builder/boss_emitter.rs`

**Tests FIRST**:
```rust
fn monster_emits_all_fields_including_img()  // img_data present -> .img.{data} in output
fn boss_emits_from_fight_units_only()        // no top-level template/hp/sd -> uses fight_units
fn monster_sd_is_typed_dice_faces()          // sd field is DiceFaces -- validates face IDs
fn monster_modifier_chain_is_typed()         // modifier_chain is ModifierChain, not String
fn boss_fight_unit_sd_is_typed()             // BossFightUnit.sd is DiceFaces
fn monster_field_round_trip()                 // extract -> emit -> re-extract -> fields match
fn boss_field_round_trip()                    // extract -> emit -> re-extract -> fields match
fn boss_without_redundant_toplevel()          // Boss IR has no template/hp/sd/sprite_name at top level
```

**Implementation**: Remove raw from Monster/Boss. **Remove `template`, `hp`, `sd`, `sprite_name` from Boss** -- these are purely redundant duplicates of fight_units[0] data (the parser copies them from the first fight unit during extraction; the emitter never reads them; no other code references them). This is a verified code fact, not a design decision. **Preserve Boss.variant and Boss.level** -- these are NOT redundant (variant is used for classification, level is the ch.omN value, and neither exists in BossFightUnit). Add monster img_data emission. Boss emitter already uses fight_units (verified in source). **Type upgrades on Monster**: `sd: Option<String>` → `Option<DiceFaces>`, `modifier_chain: Option<String>` → `Option<ModifierChain>`. **Type upgrades on Boss**: `modifier_chain: Option<String>` → `Option<ModifierChain>`. **Type upgrades on BossFightUnit**: `sd: Option<String>` → `Option<DiceFaces>`.

**Files** (5 modified):
1. `compiler/src/ir/mod.rs` -- remove `raw: Option<String>` from Monster (line 154) and Boss (line 179); remove redundant Boss top-level fields (template, hp, sd, sprite_name) but keep variant and level
2. `compiler/src/extractor/monster_parser.rs` -- remove raw fallback
3. `compiler/src/extractor/boss_parser.rs` -- remove raw fallback, stop populating redundant top-level fields
4. `compiler/src/builder/monster_emitter.rs` -- remove raw passthrough, add .img. emission
5. `compiler/src/builder/boss_emitter.rs` -- remove raw passthrough

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all pass
- [ ] Round-trip: monsters and bosses survive extract -> emit -> re-extract
- [ ] No `raw` field references remain in monster/boss code paths
- [ ] Boss struct has no redundant top-level template/hp/sd/sprite_name

**No deferral**: Removing Boss top-level fields (template, hp, sd, sprite_name) is mandatory in this chunk. These fields are dead code -- the emitter only reads fight_units. Fix all callsites that reference them.

---

## Chunk 5: Remove original_modifiers from ModIR

**Scope**: Remove `original_modifiers: Option<Vec<String>>` from ModIR. Force the builder to use type-based assembly for all mods (not just merged ones). This is the prerequisite for structural field-based emission.
**Dependencies**: Chunks 2, 3, 4 (all content-type raw removed)
**Read first**: `compiler/src/ir/mod.rs` (ModIR.original_modifiers), `compiler/src/builder/mod.rs` (original_modifiers passthrough at line 23), `compiler/src/extractor/mod.rs` (sets original_modifiers at line 108), `compiler/src/ir/merge.rs` (clears original_modifiers at line 87)

**Tests FIRST**:
```rust
fn extracted_ir_has_no_original_modifiers()  // ModIR struct has no original_modifiers field
fn build_uses_type_assembly()                // build() always assembles from typed fields
fn build_produces_same_modifier_count()      // extract(sliceymon.txt) -> build -> same number of modifiers
```

**Implementation**: Remove original_modifiers from ModIR struct, ModIR::empty(), extractor output, builder passthrough check, and merge clear. Builder now always takes the type-based assembly path (currently lines 33-141 of builder/mod.rs).

**Files** (4 modified):
1. `compiler/src/ir/mod.rs` -- remove `original_modifiers` field from ModIR struct and ModIR::empty()
2. `compiler/src/extractor/mod.rs` -- remove `original_modifiers: Some(modifier_strings)` from extract() return
3. `compiler/src/builder/mod.rs` -- remove `if let Some(ref originals) = ir.original_modifiers` passthrough block (lines 23-29)
4. `compiler/src/ir/merge.rs` -- remove `base.original_modifiers = None` (line 87)

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all pass
- [ ] `~/.cargo/bin/cargo clippy` -- no warnings about original_modifiers
- [ ] Build from extracted IR produces valid output (type-based assembly works end-to-end)

**If blocked**: If type-based assembly produces subtly different output than raw passthrough (e.g., different modifier ordering, whitespace), the round-trip tests will fail. Fix the builder ordering to match the expected output, don't re-add original_modifiers.

---

## Chunk 6: Structural Field-Based Emission (replace raw: String)

**Scope**: Replace `raw: String` on StructuralModifier with full field-based parsing and emission for all structural types. After this chunk, there are zero raw fields anywhere in the IR.
**Dependencies**: Chunk 5 (original_modifiers removed)
**Read first**: `compiler/src/ir/mod.rs` (StructuralModifier, StructuralContent), `compiler/src/builder/structural_emitter.rs` (currently always emits raw), `compiler/src/extractor/structural_parser.rs` (current content parsing)

**Structural types and their target schemas** (reverse-engineered from real textmod examples):

### Shared types used across ALL modifier types

```rust
/// Dice faces — the .sd. field on heroes, replica items, monsters, spells, etc.
/// Format: colon-separated entries, each is "0" (blank) or "FaceID-Pips"
/// Hero dice always have 6 faces. Spells/items may have fewer.
/// Example: "34-1:30-1:0:0:30-1:0" → [Active(34,1), Active(30,1), Blank, Blank, Active(30,1), Blank]
struct DiceFaces {
    faces: Vec<DiceFace>,
}

enum DiceFace {
    Blank,                         // "0"
    Active {
        face_id: u16,              // game Face ID (validated against SLICEYMON_AUDIT.md per template)
        pips: u8,                  // number of pips (1-8 typically)
    },
}

/// Modifier chain — the equipment, facades, triggers, and properties applied to a
/// monster, boss, replica item, or other entity. Replaces raw `modifier_chain: Option<String>`
/// and `item_modifiers: Option<String>` fields.
/// Example: "i.hat.slate.i.pharaoh curse.i.col.pharaoh curse.m.2.i.left2.facade.Ese80:0.bal.quartz"
struct ModifierChain {
    entries: Vec<ChainEntry>,
}

enum ChainEntry {
    ItemEquip {                    // i.[slot].[item] or i.[item]
        slot: Option<String>,      // "hat", "left", "right2", "topbot", "col", "all", "mid", "k", "self", "t"
        item: String,              // "slate", "pharaoh curse", "Blindfold", "Sarcophagus"
    },
    Facade {                       // i.[slot].facade.ID:value or .facade.ID:value
        slot: Option<String>,      // "left2", "topbot", "right2"
        facade_id: String,         // "Ese80:0", "Bal21:0", "dar2:95:50:30" (atomic format ID)
    },
    TriggerHpData {                // i.triggerhpdata.(content)
        content: ItemDefinition,   // the trigger definition (reuses ItemDefinition)
    },
    Sticker {                      // .sticker.ID or i.[slot].sticker.ID
        slot: Option<String>,
        sticker_id: String,        // "ritemx.dae9", "t.sniper" (atomic format ID)
    },
    Keyword(String),               // bare keywords: "exert", "jinx", "underdog", "wither"
    Multiplier(u8),                // .m.N
    Balance(String),               // .bal.TIER ("quartz", "gold", etc.)
    Part(u8),                      // .part.N
    Hp(u16),                       // .hp.N override
    Color(char),                   // .col.X
    ToggleTarget,                  // .togtarg
    ToggleFriendly,                // .togfri
    AllItem {                      // .allitem.[keyword]
        keyword: String,
    },
}

/// Common suffix properties found on most structural modifiers
struct StructuralSuffix {
    mn_name: Option<String>,   // .mn.NAME at depth 0
    doc: Option<String>,       // .doc.TEXT
    modtier: Option<i8>,       // .modtier.N
    part: Option<u8>,          // .part.N
    flags: Vec<String>,        // &hidden, &temporary, etc.
}

/// Action triggered by a selector option (@N prefix)
enum SelectorAction {
    ApplyModifier(ModifierPayload), // @N!m(PAYLOAD) — apply a modifier
    SetVariable(String),            // @N!vNAME — set a game variable (variable names are atomic)
    GiveItem(ItemDefinition),       // @N!i(ITEM_DEF) — give an item
    FlavorText(String),             // @N4TEXT — display text (rich text with [tags])
    Skip,                           // @Ns — skip/separator
}

/// Typed payload for modifier applications (the content inside @N!m(...))
/// Reverse-engineered from all 4 test mods — covers: party/add, diff, ch (variable overrides),
/// skip, itempool, game commands (name^value), and level-scoped modifiers (N.content).
enum ModifierPayload {
    PartyAdd {                      // party.(member) or add.(member)
        action: PartyAction,        // Party or Add
        member: PartyMember,        // reuses shared PartyMember type
    },
    DifficultySet {                 // diff.LEVEL
        level: String,              // "heaven", "Easy", "hard", "unfair", "brutal", "hell"
    },
    VariableOverrides {             // ch.ovVAR@4vVAR@4vVAR...
        overrides: Vec<VariableOverride>,
    },
    ItemPoolMod(ItemPool),          // itempool.(...) — reuses ItemPool type
    GameCommand {                   // name^value, keyword, slot assignment
        command: String,            // "Versatile", "Worse Items", "Wurst"
        value: Option<String>,      // "1", "/5", etc.
    },
    LevelScoped {                   // N.CONTENT — floor-scoped modifier
        level: u8,
        inner: Box<ModifierPayload>,
    },
    SkipAction {                    // skip&hidden&temporary
        flags: Vec<String>,
    },
    Compound {                      // ((PAYLOAD)&flags) — wrapped compound
        inner: Box<ModifierPayload>,
        flags: Vec<String>,
        mn_name: Option<String>,
    },
}

enum PartyAction { Party, Add }

struct VariableOverride {
    variable: String,               // e.g. "XV1", "YV1", "AlphaV1"
    action: VariableAction,         // Override or Set
}

enum VariableAction {
    Override,                       // ovNAME
    Set,                            // vNAME
}

/// Typed item definition (content inside itempool entries and @N!i(...) actions)
/// Structure: SLOT.(TEMPLATE.PROPS) or TEMPLATE.PROPS
struct ItemDefinition {
    slot: Option<String>,           // "right2", "topbot", "all", "k", "mid", "left", "rightmost"
    template: String,               // "hat.Ace", "hat.Fighter", "hat.Lost", "Gauntlet"
    slot_assignments: Vec<String>,  // i.SLOT entries (e.g., "i.right2", "i.k", "i.all")
    sticker: Option<String>,        // sticker.ID (e.g., "ritemx.dae9")
    facade: Option<String>,         // facade.ID:N (e.g., "The3:50", "Bal21:0")
    keywords: Vec<String>,          // bare keywords (e.g., "underdog", "treble")
    sd: Option<DiceFaces>,           // .sd. face override (reuses shared DiceFaces type)
    img_data: Option<String>,       // .img. sprite data
    name: Option<String>,           // .n. display name
    part: Option<u8>,               // .part.N for append
    alternatives: Vec<ItemDefinition>, // #-separated pool alternatives
}

/// A single option in a Selector, Difficulty, or EndScreen
struct SelectorOption {
    label: String,             // display text, may include [color] tags
    actions: Vec<SelectorAction>,
}
```

### 1. PartyConfig
Format: `=party.TEMPLATE.PROPS.n.NAME+(TEMPLATE.PROPS.n.NAME)+...(.mn.NAME)`
```rust
PartyConfig {
    host: PartyMember,                    // first member (after =party.)
    additional_members: Vec<PartyMember>, // +-separated at depth 0
    suffix: StructuralSuffix,
}
struct PartyMember {
    template: String,        // e.g. "Roulette", "Spellblade"
    name: Option<String>,    // .n. value
    color: Option<char>,     // .col. value
    tier: Option<u8>,        // .tier. value
    img_data: Option<String>,// .img. value
    doc: Option<String>,     // .doc. value
}
```

### 2. EventModifier
Format: `=ACTION&FLAG&FLAG...(.mn.NAME)`
```rust
EventModifier {
    action: String,           // primary action after = (e.g. "Skip All")
    flags: Vec<String>,       // &-separated flags (e.g. "e2.1.phi.1", "Hidden")
    suffix: StructuralSuffix,
}
```

### 3. Dialog
Format: `LEVEL.ph.4 RICH_TEXT.mn.NAME`
```rust
Dialog {
    level: u8,                // floor/order number
    text: String,             // rich text with [tag] formatting ([nh], [white], [cu], etc.)
    suffix: StructuralSuffix,
}
```

### 4. ArtCredits
Format: same as Dialog but classified by `.mn.Art Credits`
```rust
ArtCredits {
    level: u8,
    text: String,             // credits text with [tag] formatting
    is_wrapped: bool,         // wrapped in (( ))
    suffix: StructuralSuffix,
}
```

### 5. HeroPoolBase
Format: `heropool.CONTENT(.part.N)(.mn.NAME)(.doc.TEXT)(.modtier.N)`
```rust
HeroPoolBase {
    action: HeroPoolAction,
    suffix: StructuralSuffix,
}
enum HeroPoolAction {
    Clear,                    // heropool.o0.0.part.0 (clear vanilla pool)
    Define {
        entries: Vec<HeroPoolEntry>,
    },
}
enum HeroPoolEntry {
    VanillaClass(String),     // bare class name: "Enchanter", "Thief" (atomic identifier)
    InlineHero {              // (replica.TEMPLATE.PROPS.n.NAME) — same fields as PoolEntry::CustomHero
        template: String,
        abilitydata: Option<AbilityData>,
        sd: Option<DiceFaces>,
        hp: Option<u16>,
        color: Option<char>,
        tier: Option<u8>,
        img_data: Option<String>,
        facade: Option<String>,
        speech: Option<String>,
        name: String,
    },
}

/// Spell/ability definition inside .abilitydata.(...) blocks
/// Format: (TEMPLATE.sd.FACES[.img.ICON].n.SPELL_NAME)
struct AbilityData {
    template: String,          // spell template: "Fey", "Lost", "Fighter", etc.
    sd: DiceFaces,             // face data (reuses shared DiceFaces type)
    img_data: Option<String>,  // spell icon (.img. encoding)
    name: String,              // spell display name (.n. value)
}
```

### 6. LevelUpAction
Format: `FLAG&FLAG&ph.!m(FLAG&FLAG&ACTION)`
```rust
LevelUpAction {
    outer_flags: Vec<String>, // ["hidden", "temporary"]
    inner_flags: Vec<String>, // ["hidden", "temporary"]
    action: String,           // "level up"
}
```

### 7. ItemPool (most complex — 4 sub-formats)
```rust
ItemPool {
    variant: ItemPoolVariant,
    trigger: Option<HeroTrigger>, // if hero-triggered (ph.b wrapper)
    suffix: StructuralSuffix,
}
enum ItemPoolVariant {
    Clear {
        target: String,       // existing pool name to clear ("Void", "uy")
    },
    ItemDefinitions {
        items: Vec<ItemEntry>,
    },
    ItemRenames {
        renames: Vec<ItemRename>,
    },
}
struct ItemEntry {
    definition: ItemDefinition, // typed item definition (reuses shared ItemDefinition type)
    name: String,              // .n. value (display name for the item)
    tier: Option<i8>,          // .tier. value
    img_data: Option<String>,  // .img. value (item icon sprite)
}
struct ItemRename {
    original_name: String,     // "Antivenom", "Bone Charm"
    new_name: String,          // .n. value
    tier: Option<i8>,
    img_data: Option<String>,
}
struct HeroTrigger {
    hero_name: String,         // from ph.b[NAME]
    condition_value: u8,       // the ;N; number
    wrapper_mn: Option<String>,// .mn. on the wrapper
}
```

### 8. Selector
Format: `LEVEL.ph.sPROMPT@1LABEL@2!m(PAYLOAD)@1LABEL...(.mn.NAME)`
```rust
Selector {
    level: u8,
    prompt: String,                   // text before first @1
    options: Vec<SelectorOption>,     // shared SelectorOption type
    is_wrapped: bool,                 // wrapped in (( ))
    chained: Vec<Selector>,          // &hidden,-chained selectors in same modifier
    suffix: StructuralSuffix,
}
```

### 9. Difficulty
Format: same grammar as Selector but options contain `diff.LEVEL` payloads
```rust
Difficulty {
    level: u8,
    prompt: String,
    options: Vec<DifficultyOption>,
    is_wrapped: bool,
    suffix: StructuralSuffix,
}
struct DifficultyOption {
    label: String,                    // "[white]Heaven", "[green]Easy"
    difficulty_key: Option<String>,   // "heaven", "Easy", "hard", "unfair", "brutal", "hell"
    actions: Vec<SelectorAction>,     // shared SelectorAction type
}
```

### 10. GenSelect
Format: `LEVEL.ph.cTYPE#COUNT;OPTION@3OPTION...;PROMPT(.mn.NAME)`
```rust
GenSelect {
    level: u8,
    choice_type: String,              // "Number"
    choice_count: u8,                 // from #N
    options: Vec<GenSelectOption>,
    prompt: String,                   // text after final ; separator
    is_wrapped: bool,
    suffix: StructuralSuffix,
}
struct GenSelectOption {
    variables: Vec<String>,           // vNAME entries (variable names are atomic identifiers)
    modifier_payload: Option<ModifierPayload>, // m(PAYLOAD) — reuses shared ModifierPayload type
    label: Option<String>,            // .mn. within option
    doc: Option<String>,
}
```

### 11. EndScreen
Format: same as Selector but level is always 20 (final floor)
```rust
EndScreen {
    level: u8,                        // always 20
    text: String,                     // farewell/credits text (prompt)
    options: Vec<SelectorOption>,     // typically trivial ("Have Fun!")
    suffix: StructuralSuffix,
}
```
Classification: level 20 + `.ph.s` + no `diff.` keyword.

### 12. BossModifier
Format: `LEVEL.FLAG&LEVEL.FLAG&...(.mn.NAME)`
```rust
BossModifier {
    rules: Vec<BossRule>,
    suffix: StructuralSuffix,
}
struct BossRule {
    level: u8,                // floor number (4, 8, 12, 16, 20)
    flag: BossFlag,
}
enum BossFlag {
    NoFlee,
    Horde,
    Custom(String),           // for flags not yet catalogued
}
```

### 13. PoolReplacement
Format: `((heropool.ENTRY+ENTRY+...)&FLAGS)(.doc.TEXT)(.mn.NAME)`
```rust
PoolReplacement {
    entries: Vec<PoolEntry>,
    inner_flags: Vec<String>,         // ["Hidden"]
    suffix: StructuralSuffix,
}
enum PoolEntry {
    VanillaRef(String),               // bare class name: "Thief", "Scoundrel"
    CustomHero {
        template: String,             // "replica.Reflection", "replica.Spade"
        abilitydata: Option<AbilityData>, // .abilitydata.(...) — reuses shared AbilityData type
        sd: Option<DiceFaces>,         // .sd. face data (reuses shared DiceFaces type)
        hp: Option<u16>,              // .hp. health
        color: Option<char>,          // .col. color
        tier: Option<u8>,             // .tier. tier
        img_data: Option<String>,     // .img. sprite data
        facade: Option<String>,       // .facade. appearance
        speech: Option<String>,       // .speech. battle cries
        name: String,                 // .n. display name
    },
}
```

### Schema completeness rule
- If an `Unknown` type appears during parsing, it is a **parser gap** to investigate and resolve — not an accepted fallback. The parser must fully handle every structural type the game emits.
- No `String` field may store a multi-property blob. If a field contains `.`-separated properties, those properties must be parsed into their own fields.
- **Acceptable `String` fields** (these are atomic values, not blobs):
  - Display text: Dialog.text, prompt strings, labels, rich text with `[tag]` formatting
  - Identifiers: template names, variable names, class names, Pokemon names
  - Encoded data: `.img.` sprite encodings only
  - Sticker/facade IDs: short format identifiers (e.g., "ritemx.dae9", "The3:50")
  - Event config keys: game-engine parameters (e.g., "e2.1.phi.1")
- **NOT acceptable as `String`**:
  - `.sd.` face data → use `DiceFaces { faces: Vec<DiceFace> }` (validates face count, FaceID, pips)
  - `item_modifiers` → use `ModifierChain` (typed equipment, facades, triggers)
  - `modifier_chain` → use `ModifierChain` (same type)
  - Any field containing multiple `.`-separated properties, `&`-separated flags, `+`-separated items, or nested parenthesized content
- All types must derive `JsonSchema` (schemars) so the JSON Schema is authorable and validatable by external tools.

**Tests FIRST**:
```rust
fn emit_party_config()             // PartyConfig fields -> valid =party. string with members
fn emit_dialog()                   // Dialog fields -> valid LEVEL.ph.4 TEXT.mn.NAME string
fn emit_art_credits()              // ArtCredits fields -> valid credits dialog
fn emit_selector()                 // Selector with options -> valid @1label@2!m(payload) string
fn emit_selector_chained()         // Multiple chained selectors in one modifier
fn emit_item_pool_clear()          // ItemPool::Clear -> valid itempool.TARGET.part.0 string
fn emit_item_pool_definitions()    // ItemPool::ItemDefinitions -> valid (MOD).n.NAME.tier.N+... string
fn emit_item_pool_renames()        // ItemPool::ItemRenames -> valid OLD.n.NEW.img.X+... string
fn emit_item_pool_hero_triggered() // ItemPool with HeroTrigger wrapper -> valid ph.b[NAME];N;!mitempool... string
fn emit_boss_modifier()            // BossModifier with rules -> valid LEVEL.FLAG&LEVEL.FLAG string
fn emit_gen_select()               // GenSelect with options -> valid ph.cTYPE#N;OPTION@3OPTION;PROMPT string
fn emit_difficulty()               // Difficulty with options -> valid diff. payloads in selector
fn emit_level_up_action()          // LevelUpAction -> valid hidden&temporary&ph.!m(hidden&temporary&level up)
fn emit_hero_pool_base_clear()     // HeroPoolBase::Clear -> valid heropool.o0.0.part.0 string
fn emit_hero_pool_base_define()    // HeroPoolBase::Define -> valid heropool.ENTRY+ENTRY string
fn emit_pool_replacement()         // PoolReplacement -> valid ((heropool.ENTRY+ENTRY))&Hidden string
fn emit_end_screen()               // EndScreen -> valid 20.ph.s TEXT @1option string
fn emit_event_modifier()           // EventModifier -> valid =ACTION&FLAG string
fn structural_round_trip_all_types()  // extract -> emit -> re-extract for each structural type
fn structural_modifier_has_no_raw()   // StructuralModifier struct has no raw: String field
```

**Implementation**: Remove `raw: String` from StructuralModifier. Remove `StructuralModifier::new_raw()` convenience constructor. Replace all StructuralContent variants with the typed schemas defined above. Add shared types (StructuralSuffix, SelectorOption, SelectorAction, PartyMember, etc.). Implement `emit()` dispatch for each variant in structural_emitter.rs. Update structural_parser to populate the new typed variants.

**Files** (4 modified):
1. `compiler/src/ir/mod.rs` -- remove `raw: String` from StructuralModifier, remove `new_raw()`, enrich StructuralContent variants
2. `compiler/src/extractor/structural_parser.rs` -- populate richer content variants (no more Raw fallback for typed structurals)
3. `compiler/src/builder/structural_emitter.rs` -- implement field-based emit() for each StructuralContent variant
4. `compiler/src/extractor/mod.rs` -- update `make_structural()` to not pass raw (or adapt to new StructuralModifier constructor)

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all pass
- [ ] No `raw` field anywhere in StructuralModifier
- [ ] Round-trip: extract(build(extract(sliceymon.txt))) == extract(sliceymon.txt) for structural modifiers
- [ ] All 4 test mods produce valid output through type-based structural emission

**No Opaque fallback**: Every structural type must be fully parsed into fields and emitted from those fields. There is no `Opaque { text: String }` variant. If a structural type is difficult to parse, that difficulty must be resolved -- not worked around with an opaque passthrough. The game has a parser for these types; we are reverse-engineering it, and every type must be fully understood.

---

## Chunk 7: CRUD API [PARALLEL GROUP B]

**Scope**: Add library functions for individual item operations on ModIR.
**Dependencies**: Chunk 6 (all raw removed, structural emission works)
**Parallel with**: Chunks 8, 9
**Read first**: `compiler/src/ir/mod.rs` (ModIR, Hero, ReplicaItem types), `compiler/src/ir/merge.rs` (existing merge logic pattern), `compiler/src/error.rs` (CompilerError variants)

**Tests FIRST**:
```rust
fn add_hero_to_ir()                   // Empty IR + hero -> IR has 1 hero
fn add_hero_duplicate_color_errors()  // IR has color 'a' -> adding another color 'a' = CompilerError
fn remove_hero_by_name()              // IR with "Gible" -> remove "Gible" -> gone, len decreased by 1
fn remove_hero_not_found_errors()     // Remove "NonExistent" -> CompilerError
fn update_hero_by_name()              // IR has "Gible" -> update with modified "Gible" -> replaced in-place
fn add_replica_item_duplicate_name_errors() // "Charmander" in heroes.mn_name -> add_replica_item("Charmander") = error
fn remove_replica_item_by_name()          // IR with replica item "Pikachu" -> remove -> gone
fn add_monster()                      // Empty IR + monster -> IR has 1 monster
fn remove_monster()                   // IR with monster "Wooper" -> remove -> gone
fn add_boss()                         // Empty IR + boss -> IR has 1 boss
fn remove_boss()                      // IR with boss "Mewtwo" -> remove -> gone
fn crud_preserves_other_types()       // Adding a hero doesn't touch replica_items/monsters/bosses
```

**Implementation**:
- New `ir/ops.rs`: Add methods on ModIR (impl block in separate file for modularity):
  ```rust
  impl ModIR {
      pub fn add_hero(&mut self, hero: Hero) -> Result<(), CompilerError>
      pub fn remove_hero(&mut self, mn_name: &str) -> Result<(), CompilerError>
      pub fn update_hero(&mut self, hero: Hero) -> Result<(), CompilerError>
      pub fn add_replica_item(&mut self, item: ReplicaItem) -> Result<(), CompilerError>
      pub fn remove_replica_item(&mut self, name: &str) -> Result<(), CompilerError>
      // ... same for Monster, Boss
  }
  ```
- Cross-category duplicate check: Hero.mn_name vs ReplicaItem.name vs Monster.name vs Boss.name
- Color uniqueness check: hero.color must not match any existing hero.color (excluding the hero being updated)
- Add new CompilerError variants: `DuplicatePokemon { name, existing_category, new_category }`, `DuplicateColor { color, existing_hero }`, `NotFound { type_name, key }`
- Expose via `lib.rs`

**Files** (4 modified/new):
1. `compiler/src/ir/ops.rs` -- NEW: CRUD methods on ModIR
2. `compiler/src/ir/mod.rs` -- add `pub mod ops;`
3. `compiler/src/error.rs` -- add DuplicatePokemon, DuplicateColor, NotFound variants
4. `compiler/src/lib.rs` -- re-export CRUD methods

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all CRUD tests pass
- [ ] Adding a hero with taken color returns error (not panic)
- [ ] Cross-category duplicate prevention works for all type combinations

---

## Chunk 8: Single-Item Build + Validate [PARALLEL GROUP B]

**Scope**: Library functions to build/validate a single modifier in isolation.
**Dependencies**: Chunk 6 (all raw removed, emitters work field-based)
**Parallel with**: Chunks 7, 9
**Read first**: `compiler/src/lib.rs` (current public API), `compiler/src/builder/hero_emitter.rs` (emit signature), `compiler/src/validator.rs` (validate signature and Finding type)

**Tests FIRST**:
```rust
fn build_single_hero()                // One hero -> valid modifier string with balanced parens
fn build_single_replica_item()        // One replica item -> valid modifier string
fn build_single_monster()             // One monster -> valid modifier string
fn build_single_boss()                // One boss -> valid modifier string with fight units
fn validate_single_hero_ok()          // Valid hero -> ValidationReport with 0 errors
fn validate_single_hero_bad_parens()  // Hero with malformed sd -> paren/face error
fn validate_single_hero_bad_faces()   // Hero with 7 faces -> W001 warning
fn validate_single_replica_item_ok()  // Valid replica item -> 0 errors
fn validate_preserves_context()       // Hero with color 'a' validated against IR with existing color 'a' -> error
```

**Implementation**:
- `lib.rs`: Add public functions (thin wrappers around existing emitters/validator):
  ```rust
  pub fn build_hero(hero: &Hero, sprites: &HashMap<String, String>) -> Result<String, CompilerError>
  pub fn build_replica_item(item: &ReplicaItem) -> Result<String, CompilerError>
  pub fn build_monster(monster: &Monster) -> Result<String, CompilerError>
  pub fn build_boss(boss: &Boss) -> Result<String, CompilerError>
  pub fn validate_hero(hero: &Hero) -> ValidationReport
  pub fn validate_hero_in_context(hero: &Hero, ir: &ModIR) -> ValidationReport
  ```
- Single-item validate: emit the item, then run existing validation rules on the emitted string
- Context-aware validate: check color conflict and Pokemon duplicate against provided IR

**Files** (2 modified):
1. `compiler/src/lib.rs` -- add single-item build/validate public functions
2. `compiler/src/validator.rs` -- add validate_hero(), validate_hero_in_context() functions that operate on IR types directly

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all single-item tests pass
- [ ] build_hero produces output that passes validate()
- [ ] validate_hero_in_context detects color conflicts

---

## Chunk 9: Derived Structural Modifiers [PARALLEL GROUP B]

**Scope**: Auto-generate structural modifiers that are derived from content (character selection, hero pools).
**Dependencies**: Chunk 6 (structural emission works)
**Parallel with**: Chunks 7, 8
**Read first**: `compiler/src/builder/mod.rs` (assembly order), `compiler/src/ir/mod.rs` (StructuralType::Selector, StructuralType::HeroPoolBase), `compiler/src/extractor/structural_parser.rs` (current parsing of these types)

**Tests FIRST**:
```rust
fn generate_char_selection_from_heroes()
    // 3 heroes at colors 'a','b','c' -> Selector with 3 @N options matching hero mn_names

fn generate_char_selection_alphabetical()
    // Heroes with colors 'c','a','b' -> selection sorted by color alphabetically

fn generate_hero_pool_base_from_heroes()
    // 3 heroes with internal_names ["x","y","z"] -> HeroPoolBase with hero_refs ["x","y","z"]

fn char_selection_updates_on_add_hero()
    // Add hero at color 'd' -> char selection now has 4 entries

fn char_selection_updates_on_remove_hero()
    // Remove hero at color 'b' -> char selection now has 2 entries

fn builder_auto_generates_derived_structurals()
    // IR with 3 heroes, no explicit Selector or HeroPoolBase in structural vec
    // build() output includes auto-generated char selection and hero pool base
```

**Implementation**:
- New `builder/derived.rs`: Functions that generate structural modifiers from IR content:
  ```rust
  pub fn generate_char_selection(heroes: &[Hero]) -> StructuralModifier
  pub fn generate_hero_pool_base(heroes: &[Hero]) -> StructuralModifier
  ```
- `builder/mod.rs`: Call these during assembly if no explicit structural of that type exists in the IR
- Mark derived structurals with a flag so they are not serialized to IR JSON (they are computed, not stored)

**Files** (3 modified/new):
1. `compiler/src/builder/derived.rs` -- NEW: generate_char_selection(), generate_hero_pool_base()
2. `compiler/src/builder/mod.rs` -- add `pub mod derived;`, call derived generators during assembly
3. `compiler/src/ir/mod.rs` -- add `derived: bool` flag to StructuralModifier (skip_serializing_if true)

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all derived structural tests pass
- [ ] build(extract(sliceymon.txt)) produces output with char selection matching the extracted heroes
- [ ] Derived structurals not included in JSON serialization of IR

---

## Chunk 10: Semantic Validation

**Scope**: Deep validation for hand-authored content -- face IDs, templates, cross-category duplicates.
**Dependencies**: Chunks 7, 8, 9 (CRUD + single-item validate + derived structurals complete)
**Read first**: `compiler/src/validator.rs` (existing rules E001-E016, W001-W007), `SLICEYMON_AUDIT.md` (Face ID tables per template)

**Tests FIRST**:
```rust
fn validate_face_id_for_template()
    // Face 42 on template "Fey" -> E017 error (42 not in Fey's approved list)
    // Face 15 on template "Fey" -> no error (15 is valid for Fey)

fn validate_template_exists()
    // template "NonExistent" -> E018 error
    // template "Lost" -> no error (Lost is a known template)

fn validate_color_uniqueness()
    // Two heroes with color 'a' -> E019 error naming both heroes

fn validate_pokemon_uniqueness_across_categories()
    // "Charmander" in heroes AND replica_items -> E020 error

fn validate_hp_range_per_tier()
    // T1 hero with HP 50 -> W008 warning (expected 3-6 for T1)

fn validate_sd_face_count()
    // HeroBlock with .sd. containing 7 colon-separated faces -> W009 warning (expected 6)

fn validate_spell_face_ids()
    // abilitydata with face 42 on Fey spell template -> E021 error

fn validate_replica_item_template()
    // Replica item with template "Statue" -> W010 warning (unexpected template for simple replica items)

fn validate_replica_item_with_ability_hp()
    // Replica item with ability and HP 30 -> W011 warning (expected higher HP for items with abilities)
```

**Implementation**:
- `validator.rs`: Add semantic validation rules operating on IR types (not raw strings):
  - E017: Face ID validation table per template (from SLICEYMON_AUDIT -- hardcoded approved face ID sets)
  - E018: Template existence check (known template list hardcoded from game data)
  - E019: Color uniqueness across heroes (iterate heroes, check for duplicate colors)
  - E020: Name uniqueness across hero.mn_name, replica_item.name, monster.name, boss.name
  - E021: Spell face ID validation (abilitydata face IDs checked against spell template's approved set)
  - W008-W011: Per-tier HP budget, face count, replica item template, replica item HP
- These rules fire on `validate_hero()`, `validate_hero_in_context()`, and during `validate()` on the full IR
- New rule IDs: E017-E021 for semantic errors, W008-W011 for semantic warnings

**Files** (1 modified):
1. `compiler/src/validator.rs` -- add semantic validation functions and face ID tables

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all semantic validation tests pass
- [ ] Existing E001-E016 rules still work unchanged
- [ ] validate(sliceymon.txt) produces 0 semantic errors (it's a known-good mod)

---

## Chunk 11: Structured Errors [PARALLEL GROUP C]

**Scope**: Upgrade error reporting from flat strings to structured objects with field paths and fix suggestions.
**Dependencies**: Chunk 10 (semantic validation complete)
**Parallel with**: Chunk 12
**Read first**: `compiler/src/validator.rs` (current Finding struct at lines 54-65 -- has rule_id, modifier_index, modifier_name, position, context, message), `compiler/src/error.rs` (CompilerError variants)

**Tests FIRST**:
```rust
fn finding_has_field_path()
    // Validate hero with bad sd -> Finding includes field_path "heroes[0].blocks[2].sd"

fn finding_has_suggestion()
    // E017 face ID error -> suggestion: "Valid face IDs for Fey: 15, 32, 34, ..."

fn finding_serializes_to_json()
    // Finding -> JSON with keys: rule_id, severity, message, field_path, suggestion

fn finding_has_severity_enum()
    // Finding.severity is Severity::Error or Severity::Warning (not inferred from rule_id prefix)

fn validation_report_groups_by_severity()
    // Report with errors + warnings -> report.errors and report.warnings correctly separated
```

**Implementation**:
- `validator.rs`: Extend `Finding` struct (currently has 6 fields, add 2 new + 1 renamed):
  ```rust
  pub struct Finding {
      pub rule_id: String,
      pub severity: Severity,             // NEW: explicit enum (currently inferred from errors/warnings vec placement)
      pub message: String,
      pub field_path: Option<String>,     // NEW: "heroes[3].blocks[2].sd"
      pub suggestion: Option<String>,     // NEW: "Valid face IDs for Fey: ..."
      pub modifier_index: Option<usize>,  // existing
      pub modifier_name: Option<String>,  // existing
      pub position: Option<usize>,        // existing
      pub context: Option<String>,        // existing
  }
  ```
- Add `Severity` enum: `pub enum Severity { Error, Warning, Info }`
- Update all existing validation rule sites (E001-E021, W001-W011) to populate field_path and suggestion where applicable
- Ensure Finding serializes cleanly to JSON for web app consumption

**Files** (2 modified):
1. `compiler/src/validator.rs` -- extend Finding struct, add Severity enum, update all rule call sites
2. `compiler/src/lib.rs` -- re-export Severity enum

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all tests pass
- [ ] Finding JSON output includes field_path and suggestion when available
- [ ] Existing validation behavior unchanged (no regressions)

---

## Chunk 12: Replace Patch with Overlay + Provenance [PARALLEL GROUP C]

**Scope**: Replace raw text-surgery `patch` command with IR-based `overlay`. Add provenance tracking to IR (base vs custom).
**Dependencies**: Chunk 7 (CRUD ops create ir/ops.rs), Chunk 10 (semantic validation complete)
**Parallel with**: Chunk 11
**Read first**: `compiler/src/main.rs` (current Patch command at line 46), `compiler/src/ir/merge.rs` (existing merge logic), `compiler/src/ir/mod.rs` (all content types that need source field), `compiler/src/ir/ops.rs` (CRUD methods to add Source::Custom)

**Tests FIRST**:
```rust
fn overlay_replaces_hero_by_name()    // base has "Gible", overlay has "Gible" -> merged IR has overlay version
fn overlay_adds_new_hero()            // base has 3 heroes, overlay adds 1 -> merged has 4
fn overlay_accepts_json()             // overlay IR loaded from JSON file -> merges correctly
fn overlay_handles_all_types()        // overlay with hero+replica_item+monster+boss -> all merged
fn provenance_tracks_base_vs_custom()
    // Extract base mod -> all items have source == Source::Base
    // Add hero via CRUD -> new hero has source == Source::Custom
    // Merge overlay -> overlay items have source == Source::Overlay
fn provenance_survives_serialization()
    // IR with Source::Custom -> JSON -> deserialize -> source still Custom
fn cli_overlay_command()              // textmod-compiler overlay base.txt --with overlay.json --output out.txt
```

**Implementation**:
- `ir/mod.rs`: Add provenance enum and field:
  ```rust
  #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
  pub enum Source { #[default] Base, Custom, Overlay }
  ```
  Add `#[serde(default)] pub source: Source` to Hero, ReplicaItem, Monster, Boss, StructuralModifier
- `extractor/mod.rs`: Extracted items get `Source::Base` (default, no code change needed)
- `ir/ops.rs` (from Chunk 7): CRUD operations set `Source::Custom`
- `ir/merge.rs`: Overlay items get `Source::Overlay` before merging
- `main.rs`: Replace `Patch` command with `Overlay`:
  ```
  Overlay { base: PathBuf, with: PathBuf, output: PathBuf }
  ```
  Workflow: extract base -> load overlay IR from JSON -> merge -> build
- Remove old `patch_textmod()` function and Patch command entirely

**Files** (5 modified):
1. `compiler/src/ir/mod.rs` -- add Source enum, add source field to all content types
2. `compiler/src/ir/merge.rs` -- mark overlay items as Source::Overlay before merge
3. `compiler/src/ir/ops.rs` -- mark CRUD-added items as Source::Custom
4. `compiler/src/main.rs` -- replace Patch command with Overlay command, remove patch_textmod()
5. `compiler/src/lib.rs` -- add overlay convenience function

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all overlay and provenance tests pass
- [ ] Old Patch command removed, Overlay command works end-to-end
- [ ] Source survives JSON round-trip

---

## Chunk 13: JSON Schema + Full Integration Tests

**Scope**: Generate JSON Schema from IR types. Full round-trip testing across all 4 mods. End-to-end workflow test simulating Sliceymon+ build.
**Dependencies**: Chunks 11, 12 (structured errors + overlay complete)
**Read first**: `compiler/Cargo.toml` (current dependencies), `compiler/src/ir/mod.rs` (all IR types), `working-mods/` directory (4 test mods)

**Tests FIRST**:
```rust
fn schema_validates_hand_authored_hero()  // JSON Schema accepts valid hero JSON
fn build_from_partial_ir()                // IR with only heroes -> builds just heroes section
fn overlay_end_to_end()                   // base textmod + overlay JSON -> valid combined mod

fn round_trip_sliceymon_all_types()       // extract -> build -> re-extract: all fields identical
fn round_trip_pansaer_all_types()         // same for pansaer
fn round_trip_punpuns_all_types()         // same for punpuns
fn round_trip_community_all_types()       // same for community

fn rebuilt_sliceymon_validates()           // build(extract(sliceymon.txt)) -> validate -> 0 errors
fn rebuilt_pansaer_validates()             // same for pansaer

fn sliceymon_plus_workflow()
    // Extract base sliceymon -> add 22 heroes via JSON overlay
    // Auto-generated char selection includes all hero colors
    // Validate -> 0 errors
    // Build -> valid textmod string
```

**Implementation**:
- `Cargo.toml`: Add `schemars = "0.8"` to dependencies (WASM-compatible)
- `ir/mod.rs`: Add `#[derive(JsonSchema)]` to ALL IR types. This includes the top-level types (ModIR, Hero, HeroBlock, ReplicaItem, Monster, Boss, BossFightUnit, StructuralModifier, StructuralContent) AND every shared/nested type: DiceFaces, DiceFace (Chunks 2-4), ModifierChain, ChainEntry (Chunks 2-4), AbilityData (Chunks 2-3), StructuralSuffix, SelectorOption, SelectorAction, ModifierPayload, PartyAction, VariableOverride, VariableAction, ItemDefinition, PartyMember, HeroPoolAction, HeroPoolEntry, ItemPoolVariant, ItemEntry, ItemRename, HeroTrigger, DifficultyOption, GenSelectOption, BossRule, BossFlag, PoolEntry (Chunk 6). Every type in the IR must appear in the generated JSON Schema.
- `main.rs`: Add `Schema { output: Option<PathBuf> }` command that calls `schemars::schema_for::<ModIR>()`
- Create `compiler/examples/hero_charmander.json` and `compiler/examples/overlay_sample.json` as reference documentation
- Rewrite `tests/roundtrip_tests.rs` -- field-by-field IR comparison, no raw fields anywhere

**Files** (5 modified/new):
1. `compiler/Cargo.toml` -- add schemars dependency
2. `compiler/src/ir/mod.rs` -- add `#[derive(JsonSchema)]` to all types
3. `compiler/src/main.rs` -- add Schema subcommand
4. `compiler/examples/hero_charmander.json` -- NEW: example hero JSON for documentation
5. `compiler/tests/roundtrip_tests.rs` -- rewrite: field comparison across all 4 mods

**Verification**:
- [ ] `~/.cargo/bin/cargo test` -- all pass (including new round-trip tests)
- [ ] `~/.cargo/bin/cargo clippy -- -D warnings` -- clean
- [ ] `textmod-compiler schema --output schema.json` -- produces valid JSON Schema
- [ ] `textmod-compiler validate working-mods/sliceymon.txt --round-trip` -- passes
- [ ] `textmod-compiler validate working-mods/pansaer.txt --round-trip` -- passes
- [ ] `textmod-compiler validate working-mods/punpuns.txt --round-trip` -- passes
- [ ] `textmod-compiler overlay working-mods/sliceymon.txt --with examples/overlay_sample.json --output /tmp/test.txt` -- produces valid output

**If blocked**: If schemars cannot derive JsonSchema for certain types (e.g., enum variants with complex data), add manual JsonSchema implementations for those types. The schema is useful but not blocking -- the round-trip and integration tests are the critical verification for this chunk.

---

## Critical Files Summary

| File | Chunks | What Changes |
|------|--------|-------------|
| `Cargo.toml` | 13 | Add schemars |
| `ir/mod.rs` | 1-7,9,12,13 | img_data, remove raw, CRUD module, derived flag, provenance, JsonSchema |
| `ir/ops.rs` | 7,12 | NEW (7) -- CRUD operations; updated (12) -- Source::Custom on CRUD adds |
| `ir/merge.rs` | 5,12 | Remove original_modifiers, provenance |
| `util.rs` | 1 | extract_img_data() |
| `lib.rs` | 7,8,11,12 | Public API: CRUD, single-item build/validate, re-exports |
| `error.rs` | 7 | CRUD error variants (DuplicatePokemon, DuplicateColor, NotFound) |
| `validator.rs` | 8,10,11 | Single-item validate, semantic rules, structured findings |
| `extractor/hero_parser.rs` | 1,2 | img_data, remove raw |
| `extractor/replica_item_parser.rs` | 1,3 | img_data, remove raw |
| `extractor/monster_parser.rs` | 1,4 | img_data, remove raw |
| `extractor/boss_parser.rs` | 4 | Remove raw, clean up |
| `extractor/structural_parser.rs` | 6 | Full content parsing |
| `extractor/mod.rs` | 5,6 | Remove original_modifiers, update make_structural() |
| `builder/hero_emitter.rs` | 2 | Remove raw, use img_data, emit hue/facades/items_inside |
| `builder/replica_item_emitter.rs` | 3 | Remove raw, emit img_data |
| `builder/monster_emitter.rs` | 4 | Remove raw, emit img_data |
| `builder/boss_emitter.rs` | 4 | Remove raw |
| `builder/structural_emitter.rs` | 6 | Full field-based emission |
| `builder/mod.rs` | 5,9 | Remove original_modifiers passthrough, derived structurals |
| `builder/derived.rs` | 9 | NEW -- auto-generate char selection, hero pools |
| `main.rs` | 12,13 | Overlay command, Schema command |
