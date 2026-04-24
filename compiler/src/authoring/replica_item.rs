#![allow(missing_docs)]
//! Authoring builder for `ReplicaItem` — the trigger-based summon shape.
//!
//! Two builders, one per player-action variant: `SideUseBuilder` and
//! `CastBuilder`. Both use a `PhantomData` type-state so `.build()` is only
//! available once dice have been set — a hallucinated empty-dice builder is
//! a compile error, not a runtime surprise.
//!
//! Cast carries no ability-payload field (corpus has zero depth-0
//! `.n.<spell_name>` inside the spell-cast envelope's inner body; parent
//! plan §1.1). Widening is a separate PR with variant fields — see the
//! `SummonTrigger::Cast` doc-comment in `ir/mod.rs` for the full rationale.

use std::marker::PhantomData;

use crate::authoring::SpriteId;
use crate::ir::{DiceFaces, DiceLocation, ModifierChain, ReplicaItem, Source, SummonTrigger};

/// Type-state flag: dice have not been set.
pub struct NoDice;
/// Type-state flag: dice have been set — `.build()` now available.
pub struct HasDice;

// ---------------------------------------------------------------------
// SideUseBuilder — thief-side summons (OuterPreface + InnerWrapper corpus)
// ---------------------------------------------------------------------

pub struct SideUseBuilder<D> {
    container_name: String,
    target_pokemon: String,
    enemy_template: Option<String>,
    team_template: Option<String>,
    sprite: Option<SpriteId>,
    dice: DiceFaces,
    dice_location: DiceLocation,
    tier: Option<u8>,
    hp: Option<u16>,
    color: Option<char>,
    sticker_stack: Option<ModifierChain>,
    speech: Option<String>,
    doc: Option<String>,
    toggle_flags: Option<String>,
    item_modifiers: Option<ModifierChain>,
    source: Source,
    _state: PhantomData<D>,
}

impl SideUseBuilder<NoDice> {
    pub fn new(container_name: impl Into<String>, target_pokemon: impl Into<String>) -> Self {
        Self {
            container_name: container_name.into(),
            target_pokemon: target_pokemon.into(),
            enemy_template: None,
            team_template: None,
            sprite: None,
            dice: DiceFaces::default(),
            dice_location: DiceLocation::OuterPreface,
            tier: None,
            hp: None,
            color: None,
            sticker_stack: None,
            speech: None,
            doc: None,
            toggle_flags: None,
            item_modifiers: None,
            source: Source::default(),
            _state: PhantomData,
        }
    }

    /// Sets dice and advances the type-state to `HasDice`, unlocking
    /// `.build()`. Before this, `.build()` is a compile error.
    pub fn dice(self, faces: DiceFaces) -> SideUseBuilder<HasDice> {
        SideUseBuilder {
            container_name: self.container_name,
            target_pokemon: self.target_pokemon,
            enemy_template: self.enemy_template,
            team_template: self.team_template,
            sprite: self.sprite,
            dice: faces,
            dice_location: self.dice_location,
            tier: self.tier,
            hp: self.hp,
            color: self.color,
            sticker_stack: self.sticker_stack,
            speech: self.speech,
            doc: self.doc,
            toggle_flags: self.toggle_flags,
            item_modifiers: self.item_modifiers,
            source: self.source,
            _state: PhantomData,
        }
    }
}

// Setters available in either state; applied to both variants.
impl<D> SideUseBuilder<D> {
    pub fn enemy_template(mut self, v: impl Into<String>) -> Self {
        self.enemy_template = Some(v.into());
        self
    }
    pub fn team_template(mut self, v: impl Into<String>) -> Self {
        self.team_template = Some(v.into());
        self
    }
    pub fn sprite(mut self, v: SpriteId) -> Self {
        self.sprite = Some(v);
        self
    }
    pub fn dice_location(mut self, v: DiceLocation) -> Self {
        self.dice_location = v;
        self
    }
    pub fn tier(mut self, v: u8) -> Self {
        self.tier = Some(v);
        self
    }
    pub fn hp(mut self, v: u16) -> Self {
        self.hp = Some(v);
        self
    }
    pub fn color(mut self, v: char) -> Self {
        self.color = Some(v);
        self
    }
    pub fn sticker_stack(mut self, v: ModifierChain) -> Self {
        self.sticker_stack = Some(v);
        self
    }
    pub fn speech(mut self, v: impl Into<String>) -> Self {
        self.speech = Some(v.into());
        self
    }
    pub fn doc(mut self, v: impl Into<String>) -> Self {
        self.doc = Some(v.into());
        self
    }
    pub fn toggle_flags(mut self, v: impl Into<String>) -> Self {
        self.toggle_flags = Some(v.into());
        self
    }
    pub fn item_modifiers(mut self, v: ModifierChain) -> Self {
        self.item_modifiers = Some(v);
        self
    }
    pub fn source(mut self, v: Source) -> Self {
        self.source = v;
        self
    }
}

impl SideUseBuilder<HasDice> {
    pub fn build(self) -> ReplicaItem {
        ReplicaItem {
            container_name: self.container_name,
            target_pokemon: self.target_pokemon,
            trigger: SummonTrigger::SideUse {
                dice: self.dice,
                dice_location: self.dice_location,
            },
            enemy_template: self.enemy_template.unwrap_or_default(),
            team_template: self.team_template.unwrap_or_default(),
            tier: self.tier,
            hp: self.hp,
            color: self.color,
            sprite: self.sprite.unwrap_or_else(|| SpriteId::owned("", "")),
            sticker_stack: self.sticker_stack,
            speech: self.speech,
            doc: self.doc,
            toggle_flags: self.toggle_flags,
            item_modifiers: self.item_modifiers,
            source: self.source,
        }
    }
}

// ---------------------------------------------------------------------
// CastBuilder — spell-cast summons. No ability-payload field (corpus zero-ev.)
// ---------------------------------------------------------------------

pub struct CastBuilder<D> {
    container_name: String,
    target_pokemon: String,
    enemy_template: Option<String>,
    team_template: Option<String>,
    sprite: Option<SpriteId>,
    dice: DiceFaces,
    tier: Option<u8>,
    hp: Option<u16>,
    color: Option<char>,
    sticker_stack: Option<ModifierChain>,
    speech: Option<String>,
    doc: Option<String>,
    toggle_flags: Option<String>,
    item_modifiers: Option<ModifierChain>,
    source: Source,
    _state: PhantomData<D>,
}

impl CastBuilder<NoDice> {
    pub fn new(container_name: impl Into<String>, target_pokemon: impl Into<String>) -> Self {
        Self {
            container_name: container_name.into(),
            target_pokemon: target_pokemon.into(),
            enemy_template: None,
            team_template: None,
            sprite: None,
            dice: DiceFaces::default(),
            tier: None,
            hp: None,
            color: None,
            sticker_stack: None,
            speech: None,
            doc: None,
            toggle_flags: None,
            item_modifiers: None,
            source: Source::default(),
            _state: PhantomData,
        }
    }

    /// Sets per-item Cast dice and advances the type-state to `HasDice`,
    /// unlocking `.build()`. The outer universal cast-template + cast-dice
    /// are emitter literals in `builder/replica_item_emitter.rs` — builder
    /// exposes no knob for them.
    pub fn dice(self, faces: DiceFaces) -> CastBuilder<HasDice> {
        CastBuilder {
            container_name: self.container_name,
            target_pokemon: self.target_pokemon,
            enemy_template: self.enemy_template,
            team_template: self.team_template,
            sprite: self.sprite,
            dice: faces,
            tier: self.tier,
            hp: self.hp,
            color: self.color,
            sticker_stack: self.sticker_stack,
            speech: self.speech,
            doc: self.doc,
            toggle_flags: self.toggle_flags,
            item_modifiers: self.item_modifiers,
            source: self.source,
            _state: PhantomData,
        }
    }
}

impl<D> CastBuilder<D> {
    pub fn enemy_template(mut self, v: impl Into<String>) -> Self {
        self.enemy_template = Some(v.into());
        self
    }
    pub fn team_template(mut self, v: impl Into<String>) -> Self {
        self.team_template = Some(v.into());
        self
    }
    pub fn sprite(mut self, v: SpriteId) -> Self {
        self.sprite = Some(v);
        self
    }
    pub fn tier(mut self, v: u8) -> Self {
        self.tier = Some(v);
        self
    }
    pub fn hp(mut self, v: u16) -> Self {
        self.hp = Some(v);
        self
    }
    pub fn color(mut self, v: char) -> Self {
        self.color = Some(v);
        self
    }
    pub fn sticker_stack(mut self, v: ModifierChain) -> Self {
        self.sticker_stack = Some(v);
        self
    }
    pub fn speech(mut self, v: impl Into<String>) -> Self {
        self.speech = Some(v.into());
        self
    }
    pub fn doc(mut self, v: impl Into<String>) -> Self {
        self.doc = Some(v.into());
        self
    }
    pub fn toggle_flags(mut self, v: impl Into<String>) -> Self {
        self.toggle_flags = Some(v.into());
        self
    }
    pub fn item_modifiers(mut self, v: ModifierChain) -> Self {
        self.item_modifiers = Some(v);
        self
    }
    pub fn source(mut self, v: Source) -> Self {
        self.source = v;
        self
    }
}

impl CastBuilder<HasDice> {
    pub fn build(self) -> ReplicaItem {
        ReplicaItem {
            container_name: self.container_name,
            target_pokemon: self.target_pokemon,
            trigger: SummonTrigger::Cast {
                dice: self.dice,
            },
            enemy_template: self.enemy_template.unwrap_or_default(),
            team_template: self.team_template.unwrap_or_default(),
            tier: self.tier,
            hp: self.hp,
            color: self.color,
            sprite: self.sprite.unwrap_or_else(|| SpriteId::owned("", "")),
            sticker_stack: self.sticker_stack,
            speech: self.speech,
            doc: self.doc,
            toggle_flags: self.toggle_flags,
            item_modifiers: self.item_modifiers,
            source: self.source,
        }
    }
}

// =========================================================================
// Chunk 8A tests: T24 / T25 / T26 / T26a
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// T24 — SideUse builder round-trips construction through type-state.
    #[test]
    fn replica_item_builder_sideuse() {
        let item = SideUseBuilder::new("Great Ball", "Ivysaur")
            .enemy_template("Wolf")
            .team_template("housecat")
            .sprite(SpriteId::owned("ivysaur", ""))
            .dice(DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1"))
            .dice_location(DiceLocation::OuterPreface)
            .build();

        assert_eq!(item.container_name, "Great Ball");
        assert_eq!(item.target_pokemon, "Ivysaur");
        assert!(matches!(
            item.trigger,
            SummonTrigger::SideUse {
                dice_location: DiceLocation::OuterPreface,
                ..
            }
        ));
    }

    /// T25 — Cast builder round-trips construction through type-state.
    #[test]
    fn replica_item_builder_cast() {
        let item = CastBuilder::new("Silver Wing", "Lugia")
            .enemy_template("Wolf")
            .team_template("prodigy")
            .sprite(SpriteId::owned("lugia", ""))
            .dice(DiceFaces::parse("15-20:15-20:36-10:36-10:184-5:184-5"))
            .build();

        assert_eq!(item.container_name, "Silver Wing");
        assert!(matches!(item.trigger, SummonTrigger::Cast { .. }));
    }

    /// T26 — Emitter produces structurally valid output for builder-constructed
    /// IR. String-containment (NOT byte-equality vs corpus — that ships in 8B
    /// as T1). Scope: emitter structurally correct for authoring-constructed
    /// input.
    #[test]
    fn replica_item_emits_inside_itempool() {
        let item = SideUseBuilder::new("Poke Ball", "Pikachu")
            .enemy_template("Wolf")
            .team_template("housecat")
            .sprite(SpriteId::owned("pikachu", ""))
            .dice(DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1"))
            .dice_location(DiceLocation::OuterPreface)
            .build();

        let emitted = crate::builder::replica_item_emitter::emit_replica_item(&item);
        assert!(
            emitted.contains("hat.replica.Thief.n.Pikachu"),
            "SideUse outer preface emit missing; got: {}",
            emitted
        );
        assert!(
            emitted.contains("vase.(add.((replica.housecat.n.Pikachu"),
            "vase-add pair missing; got: {}",
            emitted
        );
        assert!(
            emitted.contains(".mn.Pikachu"),
            ".mn. tag missing; got: {}",
            emitted
        );
    }

    /// T26a — Source-vs-IR divergence guard. Builds synthetic ReplicaItems
    /// whose `team_template` casing / Pokemon pairings are deliberately
    /// non-corpus; asserts the emitter routes those IR bytes verbatim. If
    /// any code path substitutes a registry-canonical or derived value for
    /// the builder's source bytes, one of these assertions fails loudly.
    #[test]
    fn replica_item_emitter_preserves_source_byte_templates() {
        // SideUse / non-corpus casing: "Housecat" (capital H) for Ivysaur —
        // corpus entry for Ivysaur uses lowercase "housecat". If the emitter
        // reached for a canonical lowercasing or registry-keyed lookup, the
        // emitted `vase.(add.((replica.housecat.n.Ivysaur` would LOSE the
        // capital `H` the builder set.
        let item = SideUseBuilder::new("Synthetic Ball", "Ivysaur")
            .enemy_template("Wolf")
            .team_template("Housecat")
            .sprite(SpriteId::owned("ivysaur", ""))
            .dice(DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1"))
            .dice_location(DiceLocation::OuterPreface)
            .build();
        let emitted = crate::builder::replica_item_emitter::emit_replica_item(&item);
        assert!(
            emitted.contains("vase.(add.((replica.Housecat.n.Ivysaur"),
            "emitter must route team_template bytes verbatim — no case \
             normalization, no registry lookup. Got: {}",
            emitted,
        );

        // Cast / deliberately non-corpus pairing: team_template = "prodigy"
        // combined with Groudon (corpus entry uses "Statue"). Must emit
        // exactly what the IR says, not what corpus prefers for that Pokemon.
        let cast_item = CastBuilder::new("Synthetic Orb", "Groudon")
            .enemy_template("dragon")
            .team_template("prodigy")
            .sprite(SpriteId::owned("groudon", ""))
            .dice(DiceFaces::parse("36-10:36-10:0:0:36-10:0"))
            .build();
        let cast_emitted =
            crate::builder::replica_item_emitter::emit_replica_item(&cast_item);
        assert!(
            cast_emitted.contains("vase.(add.((replica.prodigy.n.Groudon"),
            "Cast emitter must route team_template from IR — not a per-Pokemon \
             registry default. Got: {}",
            cast_emitted,
        );
    }
}
