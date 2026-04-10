use crate::ir::{Monster, Source};
use crate::util;

pub fn parse_monster(modifier: &str, _modifier_index: usize) -> Monster {
    let name = util::extract_mn_name(modifier)
        .or_else(|| util::extract_last_n_name(modifier))
        .unwrap_or_default();
    let base_template = extract_base_template(modifier).unwrap_or_default();
    let floor_range = extract_floor_range(modifier).unwrap_or_default();
    let color = util::extract_color(modifier);
    let doc = util::extract_simple_prop(modifier, ".doc.");
    let modifier_chain = util::extract_modifier_chain(modifier)
        .map(|s| crate::ir::ModifierChain::parse(&s));

    // sprite_name: use name as key for sprite lookup
    let sprite_name = if name.is_empty() { None } else { Some(name.clone()) };
    let img_data = util::extract_img_data(modifier);

    Monster {
        name,
        base_template,
        floor_range,
        hp: util::extract_hp(modifier, false),
        sd: util::extract_sd(modifier, false).map(|s| crate::ir::DiceFaces::parse(&s)),
        sprite_name,
        color,
        doc,
        modifier_chain,
        balance: extract_balance(modifier),
        img_data,
        source: Source::Base,
    }
}

fn extract_base_template(modifier: &str) -> Option<String> {
    let lower = modifier.to_lowercase();
    let pos = lower.find("monsterpool.")?;
    let after = &modifier[pos + "monsterpool.".len()..];
    let trimmed = after.trim_start_matches('(');
    let content = trimmed.strip_prefix("replica.").unwrap_or(trimmed);
    let end = content.find('.').unwrap_or(content.len().min(30));
    let template = &content[..end];
    if template.is_empty() { None } else { Some(template.to_string()) }
}

fn extract_floor_range(modifier: &str) -> Option<String> {
    let content = modifier.trim_start_matches('(');
    let bytes = content.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
    if i == 0 || i >= bytes.len() || bytes[i] != b'-' { return None; }
    i += 1;
    let dash_start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
    if i == dash_start { return None; }
    let range = &content[..i];
    if content[i..].contains("monsterpool.") { Some(range.to_string()) } else { None }
}

fn extract_balance(modifier: &str) -> Option<String> {
    let marker = ".bal.";
    let pos = modifier.find(marker)?;
    let start = pos + marker.len();
    let remaining = &modifier[start..];
    let dot_end = remaining.find('.').unwrap_or(remaining.len());
    let paren_end = remaining.find(')').unwrap_or(remaining.len());
    let end = dot_end.min(paren_end);
    let val = &remaining[..end];
    if val.is_empty() { None } else { Some(val.to_string()) }
}
