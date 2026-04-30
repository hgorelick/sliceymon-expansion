pub mod merge;
pub mod ops;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::authoring::{FaceIdValue, Pips, SpriteId};
use crate::finding::Finding;

// -- Provenance --

/// Provenance tracking — where an IR item originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default, JsonSchema)]
pub enum Source {
    /// Extracted from a base textmod (default).
    #[default]
    Base,
    /// Added programmatically via CRUD operations.
    Custom,
    /// Came from an overlay merge.
    Overlay,
}

impl Source {
    fn is_base(&self) -> bool { *self == Source::Base }
}

// -- Shared Types --

/// Dice faces — the .sd. field on heroes, replica items, monsters, spells, etc.
/// Format: colon-separated entries, each is "0" (blank) or "FaceID-Pips"
/// Example: "34-1:30-1:0:0:30-1:0" → [Active(34,1), Active(30,1), Blank, Blank, Active(30,1), Blank]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct DiceFaces {
    pub faces: Vec<DiceFace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum DiceFace {
    Blank,
    Active { face_id: FaceIdValue, pips: Pips },
}

impl DiceFaces {
    /// Parse from the colon-separated .sd. format: "34-1:30-1:0:0:30-1:0"
    pub fn parse(s: &str) -> Self {
        let faces = s.split(':').map(|entry| {
            let entry = entry.trim();
            if entry == "0" || entry == "0-0" || entry.is_empty() {
                DiceFace::Blank
            } else if let Some(dash_pos) = entry.find('-') {
                // `face_id-pips` where pips may be negative (e.g. `13--1`)
                // Split on FIRST dash — everything after is the signed pips value.
                let (id_str, pips_with_dash) = entry.split_at(dash_pos);
                let pips_str = &pips_with_dash[1..]; // skip the separator
                let raw_id = id_str.parse::<u16>().unwrap_or(0);
                let raw_pips = pips_str.parse::<i16>().unwrap_or(0);
                if raw_id == 0 && raw_pips == 0 {
                    DiceFace::Blank
                } else {
                    DiceFace::Active {
                        face_id: FaceIdValue::try_new(raw_id),
                        pips: Pips::new(raw_pips),
                    }
                }
            } else {
                // Bare number with no dash — treat as face_id with 0 pips
                let raw_id = entry.parse::<u16>().unwrap_or(0);
                if raw_id == 0 {
                    DiceFace::Blank
                } else {
                    DiceFace::Active {
                        face_id: FaceIdValue::try_new(raw_id),
                        pips: Pips::new(0),
                    }
                }
            }
        }).collect();
        DiceFaces { faces }
    }

    /// Emit back to colon-separated .sd. format.
    pub fn emit(&self) -> String {
        self.faces.iter().map(|f| match f {
            DiceFace::Blank => "0".to_string(),
            // Negative pips render as `ID--N` because the signed value prints its own `-`.
            DiceFace::Active { face_id, pips } => format!("{}-{}", face_id.raw(), pips.raw()),
        }).collect::<Vec<_>>().join(":")
    }
}

impl fmt::Display for DiceFaces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.emit())
    }
}

// -- Modifier Chains --

/// Modifier chain — equipment, facades, triggers, and properties applied to an entity.
/// Replaces raw `modifier_chain: Option<String>` and `item_modifiers: Option<String>` fields.
///
/// A chain is a sequence of segments, each introduced by `.i.` or `.sticker.` at paren depth 0.
/// Within a segment, `#` separates sub-entries (items, facades, keywords, toggles, etc.).
///
/// Examples:
/// - `.i.left.k.scared#facade.bas170:55` — one segment: slot "left", keyword + facade
/// - `.i.topbot.facade.eba3:0` — one segment: slot "topbot", facade
/// - `.i.(left.hat.(statue...))` — one segment: nested parenthesized content
/// - `.sticker.k.dejavu#k.exert#sidesc.TEXT` — sticker segment with sub-entries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ModifierChain {
    pub segments: Vec<ChainSegment>,
}

/// A single segment within a modifier chain.
/// Each segment corresponds to one `.i.` or `.sticker.` group, with typed sub-entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum ChainSegment {
    /// `.i.` — item/equipment segment
    Item {
        sub_entries: Vec<ChainEntry>,
    },
    /// `.sticker.` — sticker segment
    Sticker {
        sub_entries: Vec<ChainEntry>,
    },
}

impl ModifierChain {
    /// Parse a modifier chain string into segments with typed entries.
    /// The input is the output of `util::extract_modifier_chain()` — a concatenation of
    /// `.i.` and `.sticker.` groups with no intervening non-chain content.
    pub fn parse(s: &str) -> Self {
        let mut segments = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            // Find the next segment start
            let (is_item, prefix_len) = if i + 3 <= bytes.len() && &s[i..i + 3] == ".i." {
                (true, 3)
            } else if i + 9 <= bytes.len() && &s[i..i + 9] == ".sticker." {
                (false, 9)
            } else {
                i += 1;
                continue;
            };

            let content_start = i + prefix_len;
            // Scan forward to find the end of this segment:
            // next `.i.` or `.sticker.` at paren depth 0
            let mut j = content_start;
            let mut depth: i32 = 0;
            while j < bytes.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => depth -= 1,
                    b'.' if depth == 0 => {
                        if j + 3 <= bytes.len() && &s[j..j + 3] == ".i." {
                            break;
                        }
                        if j + 9 <= bytes.len() && &s[j..j + 9] == ".sticker." {
                            break;
                        }
                    }
                    _ => {}
                }
                j += 1;
            }

            let content = &s[content_start..j];
            if !content.is_empty() {
                let sub_entries = crate::extractor::chain_parser::parse_chain_entries(content);
                if is_item {
                    segments.push(ChainSegment::Item { sub_entries });
                } else {
                    segments.push(ChainSegment::Sticker { sub_entries });
                }
            }
            i = j;
        }

        ModifierChain { segments }
    }

    /// Emit the modifier chain back to its string representation.
    /// Guaranteed: `ModifierChain::parse(s).emit() == s` for any valid chain string.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        for seg in &self.segments {
            match seg {
                ChainSegment::Item { sub_entries } => {
                    out.push_str(".i.");
                    out.push_str(&crate::builder::chain_emitter::emit_chain_entries(sub_entries));
                }
                ChainSegment::Sticker { sub_entries } => {
                    out.push_str(".sticker.");
                    out.push_str(&crate::builder::chain_emitter::emit_chain_entries(sub_entries));
                }
            }
        }
        out
    }

    /// Extract facade values from this chain (replaces `util::extract_facades_from_chain`).
    pub fn facades(&self) -> Vec<String> {
        let chain_str = self.emit();
        crate::util::extract_facades_from_chain(&chain_str)
    }
}

impl fmt::Display for ModifierChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.emit())
    }
}

#[cfg(test)]
mod modifier_chain_tests {
    use super::*;

    #[test]
    fn roundtrip_simple_chain() {
        let s = ".i.left.k.scared#facade.bas170:55";
        assert_eq!(ModifierChain::parse(s).emit(), s);
    }

    #[test]
    fn roundtrip_multi_segment() {
        let s = ".i.left.k.scared#facade.bas170:55.i.topbot.facade.eba3:0";
        let chain = ModifierChain::parse(s);
        assert_eq!(chain.segments.len(), 2);
        assert!(matches!(&chain.segments[0], ChainSegment::Item { .. }));
        assert!(matches!(&chain.segments[1], ChainSegment::Item { .. }));
        assert_eq!(chain.emit(), s);
    }

    #[test]
    fn roundtrip_sticker_segment() {
        let s = ".sticker.k.dejavu#k.exert#sidesc.Add [pink]dejavu[cu] to target's sides this turn";
        let chain = ModifierChain::parse(s);
        assert_eq!(chain.segments.len(), 1);
        assert!(matches!(&chain.segments[0], ChainSegment::Sticker { .. }));
        assert_eq!(chain.emit(), s);
    }

    #[test]
    fn roundtrip_nested_parens() {
        let s = ".i.(left.hat.(statue.sd.15-2.i.left.k.damage)).i.topbot.facade.ite163:95:25:10";
        let chain = ModifierChain::parse(s);
        // The .i. inside parens should NOT be a split point
        assert_eq!(chain.segments.len(), 2);
        assert_eq!(chain.emit(), s);
    }

    #[test]
    fn roundtrip_mixed_item_sticker() {
        let s = ".i.left.k.pain.sticker.ritemx.dae9";
        let chain = ModifierChain::parse(s);
        assert_eq!(chain.segments.len(), 2);
        assert!(matches!(&chain.segments[0], ChainSegment::Item { .. }));
        assert!(matches!(&chain.segments[1], ChainSegment::Sticker { .. }));
        assert_eq!(chain.emit(), s);
    }

    #[test]
    fn empty_string_produces_empty_chain() {
        let chain = ModifierChain::parse("");
        assert!(chain.segments.is_empty());
        assert_eq!(chain.emit(), "");
    }

    #[test]
    fn facades_extraction() {
        let chain = ModifierChain::parse(".i.left.k.scared#facade.bas170:55.i.topbot.facade.eba3:0");
        let facades = chain.facades();
        assert!(facades.contains(&"bas170:55".to_string()));
        assert!(facades.contains(&"eba3:0".to_string()));
    }

    #[test]
    fn typed_entries_populated() {
        let chain = ModifierChain::parse(".i.left.k.scared#facade.bas170:55");
        if let ChainSegment::Item { sub_entries } = &chain.segments[0] {
            assert_eq!(sub_entries.len(), 2);
            assert!(matches!(&sub_entries[0], ChainEntry::Keyword { keyword, position }
                if keyword == "scared" && position.as_deref() == Some("left")));
            assert!(matches!(&sub_entries[1], ChainEntry::Facade { entity_code, parameter }
                if entity_code == "bas170" && parameter == "55"));
        } else {
            panic!("Expected Item segment");
        }
    }

    #[test]
    fn parenthesized_entry_roundtrip() {
        let s = ".i.(left.hat.(statue.sd.15-2))";
        let chain = ModifierChain::parse(s);
        assert_eq!(chain.emit(), s);
    }
}

/// Spell/ability definition inside .abilitydata.(...) blocks.
/// Format: (TEMPLATE.sd.FACES[.i.ITEMS][.img.ICON][.hsv.HUE].n.SPELL_NAME)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AbilityData {
    pub template: String,
    pub sd: DiceFaces,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<SpriteId>,
    pub name: String,
    /// Item/modifier chain inside the ability (e.g., .i.Ritemx.62e8.i.left.k.cleanse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<ModifierChain>,
    /// HSV color adjustment (e.g., "50:-20:0")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsv: Option<String>,
    /// Ability classification: spell (mana cost on side 5) or tactic (costs on sides 3/4/6).
    /// Derived from dice face data, not parsed from text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability_type: Option<AbilityType>,
}

impl AbilityData {
    /// Parse from the content inside .abilitydata.(...) — the string includes outer parens.
    pub fn parse(s: &str) -> Self {
        // Strip outer parens if present
        let inner = s.strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(s);

        // Template is the first segment before '.'
        let template = inner.find('.')
            .map(|end| inner[..end].to_string())
            .unwrap_or_else(|| inner.to_string());

        // Extract fields from the inner content using util functions
        let sd = crate::util::extract_sd(inner, false)
            .map(|s| DiceFaces::parse(&s))
            .unwrap_or_else(|| DiceFaces { faces: vec![] });

        let name = crate::util::extract_last_n_name(inner)
            .unwrap_or_default();

        let img_data = crate::util::extract_simple_prop(inner, ".img.");

        let modifier_chain = crate::util::extract_modifier_chain(inner)
            .map(|s| ModifierChain::parse(&s));

        let hsv = crate::util::extract_simple_prop(inner, ".hsv.");

        let sprite = img_data.map(|img| SpriteId::owned(name.clone(), img));

        AbilityData { template, sd, sprite, name, modifier_chain, hsv, ability_type: None }
    }

    /// Emit back to the .abilitydata.(...) format (including outer parens).
    pub fn emit(&self) -> String {
        let mut out = String::from("(");
        out.push_str(&self.template);
        out.push_str(".sd.");
        out.push_str(&self.sd.emit());
        if let Some(ref chain) = self.modifier_chain {
            out.push_str(&chain.emit());
        }
        if let Some(ref s) = self.sprite {
            if !s.img_data().is_empty() {
                out.push_str(".img.");
                out.push_str(s.img_data());
            }
        }
        if let Some(ref hsv) = self.hsv {
            out.push_str(".hsv.");
            out.push_str(hsv);
        }
        out.push_str(".n.");
        out.push_str(&self.name);
        out.push(')');
        out
    }
}

// -- Trigger HP Data --

/// Entity definition triggered at an HP threshold.
/// Format: `(TEMPLATE[.hp.N][.sd.FACES][.col.X][.i.CHAIN][.img.DATA][.n.NAME])`
/// Stored including outer parens. Uses the same property markers as other entity types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TriggerHpDef {
    pub template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sd: Option<DiceFaces>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<ModifierChain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<SpriteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<u8>,
}

impl TriggerHpDef {
    /// Parse from the full `.triggerhpdata.(...)` value (including outer parens).
    pub fn parse(s: &str) -> Self {
        let inner = s.strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(s);

        // Template is the first segment before '.'
        let template = inner.find('.')
            .map(|end| inner[..end].to_string())
            .unwrap_or_else(|| inner.to_string());

        let hp = crate::util::extract_hp(inner, false);
        let sd = crate::util::extract_sd(inner, false)
            .map(|s| DiceFaces::parse(&s));
        let color = crate::util::extract_color(inner, false);
        let modifier_chain = crate::util::extract_modifier_chain(inner)
            .map(|s| ModifierChain::parse(&s));
        let img_data = crate::util::extract_img_data(inner);
        let name = crate::util::extract_last_n_name(inner);
        let tier = crate::util::extract_simple_prop(inner, ".tier.")
            .and_then(|v| v.parse::<u8>().ok());
        let part = crate::util::extract_simple_prop(inner, ".part.")
            .and_then(|v| v.parse::<u8>().ok());

        // Anonymous triggerhps (no `.n.`) store an empty sprite name; emit only reads
        // `sprite.img_data()`, so the name is observably absent. Using the template
        // name here would be a semantic lie stored in the IR.
        let sprite = img_data.map(|img| SpriteId::owned(name.clone().unwrap_or_default(), img));

        TriggerHpDef { template, hp, sd, color, modifier_chain, sprite, name, tier, part }
    }

    /// Emit back to the `(TEMPLATE.props...)` format (including outer parens).
    pub fn emit(&self) -> String {
        let mut out = String::from("(");
        out.push_str(&self.template);

        if let Some(ref c) = self.color {
            out.push_str(".col.");
            out.push(*c);
        }

        if let Some(hp) = self.hp {
            out.push_str(".hp.");
            out.push_str(&hp.to_string());
        }

        if let Some(ref chain) = self.modifier_chain {
            out.push_str(&chain.emit());
        }

        if let Some(ref sd) = self.sd {
            out.push_str(".sd.");
            out.push_str(&sd.emit());
        }

        if let Some(tier) = self.tier {
            out.push_str(".tier.");
            out.push_str(&tier.to_string());
        }

        if let Some(part) = self.part {
            out.push_str(".part.");
            out.push_str(&part.to_string());
        }

        if let Some(ref s) = self.sprite {
            if !s.img_data().is_empty() {
                out.push_str(".img.");
                out.push_str(s.img_data());
            }
        }

        if let Some(ref name) = self.name {
            out.push_str(".n.");
            out.push_str(name);
        }

        out.push(')');
        out
    }
}

// -- Top-level IR --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ModIR {
    pub heroes: Vec<Hero>,
    pub replica_items: Vec<ReplicaItem>,
    pub monsters: Vec<Monster>,
    pub bosses: Vec<Boss>,
    pub structural: Vec<StructuralModifier>,
    /// Non-fatal findings accumulated during extract / merge / build.
    /// Empty-by-default so mods without findings serialize unchanged.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Finding>,
}

impl ModIR {
    pub fn empty() -> Self {
        ModIR {
            heroes: Vec::new(),
            replica_items: Vec::new(),
            monsters: Vec::new(),
            bosses: Vec::new(),
            structural: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

// -- Heroes --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub enum HeroFormat {
    Sliceymon,
    Grouped,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Hero {
    pub internal_name: String,
    pub mn_name: String,
    pub color: char,
    #[serde(default)]
    pub format: HeroFormat,
    pub blocks: Vec<HeroBlock>,
    #[serde(default)]
    pub removed: bool,
    #[serde(default, skip_serializing_if = "Source::is_base")]
    pub source: Source,
}

impl Hero {
    pub fn new(internal_name: impl Into<String>, mn_name: impl Into<String>, color: char) -> Self {
        Self {
            internal_name: internal_name.into(),
            mn_name: mn_name.into(),
            color,
            format: HeroFormat::default(),
            blocks: Vec::new(),
            removed: false,
            source: Source::Base,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct HeroBlock {
    pub template: String,
    pub tier: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<u16>,
    pub sd: DiceFaces,
    /// True if this block is a bare template (not wrapped in `(replica....)` parens).
    /// Bare blocks emit as `Template.props...` instead of `(replica.Template.props...)`.
    #[serde(default, skip_serializing_if = "is_false")]
    pub bare: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    pub sprite: SpriteId,
    pub speech: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abilitydata: Option<AbilityData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggerhpdata: Option<TriggerHpDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<ModifierChain>,
    #[serde(default)]
    pub facades: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_inside: Option<ModifierChain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_outside: Option<ModifierChain>,
}

// -- Replica Items --
//
// A summon item extracted from an entry inside `itempool.((…))`.
// An entry is a `ReplicaItem` iff its inner
// `hat.(replica.Thief.i.(all.(…)))` wrapper contains BOTH a
// `hat.egg.<enemy_template>.n.<target_name>…` egg and a matching
// `vase.(add.((replica.<team_template>.n.<target_name>…)))` team-join replica.

/// A summon item extracted from an entry inside `itempool.((…))`.
///
/// An entry is classified as a `ReplicaItem` iff its inner
/// `hat.(replica.Thief.i.(all.(…)))` wrapper contains BOTH:
///
///   1. a `hat.egg.<enemy_template>.n.<target_name>…` sub-block (the
///      summoned enemy that must be defeated), AND
///   2. a `vase.(add.((replica.<team_template>.n.<target_name>…)))`
///      sub-block (the team-join replica emitted on defeat) whose
///      `<target_name>` matches the egg's.
///
/// No raw-passthrough escape hatch — every field below must be derivable
/// from verified corpus bytes; if implementation finds a sub-block not
/// covered by the field list below, the struct must be widened in the same
/// commit (`extras: Vec<RawSubBlock>` is explicitly rejected).
///
/// 8b widening obligation: the Red Orb (Groudon) entry on sliceymon line
/// 117 contains a nested `hat.egg.(wolf.n.Geyser.sd.…)` inside the outer
/// `hat.egg.dragon.n.Groudon`. The single `enemy_template: String` below
/// captures only the outer template; 8b must widen the struct (e.g. an
/// `Option<NestedEgg>` field) before producing any `Summon(i)` entry whose
/// body contains a nested `hat.egg.`. 8a's stub never classifies this case.
///
/// 8a stub note: the stub `extract_from_itempool` never produces a
/// `ReplicaItem` — every entry is demoted to `ItempoolItem::NonSummon`.
/// Field population is exercised by authoring-builder tests + compile-guards.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ReplicaItem {
    pub container_name: String,
    pub target_name: String,
    pub trigger: SummonTrigger,
    pub enemy_template: String,
    pub team_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    pub sprite: SpriteId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_stack: Option<ModifierChain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggle_flags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_modifiers: Option<ModifierChain>,
    #[serde(default, skip_serializing_if = "Source::is_base")]
    pub source: Source,
}

/// The player action that triggers the summon. Two variants — `SideUse`
/// (use a thief side) and `Cast` (cast a thief spell) — capture the only
/// two distinct game mechanics observed in the corpus. Historical
/// "OnWrapped" (Master Ball?) is NOT a third variant: engine reads
/// `hat.Thief.sd.<faces>` identically whether dice live on the outer
/// preface or inside the wrapper — captured by `dice_location` on SideUse.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum SummonTrigger {
    /// Player uses a thief-side during a fight. Corpus:
    ///   `OuterPreface` = 18 (every Ball entry except `Master Ball?`),
    ///   `InnerWrapper` = 1 (`Master Ball?` only).
    SideUse {
        dice: DiceFaces,
        dice_location: DiceLocation,
    },

    /// Player casts a thief-spell. The summon fires on cast. Corpus count:
    /// 4 (Rainbow Wing, Silver Wing, Blue Orb, Red Orb).
    ///
    /// Payload: `dice: DiceFaces` — the PER-ITEM inner
    /// `.i.hat.(replica.thief.sd.<faces>)` chain segment, NOT the outer
    /// `thief.sd.<UNIVERSAL>`. Outer cast-template (`"thief"`) and outer
    /// cast-dice are emitter literals in `builder/replica_item_emitter.rs`.
    /// Widening contract: if a future corpus entry has a different outer
    /// template or depth-0 `.n.`, lift the constants into variant fields
    /// in the same PR.
    Cast { dice: DiceFaces },
}

/// Where the dice live in the source bytes for a `SideUse` summon.
/// Source-shape sub-axis — both locations produce identical engine
/// behavior; the discriminator exists only to make extract → build
/// round-trip byte-equal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum DiceLocation {
    /// Outer flat preface: `hat.replica.Thief.n.<target_name>.sd.<faces>` sits
    /// BEFORE the wrapper's opening. 18 corpus entries.
    OuterPreface,
    /// Inner wrapper: no outer preface; dice live inside the wrapper's egg
    /// body as `.i.(hat.Thief.sd.<faces>)`. 1 corpus entry (Master Ball?).
    InnerWrapper,
}

impl SummonTrigger {
    /// Shared accessor — every consumer (emitter, xref, authoring) routes
    /// dice through this method. Variant-branching for dice access is
    /// forbidden; this is the hook rule against duplicated incantations.
    pub fn dice_faces(&self) -> &DiceFaces {
        match self {
            SummonTrigger::SideUse { dice, .. } => dice,
            SummonTrigger::Cast { dice } => dice,
        }
    }

    /// Lossy projection to `ReplicaTriggerKey` — the same discriminant the
    /// round-9 merge predicate (`merge.rs:151-171`) and the round-12 add
    /// predicate (`ops.rs:104-152`) key on, with no payload. Use this when
    /// a CRUD caller needs to identify *which* trigger variant on a given
    /// `target_name` without supplying its dice/`dice_location` payload —
    /// e.g. `remove_replica_item(name, key)`.
    pub fn key(&self) -> ReplicaTriggerKey {
        match self {
            SummonTrigger::SideUse { .. } => ReplicaTriggerKey::SideUse,
            SummonTrigger::Cast { .. } => ReplicaTriggerKey::Cast,
        }
    }
}

/// Discriminant-only projection of `SummonTrigger`. CRUD operations that
/// need to address a specific trigger variant on a `target_name` (e.g.
/// `remove_replica_item`) take this enum as a parameter so callers do not
/// have to fabricate dice/`dice_location` payload they do not care about.
/// Pairs with `target_name` to form the round-9/12 uniqueness key
/// `(target_name, trigger discriminant)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum ReplicaTriggerKey {
    SideUse,
    Cast,
}

/// Typed sum for `StructuralContent::ItemPool.items`. Every itempool entry
/// is one of:
///   - a summon (index into `ModIR.replica_items`), or
///   - a NON-summon entry (everything else in an itempool — base-game refs,
///     multipliers, ritemx refs, splices, inline definitions).
///
/// TRANSITIONAL raw-passthrough form (8a only). The `NonSummon` variant
/// carries a raw `content: String` — a known, tracked SPEC §3.2 violation
/// that 8A.5 closes by replacing the variant with a typed `NonSummonEntry`
/// sum before 8b starts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum ItempoolItem {
    /// Index into `ModIR.replica_items`. Index stability is enforced by
    /// `ir::ops::remove_replica_item`, which removes the entry matching
    /// `(target_name, ReplicaTriggerKey)` — the round-9/12 multi-trigger
    /// uniqueness key — and re-indexes every `Summon(i)` accordingly.
    Summon(usize),
    /// Non-summon itempool entry — transitional raw-passthrough. `name` is
    /// the entry's inline `.n.<name>` where one exists (empty for the 8a
    /// stub's whole-pool passthrough); `tier` is the entry's `.tier.<n>`
    /// where one exists; `content` is the verbatim entry body bytes. 8A.5
    /// replaces this variant with the typed `NonSummonEntry` sum.
    NonSummon {
        name: String,
        tier: Option<i8>,
        content: String,
    },
}

// -- Monsters --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct Monster {
    pub name: String,
    pub base_template: String,
    pub floor_range: String,
    pub hp: Option<u16>,
    pub sd: Option<DiceFaces>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sprite: Option<SpriteId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<ModifierChain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(default, skip_serializing_if = "Source::is_base")]
    pub source: Source,
}

impl Monster {
    pub fn new(
        name: impl Into<String>,
        base_template: impl Into<String>,
        floor_range: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            base_template: base_template.into(),
            floor_range: floor_range.into(),
            ..Default::default()
        }
    }
}

// -- Bosses --

/// Boss format — how the encounter is encoded in the textmod.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub enum BossFormat {
    /// Standard: `ch.omN.fight.(units+...)` or `ch.omN.fight.units+...`
    #[default]
    Standard,
    /// Event: `ch.om(N.ph.s...narrative...fight content...)` — interactive boss event
    /// with authored narrative text, selector options, and embedded fight data.
    Event,
    /// Encounter: `1.ph.bX;1;!m(N.fight....)`
    Encounter,
}

/// A boss encounter definition. Can contain multiple fight variants
/// (alternative boss fights for a floor — the game randomly selects one).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct Boss {
    /// Overall encounter name (e.g., "Floor8", "ZangooseQuagsireAriados")
    pub name: String,
    /// Floor level
    pub level: Option<u8>,
    #[serde(default)]
    pub format: BossFormat,
    /// Single letter after ph.b (X, Y, B, L) — Encounter format only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter_id: Option<char>,
    /// Fight definitions — one or more alternative fights for this encounter
    pub fights: Vec<FightDefinition>,
    /// Event phases — structured representation of event boss narrative content.
    /// For round-trip safety, event body text is stored as a single Message phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_phases: Option<Vec<Phase>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<ModifierChain>,
    #[serde(default, skip_serializing_if = "Source::is_base")]
    pub source: Source,
}

impl Boss {
    pub fn new(name: impl Into<String>, level: Option<u8>) -> Self {
        Self {
            name: name.into(),
            level,
            ..Default::default()
        }
    }
}

// BossFightVariant and BossFightUnit have been replaced by FightDefinition and FightUnit
// (defined below in the Fight Definitions section).

// -- Structural Modifiers --

fn is_false(b: &bool) -> bool { !*b }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct StructuralModifier {
    pub modifier_type: StructuralType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub content: StructuralContent,
    #[serde(default, skip_serializing_if = "is_false")]
    pub derived: bool,
    #[serde(default, skip_serializing_if = "Source::is_base")]
    pub source: Source,
}

impl StructuralModifier {
    /// Convenience constructor — creates a structural modifier with the given body
    /// stored in the appropriate content variant with empty summary fields.
    pub fn new(modifier_type: StructuralType, body: String) -> Self {
        let name = crate::util::extract_mn_name(&body);
        let content = StructuralContent::from_body(&modifier_type, body);
        Self { modifier_type, name, content, derived: false, source: Source::Base }
    }

    /// Get the body text of this structural modifier.
    pub fn body(&self) -> &str {
        self.content.body()
    }

    /// Whether this modifier is a derived structural — one that `build` and
    /// `merge` regenerate from `ir` content rather than carrying through from
    /// input.
    ///
    /// Per SPEC §4, the four derived kinds are Character Selection,
    /// HeroPoolBase, PoolReplacement, and Hero-bound ItemPool. Whether a
    /// particular IR `StructuralModifier` of one of those types is *actually*
    /// the derived flavor (vs. an authored same-typed modifier with
    /// coincidentally-overlapping shape) cannot be decided by type alone —
    /// e.g. sliceymon's 5 `Selector` modifiers named "" are boss-fight
    /// warning selectors, not the top-level character picker. We therefore
    /// use the explicit `derived: bool` flag as the authoritative signal:
    /// generators set it on output, and the flag round-trips through serde
    /// so downstream consumers (merge, build) can strip safely.
    ///
    /// This is narrower than a type-only heuristic would be; the tradeoff is
    /// that a user who hand-authors a `StructuralModifier` of a derived type
    /// without setting `derived: true` will have it carried through rather
    /// than stripped. That is intentional — if the user didn't declare it as
    /// derived, we have no way to distinguish it from an authored shape that
    /// happens to coincide, and silently stripping would destroy content.
    pub fn is_derived(&self) -> bool {
        self.derived
            && matches!(
                self.modifier_type,
                StructuralType::Selector
                    | StructuralType::HeroPoolBase
                    | StructuralType::PoolReplacement
                    | StructuralType::ItemPool
            )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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
    PhaseModifier,
    Choosable,
    ValueModifier,
    HiddenModifier,
    FightModifier,
}

/// Structural modifier content. Each variant stores a `body` field containing the full
/// modifier text for emission, plus typed summary fields for introspection/querying.
///
/// The `body` fields contain game modifier text — rich display text with `[tag]` formatting,
/// game engine instructions, or structured content. These are NOT multi-property blobs:
/// they are the authoritative content for each typed modifier category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum StructuralContent {
    HeroPoolBase {
        body: String,
        hero_refs: Vec<String>,
    },
    ItemPool {
        items: Vec<ItempoolItem>,
    },
    BossModifier {
        body: String,
        flags: Vec<String>,
    },
    PartyConfig {
        body: String,
        party_name: String,
        members: Vec<String>,
    },
    EventModifier {
        body: String,
        event_name: String,
    },
    Dialog {
        body: String,
        phase: String,
    },
    ArtCredits {
        body: String,
    },
    Selector {
        body: String,
        options: Vec<String>,
    },
    GenSelect {
        body: String,
        options: Vec<String>,
    },
    Difficulty {
        body: String,
        options: Vec<String>,
    },
    LevelUpAction {
        body: String,
    },
    PoolReplacement {
        body: String,
        hero_names: Vec<String>,
    },
    EndScreen {
        body: String,
    },
    PhaseModifier {
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        level_scope: Option<LevelScope>,
        #[serde(skip_serializing_if = "Option::is_none")]
        phase: Option<Phase>,
    },
    Choosable {
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        level_scope: Option<LevelScope>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<RewardTag>,
    },
    ValueModifier {
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        level_scope: Option<LevelScope>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value_ref: Option<ValueRef>,
    },
    HiddenModifier {
        body: String,
        modifier_type: HiddenModifierType,
    },
    FightModifier {
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        level_scope: Option<LevelScope>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fight: Option<FightDefinition>,
    },
}

impl StructuralContent {
    /// Create a content variant for the given type with just a body string
    /// and empty summary fields.
    pub fn from_body(stype: &StructuralType, body: String) -> Self {
        match stype {
            StructuralType::HeroPoolBase => Self::HeroPoolBase { body, hero_refs: vec![] },
            StructuralType::ItemPool => Self::ItemPool {
                // Transitional raw-passthrough per §3.2: `from_body` is called
                // by `StructuralModifier::new` from callers that still supply a
                // raw body string. Wrap the whole body in a single sentinel
                // NonSummon (empty name, None tier) — the emitter's
                // `emit_itempool` treats this as the stub-passthrough path and
                // re-emits `content` verbatim. 8a's extractor bypasses this
                // and calls `extract_from_itempool` directly; `from_body` stays
                // consistent for in-crate callers that still route through it.
                items: if body.is_empty() {
                    Vec::new()
                } else {
                    vec![ItempoolItem::NonSummon {
                        name: String::new(),
                        tier: None,
                        content: body,
                    }]
                },
            },
            StructuralType::BossModifier => Self::BossModifier { body, flags: vec![] },
            StructuralType::PartyConfig => Self::PartyConfig { body, party_name: String::new(), members: vec![] },
            StructuralType::EventModifier => Self::EventModifier { body, event_name: String::new() },
            StructuralType::Dialog => Self::Dialog { body, phase: String::new() },
            StructuralType::ArtCredits => Self::ArtCredits { body },
            StructuralType::Selector => Self::Selector { body, options: vec![] },
            StructuralType::GenSelect => Self::GenSelect { body, options: vec![] },
            StructuralType::Difficulty => Self::Difficulty { body, options: vec![] },
            StructuralType::LevelUpAction => Self::LevelUpAction { body },
            StructuralType::PoolReplacement => Self::PoolReplacement { body, hero_names: vec![] },
            StructuralType::EndScreen => Self::EndScreen { body },
            StructuralType::PhaseModifier => Self::PhaseModifier { body, level_scope: None, phase: None },
            StructuralType::Choosable => Self::Choosable { body, level_scope: None, tag: None },
            StructuralType::ValueModifier => Self::ValueModifier { body, level_scope: None, value_ref: None },
            StructuralType::HiddenModifier => Self::HiddenModifier { body, modifier_type: HiddenModifierType::Skip },
            StructuralType::FightModifier => Self::FightModifier { body, level_scope: None, fight: None },
        }
    }

    /// Get the body text of this structural content.
    pub fn body(&self) -> &str {
        match self {
            Self::HeroPoolBase { body, .. } => body,
            Self::ItemPool { items } => {
                // The ItemPool variant no longer stores a raw body. For the
                // stub-sentinel shape (single NonSummon with empty name + None
                // tier), return the raw `content` bytes so callers that still
                // read `body()` on an ItemPool see the source bytes back. The
                // authoritative emitter path is `replica_item_emitter::emit_itempool`
                // via the structural emitter dispatch; this accessor is a
                // compatibility shim for call sites that pre-date the 8a
                // trigger-IR rewrite. Non-sentinel shapes fall through to "".
                match items.as_slice() {
                    [ItempoolItem::NonSummon { content, name, tier }]
                        if name.is_empty() && tier.is_none() =>
                    {
                        content.as_str()
                    }
                    _ => "",
                }
            }
            Self::BossModifier { body, .. } => body,
            Self::PartyConfig { body, .. } => body,
            Self::EventModifier { body, .. } => body,
            Self::Dialog { body, .. } => body,
            Self::ArtCredits { body } => body,
            Self::Selector { body, .. } => body,
            Self::GenSelect { body, .. } => body,
            Self::Difficulty { body, .. } => body,
            Self::LevelUpAction { body } => body,
            Self::PoolReplacement { body, .. } => body,
            Self::EndScreen { body } => body,
            Self::PhaseModifier { body, .. } => body,
            Self::Choosable { body, .. } => body,
            Self::ValueModifier { body, .. } => body,
            Self::HiddenModifier { body, .. } => body,
            Self::FightModifier { body, .. } => body,
        }
    }
}

// =========================================================================
// Types for Textmod API integration
// =========================================================================

// -- Phase System --

/// Phase — control flow unit for custom modes and events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Phase {
    pub phase_type: PhaseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_scope: Option<LevelScope>,
    pub content: PhaseContent,
}

/// Phase type codes — the character after "ph." in the textmod.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum PhaseType {
    SimpleChoice,   // ph.!
    PlayerRolling,  // ph.0
    Targeting,      // ph.1
    LevelEnd,       // ph.2
    EnemyRolling,   // ph.3
    Message,        // ph.4
    HeroChange,     // ph.5
    Reset,          // ph.6
    ItemCombine,    // ph.7
    PositionSwap,   // ph.8
    Challenge,      // ph.9
    Boolean,        // ph.b
    Choice,         // ph.c
    Damage,         // ph.d
    RunEnd,         // ph.e
    Linked,         // ph.l
    RandomReveal,   // ph.r
    Seq,            // ph.s
    Trade,          // ph.t
    PhaseGenerator, // ph.g
    Boolean2,       // ph.z
}

/// Phase content — variant-specific data for each phase type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum PhaseContent {
    SimpleChoice {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        rewards: Vec<RewardTag>,
    },
    Message {
        text: RichText,
        #[serde(skip_serializing_if = "Option::is_none")]
        button_text: Option<String>,
    },
    HeroChange {
        hero_position: u8,
        change_type: HeroChangeType,
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
        combine_type: String,
    },
    RandomReveal {
        reward: RewardTag,
    },
    PhaseGenerator {
        gen_type: PhaseGenType,
    },
    RunEnd,
    Reset,
    PlayerRolling,
    Targeting,
    EnemyRolling,
    Damage,
}

/// An option in a SeqPhase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SeqOption {
    pub button_text: String,
    pub phases: Vec<Phase>,
}

// -- Reward Tags --

/// Choosable/SCPhase reward tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RewardTag {
    pub tag_type: RewardTagType,
    pub content: String,
}

/// Reward tag type — the single-letter prefix in reward strings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RewardTagType {
    Modifier,     // m
    Item,         // i
    Levelup,      // l
    Hero,         // g
    Random,       // r — tier~amount~tag
    RandomRange,  // q — tier1~tier2~amount~tag
    Or,           // o — @4 delimiter
    Enu,          // e
    Value,        // v — variable system
    Replace,      // p — pm(old)~(new)
    Skip,         // s
}

// -- Level Scoping --

/// Level/floor scoping prefix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LevelScope {
    pub start: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u8>,
}

// -- Chain Entry Types --

/// A single #-separated entry within an .i. or .sticker. segment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum ChainEntry {
    Hat {
        entity: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        position: Option<String>,
    },
    Splice {
        item: String,
    },
    Cast {
        effect: String,
    },
    Enchant {
        modifier: String,
    },
    Learn {
        ability: String,
    },
    TogItem {
        tog_type: TogType,
        #[serde(skip_serializing_if = "Option::is_none")]
        position: Option<String>,
    },
    Keyword {
        keyword: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        position: Option<String>,
    },
    Facade {
        entity_code: String,
        parameter: String,
    },
    Sidesc {
        text: String,
    },
    /// Entity reference: `r<type>.<hex_hash>[.part.<n>][.m.<n>]`
    EntityRef {
        kind: RefKind,
        hash: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        part: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        multiplier: Option<u8>,
    },
    Memory,
    /// Bare game item reference (e.g., "Blindfold", "Eye of Horus", "Chainmail").
    ItemRef {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        position: Option<String>,
    },
    /// Parenthesized group of entries — preserves (...) wrapping for round-trip fidelity.
    Parenthesized {
        entries: Vec<ChainEntry>,
    },
}

/// Entity reference type prefix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RefKind {
    Item,     // ritemx
    Modifier, // rmod
    Monster,  // rmon
}

// -- Side Positions & Tog Types --

/// Side position selector for dice faces.
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

/// Tog item type — the specific toggle behavior.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum TogType {
    Time,  // togtime
    Targ,  // togtarg
    Fri,   // togfri
    Vis,   // togvis
    Eft,   // togeft
    Pip,   // togpip
    Key,   // togkey
    Unt,   // togunt
    Res,   // togres
    ResM,  // togresm
    ResA,  // togresa
    ResO,  // togreso
    ResX,  // togresx
    ResN,  // togresn
    ResS,  // togress
}

// -- Value System --

/// Value system variable reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ValueRef {
    pub name: String,
    pub amount: i32,
}

// -- Rich Text --

/// Rich text — a newtype preserving exact content with validation behavior.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RichText(pub String);

impl RichText {
    pub fn new(s: impl Into<String>) -> Self {
        RichText(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this richtext contains any formatting tags.
    pub fn has_tags(&self) -> bool {
        self.0.contains('[')
    }
}

impl fmt::Display for RichText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// -- Fight Definitions (generalized beyond Boss context) --

/// Fight definition — generalizable fight encounter (used by bosses, phases, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct FightDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    pub enemies: Vec<FightUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}

impl FightDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Unified fight unit — shared between bosses and general fights.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct FightUnit {
    pub template: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sd: Option<DiceFaces>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<SpriteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_override: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_chain: Option<ModifierChain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsv: Option<String>,
    /// Nested fight units spawned inside this unit.
    /// In source: `template.((child1+child2))` — double parens wrap a group.
    ///            `template.(child)` — single paren wraps a transform.
    /// Used by challenge-mode bosses for egg/spawn groups and egg transforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_units: Option<Vec<FightUnit>>,
    /// When true, nested was encoded with a single `.(child)` paren in source;
    /// emit uses single-paren too. Default (false) emits `.((child+child))`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub nested_single_paren: bool,
    /// True when source encoded template+name as `(Template.n.Name).rest`.
    /// Emit preserves this wrapping so re-paste matches source structure.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub head_paren: bool,
    /// True when the whole unit is wrapped in outer parens: `(Template.props)`.
    /// Mutually exclusive with `head_paren`. Flat units have neither set.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub outer_paren: bool,
    /// `.part.N` — part index for multi-part units. Appears on some fight units
    /// (e.g. Troll.`.i.t.Sarcophagus.part.0`). Distinct from the `.part.1`
    /// appending convention used on modifier lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<u16>,
    /// Unit-level `.k.KEYWORD` segments that appear AFTER a `.t.X` template
    /// override. Each entry is one bare keyword word (fully structured — not
    /// raw syntax). Emit places them right after `.t.X`, matching source.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub post_override_keywords: Vec<String>,
    /// Source-order list of property markers for this unit's body. Emit follows
    /// this order so re-paste matches source structure. Empty → canonical order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_order: Vec<FightUnitMarker>,
}

impl FightUnit {
    pub fn new(template: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            template: template.into(),
            name: name.into(),
            ..Default::default()
        }
    }
}

/// Body property markers in source order for FightUnit re-emission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum FightUnitMarker {
    Nested,
    Col,
    Hsv,
    /// One modifier-chain segment at source position. The `usize` indexes into
    /// `FightUnit.modifier_chain.segments`. Each chain segment gets its own
    /// entry so non-chain props (notably `.doc.`) can interleave in source
    /// order — e.g. source `.i.x3.X.doc.Y.i.left2.Z` emits as
    /// `[Chain(0), Doc, Chain(1)]`.
    Chain(usize),
    Hp,
    Sd,
    Name,
    Doc,
    Img,
    TemplateOverride,
    Part,
}

// -- Choice / Hero Change / Phase Generator Types --

/// Choice phase type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum ChoiceType {
    PointBuy { budget: i32 },
    Number { count: u8 },
    UpToNumber { max: u8 },
    Optional,
}

/// Hero change type in HeroChangePhase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum HeroChangeType {
    RandomClass,    // 0
    GeneratedHero,  // 1
}

/// Phase generator type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum PhaseGenType {
    Hero, // ph.gh
    Item, // ph.gi
}

// -- Ability Type --

/// Ability classification derived from which dice sides have data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum AbilityType {
    Spell { mana_cost: u16 },
    Tactic { cost_count: u8 },
}

// -- Entity Wrappers --

/// Entity wrapper types (orb, vase, jinx, egg).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum EntityWrapper {
    Orb { entity: String },
    Vase { entity: String },
    Jinx { modifier: String },
    Egg { entity: String },
}

// -- Hidden Modifier Type --

/// Known hidden modifier type.
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

#[cfg(test)]
mod new_type_tests {
    use super::*;

    #[test]
    fn test_phase_serialization_roundtrip() {
        let phase = Phase {
            phase_type: PhaseType::Message,
            level_scope: None,
            content: PhaseContent::Message {
                text: RichText::new("Hello World"),
                button_text: Some("OK".to_string()),
            },
        };
        let json = serde_json::to_string(&phase).unwrap();
        let back: Phase = serde_json::from_str(&json).unwrap();
        assert_eq!(phase, back);
    }

    #[test]
    fn test_phase_boolean_boxed_recursion() {
        let inner1 = Phase {
            phase_type: PhaseType::Message,
            level_scope: None,
            content: PhaseContent::Message {
                text: RichText::new("win"),
                button_text: None,
            },
        };
        let inner2 = Phase {
            phase_type: PhaseType::Linked,
            level_scope: None,
            content: PhaseContent::Linked {
                phases: vec![inner1.clone()],
            },
        };
        let boolean = Phase {
            phase_type: PhaseType::Boolean,
            level_scope: None,
            content: PhaseContent::Boolean {
                value_name: "score".to_string(),
                threshold: 5,
                if_true: Box::new(inner1),
                if_false: Box::new(inner2),
            },
        };
        let json = serde_json::to_string(&boolean).unwrap();
        let back: Phase = serde_json::from_str(&json).unwrap();
        assert_eq!(boolean, back);
    }

    #[test]
    fn test_richtext_newtype() {
        let rt = RichText::new("hello");
        assert_eq!(rt.as_str(), "hello");
        assert!(!rt.has_tags());

        let tagged = RichText::new("[orange]hello[cu]");
        assert!(tagged.has_tags());
    }

    #[test]
    fn test_fight_unit_default_fields() {
        let unit = FightUnit {
            template: "Sniper".to_string(),
            name: "Wooper".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&unit).unwrap();
        let back: FightUnit = serde_json::from_str(&json).unwrap();
        assert_eq!(unit, back);
        // Optional fields should be absent in JSON
        assert!(!json.contains("hp"));
        assert!(!json.contains("color"));
    }

    #[test]
    fn test_fight_definition_with_trigger() {
        let fight = FightDefinition {
            level: Some(8),
            enemies: vec![FightUnit {
                template: "Basalt".to_string(),
                name: "Boss".to_string(),
                hp: Some(10),
                ..Default::default()
            }],
            name: Some("Floor8".to_string()),
            trigger: Some("@4m4".to_string()),
        };
        let json = serde_json::to_string(&fight).unwrap();
        let back: FightDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(fight, back);
    }

    #[test]
    fn test_chain_entry_variants_serialize() {
        let entries: Vec<ChainEntry> = vec![
            ChainEntry::Hat { entity: "Ace".to_string(), position: None },
            ChainEntry::Splice { item: "Sword".to_string() },
            ChainEntry::Cast { effect: "crush".to_string() },
            ChainEntry::Enchant { modifier: "Power".to_string() },
            ChainEntry::Learn { ability: "Fireball".to_string() },
            ChainEntry::TogItem { tog_type: TogType::Res, position: None },
            ChainEntry::Keyword { keyword: "scared".to_string(), position: Some("left".to_string()) },
            ChainEntry::Facade { entity_code: "bas170".to_string(), parameter: "55".to_string() },
            ChainEntry::Sidesc { text: "hello".to_string() },
            ChainEntry::EntityRef { kind: RefKind::Item, hash: "dae9".to_string(), part: None, multiplier: None },
            ChainEntry::Memory,
            ChainEntry::ItemRef { name: "Blindfold".to_string(), position: None },
            ChainEntry::Parenthesized { entries: vec![ChainEntry::Memory] },
        ];
        // Each variant should produce unique JSON
        let jsons: Vec<String> = entries.iter()
            .map(|e| serde_json::to_string(e).unwrap())
            .collect();
        for (i, a) in jsons.iter().enumerate() {
            for (j, b) in jsons.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b, "variants {} and {} produced identical JSON", i, j);
                }
            }
        }
    }

    #[test]
    fn test_level_scope_serialization() {
        let scopes = vec![
            LevelScope { start: 5, end: None, interval: None, offset: None },
            LevelScope { start: 3, end: Some(7), interval: None, offset: None },
            LevelScope { start: 0, end: None, interval: Some(2), offset: None },
            LevelScope { start: 0, end: None, interval: Some(3), offset: Some(1) },
        ];
        for scope in &scopes {
            let json = serde_json::to_string(scope).unwrap();
            let back: LevelScope = serde_json::from_str(&json).unwrap();
            assert_eq!(*scope, back);
        }
    }

    #[test]
    fn test_phase_type_all_variants_serialize() {
        let variants = vec![
            PhaseType::SimpleChoice, PhaseType::PlayerRolling, PhaseType::Targeting,
            PhaseType::LevelEnd, PhaseType::EnemyRolling, PhaseType::Message,
            PhaseType::HeroChange, PhaseType::Reset, PhaseType::ItemCombine,
            PhaseType::PositionSwap, PhaseType::Challenge, PhaseType::Boolean,
            PhaseType::Choice, PhaseType::Damage, PhaseType::RunEnd,
            PhaseType::Linked, PhaseType::RandomReveal, PhaseType::Seq,
            PhaseType::Trade, PhaseType::PhaseGenerator, PhaseType::Boolean2,
        ];
        assert_eq!(variants.len(), 21);
        for v in &variants {
            let json = serde_json::to_string(v).unwrap();
            let back: PhaseType = serde_json::from_str(&json).unwrap();
            assert_eq!(*v, back);
        }
    }
}

// =========================================================================
// Default + ::new(identity) constructor tests
// =========================================================================

#[cfg(test)]
mod chunk_1_tests {
    use super::*;

    #[test]
    fn hero_new_defaults_empty_blocks() {
        let h = Hero::new("x", "X", 'r');
        assert_eq!(h.internal_name, "x");
        assert_eq!(h.mn_name, "X");
        assert_eq!(h.color, 'r');
        assert!(h.blocks.is_empty());
        assert!(!h.removed);
        assert_eq!(h.source, Source::Base);
    }

    #[test]
    fn monster_new_defaults_optional_fields_none() {
        let m = Monster::new("Foo", "bas", "1-5");
        assert_eq!(m.name, "Foo");
        assert_eq!(m.base_template, "bas");
        assert_eq!(m.floor_range, "1-5");
        assert!(m.sd.is_none());
        assert!(m.sprite.is_none());
        assert!(m.hp.is_none());
        assert!(m.color.is_none());
        assert_eq!(m.source, Source::Base);
    }

    #[test]
    fn boss_default_roundtrip() {
        let b = Boss::default();
        let json = serde_json::to_string(&b).unwrap();
        let back: Boss = serde_json::from_str(&json).unwrap();
        assert_eq!(b, back);
    }

    #[test]
    fn boss_new_sets_identity() {
        let b = Boss::new("Floor8", Some(8));
        assert_eq!(b.name, "Floor8");
        assert_eq!(b.level, Some(8));
        assert!(b.fights.is_empty());
    }

    #[test]
    fn fight_definition_new_equals_default() {
        assert_eq!(FightDefinition::new(), FightDefinition::default());
    }

    #[test]
    fn fight_unit_new_sets_identity() {
        let u = FightUnit::new("Sniper", "Wooper");
        assert_eq!(u.template, "Sniper");
        assert_eq!(u.name, "Wooper");
        assert!(u.hp.is_none());
    }

    #[test]
    fn dice_faces_default_is_empty() {
        assert!(DiceFaces::default().faces.is_empty());
    }

    /// `FaceIdValue` + `Pips` replaced `u16` + `i16` on `DiceFace::Active`.
    /// This test proves the newtype flip preserves byte-for-byte roundtrip
    /// across the full shape space: corpus-known FaceID, blank, corpus-unknown
    /// FaceID, and negative pips.
    #[test]
    fn diceface_roundtrip_through_newtypes() {
        let input = "34-1:0:170-5:0:0:13--1";
        let parsed = DiceFaces::parse(input);
        assert_eq!(parsed.emit(), input);
        // Shape check — Active/Blank alternation + Unknown preservation if any.
        assert_eq!(parsed.faces.len(), 6);
        match &parsed.faces[0] {
            DiceFace::Active { face_id, pips } => {
                assert_eq!(face_id.raw(), 34);
                assert_eq!(pips.raw(), 1);
                assert!(face_id.is_known(), "34 is curated");
            }
            _ => panic!("faces[0] must be Active"),
        }
        assert!(matches!(parsed.faces[1], DiceFace::Blank));
        match &parsed.faces[5] {
            DiceFace::Active { face_id, pips } => {
                assert_eq!(face_id.raw(), 13);
                assert_eq!(pips.raw(), -1);
            }
            _ => panic!("faces[5] must be Active with negative pips"),
        }
    }

    /// Regression: a FaceID outside the whitelist must still round-trip.
    #[test]
    fn diceface_unknown_face_id_roundtrips() {
        let input = "9999-2:0";
        let parsed = DiceFaces::parse(input);
        assert_eq!(parsed.emit(), input);
        match &parsed.faces[0] {
            DiceFace::Active { face_id, .. } => {
                assert!(!face_id.is_known(), "9999 is outside the corpus whitelist");
                assert_eq!(face_id.raw(), 9999);
            }
            _ => panic!("faces[0] must be Active"),
        }
    }

    /// The 2026-04-20 "no legacy back-compat" ruling on sprite shape:
    /// `HeroBlock.sprite` is a required `SpriteId`. A struct literal
    /// omitting `sprite` must fail to compile — this test is the positive
    /// side of that proof (construction with `SpriteId::owned` succeeds);
    /// the negative side is exercised by every callsite that still spells
    /// the old `sprite_name` / `img_data` fields failing `cargo build`.
    #[test]
    fn heroblock_sprite_required() {
        let block = HeroBlock {
            template: "Lost".into(),
            tier: Some(1),
            hp: Some(5),
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            bare: false,
            color: None,
            sprite: SpriteId::owned("Pikachu", "ABC123"),
            speech: "!".into(),
            name: "Pikachu".into(),
            doc: None,
            abilitydata: None,
            triggerhpdata: None,
            hue: None,
            modifier_chain: None,
            facades: vec![],
            items_inside: None,
            items_outside: None,
        };
        assert_eq!(block.sprite.name(), "Pikachu");
        assert_eq!(block.sprite.img_data(), "ABC123");
    }

    /// The 2026-04-20 "no legacy back-compat" ruling (verbatim user wording:
    /// "no legacy, always choose correctness over back-compat"): the only
    /// accepted JSON shape for a sprite-bearing IR
    /// type is the new flat `sprite: { name, img_data }` object. Legacy JSON
    /// that ships `sprite_name` + `img_data` as split top-level keys must
    /// fail to deserialize; no compat shim exists.
    #[test]
    fn serde_breaking_change_on_sprite_shape() {
        // Positive: new flat shape deserializes.
        let json_new = r#"{
            "template":"Lost","tier":1,"hp":5,
            "sd":{"faces":[{"Blank":null},{"Blank":null},{"Blank":null},{"Blank":null},{"Blank":null},{"Blank":null}]},
            "sprite":{"name":"Pikachu","img_data":"ABC"},
            "speech":"!","name":"Pikachu","facades":[]
        }"#;
        let parsed: Result<HeroBlock, _> = serde_json::from_str(json_new);
        assert!(parsed.is_ok(), "new flat sprite shape must deserialize: {:?}", parsed.err());
        let block = parsed.unwrap();
        assert_eq!(block.sprite.name(), "Pikachu");
        assert_eq!(block.sprite.img_data(), "ABC");

        // Negative: legacy `sprite_name` + `img_data` keys, no `sprite` field,
        // must fail — serde demands the required `sprite` field.
        let json_legacy = r#"{
            "template":"Lost","tier":1,"hp":5,
            "sd":{"faces":[{"Blank":null},{"Blank":null},{"Blank":null},{"Blank":null},{"Blank":null},{"Blank":null}]},
            "sprite_name":"Pikachu","img_data":"ABC",
            "speech":"!","name":"Pikachu","facades":[]
        }"#;
        let parsed_legacy: Result<HeroBlock, _> = serde_json::from_str(json_legacy);
        assert!(
            parsed_legacy.is_err(),
            "legacy sprite_name+img_data JSON must NOT deserialize into the post-2026-04-20 HeroBlock"
        );
    }
}

// =========================================================================
// New-enum variant + serde-roundtrip pins
// =========================================================================

#[cfg(test)]
mod new_enum_compile_guards {
    use super::*;

    /// Every SummonTrigger variant is constructible and equality-sensible.
    #[test]
    fn summon_trigger_variants_compile_and_eq() {
        let dice = DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1");
        let a = SummonTrigger::SideUse {
            dice: dice.clone(),
            dice_location: DiceLocation::OuterPreface,
        };
        let b = SummonTrigger::SideUse {
            dice: dice.clone(),
            dice_location: DiceLocation::InnerWrapper,
        };
        let c = SummonTrigger::Cast { dice: dice.clone() };
        assert_ne!(a, b, "OuterPreface vs InnerWrapper must be distinct");
        assert_ne!(a, c, "SideUse vs Cast must be distinct");
        // dice_faces() shared accessor returns the same payload across variants.
        assert_eq!(a.dice_faces(), &dice);
        assert_eq!(b.dice_faces(), &dice);
        assert_eq!(c.dice_faces(), &dice);
    }

    /// ItempoolItem's two transitional variants (Summon index, and the
    /// transitional raw-passthrough NonSummon { name, tier, content }) are
    /// constructible, equality-sensible, and serde-roundtrippable. 8A.5
    /// retypes NonSummon into a typed NonSummonEntry sum and extends this
    /// coverage to each typed variant; until then, this test pins the
    /// transitional shape.
    #[test]
    fn itempool_item_variants_compile_and_eq() {
        let summon = ItempoolItem::Summon(0);
        let nonsummon = ItempoolItem::NonSummon {
            name: String::new(),
            tier: None,
            content: "hat.Dragon Egg".into(),
        };
        assert_ne!(summon, nonsummon, "Summon vs NonSummon must be distinct");
        assert_eq!(summon, ItempoolItem::Summon(0), "Summon(0) equals itself");
        assert_ne!(
            summon,
            ItempoolItem::Summon(1),
            "Summon(0) vs Summon(1) must be distinct"
        );

        // Serde round-trip — anchors the Deserialize derive that extract JSON
        // relies on. Breaks loudly if a future edit drops Deserialize, which
        // would silently break `extract` JSON output.
        for item in [summon, nonsummon] {
            let j = serde_json::to_string(&item).expect("ItempoolItem serializes");
            let r: ItempoolItem =
                serde_json::from_str(&j).expect("ItempoolItem deserializes");
            assert_eq!(item, r);
        }

        // SummonTrigger serde round-trip too — catches a dropped Deserialize
        // on SummonTrigger or DiceLocation.
        let t = SummonTrigger::Cast {
            dice: DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1"),
        };
        let j = serde_json::to_string(&t).expect("SummonTrigger serializes");
        let r: SummonTrigger =
            serde_json::from_str(&j).expect("SummonTrigger deserializes");
        assert_eq!(t, r);
    }
}

