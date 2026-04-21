use crate::ir::{ReplicaItem, ReplicaItemContainer, Source};
use crate::util;

/// Parse a simple replica item (no ability) from a modifier string.
/// Format: itempool.((hat.replica.TEMPLATE...)).n.CONTAINER_NAME.mn.NAME
pub fn parse_simple(modifier: &str, _modifier_index: usize) -> ReplicaItem {
    let name = util::extract_mn_name(modifier).unwrap_or_default();
    let sd = util::extract_sd(modifier, false)
        .map(|s| crate::ir::DiceFaces::parse(&s))
        .unwrap_or_else(|| crate::ir::DiceFaces { faces: vec![] });
    let template = util::extract_template(modifier).unwrap_or_default();
    let hp = util::extract_hp(modifier, false);
    let color = util::extract_color(modifier);
    let tier = util::extract_simple_prop(modifier, ".tier.")
        .and_then(|v| v.parse::<u8>().ok());
    let item_modifiers = util::extract_modifier_chain(modifier)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let sticker = util::extract_simple_prop(modifier, ".sticker.");
    let toggle_flags = extract_toggle_flags(modifier);
    let img_data = util::extract_img_data(modifier);
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
    let name = util::extract_mn_name(modifier).unwrap_or_default();
    let sd = util::extract_sd(modifier, false)
        .map(|s| crate::ir::DiceFaces::parse(&s))
        .unwrap_or_else(|| crate::ir::DiceFaces { faces: vec![] });
    let hp = util::extract_hp(modifier, false);
    let color = util::extract_color(modifier);
    let doc = util::extract_simple_prop(modifier, ".doc.");
    let speech = util::extract_simple_prop(modifier, ".speech.");
    let abilitydata = util::extract_nested_prop(modifier, ".abilitydata.")
        .map(|s| crate::ir::AbilityData::parse(&s));
    let item_modifiers = util::extract_modifier_chain(modifier)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let img_data = util::extract_img_data(modifier);
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
    // Strip the leading `item.` (case-insensitive).
    let body = modifier
        .get(5..)
        .filter(|_| modifier.len() >= 5 && modifier[..5].eq_ignore_ascii_case("item."))
        .unwrap_or(modifier);
    // `n.NAME` at depth 0 is the character name for Legendary (no outer
    // container wraps it, so there is only one `.n.`).
    let name = util::find_at_depth0(body, ".n.")
        .map(|pos| {
            let start = pos + 3;
            let remaining = &body[start..];
            let end = remaining
                .find(['.', '+', ')', '&', '@', ','])
                .unwrap_or(remaining.len());
            body[start..start + end].to_string()
        })
        .unwrap_or_default();
    let sd = util::extract_sd(body, false)
        .map(|s| crate::ir::DiceFaces::parse(&s))
        .unwrap_or_else(|| crate::ir::DiceFaces { faces: vec![] });
    let template = body
        .find('.')
        .map(|i| body[..i].to_string())
        .unwrap_or_else(|| body.to_string());
    let hp = util::extract_hp(body, false);
    let color = util::extract_color(body);
    let doc = util::extract_simple_prop(body, ".doc.");
    let speech = util::extract_simple_prop(body, ".speech.");
    let abilitydata = util::extract_nested_prop(body, ".cast.")
        .map(|s| crate::ir::AbilityData::parse(&s));
    let item_modifiers = util::extract_modifier_chain(body)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let img_data = util::extract_img_data(body);
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
