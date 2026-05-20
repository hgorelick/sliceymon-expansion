//! Derived structural modifiers — auto-generated from IR content.
//!
//! Character selection (Selector) and hero pool base (HeroPoolBase) can be
//! derived from the hero list. These are generated during build if no explicit
//! structural of that type exists in the IR.

use crate::ir::{
    Hero, ItempoolItem, ReplicaItem, Source, StructuralContent, StructuralModifier,
    StructuralType, SummonTrigger,
};

/// Generate a character selection Selector from the hero list in the
/// canonical SeqPhase form per `reference/textmod_guide.md` §SeqPhase
/// (decisions.md 2026-05-18):
/// `ph.sChoose a Party@1[<H1>][<H2>]…[<HN>]@2!mparty.<H1>+<H2>+…+<HN>` — one
/// button containing the full party list of bracketed `mn_name` labels,
/// followed by a `@2!mparty.+`-joined add action. The bracketed-label list,
/// the `mparty.+` list, and `content.options` all iterate the same
/// color-sorted hero view, so the three ordered sequences agree on hero
/// order. Output is `derived: true` so the strip-and-regen cycle owns it;
/// `name: None` matches the canonical-shape convention shared with
/// `generate_hero_pool_base`.
pub fn generate_char_selection(heroes: &[Hero]) -> StructuralModifier {
    let mut sorted: Vec<&Hero> = heroes.iter().collect();
    sorted.sort_by_key(|h| h.color);

    let mut bracketed = String::new();
    let mut party_list = String::new();
    let mut options = Vec::new();

    for (i, hero) in sorted.iter().enumerate() {
        bracketed.push('[');
        bracketed.push_str(&hero.mn_name);
        bracketed.push(']');
        if i > 0 {
            party_list.push('+');
        }
        party_list.push_str(&hero.mn_name);
        options.push(hero.mn_name.clone());
    }

    let body = format!(
        "ph.sChoose a Party@1{bracketed}@2!mparty.{party_list}"
    );

    StructuralModifier {
        modifier_type: StructuralType::Selector,
        name: None,
        content: StructuralContent::Selector { body, options },
        derived: true,
        source: Source::Base,
    }
}

/// Generate a canonical-shape `PoolReplacement` structural from the hero
/// list — emits `((heropool.<h1.internal_name>+<h2.internal_name>+…))` for
/// the `derived: true` synthesized case. The four-mod byte-match against
/// `working-mods/punpuns.txt`'s richer
/// `((heropool.<list>)&Hidden).doc.<text>.mn.<name>` corpus shape, including
/// inline `(replica.<X>.abilitydata.(...)).n.<X>` rows, is delivered by the
/// future typed-payload retype of `StructuralContent::PoolReplacement` per
/// decisions.md 2026-05-19, which widens this signature to consume the typed
/// payload. `name: None` here pins the seam the retype widens.
pub fn generate_pool_replacement(heroes: &[Hero]) -> StructuralModifier {
    let hero_names: Vec<String> = heroes.iter().map(|h| h.internal_name.clone()).collect();
    let body = format!("((heropool.{}))", hero_names.join("+"));

    StructuralModifier {
        modifier_type: StructuralType::PoolReplacement,
        name: None,
        content: StructuralContent::PoolReplacement { body, hero_names },
        derived: true,
        source: Source::Base,
    }
}

/// Generate hero-bound ItemPool structurals from the trigger-based
/// `ReplicaItem` list.
///
/// Walks `replica_items`; each `SummonTrigger::SideUse` (both `OuterPreface`
/// and `InnerWrapper` — `dice_location` is a source-shape sub-axis, not a
/// game-mechanic axis) whose `target_name` matches a hero's `mn_name`
/// routes into a hero-bound pool keyed on that hero's `internal_name`.
/// `Cast` entries are skipped — Cast summons have their own top-level
/// emission path per the emitter's trigger dispatch and are not
/// hero-pool-routed.
///
/// Post-8A stub: the `extract_from_itempool` stub produces zero
/// `ReplicaItem` entries, so this function produces zero output from
/// extracted corpus input. The future real parser surfaces SideUse entries
/// that this function routes into hero-bound pools. A byte-match-vs-
/// sliceymon round-trip test for this function lands with that real parser
/// (it requires `ir.replica_items` populated).
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
            if replica.target_name.to_lowercase() != hero_lower {
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
        // Should contain auto-generated selector and hero pool in the
        // canonical SeqPhase form per reference/textmod_guide.md §SeqPhase
        // (per decisions.md 2026-05-18): one button with bracketed labels
        // followed by an `@2!mparty.+`-joined add action.
        assert!(
            output.contains("ph.sChoose a Party@1[Alpha][Beta][Gamma]"),
            "missing canonical-form bracketed-label list — got:\n{}",
            output
        );
        assert!(
            output.contains("@2!mparty.Alpha+Beta+Gamma"),
            "missing canonical-form mparty.+ add action — got:\n{}",
            output
        );
        assert!(output.contains("heropool."), "missing hero pool base");
    }

    fn make_sideuse_replica(target: &str) -> ReplicaItem {
        use crate::ir::{DiceLocation, SummonTrigger};
        ReplicaItem {
            container_name: format!("{} Ball", target),
            target_name: target.to_string(),
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
            target_name: target.to_string(),
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
    /// `target_name` matches a hero's `mn_name` into a hero-bound
    /// `StructuralType::ItemPool` keyed as `<Hero>Item`. Cast entries are
    /// skipped (they emit through the top-level replica loop, not the pool).
    #[test]
    fn generate_hero_item_pool_routes_sideuse_by_target_name() {
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

    /// `generate_pool_replacement` emits the canonical `((heropool.<list>))`
    /// shape from `&[Hero]`, populates `hero_names` from `internal_name`, and
    /// sets `derived: true` + `source: Source::Base`. Pinned per
    /// decisions.md 2026-05-19 (the canonical-shape regenerator; the future
    /// typed-payload retype widens this signature later).
    #[test]
    fn generate_pool_replacement_canonical_shape() {
        let heroes = vec![
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
            make_hero("Gamma", 'c'),
        ];
        let modifier = generate_pool_replacement(&heroes);
        assert_eq!(modifier.modifier_type, StructuralType::PoolReplacement);
        assert!(modifier.derived);
        assert_eq!(modifier.source, Source::Base);
        assert_eq!(modifier.name, None,
            "canonical-shape body carries no .mn.<name> suffix; sibling chunk plumbs typed name");
        match &modifier.content {
            StructuralContent::PoolReplacement { body, hero_names } => {
                assert_eq!(body, "((heropool.alpha+beta+gamma))");
                assert_eq!(hero_names, &["alpha", "beta", "gamma"]);
            }
            other => panic!("expected PoolReplacement content, got {:?}", other),
        }
    }

    /// Source-vs-IR divergence: altering `internal_name` shifts the regenerated
    /// `body` bytes accordingly (proves the regenerator reads from content
    /// rather than hardcoding canonical bytes; mirrors the brief's
    /// "Derived-structural regenerators are complete and corpus-matched"
    /// Goal verifier's per-regenerator divergence property).
    #[test]
    fn generate_pool_replacement_reads_internal_name_from_content() {
        let mut heroes = vec![make_hero("Foo", 'a'), make_hero("Bar", 'b')];
        // Alter internal_names — the regenerated body must reflect them.
        heroes[0].internal_name = "renamedfoo".into();
        heroes[1].internal_name = "renamedbar".into();
        let modifier = generate_pool_replacement(&heroes);
        match &modifier.content {
            StructuralContent::PoolReplacement { body, hero_names } => {
                assert_eq!(body, "((heropool.renamedfoo+renamedbar))");
                assert_eq!(hero_names, &["renamedfoo", "renamedbar"]);
            }
            other => panic!("expected PoolReplacement content, got {:?}", other),
        }
    }

    /// `generate_char_selection`'s rewritten body matches the canonical
    /// SeqPhase form per `reference/textmod_guide.md` §SeqPhase
    /// (`ph.sChoose a Party@1[Hero1][Hero2]…@2!mparty.Hero1+Hero2+…`) per
    /// decisions.md 2026-05-18. The adversarial non-color-sorted input order
    /// discriminates between an implementation that iterates the color-sorted
    /// view (correct) and one that iterates the raw `&[Hero]` (regression).
    #[test]
    fn generate_char_selection_canonical_form_byte_match() {
        let heroes = vec![
            make_hero("Gamma", 'c'),
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
        ];
        let sel = generate_char_selection(&heroes);
        assert_eq!(sel.modifier_type, StructuralType::Selector);
        assert!(sel.derived);
        assert_eq!(sel.source, Source::Base);
        assert_eq!(sel.name, None);
        match &sel.content {
            StructuralContent::Selector { body, options } => {
                assert_eq!(
                    body,
                    "ph.sChoose a Party@1[Alpha][Beta][Gamma]@2!mparty.Alpha+Beta+Gamma"
                );
                // options[] also iterates the color-sorted view — pinned by the
                // existing generate_char_selection_alphabetical test.
                assert_eq!(options, &["Alpha", "Beta", "Gamma"]);
            }
            other => panic!("expected Selector content, got {:?}", other),
        }
    }

    /// Source-vs-IR divergence for `generate_char_selection`: altered
    /// `mn_name` values must shift the body bytes in both the bracketed-label
    /// list AND the `@2!mparty.+`-joined add-action list.
    #[test]
    fn generate_char_selection_reads_mn_name_from_content() {
        let mut heroes = vec![make_hero("Original1", 'a'), make_hero("Original2", 'b')];
        heroes[0].mn_name = "Renamed1".into();
        heroes[1].mn_name = "Renamed2".into();
        let sel = generate_char_selection(&heroes);
        match &sel.content {
            StructuralContent::Selector { body, options } => {
                assert_eq!(
                    body,
                    "ph.sChoose a Party@1[Renamed1][Renamed2]@2!mparty.Renamed1+Renamed2"
                );
                assert_eq!(options, &["Renamed1", "Renamed2"]);
            }
            other => panic!("expected Selector content, got {:?}", other),
        }
    }

    /// The strip-regenerate cycle (SPEC §4) must re-author stripped derived
    /// hero-bound ItemPools. Pre-8A the `regenerate_derived_kinds` match arm
    /// for `ItemPool` dropped the kind silently — this test locks the wiring
    /// so a regression that re-introduces `_ => {}` for `ItemPool` fails
    /// loudly. Guards against the "dead `generate_hero_item_pool`" class of
    /// defect caught in PR #14 round-1 tribunal.
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
