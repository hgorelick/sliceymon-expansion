use crate::ir::{ReplicaItem, ReplicaItemContainer, Source};
use crate::util;

/// Parse a simple replica item (no ability) from a modifier string.
/// Format: itempool.((hat.replica.TEMPLATE...)).n.CONTAINER_NAME.mn.NAME
pub fn parse_simple(modifier: &str, _modifier_index: usize) -> ReplicaItem {
    // Scalar fields (hp/sd/color/img) must only be scanned in the prefix
    // preceding any depth-0 chain / cast / ability block. Chain sub-entries
    // carry free-form `.sidesc.` / `.enchant.` text and the outer cast-like
    // `.abilitydata.(...)` region can contain `.hp.` / `.col.` / `.sd.` /
    // `.img.` substrings at nested depth — none of those may leak into
    // top-level IR fields. (§F10, Chunk 9.)
    //
    // Captures keep non-depth-aware scans inside `scalar_slice`: the legit
    // scalars live at paren depth ≥ 2 (inside the outer `((hat.replica...))`
    // wrap), so `depth_aware = true` would skip them. Correctness for the
    // leak class is instead guaranteed by emission order (scalars always
    // emitted before the replica's chain segment), plus `scalar_slice`
    // trimming the outer sticker / cast / ability regions.
    let scalar_slice = util::slice_before_chain_and_cast(modifier);
    let name = util::extract_mn_name(modifier).unwrap_or_default();
    let sd = util::extract_sd(scalar_slice, false)
        .map(|s| crate::ir::DiceFaces::parse(&s))
        .unwrap_or_else(|| crate::ir::DiceFaces { faces: vec![] });
    let template = util::extract_template(modifier).unwrap_or_default();
    let hp = util::extract_hp(scalar_slice, false);
    let color = util::extract_color(scalar_slice, false);
    let tier = util::extract_simple_prop(modifier, ".tier.")
        .and_then(|v| v.parse::<u8>().ok());
    let item_modifiers = util::extract_modifier_chain(modifier)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let sticker = util::extract_simple_prop(modifier, ".sticker.");
    let toggle_flags = extract_toggle_flags(modifier);
    let img_data = util::extract_img_data(scalar_slice);
    let sprite = crate::authoring::SpriteId::owned(name.clone(), img_data.unwrap_or_default());

    ReplicaItem {
        name,
        container: ReplicaItemContainer::Capture {
            name: extract_container_name(modifier).unwrap_or_default(),
        },
        tier,
        template,
        hp,
        sd,
        sprite,
        color,
        item_modifiers,
        sticker,
        toggle_flags,
        // Simple items don't have these
        doc: None,
        speech: None,
        abilitydata: None,
        source: Source::Base,
    }
}

/// Parse a replica item with ability from a modifier string.
/// Format: itempool.((hat.(replica.TEMPLATE...cast.ABILITY...))).n.CONTAINER_NAME.mn.NAME
pub fn parse_with_ability(modifier: &str, _modifier_index: usize) -> ReplicaItem {
    // See parse_simple for rationale (§F10, Chunk 9). Non-depth-aware
    // because the legit scalars live inside the outer `((hat.(replica...)))`
    // wrap; `scalar_slice` trims the outer `.abilitydata.(...)` region so
    // its interior can't leak into top-level fields.
    let scalar_slice = util::slice_before_chain_and_cast(modifier);
    let name = util::extract_mn_name(modifier).unwrap_or_default();
    let sd = util::extract_sd(scalar_slice, false)
        .map(|s| crate::ir::DiceFaces::parse(&s))
        .unwrap_or_else(|| crate::ir::DiceFaces { faces: vec![] });
    let hp = util::extract_hp(scalar_slice, false);
    let color = util::extract_color(scalar_slice, false);
    let doc = util::extract_simple_prop(modifier, ".doc.");
    let speech = util::extract_simple_prop(modifier, ".speech.");
    let abilitydata = util::extract_nested_prop(modifier, ".abilitydata.")
        .map(|s| crate::ir::AbilityData::parse(&s));
    let item_modifiers = util::extract_modifier_chain(modifier)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let img_data = util::extract_img_data(scalar_slice);
    let sprite = crate::authoring::SpriteId::owned(name.clone(), img_data.unwrap_or_default());

    // Container name: the outer .n. (last one at depth 0, outside replica)
    let container_name = extract_outer_n_name(modifier).unwrap_or_default();

    ReplicaItem {
        name,
        container: ReplicaItemContainer::Capture { name: container_name },
        template: util::extract_template(modifier).unwrap_or_default(),
        hp,
        sd,
        sprite,
        color,
        doc,
        speech,
        abilitydata,
        item_modifiers,
        // With-ability items typically don't have these (but fields exist if needed)
        tier: None,
        sticker: None,
        toggle_flags: None,
        source: Source::Base,
    }
}

/// Extract the container name (first .n. at depth 0, outside replica block).
fn extract_container_name(modifier: &str) -> Option<String> {
    let pos = util::find_at_depth0(modifier, ".n.")?;
    let start = pos + 3;
    let remaining = &modifier[start..];
    let end = remaining
        .find('.')
        .or_else(|| remaining.find('+'))
        .or_else(|| remaining.find(')'))
        .unwrap_or(remaining.len());
    let name = &remaining[..end];
    if name.is_empty() { None } else { Some(name.to_string()) }
}

/// Extract the outer .n. name (last at depth 0) for items with deeper nesting.
fn extract_outer_n_name(modifier: &str) -> Option<String> {
    let pos = util::find_last_at_depth0(modifier, ".n.")?;
    let start = pos + 3;
    let remaining = &modifier[start..];
    let end = remaining
        .find(['.', '+', ')', '&', '@', ','])
        .unwrap_or(remaining.len());
    let name = &remaining[..end];
    if name.is_empty() { None } else { Some(name.to_string()) }
}

/// Parse a top-level Legendary replica item: `item.TEMPLATE.[props].n.NAME[.cast.ABILITY]`.
///
/// Legendaries carry no container name; `ReplicaItemContainer::Legendary` makes
/// that unrepresentable at the type level.
pub fn parse_legendary(modifier: &str, _modifier_index: usize) -> ReplicaItem {
    // Strip the leading `item.` (case-insensitive). The classifier's
    // `ModifierType::Legendary` gate (extractor/classifier.rs) only routes
    // top-level `item.*` here; if that invariant is ever broken, fail loud
    // rather than silently parsing garbage as a Legendary body.
    let body = modifier
        .get(5..)
        .filter(|_| modifier.len() >= 5 && modifier[..5].eq_ignore_ascii_case("item."))
        .expect(
            "parse_legendary invoked with a modifier that does not start with `item.` — \
             classifier invariant broken (see extractor/classifier.rs)",
        );
    // Scalar fields (hp/sd/color/img) must only be scanned in the top-level
    // prefix that precedes any chain (`.i.` / `.sticker.`) or `.cast.` block.
    // Chain sub-entries carry free-form `.sidesc.` / `.enchant.` text and
    // cast-effect bodies carry `.hp.` / `.col.` / `.sd.` / `.img.` at
    // cast-interior depth — neither may leak into top-level fields. The
    // `slice_before_chain_and_cast` helper is strictly broader than the
    // pre-cast slice Chunk 6 used here: it trims at the earliest of
    // `.i.` / `.sticker.` / `.cast.`. (§F10, Chunk 9.)
    let scalar_slice = util::slice_before_chain_and_cast(body);
    // Chain / name extraction scope to the pre-cast region (Chunk 6 hardening
    // — preserved): emission places the chain at the head of the body
    // (before sd/name/cast), and the character `.n.NAME` lands AFTER the
    // chain and BEFORE `.cast.`. The LAST `.n.` at depth 0 in the pre-cast
    // region is therefore the character name — chain-internal `.n.` tokens
    // (e.g. `.i.hat.statue.n.Viscera`) appear earlier and must not win.
    let before_cast = util::find_at_depth0(body, ".cast.")
        .map(|pos| &body[..pos])
        .unwrap_or(body);
    let name = util::find_last_at_depth0(before_cast, ".n.")
        .map(|pos| {
            let start = pos + 3;
            let remaining = &before_cast[start..];
            let end = remaining
                .find(['.', '+', ')', '&', '@', ','])
                .unwrap_or(remaining.len());
            before_cast[start..start + end].to_string()
        })
        .unwrap_or_default();
    let sd = util::extract_sd(scalar_slice, true)
        .map(|s| crate::ir::DiceFaces::parse(&s))
        .unwrap_or_else(|| crate::ir::DiceFaces { faces: vec![] });
    let template = body
        .find('.')
        .map(|i| body[..i].to_string())
        .unwrap_or_else(|| body.to_string());
    let hp = util::extract_hp(scalar_slice, true);
    let color = util::extract_color(scalar_slice, true);
    let doc = util::extract_simple_prop(body, ".doc.");
    let speech = util::extract_simple_prop(body, ".speech.");
    // Cast extraction intentionally scans the full `body` — it needs to
    // see the `.cast.` region the slice excludes.
    let abilitydata = util::extract_nested_prop(body, ".cast.")
        .map(|s| crate::ir::AbilityData::parse(&s));
    // Chain scope mirrors `before_cast` (not `scalar_slice`): a false
    // `.i.` / `.sticker.` later in `body` (e.g. a stray `.i.` inside a
    // `.speech.` / `.doc.` value leftover after simple-prop extraction)
    // cannot legitimately be a chain segment of this Legendary.
    let item_modifiers = util::extract_modifier_chain(before_cast)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let img_data = util::extract_img_data(scalar_slice);
    let sprite = crate::authoring::SpriteId::owned(name.clone(), img_data.unwrap_or_default());

    ReplicaItem {
        name,
        container: ReplicaItemContainer::Legendary,
        template,
        hp,
        sd,
        sprite,
        color,
        doc,
        speech,
        abilitydata,
        item_modifiers,
        tier: None,
        sticker: None,
        toggle_flags: None,
        source: Source::Base,
    }
}

#[cfg(test)]
mod tests {
    //! Chunk 6 — source-vs-IR parse tests.
    //!
    //! These tests pin extraction to read from the *source bytes*, not a
    //! derived / canonical / registry lookup. A regression that reached into
    //! (say) a ball-name registry instead of the raw modifier would pass an
    //! IR-vs-IR roundtrip and still be silently wrong — these tests catch
    //! that by using an invented ball name no registry would produce.
    use super::*;
    use crate::ir::ReplicaItemContainer;

    #[test]
    fn classifies_capture_into_enum_with_container_name_from_source() {
        // Minimal sliceymon-shaped capture. The outer `.n.Quux` is an
        // invented ball name on purpose — if the extractor reached for a
        // registry of "known balls", this would come back wrong.
        let modifier = "itempool.((hat.replica.Thief.n.Zoroark.sd.0:0:0:0:0:0)).n.Quux.tier.3.mn.Zoroark";
        let item = parse_simple(modifier, 0);
        assert_eq!(item.name, "Zoroark");
        assert_eq!(
            item.container,
            ReplicaItemContainer::Capture { name: "Quux".into() },
            "container name must come from the source `.n.` bytes, not a registry lookup"
        );
    }

    #[test]
    fn classifies_capture_with_ability_into_enum() {
        // `.abilitydata.` at depth 0 is the parser's actual marker — cast.
        // lives inside the inner replica paren group. `ZZZ` as the outer
        // `.n.` is invented so a registry-lookup regression would fail here.
        let modifier = "itempool.((hat.(replica.Alpha.sd.0:0:0:0:0:0.n.Mewtwo))).abilitydata.(Fey.sd.0:0:0:0:0:0.n.Psy).n.ZZZ.mn.Mewtwo";
        let item = parse_with_ability(modifier, 0);
        assert_eq!(item.name, "Mewtwo");
        assert_eq!(
            item.container,
            ReplicaItemContainer::Capture { name: "ZZZ".into() },
            "with-ability path must still produce a Capture, carrying the outer .n. source bytes"
        );
        assert!(item.abilitydata.is_some());
    }

    #[test]
    fn classifies_legendary_into_enum() {
        let modifier = "item.Alpha.sd.0:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0);
        assert_eq!(item.name, "Mew");
        assert_eq!(item.container, ReplicaItemContainer::Legendary);
        assert_eq!(item.template, "Alpha");
    }

    #[test]
    fn legendary_name_is_last_depth0_n_before_cast() {
        // A `.i.` chain segment can carry a depth-0 `.n.NAME` (e.g. an item
        // with a named reference). Emission always places the character
        // `.n.NAME` *after* the chain and *before* `.cast.`, so the LAST
        // pre-cast depth-0 `.n.` is the character name. First-match would
        // incorrectly pick the chain's internal `.n.`.
        let modifier = "item.Alpha.hp.9.i.hat.statue.n.Viscera.sd.0:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0);
        assert_eq!(item.name, "Mew", "character name must be the last pre-cast `.n.`, not a chain-internal `.n.`");
    }

    // ---------------------------------------------------------------------
    // Chunk 9 / §F10 — chain-interior scalar leakage tests.
    //
    // Every test below is source-vs-IR divergent by construction: the
    // modifier's chain / cast region carries a literal `.hp.` / `.col.` /
    // `.sd.` / `.img.` substring whose byte interpretation would flip the
    // corresponding top-level IR field if the parser reached for bytes
    // beyond `slice_before_chain_and_cast`. An IR-vs-IR roundtrip baseline
    // would not catch this class — both extract passes would resolve the
    // leak the same way. These tests fail only when the parser walks into
    // chain / cast bytes it shouldn't.
    // ---------------------------------------------------------------------

    #[test]
    fn legendary_hp_ignores_chain_interior_sidesc() {
        // `.sticker.sidesc.hp.99` is free-form chain text; only the
        // top-level (pre-chain, pre-cast) region may supply `.hp.`. No
        // top-level `.hp.` → `None`. A non-depth-aware `extract_hp` would
        // wrongly return `Some(99)`.
        let modifier = "item.Alpha.sticker.sidesc.hp.99.sd.0:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0);
        assert_eq!(item.hp, None, "chain-interior .hp. must not leak into top-level hp");
    }

    #[test]
    fn legendary_color_ignores_chain_interior_sidesc() {
        let modifier = "item.Alpha.sticker.sidesc.col.z.sd.0:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0);
        assert_eq!(item.color, None, "chain-interior .col. must not leak into top-level color");
    }

    #[test]
    fn legendary_sd_ignores_chain_interior_sidesc() {
        // No top-level `.sd.`; the chain's sidesc text carries a decoy
        // `.sd.999-1:0:0:0:0:0`. With `scalar_slice` applied, the slice
        // ends at the depth-0 `.sticker.` so the decoy never enters the
        // scan — expected `sd` is the empty `DiceFaces`. A non-depth-aware
        // first-match on the full body would return the decoy and fail
        // this assertion (source-vs-IR divergence, per chunk-impl rule 2).
        let modifier = "item.Alpha.sticker.sidesc.sd.999-1:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0);
        assert_eq!(
            item.sd,
            crate::ir::DiceFaces { faces: vec![] },
            "chain-interior decoy .sd. must not leak into top-level sd when no top-level .sd. exists",
        );
    }

    #[test]
    fn legendary_img_ignores_chain_interior_sidesc() {
        // No top-level `.img.`; the chain's sidesc text carries
        // `.img.BADIMG`. A non-depth-aware scan (or one that walks past
        // the chain start) would produce a populated sprite. The helper
        // must trim the chain region so img_data stays empty.
        let modifier = "item.Alpha.sticker.sidesc.img.BADIMG.sd.0:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0);
        assert_eq!(
            item.sprite.img_data(),
            "",
            "chain-interior .img. must not leak into top-level sprite img_data",
        );
    }

    #[test]
    fn capture_hp_ignores_chain_interior_sidesc() {
        // Capture path (parse_simple): no top-level `.hp.`; the chain's
        // sticker sidesc text — wrapped in parens, so the decoy sits at
        // depth ≥ 2 — carries `.hp.5`. A non-depth-aware or non-sliced
        // scan would return Some(5).
        let modifier = "itempool.((hat.replica.Thief.n.Pika.sd.0:0:0:0:0:0)).sticker.(sidesc.(.hp.5)).n.Quux.tier.3.mn.Pika";
        let item = parse_simple(modifier, 0);
        assert_eq!(item.hp, None, "chain-interior .hp.5 (depth ≥ 2) must not leak into top-level hp");
    }

    #[test]
    fn capture_color_ignores_cast_interior() {
        // Capture-with-ability path (parse_with_ability): no top-level
        // `.col.`; the `.abilitydata.(...)` cast effect carries `.col.x`
        // at depth ≥ 2. `scalar_slice` closes this leak: because
        // `slice_before_chain_and_cast` lists `.abilitydata.` as a
        // depth-0 trim marker (util.rs), the slice ends before the cast
        // effect and the scan never sees the decoy. `depth_aware = false`
        // is the correct flag here because legit Capture scalars live at
        // depth ≥ 2 inside `((hat.(replica...)))`; `depth_aware = true`
        // would skip them.
        let modifier = "itempool.((hat.(replica.Alpha.sd.0:0:0:0:0:0.n.Mewtwo))).abilitydata.(Fey.sd.0:0:0:0:0:0.(.col.x.hp.9).n.Psy).n.ZZZ.mn.Mewtwo";
        let item = parse_with_ability(modifier, 0);
        assert_eq!(item.color, None, "cast-interior .col.x at depth ≥ 2 must not leak into top-level color");
        assert_eq!(item.hp, None, "cast-interior .hp.9 at depth ≥ 2 must not leak into top-level hp");
    }

    #[test]
    fn legendary_ignores_cast_interior_hp_color_sd_img() {
        // Top-level Legendary declares NO hp / color / img / sd. Its cast
        // block carries `.hp.`, `.col.`, `.sd.`, `.img.` at cast-interior
        // depth. A naive non-depth-aware scan (`content.find(".hp.")`) would
        // pull those cast-interior values up into the top-level fields —
        // silently flipping `None`/empty into `Some(...)` at parse time.
        // Guards against that leakage.
        let modifier = "item.Alpha.n.Mew.cast.(Spell.sd.170-1:0:0:0:0:0.col.a.hp.5.img.bas99:9.n.Psy)";
        let item = parse_legendary(modifier, 0);
        assert_eq!(item.name, "Mew");
        assert_eq!(item.template, "Alpha");
        assert_eq!(item.hp, None, "cast-interior .hp. must not leak into top-level hp");
        assert_eq!(item.color, None, "cast-interior .col. must not leak into top-level color");
        assert_eq!(
            item.sd,
            crate::ir::DiceFaces { faces: vec![] },
            "cast-interior .sd. must not leak into top-level sd",
        );
        assert_eq!(
            item.sprite.img_data(),
            "",
            "cast-interior .img. must not leak into top-level sprite img_data",
        );
        assert!(item.abilitydata.is_some(), "cast block still parses into abilitydata");
    }
}

/// Extract #tog toggle flags from modifier.
fn extract_toggle_flags(modifier: &str) -> Option<String> {
    let mut flags = Vec::new();
    let mut search_from = 0;
    while let Some(pos) = modifier[search_from..].find("#tog") {
        let abs_pos = search_from + pos;
        let remaining = &modifier[abs_pos..];
        let end = remaining[1..].find(['#', '.', ',', ')'])
            .map(|e| e + 1)
            .unwrap_or(remaining.len());
        flags.push(remaining[..end].to_string());
        search_from = abs_pos + end;
    }
    if flags.is_empty() { None } else { Some(flags.join("")) }
}
