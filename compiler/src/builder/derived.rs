//! Derived structural modifiers — auto-generated from IR content.
//!
//! Character selection (Selector) and hero pool base (HeroPoolBase) can be
//! derived from the hero list. These are generated during build if no explicit
//! structural of that type exists in the IR.

use crate::ir::{
    Hero, ItempoolItem, ReplicaItem, Source, StructuralContent, StructuralModifier,
    StructuralType, SummonTrigger,
};

/// Generate a character selection Selector from the hero list.
/// Options are sorted by hero color alphabetically.
pub fn generate_char_selection(heroes: &[Hero]) -> StructuralModifier {
    let mut sorted: Vec<&Hero> = heroes.iter().collect();
    sorted.sort_by_key(|h| h.color);

    let mut body = String::from("1.ph.s");
    let mut options = Vec::new();

    for hero in &sorted {
        body.push_str(&format!("@1{}", hero.mn_name));
        options.push(hero.mn_name.clone());
    }

    StructuralModifier {
        modifier_type: StructuralType::Selector,
        name: None,
        content: StructuralContent::Selector { body, options },
        derived: true,
        source: Source::Base,
    }
}

/// Generate hero-bound ItemPool structurals from the trigger-based
/// `ReplicaItem` list.
///
/// Walks `replica_items`; each `SummonTrigger::SideUse` (both `OuterPreface`
/// and `InnerWrapper` — `dice_location` is a source-shape sub-axis, not a
/// game-mechanic axis) whose `target_pokemon` matches a hero's `mn_name`
/// routes into a hero-bound pool keyed on that hero's `internal_name`.
/// `Cast` entries are skipped — Cast summons have their own top-level
/// emission path per the emitter's trigger dispatch and are not
/// hero-pool-routed.
///
/// Post-8A stub: the `extract_from_itempool` stub produces zero
/// `ReplicaItem` entries, so this function produces zero output from
/// extracted corpus input. 8B's real parser surfaces SideUse entries that
/// this function routes into hero-bound pools. A byte-match-vs-sliceymon
/// round-trip test for this function ships in 8B (it requires the real
/// parser to populate `ir.replica_items`).
///
/// Returned `StructuralModifier` entries carry `derived: true` so the
/// merge / build strip-regenerate cycle (SPEC §4) handles them uniformly
/// with `generate_char_selection` / `generate_hero_pool_base`.
pub fn generate_hero_item_pool(
    heroes: &[Hero],
    replica_items: &[ReplicaItem],
) -> Vec<StructuralModifier> {
    let mut out: Vec<StructuralModifier> = Vec::new();
    for hero in heroes {
        let hero_lower = hero.mn_name.to_lowercase();
        let mut items: Vec<ItempoolItem> = Vec::new();
        for (i, replica) in replica_items.iter().enumerate() {
            if !matches!(replica.trigger, SummonTrigger::SideUse { .. }) {
                continue;
            }
            if replica.target_pokemon.to_lowercase() != hero_lower {
                continue;
            }
            items.push(ItempoolItem::Summon(i));
        }
        if items.is_empty() {
            continue;
        }
        out.push(StructuralModifier {
            modifier_type: StructuralType::ItemPool,
            name: Some(format!("{}Item", hero.mn_name)),
            content: StructuralContent::ItemPool { items },
            derived: true,
            source: Source::Base,
        });
    }
    out
}

/// Generate a HeroPoolBase from the hero list.
/// Lists hero internal_names as pool references.
pub fn generate_hero_pool_base(heroes: &[Hero]) -> StructuralModifier {
    let hero_refs: Vec<String> = heroes.iter().map(|h| h.internal_name.clone()).collect();
    let body = format!("heropool.{}", hero_refs.join("."));

    StructuralModifier {
        modifier_type: StructuralType::HeroPoolBase,
        name: None,
        content: StructuralContent::HeroPoolBase {
            body,
            hero_refs,
        },
        derived: true,
        source: Source::Base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{DiceFaces, HeroBlock, HeroFormat};

    fn make_hero(name: &str, color: char) -> Hero {
        Hero {
            internal_name: name.to_lowercase(),
            mn_name: name.to_string(),
            color,
            format: HeroFormat::Sliceymon,
            blocks: vec![HeroBlock {
                template: "Lost".into(),
                tier: Some(1),
                hp: Some(5),
                sd: DiceFaces::parse("0:0:0:0:0:0"),
                color: None,
                sprite: crate::authoring::SpriteId::owned(name.to_string(), "test"),
                speech: "!".into(),
                name: name.into(),
                doc: None,
                abilitydata: None,
                triggerhpdata: None,
                hue: None,
                modifier_chain: None,
                facades: vec![],
                items_inside: None,
                items_outside: None,
                bare: false,
            }],
            removed: false,
            source: Source::Base,
        }
    }

    #[test]
    fn generate_char_selection_from_heroes() {
        let heroes = vec![
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
            make_hero("Gamma", 'c'),
        ];
        let sel = generate_char_selection(&heroes);
        assert_eq!(sel.modifier_type, StructuralType::Selector);
        assert!(sel.derived);
        if let StructuralContent::Selector { options, .. } = &sel.content {
            assert_eq!(options.len(), 3);
            assert_eq!(options[0], "Alpha");
            assert_eq!(options[1], "Beta");
            assert_eq!(options[2], "Gamma");
        } else {
            panic!("Expected Selector content");
        }
    }

    #[test]
    fn generate_char_selection_alphabetical() {
        let heroes = vec![
            make_hero("Gamma", 'c'),
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
        ];
        let sel = generate_char_selection(&heroes);
        if let StructuralContent::Selector { options, .. } = &sel.content {
            assert_eq!(options, &["Alpha", "Beta", "Gamma"]);
        } else {
            panic!("Expected Selector content");
        }
    }

    #[test]
    fn generate_hero_pool_base_from_heroes() {
        let heroes = vec![
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
            make_hero("Gamma", 'c'),
        ];
        let pool = generate_hero_pool_base(&heroes);
        assert_eq!(pool.modifier_type, StructuralType::HeroPoolBase);
        assert!(pool.derived);
        if let StructuralContent::HeroPoolBase { hero_refs, .. } = &pool.content {
            assert_eq!(hero_refs, &["alpha", "beta", "gamma"]);
        } else {
            panic!("Expected HeroPoolBase content");
        }
    }

    #[test]
    fn char_selection_updates_on_add_hero() {
        let mut heroes = vec![
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
            make_hero("Gamma", 'c'),
        ];
        heroes.push(make_hero("Delta", 'd'));
        let sel = generate_char_selection(&heroes);
        if let StructuralContent::Selector { options, .. } = &sel.content {
            assert_eq!(options.len(), 4);
        } else {
            panic!("Expected Selector content");
        }
    }

    #[test]
    fn char_selection_updates_on_remove_hero() {
        let heroes = vec![
            make_hero("Alpha", 'a'),
            make_hero("Gamma", 'c'),
        ];
        let sel = generate_char_selection(&heroes);
        if let StructuralContent::Selector { options, .. } = &sel.content {
            assert_eq!(options.len(), 2);
        } else {
            panic!("Expected Selector content");
        }
    }

    #[test]
    fn derived_flag_set_on_generated() {
        let heroes = vec![make_hero("Alpha", 'a')];
        assert!(generate_char_selection(&heroes).derived);
        assert!(generate_hero_pool_base(&heroes).derived);
    }

    #[test]
    fn builder_auto_generates_derived_structurals() {
        use crate::ir::ModIR;

        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Alpha", 'a'));
        ir.heroes.push(make_hero("Beta", 'b'));
        ir.heroes.push(make_hero("Gamma", 'c'));

        let output = crate::builder::build_complete(&ir).unwrap();
        // Should contain auto-generated selector and hero pool
        assert!(output.contains("@1Alpha"), "missing char selection option Alpha");
        assert!(output.contains("@1Beta"), "missing char selection option Beta");
        assert!(output.contains("heropool."), "missing hero pool base");
    }

    fn make_sideuse_replica(target: &str) -> ReplicaItem {
        use crate::ir::{DiceLocation, SummonTrigger};
        ReplicaItem {
            container_name: format!("{} Ball", target),
            target_pokemon: target.to_string(),
            trigger: SummonTrigger::SideUse {
                dice: DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1"),
                dice_location: DiceLocation::OuterPreface,
            },
            enemy_template: "Wolf".into(),
            team_template: "housecat".into(),
            tier: Some(1),
            hp: Some(4),
            color: None,
            sprite: crate::authoring::SpriteId::owned(target.to_lowercase(), ""),
            sticker_stack: None,
            speech: None,
            doc: None,
            toggle_flags: None,
            item_modifiers: None,
            source: Source::Base,
        }
    }

    fn make_cast_replica(target: &str) -> ReplicaItem {
        use crate::ir::SummonTrigger;
        ReplicaItem {
            container_name: format!("{} Orb", target),
            target_pokemon: target.to_string(),
            trigger: SummonTrigger::Cast {
                dice: DiceFaces::parse("36-10:36-10:0:0:36-10:0"),
            },
            enemy_template: "dragon".into(),
            team_template: "prodigy".into(),
            tier: Some(3),
            hp: Some(30),
            color: None,
            sprite: crate::authoring::SpriteId::owned(target.to_lowercase(), ""),
            sticker_stack: None,
            speech: None,
            doc: None,
            toggle_flags: None,
            item_modifiers: None,
            source: Source::Base,
        }
    }

    /// `generate_hero_item_pool` routes each SideUse replica whose
    /// `target_pokemon` matches a hero's `mn_name` into a hero-bound
    /// `StructuralType::ItemPool` keyed as `<Hero>Item`. Cast entries are
    /// skipped (they emit through the top-level replica loop, not the pool).
    #[test]
    fn generate_hero_item_pool_routes_sideuse_by_target_pokemon() {
        let heroes = vec![make_hero("Alpha", 'a'), make_hero("Beta", 'b')];
        let replica_items = vec![
            make_sideuse_replica("Alpha"), // index 0 — Alpha's pool
            make_cast_replica("Alpha"),    // index 1 — skipped (Cast)
            make_sideuse_replica("Beta"),  // index 2 — Beta's pool
            make_sideuse_replica("Alpha"), // index 3 — Alpha's pool again
        ];
        let pools = generate_hero_item_pool(&heroes, &replica_items);

        assert_eq!(pools.len(), 2, "one hero-bound pool per hero with >=1 SideUse match");

        let alpha_pool = pools.iter().find(|p| p.name.as_deref() == Some("AlphaItem"))
            .expect("AlphaItem pool present");
        assert_eq!(alpha_pool.modifier_type, StructuralType::ItemPool);
        assert!(alpha_pool.derived, "derived flag must be set so strip+regenerate cycle owns the pool");
        assert_eq!(alpha_pool.source, Source::Base);
        match &alpha_pool.content {
            StructuralContent::ItemPool { items } => {
                assert_eq!(
                    items,
                    &vec![ItempoolItem::Summon(0), ItempoolItem::Summon(3)],
                    "Alpha pool routes replica indices 0 and 3; Cast at index 1 skipped"
                );
            }
            other => panic!("expected ItemPool content, got {:?}", other),
        }

        let beta_pool = pools.iter().find(|p| p.name.as_deref() == Some("BetaItem"))
            .expect("BetaItem pool present");
        match &beta_pool.content {
            StructuralContent::ItemPool { items } => {
                assert_eq!(items, &vec![ItempoolItem::Summon(2)]);
            }
            other => panic!("expected ItemPool content, got {:?}", other),
        }
    }

    /// Heroes with no SideUse matches produce no pool at all — the derived
    /// structural is absent rather than empty.
    #[test]
    fn generate_hero_item_pool_skips_heroes_with_no_sideuse_matches() {
        let heroes = vec![make_hero("Alpha", 'a'), make_hero("Beta", 'b')];
        let replica_items = vec![make_sideuse_replica("Alpha")];
        let pools = generate_hero_item_pool(&heroes, &replica_items);
        assert_eq!(pools.len(), 1, "only heroes with matches get a pool");
        assert_eq!(pools[0].name.as_deref(), Some("AlphaItem"));
    }

    /// The strip-regenerate cycle (SPEC §4) must re-author stripped derived
    /// hero-bound ItemPools. Pre-8A the `regenerate_derived_kinds` match arm
    /// for `ItemPool` dropped the kind silently — this test locks the wiring
    /// so a regression that re-introduces `_ => {}` for `ItemPool` fails
    /// loudly. Guards against the "dead `generate_hero_item_pool`" class of
    /// defect caught in the round-1 tribunal.
    #[test]
    fn regenerate_derived_kinds_rebuilds_hero_item_pool() {
        use crate::ir::merge::regenerate_derived_kinds;

        let heroes = vec![make_hero("Alpha", 'a')];
        let replica_items = vec![make_sideuse_replica("Alpha")];
        let mut structural: Vec<StructuralModifier> = Vec::new();

        regenerate_derived_kinds(
            &mut structural,
            &heroes,
            &replica_items,
            &[StructuralType::ItemPool],
        );

        assert_eq!(
            structural.len(),
            1,
            "ItemPool arm must regenerate via generate_hero_item_pool"
        );
        assert_eq!(structural[0].modifier_type, StructuralType::ItemPool);
        assert_eq!(structural[0].name.as_deref(), Some("AlphaItem"));
        match &structural[0].content {
            StructuralContent::ItemPool { items } => {
                assert_eq!(items, &vec![ItempoolItem::Summon(0)]);
            }
            other => panic!("expected ItemPool content, got {:?}", other),
        }
    }
}
