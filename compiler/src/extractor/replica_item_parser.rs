use crate::ir::{ReplicaItem, Source};
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
    let sprite = crate::authoring::SpriteId::lookup(&name)
        .cloned()
        .unwrap_or_else(|| {
            crate::authoring::SpriteId::owned(name.clone(), img_data.unwrap_or_default())
        });

    ReplicaItem {
        name,
        container_name: extract_container_name(modifier).unwrap_or_default(),
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
    let sprite = crate::authoring::SpriteId::lookup(&name)
        .cloned()
        .unwrap_or_else(|| {
            crate::authoring::SpriteId::owned(name.clone(), img_data.unwrap_or_default())
        });

    // Container name: the outer .n. (last one at depth 0, outside replica)
    let container_name = extract_outer_n_name(modifier).unwrap_or_default();

    ReplicaItem {
        name,
        container_name,
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
