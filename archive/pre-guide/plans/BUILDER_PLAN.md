# Plan: Textmod Compiler in Rust (Extractor + Builder)

## Context

Building a **generic Slice & Dice textmod compiler** in Rust. It parses any textmod into a structured intermediate representation (IR) and compiles an IR back into a valid, pasteable textmod. Tested via round-trip against three real mods (pansaer, punpuns, sliceymon). Once proven, used to build the Sliceymon+ expansion.

**Why Rust**: Compiles to WASM for a future browser-based mod builder app. Strong type system makes the IR robust. Proper CLI tool, not project scripts.

**AI-friendly IR**: The IR format should be easy for Claude (or any LLM) to author directly from design documents. This means: plain JSON registry with no obscure encodings, hero files as simple single-line text, clear error messages from the builder when something is wrong. The workflow is: human writes design docs -> Claude reads docs and authors the IR -> compiler builds the textmod.

**Validation**: `extract(build(extract(mod))) == extract(mod)` -- semantic comparison of extracted IR, not string diffs. Tested against all 3 working mods.

## What's Already Done (JS, stays as-is for now)

- 26 hero generated files in `generated/` (plain text -- language-agnostic)
- 128 sprites in `tools/sprite_encodings.json`
- 7 hero configs in `tools/hero_configs/*.json` (subset of heroes; others are generated directly)
- These are just data files -- the Rust compiler reads them directly

## Out of Scope

- Porting sprite encoding tools to Rust (stays in JS for now)
- Porting `validate_textmod.js` to Rust (stays in JS; Rust tests may shell out to it as a secondary check but must not depend on it for core correctness)
- GUI / web frontend (future work after compiler is proven)
- Modifying the existing JS toolchain (it stays as-is)
- Auto-downloading sprites or game assets
- Supporting textmod formats from games other than Slice & Dice

## Textmod Format Discovery (Critical)

The three test mods use **three different delimiter formats**. The extractor MUST handle all of them:

| Mod | Format | Details |
|-----|--------|---------|
| **pansaer** | Single line, comma-separated | 350KB, 0 newlines, 76 modifiers split by `,` |
| **punpuns** | Alternating content/comma lines | 186 lines, 75 modifiers: modifier, `,`, modifier, `,`, ... |
| **sliceymon** | One comma-terminated modifier per line, blank spacers | 153 lines, 92 modifiers (most lines have 1 modifier, but lines 11, 99, 137, 147 contain multiple) |

**Implication**: The extractor's first job is to split raw text into modifiers. It must NOT assume newlines as delimiters. The canonical split strategy:

1. Split on commas at parenthesis depth 0 (handles all three formats)
2. Trim whitespace and newlines from each resulting modifier
3. Filter empty strings

This handles all three formats uniformly because ALL three use commas as delimiters:
- Pansaer: 76 modifiers on one line, commas between them, no newlines at all
- Punpuns: comma on its own line between modifiers (75 modifiers total)
- Sliceymon: comma-terminated, mostly one modifier per line but 4 lines contain multiple (92 modifiers across 77 non-empty lines)

**No format detection needed** -- always split on depth-0 commas, trim whitespace/newlines from each result, filter empty strings. This single strategy handles all three formats.

## Project Structure

```
compiler/
  Cargo.toml
  src/
    main.rs                    # CLI: extract / build subcommands
    lib.rs                     # Library root (WASM-compatible, no filesystem)
    error.rs                   # CompilerError type with position tracking
    ir/
      mod.rs                   # ModIR, Hero, Capture, Monster, etc.
      merge.rs                 # Overlay merge logic (IR + IR -> IR)
    extractor/
      mod.rs                   # Top-level: textmod string -> ModIR
      splitter.rs              # Raw text -> Vec<modifier strings> (depth-0 comma splitting)
      classifier.rs            # Classify modifier strings by type
      hero_parser.rs           # Parse hero modifier -> Hero struct
      capture_parser.rs        # Parse capture modifier -> Capture struct
      monster_parser.rs        # Parse monster modifier -> Monster struct
      boss_parser.rs           # Parse boss modifier -> Boss struct
    builder/
      mod.rs                   # Top-level: ModIR -> textmod string
      hero_emitter.rs          # Hero struct -> modifier string
      charselect_emitter.rs    # CharSelect -> modifier string
      ditto_emitter.rs         # DittoConfig -> modifier string
      capture_emitter.rs       # Capture struct -> modifier string
      monster_emitter.rs       # Monster struct -> modifier string
      boss_emitter.rs          # Boss struct -> modifier string
      structural_emitter.rs    # StructuralModifier -> modifier string
  tests/
    splitter_tests.rs          # Modifier splitting across all 3 formats
    classifier_tests.rs        # Modifier classification
    hero_tests.rs              # Hero parse + emit round-trip
    capture_tests.rs           # Capture parse + emit
    boss_tests.rs              # Boss + monster parse + emit
    roundtrip_tests.rs         # Full round-trip on pansaer, punpuns, sliceymon
    expansion_tests.rs         # Sliceymon+ specific tests
```

## CLI Interface

```bash
# Extract a textmod into IR (directory of structured files)
textmod-compiler extract sliceymon.txt --output ir/sliceymon/

# Build a textmod from an IR directory (extracted or hand-authored)
textmod-compiler build ir/sliceymon/ --output sliceymon_rebuilt.txt

# Build with overlay (base IR + expansion changes merged)
textmod-compiler build ir/sliceymon/ --overlay ir/sliceymon_plus/ --output textmod_expanded.txt

# Build from scratch (no extraction needed -- just a registry + component files)
textmod-compiler build my_mod/ --output my_mod.txt

# Validate only (extract + re-extract, report differences)
textmod-compiler validate sliceymon.txt
```

The builder doesn't care whether the IR was produced by the extractor, authored by hand, or generated by an LLM. A mod built from scratch just needs a `registry.json` and the component files it references (hero files, structural files, sprite data). The extractor is one way to produce an IR -- but not the only way.

**Optimized for LLM authoring**: The IR format is designed so that Claude can read a design doc (like `hero_designs_batch1.md`) and directly produce the registry entries and hero files. The builder then validates and compiles, catching format errors with clear messages that Claude can fix in a feedback loop:

```
Claude reads design docs -> writes registry.json + hero files
    -> compiler build -> errors? -> Claude fixes -> rebuild -> success
```

## Error Design

All errors carry context sufficient for an LLM to fix the issue without additional investigation.

```rust
#[derive(Debug)]
pub enum CompilerError {
    /// Modifier splitting failed
    SplitError { raw_position: usize, message: String },

    /// Modifier could not be classified
    ClassifyError { modifier_index: usize, preview: String, message: String },

    /// Hero parsing failed
    HeroParseError {
        modifier_index: usize,
        hero_name: String,
        tier_index: Option<usize>,
        position: usize,       // character position within modifier
        expected: String,
        found: String,
    },

    /// Parenthesis balance error (critical -- game silently rejects)
    ParenError {
        modifier_index: usize,
        position: usize,
        depth: i32,
        context: String,       // surrounding 40 chars
    },

    /// Builder emission error
    BuildError { component: String, message: String },

    /// Overlay merge conflict
    MergeConflict { key: String, base_value: String, overlay_value: String },

    /// Sprite resolution failed (sprite_name not found in registry)
    SpriteNotFound { sprite_name: String, hero_name: String, tier_index: usize },

    /// IR validation error (structural invariant violated)
    ValidationError { message: String },

    /// IO error (CLI only -- never in library code)
    IoError(std::io::Error),
}
```

Errors implement `Display` with human-readable messages that include the fix hint:
```
HeroParseError: hero "torchic", tier 2: expected ".tier." at position 847, found ".col."
  hint: property order must be template -> col -> tier -> hp -> sd -> img -> ...
```

## Overlay Merge Strategy

Overlays allow building an expanded mod from a base IR plus additions/replacements.

### Merge Rules

1. **Heroes**: Overlay heroes REPLACE base heroes with the same `internal_name`. New heroes are ADDED. To remove a hero, the overlay sets `removed: true` on that hero entry in the registry.
2. **Captures**: Same as heroes -- match by `pokemon` name, replace or add.
3. **Monsters**: Same -- match by `name`, replace or add.
4. **Bosses**: Same -- match by `name`, replace or add.
5. **Structural**: Overlay structural modifiers REPLACE base modifiers with matching `modifier_type`. If the overlay has `StructuralType::Unknown`, it is appended (not merged).
6. **CharSelect**: Overlay's CharSelect completely replaces base CharSelect (if present).
7. **Ditto**: Overlay's DittoConfig completely replaces base DittoConfig (if present).
8. **Conflict resolution**: If a merge conflict is ambiguous, emit `MergeConflict` error with both values. The user must resolve it in the overlay.

### Merge Implementation

```rust
// ir/merge.rs
pub fn merge(base: ModIR, overlay: ModIR) -> Result<ModIR, CompilerError> { ... }
```

The merge function is pure (no IO), operates on owned `ModIR` values, and is WASM-safe.

## Mod IR Types

```rust
// -- Top-level IR --

pub struct ModIR {
    pub heroes: Vec<Hero>,
    pub captures: Vec<Capture>,
    pub legendaries: Vec<Legendary>,
    pub monsters: Vec<Monster>,
    pub bosses: Vec<Boss>,
    pub structural: Vec<StructuralModifier>,
    pub charselect: Option<CharSelect>,
    pub ditto: Option<DittoConfig>,
}

// -- Heroes --

pub struct Hero {
    pub internal_name: String,       // "snorunt"
    pub mn_name: String,             // "Snorunt"
    pub color: char,                 // 'b' -- consistent across all tiers
    pub tiers: Vec<HeroTier>,        // variable count: typically 5 (T1, T2a, T2b, T3a, T3b)
                                     // but varies: Eevee=17, Scyther=6, Rockruff=6, etc.
    pub removed: bool,               // for overlay: marks hero for removal
    pub raw: Option<String>,         // preserve original if not re-emitting
}

pub struct HeroTier {
    pub template: String,            // "Eccentric", "Thief", "Lost", "Statue"
    pub tier: Option<u8>,            // None = omitted, Some(1), Some(2), Some(3)
                                     // NOTE: T1 sometimes has explicit .tier.1 (e.g., Ditto),
                                     // T2 sometimes omits .tier. entirely. Don't assume a rule.
    pub hp: u16,
    pub sd: String,                  // "93-2:93-2:56-1:56-1:0:0"
    pub sprite_name: String,         // "Snorunt" -- resolved to img encoding at build time
    pub speech: String,              // "Snor!~Runt!"
    pub name: String,                // display name "Snorunt"
    pub doc: Option<String>,         // tooltip doc string
    pub abilitydata: Option<String>, // spell definition data
    pub triggerhpdata: Option<String>, // HP trigger effect data
    pub hue: Option<String>,         // sprite hue override e.g. "10"
    // --- Modifier chain (order-sensitive) ---
    // Heroes have complex interleaved .i./.k./.facade./.sticker. chains.
    // Example: .i.left.k.scared#facade.bas170:55.i.col.k.pain#facade.eba3:0:20:0
    // A single hero can have MULTIPLE facades (Gible has 2, Porygon has 15!).
    // The game processes these left-to-right, so order matters.
    // We store the full modifier chain as a single string to preserve ordering,
    // plus extract facade values separately for queryability.
    pub modifier_chain: Option<String>, // full .i./.k./.facade./.sticker. chain, order-preserved
    pub facades: Vec<String>,        // extracted facade values: ["bas170:55", "eba3:0:20:0"]
    // --- Compound hero support ---
    pub items_inside: Option<String>,  // for compound heroes (Porygon-style nested items)
    pub items_outside: Option<String>, // items after the replica close paren
}

// NOTE: Heroes in the IR use `sprite_name` (e.g., "Snorunt"), not raw img encodings.
// The builder resolves sprite_name -> img encoding from sprite_encodings.json at build time.
// This keeps hero files readable and authorable by humans/LLMs -- no one needs to touch
// the 200+ char sprite strings directly.

// -- Character Selection --

pub struct CharSelect {
    pub phases: Vec<CharSelectPhase>,
    pub raw: Option<String>,         // preserve original if not re-emitting
}

pub struct CharSelectPhase {
    pub entries: Vec<CharSelectEntry>,
}

pub struct CharSelectEntry {
    pub color: char,                 // 'a'
    pub color_name: String,          // "amber"
    pub pokemon_name: String,        // "gible"
    pub sprite_name: String,         // resolved at build time
    pub variable: String,            // "gibleV1"
}

// -- Ditto --

pub struct DittoConfig {
    pub t3_forms: Vec<DittoForm>,    // one per hero's T3 variants
    pub raw: Option<String>,
}

pub struct DittoForm {
    pub hero_name: String,           // "Garchomp"
    pub template: String,            // "Lost"
    pub color: char,
    pub hp: u16,
    pub sd: String,
    pub sprite_name: String,
    pub modifier_chain: Option<String>, // .i./.k./.facade. chain (consistent with HeroTier)
    pub facades: Vec<String>,        // extracted facade values
}

// -- Captures --
// Capture lines are deeply nested itempool structures. The IR preserves the semantic
// meaning but the raw format is complex (hat/replica/togfri/ritemx/sticker chains).

pub struct Capture {
    pub pokemon: String,             // "Ivysaur"
    pub ball_name: String,           // "Great Ball" -- extracted from .n. of the ball item
    pub ball_tier: Option<u8>,       // item tier 0-9 (e.g., Mew=3, Jirachi=4, Kangaskhan=5)
                                     // expansion plan assigns specific tiers to new captures
    pub template: String,            // "Thief" (the replica base)
    pub hp: Option<u16>,             // some captures have HP
    pub sd: String,                  // dice faces
    pub sprite_name: String,         // resolved at build time
    pub item_modifiers: Option<String>, // .i./.k. chains
    pub sticker: Option<String>,     // sticker data
    pub toggle_flags: Option<String>, // togfri/togkey/togres etc.
    pub raw: Option<String>,         // fallback: preserve raw if parsing is incomplete
}

// -- Legendaries --

pub struct Legendary {
    pub pokemon: String,             // "Ho-Oh"
    pub summoning_item: String,      // "Rainbow Wing"
    pub template: String,            // "Thief"
    pub hp: Option<u16>,
    pub sd: String,
    pub sprite_name: String,
    pub abilitydata: Option<String>, // spell data
    pub item_modifiers: Option<String>,
    pub raw: Option<String>,
}

// -- Monsters --
// Monster lines are complex nested structures with floor ranges, balance modifiers,
// template bases, and optional nested items. Given complexity, monsters use a
// semi-structured approach: parsed floor range + name + base template, with the
// detailed modifier body preserved as raw text for now.

pub struct Monster {
    pub name: String,                // "Wooper"
    pub base_template: String,       // "Slimelet", "Bones", "Shade"
    pub floor_range: String,         // "1-3", "9-11", "17-19"
    pub hp: Option<u16>,
    pub sd: Option<String>,
    pub sprite_name: Option<String>,
    pub balance: Option<String>,     // .bal. modifier
    pub doc: Option<String>,
    pub raw: String,                 // full raw modifier (always preserved)
}

// -- Bosses --
// Boss lines are the most complex modifiers: combat definitions with multiple enemies,
// phase transitions, nested items, summon mechanics. Always preserved as raw text
// with minimal semantic extraction for indexing/overlay purposes.

pub struct Boss {
    pub name: String,                // "Quagsire", "Exeggutor", "Xerneas"
    pub level: Option<u8>,           // 4, 8, 12, 16, 20
    pub variant: Option<String>,     // "gen6", "gen7", "x-variant", "y-variant"
    pub raw: String,                 // full raw modifier (always preserved)
}

// -- Structural Modifiers --

pub struct StructuralModifier {
    pub modifier_type: StructuralType,
    pub raw: String,                 // always preserved as raw text
}

pub enum StructuralType {
    PartyConfig,        // =party.(...) -- starting heroes
    EventModifier,      // =Skip All&... or =... (non-party = prefixed modifiers)
    Dialog,             // ph.4 messages
    HeroPoolBase,       // heropool template list (no heroes, just base templates)
    LevelUpAction,      // hidden level-up trigger
    ItemPool,           // TM items, consumables, held items, eggs
    Selector,           // ph.s selection menus (layout, starting levels, etc.)
    GenSelect,          // Boss generation selection menu (a specific selector)
    Difficulty,         // Difficulty selection (a specific selector)
    EndScreen,          // End screen / send teams
    BossModifier,       // Boss behavior modifiers (no flee, horde)
    ArtCredits,         // Art/community credits
    Unknown,            // Unrecognized -- preserved verbatim
}
```

## WASM Compatibility Rules

The `lib.rs` module and everything it depends on (ir/, extractor/, builder/) must be WASM-compatible:

1. **No `std::fs`** in library code. All IO happens in `main.rs` (CLI only).
2. **No `glob` crate** in library code. File discovery is a CLI concern.
3. **No `std::process`** in library code. Shelling out to validators is a CLI/test concern.
4. **All core functions take `&str` input and return `String` or `Result<T, CompilerError>`**.
5. **Sprite resolution** takes a `HashMap<String, String>` (name -> encoding), not a file path.

The public API surface for WASM:

```rust
// lib.rs -- WASM-safe public API
pub fn extract(textmod: &str) -> Result<ModIR, CompilerError>;
pub fn build(ir: &ModIR, sprites: &HashMap<String, String>) -> Result<String, CompilerError>;
pub fn merge(base: ModIR, overlay: ModIR) -> Result<ModIR, CompilerError>;
pub fn ir_to_json(ir: &ModIR) -> Result<String, serde_json::Error>;
pub fn ir_from_json(json: &str) -> Result<ModIR, serde_json::Error>;
```

`main.rs` handles all filesystem operations: reading files, discovering files in directories, writing output. It calls the library functions with string data.

## TDD Progression

### Checkpoint Configuration

- **Total chunks**: 8
- **Checkpoint frequency**: Every 2 chunks
- **Critical checkpoints**: After chunk 3 (hero parser -- most complex), after chunk 6 (round-trip)

### Parallel Execution Map

```
Foundation (sequential): Chunk 1 (project + error types + IR)
                              |
                              v
                         Chunk 2 (splitter + classifier)
                              |
                              v
                         Chunk 3 (hero parser) [CRITICAL CHECKPOINT]
                              |
                              v
                         Chunk 4 (hero emitter + builder core)
                              |
                    +---------+---------+---------+
                    v         v         v         v
               charselect   ditto   captures  monsters+bosses
                  (5a)      (5b)     (5c)       (5d)
                    |         |         |         |
                    +---------+---------+---------+
                              |
                         Chunk 6 (round-trip tests) [CRITICAL CHECKPOINT]
                              |
                              v
                         Chunk 7 (overlay merge + expansion build)
                              |
                              v
                         Chunk 8 (in-game testing)

Minimum wall-clock rounds: 8 (chunk 5a-5d can run in parallel but all need chunk 4)
```

---

### Chunk 1: Foundation -- Project Setup, Error Types, IR Types

**Scope**: Create the Rust project, define all IR types with serde derives, define the error type, set up the library/CLI split.

**Files**:
- `compiler/Cargo.toml`
- `compiler/src/main.rs` (stub CLI with clap)
- `compiler/src/lib.rs` (public API stubs)
- `compiler/src/error.rs`
- `compiler/src/ir/mod.rs`

**Dependencies**: None

**Read first**:
- `SLICEYMON_AUDIT.md` (property codes, hero structure)
- `working-mods/sliceymon.txt` (real data shapes)
- This plan (IR types section)

**Requirements**:
- All IR types from the "Mod IR Types" section above
- `CompilerError` from the "Error Design" section above
- Serde `Serialize`/`Deserialize` on all IR types
- `lib.rs` exports public API stubs that return `todo!()`
- `main.rs` uses `clap` derive for `extract`/`build`/`validate` subcommands (stubs)
- No filesystem access in lib.rs

**TDD**:
1. `ir_types_serialize_roundtrip` -- create a minimal `ModIR`, serialize to JSON, deserialize, assert equal
2. `hero_tier_required_fields` -- verify `HeroTier` requires template, hp, sd, sprite_name, speech, name
3. `compiler_error_display` -- verify error Display impl includes position info and hint
4. `empty_mod_ir_serializes` -- empty ModIR (no heroes, etc.) serializes cleanly

**Verification**:
- [ ] `cargo build` succeeds
- [ ] `cargo test` passes (4 tests)
- [ ] No `std::fs` or `std::process` in lib.rs/ir/error.rs
- [ ] All IR types have `Serialize, Deserialize, Debug, Clone, PartialEq`

**If blocked**: If `clap` derive causes issues with WASM target, use plain arg parsing in main.rs and keep clap as a CLI-only dependency behind a feature flag.

---

### Chunk 2: Splitter + Classifier

**Scope**: Split raw textmod text into individual modifiers (handling all 3 formats), then classify each modifier by type.

**Files**:
- `compiler/src/extractor/mod.rs`
- `compiler/src/extractor/splitter.rs`
- `compiler/src/extractor/classifier.rs`
- `compiler/tests/splitter_tests.rs`
- `compiler/tests/classifier_tests.rs`

**Dependencies**: Chunk 1

**Read first**:
- `working-mods/pansaer.txt` (comma-separated, single line)
- `working-mods/punpuns.txt` (alternating content/comma lines)
- `working-mods/sliceymon.txt` (newline-separated with blank spacers)
- This plan, "Textmod Format Discovery" section

**Requirements**:
- `splitter::split_modifiers(text: &str) -> Vec<String>` handles all 3 formats
- Always split on depth-0 commas (all three test mods use commas as delimiters)
- Depth tracking for parentheses (commas inside parens are NOT delimiters)
- `classifier::classify(modifier: &str) -> Result<ModifierType, CompilerError>`
- Classification patterns (order matters -- first match wins):
  - Hero: contains `heropool` (case-insensitive) AND contains `replica.` (covers sliceymon `ph.b...heropool.(replica.`, punpuns `heropool.(replica.`, pansaer `Heropool.(replica.`)
  - HeroPoolBase: contains `heropool` (case-insensitive) AND does NOT contain `replica.` (template lists like `heropool.Thief+Scoundrel+...`)
  - Monster: contains `monsterpool.` (must check before Boss since both can have nested structures)
  - Boss: contains `.fight.` (covers `ch.om*.fight.`, `ph.b...fight.`, and direct fight modifiers)
  - Capture: starts with `itempool.` (case-insensitive) AND contains `hat.replica` (captures use hat+replica wrappers)
  - Legendary: starts with `itempool.` (case-insensitive) AND contains `cast.sthief` (legendary summons have cast+thief spell patterns)
  - ItemPool: starts with `itempool.` or `Itempool.` (remaining itempools after capture/legendary classification)
  - PartyConfig: starts with `=party.` (exact prefix match -- not just `=`)
  - EventModifier: starts with `=` but NOT `=party.` (e.g., `=Skip All&...` in pansaer)
  - Dialog: contains `.ph.4` (message dialogs)
  - Selector: contains `.ph.s` (selection menus -- maps to GenSelect, Difficulty, or structural)
  - BossModifier: contains boss behavior keywords (`no flee`, `horde`, etc.) but no `.fight.`
  - Unknown: anything else (preserved verbatim)

**TDD**:
1. `split_pansaer_gives_76_modifiers` -- split pansaer.txt, assert 76 modifiers
2. `split_punpuns_gives_75_modifiers` -- split punpuns.txt, assert 75 modifiers (depth-0 comma split)
3. `split_sliceymon_gives_92_modifiers` -- split sliceymon.txt, assert 92 modifiers
   (most lines have 1 modifier, but lines 11/99/137/147 contain multiple)
4. `split_respects_paren_depth` -- commas inside `(...)` are not split points
5. `classify_sliceymon_hero` -- a sliceymon hero line classifies as Hero
6. `classify_pansaer_hero` -- pansaer hero (`Heropool.(replica.` with capital H, no `ph.b`) classifies as Hero
7. `classify_sliceymon_capture` -- capture itempool line classifies as Capture
8. `classify_sliceymon_monster` -- monsterpool line classifies as Monster
9. `classify_sliceymon_boss` -- boss combat line classifies as Boss
10. `classify_all_sliceymon_no_unknowns` -- every sliceymon modifier classifies to a known type (no Unknown)
11. `classify_all_punpuns_no_panics` -- every punpuns modifier classifies without panic
12. `classify_all_pansaer_no_panics` -- every pansaer modifier classifies without panic

**Verification**:
- [ ] All 12 tests pass
- [ ] Each mod splits to the expected modifier count
- [ ] No `std::fs` in splitter.rs or classifier.rs (tests load files via test helpers)

**If blocked**: If pansaer uses commas inside modifier text (not just as delimiters), the depth-0 comma strategy may need refinement. Fallback: add a secondary heuristic that checks if comma-split fragments start with known modifier prefixes.

---

### Chunk 3: Hero Parser [CRITICAL CHECKPOINT]

**Scope**: Parse hero modifier strings into `Hero` structs. This is the most complex parser -- heroes have variable tiers (typically 5, but Missingno=1, Scyther=6, Eevee=17, Ditto sub-mods vary), nested parentheses, facade/triggerhpdata/sticker/item_modifiers, and vary significantly across mods.

**Files**:
- `compiler/src/extractor/hero_parser.rs`
- `compiler/tests/hero_tests.rs` (parse tests only; emit tests in chunk 4)

**Dependencies**: Chunks 1, 2

**Read first**:
- `SLICEYMON_AUDIT.md` (all 44 hero lines documented with properties)
- `generated/line_69_torchic.txt` (known-good generated hero line)
- `tools/hero_configs/torchic.json` (hero config showing expected parse result)
- `working-mods/sliceymon.txt` lines 13-99 (real hero data)

**Requirements**:
- Parse the `hidden&temporary&ph.b[name];1;!mheropool.` prefix (extract internal_name)
- Split into tier blocks at depth-0 `+` separators
- For each tier block, extract: template, color, tier, hp, sd, img (stored as raw for now), speech, name
- Also extract optional: facade, triggerhpdata, doc, abilitydata, hue, item_modifiers (.i./.k. chains), sticker
- Extract `.mn.` suffix (the mn_name)
- Extract `.part.1&hidden` suffix pattern
- Property extraction must be order-tolerant (different heroes have different property orders)
- Paren depth tracking at every step; emit `ParenError` if unbalanced
- Hero lines that fail to parse should populate `raw` field and emit a warning (not a hard error), allowing round-trip via raw passthrough

**TDD**:
1. `parse_simple_hero_no_spells` -- parse a hero with no abilitydata/facade/doc (e.g., Gible)
2. `parse_hero_with_spell` -- parse Torchic/Blaziken (has abilitydata)
3. `parse_hero_with_facade` -- parse Gible/Garchomp (has facade data)
4. `parse_hero_with_triggerhpdata` -- parse Turtwig (has triggerhpdata)
5. `parse_hero_with_doc` -- parse Larvitar (has doc string)
6. `parse_hero_with_hue` -- parse Larvesta (has hue on spell icons)
7. `parse_hero_with_sticker` -- parse Eevee (has sticker)
8. `parse_compound_hero` -- parse Porygon (has items_inside)
9. `parse_hero_tier_count_varies` -- verify tier counts: Gible=5, Scyther=6, Eevee=17 (not fixed at 5)
10. `parse_hero_paren_balanced` -- verify every tier block has balanced parens
11. `parse_all_sliceymon_heroes` -- parse all sliceymon hero modifiers: 42 regular heroes (ph.b+heropool), 1 Missingno (heropool only, no ph.b), 5 Ditto sub-modifiers. Assert 48 total Hero-classified modifiers parse.
12. `parse_hero_extracts_correct_hp` -- verify HP matches SLICEYMON_AUDIT for 5 known heroes
13. `parse_hero_extracts_correct_sd` -- verify sd matches SLICEYMON_AUDIT for 5 known heroes
14. `parse_hero_mn_name_correct` -- verify mn_name is extracted correctly
15. `parse_hero_multiple_facades` -- verify Gible parses with 2 facades, Porygon with 15
16. `parse_hero_modifier_chain_preserved` -- verify full .i./.k./.facade. chain is preserved in order
17. `parse_pansaer_hero` -- parse at least one pansaer hero (uses `Heropool.` with capital H, no `ph.b` wrapper)

**Verification**:
- [ ] All 17 tests pass
- [ ] All 48 sliceymon hero-classified modifiers parse without errors (42 regular + 1 Missingno + 5 Ditto)
- [ ] At least 1 pansaer hero parses without errors
- [ ] HP and sd values match SLICEYMON_AUDIT.md spot-checks
- [ ] Multi-facade heroes (Gible, Porygon) parse all facades
- [ ] No `std::fs` in hero_parser.rs

**If blocked**: If some heroes have unusual structures that break the parser, use the `raw` fallback: store the full modifier in `raw` and emit a warning. This preserves round-trip capability while flagging the issue.

---

### Chunk 4: Hero Emitter + Builder Core

**Scope**: Emit Hero structs back to modifier strings. Build the core assembly pipeline that combines all emitted modifiers into a complete textmod.

**Files**:
- `compiler/src/builder/mod.rs`
- `compiler/src/builder/hero_emitter.rs`
- `compiler/src/builder/structural_emitter.rs`
- `compiler/tests/builder_tests.rs` (hero emit + core assembly tests)

**Dependencies**: Chunks 1, 2, 3

**Read first**:
- `generated/line_69_torchic.txt` (known-good hero output format)
- `SLICEYMON_AUDIT.md` (property order, format rules)
- `tools/sprite_encodings.json` (sprite resolution)

**Requirements**:
- `hero_emitter::emit(hero: &Hero, sprites: &HashMap<String, String>) -> Result<String, CompilerError>`
- Resolves `sprite_name` to `.img.` encoding from sprites map
- Emits `SpriteNotFound` error if sprite missing
- Maintains exact property order: `replica.[Template].col.[color].tier.[N].hp.[N].sd.[faces].img.[sprite]...`
- `.n.NAME` is always last before `+` or line end
- `.speech.` is outside the replica parens
- Tier separators `+` at depth 0 (never nested)
- `.part.1&hidden.mn.[Name]@2!m(skip&hidden&temporary),` suffix
- If `hero.raw` is Some, emit raw directly (passthrough mode)
- `builder::build(ir: &ModIR, sprites: &HashMap<String, String>) -> Result<String, CompilerError>`
- Assembly order matches sliceymon convention: party, dialogs, charselect, heroes, level-up, items, captures, legendaries, monsters, bosses, boss modifiers, difficulty, end screen
- Output format: one modifier per line with blank spacer lines (sliceymon format)
- `structural_emitter::emit(s: &StructuralModifier) -> String` -- passthrough of raw text

**TDD**:
1. `emit_hero_paren_balanced` -- emit a hero, verify parens balanced
2. `emit_hero_tier_separators_at_depth_0` -- verify all `+` chars are at paren depth 0
3. `emit_hero_name_last` -- verify `.n.NAME` is last property before each `+`
4. `emit_hero_sprite_resolved` -- verify `.img.` contains actual encoding, not sprite_name
5. `emit_hero_sprite_not_found_error` -- emit with missing sprite, verify SpriteNotFound error
6. `emit_hero_matches_known_good` -- emit Torchic from parsed config, compare to generated/line_69_torchic.txt
7. `emit_hero_raw_passthrough` -- hero with `raw: Some(...)` emits raw text unchanged
8. `emit_hero_part1_suffix` -- verify .part.1&hidden on last T3, .mn.[Name]@2!m(...) at end
9. `emit_hero_modifier_chain` -- verify .i./.k./.facade. chains emitted in preserved order
10. `emit_hero_ascii_only` -- verify no non-ASCII characters in emitted output
11. `build_minimal_textmod` -- build ModIR with 1 hero + 1 structural, verify output is valid
12. `build_assembly_order` -- build with all component types, verify they appear in correct order
13. `build_mn_suffixes` -- verify every emitted modifier ends with .mn.SomeName,

**Verification**:
- [ ] All 13 tests pass
- [ ] Emitted Torchic matches known-good output exactly
- [ ] No `std::fs` in builder code (sprites passed as HashMap)

**If blocked**: If property order varies between heroes (it does -- some have `.tier.` omitted for T1), the emitter needs conditional logic per tier. Fall back to matching the original's property order if exact reproduction is needed.

---

### Chunk 5a: CharSelect Emitter

**Scope**: Generate character selection modifier from hero list.

**Files**:
- `compiler/src/builder/charselect_emitter.rs`

**Dependencies**: Chunk 4

**Read first**:
- `working-mods/sliceymon.txt` line 11 (charselect structure)
- `SLICEYMON_AUDIT.md` "Character Selection Draft System" section

**Requirements**:
- Generate the `1.ph.s...` selection line from `CharSelect` struct
- Each entry: `@1[color]X - pokemon@2!m(party.(dabble.tier.0.n.X.col.X.img.[mini_sprite]))@2!v[pokemon]V1`
- Entries sorted by color letter
- Phase 1 and Phase 2 entries
- If `CharSelect.raw` is Some, emit raw directly

**TDD**:
1. `charselect_has_all_colors` -- every hero color appears in charselect
2. `charselect_sorted_by_color` -- entries sorted alphabetically by color
3. `charselect_phase1_and_phase2` -- both phases present
4. `charselect_paren_balanced` -- output has balanced parens

**Verification**:
- [ ] All 4 tests pass

**If blocked**: If charselect format varies between mods (punpuns uses a different layout selection), use raw passthrough for non-sliceymon charselect. Only the sliceymon format needs full generation.

---

### Chunk 5b: Ditto Emitter

**Scope**: Generate Ditto modifier from hero T3 forms.

**Files**:
- `compiler/src/builder/ditto_emitter.rs`

**Dependencies**: Chunk 4

**Read first**:
- `working-mods/sliceymon.txt` line 99 (Ditto structure -- 66KB!)
- `SLICEYMON_AUDIT.md` Ditto section

**Requirements**:
- Generate the massive Ditto line containing every hero's T3 forms as Ditto transformations
- Each T3 form: replica with Ditto's color (w), sprite from original hero, stats from original
- If `DittoConfig.raw` is Some, emit raw directly

**TDD**:
1. `ditto_has_t3_for_every_hero` -- count of Ditto forms matches hero count * 2 (two T3 variants each)
2. `ditto_paren_balanced` -- the enormous Ditto line has balanced parens
3. `ditto_uses_white_color` -- all Ditto forms use `.col.w`
4. `ditto_raw_passthrough` -- DittoConfig with raw emits raw unchanged

**Verification**:
- [ ] All 4 tests pass

**If blocked**: If Ditto structure is too complex to generate from scratch, use raw passthrough from the extracted Ditto line and only regenerate when the expansion adds/removes heroes.

---

### Chunk 5c: Capture + Legendary Emitter

**Scope**: Parse and emit capture and legendary modifiers.

**Files**:
- `compiler/src/extractor/capture_parser.rs`
- `compiler/src/builder/capture_emitter.rs`
- `compiler/tests/capture_tests.rs`

**Dependencies**: Chunk 4

**Read first**:
- `working-mods/sliceymon.txt` lines 111, 113 (captures), lines 115, 117 (legendaries)
- `SLICEYMON_AUDIT.md` capture/legendary sections

**Requirements**:
- Parse `itempool.((hat.replica...` capture lines into Capture structs
- Parse `itempool.((hat.(replica...` legendary lines into Legendary structs
- Captures and legendaries with `raw: Some(...)` use raw passthrough
- For initial implementation, parsing extracts name/template/sd and preserves the rest in raw
- Emitter reconstructs from raw when available, from fields when not

**TDD**:
1. `parse_capture_extracts_pokemon_name` -- parse a capture line, verify pokemon name
2. `parse_capture_extracts_sd` -- parse a capture line, verify dice faces
3. `parse_legendary_extracts_name` -- parse a legendary line, verify name
4. `roundtrip_captures_raw` -- parse then emit captures, verify output matches original
5. `roundtrip_legendaries_raw` -- parse then emit legendaries, verify output matches original

**Verification**:
- [ ] All 5 tests pass
- [ ] Capture round-trip produces identical output

**If blocked**: If capture/legendary parsing proves too complex due to deep nesting, use raw-only mode: store entire modifier as `raw`, extract only the pokemon name via regex for indexing. Round-trip via raw passthrough.

---

### Chunk 5d: Monster + Boss Parser/Emitter

**Scope**: Parse and emit monster and boss modifiers. Both use raw passthrough with minimal semantic extraction.

**Files**:
- `compiler/src/extractor/monster_parser.rs`
- `compiler/src/extractor/boss_parser.rs`
- `compiler/src/builder/monster_emitter.rs`
- `compiler/src/builder/boss_emitter.rs`
- `compiler/tests/boss_tests.rs` (also contains monster tests -- combined to stay within file limit)

**Dependencies**: Chunk 4

**Read first**:
- `working-mods/sliceymon.txt` lines 119-129 (monsters), lines 131-149 (bosses)
- `working-mods/punpuns.txt` (many monster/boss lines for generality)
- `SLICEYMON_AUDIT.md` monster/boss sections

**Requirements**:
- `monster_parser`: extract name, base_template, floor_range, hp, sd from `(N-M.monsterpool.(...)` pattern. Store full raw text.
- `boss_parser`: extract name, level (from `ch.omN`), variant. Store full raw text.
- Both emitters use raw passthrough (monsters and bosses are too complex for full semantic parsing in v1)
- Pansaer boss lines (which use `.fight.` differently) must not panic

**TDD**:
1. `parse_monster_extracts_name` -- parse Wooper monster, verify name
2. `parse_monster_extracts_floor_range` -- verify "1-3" extracted
3. `parse_monster_extracts_template` -- verify "Slimelet" extracted
4. `parse_boss_extracts_name` -- parse Quagsire boss, verify name
5. `parse_boss_extracts_level` -- verify level 4 extracted
6. `roundtrip_monsters_raw` -- parse then emit, verify identical output
7. `roundtrip_bosses_raw` -- parse then emit, verify identical output
8. `parse_punpuns_monsters_no_panic` -- parse all punpuns monsters without panic

**Verification**:
- [ ] All 8 tests pass
- [ ] Monster and boss round-trips produce identical output

**If blocked**: Since both monsters and bosses use raw passthrough, parsing failures should be rare. If extraction of name/level fails for exotic formats, store `name: "unknown"` and rely on raw for round-trip.

---

### Chunk 6: Full Round-Trip Tests [CRITICAL CHECKPOINT]

**Scope**: Run the full `extract(build(extract(mod))) == extract(mod)` validation against all three working mods.

**Files**:
- `compiler/tests/roundtrip_tests.rs`
- `compiler/src/ir/merge.rs` (stub -- needed for chunk 7 but merge tests come later)

**Dependencies**: Chunks 1-5

**Read first**:
- `working-mods/pansaer.txt`
- `working-mods/punpuns.txt`
- `working-mods/sliceymon.txt`
- `tools/sprite_encodings.json`

**Requirements**:
- `roundtrip_pansaer`: extract -> build -> extract -> compare IRs
- `roundtrip_punpuns`: extract -> build -> extract -> compare IRs
- `roundtrip_sliceymon`: extract -> build -> extract -> compare IRs
- `assert_ir_equal` compares: hero count, hero names, hero HP/sd per tier, capture count, monster count, boss count, structural count
- For heroes using raw passthrough, raw strings must match exactly
- For heroes fully parsed, semantic fields must match (not string equality -- formatting may differ)
- Build output should also be manually inspectable (write to `test_output/` dir)

**TDD**:
1. `roundtrip_pansaer` -- full round-trip, assert IR equality
2. `roundtrip_punpuns` -- full round-trip, assert IR equality
3. `roundtrip_sliceymon` -- full round-trip, assert IR equality
4. `roundtrip_sliceymon_hero_count` -- verify 48 hero-classified modifiers extracted and re-extracted (42 regular + 1 Missingno + 5 Ditto)
5. `roundtrip_sliceymon_capture_count` -- verify capture count preserved
6. `roundtrip_sliceymon_boss_count` -- verify boss count preserved
7. `build_output_is_valid_text` -- built output contains no non-ASCII, balanced parens globally

**Verification**:
- [ ] All 7 tests pass
- [ ] All three mods round-trip cleanly
- [ ] Built output for sliceymon is inspectable and looks correct

**If blocked**: If round-trip fails for specific modifiers, add those modifiers to `raw` passthrough and log which ones need parser improvements. The round-trip invariant must hold even if some components use raw passthrough.

---

### Chunk 7: Overlay Merge + Expansion Build

**Scope**: Implement the overlay merge strategy and build the Sliceymon+ expansion.

**Files**:
- `compiler/src/ir/merge.rs` (full implementation)
- `compiler/tests/expansion_tests.rs`

**Dependencies**: Chunk 6

**Read first**:
- `plans/EXPANSION_PLAN.md` (full expansion specification)
- `plans/FULL_ROSTER.md` (hero/capture/monster/boss assignments)
- `generated/*.txt` (all 26 generated hero files)
- This plan, "Overlay Merge Strategy" section

**Requirements**:
- `merge(base: ModIR, overlay: ModIR) -> Result<ModIR, CompilerError>`
- Heroes: match by internal_name, replace or add. Removed heroes filtered out.
- Captures: match by pokemon name, replace or add.
- Monsters/bosses: match by name, replace or add.
- Structural: match by type, replace.
- CharSelect/Ditto: overlay replaces base entirely.
- MergeConflict error for ambiguous cases.
- Build Sliceymon+ by: extract sliceymon.txt -> merge with expansion overlay -> build

**TDD**:
1. `merge_adds_new_hero` -- overlay with new hero adds it to base
2. `merge_replaces_existing_hero` -- overlay hero with same name replaces base
3. `merge_removes_hero` -- overlay hero with removed=true removes from base
4. `merge_preserves_unmodified` -- base heroes not in overlay are preserved
5. `merge_replaces_charselect` -- overlay charselect replaces base
6. `expansion_all_25_color_letters` -- built expansion has heroes for all 25 color letters
   (A,B,C,D,E,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z -- no F; D=hidden Missingno;
   E and J are new expansion colors)
7. `expansion_removed_captures_absent` -- specifically: Ivysaur, Pikachu, Charizard,
   Metagross, Poliwag, and Arceus captures are all absent (per EXPANSION_PLAN Part 2)
8. `expansion_new_heroes_present` -- Charmander, Torchic, etc. are present
9. `expansion_passes_paren_check` -- entire built output has balanced parens
10. `build_from_scratch_no_extraction` -- build Sliceymon+ from hand-authored IR only (no extraction step)
11. `expansion_part_suffixes_correct` -- hero lines use .part.1&hidden, item/monster pools
    use .part.0 where appropriate
12. `expansion_mn_suffixes_present` -- every modifier ends with .mn.SomeName,
13. `expansion_no_duplicate_pokemon` -- no Pokemon in more than one pool per FULL_ROSTER.md

**Verification**:
- [ ] All 13 tests pass
- [ ] Expansion output has correct hero count (46 heroes across 25 colors)
- [ ] No duplicate Pokemon across hero/capture/monster/boss pools
- [ ] Built expansion output is ready for in-game testing

**If blocked**: If the expansion overlay is too complex to author immediately, start with a minimal overlay (add 1 hero, remove 1 capture) and expand iteratively.

---

### Chunk 8: In-Game Testing

**Scope**: Validate compiler output by pasting into the actual game on mobile.

**Files**: No new code files. Test artifacts:
- `test_output/sliceymon_rebuilt.txt` (round-trip rebuild)
- `test_output/sliceymon_plus.txt` (expansion build)

**Dependencies**: Chunk 7

**Requirements**:
- Paste `sliceymon_rebuilt.txt` into game -- verify it loads identically to original
- Paste `sliceymon_plus.txt` into game -- verify all new heroes appear
- Template isolation tests: paste individual hero modifiers to verify they work
- Full playthrough: play through a complete run with new heroes

**This chunk is manual -- no automated tests.** User pastes into game and reports issues.

**If blocked**: If paste fails, use binary search to isolate which modifier is causing the failure. Check against `validate_textmod.js` output for hints.

---

### Final Verification (After All Chunks)

- [ ] `cargo test` passes all tests (all chunks)
- [ ] `cargo build --target wasm32-unknown-unknown` compiles (WASM compatibility)
- [ ] Round-trip test passes for all 3 working mods (pansaer, punpuns, sliceymon)
- [ ] Sliceymon+ expansion builds successfully with correct hero count (46 heroes: 23 existing + 21 new + 2 new colors)
- [ ] No duplicate Pokemon across hero/capture/monster/boss pools in expansion output
- [ ] Expansion output pastes into game and loads without errors
- [ ] At least one full playthrough completed with new heroes
- [ ] No `std::fs`, `std::process`, or `glob` in any library code (grep check)

---

## Test Mods

| Mod | Location | Format | Modifiers | Notes |
|-----|----------|--------|-----------|-------|
| pansaer.txt | working-mods/ | Single-line, comma-separated | 76 | `Heropool.` (capital H), no ph.b wrapper, uses 38 unique templates, extensive template coverage |
| punpuns.txt | working-mods/ | Alternating content/comma lines | 75 | Most boss/monster lines of any test mod |
| sliceymon.txt | working-mods/ | Newline-separated, comma-terminated, blank spacers | 92 | Our base mod: Ditto + captures + legendaries + monsters |

## Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
clap = { version = "4", features = ["derive"] }

[dev-dependencies]
assert_cmd = "2"
```

**Note**: `clap` is behind a target gate so it's excluded from WASM builds. The `glob` crate is NOT used -- file discovery uses `std::fs::read_dir` in `main.rs` only. `assert_cmd` is dev-only for optional integration tests that shell out to the JS validator.

## What Stays in JS (for now)

- `tools/generate_hero.js` -- config-driven hero line generator (output consumed by Rust compiler)
- `tools/encode_sprite.js` / `tools/batch_sprites.js` -- sprite encoding (could port later)
- `tools/validate_textmod.js` -- validation (optional secondary check from Rust tests, not a dependency)
- `tools/hero_configs/*.json` -- data files, language-agnostic
- `generated/*.txt` -- data files, language-agnostic
