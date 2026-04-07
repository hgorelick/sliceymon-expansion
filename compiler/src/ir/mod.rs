pub mod merge;

use serde::{Deserialize, Serialize};

// -- Top-level IR --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModIR {
    pub heroes: Vec<Hero>,
    pub captures: Vec<Capture>,
    pub legendaries: Vec<Legendary>,
    pub monsters: Vec<Monster>,
    pub bosses: Vec<Boss>,
    pub structural: Vec<StructuralModifier>,
    /// Original modifier strings in extraction order. When present, the builder
    /// emits these directly instead of using type-based assembly. Cleared on merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_modifiers: Option<Vec<String>>,
}

impl ModIR {
    pub fn empty() -> Self {
        ModIR {
            heroes: Vec::new(),
            captures: Vec::new(),
            legendaries: Vec::new(),
            monsters: Vec::new(),
            bosses: Vec::new(),
            structural: Vec::new(),
            original_modifiers: None,
        }
    }
}

// -- Heroes --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum HeroFormat {
    Sliceymon,
    Grouped,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hero {
    pub internal_name: String,
    pub mn_name: String,
    pub color: char,
    #[serde(default)]
    pub format: HeroFormat,
    pub blocks: Vec<HeroBlock>,
    #[serde(default)]
    pub removed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroBlock {
    pub template: String,
    pub tier: Option<u8>,
    pub hp: u16,
    pub sd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    pub sprite_name: String,
    pub speech: String,
    pub name: String,
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

// -- Captures --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Capture {
    pub pokemon: String,
    pub ball_name: String,
    pub ball_tier: Option<u8>,
    pub template: String,
    pub hp: Option<u16>,
    pub sd: String,
    pub sprite_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_modifiers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggle_flags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}

// -- Legendaries --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legendary {
    pub pokemon: String,
    pub summoning_item: String,
    pub template: String,
    pub hp: Option<u16>,
    pub sd: String,
    pub sprite_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abilitydata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_modifiers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}

// -- Monsters --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Monster {
    pub name: String,
    pub base_template: String,
    pub floor_range: String,
    pub hp: Option<u16>,
    pub sd: Option<String>,
    pub sprite_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}

// -- Bosses --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Boss {
    pub name: String,
    pub level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<String>,
    pub fight_units: Vec<BossFightUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}

/// A unit within a boss fight (parsed from +separated blocks inside .fight.(...))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BossFightUnit {
    pub template: String,
    pub name: String,
    pub hp: Option<u16>,
    pub sd: Option<String>,
    pub sprite_data: Option<String>,
}

// -- Structural Modifiers --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuralModifier {
    pub modifier_type: StructuralType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub content: StructuralContent,
    pub raw: String,
}

impl StructuralModifier {
    /// Convenience constructor — fills name=None, content=Raw.
    pub fn new_raw(modifier_type: StructuralType, raw: String) -> Self {
        Self {
            modifier_type,
            name: None,
            content: StructuralContent::Raw,
            raw,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructuralType {
    PartyConfig,
    EventModifier,
    Dialog,
    HeroPoolBase,
    LevelUpAction,
    ItemPool,
    Selector,
    GenSelect,
    Difficulty,
    EndScreen,
    BossModifier,
    ArtCredits,
    PoolReplacement,
    Unknown,
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
    PoolReplacement { hero_names: Vec<String> },
    #[default]
    Raw,
}

/// An item entry within an ItemPool structural modifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPoolEntry {
    pub name: String,
    pub tier: Option<i8>,
    pub content: String,
}
