use crate::ir::{Capture, Legendary};
use crate::util;

pub fn parse_capture(modifier: &str, _modifier_index: usize) -> Capture {
    let pokemon = util::extract_mn_name(modifier).unwrap_or_default();
    let sd = util::extract_sd(modifier, false).unwrap_or_default();
    let template = util::extract_template(modifier).unwrap_or_default();
    let hp = util::extract_hp(modifier, false);
    let color = util::extract_color(modifier);
    let sprite_name = pokemon.clone();
    let ball_tier = util::extract_simple_prop(modifier, ".tier.")
        .and_then(|v| v.parse::<u8>().ok());
    let item_modifiers = util::extract_modifier_chain(modifier);
    let sticker = util::extract_simple_prop(modifier, ".sticker.");
    let toggle_flags = extract_toggle_flags(modifier);

    Capture {
        pokemon,
        ball_name: extract_ball_name(modifier).unwrap_or_default(),
        ball_tier,
        template,
        hp,
        sd,
        sprite_name,
        color,
        item_modifiers,
        sticker,
        toggle_flags,
        raw: Some(modifier.to_string()),
    }
}

pub fn parse_legendary(modifier: &str, _modifier_index: usize) -> Legendary {
    let pokemon = util::extract_mn_name(modifier).unwrap_or_default();
    let sd = util::extract_sd(modifier, false).unwrap_or_default();
    let hp = util::extract_hp(modifier, false);
    let color = util::extract_color(modifier);
    let sprite_name = pokemon.clone();
    let doc = util::extract_simple_prop(modifier, ".doc.");
    let speech = util::extract_simple_prop(modifier, ".speech.");
    let abilitydata = util::extract_nested_prop(modifier, ".abilitydata.");
    let item_modifiers = util::extract_modifier_chain(modifier);

    // Extract summoning item name: the outer .n. (first one, outside replica)
    let summoning_item = extract_outer_n_name(modifier).unwrap_or_default();

    Legendary {
        pokemon,
        summoning_item,
        template: util::extract_template(modifier).unwrap_or_default(),
        hp,
        sd,
        sprite_name,
        color,
        doc,
        speech,
        abilitydata,
        item_modifiers,
        raw: Some(modifier.to_string()),
    }
}

fn extract_ball_name(modifier: &str) -> Option<String> {
    // Ball name is the first .n. at depth 0 (outside replica block)
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

/// Extract the outer .n. name for legendaries (summoning item name).
/// This is the .n. outside the deepest replica block, typically in the item wrapper.
fn extract_outer_n_name(modifier: &str) -> Option<String> {
    // Find the last .n. at depth 0
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
        // Find end of flag (next # or . or end)
        let remaining = &modifier[abs_pos..];
        let end = remaining[1..].find(['#', '.', ',', ')'])
            .map(|e| e + 1)
            .unwrap_or(remaining.len());
        flags.push(remaining[..end].to_string());
        search_from = abs_pos + end;
    }
    if flags.is_empty() { None } else { Some(flags.join("")) }
}
