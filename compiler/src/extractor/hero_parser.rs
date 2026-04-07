use crate::error::CompilerError;
use crate::ir::{Hero, HeroBlock, HeroFormat};
use crate::util;
use crate::extractor::classifier;

/// Parse a hero modifier string into a Hero struct.
/// Falls back to raw passthrough if parsing fails.
pub fn parse_hero(modifier: &str, modifier_index: usize) -> Hero {
    let format = classifier::detect_hero_format(modifier);
    match format {
        HeroFormat::Sliceymon => parse_sliceymon(modifier, modifier_index),
        HeroFormat::Grouped | HeroFormat::Unknown => parse_grouped(modifier, modifier_index),
    }
}

/// Parse a Sliceymon-format hero (ph.b prefix, !mheropool.).
fn parse_sliceymon(modifier: &str, modifier_index: usize) -> Hero {
    match try_parse_sliceymon(modifier, modifier_index) {
        Ok(hero) => hero,
        Err(_e) => {
            let internal_name = extract_internal_name(modifier).unwrap_or_default();
            let mn_name = util::extract_mn_name(modifier)
                .or_else(|| util::extract_last_n_name(modifier))
                .unwrap_or_else(|| internal_name.clone());
            Hero {
                internal_name,
                mn_name,
                color: '?',
                format: HeroFormat::Sliceymon,
                blocks: vec![],
                removed: false,
                raw: Some(modifier.to_string()),
            }
        }
    }
}

/// Parse a grouped-format hero (pansaer/punpuns/community: heropool.Name+...replica blocks).
fn parse_grouped(modifier: &str, _modifier_index: usize) -> Hero {
    let internal_name = extract_internal_name(modifier)
        .or_else(|| extract_grouped_name(modifier))
        .unwrap_or_default();
    let mn_name = util::extract_mn_name(modifier)
        .or_else(|| util::extract_last_n_name(modifier))
        .unwrap_or_else(|| {
            let mut s = internal_name.clone();
            if let Some(c) = s.get_mut(0..1) {
                c.make_ascii_uppercase();
            }
            s
        });

    let color = util::extract_color(modifier).unwrap_or('?');

    // Find heropool content
    let hp_marker = find_heropool_marker(modifier);
    let blocks = if let Some(hp_pos) = hp_marker {
        let content_start = hp_pos + "heropool.".len();
        let heropool_content = &modifier[content_start..];
        let block_strs = util::split_at_depth0(heropool_content, '+');

        let mut parsed_blocks = Vec::new();
        for bs in &block_strs {
            let bs = bs.trim();
            if bs.is_empty() {
                continue;
            }
            // Each block can be a bare name or a (replica.Template...).n.Name
            if let Some(block) = try_parse_grouped_block(bs) {
                parsed_blocks.push(block);
            }
        }
        parsed_blocks
    } else {
        vec![]
    };

    Hero {
        internal_name,
        mn_name,
        color,
        format: HeroFormat::Grouped,
        blocks,
        removed: false,
        raw: Some(modifier.to_string()),
    }
}

/// Try to parse a single block from a grouped hero format.
fn try_parse_grouped_block(block: &str) -> Option<HeroBlock> {
    // Blocks are either:
    // 1. (replica.Template.col.X.hp.N.sd.FACES.img.DATA).speech.X.n.Name
    // 2. Plain name reference (no parens) — skip these
    if !block.contains('(') {
        return None;
    }

    let open_pos = block.find('(')?;
    let close_pos = util::find_matching_close_paren(block, open_pos)?;

    let replica_content = &block[open_pos + 1..close_pos];
    let outside_content = &block[close_pos + 1..];

    let template = replica_content
        .strip_prefix("replica.")
        .and_then(|r| r.find('.').map(|end| r[..end].to_string()))
        .unwrap_or_default();

    let tier = extract_tier_number(replica_content);
    let hp = util::extract_hp(replica_content, true);
    let sd = util::extract_sd(replica_content, true);
    let block_color = util::extract_color(replica_content);

    let abilitydata = util::extract_nested_prop(replica_content, ".abilitydata.")
        .or_else(|| util::extract_nested_prop(outside_content, ".abilitydata."));
    let triggerhpdata = util::extract_nested_prop(replica_content, ".triggerhpdata.")
        .or_else(|| util::extract_nested_prop(outside_content, ".triggerhpdata."));
    let hue = util::extract_simple_prop(replica_content, ".hue.")
        .or_else(|| util::extract_simple_prop(outside_content, ".hue."));

    let modifier_chain = util::extract_modifier_chain(replica_content);
    let facades = util::extract_facades_from_chain(modifier_chain.as_deref().unwrap_or(""));
    let items_inside: Option<String> = None;

    let speech = util::extract_simple_prop(outside_content, ".speech.");
    let name = extract_display_name(outside_content);
    let doc = util::extract_simple_prop(outside_content, ".doc.");
    let items_outside = extract_items_outside(outside_content);

    let sprite_name = name.clone();

    Some(HeroBlock {
        template,
        tier,
        hp: hp.unwrap_or(0),
        sd: sd.unwrap_or_default(),
        color: block_color,
        sprite_name,
        speech: speech.unwrap_or_default(),
        name,
        doc,
        abilitydata,
        triggerhpdata,
        hue,
        modifier_chain,
        facades,
        items_inside,
        items_outside,
    })
}

/// Extract the hero name from a grouped format (heropool.Name+...).
fn extract_grouped_name(modifier: &str) -> Option<String> {
    // Try .mn. first (most reliable)
    if let Some(mn) = util::extract_mn_name(modifier) {
        return Some(mn.to_lowercase());
    }
    // Try extracting from heropool. prefix
    if let Some(hp_pos) = find_heropool_marker(modifier) {
        let start = hp_pos + "heropool.".len();
        let remaining = &modifier[start..];
        // Take content up to first . or + or ( at depth 0
        let end = remaining.find(['.', '+', '('])
            .unwrap_or(remaining.len());
        let name = &remaining[..end];
        if !name.is_empty() {
            return Some(name.to_lowercase());
        }
    }
    util::extract_last_n_name(modifier).map(|n| n.to_lowercase())
}

/// Extract internal_name from prefix: `ph.b[name];` or lowercase of .mn./.n. value.
fn extract_internal_name(modifier: &str) -> Option<String> {
    if let Some(phb) = modifier.find("ph.b") {
        let start = phb + 4;
        if let Some(semi) = modifier[start..].find(';') {
            let name = &modifier[start..start + semi];
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }
    if let Some(mn) = util::extract_mn_name(modifier) {
        return Some(mn.to_lowercase());
    }
    util::extract_last_n_name(modifier).map(|n| n.to_lowercase())
}

fn try_parse_sliceymon(modifier: &str, modifier_index: usize) -> Result<Hero, CompilerError> {
    let internal_name = extract_internal_name(modifier).unwrap_or_default();

    let hp_marker = find_heropool_marker(modifier).ok_or_else(|| CompilerError::HeroParseError {
        modifier_index,
        hero_name: internal_name.clone(),
        tier_index: None,
        position: 0,
        expected: "heropool.".to_string(),
        found: modifier[..modifier.len().min(40)].to_string(),
    })?;

    let content_start = hp_marker + "heropool.".len();
    let heropool_content = &modifier[content_start..];

    let mut tier_strs = util::split_at_depth0(heropool_content, '+');

    if tier_strs.is_empty() {
        return Err(CompilerError::HeroParseError {
            modifier_index,
            hero_name: internal_name.clone(),
            tier_index: None,
            position: content_start,
            expected: "at least one tier block".to_string(),
            found: "empty heropool content".to_string(),
        });
    }

    // Separate suffix from last tier
    let last = tier_strs.last_mut().unwrap();
    separate_suffix(last);

    // Parse each tier block
    let mut blocks = Vec::new();
    for (i, ts) in tier_strs.iter().enumerate() {
        match parse_tier_block(ts, modifier_index, &internal_name, i) {
            Ok(block) => blocks.push(block),
            Err(e) => return Err(e),
        }
    }

    let color = util::extract_color(heropool_content).unwrap_or('?');

    let mn_name = util::extract_mn_name(modifier)
        .or_else(|| util::extract_last_n_name(modifier))
        .unwrap_or_else(|| {
            let mut s = internal_name.clone();
            if let Some(c) = s.get_mut(0..1) {
                c.make_ascii_uppercase();
            }
            s
        });

    Ok(Hero {
        internal_name,
        mn_name,
        color,
        format: HeroFormat::Sliceymon,
        blocks,
        removed: false,
        raw: Some(modifier.to_string()),
    })
}

/// Find "heropool." (case-insensitive) without allocating a lowercased copy.
fn find_heropool_marker(modifier: &str) -> Option<usize> {
    let needle = b"heropool.";
    let bytes = modifier.as_bytes();
    if bytes.len() < needle.len() {
        return None;
    }
    (0..=bytes.len() - needle.len()).find(|&i| {
        bytes[i..i + needle.len()]
            .iter()
            .zip(needle)
            .all(|(h, n)| h.to_ascii_lowercase() == *n)
    })
}

/// Separate the suffix from the last tier block string, modifying it in place.
fn separate_suffix(last_tier: &mut String) -> String {
    // Find last close-paren at depth 0
    let bytes = last_tier.as_bytes();
    let mut last_close_paren = None;
    let mut depth: i32 = 0;

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    last_close_paren = Some(i);
                }
            }
            _ => {}
        }
    }

    if let Some(lcp) = last_close_paren {
        let after_parens = &last_tier[lcp + 1..];
        if let Some(n_pos) = util::find_last_at_depth0(after_parens, ".n.") {
            let abs_n_pos = lcp + 1 + n_pos;
            let after_n = &last_tier[abs_n_pos + 3..];

            let mut name_end = after_n.len();
            for marker in &[".part.", ".mn.", "&"] {
                if let Some(pos) = after_n.find(marker) {
                    if pos < name_end {
                        name_end = pos;
                    }
                }
            }

            let suffix_start = abs_n_pos + 3 + name_end;
            if suffix_start < last_tier.len() {
                let suffix = last_tier[suffix_start..].to_string();
                last_tier.truncate(suffix_start);
                return suffix;
            }
        }
    } else {
        for marker in &[".part.", ".mn."] {
            if let Some(pos) = util::find_last_at_depth0(last_tier, marker) {
                if last_tier[..pos].contains(".n.") {
                    let suffix = last_tier[pos..].to_string();
                    last_tier.truncate(pos);
                    return suffix;
                }
            }
        }
        if last_tier.ends_with("&hidden") {
            let suffix = "&hidden".to_string();
            last_tier.truncate(last_tier.len() - 7);
            return suffix;
        }
    }

    String::new()
}

/// Parse a single tier block: `(replica.Template.props...).speech.X.n.Name`
fn parse_tier_block(
    block: &str,
    modifier_index: usize,
    hero_name: &str,
    tier_index: usize,
) -> Result<HeroBlock, CompilerError> {
    let block = block.trim();

    let open_pos = block.find('(').ok_or_else(|| CompilerError::HeroParseError {
        modifier_index,
        hero_name: hero_name.to_string(),
        tier_index: Some(tier_index),
        position: 0,
        expected: "opening '('".to_string(),
        found: block[..block.len().min(40)].to_string(),
    })?;

    let close_pos = util::find_matching_close_paren(block, open_pos).ok_or_else(|| {
        CompilerError::ParenError {
            modifier_index,
            position: open_pos,
            depth: 1,
            context: block[open_pos..block.len().min(open_pos + 40)].to_string(),
        }
    })?;

    let replica_content = &block[open_pos + 1..close_pos];
    let outside_content = &block[close_pos + 1..];

    // Parse replica content using util functions
    let template = replica_content
        .strip_prefix("replica.")
        .and_then(|r| r.find('.').map(|end| r[..end].to_string()))
        .unwrap_or_default();

    let tier = extract_tier_number(replica_content);
    let hp = util::extract_hp(replica_content, true);
    let sd = util::extract_sd(replica_content, true);
    let block_color = util::extract_color(replica_content);
    let _img = extract_img_at_depth0(replica_content);

    // Abilitydata/triggerhpdata can be inside OR outside the replica parens
    let abilitydata = util::extract_nested_prop(replica_content, ".abilitydata.")
        .or_else(|| util::extract_nested_prop(outside_content, ".abilitydata."));
    let triggerhpdata = util::extract_nested_prop(replica_content, ".triggerhpdata.")
        .or_else(|| util::extract_nested_prop(outside_content, ".triggerhpdata."));
    let hue = util::extract_simple_prop(replica_content, ".hue.")
        .or_else(|| util::extract_simple_prop(outside_content, ".hue."));

    let modifier_chain = util::extract_modifier_chain(replica_content);
    let facades = util::extract_facades_from_chain(modifier_chain.as_deref().unwrap_or(""));
    let items_inside: Option<String> = None;

    let speech = util::extract_simple_prop(outside_content, ".speech.");
    let name = extract_display_name(outside_content);
    let doc = util::extract_simple_prop(outside_content, ".doc.");
    let items_outside = extract_items_outside(outside_content);

    let sprite_name = name.clone();

    Ok(HeroBlock {
        template,
        tier,
        hp: hp.unwrap_or(0),
        sd: sd.unwrap_or_default(),
        color: block_color,
        sprite_name,
        speech: speech.unwrap_or_default(),
        name,
        doc,
        abilitydata,
        triggerhpdata,
        hue,
        modifier_chain,
        facades,
        items_inside,
        items_outside,
    })
}

/// Extract `.tier.N` value at depth 0.
fn extract_tier_number(content: &str) -> Option<u8> {
    let pos = util::find_at_depth0(content, ".tier.")?;
    let val_start = pos + ".tier.".len();
    let b = content.as_bytes().get(val_start)?;
    if b.is_ascii_digit() { Some(b - b'0') } else { None }
}

/// Extract last `.img.DATA` value at depth 0.
fn extract_img_at_depth0(content: &str) -> Option<String> {
    let pos = util::find_last_at_depth0(content, ".img.")?;
    let val_start = pos + ".img.".len();
    let val = &content[val_start..];
    if val.is_empty() { None } else { Some(val.to_string()) }
}

/// Extract display name from last `.n.NAME` at depth 0 in outside content.
fn extract_display_name(outside: &str) -> String {
    if let Some(pos) = util::find_last_at_depth0(outside, ".n.") {
        let val_start = pos + 3;
        let remaining = &outside[val_start..];
        let val_end = remaining
            .find(".part.")
            .or_else(|| remaining.find(".mn."))
            .or_else(|| remaining.find('&'))
            .or_else(|| remaining.find('+'))
            .unwrap_or(remaining.len());
        return remaining[..val_end].to_string();
    }
    String::new()
}

/// Extract items outside the replica block (like `.i.self.Something`).
fn extract_items_outside(outside: &str) -> Option<String> {
    let mut items = Vec::new();
    let mut search_from = 0;
    while let Some(pos) = outside[search_from..].find(".i.") {
        let abs_pos = search_from + pos;
        let remaining = &outside[abs_pos + 3..];
        let val_end = util::find_next_prop_boundary(remaining);
        items.push(&outside[abs_pos..abs_pos + 3 + val_end]);
        search_from = abs_pos + 3 + val_end;
    }
    if items.is_empty() { None } else { Some(items.join("")) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_at_depth0() {
        let input = "(a+b)+(c+d)+(e)";
        let parts = util::split_at_depth0(input, '+');
        assert_eq!(parts, vec!["(a+b)", "(c+d)", "(e)"]);
    }

    #[test]
    fn test_find_matching_close_paren() {
        let input = "(a(b)c)d";
        assert_eq!(util::find_matching_close_paren(input, 0), Some(6));
    }

    #[test]
    fn test_extract_template() {
        assert_eq!(
            "replica.Lost.col.a".strip_prefix("replica.").and_then(|r| r.find('.').map(|e| r[..e].to_string())),
            Some("Lost".to_string())
        );
    }

    #[test]
    fn test_extract_sd() {
        let content = "replica.Lost.col.a.hp.5.sd.170-3:158-1:158-1:158-1:43:0.img.abc";
        assert_eq!(
            util::extract_sd(content, true),
            Some("170-3:158-1:158-1:158-1:43:0".to_string())
        );
    }

    #[test]
    fn test_extract_hp() {
        let content = "replica.Lost.col.a.hp.5.sd.170-3:0:0:0:0:0";
        assert_eq!(util::extract_hp(content, true), Some(5));
        let content2 = "replica.Lost.hp.12.tier.3";
        assert_eq!(util::extract_hp(content2, true), Some(12));
    }

    #[test]
    fn test_find_hero_color() {
        let content = "(replica.Lost.col.a.hp.5.sd.0:0:0:0:0:0)";
        assert_eq!(util::extract_color(content), Some('a'));
        // .i.col should NOT be detected as color
        let content2 = "(replica.Lost.i.col.k.pain.col.b)";
        assert_eq!(util::extract_color(content2), Some('b'));
    }
}
