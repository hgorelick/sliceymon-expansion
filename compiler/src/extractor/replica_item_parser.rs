use crate::error::CompilerError;
use crate::ir::{ReplicaItem, Source};
use crate::util;

/// Parse a top-level Legendary replica item: `item.TEMPLATE.[props].n.NAME[.abilitydata.ABILITY]`.
///
/// Legendaries are the only replica shape the classifier produces; Capture-shaped
/// items route as `ItemPool` structurals because no corpus instance has ever
/// been observed in the 4 working mods (chunk-impl rule 3).
///
/// Returns a structured error rather than panicking if the classifier ever
/// routes a non-`item.*` modifier here — the caller in `extractor/mod.rs`
/// propagates it through `?`.
pub fn parse_legendary(modifier: &str, modifier_index: usize) -> Result<ReplicaItem, CompilerError> {
    // Strip the leading `item.` (case-insensitive). The classifier's
    // `ModifierType::Legendary` gate (extractor/classifier.rs) only routes
    // top-level `item.*` here; if that invariant is ever broken, surface a
    // structured error rather than panicking.
    let Some(body) = modifier
        .get(5..)
        .filter(|_| modifier.len() >= 5 && modifier[..5].eq_ignore_ascii_case("item."))
    else {
        return Err(CompilerError::build(
            "replica_item_parser::parse_legendary",
            format!(
                "expected modifier to start with `item.` (case-insensitive), got: {:.40}",
                modifier
            ),
        )
        .with_field_path(format!("replica_items[{}]", modifier_index))
        .with_suggestion(
            "classifier::ModifierType::Legendary must only route modifiers beginning with `item.`",
        ));
    };
    // Scalar fields (hp/sd/color/img) must only be scanned in the top-level
    // prefix that precedes any chain (`.i.` / `.sticker.`) or ability block.
    // Chain sub-entries carry free-form `.sidesc.` / `.enchant.` text and
    // ability-effect bodies carry `.hp.` / `.col.` / `.sd.` / `.img.` at
    // ability-interior depth — neither may leak into top-level fields. The
    // `slice_before_chain_and_cast` helper trims at the earliest of
    // `.i.` / `.sticker.` / `.abilitydata.` (§F10-MARKERS, Chunk 9). The
    // property marker for an ability body is `.abilitydata.` per the
    // textmod guide (reference/textmod_guide.md lines 747 / 857 / 975-981);
    // `cast.TRIGGER` in the corpus is a chain keyword (guide lines 642-645),
    // not a property marker.
    let scalar_slice = util::slice_before_chain_and_cast(body);
    // Chain / name extraction scope to the pre-ability region (Chunk 6
    // hardening — preserved, renamed from `before_cast`): emission places
    // the chain at the head of the body (before sd/name/abilitydata), and
    // the character `.n.NAME` lands AFTER the chain and BEFORE
    // `.abilitydata.`. The LAST `.n.` at depth 0 in the pre-ability region
    // is therefore the character name — chain-internal `.n.` tokens (e.g.
    // `.i.hat.statue.n.Viscera`) appear earlier and must not win.
    let before_ability = util::find_at_depth0(body, ".abilitydata.")
        .map(|pos| &body[..pos])
        .unwrap_or(body);
    let name = util::find_last_at_depth0(before_ability, ".n.")
        .map(|pos| {
            let start = pos + 3;
            let remaining = &before_ability[start..];
            let end = remaining
                .find(['.', '+', ')', '&', '@', ','])
                .unwrap_or(remaining.len());
            before_ability[start..start + end].to_string()
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
    // Ability extraction intentionally scans the full `body` — it needs to
    // see the `.abilitydata.` region the slice excludes.
    let abilitydata = util::extract_nested_prop(body, ".abilitydata.")
        .map(|s| crate::ir::AbilityData::parse(&s));
    // Chain scope mirrors `before_ability` (not `scalar_slice`): a false
    // `.i.` / `.sticker.` later in `body` (e.g. a stray `.i.` inside a
    // `.speech.` / `.doc.` value leftover after simple-prop extraction)
    // cannot legitimately be a chain segment of this Legendary.
    let item_modifiers = util::extract_modifier_chain(before_ability)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let img_data = util::extract_img_data(scalar_slice);
    let sprite = crate::authoring::SpriteId::owned(name.clone(), img_data.unwrap_or_default());

    Ok(ReplicaItem {
        name,
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
    })
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

    #[test]
    fn parses_legendary_from_top_level_item() {
        let modifier = "item.Alpha.sd.0:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0).expect("valid legendary parses");
        assert_eq!(item.name, "Mew");
        assert_eq!(item.template, "Alpha");
    }

    #[test]
    fn legendary_name_is_last_depth0_n_before_ability() {
        // A `.i.` chain segment can carry a depth-0 `.n.NAME` (e.g. an item
        // with a named reference). Emission always places the character
        // `.n.NAME` *after* the chain and *before* `.abilitydata.`, so the
        // LAST pre-ability depth-0 `.n.` is the character name. First-match
        // would incorrectly pick the chain's internal `.n.`.
        let modifier = "item.Alpha.hp.9.i.hat.statue.n.Viscera.sd.0:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0).expect("valid legendary parses");
        assert_eq!(item.name, "Mew", "character name must be the last pre-ability `.n.`, not a chain-internal `.n.`");
    }

    // ---------------------------------------------------------------------
    // Chunk 9 / §F10 — chain-interior scalar leakage tests.
    //
    // Every test below is source-vs-IR divergent by construction: the
    // modifier's chain / ability region carries a literal `.hp.` / `.col.` /
    // `.sd.` / `.img.` substring whose byte interpretation would flip the
    // corresponding top-level IR field if the parser reached for bytes
    // beyond `slice_before_chain_and_cast`. An IR-vs-IR roundtrip baseline
    // would not catch this class — both extract passes would resolve the
    // leak the same way. These tests fail only when the parser walks into
    // chain / ability bytes it shouldn't.
    // ---------------------------------------------------------------------

    #[test]
    fn legendary_hp_ignores_chain_interior_sidesc() {
        // `.sticker.sidesc.hp.99` is free-form chain text; only the
        // top-level (pre-chain, pre-ability) region may supply `.hp.`. No
        // top-level `.hp.` → `None`. A non-depth-aware `extract_hp` would
        // wrongly return `Some(99)`.
        let modifier = "item.Alpha.sticker.sidesc.hp.99.sd.0:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0).expect("valid legendary parses");
        assert_eq!(item.hp, None, "chain-interior .hp. must not leak into top-level hp");
    }

    #[test]
    fn legendary_color_ignores_chain_interior_sidesc() {
        let modifier = "item.Alpha.sticker.sidesc.col.z.sd.0:0:0:0:0:0.n.Mew";
        let item = parse_legendary(modifier, 0).expect("valid legendary parses");
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
        let item = parse_legendary(modifier, 0).expect("valid legendary parses");
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
        let item = parse_legendary(modifier, 0).expect("valid legendary parses");
        assert_eq!(
            item.sprite.img_data(),
            "",
            "chain-interior .img. must not leak into top-level sprite img_data",
        );
    }

    #[test]
    fn legendary_ignores_abilitydata_interior_hp_color_sd_img() {
        // Top-level Legendary declares NO hp / color / img / sd. Its
        // `.abilitydata.(body)` carries `.hp.`, `.col.`, `.sd.`, `.img.` at
        // ability-interior depth. A naive non-depth-aware scan
        // (`content.find(".hp.")`) would pull those ability-interior values
        // up into the top-level fields — silently flipping `None`/empty
        // into `Some(...)` at parse time. Guards against that leakage.
        let modifier = "item.Alpha.n.Mew.abilitydata.(Spell.sd.170-1:0:0:0:0:0.col.a.hp.5.img.bas99:9.n.Psy)";
        let item = parse_legendary(modifier, 0).expect("valid legendary parses");
        assert_eq!(item.name, "Mew");
        assert_eq!(item.template, "Alpha");
        assert_eq!(item.hp, None, "ability-interior .hp. must not leak into top-level hp");
        assert_eq!(item.color, None, "ability-interior .col. must not leak into top-level color");
        assert_eq!(
            item.sd,
            crate::ir::DiceFaces { faces: vec![] },
            "ability-interior .sd. must not leak into top-level sd",
        );
        assert_eq!(
            item.sprite.img_data(),
            "",
            "ability-interior .img. must not leak into top-level sprite img_data",
        );
        assert!(item.abilitydata.is_some(), "ability body still parses into abilitydata");
    }

    #[test]
    fn legendary_without_item_prefix_propagates_error() {
        // Classifier invariant: `ModifierType::Legendary` is only produced for
        // top-level `item.*` modifiers. If that invariant is ever broken, the
        // parser must surface a structured error — not panic, not silently
        // parse the wrong shape. Source-vs-IR: the returned error carries the
        // *source* modifier-index, so a regression that derived the index from
        // some other state (e.g. the position of the last-successfully-parsed
        // item) would fail this assertion.
        let modifier = "boss.AAA.hp.100.n.Nope";
        let err = parse_legendary(modifier, 42).expect_err("non-`item.` prefix must not parse");
        match err.kind.as_ref() {
            crate::error::ErrorKind::Build { component, .. } => {
                assert_eq!(component, "replica_item_parser::parse_legendary");
            }
            other => panic!("expected Build error, got {:?}", other),
        }
        assert_eq!(err.field_path.as_deref(), Some("replica_items[42]"));
        assert!(err.suggestion.is_some(), "classifier-invariant hint must be present");
    }
}
