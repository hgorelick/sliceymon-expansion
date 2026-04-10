# Textmod API Integration Plan

## Spec: Integrating Thunder's Undocumented Textmod Guide v3.2

### Source Material

- **Thunder's Undocumented Textmod Guide (v3.2)** — community reference documenting undocumented/advanced textmod APIs
- **Location**: `reference/UndocumentedTextmodGuide_V3.2_.html` (local copy -- already present in the repo)
- **Online**: [Google Doc](https://docs.google.com/document/d/1JUUr5qgPKS1AhcZOwHR8P-DMQID_-BelTvt-i99aicg)
- **tann.fun resource page**: https://tann.fun/games/dice/textmod/

### Prerequisite

The guide file `reference/UndocumentedTextmodGuide_V3.2_.html` is already present in the repo. No download needed.

### Problem Statement

Our compiler currently models 5 entity types (Hero, ReplicaItem, Monster, Boss, StructuralModifier) with field-based IR extraction and emission. However, the textmod system is far richer than our IR represents — it's essentially a **domain-specific programming language** with:

- **18 phase types** forming a control-flow system (conditionals, sequences, loops, trade screens)
- **10+ tog items** that compose entity behavior via logic-gate operations
- **A variable system** (Values) with conditional branching (BooleanPhase)
- **Compositional item building** (hat, splice, sticker, cast, enchant)
- **Rich text formatting** with color tags, entity images, and custom descriptions
- **Side position selectors** (top, mid, bot, left, right, topbot, rightmost)
- **Targeting restriction logic** (togres + 6 boolean variants: AND, OR, XOR, NOT, SWAP, MULTIPLY)

Our compiler treats most of this as opaque strings inside `StructuralModifier.body`, `ModifierChain.content`, or `Boss.event_body`. To become a proper mod-building backend, we need structured understanding of these systems — for validation, for CRUD operations, and for the eventual web UI.

---

## Gap Analysis: Current IR vs. Full Textmod API

### Layer 1: Entity Data (MOSTLY COVERED)

What we model today. These are the "nouns" of the textmod language.

| System | Current State | Gap |
|--------|--------------|-----|
| Heroes (HeroBlock) | Fully modeled: sd, hp, col, tier, img, speech, n, mn | None |
| ReplicaItems | Fully modeled: template, sd, hp, col, tier, img, ability_data | None |
| Monsters | Fully modeled: base_template, floor_range, hp, sd, col, bal | None |
| Bosses | 3 formats modeled: Standard, Event, Encounter | Event body is opaque string |
| DiceFaces | Blank/Active with face_id + pips | No semantic knowledge of what face IDs mean |
| AbilityData | Template, sd, img, name, modifier_chain | No validation of spell vs tactic rules (which sides matter) |
| TriggerHPData | Field exists on IR types | No validation of HP→pip mapping rules |
| ModifierChain | Item/Sticker segments with content strings | Segments are opaque — no structured parsing of hat/splice/tog/cast composition |

### Layer 2: Composition & Behavior (NOT COVERED)

The "verbs" and "adjectives" — how entities are modified and composed.

| System | Current State | What Thunder's Guide Documents |
|--------|--------------|-------------------------------|
| **Tog Items** | Appear as opaque text in modifier chains | 10 items with distinct semantics: togtime, togtarg, togfri, togvis, togeft, togpip, togkey, togunt, togres, togresm |
| **Togres Variants** | Not modeled | 6 boolean logic variants: togresa (AND), togreso (OR), togresx (XOR), togresn (NOT), togress (SWAP), togresm (MULTIPLY) |
| **Side Position Selectors** | Appear in modifier chains as opaque text | top, mid, bot, left, right, topbot, rightmost, right2 — determine which dice sides are affected |
| **Keywords (k.)** | Not modeled | Keyword application via `k.keyword` syntax — engage, cruel, trio, first, sixth, etc. |
| **Hat Composition** | Part of opaque modifier chain | `hat.Entity` copies sides from another entity — core composition mechanism |
| **Splice** | Not modeled | `splice.Item` merges item conditional bonuses |
| **Cast** | Partially in modifier chains | `cast.crush`, `cast.drop` — side visual/targeting effects |
| **Enchant** | Not modeled | `enchant.Modifier` — apply modifier as side effect |
| **Facade** | Field exists but content is opaque | `facade.EntityCode:Parameter` — visual side override |
| **Memory** | Not modeled | Hidden item that reverts left side to default after tog modifications |
| **Sidesc** | Not modeled | `sidesc.text` — custom side description with richtext formatting |

### Layer 3: Game Flow & Control (NOT COVERED)

The "control flow" — phases, rewards, conditionals, variables.

| System | Current State | What Thunder's Guide Documents |
|--------|--------------|-------------------------------|
| **Phases (18 types)** | Appear as opaque strings in structural modifiers or boss events | Full syntax for each: SimpleChoicePhase, PlayerRolling, Targeting, LevelEnd, EnemyRolling, Message, HeroChange, Reset, ItemCombine, PositionSwap, Challenge, BooleanPhase, ChoicePhase, Damage, RunEnd, Linked, RandomReveal, SeqPhase, Trade, PhaseGenerator, BooleanPhase2 |
| **Choosable Tags** | Not modeled | 11 tag types: m (Modifier), i (Item), l (Levelup), g (Hero), r (Random), q (RandomRange), o (Or), e (Enu), v (Value), p (Replace), s (Skip) |
| **Value System** | Not modeled | Custom variables with `v(name)V(amount)` syntax, displayed via `[val(name)]` |
| **BooleanPhase** | Not modeled | `ph.b(value);##;phaseA@2phaseB` — conditional branching on values |
| **LinkedPhase** | Not modeled | `ph.l(phase1)@1(phase2)` — sequential phase chaining |
| **SeqPhase** | Partially in boss event_body | `ph.s(message)@1(button)@2(phase)` — interactive choice trees with `@1`/`@2` delimiters |
| **ChoicePhase** | Not modeled | PointBuy, Number, UpToNumber, Optional — 4 choice screen types |
| **Level Scoping** | Partially in monster floor_range | `N.`, `N-M.`, `eN.`, `lvl.` — floor targeting for any modifier |
| **phi./phmp.** | Not modeled | Phase index shortcuts and modifier pick screens |
| **Hidden Modifiers** | Used (hidden, temporary) but not as typed concepts | Skip, Wish, Clear Party, Missing, Skip All, Add/Minus Fight, Cursemode Loopdiff |
| **Fight Definition** | Modeled in bosses only | `fight.Enemy1+Enemy2.mn.Name` — generalizable beyond boss context |

### Layer 4: Presentation (NOT COVERED)

Text formatting, images, descriptions.

| System | Current State | What Thunder's Guide Documents |
|--------|--------------|-------------------------------|
| **Richtext Colors** | Opaque in speech/doc | `[orange]`, `[yellow]`, `[light]`, `[blue]`, `[red]`, `[cu]` (close), `[n]` (newline), `[nh]` (no highlight) |
| **Entity Images in Text** | Not modeled | `[EntityName]` in text renders entity sprite |
| **Custom Images** | img_data extracted but format not validated | Base64-like encoded images in `[...]` blocks |
| **Doc Formatting** | Treated as opaque string | `doc.text` with richtext support, entity references, custom images |
| **Sidesc Formatting** | Not modeled | `sidesc.text` with `[pips]` placeholder and richtext |
| **HSV/Hue/Draw** | HSV on AbilityData only | `.hsv.+-:+-:+-`, `.hue.+-`, `.draw.tx:##:##`, `.b.fff` — general texture modification |
| **Rect** | Not modeled | `.rect.##:###` — sprite rectangle override |

---

## Architectural Strategy

### Modeling Depth Spectrum

Not every system needs the same level of modeling. The right approach depends on what the mod-builder needs to do with it:

```
Level 0: Opaque String       — Store as-is, no validation, no editing UI
Level 1: Validated String     — Store as string, validate structure/syntax
Level 2: Typed Enum/Struct    — Parse into IR fields, validate semantics, CRUD
Level 3: Full Compositional   — Parse substructure, enable UI composition, cross-reference
```

### Recommended Modeling Depth Per System

| System | Level | Rationale |
|--------|-------|-----------|
| **Phases** | **Level 2** | Users need to create/edit phases in the mod builder. Must parse phase type + arguments. Critical for boss events, custom modes. |
| **Tog Items** | **Level 2** | Users compose hero behavior via tog items. Must know which tog item, which side position, and validate combinations. |
| **Togres Variants** | **Level 2** | Logic-gate composition is a core advanced mechanic. Must model the boolean operator type for validation and UI. |
| **Choosable Tags** | **Level 2** | Reward systems are core to custom mode building. Must parse tag type + reward reference. |
| **Value System** | **Level 2** | Variables + conditionals are core to custom game modes. Must track variable names and reference sites. |
| **Side Position Selectors** | **Level 2** | Side targeting is fundamental to item/tog composition. Must parse which sides are affected. |
| **Modifier Chain (hat/splice/cast/enchant)** | **Level 3** | The primary composition mechanism. Must parse each segment type, validate entity references, enable UI drag-and-drop composition. |
| **Fight Definition** | **Level 2** | Generalize beyond bosses — fights appear in phases, vase entities, choosables. Must parse enemy list + name. |
| **Richtext Formatting** | **Level 1** | Validate bracket balance and known color tags. Don't need full AST — string with validation is sufficient. |
| **Doc/Sidesc** | **Level 1** | Validate syntax, preserve content. Users edit as formatted text, not structured data. |
| **Hidden Items** | **Level 1** | Known set of 23 items. Validate names against known list. |
| **Hidden Modifiers** | **Level 1** | Known set of 12 modifiers. Validate names against known list. |
| **HSV/Hue/Draw/Rect** | **Level 1** | Validate format (signed numbers, hex colors). Simple structural validation. |
| **Facade** | **Level 1** | Validate `EntityCode:Parameter` format. Don't need to resolve entity codes. |
| **TriggerHPData mechanics** | **Level 1** | Validate HP value ranges. Document the HP→pip mapping table for UI reference. |
| **AbilityData spell/tactic rules** | **Level 2** | Validate which sides are used for what (Side 1=primary, Side 2=secondary, Side 5=mana cost, Sides 3/4/6=tactic costs). |
| **Level Scoping** | **Level 2** | Parse `N.`, `N-M.`, `eN.`, `lvl.` prefixes into structured fields. Used across phases, monsters, and modifiers. |
| **phi./phmp.** | **Level 1** | Validate index ranges (0-9 for phi, integer for phmp). |
| **Orb/Vase/Jinx/Egg** | **Level 1** | Validate entity type syntax. These are entity wrappers with known patterns. |
| **RNG Seeds** | **Level 0** | Opaque generation seeds. No user-facing editing needed. |

---

## IR Type Changes Required

### New Types

```rust
/// Phase system — control flow for custom modes and events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Phase {
    pub phase_type: PhaseType,
    pub level_scope: Option<LevelScope>,
    pub content: PhaseContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum PhaseType {
    SimpleChoice,       // ph.!
    PlayerRolling,      // ph.0
    Targeting,          // ph.1
    LevelEnd,           // ph.2
    EnemyRolling,       // ph.3
    Message,            // ph.4
    HeroChange,         // ph.5
    Reset,              // ph.6
    ItemCombine,        // ph.7
    PositionSwap,       // ph.8
    Challenge,          // ph.9
    Boolean,            // ph.b
    Choice,             // ph.c
    Damage,             // ph.d
    RunEnd,             // ph.e
    Linked,             // ph.l
    RandomReveal,       // ph.r
    Seq,                // ph.s
    Trade,              // ph.t
    PhaseGenerator,     // ph.g
    Boolean2,           // ph.z
}
// NOTE: No Unknown variant. If the parser encounters an unrecognized phase code,
// it returns a CompilerError. The textmod API has a finite set of phase codes
// documented in Thunder's guide — an unrecognized code is a parse error, not
// valid data to preserve.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum PhaseContent {
    SimpleChoice {
        title: Option<String>,
        rewards: Vec<RewardTag>,
    },
    Message {
        text: RichText,
        button_text: Option<String>,
    },
    HeroChange {
        hero_position: u8,
        change_type: HeroChangeType, // RandomClass or GeneratedHero
    },
    Boolean {
        value_name: String,
        threshold: i32,
        if_true: Box<Phase>,
        if_false: Box<Phase>,
    },
    Linked {
        phases: Vec<Phase>,
    },
    Seq {
        message: RichText,
        options: Vec<SeqOption>,
    },
    Trade {
        rewards: Vec<RewardTag>,
    },
    Choice {
        choice_type: ChoiceType,
        rewards: Vec<RewardTag>,
    },
    LevelEnd {
        phases: Vec<Phase>,
    },
    Challenge {
        reward: Vec<RewardTag>,
        extra_monsters: Vec<String>,
    },
    PositionSwap {
        first: u8,
        second: u8,
    },
    ItemCombine {
        combine_type: String, // "SecondHighestToTierThrees" or "ZeroToThreeToSingle"
    },
    RandomReveal {
        reward: RewardTag,
    },
    PhaseGenerator {
        gen_type: PhaseGenType, // Hero or Item
    },
    RunEnd,
    Reset,
    PlayerRolling,
    Targeting,
    EnemyRolling,
    Damage,
}
// NOTE: No Unparsed variant. If the parser can identify the phase type but cannot
// parse its content, it returns a CompilerError with the phase type, position, and
// what it expected vs. what it found. Silently swallowing unparseable content into
// a raw string violates the no-raw-passthrough principle and masks bugs.
//
// DESIGN NOTE: 17 variants is large but intentional. Each phase type has unique
// syntax and content fields -- a trait-based approach (Box<dyn PhaseContent>) would
// lose exhaustive match checking and serde derive convenience. The 17 variants
// map 1:1 to the 17 distinct phase codes in Thunder's guide, and each requires
// its own parse/emit/validate logic regardless of representation. Exhaustive
// match in Rust guarantees that adding a new phase type is a compile error in
// every consumer until handled.
//
// DESIGN NOTE: PhaseContent has 17+ variants, one per phase type code from Thunder's
// guide. A trait-based approach was considered and rejected because:
// 1. Exhaustive match checking catches missing handlers at compile time
// 2. serde derive works cleanly with enums (no trait object serialization)
// 3. 1:1 mapping to Thunder's guide phase codes makes the enum self-documenting
// The variant count is intentional, not accidental complexity.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SeqOption {
    pub button_text: String,
    pub phases: Vec<Phase>,
}

/// Choosable/SCPhase reward tag
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RewardTag {
    pub tag_type: RewardTagType,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RewardTagType {
    Modifier,       // m
    Item,           // i
    Levelup,        // l
    Hero,           // g
    Random,         // r — needs tier~amount~tag parsing
    RandomRange,    // q — needs tier1~tier2~amount~tag parsing
    Or,             // o — uses @4 delimiter
    Enu,            // e
    Value,          // v — variable system
    Replace,        // p — pm(old)~(new)
    Skip,           // s
}

/// Level/floor scoping
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LevelScope {
    pub start: u8,
    pub end: Option<u8>,          // None = single floor
    pub interval: Option<u8>,     // eN = every N floors
    pub offset: Option<u8>,       // eN.M = every N starting at M
}

/// Typed chain segment — replaces the old `ChainSegment` struct.
///
/// **Naming note**: The existing `ChainSegment` struct (with `kind: SegmentKind` and
/// `content: String`) is REPLACED by this enum. The migration path is:
/// 1. Replace existing `ChainSegment` struct with this enum (same name, new definition)
/// 2. `SegmentKind` enum is REMOVED — every segment must be either Item or Sticker.
///    If the parser cannot determine which, it returns a CompilerError.
/// 3. The `content` field is replaced by structured fields per variant
/// 4. `ModifierChain.segments` keeps its type `Vec<ChainSegment>` — only the
///    definition of `ChainSegment` changes from struct to enum.
///
/// The blast radius is contained: `ChainSegment` and `SegmentKind` are only
/// directly referenced in `ir/mod.rs` (struct definition, inline tests, and
/// `ModifierChain::parse()`/`emit()` methods). The parsers and emitters use
/// `ModifierChain` as an opaque type — they call `ModifierChain::parse()` and
/// `chain.emit()` without constructing `ChainSegment` values directly.
/// Therefore, the migration is primarily within `ir/mod.rs`.
///
/// Files that need updating:
/// - `ir/mod.rs` — struct/enum definition, `ModifierChain::parse()`/`emit()`,
///   inline `#[cfg(test)]` module (7 tests reference `SegmentKind`/`ChainSegment` fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum ChainSegment {
    /// `.i.` segment — item/equipment with optional side position and nested sub-entries
    Item {
        name: String,
        position: Option<SidePosition>,
        /// `#`-separated sub-entries within this item segment (keywords, facades, etc.)
        sub_entries: Vec<ChainEntry>,
    },
    /// `.sticker.` segment
    Sticker {
        target: String,
        position: Option<SidePosition>,
        sub_entries: Vec<ChainEntry>,
    },
}
// NOTE: No Raw variant. Every modifier chain segment is either `.i.` or `.sticker.`.
// If the parser cannot determine the segment type, it returns a CompilerError.
// The old `SegmentKind` enum is REMOVED entirely — it is implicit in the variant.

/// A single `#`-separated entry within an `.i.` or `.sticker.` segment.
/// These are the individual modifiers applied within a chain segment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum ChainEntry {
    Hat {
        entity: String,
        position: Option<SidePosition>,
    },
    Splice {
        item: String,
    },
    Cast {
        effect: String,          // crush, drop, slice, etc.
    },
    Enchant {
        modifier: String,
    },
    Learn {
        ability: String,
    },
    TogItem {
        tog_type: TogType,
        position: Option<SidePosition>,
    },
    Keyword {
        keyword: String,
        position: Option<SidePosition>,
    },
    Facade {
        entity_code: String,
        parameter: String,
    },
    Sidesc {
        text: String,
    },
    /// Entity reference: `r<type>.<hex_hash>[.part.<n>][.m.<n>][.i.<chain>][.n.<name>]`
    /// Three known ref types from Thunder's guide and working mods:
    ///   - `ritemx.<hash>` — item refs (100+ unique in sliceymon alone)
    ///   - `rmod.<hash>`   — modifier refs (19 unique across test mods)
    ///   - `rmon.<hash>`   — monster refs (9 unique across test mods)
    /// Suffix modifiers are shared across all ref types.
    EntityRef {
        kind: RefKind,
        /// Hex hash stored as String. Validation (hex-only chars, length bounds) is
        /// a validator concern (Chunk 10), not a parse-time concern -- the parser
        /// accepts any non-delimiter characters after the `r<type>.` prefix as the hash.
        hash: String,
        part: Option<u8>,
        multiplier: Option<u8>,
    },
    Memory,
}
// NOTE: No Raw variant. Every `#`-separated entry within a chain segment must be
// parseable into a typed variant. The `EntityRef` variant handles entity references
// using the `r<type>.<hash>` pattern from Thunder's guide. It is NOT a raw passthrough
// but a typed reference with an enumerated kind and optional suffixes.
// If the parser encounters content that is neither a known prefix pattern nor a
// valid entity reference, it returns a CompilerError with the entry content and
// its position in the chain.
//
// IMPORTANT: The no-Raw constraint is validated by
// `test_all_chain_entries_typed_across_test_mods` in Chunk 4, which parses every
// chain entry from all 4 test mods. If ANY real-world entry cannot be parsed into
// a typed variant, a new variant must be added. Do NOT introduce a Raw fallback.

/// Entity reference type. Three known ref prefixes from Thunder's guide:
/// `ritemx` (items), `rmod` (modifiers), `rmon` (monsters).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RefKind {
    Item,       // ritemx
    Modifier,   // rmod
    Monster,    // rmon
}
// NOTE: EntityRef.hash is stored as String, not validated as hex at parse time.
// Hex format validation is a Chunk 10 validator concern (rule V050), not a
// parse-time concern — the parser's job is structural extraction, the validator's
// job is semantic checking.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum SidePosition {
    Top,
    Mid,
    Bot,
    Left,
    Right,
    TopBot,
    Rightmost,
    Right2,
    All,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum TogType {
    Time,       // togtime
    Targ,       // togtarg
    Fri,        // togfri
    Vis,        // togvis
    Eft,        // togeft
    Pip,        // togpip
    Key,        // togkey
    Unt,        // togunt
    Res,        // togres — basic restriction copy
    ResM,       // togresm — multiply (x2 conditional)
    ResA,       // togresa — AND
    ResO,       // togreso — OR
    ResX,       // togresx — XOR
    ResN,       // togresn — NOT
    ResS,       // togress — SWAP
}

/// Value system variable reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ValueRef {
    pub name: String,
    pub amount: i32,
}

/// Rich text — a newtype with validation behavior.
/// Richtext strings contain [color], [entity], and [n] formatting tags.
/// The newtype provides validation methods that check bracket balance,
/// known color tag names, and entity reference format. Content is preserved
/// exactly for emission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RichText(pub String);

impl RichText {
    pub fn new(s: impl Into<String>) -> Self { RichText(s.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
    /// Check if this richtext contains any formatting tags at all.
    pub fn has_tags(&self) -> bool { self.0.contains('[') }
}
// ARCHITECTURE NOTE: Richtext VALIDATION lives in validator.rs as
// `validate_richtext(rt: &RichText) -> Vec<Finding>`, NOT as a method on RichText.
// This avoids a circular module dependency: ir/mod.rs cannot import from validator.rs
// (which imports ir types). The RichText newtype provides structural methods
// (new, as_str, has_tags) but validation logic that produces Finding values stays in
// the validator module where Finding is defined.

/// Fight definition — generalized beyond Boss context.
///
/// **Migration note**: This REPLACES `BossFightVariant` + `BossFightUnit` from the
/// existing IR. The existing `Boss.variants: Vec<BossFightVariant>` becomes
/// `Boss.fights: Vec<FightDefinition>`. Each `BossFightVariant` maps to a
/// `FightDefinition` with its `fight_units` becoming `enemies`.
///
/// The existing `BossFightUnit` fields map 1:1 to `FightUnit` fields (plus new
/// `color` and `hsv` fields for general fight contexts). The `trigger` field from
/// `BossFightVariant` moves to `FightDefinition`. The `level` field is NEW on
/// `FightDefinition` — for boss migration it is populated from `Boss.level`;
/// for general fight contexts (fights in phases) it comes from the fight's own
/// level scope prefix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FightDefinition {
    pub level: Option<u8>,
    pub enemies: Vec<FightUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,     // .mn. name
    /// Variant trigger suffix (e.g., "@4m4") — from BossFightVariant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}
// MIGRATION NOTE: BossFightVariant.name is `String` but FightDefinition.name is
// `Option<String>`. All existing code that accesses `.name` on fight variants must
// be updated to handle the Option. This affects boss_parser, boss_emitter, and test
// assertion code that pattern-matches on name.

/// Unified fight unit — shared between bosses and general fights.
/// REPLACES `BossFightUnit` (same fields plus `color` and `hsv` for general fight contexts).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FightUnit {
    pub template: String,
    pub name: String,             // Same as BossFightUnit.name (already String, not Option)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sd: Option<DiceFaces>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_override: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<ModifierChain>,  // Uses ModifierChain, NOT Vec<ChainSegment>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsv: Option<String>,
}

/// Choice phase type — uses adjacently tagged serde for clean JSON.
/// JSON output: {"type": "PointBuy", "budget": 5}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum ChoiceType {
    PointBuy { budget: i32 },
    Number { count: u8 },
    UpToNumber { max: u8 },
    Optional,
}

/// Hero change type in HeroChangePhase
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum HeroChangeType {
    RandomClass,     // 0
    GeneratedHero,   // 1
}

/// Phase generator type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum PhaseGenType {
    Hero,  // ph.gh
    Item,  // ph.gi
}

// AbilityData — MODIFY existing struct (compiler/src/ir/mod.rs line 268)
// Add one new field to the existing AbilityData struct:
//
// WHEN: Chunk 3 (ability_type field added during structural expansion)
//   pub struct AbilityData {
//       pub template: String,
//       pub sd: DiceFaces,
//       pub img_data: Option<String>,
//       pub name: String,
//       pub modifier_chain: Option<ModifierChain>,  // KEEP existing type
//       pub hsv: Option<String>,
//       #[serde(skip_serializing_if = "Option::is_none")]
//       pub ability_type: Option<AbilityType>,  // NEW: derived from which sides have data
//   }
//
// NOTE: modifier_chain stays as Option<ModifierChain>, NOT Option<Vec<ChainSegment>>.
// The ModifierChain struct's internal ChainSegment type will change (struct→enum),
// but AbilityData's field type stays the same.
// Side semantics: Side 1=primary, Side 2=secondary untargeted,
// Side 5=mana cost (makes it a spell), Sides 3/4/6=tactic costs

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum AbilityType {
    Spell { mana_cost: u16 },    // Side 5 has data → spell
    Tactic { cost_count: u8 },   // Sides 3/4/6 have data → tactic (1-3 costs)
}

/// Entity wrapper types (orb, vase, jinx, egg).
/// These appear within modifier chains, fight units, and structural modifiers.
/// Each wrapper prefix (`orb.s`, `vase.`, `jinx.`, `egg.`) is recognized by
/// the chain parser and could be modeled as ChainEntry variants in the future.
/// For now they are used as a validation-level concept — the validator checks
/// wrapper syntax when encountered in chain entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum EntityWrapper {
    Orb { entity: String },     // orb.s[entity] — passive on death
    Vase { entity: String },    // vase.([modifiers]) — death trigger
    Jinx { modifier: String },  // jinx.[modifier] — monster death modifier
    Egg { entity: String },     // egg.[entity] — summon
}
```

### Modified Existing Types (TARGET STATE -- changes are spread across Chunks 3 and 8)

```rust
// ModifierChain: segments field type name stays Vec<ChainSegment>, but
// ChainSegment changes from struct {kind, content} to the enum defined above.
// The ModifierChain::parse() and ModifierChain::emit() methods must be rewritten
// to construct/destructure the new ChainSegment enum variants.
// SegmentKind enum is REMOVED — the Item/Sticker distinction is now implicit
// in the ChainSegment enum variant. All SegmentKind references must be deleted.
// WHEN: Chunk 4 (chain migration + typed parsing)
pub struct ModifierChain {
    pub segments: Vec<ChainSegment>,  // ChainSegment is now an enum (Item/Sticker)
}

// Boss: Replace BossFightVariant/BossFightUnit with FightDefinition/FightUnit.
// event_body is replaced with event_phases after the phase parser exists.
// WHEN: Chunk 8 (boss migration -- after phase parser in Chunk 6)
pub struct Boss {
    // ... existing fields ...
    pub fights: Vec<FightDefinition>,   // Replaces variants: Vec<BossFightVariant>
    pub event_phases: Option<Vec<Phase>>,  // Replaces event_body: Option<String>
}

// StructuralType: Add new variants to match StructuralContent
// WHEN: Chunk 3 (structural type expansion)
pub enum StructuralType {
    // ... existing variants ...
    PhaseModifier,
    Choosable,
    ValueModifier,
    HiddenModifier,
    FightModifier,
}

// StructuralContent: Add new variants (each with body for emission, typed fields populated later)
//
// IMPORTANT: The typed fields (`phase`, `tag`, `value_ref`, `fight`) are `Option<T>` because
// their parsers don't exist until Chunks 6-8. Chunk 3 adds the variants with `body` populated
// and typed fields set to `None`. Chunks 6-8 populate the typed fields via their respective
// parsers. The emitter uses the typed fields when present, falling back to `body` when `None`.
// This is NOT deferred correctness — it's staged construction where body is always authoritative
// for emission and the typed fields provide structured access for validation/UI.
pub enum StructuralContent {
    // ... existing variants ...
    PhaseModifier {
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        level_scope: Option<LevelScope>,
        #[serde(skip_serializing_if = "Option::is_none")]
        phase: Option<Phase>,       // Populated by Chunk 6 (phase parser)
    },
    Choosable {
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        level_scope: Option<LevelScope>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<RewardTag>,     // Populated by Chunk 7 (reward parser)
    },
    ValueModifier {
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        level_scope: Option<LevelScope>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value_ref: Option<ValueRef>,// Populated by Chunk 7 (reward parser)
    },
    HiddenModifier {
        body: String,
        modifier_type: HiddenModifierType,  // Determinable from body string in Chunk 3
    },
    FightModifier {
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        level_scope: Option<LevelScope>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fight: Option<FightDefinition>,  // Populated by Chunk 8 (fight parser)
    },
}
// EMITTER CONSTRAINT: New StructuralContent variants (PhaseModifier, Choosable,
// ValueModifier, FightModifier) MUST emit from `body` (never from typed fields)
// until the typed fields are proven correct via round-trip testing in Chunk 11.
// This prevents silent data loss if typed fields disagree with body content.
// The structural_emitter always has body available as the authoritative source.

// New hidden modifier enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum HiddenModifierType {
    Skip,
    Wish,
    ClearParty,
    Missing,
    Temporary,
    Hidden,
    SkipAll,
    AddFight,
    Add10Fights,
    Add100Fights,
    MinusFight,
    CursemodeLoopdiff,
}
```

### New Validation Rules

```
// Phase validation
V001: Phase type recognized (unrecognized phase code is a parse error)
V002: BooleanPhase value name references a defined Value
V003: LinkedPhase contains at least 2 phases
V004: SeqPhase option count matches phase count
V005: HeroChangePhase position within party size (0-4)
V006: ChallengePhase has valid JSON structure
V007: ChoicePhase budget/count is positive
V008: Phase delimiters correct per type (@1 for Linked, @2 for Boolean, @3 for SCPhase, @6/@7 for Boolean2)

// Modifier chain validation
V010: Tog items reference valid side positions
V011: Togres variants follow valid boolean composition order
V012: Hat references a known entity template
V013: Memory appears after tog modification (not standalone)
V014: Facade uses valid EntityCode:Parameter format
V015: Keyword names from known keyword list

// Choosable/reward validation
V020: Reward tag type matches content format
V021: Random tag has valid tier~amount~tag format
V022: Value name is consistent across set/check sites
V023: Replace tag references a valid modifier name

// Composition validation
V030: AbilityData side usage matches spell/tactic rules
V031: TriggerHPData HP value in valid range
V032: Level scope range is valid (start <= end, interval > 0)

// Richtext validation
V040: Richtext bracket balance (color tags opened/closed)
V041: Known color tag names only
V042: Entity references in [EntityName] resolve to known entities
```

---

## Implementation Plan

### Overview

Integrate Thunder's Undocumented Textmod Guide v3.2 into the compiler's IR, extractors, builders, and validators to model the full textmod API. This transforms the compiler from a hero/monster/boss tool into a complete mod-building backend that understands phases, composition, variables, and the full game-flow system.

### Checkpoint Configuration

- Total chunks: 11
- Checkpoint frequency: After critical chunks and every 2 non-critical chunks
- Critical checkpoints: After chunks 2, 3, 4, 8 (new IR types, structural migration, chain migration, fight+boss migration)
- Non-critical checkpoint pairs: after chunks {5,6}, {9,10}
- **Invariant**: After EVERY chunk, `cargo test` must pass with 0 failures. A chunk that leaves tests broken is incomplete.

### Parallel Execution Map

```
Foundation (sequential):
  Chunk 1: Reference data + known constants

Core IR (sequential, after Chunk 1):
  Chunk 2: New IR type definitions (ADDITIVE ONLY)                             [CRITICAL]

Structural Migration (sequential, after Chunk 2):
  Chunk 3: Structural type + error variant migration                           [CRITICAL]
    Adds: new StructuralContent/StructuralType variants, AbilityData.ability_type,
          CompilerError variants, ModifierType enum variants (stubs), classifier stubs.
    Does NOT touch: ModifierChain, ChainSegment, SegmentKind (those move in Chunk 4).

Parallel Group A (after Chunk 3):
  Chunk 4: Chain migration + typed chain entry parsing                         [CRITICAL]
    Replaces ChainSegment struct with enum, removes SegmentKind, AND parses
    #-delimited content into typed ChainEntry variants — atomically in one chunk.
    No raw_content field, no empty sub_entries, no interim state.
  Chunk 5: Level scoping + richtext validation

Sequential (after Chunk 5):
  Chunk 6: Phase system parser                               (needs Chunk 5 for LevelScope)
    NOTE: Chunk 6 has a hard dependency on Chunk 5 (LevelScope parser).
    It CANNOT run in parallel with Chunk 5. Chunks 4+5 run in parallel;
    Chunk 6 starts after Chunk 5 completes. Chunk 4 may still be running.

Parallel Group B (after Chunk 6):
  Chunk 7: Choosable/reward tag system                       (needs Chunk 6)
  Chunk 8: Fight generalization + Boss migration             (needs Chunks 3, 4, 6) [CRITICAL]
    Replaces BossFightUnit/BossFightVariant with FightUnit/FightDefinition.
    Replaces Boss.event_body with Boss.event_phases (needs phase parser from Chunk 6).
    Full blast radius: ir/mod.rs, boss_parser, boss_emitter, ir/merge.rs,
    ir/ops.rs, fight_parser (NEW), fight_emitter (NEW), tests.

Parallel Group C (after Group B):
  Chunk 9:  Builders/emitters for all new types              (needs Chunks 6-8)
  Chunk 10: Validator rules for all new systems              (needs Chunks 6-8)

Integration (sequential, after all):
  Chunk 11: Round-trip tests, cross-reference validation, schema update

Minimum wall-clock rounds: 8 (vs 11 sequential)
  Round 1: Chunk 1
  Round 2: Chunk 2
  Round 3: Chunk 3
  Round 4: Chunks 4 + 5 (parallel)
  Round 5: Chunk 6 (needs Chunk 5; Chunk 4 may still be finishing)
  Round 6: Chunks 7 + 8 (parallel)
  Round 7: Chunks 9 + 10 (parallel)
  Round 8: Chunk 11
Critical path: 1 -> 2 -> 3 -> 5 -> 6 -> 8 -> 9 -> 11 (8 rounds)
```

### Cross-Chunk File Dependency Matrix

Files touched by multiple chunks. Implementers must coordinate on these:

| File | Chunks | Notes |
|------|--------|-------|
| `ir/mod.rs` | 2 (new types), 3 (StructuralContent variants, AbilityData), 4 (ChainSegment migration), 8 (Boss migration) | Four modification rounds -- each must leave cargo test passing |
| `ir/ops.rs` | 3 (structural CRUD), 8 (boss CRUD field renames) | Test helpers construct Boss/FightUnit |
| `ir/merge.rs` | 3 (new StructuralType merge), 8 (boss field renames) | Merge logic for structural + boss types |
| `error.rs` | 3 (add PhaseParseError, ChainParseError, RewardParseError) | One-time addition |
| `extractor/mod.rs` | 3 (dispatch stubs), 4 (chain_parser module), 5 (level_scope_parser module), 6 (phase_parser module), 7 (reward_parser module), 8 (fight_parser module) | Module declarations + dispatch arms |
| `extractor/classifier.rs` | 3 (new ModifierType variant stubs -- classify() never produces them), 7 (real classification patterns) | Chunk 3 adds enum variants with stub detection. Chunk 7 adds real detection patterns. Classification order is sensitive. |
| `extractor/structural_parser.rs` | 3 (stub arms for new variants), 7 (typed parsing for choosable/value/phase) | Chunk 3 stubs; Chunk 7 replaces stubs with real parsing |
| `extractor/boss_parser.rs` | 8 (fight parser extraction, event_phases) | Single modification round |
| `builder/mod.rs` | 3 (StructuralType emission loops -- without these, new structural types are silently dropped), 4 (chain_emitter module), 8 (fight_emitter module), 9 (phase_emitter module, assembly order) | Module declarations + filter arms |
| `builder/structural_emitter.rs` | 3 (body-based emission for new variants -- MUST emit from body, see emitter constraint), 9 (field-based emission when typed fields proven) | Chunk 3: `body.clone()` emission; Chunk 9: structured emission. |
| `builder/boss_emitter.rs` | 8 (fight emitter extraction) | Single modification round |
| `validator.rs` | 5 (validate_richtext function), 10 (new rules), 11 (cross-reference rules) | Richtext validation in Chunk 5; rules added in Chunk 10; cross-refs completed in Chunk 11 |
| `lib.rs` | 1 (constants module), 9 (public exports) | Module declarations + API surface |
| `tests/builder_tests.rs` | 8 (BossFightUnit/BossFightVariant -> FightUnit/FightDefinition) | Constructs boss types directly -- must be updated with type renames |
| `tests/expansion_tests.rs` | 8 (event_body -> event_phases) | References event_body field on Boss |
| `tests/boss_tests.rs` | 8 (boss field renames) | Boss assertion updates |

---

### Chunk 1: Reference Data & Known Constants

**Scope**: Establish source-of-truth constant tables derived from Thunder's guide. These are the "dictionaries" that validators and parsers reference.

**Read First**:
- `reference/UndocumentedTextmodGuide_V3.2_.html` — primary source for all constants
- `compiler/src/lib.rs` — to see where to declare the new module

**Files** (2 files):
- `compiler/src/constants.rs` (NEW)
- `compiler/src/lib.rs` (MODIFY — add `pub mod constants;`)

**Dependencies**: None

**Constraints**:
- Use ONLY values documented in Thunder's guide v3.2. Do NOT invent or extrapolate.
- All constants must be `pub const` arrays, `pub fn is_known_X(s: &str) -> bool` match functions, or enums. No inline magic strings.
- No `std::fs` or I/O — WASM-safe.

**Requirements**:
- Known phase type codes: `!`, `0`-`9`, `b`, `c`, `d`, `e`, `l`, `r`, `s`, `t`, `g`, `z`
- Known tog item names: `togtime`, `togtarg`, `togfri`, `togvis`, `togeft`, `togpip`, `togkey`, `togunt`, `togres`, `togresm`, `togresa`, `togreso`, `togresx`, `togresn`, `togress`
- Known reward tag letters: `m`, `i`, `l`, `g`, `r`, `q`, `o`, `e`, `v`, `p`, `s`
- Known hidden modifier names: `Skip`, `Wish`, `Clear Party`, `Missing`, `Temporary`, `Hidden`, `Skip All`, `Add Fight`, `Add 10 Fights`, `Add 100 Fights`, `Minus Fight`, `Cursemode Loopdiff`
- Known hidden items (non-tog): `rgreen`, `clearicon`, `cleardesc`, `Idol of Chrzktx`, `Idol of Aiiu`, `Idol of Pythagoras`, `False Idol`
- Known side positions: `top`, `mid`/`middle`, `bot`/`bottom`, `left`, `right`, `topbot`, `rightmost`, `right2`, `all`
- Known richtext color tags: `orange`, `yellow`, `light`, `blue`, `red`, `cu`, `n`, `nh`
- Known choice types: `PointBuy`, `Number`, `UpToNumber`, `Optional`
- Phase delimiter table: `@1` (Linked), `@2` (Boolean, LevelEnd), `@3` (SCPhase/Choice rewards), `@4` (Or tag), `@6`/`@7` (Boolean2)
- phi. index table: 0-9 mapping to phase types
- ItemCombine type names: `SecondHighestToTierThrees`, `ZeroToThreeToSingle`
- AbilityData side semantics: Side 1=primary, Side 2=secondary untargeted, Side 5=mana cost, Sides 3/4/6=tactic costs
- TriggerHPData HP→pip position mapping table (HP 1-21 + formula for 22+)
- Entity wrapper prefixes: `orb.s`, `vase.`, `jinx.`, `egg.`
- Known untargeted effect face IDs: Reroll=125, Revive=35/136/166, Mana=76, Damage all=34/128, Shield all=72/73, Heal all=107, Damage ALL=54/158/160

**TDD — Specific Test Cases** (in `compiler/src/constants.rs` as `#[cfg(test)]` module):
- `test_known_phase_codes_contains_all_21` — verify PHASE_CODES array has exactly 21 entries
- `test_known_tog_names_contains_all_15` — verify TOG_ITEMS array has 15 entries
- `test_is_known_phase_code_accepts_valid` — `is_known_phase_code('b')` returns true
- `test_is_known_phase_code_rejects_invalid` — `is_known_phase_code('x')` returns false
- `test_is_known_tog_name_accepts_togres` — `is_known_tog("togres")` returns true
- `test_is_known_tog_name_accepts_togresm` — `is_known_tog("togresm")` returns true (togres substring must not false-match)
- `test_is_known_side_position_accepts_aliases` — `is_known_side("mid")` and `is_known_side("middle")` both return true
- `test_is_known_richtext_tag_accepts_cu` — `is_known_richtext_tag("cu")` returns true
- `test_is_known_richtext_tag_rejects_unknown` — `is_known_richtext_tag("purple")` returns false
- `test_reward_tag_letter_m_maps_to_modifier` — verify reward tag lookup
- `test_hidden_modifier_names_complete` — verify the list has exactly 12 entries

**If Blocked**: This chunk has no dependencies. If Thunder's guide is ambiguous on a constant value, add a `// VERIFY:` comment and include the value provisionally.

**Verification**:
- [ ] Every constant traces back to a specific section in Thunder's guide
- [ ] No invented/hallucinated values
- [ ] Constants are `pub const` arrays or match functions, not inline magic strings
- [ ] `pub mod constants;` added to `lib.rs`
- [ ] All TDD tests pass: `~/.cargo/bin/cargo test constants`
- [ ] Compiles cleanly: `~/.cargo/bin/cargo check`

---

### Chunk 2: IR Type Definitions (Additive Only) [CRITICAL CHECKPOINT]

**Scope**: Add all NEW IR types. This chunk is ADDITIVE ONLY -- it does NOT modify any existing types (`ModifierChain`, `ChainSegment`, `Boss`, `AbilityData`, `StructuralContent`, `StructuralType`). Existing type modifications are deferred to Chunks 3 and 8 where the full blast radius is handled.

**Read First**:
- `compiler/src/ir/mod.rs` -- current IR types, to avoid naming collisions
- `compiler/src/ir/ops.rs` -- current CRUD operations
- `compiler/src/lib.rs` -- module structure
- `compiler/src/constants.rs` -- constants from Chunk 1 (referenced by new types)

**Files** (1 file):
- `compiler/src/ir/mod.rs` (MODIFY -- add new types only, no changes to existing types)

**Dependencies**: Chunk 1

**Constraints**:
- ADDITIVE ONLY: Do not modify `ModifierChain`, `ChainSegment`, `Boss`, `BossFightUnit`, `BossFightVariant`, `AbilityData`, `StructuralContent`, or `StructuralType`. Do not remove `SegmentKind` yet. Those changes happen in Chunks 3 and 8.
- All new types must derive: `Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema` — ALL types need `PartialEq` for round-trip and test assertions. The code samples in the "New Types" section already include `PartialEq` on every derive line.
- All `Option` fields must have `#[serde(skip_serializing_if = "Option::is_none")]`
- Box recursive types (`Phase` inside `PhaseContent::Boolean`, `PhaseContent::Linked`)
- No `std::fs` -- WASM-safe
- No `unwrap()` in new code

**Requirements**:
- Add all types from the "New Types" section of this spec:
  - `Phase`, `PhaseType` (no Unknown variant — unrecognized codes are parse errors), `PhaseContent` (no Unparsed variant — unparseable content is a parse error)
  - `SeqOption`
  - `RewardTag`, `RewardTagType` (11 variants)
  - `LevelScope`
  - `RefKind` enum (Item, Modifier, Monster — for entity references)
  - `ChainEntry` enum (Hat, Splice, Cast, Enchant, Learn, TogItem, Keyword, Facade, Sidesc, EntityRef, Memory — no Raw variant)
  - `SidePosition`, `TogType`
  - `ValueRef`
  - `RichText` (newtype wrapper: `pub struct RichText(pub String)`)
  - `FightDefinition`, `FightUnit` (these are NEW types; existing `BossFightUnit`/`BossFightVariant` stay until Chunk 8)
  - `ChoiceType`, `HeroChangeType`, `PhaseGenType`
  - `AbilityType` (Spell/Tactic)
  - `EntityWrapper` (Orb/Vase/Jinx/Egg)
  - `HiddenModifierType`
- Enforce phase recursion depth: add `pub const MAX_PHASE_DEPTH: usize = 10;` to constants or ir module
- Do NOT modify any existing types in this chunk

**TDD -- Specific Test Cases** (in `compiler/src/ir/mod.rs` as `#[cfg(test)]`):
- `test_phase_serialization_roundtrip` -- serialize a `Phase` to JSON and back
- `test_phase_boolean_boxed_recursion` -- construct Boolean phase with nested Linked; verify it compiles and serializes
- `test_richtext_newtype` -- `RichText::new("hello").as_str() == "hello"`
- `test_fight_unit_default_fields` -- construct `FightUnit` with only required fields
- `test_fight_definition_with_trigger` -- construct with trigger, verify serialization
- `test_chain_entry_variants_serialize` -- each `ChainEntry` variant serializes to distinct JSON
- `test_level_scope_serialization` -- LevelScope round-trips through JSON
- `test_phase_type_all_variants_serialize` -- every PhaseType variant serializes and deserializes correctly

**If Blocked**: If a type design is unclear, add the type with a `// TODO: verify against Thunder's guide` comment. The type can be refined in later chunks as long as the struct/enum skeleton is present.

**Verification**:
- [ ] `cargo check` passes
- [ ] `cargo test` passes -- ALL existing tests still pass (this chunk is additive, no breakage)
- [ ] JSON Schema generation succeeds
- [ ] All TDD test cases pass
- [ ] No `unwrap()` in new code
- [ ] All types are WASM-safe (no `std::fs`)

---

### Chunk 3: Structural Type + Error Variant Migration [CRITICAL CHECKPOINT]

**Scope**: Add new `StructuralContent` variants, `StructuralType` entries, `AbilityData.ability_type`, `CompilerError` variants, and `ModifierType` classifier stubs. This chunk does NOT touch `ChainSegment`, `SegmentKind`, or `ModifierChain` -- those are handled atomically in Chunk 4.

**Read First**:
- `compiler/src/ir/mod.rs` -- `StructuralContent`, `StructuralType`, `AbilityData` (post-Chunk 2)
- `compiler/src/ir/merge.rs` -- merge logic for structural types
- `compiler/src/ir/ops.rs` -- CRUD operations
- `compiler/src/error.rs` -- existing error variants
- `compiler/src/extractor/classifier.rs` -- `ModifierType` enum
- `compiler/src/extractor/mod.rs` -- dispatch arms
- `compiler/src/extractor/structural_parser.rs` -- structural content parsing
- `compiler/src/builder/structural_emitter.rs` -- structural emission
- `compiler/src/builder/mod.rs` -- assembly order and filter arms

**Files** (9 files -- migration chunk, enum variants must move in lockstep for Rust exhaustive match):
- `compiler/src/ir/mod.rs` (MODIFY -- add StructuralContent/StructuralType variants, add ability_type to AbilityData)
- `compiler/src/ir/merge.rs` (MODIFY -- handle new StructuralType variants in merge logic)
- `compiler/src/ir/ops.rs` (MODIFY -- handle new structural types in CRUD)
- `compiler/src/error.rs` (MODIFY -- add ChainParseError, PhaseParseError, RewardParseError variants)
- `compiler/src/extractor/structural_parser.rs` (MODIFY -- add stub parse arms for new StructuralContent variants)
- `compiler/src/builder/structural_emitter.rs` (MODIFY -- emit new StructuralContent variants via body.clone())
- `compiler/src/builder/mod.rs` (MODIFY -- add emission loops for new StructuralType variants; without this, new types are silently dropped from output)
- `compiler/src/extractor/classifier.rs` (MODIFY -- add new ModifierType variants: PhaseModifier, Choosable, ValueModifier, HiddenModifier, FightModifier. Detection patterns are STUBS -- classify() never produces these variants until Chunk 7 adds real patterns)
- `compiler/src/extractor/mod.rs` (MODIFY -- add dispatch arms for new ModifierType variants, routing to structural_parser)

**Dependencies**: Chunk 2

**Constraints**:
- New `StructuralContent` variants: `PhaseModifier`, `Choosable`, `ValueModifier`, `HiddenModifier`, `FightModifier` -- each with a `body: String` field like existing variants, for round-trip safety.
- New `StructuralType` entries: `PhaseModifier`, `Choosable`, `ValueModifier`, `HiddenModifier`, `FightModifier` to match.
- `StructuralContent::body()` and `StructuralContent::from_body()` exhaustive match arms must be updated for new variants.
- `AbilityData`: add `#[serde(skip_serializing_if = "Option::is_none")] pub ability_type: Option<AbilityType>` -- existing parse/emit functions unchanged (ability_type is derived later by validator).
- New `CompilerError` variants: `ChainParseError`, `PhaseParseError`, `RewardParseError` -- each with structured fields (content, position, expected, found). These are added now so later chunks can use them without modifying `error.rs`.
- Classifier stubs: new `ModifierType` variants compile but classify() never produces them. The new `ModifierType` variants simply have no matching pattern in classify() until Chunk 7 adds real patterns.
- `builder/mod.rs` MUST add emission loops (`.filter()` blocks) for every new `StructuralType` variant, or those modifiers will be silently dropped from output.
- This chunk does NOT touch `ChainSegment`, `SegmentKind`, or `ModifierChain`. Those are handled atomically in Chunk 4.

**TDD -- Specific Test Cases**:
- `test_structural_content_new_variants_have_body` -- PhaseModifier, Choosable etc. all return body from `.body()`
- `test_structural_content_from_body_new_variants` -- `from_body()` handles new StructuralType entries
- `test_ability_data_with_ability_type_none` -- existing AbilityData serialization unchanged when ability_type is None
- `test_merge_handles_new_structural_types` -- merge with new StructuralType variants works
- `test_new_modifier_types_exist` -- verify `ModifierType::PhaseModifier`, `Choosable`, `ValueModifier`, `HiddenModifier`, `FightModifier` enum variants compile
- `test_existing_classifier_not_regressed` -- classify all modifiers from 4 test mods, verify identical results to pre-migration baseline
- `test_structural_emitter_new_variants` -- each new StructuralContent variant emits its body correctly
- `test_new_structural_types_emitted_not_dropped` -- verifies `builder/mod.rs` includes new types in emission output
- `test_new_error_variants_display` -- `CompilerError::ChainParseError`, `PhaseParseError`, `RewardParseError` produce descriptive Display output

**If Blocked**: Focus on the StructuralContent additions first. The classifier stub variants can have the detect functions added but classify() can initially never produce them (real detection patterns are added in Chunk 7). The new ModifierType variants simply have no matching pattern in classify() until Chunk 7.

**Verification**:
- [ ] `cargo test` -- ALL tests pass (0 failures). Non-negotiable for a migration chunk.
- [ ] `cargo clippy` -- no new warnings
- [ ] Round-trip fidelity preserved for all 4 test mods
- [ ] New `StructuralContent` variants compile and serialize
- [ ] `StructuralContent::body()` and `StructuralContent::from_body()` exhaustive match arms updated
- [ ] `ir/merge.rs` handles new structural types without panicking
- [ ] `builder/mod.rs` emission of new structural types verified
- [ ] `builder/mod.rs` has emission loops for all new StructuralType variants
- [ ] AbilityData serialization unchanged when ability_type is None
- [ ] New `CompilerError` variants compile and Display correctly

---

### Chunk 4: Chain Migration + Typed Chain Entry Parsing [CRITICAL CHECKPOINT]

**Scope**: Replace the existing `ChainSegment` struct and `SegmentKind` enum with the new `ChainSegment` enum (Item/Sticker). Remove `SegmentKind` entirely. Rewrite `ModifierChain::parse()` and `emit()`. AND parse `#`-delimited content within each segment into fully typed `Vec<ChainEntry>` entries -- all atomically in one chunk. When `ChainSegment` becomes an enum, its `sub_entries` are immediately populated with typed `ChainEntry` variants. No interim raw state, no empty sub_entries, no `raw_content` field.

**Read First**:
- `compiler/src/ir/mod.rs` -- existing `ChainSegment`, `SegmentKind`, `ModifierChain` (to be migrated), `ChainEntry` enum variants (from Chunk 2)
- `compiler/src/constants.rs` -- tog item names, side position names

**Files** (5 files):
- `compiler/src/ir/mod.rs` (MODIFY -- replace ChainSegment struct with enum, remove SegmentKind, rewrite ModifierChain::parse()/emit())
- `compiler/src/extractor/chain_parser.rs` (NEW -- parse #-delimited content into typed ChainEntry variants)
- `compiler/src/builder/chain_emitter.rs` (NEW -- emit Vec<ChainEntry> back to #-delimited content)
- `compiler/src/extractor/mod.rs` (MODIFY -- add `pub mod chain_parser;`)
- `compiler/src/builder/mod.rs` (MODIFY -- add `pub mod chain_emitter;`)

**Dependencies**: Chunks 2, 3
**Parallel with**: Chunk 5

**Constraints**:
- The new `ChainSegment` enum (Item/Sticker) preserves the `.i.`/`.sticker.` segment boundary semantics. The `sub_entries: Vec<ChainEntry>` field IS populated with typed entries immediately during this chunk -- there is no interim state where sub_entries is empty.
- `ModifierChain::parse()` and `ModifierChain::emit()` must be rewritten to work with the new enum AND call the chain entry parser/emitter for sub_entries.
- `SegmentKind` enum is REMOVED entirely. Every existing `ChainSegment { kind: SegmentKind::Item, content }` becomes `ChainSegment::Item { name, position, sub_entries }` with sub_entries populated. There is no Raw fallback.
- Round-trip MUST be preserved: `emit(parse(segment)) == original_content` for all segments in all 4 test mods.
- Segments that can't be parsed return a `CompilerError` identifying the unparseable content. No silent fallback.
- Chain order must be preserved -- tog item application order matters.

**Requirements**:
- Replace `ChainSegment` struct with enum, remove `SegmentKind`, rewrite `ModifierChain::parse()`/`emit()` to work with new enum variants.
- Parse `#`-delimited sub-entries within a chain segment into typed `ChainEntry` variants:
  - Detect `hat.Entity` -> `ChainEntry::Hat { entity, position }`
  - Detect `togXXX` -> `ChainEntry::TogItem { tog_type, position }`
  - Detect `k.keyword` -> `ChainEntry::Keyword { keyword, position }`
  - Detect `cast.effect` -> `ChainEntry::Cast { effect }`
  - Detect `splice.Item` -> `ChainEntry::Splice { item }`
  - Detect `enchant.Modifier` -> `ChainEntry::Enchant { modifier }`
  - Detect `learn.Ability` -> `ChainEntry::Learn { ability }`
  - Detect `facade.Code:Param` -> `ChainEntry::Facade { entity_code, parameter }`
  - Detect `sidesc.text` -> `ChainEntry::Sidesc { text }`
  - Detect `Memory` -> `ChainEntry::Memory`
  - Detect side positions as prefixes on any entry
  - Detect entity references: `r<type>.<hex_hash>[.part.<n>][.m.<n>]`
  - Unrecognized entries: return `CompilerError::ChainParseError` with the entry content
- Emit `Vec<ChainEntry>` back to `#`-delimited content string
- Handle nested parenthesized content: `(left.hat.Ace)#(togres)#(Memory)`

**TDD -- Specific Test Cases**:
- `test_chain_segment_item_roundtrip` -- construct `ChainSegment::Item` with typed sub_entries, emit and re-parse, compare
- `test_chain_segment_sticker_roundtrip` -- same for Sticker
- `test_modifier_chain_parse_new_enum` -- existing chain strings still parse correctly with new enum
- `test_modifier_chain_emit_new_enum` -- existing chain emit still produces same output
- `test_modifier_chain_roundtrip_all_test_mods` -- extract chains from all 4 test mods, emit, compare
- `test_parse_keyword_entry` -- `"k.scared"` -> `ChainEntry::Keyword { keyword: "scared", position: None }`
- `test_parse_facade_entry` -- `"facade.bas170:55"` -> `ChainEntry::Facade { entity_code: "bas170", parameter: "55" }`
- `test_parse_hat_entry` -- `"hat.Ace"` -> `ChainEntry::Hat { entity: "Ace", position: None }`
- `test_parse_tog_entry` -- `"togres"` -> `ChainEntry::TogItem { tog_type: TogType::Res, position: None }`
- `test_parse_tog_with_position` -- `"left.togtime"` -> `ChainEntry::TogItem { tog_type: TogType::Time, position: Some(SidePosition::Left) }`
- `test_parse_memory` -- `"Memory"` -> `ChainEntry::Memory`
- `test_parse_sidesc` -- `"sidesc.Add [pink]dejavu[cu]"` -> `ChainEntry::Sidesc { text: "Add [pink]dejavu[cu]" }`
- `test_parse_entity_ref_item` -- `"ritemx.dae9"` -> `ChainEntry::EntityRef { kind: RefKind::Item, hash: "dae9", part: None, multiplier: None }`
- `test_parse_entity_ref_with_part` -- `"ritemx.132fb.part.1"` -> `ChainEntry::EntityRef { kind: RefKind::Item, hash: "132fb", part: Some(1), multiplier: None }`
- `test_parse_entity_ref_modifier` -- `"rmod.1270"` -> `ChainEntry::EntityRef { kind: RefKind::Modifier, hash: "1270", part: None, multiplier: None }`
- `test_parse_entity_ref_monster` -- `"rmon.8"` -> `ChainEntry::EntityRef { kind: RefKind::Monster, hash: "8", part: None, multiplier: None }`
- `test_parse_entity_ref_with_multiplier` -- `"ritemx.22c42be4.part.0.m.2"` -> `ChainEntry::EntityRef { kind: RefKind::Item, hash: "22c42be4", part: Some(0), multiplier: Some(2) }`
- `test_parse_truly_unknown_errors` -- fabricated nonsense `"zzz_invalid_xyz"` -> returns `CompilerError::ChainParseError` with content
- `test_parse_hash_delimited_chain` -- `"k.scared#facade.bas170:55"` -> 2 typed entries
- `test_roundtrip_chains_from_test_mods` -- extract all chains from 4 test mods, parse entries, emit, compare
- `test_all_chain_entries_typed_across_test_mods` -- extract ALL chain segments from all 4 test mods, parse every `#`-delimited entry, assert zero `CompilerError` results. **This test MUST run and pass before the no-Raw constraint is considered proven.** If any entry from a real mod fails to parse, add a new `ChainEntry` variant for the pattern rather than introducing a Raw fallback. This is the critical gatekeeping test for the no-Raw design.
- `test_ref_kind_covers_all_prefixes` -- extract all `r`-prefixed entries from 4 test mods, verify every prefix maps to an existing `RefKind` variant (Item/Modifier/Monster). If an unknown `r<type>.` prefix is found, add a new `RefKind` variant.
- `test_nested_parens_preserved` -- `"(left.hat.(statue.sd.15-2))"` round-trips correctly

**If Blocked**: If a specific entry pattern is ambiguous, consult Thunder's guide directly. If the pattern is genuinely undocumented, add a new `ChainEntry` variant for it rather than falling back to raw. Ask the user if the pattern's semantics are unclear.

**Verification**:
- [ ] `cargo test` -- ALL tests pass (0 failures). Non-negotiable for a migration chunk.
- [ ] `cargo clippy` -- no new warnings
- [ ] Round-trip fidelity preserved for all 4 test mods
- [ ] `SegmentKind` fully removed from codebase (grep confirms zero references)
- [ ] 100% of chain entries from all 4 test mods parse into typed variants
- [ ] Round-trip: `emit(parse(segment)) == original_content` for chains from all 4 test mods
- [ ] Tog items correctly identified and typed
- [ ] Side positions correctly parsed as prefixes
- [ ] Nested parenthesized segments preserved
- [ ] No Raw/Unparsed fallback variants used -- all content is typed or errors

---

### Chunk 5: Level Scoping & Richtext Validation [PARALLEL GROUP A]

**Scope**: Parse level scope prefixes and validate richtext formatting. These are small, self-contained systems used by many other chunks.

**Read First**:
- `compiler/src/ir/mod.rs` -- `LevelScope` type definition (from Chunk 2)
- `compiler/src/util.rs` -- existing utility functions
- `compiler/src/constants.rs` -- richtext color tag constants

**Files** (3 files):
- `compiler/src/extractor/level_scope_parser.rs` (NEW)
- `compiler/src/extractor/mod.rs` (MODIFY -- add `pub mod level_scope_parser;`)
- `compiler/src/validator.rs` (MODIFY -- add `validate_richtext(rt: &RichText) -> Vec<Finding>` function)

**Dependencies**: Chunks 2, 3 (LevelScope type from Chunk 2; waits for Chunk 3 to stabilize ir/mod.rs compilation state)
**Parallel with**: Chunk 4

**Constraints**:
- Both must be pure functions (no side effects, WASM-safe)
- Level scope parser returns `(Option<LevelScope>, &str)` -- the scope and the remaining unparsed string
- Richtext validation lives in `validator.rs` (not `ir/mod.rs`) to avoid a circular module dependency: `ir` cannot import `Finding` from `validator` because `validator` imports from `ir`

**Requirements**:
- Level scope parser:
  - Parse `N.` -> `LevelScope { start: N, end: None, interval: None, offset: None }`
  - Parse `N-M.` -> `LevelScope { start: N, end: Some(M), interval: None, offset: None }`
  - Parse `eN.` -> `LevelScope { start: 0, end: None, interval: Some(N), offset: None }`
  - Parse `eN.M.` -> `LevelScope { start: 0, end: None, interval: Some(N), offset: Some(M) }`
  - Parse `lvl.` prefix combinations
  - Return `None` for no scope prefix
  - Emit function: `emit_level_scope(scope: &LevelScope) -> String`
- Richtext validation (implement as free function in `validator.rs`):
  - Validate bracket balance for color tags
  - Warn on unknown color tag names (reference constants)
  - Validate `[EntityName]` references (structural check only -- entity resolution is a later validator concern)
  - Function signature: `pub fn validate_richtext(rt: &RichText) -> Vec<Finding>` in `validator.rs`

**TDD -- Specific Test Cases**:
- `test_parse_single_floor` -- `"5.ph.4"` -> `LevelScope { start: 5, end: None, .. }`, remaining `"ph.4"`
- `test_parse_floor_range` -- `"3-7.ph.!"` -> `LevelScope { start: 3, end: Some(7), .. }`
- `test_parse_every_n` -- `"e2.ph.4"` -> `LevelScope { interval: Some(2), .. }`
- `test_parse_every_n_offset` -- `"e3.1.ph.4"` -> `LevelScope { interval: Some(3), offset: Some(1), .. }`
- `test_parse_no_scope` -- `"ph.4"` -> `None`, remaining `"ph.4"`
- `test_level_scope_roundtrip` -- `parse(emit(scope)) == scope` for all variants
- `test_richtext_balanced_tags` -- `"[orange]hello[cu]"` -> no findings
- `test_richtext_unbalanced_tag` -- `"[orange]hello"` -> warning about unclosed tag
- `test_richtext_unknown_tag` -- `"[purple]hello[cu]"` -> warning about unknown tag
- `test_richtext_entity_reference` -- `"[EntityName]"` -> no error (structural check only)
- `test_richtext_nested_tags` -- `"[orange][yellow]hi[cu][cu]"` -> no findings

**If Blocked**: If richtext validation edge cases are unclear, implement bracket balance checking first and add tag-name validation as a second pass within this chunk.

**Verification**:
- [ ] `cargo test` -- all tests pass
- [ ] Level scope round-trips: `parse(emit(scope)) == scope`
- [ ] Richtext validator catches unbalanced `[tags`
- [ ] Richtext validator accepts valid formatting from Thunder's guide examples
- [ ] Unit tests for all level scope patterns
- [ ] Unit tests for richtext edge cases (nested tags, `[cu]` closing)

---

### Chunk 6: Phase System Parser [SEQUENTIAL AFTER CHUNK 5]

**Scope**: Parse phase strings into the `Phase` IR type. This is the most complex new parser -- phases are recursive (LinkedPhase contains phases, BooleanPhase branches to phases).

**Read First**:
- `compiler/src/ir/mod.rs` -- `Phase`, `PhaseType`, `PhaseContent` types (from Chunk 2)
- `compiler/src/constants.rs` -- phase type codes, delimiter table
- `compiler/src/extractor/level_scope_parser.rs` -- LevelScope parser (from Chunk 5)
- `reference/UndocumentedTextmodGuide_V3.2_.html` -- phase syntax documentation

**Files** (2 files):
- `compiler/src/extractor/phase_parser.rs` (NEW)
- `compiler/src/extractor/mod.rs` (MODIFY -- add `pub mod phase_parser;`)

**Dependencies**: Chunks 2, 3, 5 (LevelScope parser from Chunk 5; PhaseParseError from Chunk 3)
**Note**: NOT parallel with Chunk 5 -- hard dependency on LevelScope parser. Chunk 4 may still be running when this starts.

**Constraints**:
- Enforce bounded recursion: track depth, return `CompilerError::PhaseParseError` at `MAX_PHASE_DEPTH` (10) with a message indicating the nesting limit was exceeded
- No silent fallbacks -- if a phase cannot be parsed, return a structured `CompilerError` with the phase type, position, and what was expected vs. found
- No `unwrap()` -- all parsing returns `Result`

**Requirements**:
- Parse `ph.X` prefix to determine phase type
- Parse phase-type-specific content:
  - `ph.!` -> SimpleChoice: parse `@3`-delimited rewards, optional `;` title
  - `ph.4` -> Message: parse text, optional `;` button text
  - `ph.5` -> HeroChange: parse 2 digits (position + type)
  - `ph.b` -> Boolean: parse `value;threshold;phaseA@2phaseB`
  - `ph.l` -> Linked: parse `@1`-delimited sub-phases (recursive)
  - `ph.s` -> Seq: parse initial message, then `@1`/`@2`-delimited options/phases
  - `ph.c` -> Choice: parse type (`PointBuy#N`, `Number#N`, `UpToNumber#N`, `Optional#N`), then `;` and `@3`-delimited rewards
  - `ph.t` -> Trade: parse `@3`-delimited rewards
  - `ph.r` -> RandomReveal: parse single reward
  - `ph.2` -> LevelEnd: parse JSON-like `{ps:[phase,phase]}`
  - `ph.8` -> PositionSwap: parse 2 digits
  - `ph.7` -> ItemCombine: parse type string
  - `ph.9` -> Challenge: parse JSON structure
  - `ph.g` -> PhaseGenerator: parse single char (`h` or `i`)
  - `ph.z` -> Boolean2: same as Boolean but with `@6`/`@7` delimiters
  - `ph.e` -> RunEnd (no content)
  - `ph.6` -> Reset (no content)
  - `ph.0`, `ph.1`, `ph.3`, `ph.d` -> minimal content phases
- Parse `phi.N` and `phmp.+-` shorthand forms
- Parse level scope prefix on phases: `N.ph.X`, `N-M.ph.X`, `eN.ph.X`
- Handle recursive phase nesting (LinkedPhase, BooleanPhase, SeqPhase all contain sub-phases)
- Return `CompilerError::PhaseParseError` when content can't be parsed -- include the phase type code, the raw content, and a description of what was expected

**TDD -- Specific Test Cases**:
- `test_parse_message_phase` -- `"ph.4Hello World"` -> `PhaseContent::Message { text: RichText("Hello World"), button_text: None }`
- `test_parse_message_with_button` -- `"ph.4Hello;OK"` -> `PhaseContent::Message { text: .., button_text: Some("OK") }`
- `test_parse_boolean_phase` -- `"ph.bscore;5;!m(ph.4You win)@2!m(ph.4Try again)"` -> Boolean with nested Message phases
- `test_parse_linked_phase` -- `"ph.l(ph.4Step 1)@1(ph.4Step 2)@1(ph.4Step 3)"` -> Linked with 3 sub-phases
- `test_parse_seq_phase` -- `"ph.sChoose@1Option A@2!m(ph.4Chose A)@1Option B@2!m(ph.4Chose B)"` -> Seq with 2 options
- `test_parse_simple_choice` -- `"ph.!m(modifier1)@3m(modifier2)"` -> SimpleChoice with 2 rewards
- `test_parse_run_end` -- `"ph.e"` -> PhaseContent::RunEnd
- `test_parse_reset` -- `"ph.6"` -> PhaseContent::Reset
- `test_parse_position_swap` -- `"ph.813"` -> PositionSwap { first: 1, second: 3 }
- `test_parse_unknown_phase_code_errors` -- `"ph.qsomething"` -> returns `CompilerError::PhaseParseError` with unrecognized code 'q'
- `test_parse_with_level_scope` -- `"5.ph.4Hello"` -> Phase with level_scope Some(LevelScope { start: 5, .. })
- `test_parse_deeply_nested_errors` -- 11 levels of nesting -> returns `CompilerError::PhaseParseError` with depth limit message
- `test_parse_malformed_content_errors` -- malformed phase content returns `CompilerError` with expected vs. found description
- `test_phase_roundtrip_emit_parse` -- for each phase type, emit then re-parse, compare

**If Blocked**: If a specific phase type's syntax is unclear from Thunder's guide, consult the guide directly and test with real mod content from the 4 test mods. If the syntax is genuinely undocumented, ask the user rather than silently accepting unparseable content.

**Verification**:
- [ ] `cargo test` -- all tests pass
- [ ] All 21 phase types recognized by type code
- [ ] BooleanPhase recursive parsing works (nested conditionals)
- [ ] LinkedPhase chains of 4+ phases parse correctly
- [ ] SeqPhase with multiple options and sub-phases parses correctly
- [ ] Phase delimiter table validated: @1/@2/@3/@4/@6/@7 used correctly per type
- [ ] Unrecognized phase codes produce structured CompilerError (not silent fallback)
- [ ] Malformed phase content produces structured CompilerError with expected vs. found
- [ ] Level scope prefixes on phases parsed correctly
- [ ] No stack overflow on deeply nested phases (bounded recursion enforced, errors at limit)

---

### Chunk 7: Choosable & Reward Tag System [PARALLEL GROUP B]

**Scope**: Parse choosable tags (`ch.X`) and reward references. These are the reward-granting system used by phases, choosables, and structural modifiers.

**NOTE: This chunk has 6 primary files (exceeds the 5-file limit by 1). The two NEW files (reward_parser, reward_emitter) are a single parser/emitter pair -- splitting them into a separate chunk would create a chunk with only 2 files and no useful verification boundary. The 4 MODIFY files are single-line module declarations or pattern additions.**

**Read First**:
- `compiler/src/ir/mod.rs` -- `RewardTag`, `RewardTagType`, `ValueRef` types (from Chunk 2)
- `compiler/src/extractor/classifier.rs` -- current classifier patterns
- `compiler/src/extractor/structural_parser.rs` -- current structural content parsing

**Files** (5 files):
- `compiler/src/extractor/reward_parser.rs` (NEW)
- `compiler/src/builder/reward_emitter.rs` (NEW)
- `compiler/src/extractor/structural_parser.rs` (MODIFY -- parse new StructuralContent variants)
- `compiler/src/extractor/classifier.rs` (MODIFY -- add choosable/value/phase classifier patterns)
- `compiler/src/extractor/mod.rs` (MODIFY -- add `pub mod reward_parser;` + `pub mod reward_emitter;` declaration forwarded from builder)

NOTE: `builder/mod.rs` also needs `pub mod reward_emitter;` added (1 line). This is listed as a mechanical addition rather than a primary file.

**Dependencies**: Chunks 2, 3, 6 (phase types for nested phases in rewards)
**Parallel with**: Chunk 8

**Constraints**:
- Classifier changes must not break existing classification -- add new patterns at correct priority positions. Specifically:
  - `BossEncounter` pattern (`1.ph.bX;1;!m(`) must remain BEFORE any new phase modifier pattern (`N.ph.X`)
  - Phase modifier patterns (`N.ph.X` without `;1;!m(`) must be added AFTER BossEncounter and Hero checks
  - Choosable patterns (`ch.`) must be added AFTER BossModifier check (which tests for `ch.om`)
  - Value modifier patterns (`v(name)V(amount)`) need careful positioning to avoid conflict with other `v`-prefixed content
  - Add a `test_classifier_priority_order` test that verifies all existing classifications are preserved after new patterns are added
- New `StructuralType` entries were added in Chunk 3; this chunk populates their parse logic in `structural_parser.rs`

**Requirements**:
- Parse reward tag syntax:
  - Standard tags: `m[Modifier]`, `i[Item]`, `l[Levelup]`, `g[Hero]`
  - Random: `r(tier)~(amount)~(tag)` -> separate fields
  - RandomRange: `q(tier1)~(tier2)~(amount)~(tag)`
  - Or: `o[tag1]@4[tag2]@4...`
  - Value: `v(name)V(amount)` -> `ValueRef { name, amount }`
  - Replace: `pm(modifier)~(reward)`
  - Skip: `s` (no content)
  - Enu: `e[template]`
- Parse choosable modifiers: `ch.X[content]` -> `StructuralContent::Choosable`
- Parse value modifiers: `ch.v[name]V[amount]` -> `StructuralContent::ValueModifier`
- Parse phase modifiers: `N.ph.X[content]` -> `StructuralContent::PhaseModifier`
- Update classifier to recognize choosable, value, and phase modifiers as distinct `ModifierType` variants
- Emit reward tags back to textmod syntax

**TDD -- Specific Test Cases**:
- `test_parse_modifier_tag` -- `"m(skip&hidden)"` -> `RewardTag { tag_type: Modifier, content: "skip&hidden" }`
- `test_parse_item_tag` -- `"i(Sword)"` -> `RewardTag { tag_type: Item, content: "Sword" }`
- `test_parse_random_tag` -- `"r1~3~i"` -> `RewardTag { tag_type: Random, content: "1~3~i" }`
- `test_parse_random_negative_tier` -- `"r-1~2~m"` -> parses correctly with negative tier
- `test_parse_value_tag` -- `"vscoreV50"` -> `ValueRef { name: "score", amount: 50 }`
- `test_parse_or_tag` -- `"om(A)@4m(B)@4m(C)"` -> Or tag with 3 alternatives
- `test_parse_replace_tag` -- `"pm(old_mod)~(new_reward)"` -> parsed into modifier + reward
- `test_parse_skip_tag` -- `"s"` -> `RewardTag { tag_type: Skip, content: "" }`
- `test_classifier_choosable` -- modifier starting with `ch.` classified as Choosable
- `test_classifier_phase_modifier` -- `"5.ph.4Hello"` classified as PhaseModifier
- `test_classifier_value_modifier` -- `"ch.vscoreV10"` classified as ValueModifier
- `test_classifier_existing_types_unchanged` -- existing hero/monster/boss classification unchanged
- `test_classifier_priority_order` -- run classifier on all modifiers from 4 test mods, verify zero regressions vs baseline
- `test_reward_tag_roundtrip` -- `emit(parse(tag)) == tag` for all 11 tag types

**If Blocked**: If a specific tag type's syntax is ambiguous, consult Thunder's guide and test against real mod content. If genuinely unclear, ask the user rather than storing content as a raw string.

**Verification**:
- [ ] `cargo test` -- all tests pass
- [ ] All 11 tag types parse correctly
- [ ] Random tag `tier~amount~tag` parsing handles negative tiers
- [ ] Value tag `vNameV50` correctly splits name from amount
- [ ] Or tag `@4` delimiter handled at depth 0
- [ ] Replace tag `pm(old)~(new)` parsed into modifier + reward
- [ ] Classifier correctly distinguishes choosable vs phase vs value modifiers
- [ ] Existing classifier tests still pass (no regression)
- [ ] Round-trip: `emit(parse(tag)) == tag`

---

### Chunk 8: Fight Generalization + Boss Migration [CRITICAL CHECKPOINT]

**Scope**: Replace `BossFightUnit` and `BossFightVariant` with `FightUnit` and `FightDefinition`. Replace `Boss.event_body` with `Boss.event_phases`. Extract shared fight parsing/emission from boss_parser and boss_emitter into reusable modules. Refactor boss event parsing to use the phase parser. Add entity wrapper parsing. This chunk handles both the type-level migration and the parser/emitter refactor atomically.

Full blast radius: ir/mod.rs, boss_parser, boss_emitter, ir/merge.rs, ir/ops.rs, fight_parser (NEW), fight_emitter (NEW), extractor/mod.rs, builder/mod.rs, tests/builder_tests.rs, tests/expansion_tests.rs, tests/boss_tests.rs. This chunk exceeds 5 files because it is a type migration -- partial migration would leave the codebase in an inconsistent state with both old and new types coexisting.

**Read First**:
- `compiler/src/ir/mod.rs` -- current `Boss`, `BossFightUnit`, `BossFightVariant` types; new `FightUnit`, `FightDefinition` types (from Chunk 2)
- `compiler/src/ir/merge.rs` -- boss merge logic
- `compiler/src/ir/ops.rs` -- boss CRUD operations
- `compiler/src/extractor/boss_parser.rs` -- current fight parsing (functions to extract into shared module)
- `compiler/src/builder/boss_emitter.rs` -- current fight emission
- `compiler/src/extractor/phase_parser.rs` -- phase parser (from Chunk 6)

**Files** (12 files -- type migration, all files must move in lockstep):
- `compiler/src/ir/mod.rs` (MODIFY -- remove `BossFightUnit`, `BossFightVariant`; change `Boss.variants` to `Boss.fights: Vec<FightDefinition>`; change `Boss.event_body` to `Boss.event_phases: Option<Vec<Phase>>`)
- `compiler/src/ir/merge.rs` (MODIFY -- update boss merge for new field names)
- `compiler/src/ir/ops.rs` (MODIFY -- update boss CRUD for new field names)
- `compiler/src/extractor/fight_parser.rs` (NEW -- extracted from boss_parser)
- `compiler/src/builder/fight_emitter.rs` (NEW -- extracted from boss_emitter)
- `compiler/src/extractor/boss_parser.rs` (MODIFY -- use shared fight parser, parse event_phases via phase parser)
- `compiler/src/builder/boss_emitter.rs` (MODIFY -- use shared fight emitter, emit event_phases)
- `compiler/src/extractor/mod.rs` (MODIFY -- add `pub mod fight_parser;`)
- `compiler/src/builder/mod.rs` (MODIFY -- add `pub mod fight_emitter;`)
- `compiler/tests/builder_tests.rs` (MODIFY -- BossFightUnit/BossFightVariant -> FightUnit/FightDefinition)
- `compiler/tests/expansion_tests.rs` (MODIFY -- event_body -> event_phases)
- `compiler/tests/boss_tests.rs` (MODIFY -- boss field renames)

**Dependencies**: Chunks 2, 3, 4, 6 (FightUnit/FightDefinition types from Chunk 2, ChainSegment enum from Chunk 4 for modifier chains, Phase from Chunk 6 for event_phases)
**Parallel with**: Chunk 7

**Constraints**:
- `FightUnit` includes all fields from `BossFightUnit` plus two new optional fields (`color: Option<char>`, `hsv: Option<String>`) needed for general fight contexts. Existing boss fight units map cleanly -- new fields default to `None`.
- `FightDefinition` absorbs `BossFightVariant` fields: `name` (variant label), `trigger`, `fight_units` -> `enemies`
- `Boss.fights: Vec<FightDefinition>` replaces `Boss.variants: Vec<BossFightVariant>`
- `Boss.event_phases: Option<Vec<Phase>>` replaces `Boss.event_body: Option<String>`
- `modifier_chain` on `FightUnit` stays as `Option<ModifierChain>` (NOT `Option<Vec<ChainSegment>>`)
- `Boss.event_phases: Option<Vec<Phase>>` uses the phase parser (Chunk 6) to parse the event body into structured phases. If a phase can't be parsed, the parser returns a `CompilerError` -- no silent fallback. This is NOT a case for `raw_content` or any interim raw field.
- Fight parser is shared: used by boss_parser AND available for phase contexts (`!m(4.fight.bones+zombie)`)
- Entity wrapper parsing: detect `orb.s[entity]`, `vase.([content])`, `jinx.[modifier]`, `egg.[entity]` prefixes within fight units and chain segments. These are validation-level concepts used by the fight parser.
- NOTE on FightDefinition.name: This field is `Option<String>` (unlike BossFightVariant.name which is `String`). All existing code accessing `.name` must be updated to handle the Option.

**Requirements**:
- Remove `BossFightUnit` and `BossFightVariant` from IR. Update all references.
- Extract fight parsing from boss_parser into shared `fight_parser.rs`:
  - `parse_fight(content: &str) -> Result<FightDefinition, CompilerError>`
  - Parse `fight.Enemy1+Enemy2+Enemy3.mn.Name`
  - Parse individual fight units with properties (hp, sd, img, etc.)
- Extract fight emission from boss_emitter into shared `fight_emitter.rs`:
  - `emit_fight(fight: &FightDefinition) -> Result<String, CompilerError>`
- Refactor boss_parser to use shared fight parser and phase parser for events
- Refactor boss_emitter to use shared fight emitter and phase emitter
- Add entity wrapper detection in fight units:
  - Detect `orb.s[entity]` prefix -> `EntityWrapper::Orb`
  - Detect `vase.([content])` -> `EntityWrapper::Vase`
  - Detect `jinx.[modifier]` -> `EntityWrapper::Jinx`
  - Detect `egg.[entity]` -> `EntityWrapper::Egg`

**TDD -- Specific Test Cases**:
- `test_fight_unit_has_boss_fields` -- FightUnit has all BossFightUnit fields (template, name, hp, sd, sprite_data, template_override, doc, modifier_chain) plus color, hsv
- `test_fight_definition_has_variant_fields` -- FightDefinition has name, trigger, enemies (renamed from fight_units)
- `test_boss_fights_field_exists` -- Boss has `fights: Vec<FightDefinition>` (not `variants`)
- `test_boss_event_phases_field_exists` -- Boss has `event_phases: Option<Vec<Phase>>` (not `event_body`)
- `test_no_remaining_boss_fight_unit_refs` -- grep codebase for `BossFightUnit` returns 0 matches
- `test_no_remaining_boss_fight_variant_refs` -- grep codebase for `BossFightVariant` returns 0 matches
- `test_merge_boss_new_fields` -- merge with boss overlay using new field names works
- `test_fight_parser_single_unit` -- `"Sniper.hp.3.sd.0:0:0:0:0:0.n.Wooper"` -> FightDefinition with 1 FightUnit
- `test_fight_parser_multi_unit` -- `"Sniper.n.A+Basalt.n.B"` -> FightDefinition with 2 FightUnits
- `test_fight_parser_with_modifier_chain` -- fight unit with `.i.left.k.scared` -> modifier_chain populated
- `test_boss_standard_roundtrip` -- existing standard boss tests pass with new types
- `test_boss_encounter_roundtrip` -- existing encounter boss tests pass with new types
- `test_boss_event_phases_parsed` -- event boss `ch.om(...)` has event_phases populated (not just raw string)
- `test_boss_event_parse_error` -- malformed event content returns `CompilerError::PhaseParseError` (not silent fallback)
- `test_entity_wrapper_orb` -- `"orb.sSlimelet"` -> `EntityWrapper::Orb { entity: "Slimelet" }`
- `test_entity_wrapper_egg` -- `"egg.Zombie"` -> `EntityWrapper::Egg { entity: "Zombie" }`
- `test_fight_from_phase_context` -- `"!m(4.fight.bones+zombie)"` parses fight correctly

**If Blocked**: If event phase parsing is too complex, the fight generalization (shared fight_parser/fight_emitter) can proceed independently -- split the event_phases parsing into a follow-up. Do NOT keep `event_body: Option<String>` as a parallel field, and do NOT use Unparsed fallback.

**Verification**:
- [ ] `cargo test` -- ALL tests pass (0 failures). Non-negotiable for migration chunk.
- [ ] `cargo clippy` -- no new warnings
- [ ] No remaining references to `BossFightUnit` or `BossFightVariant` (grep to confirm)
- [ ] No remaining references to `Boss.variants` or `Boss.event_body` (grep to confirm)
- [ ] `ir/merge.rs` handles boss merges with new field names
- [ ] Boss parsing still works for all 3 formats (Standard, Event, Encounter)
- [ ] Round-trip fidelity preserved for all 4 test mods
- [ ] Shared fight parser handles all fight variants from test mods
- [ ] Entity wrapper types identified and parsed


---

### Chunk 9: Builders/Emitters for All New Types [PARALLEL GROUP C]

**Scope**: Implement emission for all new IR types -- phases, reward tags, chain entries, level scopes. Extend existing structural emitter for new variants.

**Read First**:
- `compiler/src/builder/mod.rs` -- current assembly order
- `compiler/src/builder/structural_emitter.rs` -- current structural emission
- `compiler/src/builder/chain_emitter.rs` -- chain entry emission (from Chunk 4)
- `compiler/src/builder/reward_emitter.rs` -- reward emission (from Chunk 7)
- `compiler/src/constants.rs` -- delimiter table

**Files** (4 files):
- `compiler/src/builder/phase_emitter.rs` (NEW)
- `compiler/src/builder/mod.rs` (MODIFY -- assembly order for new structural types, add `pub mod phase_emitter;`)
- `compiler/src/builder/structural_emitter.rs` (MODIFY -- emit new StructuralContent variants using typed fields where populated, body fallback otherwise)
- `compiler/src/lib.rs` (MODIFY -- add single-item build/parse/emit exports for new types)

**Dependencies**: Chunks 6, 7, 8 (all parsers and types must exist)
**Parallel with**: Chunk 10

**Constraints**:
- Balanced emission by construction -- no post-hoc paren checking for new emitters
- Recursive emission must respect `MAX_PHASE_DEPTH` (same bound as parser)
- No `unwrap()` in emitter code
- No `std::fs` -- WASM-safe

**Requirements**:
- Phase emitter:
  - `emit_phase(phase: &Phase) -> Result<String, CompilerError>`
  - Emit correct `ph.X` prefix for each type
  - Emit phase-specific content with correct delimiters (@1/@2/@3 etc.)
  - Handle recursive emission (LinkedPhase, BooleanPhase contain sub-phases)
  - Emit level scope prefix when present
- Structural emitter updates:
  - Emit `StructuralContent::PhaseModifier` -> `[scope.]ph.[content]`
  - Emit `StructuralContent::Choosable` -> `ch.[tag][content]`
  - Emit `StructuralContent::ValueModifier` -> `ch.v[name]V[amount]`
  - Emit `StructuralContent::HiddenModifier` -> modifier name
  - Emit `StructuralContent::FightModifier` -> `[scope.]fight.[enemies]`
- Assembly order in `builder/mod.rs`:
  - Phase modifiers, choosables, value modifiers slot into correct positions
  - Hidden modifiers emit at the end (or with their scope)
- Single-item exports in `lib.rs`:
  - `pub fn parse_phase(content: &str) -> Result<Phase, CompilerError>`
  - `pub fn emit_phase(phase: &Phase) -> Result<String, CompilerError>`
  - `pub fn parse_reward_tag(content: &str) -> Result<RewardTag, CompilerError>`
  - `pub fn emit_reward_tag(tag: &RewardTag) -> Result<String, CompilerError>`

**TDD -- Specific Test Cases**:
- `test_emit_message_phase` -- `PhaseContent::Message { text, button_text }` -> `"ph.4text;button"`
- `test_emit_boolean_phase` -- Boolean with nested sub-phases -> correct `@2` delimiter
- `test_emit_linked_phase` -- Linked with 3 phases -> correct `@1` delimiters
- `test_emit_seq_phase` -- Seq with options -> correct `@1`/`@2` delimiters
- `test_emit_simple_choice` -- SimpleChoice with rewards -> correct `@3` delimiters
- `test_emit_all_phase_variants` -- every PhaseContent variant has an emitter path (compile-time exhaustive match)
- `test_emit_level_scope_prefix` -- phase with LevelScope emits `"5.ph.4Hello"`
- `test_emit_structural_phase_modifier` -- PhaseModifier variant emits correct format
- `test_emit_structural_choosable` -- Choosable variant emits correct format
- `test_emit_structural_hidden_modifier` -- HiddenModifier variant emits modifier name
- `test_phase_emit_parse_roundtrip` -- for each phase type, emit then re-parse, compare IR
- `test_assembly_order_includes_new_types` -- build a ModIR with phase/choosable structurals, verify they appear in output

**If Blocked**: If a specific phase type's emission format is unclear, consult Thunder's guide and verify against real mod content from the 4 test mods. Every phase type must have an emitter -- there is no Unparsed fallback to lean on.

**Verification**:
- [ ] `cargo test` -- all tests pass
- [ ] Every `PhaseContent` variant has an emitter path (no `todo!()` or `unimplemented!()`)
- [ ] Phase emission produces valid phase strings per Thunder's guide syntax
- [ ] Recursive phase emission doesn't stack overflow
- [ ] Delimiter usage matches the delimiter table from constants.rs
- [ ] Level scope emission format matches parser expectations
- [ ] Structural emission produces valid modifier strings for all new variants
- [ ] No `unwrap()` in emitter code
- [ ] Single-item exports in lib.rs compile and work

---

### Chunk 10: Validator Rules for All New Systems [PARALLEL GROUP C]

**Scope**: Add validation rules for phases, modifier chains, reward tags, and all new IR types.

**Read First**:
- `compiler/src/validator.rs` -- existing validation rules, Finding struct, Severity levels
- `compiler/src/constants.rs` -- reference data for validation
- `compiler/src/ir/mod.rs` -- all new types to validate

**Files** (1 file):
- `compiler/src/validator.rs` (MODIFY -- add new validation rules)

**Reference files** (read but not modified):
- `compiler/src/constants.rs` -- reference data for validation lookups

**Dependencies**: Chunks 6, 7, 8 (all types and parsers)
**Parallel with**: Chunk 9

**Constraints**:
- All findings must use structured `Finding` with `rule_id`, `field_path`, and `suggestion` fields populated
- Warnings for unknown-but-valid-looking content (don't block on undocumented features)
- Errors only for structurally invalid content
- No `unwrap()` -- validators must never panic

**Requirements**:
- Phase validation (V001-V008):
  - V001: (handled at parse time -- unrecognized phase codes are CompilerError, not validation findings)
  - V002: BooleanPhase value references a Value that's been set somewhere in the mod
  - V003: LinkedPhase has 2+ sub-phases
  - V004: SeqPhase option count matches phase structure
  - V005: HeroChangePhase position 0-4
  - V006: ChallengePhase JSON structure valid
  - V007: ChoicePhase budget/count positive
  - V008: Phase delimiters match expected per type
- Modifier chain validation (V010-V015):
  - V010: Tog items use valid side positions
  - V011: Togres variants have valid boolean composition (e.g., togresa must follow togres)
  - V012: Hat references known entity templates
  - V013: Memory placement is valid (after tog modification)
  - V014: Facade format valid
  - V015: Keyword names from known list (warn on unknown, don't error -- game may have undocumented keywords)
- Reward validation (V020-V023):
  - V020: Tag type matches content format
  - V021: Random tag tier/amount/tag format valid
  - V022: Value names consistent across set/check sites
  - V023: Replace tag references existing modifier
- Composition validation (V030-V032):
  - V030: AbilityData side usage matches spell vs tactic rules (derive ability_type here)
  - V031: TriggerHPData HP in valid range
  - V032: Level scope range valid
- Richtext validation (V040-V042):
  - V040: Bracket balance
  - V041: Known color tags
  - V042: Entity reference format valid
- Single-item validation export in lib.rs:
  - `pub fn validate_phase(phase: &Phase) -> ValidationReport`

**TDD -- Specific Test Cases**:
- (V001 is a parse-time error, not a validation rule -- no validator test needed)
- `test_v003_linked_phase_too_few` -- LinkedPhase with 1 sub-phase produces V003 error
- `test_v005_hero_change_out_of_range` -- HeroChangePhase position 5 produces V005 error
- `test_v007_choice_negative_budget` -- ChoicePhase with budget -1 produces V007 error
- `test_v010_tog_invalid_position` -- tog item with invalid position produces V010 error
- `test_v015_unknown_keyword_warns` -- unknown keyword produces V015 warning (not error)
- `test_v021_random_tag_malformed` -- `"r~bad~"` produces V021 error
- `test_v030_ability_type_derived` -- AbilityData with Side 5 data derives AbilityType::Spell
- `test_v032_level_scope_invalid_range` -- LevelScope with start > end produces V032 error
- `test_v040_richtext_unbalanced` -- unbalanced bracket produces V040 error
- `test_v041_unknown_color_tag` -- `"[purple]"` produces V041 warning
- `test_validation_on_all_test_mods` -- validation runs without panicking on all 4 test mods
- `test_all_findings_have_field_path` -- every Finding produced has non-None field_path

**If Blocked**: Implement structural validation rules (V001, V003-V008, V010-V015, V030-V032, V040-V042) first. Cross-reference rules (V002, V022, V023) can be added in Chunk 11 if they require full-mod context.

**Verification**:
- [ ] `cargo test` -- all tests pass
- [ ] Each validation rule has at least one test case
- [ ] Warnings for unknown-but-valid-looking content (don't block on undocumented features)
- [ ] Errors only for structurally invalid content
- [ ] All findings include field_path and suggestion
- [ ] Validation runs without panicking on any test mod content
- [ ] Performance: validation completes in <100ms for full mod

---

### Chunk 11: Round-Trip Tests, Cross-References, Schema Update [INTEGRATION]

**Scope**: Verify everything works together. Round-trip all test mods with new types. Add cross-reference validation. Regenerate JSON Schema. This is the integration and polish chunk.

**Read First**:
- `compiler/tests/roundtrip_tests.rs` -- existing round-trip test structure
- `compiler/src/validator.rs` -- existing validation rules (add cross-reference rules)
- `compiler/src/lib.rs` -- verify all exports are in place

**Files** (4 files):
- `compiler/tests/roundtrip_tests.rs` (MODIFY -- expanded assertions for new types)
- `compiler/tests/integration_tests.rs` (NEW -- cross-system tests covering phases + chains + rewards + fights)
- `compiler/src/validator.rs` (MODIFY -- add cross-reference validation rules V002, V022, V023)
- `compiler/src/lib.rs` (MODIFY -- verify and document all public exports)

**Dependencies**: ALL previous chunks

**Constraints**:
- This chunk must NOT introduce new types or structural changes -- it is verification and polish only
- Cross-reference validation rules (V002, V022, V023) deferred from Chunk 10 are completed here

**Requirements**:
- Round-trip tests (expanded in `roundtrip_tests.rs`):
  - All 4 test mods in `working-mods/`: `sliceymon.txt`, `pansaer.txt`, `punpuns.txt`, `community.txt` (loaded via existing `load_mod()` helper)
  - `extract(build(extract(mod)))` IR identity for all types including new phase/chain/reward types
  - New types don't break existing round-trip (regression prevention)
  - Assert that ALL extracted phases are typed (no Unparsed variant exists)
  - Assert that ALL extracted chain entries are typed (no Raw variant exists)
- Integration tests (in `integration_tests.rs`):
  - Recursive nesting: LinkedPhase containing BooleanPhase containing SeqPhase
  - Complex tog item chains from Thunder's guide examples
  - Full mod build from hand-authored JSON IR (validates Path B)
  - Cross-reference: BooleanPhase value names match Value tag names
  - Cross-reference: fight unit enemy references resolve to known templates
- Cross-reference validation (in `validator.rs`):
  - V002: BooleanPhase value names match Value tag set sites
  - V022: Value names consistent across set/check sites
  - V023: Replace tag references existing modifier
- JSON Schema:
  - Regenerate schema with all new types via `cargo run -- schema`
  - Verify schema validates sample IR JSON for new types

**TDD -- Specific Test Cases**:
- `test_roundtrip_sliceymon_with_new_types` -- sliceymon.txt round-trips; extracted IR has typed phases/chains
- `test_roundtrip_pansaer_with_new_types` -- pansaer.txt round-trips
- `test_roundtrip_punpuns_with_new_types` -- punpuns.txt round-trips
- `test_roundtrip_community_with_new_types` -- community.txt round-trips
- `test_hand_authored_ir_builds` -- JSON IR with phases, fights, typed chains builds to valid textmod
- `test_recursive_phase_nesting` -- Linked containing Boolean containing Seq round-trips
- `test_cross_ref_boolean_value` -- BooleanPhase references value that doesn't exist -> V002 warning
- `test_cross_ref_value_consistency` -- value set in one place, checked in another -> no warning; set only -> warning
- `test_all_entries_typed` -- 100% of extracted chain entries are typed (no Raw/Unparsed variants exist in the enum)
- `test_schema_includes_new_types` -- generated schema contains Phase, ChainEntry, FightDefinition

**If Blocked**: If a specific test mod fails round-trip due to an edge case in a new parser, add the failing content as a focused regression test and file a bug for the responsible chunk's parser. Do not block integration testing on a single edge case.

**Verification**:
- [ ] `cargo test` -- all tests pass (0 failures)
- [ ] `cargo clippy` -- no warnings
- [ ] Round-trip identity holds for all 4 test mods
- [ ] JSON Schema generates successfully and validates sample IR
- [ ] All public exports documented in lib.rs
- [ ] No performance regression (full mod round-trip under 500ms)
- [ ] 100% of chain entries are typed (no Raw/Unparsed variants in the type system)
- [ ] No `unwrap()` in library code
- [ ] No `std::fs` in library code (WASM-safe)

---

## Final Verification (After All 11 Chunks)

Run these checks against the actual codebase:

- [ ] `cargo test` -- all tests pass (0 failures)
- [ ] `cargo clippy` -- clean (0 warnings)
- [ ] Round-trip fidelity for all 4 test mods in `working-mods/`: `sliceymon.txt`, `pansaer.txt`, `punpuns.txt`, `community.txt`
- [ ] JSON Schema covers all new types (Phase, ChainEntry, FightDefinition, RewardTag, LevelScope, etc.)
- [ ] Every constant/enum in `constants.rs` traces to Thunder's guide v3.2
- [ ] No `unwrap()` in library code (grep `compiler/src/` excluding `tests/` and `main.rs`)
- [ ] No `std::fs` in library code (grep to confirm WASM-safe)
- [ ] All new `pub fn` exports documented in lib.rs with doc comments
- [ ] Single-item build/validate works: `parse_phase`, `emit_phase`, `validate_phase`, `parse_reward_tag`, `emit_reward_tag`
- [ ] Cross-reference validation catches orphaned Value/phase/reward references
- [ ] No remaining references to removed types: `BossFightUnit`, `BossFightVariant`, `SegmentKind` (grep to confirm)
- [ ] No `Raw` or `Unparsed` variants in `ChainSegment`, `ChainEntry`, or `PhaseContent` enums
- [ ] No `Unknown` variant in `PhaseType` enum
- [ ] 100% of extracted chain entries and phases are typed -- parse errors surface as `CompilerError`
- [ ] Phase recursion depth bounded at `MAX_PHASE_DEPTH` (10) -- tested with deeply nested input (returns error)
- [ ] No `raw_content` field on any `ChainSegment` variant -- all entries are typed
- [ ] HANDOFF.md updated with final state

---

## Risk Register

| Risk | Mitigation |
|------|-----------|
| Phase recursion causes stack overflow | Enforced `MAX_PHASE_DEPTH = 10` in parser; returns `CompilerError` at limit. Tested in Chunk 6. Bounded at `MAX_PHASE_DEPTH` (10). |
| Thunder's guide has undocumented exceptions | No silent fallback -- unrecognized content produces `CompilerError`. If real mods use undocumented patterns, add new typed variants. Test against all 4 test mods to ensure coverage. |
| Modifier chain migration breaks existing mods | Chunk 4 handles chain migration atomically; `cargo test` must pass before proceeding. Round-trip on all 4 test mods. |
| Boss migration breaks existing mods | Chunk 8 handles boss migration atomically; same invariant. Event body -> event_phases uses phase parser (hard dependency on Chunk 6). |
| New types bloat JSON Schema beyond useful | Use `#[serde(skip_serializing_if)]` aggressively; keep optional fields optional. |
| Game updates add new phase types | New phase codes require adding a new `PhaseType` variant and `PhaseContent` variant. This is a code change, not a data change -- intentionally strict. |
| Delimiter collision in nested phases | Depth-aware parsing for all delimiters; BooleanPhase2 (`ph.z`) exists specifically to avoid delimiter collision. |
| Undiscovered chain entry patterns in test mods | Run chain parser against all 4 test mods in Chunk 4. `test_all_chain_entries_typed_across_test_mods` is the gatekeeping test. Any failures reveal patterns that need new `ChainEntry` variants. Fix before proceeding -- do NOT introduce a Raw fallback. |
| Undiscovered entity ref prefixes (`r<type>.`) | Gatekeeping test `test_ref_kind_covers_all_prefixes` in Chunk 4 runs against all 4 test mods. Any unknown prefix surfaces immediately as a CompilerError, requiring a new RefKind variant. |
| Migration chunks exceed 5-file guideline | Chunks 3 (9 files), 8 (12 files) are type migrations where Rust's exhaustive match arms require all files to move in lockstep. Partial migration won't compile. Documented with justification in each chunk. |
| Migration chunks touch many files | Explicitly listed in chunk file lists with blast radius notes. Chunks 3 (9 files) and 8 (12 files) exceed the 5-file limit -- justified in each chunk's scope note. "If Blocked" sections provide fallback strategies. |
| Circular module dependency (ir <-> validator) | Richtext validation is a free function in `validator.rs`, not a method on `RichText`. IR types never import from validator. |

---

## Out of Scope (Explicit)

These are NOT excluded due to effort -- they're excluded because they require different work:

| Item | Why Out of Scope | Where It Belongs |
|------|-----------------|-----------------|
| Web/mobile UI components | This plan covers the backend IR/pipeline only | Frontend plan (future) |
| Punpun's hero creation guide integration | Separate resource, separate plan | `plans/HERO_GUIDE_INTEGRATION.md` |
| Community mode database scraping | External data source, not compiler concern | Tools/scripts |
| tann.fun image converter comparison | Sprite pipeline concern, not IR/parser | `tools/` comparison |
| Face ID semantic database | Maps face IDs to game mechanics -- useful but large | `plans/FACE_ID_DATABASE.md` |
| Full keyword catalog | Requires game data mining beyond Thunder's guide | Reference data task |

---

## Appendix: Phase Delimiter Quick Reference

| Delimiter | Used By | Purpose |
|-----------|---------|---------|
| `@1` | LinkedPhase, SeqPhase | Separate linked phases / separate options |
| `@2` | BooleanPhase, LevelEndPhase, SeqPhase | Separate true/false branches / phases / option sub-phases |
| `@3` | SimpleChoicePhase, ChoicePhase, TradePhase | Separate reward options |
| `@4` | Or tag | Separate Or alternatives |
| `@6` | BooleanPhase2 | Replaces `;` from BooleanPhase |
| `@7` | BooleanPhase2 | Replaces `@2` from BooleanPhase |
| `;` | BooleanPhase, ChoicePhase, MessagePhase, SCPhase | Separate value;threshold;phases / title;rewards / message;button |
| `~` | Random, RandomRange, Replace tags | Separate tier~amount~tag / modifier~reward |
| `#` | Modifier chains | Separate chain segments (within parenthesized blocks) |
| `+` | Hero tiers, fights, pools | Separate tier blocks / fight enemies / pool entries |

## Appendix: AbilityData Side Semantics

```
Side 1 (left):   Primary effect -- damage, shield, heal, etc.
Side 2 (mid):    Secondary untargeted effect -- damage all, mana, shield all
Side 3 (right):  Tactic cost 1 (if no Side 5)
Side 4:          Tactic cost 2
Side 5 (right):  Mana cost -> makes ability a Spell (pips = cost, 0-999)
Side 6:          Tactic cost 3

Rule: If Side 5 has data -> Spell. If Sides 3/4/6 have data but not 5 -> Tactic.
Max tactic costs: 3 (sides 3, 4, 6)
```

## Appendix: TriggerHPData Position Table

```
HP 1:  All HP pips
HP 2:  Every 2nd pip
HP 3:  Every 3rd pip
HP 4:  Every 4th pip
HP 5:  Every 5th pip
HP 6:  (not documented)
HP 7:  Every 10th, starting at 5
HP 8:  Every 2nd, starting at 1
HP 9:  Every 3rd, starting at 1
HP 10: Inner 1
HP 11: Inner 2
HP 12: (not documented)
HP 13: Inner 5
HP 14: Outer 1
HP 15: Outer 2
HP 16: Outer 3
HP 17: (not documented)
HP 18: (not documented)
HP 19: 2 evenly spaced
HP 20: 3 evenly spaced
HP 21: 4 evenly spaced
HP 22+: Affects (HP-20)th pip
```
