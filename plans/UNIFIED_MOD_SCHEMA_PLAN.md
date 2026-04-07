# Plan: Unified Mod Schema

## Context

The compiler's IR is currently hero-centric. Only heroes have deep parsing and structured emission. Captures, legendaries, monsters, bosses, and all 13 structural types either shallow-parse a few fields or rely entirely on raw passthrough. This means:

- **Path B (hand-authored IR) only works for heroes** — all other types need `raw` populated
- **Validation is hero-only** — content rules (face format, HP range, face IDs) don't fire on captures/monsters/bosses
- **Non-sliceymon mods get second-class treatment** — grouped heroes (pansaer/punpuns/community) fall to raw passthrough
- **The compiler can't build a mod from scratch** — only heroes can be reconstructed from fields

**Goal**: Make the IR a complete, authorable API surface. Every modifier type gets deep parsing, structured emission, and content validation. Any mod (sliceymon, pansaer, punpuns, community, or future) can be fully extracted to IR and rebuilt. Raw passthrough becomes a fallback, not the primary path.

## Out of Scope

- New CLI commands beyond existing extract/build/validate/patch
- WASM/browser integration (library stays WASM-safe but no wasm-bindgen yet)
- Sprite pipeline changes (sprites.json format unchanged)
- Game balance validation (pip budgets, tier progression, keyword budgets)
- New modifier type discovery (only handle types already classified)
- Changes to splitter.rs (comma-at-depth-0 splitting is correct)
- Sprite resolution for non-hero types (captures/monsters/bosses have inline `.img.` data, no sprite lookup needed)
- Structural content-level validation (item pool balance, party config correctness, dialog grammar are game-design concerns)
- Regex crate (continue using manual string parsing with `str` methods)
- Performance optimization (mods process in <1s — not needed)

## Current State → Target State

| Type | Parse Depth Now | Emitter Now | Parse Target | Emitter Target |
|------|----------------|-------------|--------------|----------------|
| **Hero** | Deep (471 LOC) | Sliceymon only | Deep + Grouped | Sliceymon + Grouped |
| **Capture** | Shallow (4 of 10 non-raw fields) | None | Full (all fields) | New capture_emitter.rs |
| **Legendary** | Shallow (3 of 8 non-raw fields) | None | Full (all fields) | New (in capture_emitter.rs) |
| **Monster** | Medium (6 of 8 non-raw fields) | None | Full (all fields) | New monster_emitter.rs |
| **Boss** | Shallow (3 of 3 current fields) | None | Full (all fields) | New boss_emitter.rs |
| **Structural** | None (raw only) | Raw passthrough | Key properties extracted | Structured + raw fallback |
| **Validation** | Heroes only (E004-E012) | — | All content types | — |

## What Gets Built

### 1. IR Type Changes (`src/ir/mod.rs`)

#### Hero (rename + expand)

**Rename `HeroTier` → `HeroBlock`**, add `color`:
```rust
pub struct HeroBlock {
    pub template: String,
    pub tier: Option<u8>,
    pub hp: u16,
    pub sd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,     // NEW: per-block color from .col.
    pub sprite_name: String,
    pub speech: String,
    pub name: String,
    // all existing optional fields unchanged (doc, abilitydata, triggerhpdata, hue,
    // modifier_chain, facades, items_inside, items_outside)
}
```

**Add `HeroFormat`**, rename `Hero.tiers` → `Hero.blocks`:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum HeroFormat { Sliceymon, Grouped, #[default] Unknown }
// Note: uses #[derive(Default)] on the enum — requires Rust 1.62+. No manual impl needed.

pub struct Hero {
    pub internal_name: String,
    pub mn_name: String,
    pub color: char,
    #[serde(default)]
    pub format: HeroFormat,         // NEW
    pub blocks: Vec<HeroBlock>,     // RENAMED from tiers
    pub removed: bool,
    pub raw: Option<String>,
}
```

#### Capture (populate existing fields + add color)

The Capture struct already has `ball_tier`, `hp`, `sprite_name`, `item_modifiers`, `sticker`, and `toggle_flags` fields, but the current parser leaves them as None/empty. This plan populates them with real extraction logic. Only `color` is a genuinely new field.

```rust
pub struct Capture {
    pub pokemon: String,
    pub ball_name: String,
    pub ball_tier: Option<u8>,       // POPULATE: extract from .tier. in wrapper (currently None)
    pub template: String,
    pub hp: Option<u16>,             // POPULATE: extract from .hp. in replica content (currently None)
    pub sd: String,
    pub sprite_name: String,         // POPULATE: extract from .n. = pokemon name (currently empty String)
    pub color: Option<char>,         // NEW: from .col. if present
    pub item_modifiers: Option<String>,  // POPULATE: extract .i. chains (currently None)
    pub sticker: Option<String>,     // POPULATE: extract .sticker. data (currently None)
    pub toggle_flags: Option<String>, // POPULATE: extract #tog flags (currently None)
    pub raw: Option<String>,
}
```

#### Legendary (populate existing fields + add color/doc/speech)

The Legendary struct already has `summoning_item`, `hp`, `sprite_name`, `abilitydata`, and `item_modifiers` fields, but the current parser leaves most as empty/None. This plan populates them. Fields `color`, `doc`, and `speech` are genuinely new.

```rust
pub struct Legendary {
    pub pokemon: String,
    pub summoning_item: String,      // POPULATE: extract item name from wrapper (.n. of outer item) (currently empty)
    pub template: String,
    pub hp: Option<u16>,             // POPULATE: extract from .hp. in replica (currently None)
    pub sd: String,
    pub sprite_name: String,         // POPULATE: from .n. inside replica (currently empty)
    pub color: Option<char>,         // NEW
    pub doc: Option<String>,         // NEW: from .doc.
    pub speech: Option<String>,      // NEW: from .speech.
    pub abilitydata: Option<String>, // POPULATE: extract from .abilitydata.(...) (currently None)
    pub item_modifiers: Option<String>, // POPULATE: extract .i. chains (currently None)
    pub raw: Option<String>,
}
```

#### Monster (populate existing fields + add color/modifier_chain + change raw type)

The Monster struct already has `hp`, `sd`, `sprite_name`, `doc`, and `balance` fields. The current parser populates `hp`, `sd`, and `balance` but leaves `sprite_name` and `doc` as None. Fields `color` and `modifier_chain` are genuinely new. The `raw` field changes from `String` to `Option<String>` to support Path B (hand-authored IR without raw).

**WARNING**: Changing `raw: String` to `raw: Option<String>` breaks `builder/mod.rs` line 105 (`modifiers.push(mon.raw.clone())`). Chunk 1 must fix this callsite.

```rust
pub struct Monster {
    pub name: String,
    pub base_template: String,
    pub floor_range: String,
    pub hp: Option<u16>,
    pub sd: Option<String>,
    pub sprite_name: Option<String>, // POPULATE: use name as sprite key (currently None)
    pub color: Option<char>,         // NEW: from .col.
    pub doc: Option<String>,         // POPULATE: extract from .doc. (currently None)
    pub modifier_chain: Option<String>, // NEW: .i. chains inside monster def
    pub balance: Option<String>,
    pub raw: Option<String>,         // CHANGED from String to Option<String> for Path B
}
```

#### Boss (expand significantly)

```rust
/// **WARNING**: Changing `raw: String` to `raw: Option<String>` breaks `builder/mod.rs` line 110
/// (`modifiers.push(boss.raw.clone())`). Chunk 1 must fix this callsite.
pub struct Boss {
    pub name: String,
    pub level: Option<u8>,
    pub template: Option<String>,       // NEW: first template after .fight.
    pub hp: Option<u16>,                // NEW: from .hp. in fight content
    pub sd: Option<String>,             // NEW: from .sd. in fight content
    pub sprite_name: Option<String>,    // NEW: from .n. inside fight (for sprite lookup)
    pub doc: Option<String>,            // NEW: from .doc.
    pub modifier_chain: Option<String>, // NEW: .i. chains
    pub fight_units: Vec<BossFightUnit>, // NEW: parsed units in the fight
    pub variant: Option<String>,
    pub raw: Option<String>,            // CHANGED from String to Option<String> for Path B
}

/// A unit within a boss fight (parsed from +separated blocks inside .fight.(...))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BossFightUnit {
    pub template: String,               // e.g., "Wolf", "Slimelet", "Alpha"
    pub name: String,                   // display name
    pub hp: Option<u16>,
    pub sd: Option<String>,
    pub sprite_data: Option<String>,    // inline .img. data
}
```

#### Error types (`src/error.rs`)

**No new error variants needed.** The existing `BuildError { component: String, message: String }` already handles emission failures generically. The hero_emitter already uses this pattern: `component: format!("hero:{}", hero.internal_name)`. New emitters follow the same convention:
- Capture emitter: `component: format!("capture:{}", capture.pokemon)`
- Monster emitter: `component: format!("monster:{}", monster.name)`
- Boss emitter: `component: format!("boss:{}", boss.name)`

Adding type-specific variants (`CaptureEmitError`, `MonsterEmitError`, `BossEmitError`) would duplicate `BuildError`'s role without additional type safety -- the `component` string already identifies the source. This avoids unnecessary Display match arms and error.rs churn.

**Note:** Parsers keep bare return types (fallback-on-error pattern). Parse error variants are not needed -- parsers log warnings internally and populate what they can. The emitters return `Result<String, CompilerError>` using `BuildError` (raw fallback may not be available for Path B hand-authored IR).

#### Remove CharSelect and DittoConfig from ModIR

`CharSelect` and `DittoConfig` are sliceymon-specific concepts baked into a generic pipeline:
- **"Ditto"** is just a hero with many tier blocks (one per copyable form). Any mod can have a shapeshifter hero. It should stay in `ModIR.heroes` as a normal `Hero`. Remove the special ditto detection logic (extractor/mod.rs lines 119-134), `DittoConfig`, `DittoForm`, and `ModIR.ditto`.
- **"CharSelect"** is just a `Selector` structural modifier that happens to contain `dabble.tier.0`. Only sliceymon uses it. Remove `CharSelect`, `CharSelectPhase`, `CharSelectEntry`, and `ModIR.charselect`. The modifier stays in `ModIR.structural` as `StructuralType::Selector`.

```rust
// REMOVE from ModIR:
//   pub charselect: Option<CharSelect>,
//   pub ditto: Option<DittoConfig>,
// REMOVE types: CharSelect, CharSelectPhase, CharSelectEntry, DittoConfig, DittoForm
```

Also remove:
- Ditto detection logic in `extractor/mod.rs` (lines 119-134) — the "Housecat + white + >10 tiers" check
- CharSelect detection in `extractor/mod.rs` (lines 77-91) — the `dabble.tier.0` special case in the Selector match arm. Simplify to route ALL selectors to structural unconditionally.
- Builder charselect emission block (lines 52-58) and ditto emission block (lines 70-76) in `builder/mod.rs`
- Merge logic for charselect/ditto in `merge.rs` (lines 88-96)

#### Structural (add parsed content)

```rust
pub struct StructuralModifier {
    pub modifier_type: StructuralType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,           // NEW: extracted .mn. name
    #[serde(default)]
    pub content: StructuralContent,     // NEW: parsed content enum (defaults to Raw)
    pub raw: String,
}

impl StructuralModifier {
    /// Convenience constructor — fills name=None, content=Raw. Used during extraction
    /// before structural parsing is wired (Chunk 7).
    pub fn new_raw(modifier_type: StructuralType, raw: String) -> Self {
        Self { modifier_type, name: None, content: StructuralContent::Raw, raw }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum StructuralContent {
    HeroPoolBase { hero_refs: Vec<String> },
    ItemPool { items: Vec<ItemPoolEntry> },
    BossModifier { flags: Vec<String> },
    PartyConfig { party_name: String, members: Vec<String> },
    EventModifier { event_name: String },
    Dialog { phase: String },
    Selector { options: Vec<String> },
    GenSelect { options: Vec<String> },
    LevelUpAction { content: String },
    PoolReplacement { hero_names: Vec<String> }, // NEW: extracted hero names from ((heropool.Name+Name+(replica.X...).n.X+...))
    #[default]
    Raw,  // EndScreen and Unknown only -- all other types have parsed content
    // Note: Difficulty reuses Selector { options }, ArtCredits reuses Dialog { phase }.
    // StructuralType carries the semantic distinction. 11 variants total.
}
// Note: uses #[derive(Default)] on the enum with #[default] on Raw variant. No manual impl needed.

/// An item entry within an ItemPool structural modifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPoolEntry {
    pub name: String,
    pub tier: Option<i8>,           // Can be negative (-3 in pansaer)
    pub content: String,            // Raw item modifier content
}
```

#### Merge updates (`src/ir/merge.rs`)

- **No `.tiers` reference exists in merge.rs** — merge replaces entire Hero objects by `internal_name`, never accesses hero fields directly. No rename needed.
- Remove charselect merge (lines 88-91: `if overlay.charselect.is_some() { base.charselect = overlay.charselect; }`)
- Remove ditto merge (lines 93-96: `if overlay.ditto.is_some() { base.ditto = overlay.ditto; }`)
- `Monster.raw` and `Boss.raw` changed from `String` to `Option<String>` — merge comparisons are unaffected (merge replaces by name, not by raw content)
- Structural merge: **upgrade match key from `modifier_type` to `(modifier_type, name)`**. The current merge matches structural modifiers by `modifier_type` only (line 80), which means if a mod has two Dialogs with different `.mn.` names, the overlay replaces the wrong one. Since this plan adds `name: Option<String>` to `StructuralModifier`, the merge must use it. New matching logic: if the overlay modifier has `name: Some(n)`, match against base modifiers with the same `(modifier_type, name)` pair. If `name: None`, fall back to matching by `modifier_type` only (backward-compatible for hand-authored IR that doesn't set names). When both base and overlay have parsed `content` (not Raw), overlay replaces entirely. Unknown types are still appended unconditionally.
- No new merge keys needed — captures still match by `pokemon`, monsters by `name`, etc.

### 2. Classifier Changes (`src/extractor/classifier.rs`)

Add `PoolReplacement` variant to `ModifierType`:
```
if starts_with_ci "((heropool." → PoolReplacement
```
Insert BEFORE the existing Hero check.

**Why this reclassification is safe:** In punpuns.txt, the `((heropool.` modifier contains embedded `(replica....)` hero definitions, so it currently matches `ModifierType::Hero` (because it contains both "heropool" and "replica."). However, it is semantically a heropool override — a structural modifier that replaces the game's hero roster — not a hero definition. Routing it to structural passthrough is correct. The embedded replica definitions inside it are part of the pool override content.

**Also add `PoolReplacement` to `StructuralType` enum** in `src/ir/mod.rs` (Chunk 1). Pool replacement modifiers are classified in extractor/mod.rs and routed to `structural` with `StructuralType::PoolReplacement`. Without this variant, there is no way to store them.

Add `Difficulty`, `ArtCredits`, `EndScreen` variants to `ModifierType`. These `StructuralType` variants already exist in `ir/mod.rs` and the builder already emits them, but the classifier has no rules -- Difficulty is currently misclassified as Selector (both match `.ph.s`), ArtCredits is misclassified as Dialog (both match `.ph.4`). Add classification rules BEFORE the generic Selector/Dialog catches:
```
if contains ".ph.s" AND (contains "diff." OR contains_ci "difficulty") → Difficulty
if contains ".ph.4" AND ".mn.Art Credits" → ArtCredits
```

**Difficulty rule verification against all 4 working mods:**
- sliceymon: 1 match (`.mn.Difficulty Select` -- contains `diff.heaven`, `diff.Easy`, etc.) -- CORRECT
- pansaer: 1 match (`.mn.Boss Fight 05` -- contains `diff.hard`, `diff.unfair`, `diff.Easy`) -- CORRECT, this IS a difficulty selection screen embedded in a boss fight event
- punpuns: 0 matches (2 `.ph.s` selectors, neither contains `diff.`) -- CORRECT
- community: 1 match (`.mn.menuDiff` -- contains `diff.heaven`, `diff.hard`, etc.) -- CORRECT

**ArtCredits rule verification:** Only punpuns has `.mn.Art Credits` (line 6). Sliceymon has `.mn.Credits` which is a different dialog. Community has no `.ph.4` modifiers.

EndScreen has no examples in any working mod. Add the `ModifierType::EndScreen` variant but no classification rule -- it will be classifiable only via the Unknown path (the `StructuralType::EndScreen` variant is available for hand-authored IR).

Add corresponding routing match arms in `extractor/mod.rs` (same pattern as other structural types — push to `structural` with the correct `StructuralType`).

Add `detect_hero_format()`:
```
if contains_ci "ph.b" AND "!mheropool." → Sliceymon
if contains_ci "heropool." AND "replica." → Grouped
else → Unknown
```

### 3. Parser Changes

#### hero_parser.rs
- Rename `HeroTier` → `HeroBlock` in return types (Chunk 1 — mechanical rename)
- Add `color: extract_color(content)` to block construction (Chunk 2 — new logic)
- Add `format: HeroFormat::Sliceymon` / `Grouped` / `Unknown` dispatch (Chunk 2 — new logic)
- New `parse_grouped()` function for pansaer/punpuns/community format (Chunk 2 — new logic)

#### capture_parser.rs
- `parse_capture()`: Extract `ball_tier` (from `.tier.` in wrapper), `hp` (from `.hp.` in replica), `sprite_name` (from `.n.` = pokemon name), `color` (from `.col.`), `item_modifiers` (`.i.` chains), `sticker` (`.sticker.`), `toggle_flags` (`#tog` patterns)
- `parse_legendary()`: Extract `summoning_item` (`.n.` of outer item wrapper), `hp`, `sprite_name`, `color`, `doc`, `speech`, `abilitydata`, `item_modifiers`
- **Keep bare return types** — use hero_parser pattern: try-parse internally, fall back to defaults on failure. This avoids breaking callsites in `extractor/mod.rs` during parallel execution.

#### monster_parser.rs
- Extract `sprite_name` (use `name` as sprite key since `.img.` data is inline), `color`, `doc`, `modifier_chain` (`.i.` chains inside monster definition)
- Change `raw` assignment from `modifier.to_string()` to `Some(modifier.to_string())`
- **Keep bare return type** — same rationale as capture_parser

#### boss_parser.rs
- Extract `template` (first word after `.fight.(`), `hp`, `sd`, `sprite_name`, `doc`, `modifier_chain`
- Parse `fight_units`: split `.fight.(...)` content at depth-0 `+`, extract template/name/hp/sd/sprite_data from each unit
- Change `raw` assignment from `modifier.to_string()` to `Some(modifier.to_string())`
- **Keep bare return type** — same rationale as capture_parser

#### New: structural parsing
- After classification, extract `name` via `extract_mn_name()` for all structural types
- Parse `StructuralContent` based on `modifier_type`:
  - **HeroPoolBase**: Extract hero names (split on `.` at depth 0, filter non-property segments)
  - **ItemPool**: Extract item references (split on `#` inside parens, extract `.n.` names)
  - **BossModifier**: Extract flag strings (split on `&`, parse `N.flag_type` patterns)
  - **PartyConfig**: Extract party name (after `=party.`) and member names (from `+()` blocks)
  - **EventModifier**: Extract event name (after `=`)
  - **Dialog**: Extract phase identifier (from `.ph.` marker)
  - **Selector**: Extract option names/labels
  - **GenSelect**: Extract generation options
  - **LevelUpAction**: Extract level-up content
  - **Difficulty**: Parse as Selector (identical `@1[color]Label@2!m(...)` option pattern). Returns `StructuralContent::Selector { options }` — the `StructuralType::Difficulty` tag carries the semantic distinction.
  - **ArtCredits**: Parse as Dialog (identical `.ph.4` text body pattern). Returns `StructuralContent::Dialog { phase }` — the `StructuralType::ArtCredits` tag carries the semantic distinction.
  - **PoolReplacement**: Extract hero names from `((heropool.Name+Name+(replica.X...).n.X+...))` — split at depth-0 `+`, extract plain names and `.n.` names from replica blocks. Returns `StructuralContent::PoolReplacement { hero_names }`. Does NOT deeply parse embedded replica blocks (those round-trip via raw).

### 4. New Emitters

#### capture_emitter.rs (NEW)
- `emit_capture(capture: &Capture) -> Result<String, CompilerError>`
  - Reconstruct: `itempool.((hat.replica.{template}.n.{pokemon}.sd.{sd}...)).n.{ball_name}.tier.{ball_tier}`
  - Raw fallback if `capture.raw.is_some()`
  - No sprites parameter -- captures have inline `.img.` data (per Out of Scope: "Sprite resolution for non-hero types")
- `emit_legendary(legendary: &Legendary) -> Result<String, CompilerError>`
  - Reconstruct: `itempool.((hat.(replica.{template}...cast.{abilitydata}...))).n.{summoning_item}`
  - Raw fallback if `legendary.raw.is_some()`
  - No sprites parameter -- same rationale as captures

#### monster_emitter.rs (NEW)
- `emit_monster(monster: &Monster) -> Result<String, CompilerError>`
  - Reconstruct: `{floor_range}.monsterpool.(replica.{template}.n.{name}.hp.{hp}.sd.{sd}...)`
  - Raw fallback if `monster.raw.is_some()` -- but `raw` is `Option<String>` (changed in this plan), so Path B IR may have `raw: None` and must reconstruct from fields or return `BuildError`

#### boss_emitter.rs (NEW)
- `emit_boss(boss: &Boss) -> Result<String, CompilerError>`
  - Reconstruct: `ch.om{level}.fight.({template}...{fight_content})`
  - Raw fallback if `boss.raw.is_some()` -- but `raw` is `Option<String>` (changed in this plan), so Path B IR may have `raw: None` and must reconstruct from fields or return `BuildError`

#### structural_emitter.rs (UPDATE)
- Dispatch on `structural.content`:
  - For `StructuralContent::Raw` -> emit `raw` (only fallback -- used for Unknown types and types with no parsed content)
  - For all parsed variants -> reconstruct from structured fields. This is required for Path B (hand-authored IR without raw). Each variant's reconstruction must produce valid output that the game accepts.
  - If reconstruction fails (malformed content), fall back to `raw` (always available for structural modifiers -- unlike heroes/captures/monsters/bosses, structural modifiers never have `raw: Option<String>`, they always have `raw: String`)
- **Signature stays `fn emit(s: &StructuralModifier) -> String`** (not `Result`) because structural modifiers always have `raw: String` as fallback. This differs from capture/monster/boss emitters which return `Result<String, CompilerError>` because their `raw` is `Option<String>` and Path B IR may have `raw: None`.

### 5. Validator Additions (`src/validator.rs`)

New `phase_content_blocks()` — shared content validation for ANY type with `.sd.`/`.hp.` fields.

**Design note:** The validator operates on raw textmod text, not IR types. `phase_content_blocks()` takes a raw modifier string (or a substring of one), a modifier index, an optional name, and a `&mut ValidationReport`. It uses the existing `extract_sd_faces()` and `.hp.` extraction logic (currently inline in `phase_hero()`) to validate content-level rules. This is a refactoring of existing inline logic into a shared function, not a new concept.

**Signature:** `fn phase_content_blocks(content: &str, idx: usize, mn: &Option<String>, report: &mut ValidationReport)`

Rules applied:
- **E008**: Face format validation (reuse existing `parse_face_entry()`)
- **E009**: Face ID range 0-187
- **E010**: HP range 1-999
- **W001**: Face count 4-6
- **W002**: Pip values <= 25

Apply to:
- Heroes (all blocks, via `phase_hero_blocks()` — replaces inline checks in `phase_hero()`)
- Captures (sd field — extract from classified Capture modifiers)
- Legendaries (sd field — extract from classified Legendary modifiers)
- Monsters (sd field, if present — extract from classified Monster modifiers)
- Bosses (sd field, if present — extract from classified Boss modifiers)

New rules:
- **E013**: Monster floor range format validation (`N-M` where N <= M)
- **E014**: Boss level range (1-20)
- **E015**: Abilitydata internal validation — find `.abilitydata.(...)` in raw modifier text, extract the parenthesized content, then apply `extract_sd_faces()` + `parse_face_entry()` to validate face IDs/format inside. Only validates `.sd.` faces when `.sd.` is present inside the abilitydata content (some abilitydata has no `.sd.` — skip those). Applies to heroes and legendaries that have `.abilitydata.` in their raw text.
- **E016**: Cross-type reference validation -- verify that string references resolve across the ModIR: hero names in `HeroPoolBase.hero_refs` must exist in `ModIR.heroes` (matched by `internal_name`), hero names in `PoolReplacement.hero_names` must exist in `ModIR.heroes` (matched by `internal_name`), party member names in `PartyConfig.members` must exist in heroes (also by `internal_name`), etc. **API design:** Add a NEW public function `pub fn validate_cross_references(ir: &ModIR) -> ValidationReport` alongside the existing `validate(&str)` function. This keeps the two validation architectures cleanly separated -- `validate()` operates on raw text (per-modifier), `validate_cross_references()` operates on structured IR (cross-type). The CLI calls both (wired in Chunk 7). The function iterates `ir.structural`, matches on `StructuralContent` variants, and checks references against `ir.heroes` by `internal_name` using **case-insensitive comparison** (hero_refs from raw text may be Title Case like "Thief" while `internal_name` is lowercase like "thief" for grouped heroes that lack `ph.b` prefixes). **Note:** In working mods, HeroPoolBase modifiers are typically bare `heropool.` prefixes or pool removals, not simple name lists. The `hero_refs` field will often be empty. E016 fires infrequently on HeroPoolBase in practice but is architecturally correct for hand-authored IR where hero_refs are explicitly populated.
- **W006**: Capture missing sprite_name
- **W007**: Legendary missing abilitydata

### 6. Builder Assembly (`src/builder/mod.rs`)

Remove 3 TODO stubs (charselect, ditto, capture passthrough) and replace with:
- Captures: call `capture_emitter::emit_capture()`
- Legendaries: call `capture_emitter::emit_legendary()`
- Monsters: call `monster_emitter::emit_monster()`
- Bosses: call `boss_emitter::emit_boss()`
- Structural: call updated `structural_emitter::emit()`

Remove charselect/ditto special emission paths entirely -- charselect is now a Selector structural modifier, and "ditto" heroes are normal heroes.

Add PoolReplacement filter in structural emission loop (deferred from Chunk 2).

Rename `.tiers` -> `.blocks` in hero references.

## Parallel Execution Map

```
Chunk 1 (IR Schema Foundation)
    |
    +---> Chunk 2 (Hero + Classifier)  ----+
    |         |                            |
    |         +---> Chunk 3 (Capture)  ----+-- Group B (after Chunk 2)
    |         |                            |
    |         +---> Chunk 4 (Mon+Boss) ----+
    |                                      |
    +---> Chunk 5 (Structural Parsing) ----+-- Group A (after Chunk 1)
    |                                      |
    +---> Chunk 6 (Validator)          ----+
                                           |
              +----------------------------+
              |
              +---> Chunk 7 (Builder Assembly)
                       |
                    Chunk 8 (Integration + Round-Trip Tests)
```

- **Round 1 -- Foundation (sequential):** Chunk 1
- **Round 2 -- After Chunk 1 (parallel):** Chunks 2, 5, 6
- **Round 3 -- After Chunk 2 (parallel, overlaps with 5+6 if still running):** Chunks 3, 4 (depend on Chunk 2's util.rs helper move)
- **Round 4 -- Assembly (after Rounds 2+3 complete):** Chunk 7
- **Round 5 -- Integration (after Chunk 7):** Chunk 8
- **Minimum wall-clock rounds:** 5

**Why Chunk 6 is in Group A:** The validator has two parts, both safe to parallelize:
- **Part A (per-modifier content validation):** Operates on raw textmod text using the classifier — does NOT depend on IR type parsing from Chunks 2-5. The validator's new rules (E013/E014/E015/W006/W007) use raw text extraction, not IR fields. The only soft dependency is that Chunk 6's test `validate_grouped_hero_no_wrapper_errors` requires the classifier to correctly identify grouped heroes — but the current classifier already classifies them as `Hero` type, which is sufficient.
- **Part B (cross-type reference validation, E016):** The new `validate_cross_references(&ModIR)` function reads `StructuralContent` variants (from Chunk 1 type definitions) and `ir.heroes` names. It does NOT depend on Chunks 2-5 parsing — E016 tests construct `ModIR` manually with hand-built `StructuralContent` values. The function will be useful in production only after Chunk 5+7 wire structural parsing to populate `StructuralContent`, but it compiles and tests correctly using hand-constructed IR.
- Chunk 6 can safely run in parallel with all Group A chunks.

### Parallel Group A File Conflict Notes

Chunks 2 and 5 both touch `src/extractor/mod.rs`:
- **Chunk 2**: adds `PoolReplacement` match arm (~line 29 area)
- **Chunk 5**: needs `pub mod structural_parser;` declaration (line 1-6 area)

**Mitigation:** Defer the `pub mod structural_parser;` declaration to Chunk 7 (structural_parser.rs file exists but is not in the module tree until wiring). This is consistent with how Chunks 3-4 handle their new emitter modules (capture_emitter.rs, monster_emitter.rs, boss_emitter.rs are all created but not declared until Chunk 7). Chunk 5 does NOT touch `extractor/mod.rs` -- no parallel conflict.

Chunks 3 and 4 have **no file conflicts** with any other Group A chunk. However, **they have a data dependency on Chunk 2**: both need `extract_simple_prop()`, `find_next_prop_boundary()`, `extract_modifier_chain()`, and `extract_facades_from_chain()` which Chunk 2 moves from `hero_parser.rs` to `util.rs`. **Chunks 3 and 4 must start after Chunk 2 completes** (they can still run parallel with each other and with Chunks 5+6). This changes the effective parallelism: Chunk 2 runs first in Group A, then Chunks 3+4+5+6 run in parallel.

**Chunk 7 file conflicts are safe by design:** Chunk 7 modifies both `builder/mod.rs` and `extractor/mod.rs` (same files as Chunk 2). This is safe because Chunk 7 runs AFTER all of Group A completes -- the dependency graph guarantees sequential execution. All Group A edits to these files are finalized before Chunk 7 touches them. Chunk 7 also picks up the PoolReplacement structural emission filter deferred from Chunk 2.

## Implementation Chunks

### Chunk 1: IR Schema Foundation

All type definition changes plus mechanical fixups required for compilation. Renames, serde annotations, type changes, removal of CharSelect/DittoConfig from all files that construct or reference those types, and minimal constructor updates in ALL parser files that construct modified structs. The goal is `cargo check` passes after this chunk alone.

**Note:** This chunk touches 9 files (exceeds the 5-file guideline). This is necessary because the type removals (CharSelect, DittoConfig), type changes (Monster.raw/Boss.raw to Option), and new struct fields (Capture.color, Legendary.color/doc/speech, Monster.color/modifier_chain, Boss expansion) must propagate to ALL construction sites in a single atomic chunk. The 3 parser files (capture_parser.rs, monster_parser.rs, boss_parser.rs) require minimal updates here -- adding `None`/`vec![]` defaults for new fields and changing `raw: modifier.to_string()` to `raw: Some(modifier.to_string())`. Deep parsing logic for these fields is deferred to Chunks 3-4. Splitting would leave intermediate states that don't compile.

**Read first:**
- `src/ir/mod.rs` (all current type definitions — CharSelect at line 82, DittoConfig at line 105, Hero.tiers at line 46, HeroTier at line 54)
- `src/ir/merge.rs` (charselect/ditto merge logic at lines 88-96 — NOTE: no `.tiers` reference exists in this file, merge replaces whole Hero objects)
- `src/builder/hero_emitter.rs` (2 `.tiers` references at lines 16, 31)
- `src/builder/mod.rs` (charselect emission at lines 52-58, ditto emission at lines 70-76, monster `mon.raw.clone()` at line 105, boss `boss.raw.clone()` at line 110 — these break when raw becomes Option<String>)
- `src/extractor/mod.rs` (charselect variable at line 22, ditto variable at line 23, Selector match arm with charselect detection at lines 77-91 — the entire arm needs reworking to route ALL selectors to structural, DittoConfig detection at lines 119-134, charselect/ditto in ModIR construction at lines 143-144, `.tiers` at line 124)
- `src/extractor/hero_parser.rs` (`use HeroTier` at line 2, `HeroTier` return type at line 190, `HeroTier` construction at line 244, `tiers` variable/field references at lines 19, 77, 80, 101 — note: these are `tiers: vec![]`, `let mut tiers`, `tiers.push()`, `tiers,` — NOT `.tiers` accessors, so grep for `\.tiers` won't catch them; also grep for bare `tiers`)
- `src/extractor/capture_parser.rs` (Capture constructor at line 9 — needs `color: None`; Legendary constructor at line 28 — needs `color: None, doc: None, speech: None`)
- `src/extractor/monster_parser.rs` (Monster constructor at line 11 — needs `color: None, modifier_chain: None`; `raw:` at line 20 — change from `modifier.to_string()` to `Some(modifier.to_string())`)
- `src/extractor/boss_parser.rs` (Boss constructor at line 9 — needs `template: None, hp: None, sd: None, sprite_name: None, doc: None, modifier_chain: None, fight_units: vec![]`; `raw:` at line 13 — change from `modifier.to_string()` to `Some(modifier.to_string())`)

**Files:**
| Action | File |
|--------|------|
| Edit | `src/ir/mod.rs` — rename HeroTier->HeroBlock, add HeroFormat, add Hero.format, rename Hero.tiers->blocks, add Capture.color, add Legendary.color/doc/speech, add Monster.color/modifier_chain + raw->Option, add Boss expansion (template/hp/sd/sprite_name/doc/modifier_chain/fight_units) + raw->Option + BossFightUnit struct, add StructuralModifier.name/content + StructuralContent enum + ItemPoolEntry + `StructuralModifier::new_raw()` convenience constructor (`impl` block -- needed by extractor/mod.rs constructor replacements in this same chunk), add PoolReplacement to StructuralType enum, REMOVE CharSelect/CharSelectPhase/CharSelectEntry/DittoConfig/DittoForm/ModIR.charselect/ModIR.ditto |
| Edit | `src/ir/merge.rs` — remove charselect merge (lines 88-91) and ditto merge (lines 93-96), update Monster/Boss raw comparisons to handle Option<String>, **upgrade structural merge key from `modifier_type` to `(modifier_type, name)`**: replace line 80 (`bs.modifier_type == s.modifier_type`) with `bs.modifier_type == s.modifier_type && bs.name == s.name` — this correctly handles multiple structural modifiers of the same type with different `.mn.` names (e.g., two Dialogs). When `name` is `None` on both sides, the match still works (None == None). Update doc comment (line 13) from "replace base modifiers with matching `modifier_type`" to "replace base modifiers with matching `(modifier_type, name)` pair". |
| Edit | `src/extractor/hero_parser.rs` — `use HeroTier` -> `use HeroBlock` (line 2), `HeroTier` -> `HeroBlock` in return types and construction (lines 190, 244), rename all `tiers` -> `blocks` variable/field references: `tiers: vec![]` (line 19), `let mut tiers` (line 77), `tiers.push(tier)` (line 80), `tiers,` field shorthand (line 101) |
| Edit | `src/extractor/mod.rs` — remove charselect variable (line 22) and ditto variable (line 23), simplify Selector match arm to route ALL selectors to structural (lines 77-91 — remove the `dabble.tier.0` special case entirely), remove ditto detection block (lines 119-134), remove charselect/ditto from ModIR construction (lines 143-144), `.tiers` -> `.blocks` (line 124 — note: this line is inside the ditto detection block being removed, so no rename needed), remove `CharSelect`/`DittoConfig` from `use crate::ir::*` if explicit, replace all `StructuralModifier { modifier_type: ..., raw: ... }` constructors with `StructuralModifier::new_raw(...)` (approx 10 sites, lines 36-114) |
| Edit | `src/extractor/capture_parser.rs` — add `color: None` to Capture constructor (line 9), add `color: None, doc: None, speech: None` to Legendary constructor (line 28). **Minimal defaults only** -- deep parsing logic deferred to Chunk 3. |
| Edit | `src/extractor/monster_parser.rs` — add `color: None, modifier_chain: None` to Monster constructor (line 11), change `raw: modifier.to_string()` to `raw: Some(modifier.to_string())` (line 20). **Minimal defaults only** -- deep parsing logic deferred to Chunk 4. |
| Edit | `src/extractor/boss_parser.rs` — add `template: None, hp: None, sd: None, sprite_name: None, doc: None, modifier_chain: None, fight_units: vec![]` to Boss constructor (line 9), change `raw: modifier.to_string()` to `raw: Some(modifier.to_string())` (line 13). **Minimal defaults only** -- deep parsing logic deferred to Chunk 4. |
| Edit | `src/builder/hero_emitter.rs` — `.tiers` -> `.blocks` (lines 16, 31) |
| Edit | `src/builder/mod.rs` -- remove charselect emission block (lines 52-58), remove ditto emission block (lines 70-76), fix monster emission to handle Option<String> raw (`if let Some(ref raw) = mon.raw { modifiers.push(raw.clone()); }` at line 105), fix boss emission similarly (line 110), update assembly order comment (line 12) and selector comment (line 47) to remove "charselect" references |

**TDD (verified after this chunk):**
1. `cargo check --lib` passes (library code only — this is the gate; must pass with Chunk 1 alone. Do NOT use `cargo check` or `cargo test` which also compile integration tests that need Chunk 8 updates)
2. `grep -rn "HeroTier" compiler/src/` returns 0 results
3. `grep -rn "\.tiers" compiler/src/` returns 0 results
3a. `grep -rwn "tiers" compiler/src/` returns 0 results (word-boundary grep catches ALL forms: `tiers:`, `let mut tiers`, `tiers.push()`, `tiers,` — things the `\.tiers` grep misses)
4. `grep -rn "charselect\|CharSelect\|DittoConfig\|DittoForm\|CharSelectPhase\|CharSelectEntry" compiler/src/` returns 0 results
5. All new fields have correct serde annotations (`#[serde(default)]` on HeroFormat, StructuralContent)
6. StructuralContent::Raw is the Default variant
7. ModIR::empty() compiles (no charselect/ditto fields)

**Checkpoint:** `cargo check --lib` passes (library code only). **WARNING:** `cargo test` will NOT compile after this chunk -- integration tests in `compiler/tests/` reference `HeroTier`, `.tiers`, `charselect`, `ditto`, and construct `StructuralModifier { modifier_type, raw }` without the new `name`/`content` fields. These test updates are in Chunk 8. Use `cargo check --lib` (not `cargo check` or `cargo test`) as the gate.

**If blocked:** If a file references `.tiers`, `HeroTier`, `CharSelect`, or `DittoConfig` that was missed, find with `grep -rn "HeroTier\|\.tiers\|CharSelect\|DittoConfig" compiler/src/` and fix. If a parser file has a struct constructor missing new fields, `cargo check --lib` will report the exact missing field -- add `None`/`vec![]` defaults.

---

### Chunk 2: Hero Generalization + Classifier Additions

Add HeroFormat detection, grouped parsing, PoolReplacement classification, hero emitter format dispatch. Also add Difficulty/ArtCredits/EndScreen classifier rules and routing (these share the same files -- classifier.rs and extractor/mod.rs -- and are additive, small changes).

**Note:** This chunk touches 5 files. The concerns are bundled because hero format generalization, classifier additions, and util.rs refactoring all share the same files (classifier.rs, hero_parser.rs, extractor/mod.rs). Splitting would create 2-3 chunks that each edit the same files, causing merge conflicts.

**Read first:**
- `src/extractor/classifier.rs` (classify() function, ModifierType enum)
- `src/extractor/hero_parser.rs` (try_parse_hero, parse_tier_block, separate_suffix)
- `src/builder/hero_emitter.rs` (emit function, verification helpers)

**Files:**
| Action | File |
|--------|------|
| Edit | `src/extractor/classifier.rs` -- add PoolReplacement, Difficulty, ArtCredits, EndScreen to ModifierType. Add detect_hero_format(). Insert pool replacement check before Hero check. Add Difficulty rule before Selector (`.ph.s` + `diff.`), ArtCredits rule before Dialog (`.ph.4` + `.mn.Art Credits`). EndScreen has no working-mod examples -- add variant but no classification rule yet. |
| Edit | `src/extractor/hero_parser.rs` -- add parse_grouped(), dispatch on format, add color extraction in parse_tier_block, add format field to Hero construction. Move `extract_simple_prop()`, `extract_modifier_chain()`, `extract_facades_from_chain()`, `find_next_prop_boundary()` to util.rs (they're needed by parsers in Chunks 3-4). |
| Edit | `src/util.rs` -- receive moved helper functions from hero_parser.rs. Update imports. |
| Edit | `src/builder/hero_emitter.rs` -- dispatch on hero.format after raw check, add emit_grouped(), rename "no tiers" error to "no blocks", update Sliceymon color emission to use `block.color.unwrap_or(hero.color)` instead of always `hero.color` (per-block color override for multi-color heroes) |
| Edit | `src/extractor/mod.rs` -- add PoolReplacement, Difficulty, ArtCredits, EndScreen match arms routing to structural with correct StructuralType variants |

**Note:** The PoolReplacement filter in `builder/mod.rs` structural emission loop (~3 lines) is deferred to Chunk 7, which already edits `builder/mod.rs` for emitter wiring. This keeps Chunk 2 at 5 files.

**TDD:**
1. `pansaer_heroes_have_format_grouped` — parse pansaer, each hero has `format == Grouped`
2. `punpuns_heroes_have_blocks` — parse punpuns, each hero has `blocks.len() > 0`
3. `community_heroes_have_format_grouped` — parse community.txt, heroes have Grouped format
4. `pool_replacement_classified_correctly` — `((heropool.Thief+...` → `ModifierType::PoolReplacement`
5. `pool_replacement_routed_to_structural` — extract punpuns, no hero raw contains `((heropool.`
6. `emit_grouped_hero_produces_valid_output` — construct Hero with `raw: None, format: Grouped`, call emit(), verify balanced parens + contains `heropool.`
7. `emit_grouped_hero_no_sliceymon_suffix` — verify output does NOT contain `@2!m(skip&hidden&temporary)`
8. `detect_hero_format_sliceymon` — modifier with `ph.b` + `!mheropool.` → Sliceymon
9. `detect_hero_format_grouped` — modifier with `heropool.` + `replica.` (no `ph.b`) → Grouped
10. `difficulty_classified_correctly` — sliceymon difficulty modifier (`.ph.s` + `diff.heaven`) → `ModifierType::Difficulty` (not Selector)
11. `art_credits_classified_correctly` — punpuns art credits modifier (`.ph.4` + `.mn.Art Credits`) → `ModifierType::ArtCredits` (not Dialog)
12. `difficulty_routed_to_structural` — extract sliceymon, structural contains entry with `StructuralType::Difficulty`
13. `art_credits_routed_to_structural` — extract punpuns, structural contains entry with `StructuralType::ArtCredits`
14. `emit_sliceymon_uses_block_color` — construct Hero with `color: 'a'` and one HeroBlock with `color: Some('b')`, emit as Sliceymon, verify output contains `.col.b` (block color overrides hero color)
15. `emit_sliceymon_falls_back_to_hero_color` — construct Hero with `color: 'a'` and one HeroBlock with `color: None`, emit, verify output contains `.col.a` (hero color used when block has no override)

**Checkpoint:** `cargo check --lib` passes (integration tests remain broken until Chunk 8 updates them). Pansaer/punpuns/community heroes parsed into blocks with Grouped format.

**If blocked:**
- Grouped block parsing hits unexpected structure → fall back to HeroBlock with defaults + raw passthrough. Never crash.
- Pool replacement reclassification breaks tests → it was never meaningfully parsed (always raw passthrough).

---

### Chunk 3: Capture + Legendary Deep Parsing + Emission

Full property extraction for both types. New emitter file.

**Read first:**
- `src/extractor/capture_parser.rs` (current parse_capture, parse_legendary, extract_ball_name)
- `src/ir/mod.rs` (Capture, Legendary type definitions after Chunk 1)
- Sliceymon captures/legendaries in `working-mods/sliceymon.txt` (lines ~111-115) for format reference
- `src/util.rs` (available extraction helpers)

**Files:**
| Action | File |
|--------|------|
| Edit | `src/extractor/capture_parser.rs` — expand parse_capture() to extract ball_tier, hp, sprite_name, color, item_modifiers, sticker, toggle_flags. Expand parse_legendary() to extract summoning_item, hp, sprite_name, color, doc, speech, abilitydata, item_modifiers. **Keep bare return types** (not Result) to avoid breaking callsites in extractor/mod.rs which Chunk 2 edits in parallel. Use hero_parser pattern: try-parse internally, fall back to defaults on failure. **Note on ball_tier extraction:** The existing `extract_tier_number()` in hero_parser.rs is private and designed for replica content (`.tier.N` at depth 0). For captures, `.tier.` appears in the wrapper, not inside a replica block. Use `util::extract_simple_prop()` (moved from hero_parser in Chunk 2) to extract the `.tier.` value, then parse it as `u8`. Do NOT reuse `extract_tier_number()` directly. |
| Create | `src/builder/capture_emitter.rs` -- emit_capture() and emit_legendary() functions (no sprites parameter -- captures/legendaries have inline `.img.` data). Include `#[cfg(test)] mod tests { ... }` inline unit tests for emission logic (avoids needing module declaration before Chunk 7). |

**Note on module declarations:** `capture_emitter.rs` is created here but `pub mod capture_emitter;` in `builder/mod.rs` is deferred to Chunk 7 to avoid parallel file conflicts with Chunk 2. Inline `#[cfg(test)]` tests are written here but only compile and run after Chunk 7 adds the module declaration. Parser tests (TDD items 1-7) use existing test infrastructure since `capture_parser.rs` is already declared in `extractor/mod.rs`.

**Deferred compilation risk:** The emitter code in `capture_emitter.rs` CANNOT be compiled or tested until Chunk 7 adds the `pub mod` declaration. **Mitigation:** (1) Follow the exact pattern from `hero_emitter.rs` for structure and imports. (2) Manually verify the file parses as valid Rust by checking balanced braces and correct `use` paths. (3) Keep emitter logic simple -- raw fallback first, field reconstruction second. (4) Use ONLY `crate::ir::*`, `crate::error::CompilerError`, and `std::collections::HashMap` imports (same as hero_emitter.rs).

**Anti-hallucination constraints:**
- Do NOT confuse `.tier.` inside the replica block (hero tier number) with `.tier.` in the item wrapper (ball tier). The ball tier is OUTSIDE the `(replica....)` parens.
- Do NOT confuse the first `.n.` (ball name, outside replica) with `.n.` inside the replica (pokemon name). Use depth-aware extraction.
- Use ONLY extraction helpers from `util.rs` -- do NOT duplicate logic from hero_parser.rs.

**TDD:**

Parser tests (runnable immediately via existing integration test infrastructure):
1. `parse_capture_extracts_ball_tier` — parse sliceymon capture, assert ball_tier is Some
2. `parse_capture_extracts_hp` — assert hp is Some for captures with .hp.
3. `parse_capture_extracts_sprite_name` — assert sprite_name equals pokemon name
4. `parse_capture_extracts_toggle_flags` — assert toggle_flags extracted from #tog patterns
5. `parse_legendary_extracts_abilitydata` — assert abilitydata is Some
6. `parse_legendary_extracts_doc` — assert doc is Some
7. `parse_legendary_extracts_summoning_item` — assert summoning_item non-empty

Emitter tests (written as `#[cfg(test)]` inline in capture_emitter.rs — become active when Chunk 7 adds `pub mod` declaration):
8. `emit_capture_produces_valid_output` — construct Capture with raw: None, emit, verify contains `itempool.` + `hat.replica`
9. `emit_legendary_produces_valid_output` — construct Legendary with raw: None, emit, verify contains `cast.`
10. `capture_raw_fallback` — if raw is Some, emit returns raw unchanged

**Checkpoint:** `cargo check --lib` passes. Sliceymon captures + legendaries parse with all fields populated. Emitter code written and reviewed. Parser tests (items 1-7) in `tests/capture_tests.rs` can run in isolation via `cargo test --test capture_tests` (this file has no references to types broken by Chunk 1). Full `cargo test` still fails until Chunk 8. Emitter tests activate in Chunk 7.

**If blocked:**
- Capture format varies across mods → extract what's available, leave rest as None. Parser must never crash.
- Legendary nested structure too complex → extract top-level properties, leave deeply nested content in raw for reconstruction.

---

### Chunk 4: Monster + Boss Deep Parsing + Emission

Full property extraction for both types. New emitter files.

**Read first:**
- `src/extractor/monster_parser.rs` (current parse_monster, extract_base_template, extract_floor_range, extract_balance)
- `src/extractor/boss_parser.rs` (current parse_boss, extract_level, extract_variant)
- `src/ir/mod.rs` (Monster, Boss definitions after Chunk 1)
- Monster/boss examples in `working-mods/sliceymon.txt` and `working-mods/punpuns.txt`
- `src/util.rs` (available extraction helpers)

**Files:**
| Action | File |
|--------|------|
| Edit | `src/extractor/monster_parser.rs` — add sprite_name extraction (use name as key), color, doc, modifier_chain (.i. chains). **Note:** `raw` was already changed to `Some(modifier.to_string())` in Chunk 1 -- no raw change needed here. **Keep bare return type** (not Result) — same rationale as Chunk 3. |
| Edit | `src/extractor/boss_parser.rs` — add template (first word after .fight.()), hp, sd, sprite_name, doc, modifier_chain, fight_units (parse +separated blocks inside .fight.()). **Note:** `raw` was already changed to `Some(modifier.to_string())` in Chunk 1 -- no raw change needed here. **Keep bare return type** (not Result) — same rationale as Chunk 3. |
| Create | `src/builder/monster_emitter.rs` — emit_monster(): raw passthrough if raw.is_some(), else reconstruct monsterpool format. Include `#[cfg(test)] mod tests { ... }` inline unit tests. |
| Create | `src/builder/boss_emitter.rs` — emit_boss(): raw passthrough if raw.is_some(), else reconstruct ch.omN.fight.() format with fight_units. Include `#[cfg(test)] mod tests { ... }` inline unit tests. |
| Edit | `tests/boss_tests.rs` — fix 2 broken raw assertions: `assert_eq!(&mon.raw, modifier)` → `assert_eq!(mon.raw.as_ref().unwrap(), modifier)` (line 79), `assert_eq!(&boss.raw, modifier)` → `assert_eq!(boss.raw.as_ref().unwrap(), modifier)` (line 88). This is required so `cargo test --test boss_tests` validates Chunk 4's parser work. |

**Note on module declarations:** `monster_emitter.rs` and `boss_emitter.rs` are created here but `pub mod` declarations in `builder/mod.rs` are deferred to Chunk 7 to avoid parallel file conflicts with Chunk 2. Inline `#[cfg(test)]` tests are written here but only compile and run after Chunk 7 adds the module declarations.

**Deferred compilation risk:** Same as Chunk 3 -- emitter code cannot be compiled until Chunk 7. **Mitigation:** Follow `hero_emitter.rs` pattern exactly. Use ONLY `crate::ir::*`, `crate::error::CompilerError`, and `std::collections::HashMap` imports. Keep reconstruction logic simple with raw fallback.

**Anti-hallucination constraints:**
- Do NOT confuse `.hp.` inside a boss fight unit with `.hp.` at the top level of the boss modifier. Use depth-aware extraction or positional parsing relative to `.fight.(`.
- Boss fight units are separated by `+` at depth 1 (inside the `.fight.(...)` parens), NOT at depth 0. Use `split_at_depth0()` on the CONTENT inside `.fight.(...)` after stripping the outer parens.
- Monster `.doc.` may appear OUTSIDE the `monsterpool.(...)` block in some mods. Check both inside and outside.
- Use ONLY extraction helpers from `util.rs` -- do NOT duplicate logic.

**TDD:**

Parser tests (runnable immediately via existing integration test infrastructure):
1. `parse_monster_extracts_sprite_name` — assert sprite_name is Some for monsters with .img.
2. `parse_monster_extracts_color` — assert color extracted from .col. if present
3. `parse_monster_extracts_modifier_chain` — assert modifier_chain for monsters with .i. items
4. `parse_boss_extracts_template` — assert template is Some (first template after .fight.)
5. `parse_boss_extracts_hp` — assert hp from .hp. in fight content
6. `parse_boss_extracts_doc` — assert doc from .doc. if present
7. `parse_boss_extracts_fight_units` — parse sliceymon boss, assert fight_units.len() > 0
8. `parse_boss_fight_unit_has_name` — each fight unit has non-empty name

Emitter tests (written as `#[cfg(test)]` inline — become active when Chunk 7 adds `pub mod` declarations):
9. `emit_monster_produces_valid_output` — construct Monster with raw: None, emit, verify contains `monsterpool.`
10. `emit_boss_produces_valid_output` — construct Boss with raw: None + fight_units populated, emit, verify contains `.fight.`
11. `monster_raw_fallback` — Monster with raw: Some emits raw unchanged
12. `boss_raw_fallback` — Boss with raw: Some emits raw unchanged

**Checkpoint:** `cargo check --lib` passes. Sliceymon + punpuns monsters/bosses parse with expanded fields. Parser tests (items 1-8) are written in `tests/boss_tests.rs`. **Required fix in this chunk:** fix the 2 broken assertions in boss_tests.rs (lines 79, 88: change `&mon.raw`/`&boss.raw` to `mon.raw.as_ref().unwrap()`/`boss.raw.as_ref().unwrap()`) so that `cargo test --test boss_tests` runs and validates the new parser work. This is a 2-line change — do NOT defer to Chunk 8. Emitter tests activate in Chunk 7.

**If blocked:**
- Boss nested fight structure too complex → extract top-level properties, preserve full structure in raw for reconstruction. Boss emitter uses raw as primary path until structure is fully understood.
- Monster format varies significantly → extract what exists, None for absent fields.

---

### Chunk 5: Structural Type Parsing

Add StructuralContent parsing for all active structural types.

**Read first:**
- `src/extractor/mod.rs` (structural routing, lines 35-114)
- `src/extractor/classifier.rs` (all structural type classification rules)
- `src/ir/mod.rs` (StructuralModifier, StructuralContent after Chunk 1)
- Examples of each structural type from working mods
- `src/util.rs` (extraction helpers)

**Files:**
| Action | File |
|--------|------|
| Create | `src/extractor/structural_parser.rs` — parse_structural_content() dispatches on StructuralType, returns StructuralContent. Also `extract_structural_name()` for .mn. extraction. Include `#[cfg(test)] mod tests { ... }` inline unit tests. |
| Edit | `src/builder/structural_emitter.rs` — dispatch on `s.content` variant instead of always returning `s.raw`. The `emit()` signature stays `fn emit(s: &StructuralModifier) -> String` (unchanged), but the implementation now checks `s.content`: for `StructuralContent::Raw` return `s.raw` (existing behavior); for parsed variants, attempt reconstruction and fall back to `s.raw` if reconstruction fails. |

**Note on module declaration:** `src/extractor/structural_parser.rs` is created here but the `pub mod structural_parser;` declaration in `src/extractor/mod.rs` is deferred to Chunk 7 to avoid a parallel file conflict with Chunk 2 (which also edits `extractor/mod.rs`). Inline `#[cfg(test)]` tests are written here but only compile and run after Chunk 7 adds the module declaration. The structural_emitter.rs changes compile immediately since that module is already declared.

**Deferred compilation risk:** Same as Chunks 3-4 -- parser code cannot be compiled until Chunk 7. **Mitigation:** Follow existing parser patterns (capture_parser.rs, monster_parser.rs). Use ONLY `crate::ir::*` and `crate::util` imports. The structural_emitter.rs changes CAN be compiled and tested immediately (TDD items 1-2), providing partial verification within this chunk.

**TDD:**

Structural emitter tests (runnable immediately since structural_emitter.rs is already in the module tree):
1. `structural_emitter_raw_fallback` — all types still emit raw correctly (regression)
2. `structural_emitter_dispatches_on_content` — StructuralModifier with non-Raw content emits correctly

Parser tests (written as `#[cfg(test)]` inline — become active when Chunk 7 adds module declaration):
3. `parse_heropoolbase_extracts_hero_refs` — parse HeroPoolBase raw, assert hero_refs.len() > 0
4. `parse_itempool_extracts_item_refs` — parse ItemPool raw, assert items.len() > 0
5. `parse_bossmodifier_extracts_flags` — parse BossModifier raw, assert flags non-empty
6. `parse_partyconfig_extracts_name` — parse PartyConfig raw, assert party_name non-empty
7. `parse_eventmodifier_extracts_name` — parse EventModifier raw, assert event_name non-empty
8. `parse_dialog_extracts_phase` — parse Dialog raw, assert phase non-empty
9. `structural_name_extracted` — all structural types with .mn. have name populated
10. `structural_unknown_is_raw` — Unknown type has content == StructuralContent::Raw
11. `parse_difficulty_as_selector` — parse Difficulty modifier, assert content is `Selector { options }` with options like "Heaven", "Easy", "Normal", etc.
12. `parse_art_credits_as_dialog` — parse ArtCredits modifier, assert content is `Dialog { phase }` with credits text
13. `parse_pool_replacement_extracts_hero_names` — parse punpuns PoolReplacement modifier, assert content is `PoolReplacement { hero_names }` with names like "Thief", "Scoundrel", "Reflection" etc.
14. `pool_replacement_hero_names_includes_replica_names` — verify `.n.` names from inside `(replica.X...).n.X` blocks are extracted

**Checkpoint:** `cargo check --lib` passes. Structural emitter tests pass via `cargo test --lib` (items 1-2, inline in already-declared `structural_emitter.rs`). `structural_parser.rs` written and reviewed but cannot compile until Chunk 7 adds `pub mod` declaration. Parser tests (items 3-14) activate in Chunk 7.

**If blocked:**
- Structural format too varied → extract name + whatever key property is parseable, leave content as Raw for that variant. Never crash.
- Structural content reconstruction produces different output from raw → use raw for emission, content for validation/inspection only.

---

### Chunk 6: Validator Generalization

Two concerns, clearly separated:
1. **Per-modifier content validation** (generalize E008-E012/W001-W002 to all types + new rules E013/E014/E015/W006/W007): Generalize existing raw-text validation to all types with `.sd.`/`.hp.` fields. Uses the existing `validate(&str)` API.
2. **Cross-type reference validation** (E016): New `validate_cross_references(&ModIR)` public function. Operates on structured IR, not raw text. Separate function, separate tests, separate architectural pattern.

**Read first:**
- `src/validator.rs` (current phase_hero, E004-E012, W001-W002, parse_face_entry helper, extract_sd_faces helper)
- `src/ir/mod.rs` (all IR types after Chunk 1, especially StructuralContent enum for E016)
- `src/extractor/classifier.rs` (classify() for routing non-hero types to validation)

**Files:**
| Action | File |
|--------|------|
| Edit | `src/validator.rs` — **Part A (per-modifier):** add `phase_content_blocks()` shared validator, refactor `phase_hero()` to delegate content checks to it, extend the Phase 4 loop to also call `phase_content_blocks()` on Capture/Legendary/Monster/Boss classified modifiers, add E013 (floor range), E014 (boss level), E015 (abilitydata face validation), W006 (capture missing sprite), W007 (legendary missing abilitydata). E015 implementation: find `.abilitydata.(` in modifier text, extract content up to matching `)` using `find_matching_close_paren()`, apply `extract_sd_faces()` + `parse_face_entry()` to inner content, skip if no `.sd.` inside. **Part B (cross-type):** add new public function `pub fn validate_cross_references(ir: &ModIR) -> ValidationReport`. This function iterates `ir.structural`, matches on `content` field: for `StructuralContent::HeroPoolBase { hero_refs }` check each ref exists in `ir.heroes` (by `internal_name` using **case-insensitive comparison** -- heropool refs may be Title Case like "Thief" while grouped heroes have lowercase `internal_name` like "thief"); for `StructuralContent::PartyConfig { members, .. }` check each member exists in `ir.heroes` (also by `internal_name`, case-insensitive). Findings use rule ID E016. Import `use crate::ir::{ModIR, StructuralContent};` for Part B only. **Note on HeroPoolBase hero_refs:** In working mods, HeroPoolBase modifiers are typically bare `heropool.` prefixes on hero definitions (community) or pool removals (`heropool.o0.0.part.0`), not simple name lists. The `hero_refs` field will often be empty for these. The PoolReplacement modifier (punpuns `((heropool.Thief+Scoundrel+...`) DOES have a name list but is classified as PoolReplacement, not HeroPoolBase. E016 validation on HeroPoolBase is architecturally correct but may fire infrequently in practice. |
| Edit | `tests/validator_tests.rs` — add tests for new validation on each type. E016 tests use hand-constructed `ModIR` objects (not extracted from working mods) since structural parsing (Chunk 5) may not be wired yet. |

**TDD:**

Part A — per-modifier content validation (uses existing `validate(&str)` API, same test pattern as existing tests):
1. `validate_capture_face_format` — capture with bad face format triggers E008. Input: raw capture modifier string with `.sd.sword:0:0:0:0:0` passed to `validate()`.
2. `validate_capture_face_id_range` — capture with face ID > 187 triggers E009. Input: raw capture modifier string with `.sd.999-1:0:0:0:0:0`.
3. `validate_legendary_hp_range` — legendary with hp > 999 triggers E010. Input: raw legendary modifier string with `.hp.1500`.
4. `validate_monster_floor_range` — monster with invalid floor range triggers E013. Input: raw monster modifier string with `5-3.monsterpool.` (5 > 3, invalid).
5. `validate_boss_level_range` — boss with level > 20 triggers E014. Input: raw boss modifier string with `ch.om25.fight.`.
6. `validate_capture_missing_sprite` — capture modifier raw text lacking `.img.` data or `.n.` name triggers W006 (note: validator checks raw text, not IR fields — "missing sprite" means no `.img.` present in the raw modifier).
7. `validate_legendary_missing_ability` — legendary modifier raw text lacking `.abilitydata.` triggers W007 (same principle: raw text check, not IR field check).
8. `validate_grouped_hero_no_wrapper_errors` — pansaer heroes produce zero E004/E005/E006 (wrapper rules skip grouped format — detected by absence of `ph.b` prefix).
9. `validate_abilitydata_face_format` — hero with `.abilitydata.(Fey.sd.999-1:0.img.spark.n.Test)` triggers E015 (face ID 999 out of range inside abilitydata). Input: full hero modifier string with that abilitydata embedded in a tier block.
10. `validate_abilitydata_valid_passes` — hero with valid `.abilitydata.(Fey.sd.34-1:30-1.img.spark.n.FireBlast)` produces no E015. Input: full hero modifier string with valid abilitydata.

Part B — cross-type reference validation (uses NEW `validate_cross_references(&ModIR)` API, tests construct ModIR manually):
11. `validate_cross_ref_heropool_missing_hero` — hand-construct `ModIR` with one hero (`internal_name: "warrior"`) and a structural modifier with `content: StructuralContent::HeroPoolBase { hero_refs: vec!["warrior".into(), "nonexistent".into()] }`. Call `validate_cross_references(&ir)`. Assert E016 fires for "nonexistent".
12. `validate_cross_ref_heropool_valid` — hand-construct `ModIR` where all hero_refs match existing hero `internal_name` values. Call `validate_cross_references(&ir)`. Assert zero E016 findings.
12a. `validate_cross_ref_case_insensitive` — hand-construct `ModIR` with hero `internal_name: "thief"` and structural modifier with `hero_refs: vec!["Thief".into()]`. Call `validate_cross_references(&ir)`. Assert zero E016 findings (case-insensitive match).
13. `validate_cross_ref_party_missing_member` — hand-construct `ModIR` with `StructuralContent::PartyConfig { party_name: "Team1".into(), members: vec!["nonexistent".into()] }` and no matching hero. Assert E016 fires.
14. `validate_all_mods_zero_errors` — validate all 4 working mods (pansaer, punpuns, sliceymon, community) with `validate()` — produces 0 errors each. This extends the existing 3-mod coverage to include community.txt.

**Parallel-safety note:** E016 tests construct `ModIR` with hand-built `StructuralContent` variants. They do NOT depend on Chunk 5's structural parser being wired — they only depend on Chunk 1's type definitions. E016's `validate_cross_references()` function is a new public API that does not modify or depend on the existing `validate()` function. These two validation paths are architecturally independent.

**Checkpoint:** `cargo check --lib` passes. `validate_cross_references()` compiles. Validator tests (items 1-14) in `tests/validator_tests.rs` can run in isolation via `cargo test --test validator_tests` (this file has no references to types broken by Chunk 1). Full `cargo test` still fails until Chunk 8. All 4 working mods validate with 0 errors (tested via `cargo test --test validator_tests`). `validate_cross_references()` passes its unit tests (items 11-13, 12a).

**If blocked:**
- Working mod produces false-positive errors → the mod is ground truth. Relax the rule, add TODO, report.
- Content validation on non-hero types finds real issues → report as warnings initially (new rules may need tuning).
- E016 needs `StructuralContent` variants that aren't available yet → tests use hand-constructed IR with those variants (types exist from Chunk 1). The function will be useful in production only after Chunk 5+7 wire structural parsing.

---

### Chunk 7: Builder Assembly + Routing

Wire all new emitters into builder. Wire new parsers into extractor routing. Wire `validate_cross_references` into public API and CLI. Note: CharSelect/DittoConfig removal, Monster/Boss raw Option fixups, and `.tiers` renames were already completed in Chunk 1.

**Read first:**
- `src/builder/mod.rs` (assembly logic after Chunk 1 — charselect/ditto stubs already removed, capture passthrough at ~line 89)
- `src/extractor/mod.rs` (extraction routing after Chunk 1 — charselect/ditto detection already removed)
- All new emitter files from Chunks 2-5: `capture_emitter.rs`, `monster_emitter.rs`, `boss_emitter.rs`
- All new parser files from Chunks 2-5: `structural_parser.rs`
- `src/builder/structural_emitter.rs` (updated in Chunk 5)
- `src/lib.rs` (public API — needs `validate_cross_references` re-export)
- `src/main.rs` (CLI Validate command — needs cross-reference validation wiring)

**Files:**
| Action | File |
|--------|------|
| Edit | `src/builder/mod.rs` -- add `pub mod capture_emitter;`, `pub mod monster_emitter;`, `pub mod boss_emitter;` declarations. Replace capture raw passthrough with `capture_emitter::emit_capture()` call, replace legendary raw passthrough with `capture_emitter::emit_legendary()` call, replace monster `if let Some(ref raw)` passthrough with `monster_emitter::emit_monster()` call, replace boss `if let Some(ref raw)` passthrough with `boss_emitter::emit_boss()` call. Also add PoolReplacement filter in structural emission loop (deferred from Chunk 2 to keep that chunk at 5 files). |
| Edit | `src/extractor/mod.rs` — add `pub mod structural_parser;` declaration. Wire `structural_parser::parse_structural_content()` into structural modifier construction (populate `name` and `content` fields on StructuralModifier for all structural match arms). |
| Edit | `src/lib.rs` — add `pub use validator::validate_cross_references;` to re-export the new function alongside existing `pub use validator::{validate, Finding, ValidationReport};`. |
| Edit | `src/main.rs` — in the `Commands::Validate` handler, after the existing `validate()` call succeeds, also call `let ir = textmod_compiler::extract(&textmod)?;` followed by `let xref_report = textmod_compiler::validate_cross_references(&ir);` and merge/print findings. Only run cross-reference validation when extraction succeeds (it depends on structural parsing from Chunk 5 being wired earlier in this chunk). |

**TDD:**
1. `build_from_ir_uses_capture_emitter` — build ModIR with capture (raw: None), output contains `itempool.`
2. `build_from_ir_uses_monster_emitter` — build ModIR with monster fields (raw: None), output contains `monsterpool.`
3. `build_from_ir_uses_boss_emitter` — build ModIR with boss fields (raw: None), output contains `.fight.`
4. `extract_wires_structural_parser` — extract any mod, structural items have content != Raw where applicable
5. `ditto_hero_stays_in_heroes` — extract sliceymon, hero with "Housecat" template is in heroes vec (not extracted to separate field)
6. `selector_stays_structural` — extract sliceymon, dabble.tier.0 modifier is in structural as Selector (not a separate CharSelect)
7. `no_charselect_field_on_modir` — grep verification: `grep -rn "charselect" compiler/src/ir/mod.rs` returns 0 results (Chunk 1 removed it; this confirms the removal persists through all subsequent chunks)
8. `no_ditto_field_on_modir` — grep verification: `grep -rn "ditto\|DittoConfig" compiler/src/ir/mod.rs` returns 0 results (Chunk 1 removed it; same rationale)
9. Deferred emitter inline `#[cfg(test)]` tests from Chunks 3-5 now compile and pass — run `cargo test --lib` and verify capture_emitter, monster_emitter, boss_emitter, and structural_parser tests all execute (they were written earlier but couldn't compile without the `pub mod` declarations wired in this chunk; use `--lib` because integration tests in `compiler/tests/` still reference old types until Chunk 8)
10. `validate_cross_references_in_public_api` — `textmod_compiler::validate_cross_references` is callable from lib.rs (verify with `use textmod_compiler::validate_cross_references;` in a test)
11. `cli_validate_runs_cross_references` -- run `cargo run -- validate working-mods/sliceymon.txt` and verify output includes cross-reference validation results (no errors expected)
12. `pool_replacement_emitted_in_structural` -- build ModIR with PoolReplacement structural modifier, verify it appears in output (deferred from Chunk 2)

**Checkpoint:** `cargo check --lib` and `cargo clippy --lib -- -D warnings` pass. All emitters wired. Deferred inline `#[cfg(test)]` tests from Chunks 3-5 now compile (module declarations added) and can run via `cargo test --lib`. Clean integration test files (`capture_tests`, `validator_tests`) can run via `cargo test --test capture_tests --test validator_tests`. Full `cargo test` still fails until Chunk 8 fixes remaining test files. CLI `main.rs` compiles and `cargo run -- validate working-mods/sliceymon.txt` runs successfully.

**If blocked:**
- New emitter produces different output than raw → use raw fallback (always available). Report discrepancy.
- Module import order issues → follow existing pattern in lib.rs/mod.rs.

---

### Chunk 8: Test Updates + Integration + Round-Trip

Mechanical test renames, raw->Option fixups in test constructors, and new integration tests. Round-trip validation on all 4 mods.

**Note:** This chunk touches 6 files (exceeds the 5-file guideline). This is necessary because the type renames (HeroTier->HeroBlock, .tiers->.blocks), type removals (CharSelect, DittoConfig), and raw->Option<String> changes propagate across all test files that construct IR types. Splitting would leave intermediate states that don't compile.

**Read first:**
- `tests/hero_tests.rs` (28 `.tiers` accessor references + 3 `tiers` word-boundary references in strings/comments = 31 total; use `grep -rwn "tiers"` to catch all forms — largest rename surface)
- `tests/builder_tests.rs` (3 `HeroTier` constructors at lines 23, 40, 57 + 2 `tiers:` field initializers at lines 22, 147 + `charselect: None` at lines 213, 260 + `ditto: None` at lines 214, 261 + Monster `raw: "wooper-raw".to_string()` at line 242 + Boss `raw: "boss-raw-content".to_string()` at line 248 — both break when raw becomes Option<String>)
- `tests/expansion_tests.rs` (1 `HeroTier` reference at line 14 + `tiers:` at line 14 + `.tiers[0]` at lines 61, 69 + full `merge_replaces_charselect` test at lines 120-140 constructing `CharSelect` objects + Monster `raw: String` constructors at lines 157, 172, 183 that break when raw becomes Option<String>)
- `tests/ir_tests.rs` (2 `HeroTier` references at lines 11, 48-49 + `.tiers` at line 11 + `charselect: None` at line 36 + `ditto: None` at line 37 + `charselect`/`ditto` assertions at lines 132-133)
- `tests/roundtrip_tests.rs` (2 `.tiers.len()` references at lines 74-75 + charselect presence checks at lines 93-97 + ditto presence checks at lines 101-105, 157-161, 166)
- `tests/boss_tests.rs` (ZERO HeroTier/.tiers/charselect/ditto references, BUT has `raw: String` breakage: line 79 `assert_eq!(&mon.raw, modifier)` and line 88 `assert_eq!(&boss.raw, modifier)` break when `raw` becomes `Option<String>` — must change to `assert_eq!(mon.raw.as_ref().unwrap(), modifier)` and `assert_eq!(boss.raw.as_ref().unwrap(), modifier)`)
- Verified: `capture_tests.rs`, `classifier_tests.rs`, `splitter_tests.rs`, `validator_tests.rs` have ZERO HeroTier/.tiers/charselect/ditto references and no raw type breakage

**Files:**
| Action | File |
|--------|------|
| Edit | `tests/hero_tests.rs` — `.tiers` -> `.blocks` (28 `.tiers` accessor sites + 3 `tiers` word-boundary sites = 31 total; use `grep -rwn "tiers"` to catch all forms) |
| Edit | `tests/builder_tests.rs` — HeroTier→HeroBlock (3 sites), `tiers:`→`blocks:` (2 sites at lines 22, 147), .tiers→.blocks, add `format: HeroFormat::Sliceymon` to Hero constructors, add `color: Option<char>` to HeroBlock constructors, remove `charselect: None` (lines 213, 260), remove `ditto: None` (lines 214, 261), **fix Monster/Boss raw constructors**: `raw: "wooper-raw".to_string()` → `raw: Some("wooper-raw".to_string())` (line 242) and `raw: "boss-raw-content".to_string()` → `raw: Some("boss-raw-content".to_string())` (line 248), add new Boss fields (`template: None, hp: None, sd: None, sprite_name: None, doc: None, modifier_chain: None, fight_units: vec![]`) to Boss constructors, update `StructuralModifier { modifier_type: ..., raw: ... }` constructors to use `StructuralModifier::new_raw(...)` (3 sites at lines 209, 251, 255) |
| Edit | `tests/expansion_tests.rs` — HeroTier→HeroBlock (1 site at line 14), `tiers:`→`blocks:` (1 site at line 14), .tiers→.blocks (2 sites at lines 61, 69), add `format`/`color` fields to constructors, **REMOVE entire `merge_replaces_charselect` test (lines 120-140)** which constructs `CharSelect` objects that no longer exist, remove `charselect: None`/`ditto: None` from any remaining ModIR constructors, **fix Monster raw constructors**: `raw: "wooper-raw".to_string()` → `raw: Some("wooper-raw".to_string())` etc. (lines 157, 172, 183), add new Monster fields (`color: None, modifier_chain: None`) and Boss fields to constructors, update `StructuralModifier { modifier_type: ..., raw: ... }` constructors to use `StructuralModifier::new_raw(...)` (4 sites at lines 199, 203, 211, 239), **update `merge_structural_replaces_by_type` test** (lines 196-232) to use `StructuralModifier::new_raw()` constructors and verify `(modifier_type, name)` matching, **add new `merge_structural_replaces_by_type_and_name` test** — construct base with two Dialogs (name: Some("Credits"), name: Some("Intro")), overlay replaces only the "Credits" dialog, assert both still exist with correct content |
| Verify | `tests/boss_tests.rs` — raw assertion fixes already applied in Chunk 4 (lines 79, 88). Verify `cargo test --test boss_tests` still passes after Chunk 8's other test updates. No edits needed here. |
| Edit | `tests/ir_tests.rs` — HeroTier→HeroBlock (2 sites), .tiers→.blocks (1 site), add `format`/`color` fields, update serde round-trip test to include new fields, remove `charselect: None` (line 36) and `ditto: None` (line 37) from ModIR constructors, remove charselect/ditto assertions (lines 132-133) |
| Edit | `tests/roundtrip_tests.rs` — `.tiers.len()` → `.blocks.len()` (lines 74-75), **remove charselect presence checks** (lines 93-97), **remove ditto presence checks** (lines 101-105), update/remove ditto summary output (lines 157-161, 166), add round-trip tests for all 4 mods |

**TDD:**
1. All existing tests pass with renames (regression gate — some tests like `merge_replaces_charselect` are deleted, not renamed)
2. `roundtrip_sliceymon` — extract -> build -> extract produces identical IR
3. `roundtrip_pansaer` — same
4. `roundtrip_punpuns` — same
5. `roundtrip_community` — same
6. `serde_roundtrip_hero_with_format` — serialize Hero with format field, deserialize, assert equal
7. `serde_roundtrip_structural_with_content` — serialize StructuralModifier with content, deserialize, assert equal
8. `grep -rn "HeroTier" compiler/tests/` returns 0 results
9. `grep -rn "\.tiers" compiler/tests/` returns 0 results
9a. `grep -rwn "tiers" compiler/tests/` returns 0 results (word-boundary grep catches ALL forms: `tiers:`, `.tiers`, `HeroTier` contains "tiers" substring — use this as a safety net after items 8-9)
10. `grep -rn "CharSelect\|DittoConfig\|DittoForm\|charselect\|ditto" compiler/tests/` returns 0 results
10a. Verify no bare `raw: "..."` Monster/Boss constructors remain: `grep -n "raw:" compiler/tests/boss_tests.rs compiler/tests/builder_tests.rs compiler/tests/expansion_tests.rs` shows only `raw: Some(...)`, `raw: None`, or structural `raw: "..."` (structural modifiers keep `raw: String`, not `Option`)
11. Run all deferred emitter tests from Chunks 3-5 (capture_emitter, monster_emitter, boss_emitter, structural_parser inline `#[cfg(test)]` tests) — these become active now that module declarations are wired in Chunk 7
12. `roundtrip_monsters_raw` and `roundtrip_bosses_raw` tests in boss_tests.rs pass with Option<String> raw comparisons
13. `merge_structural_replaces_by_type_and_name` — base has two Dialogs with different names, overlay replaces one by `(modifier_type, name)` match, other is preserved unchanged

**Checkpoint:** `cargo test` passes with 0 failures. `cargo clippy -- -D warnings` clean. All 4 mods round-trip.

**If blocked:**
- Round-trip mismatch on a mod → the extraction or emission has a bug. Debug by comparing IR JSON of extract(original) vs extract(rebuild). Fix the parser or emitter that diverges.
- Test file has more .tiers/charselect/ditto references than expected → run grep commands from TDD items 8-10 and fix all hits.
- Deferred emitter tests fail → debug the emitter (created in Chunks 3-4) since this is the first time they compile and run.

## Reuse

Existing utilities in `src/util.rs` that all new parsers should use:
- `extract_mn_name()`, `extract_last_n_name()` — name extraction
- `extract_sd()`, `extract_hp()`, `extract_color()`, `extract_template()` — property extraction
- `extract_nested_prop()` — for .abilitydata.(), .triggerhpdata.()
- `split_at_depth0()` — tier/block splitting
- `find_at_depth0()`, `find_matching_close_paren()` — depth-aware search
- `verify_paren_balance()`, `verify_ascii_only()` — output verification

Hero parser helpers reusable in other parsers:
- `extract_simple_prop()` — extract value between marker and next property boundary
- `extract_modifier_chain()` — extract .i./.sticker. chains
- `extract_facades_from_chain()` — extract facade names
- `find_next_prop_boundary()` — scan for next property marker

**Move hero_parser helpers to util.rs in Chunk 2** — `extract_simple_prop()`, `extract_modifier_chain()`, `extract_facades_from_chain()`, and `find_next_prop_boundary()` are needed by capture, monster, and boss parsers in Chunks 3 and 4. Moving them in Chunk 2 (which already edits hero_parser.rs) makes them available for parallel chunks.

## Self-Verification Checklist

### Type System
- [ ] `HeroTier` does not exist anywhere in the codebase
- [ ] `hero.tiers` does not exist anywhere (check `.tiers` accessors AND `tiers:` field initializers)
- [ ] `HeroFormat` enum exists with Sliceymon/Grouped/Unknown variants
- [ ] `StructuralContent` enum exists with all 11 variants (HeroPoolBase, ItemPool, BossModifier, PartyConfig, EventModifier, Dialog, Selector, GenSelect, LevelUpAction, PoolReplacement, Raw)
- [ ] All new IR fields have correct serde annotations

### Parsing Completeness
- [ ] Heroes: `blocks` populated for sliceymon AND grouped formats (all 4 mods)
- [ ] Captures: all 11 non-raw fields populated where data exists (pokemon, ball_name, ball_tier, template, hp, sd, sprite_name, color, item_modifiers, sticker, toggle_flags)
- [ ] Legendaries: all 11 non-raw fields populated where data exists (pokemon, summoning_item, template, hp, sd, sprite_name, color, doc, speech, abilitydata, item_modifiers)
- [ ] Monsters: sprite_name, color, doc, modifier_chain populated where present. `raw` is `Option<String>`
- [ ] Bosses: template, hp, sd, sprite_name, doc, fight_units populated where present. `raw` is `Option<String>`
- [ ] Boss fight_units: each unit has template and name
- [ ] No CharSelect/DittoConfig types in codebase — check BOTH `compiler/src/` AND `compiler/tests/` (removed — selector is structural, ditto is a normal hero)
- [ ] "Ditto" hero (Housecat template) stays in ModIR.heroes, not a separate field
- [ ] Selector modifiers (including dabble.tier.0) stay in ModIR.structural
- [ ] Structural: name populated, content != Raw for all types except EndScreen and Unknown
- [ ] Difficulty parsed as `Selector { options }`, ArtCredits parsed as `Dialog { phase }` (StructuralType carries semantic distinction)
- [ ] PoolReplacement parsed as `PoolReplacement { hero_names }` with extracted hero name list

### Emission
- [ ] Hero emitter handles Sliceymon and Grouped formats
- [ ] Sliceymon emitter uses `block.color.unwrap_or(hero.color)` for per-block color override
- [ ] Capture emitter reconstructs valid itempool modifier
- [ ] Legendary emitter reconstructs valid legendary modifier
- [ ] Monster emitter reconstructs valid monsterpool modifier
- [ ] Boss emitter reconstructs valid .fight. modifier
- [ ] Structural emitter handles all content variants
- [ ] No charselect_emitter.rs or ditto_emitter.rs exists
- [ ] All emitters fall back to raw when available

### Validation
- [ ] Content rules (E008-E012, W001-W002) fire on ALL types with .sd./.hp.
- [ ] Wrapper rules (E004-E006) only fire on Sliceymon heroes
- [ ] New rules E013/E014/E015/E016/W006/W007 fire correctly
- [ ] E015 validates face IDs inside `.abilitydata.(...)` content (only when `.sd.` is present inside; skips abilitydata without `.sd.`)
- [ ] E016 validates cross-type references via separate `validate_cross_references(&ModIR)` public function (heropool hero_refs, party members resolve to existing heroes)
- [ ] All 4 working mods produce 0 errors

### Quality
- [ ] `cargo test` passes with 0 failures
- [ ] `cargo clippy -- -D warnings` clean
- [ ] No `unwrap()` or `panic!()` in new library code
- [ ] All new **emitters** return `Result<String, CompilerError>` using existing `BuildError { component, message }` variant (NOT type-specific error variants -- use `component: format!("capture:{}", name)` etc.). Exception: `structural_emitter::emit()` returns `String` because structural modifiers always have `raw: String` as infallible fallback.
- [ ] All new **parsers** use bare return types with internal fallback-on-error (match hero_parser pattern: try-parse internally, populate defaults on failure, never crash)
- [ ] `validate_cross_references(&ModIR)` is a separate public function from `validate(&str)` -- two distinct validation APIs
- [ ] `validate_cross_references` is re-exported in `lib.rs` and called from CLI `Validate` command
- [ ] E016 matches hero_refs against `hero.internal_name` using case-insensitive comparison (not `mn_name` -- heropool refs use template-based names; grouped heroes have lowercase `internal_name` but Title Case refs in raw text)
- [ ] No `std::fs` in library code (WASM-safe)
- [ ] Serde roundtrip works for all modified types
- [ ] Round-trip test passes for all 4 working mods
- [ ] No `raw: "...".to_string()` patterns remain for Monster/Boss constructors in tests (all must use `raw: Some(...)` or `raw: None`)
- [ ] `boss_tests.rs` raw comparisons use `.as_ref().unwrap()` (not direct `&mon.raw` / `&boss.raw`)
- [ ] Structural merge matches by `(modifier_type, name)` pair, not `modifier_type` alone — two same-type modifiers with different names are independently replaceable

## Checkpoint Configuration

- **Total chunks:** 8
- **Checkpoint frequency:** After each chunk
- **Critical checkpoints:** After Chunk 1 (schema foundation), after Chunk 8 (integration)
