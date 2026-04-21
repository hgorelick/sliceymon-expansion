use crate::error::CompilerError;
use crate::ir::{Hero, HeroBlock, HeroFormat, Source};
use crate::util;
use crate::extractor::classifier;

/// Parse a hero modifier string into a Hero struct.
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
        Err(e) => {
            // Parse failure is now a hard error logged to stderr. We still produce a Hero
            // with empty blocks so the pipeline can report the issue rather than panicking.
            eprintln!("WARNING: Hero parse failed at modifier {}: {}", modifier_index, e);
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
                source: Source::Base,
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
            // Only strip leading whitespace — trailing whitespace may be part of
            // a `.n. ` display name (heroes can be named " " or have trailing spaces).
            let bs = bs.trim_start();
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
        source: Source::Base,
    }
}

/// Try to parse a single block from a grouped hero format.
fn try_parse_grouped_block(block: &str) -> Option<HeroBlock> {
    // Blocks are either:
    // 1. (replica.Template.col.X.hp.N.sd.FACES.img.DATA).speech.X.n.Name
    // 2. Bare template block: Template.n.Name.sd.FACES.img.DATA... (no wrapping parens)
    // 3. Plain name reference (no parens, no .sd.) — skip these
    // 4. Community nested: (((replica.((TEMPLATE.props).i.CHAIN.n.Name).abilitydata.(X)).img.HEROIMG).doc.DOC)
    let (replica_content, outside_content, is_bare) = if let Some(open_pos) = block.find('(') {
        if let Some(close_pos) = util::find_matching_close_paren(block, open_pos) {
            (&block[open_pos + 1..close_pos], &block[close_pos + 1..], false)
        } else {
            return None;
        }
    } else if block.contains(".sd.") || block.contains(".img.") {
        // Bare tier block — all content at same level, no inside/outside split
        (block, block, true)
    } else {
        return None;
    };

    // Peel successive balanced `(INNER).post_props` wrap layers, collecting the
    // post-close properties at each layer. This handles community's triple-nested
    // format: (((INNERMOST).img.X).doc.Y) — each layer contributes its own props.
    let mut outer_post_layers: Vec<String> = Vec::new();
    let mut inner = replica_content;
    loop {
        // Strip symmetric outer wrap: "(X)" -> "X" (only when the opening `(`
        // matches the FINAL `)`, so no props trail the wrap).
        if inner.starts_with('(') && inner.ends_with(')') {
            if let Some(close) = util::find_matching_close_paren(inner, 0) {
                if close == inner.len() - 1 {
                    inner = &inner[1..inner.len() - 1];
                    continue;
                }
            }
        }
        // Peel wrap with trailing properties: "(X).prop1.prop2" -> inner="X",
        // collect ".prop1.prop2" for later extraction.
        if inner.starts_with('(') {
            if let Some(close) = util::find_matching_close_paren(inner, 0) {
                if close < inner.len() - 1 {
                    outer_post_layers.push(inner[close + 1..].to_string());
                    inner = &inner[1..close];
                    continue;
                }
            }
        }
        break;
    }
    let unwrapped = inner;

    // Merge post-layer props so depth-0 extractors can find .img./.doc./etc.
    let extra_outer: String = outer_post_layers.join("");
    let outside_combined = format!("{}{}", outside_content, extra_outer);

    // Template: after stripping "replica.", also peel a leading "(template-group)"
    // used by community format (replica.(TEMPLATE.props)... instead of replica.TEMPLATE.props).
    let after_replica = unwrapped
        .strip_prefix("replica.")
        .or_else(|| unwrapped.strip_prefix("Replica."));
    let (template, template_flat) = match after_replica {
        Some(r) => {
            // Recursively peel leading (...) template-group wraps so props at
            // any nesting depth become depth-0 in the flattened content.
            // Handles:
            //   replica.TEMPLATE.props                   (sliceymon)
            //   replica.(TEMPLATE.props).rest            (community single-wrap)
            //   replica.((TEMPLATE.inner).rest1).rest2   (community double-wrap)
            let mut cur = r;
            let mut collected_after = String::new();
            while cur.starts_with('(') {
                if let Some(close) = util::find_matching_close_paren(cur, 0) {
                    let after = &cur[close + 1..];
                    collected_after.push_str(after);
                    cur = &cur[1..close];
                } else {
                    break;
                }
            }
            let end = cur.find('.').unwrap_or(cur.len());
            let t = cur[..end].to_string();
            let flat = format!("{}{}", cur, collected_after);
            (t, flat)
        }
        None => {
            let end = unwrapped.find('.').unwrap_or(unwrapped.len());
            let name = unwrapped[..end].to_string();
            (name, unwrapped.to_string())
        }
    };
    // Use template_flat for depth-0 extractions (handles community's
    // replica.(TEMPLATE_GROUP) pattern where props live inside the group).
    // outside_combined adds the post-close props from peeled layers.
    // Community format can place .col./.hp./.tier./.sd. either inside the replica
    // parens or as post-close properties on outside_combined (e.g., `(replica.zm).hp.5.col.g.tier.0`).
    // Check both locations, preferring inside.
    let tier = extract_tier_number(&template_flat)
        .or_else(|| extract_tier_number(&outside_combined));
    let hp = util::extract_hp(&template_flat, true)
        .or_else(|| util::extract_hp(&outside_combined, true));
    let sd = util::extract_sd(&template_flat, true)
        .or_else(|| util::extract_sd(&outside_combined, true))
        .map(|s| crate::ir::DiceFaces::parse(&s))
        .unwrap_or_else(|| crate::ir::DiceFaces { faces: vec![] });
    let block_color = util::extract_color(&template_flat)
        .or_else(|| util::extract_color(&outside_combined));

    let abilitydata = util::extract_nested_prop(&template_flat, ".abilitydata.")
        .or_else(|| util::extract_nested_prop(&outside_combined, ".abilitydata."))
        .map(|s| crate::ir::AbilityData::parse(&s));
    let triggerhpdata = util::extract_nested_prop(&template_flat, ".triggerhpdata.")
        .or_else(|| util::extract_nested_prop(&outside_combined, ".triggerhpdata."))
        .map(|s| crate::ir::TriggerHpDef::parse(&s));
    let hue = extract_depth0_simple_prop(&template_flat, ".hue.")
        .or_else(|| extract_depth0_simple_prop(&outside_combined, ".hue."));

    let img_data = util::extract_img_data(&template_flat)
        .or_else(|| util::extract_img_data(&outside_combined));
    let modifier_chain = util::extract_modifier_chain(&template_flat)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let facades = modifier_chain.as_ref()
        .map(|c| c.facades())
        .unwrap_or_default();
    let items_inside: Option<crate::ir::ModifierChain> = None;

    // For bare blocks, speech/name/doc are in replica_content (same level)
    let speech = util::extract_simple_prop(&outside_combined, ".speech.");
    let name = {
        let n = extract_display_name(&outside_combined);
        if n.is_empty() { extract_display_name(&template_flat) } else { n }
    };
    let doc = util::extract_simple_prop(&outside_combined, ".doc.");
    // Bare blocks have replica_content == outside_content, so extract_items_outside
    // would duplicate the entries already captured by modifier_chain. Skip it.
    let items_outside = if is_bare {
        None
    } else {
        extract_items_outside(&outside_combined)
            .map(|s| crate::ir::ModifierChain::parse(&s))
    };

    let sprite = crate::authoring::SpriteId::owned(
        name.clone(),
        img_data.clone().unwrap_or_default(),
    );

    Some(HeroBlock {
        template,
        tier,
        hp,
        sd,
        bare: is_bare,
        color: block_color,
        sprite,
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

    let hp_marker = find_heropool_marker(modifier).ok_or_else(|| {
        CompilerError::hero_parse(
            modifier_index,
            internal_name.clone(),
            None,
            0,
            "heropool.",
            modifier[..modifier.len().min(40)].to_string(),
        )
    })?;

    let content_start = hp_marker + "heropool.".len();
    let heropool_content = &modifier[content_start..];

    let mut tier_strs = util::split_at_depth0(heropool_content, '+');

    if tier_strs.is_empty() {
        return Err(CompilerError::hero_parse(
            modifier_index,
            internal_name.clone(),
            None,
            content_start,
            "at least one tier block",
            "empty heropool content",
        ));
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
        source: Source::Base,
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
    // First, check for bare-block suffix: .part.1&hidden... at depth 0
    // This must be checked BEFORE the paren-based approach because the suffix
    // itself may contain parens (e.g., .mn.Name@2!m(skip&hidden&temporary)).
    if let Some(part_pos) = util::find_last_at_depth0(last_tier, ".part.1&hidden") {
        // Verify there's block content before this (contains .sd. or .img. or .n.)
        let before = &last_tier[..part_pos];
        if before.contains(".sd.") || before.contains(".img.") || before.contains(".n.") {
            let suffix = last_tier[part_pos..].to_string();
            last_tier.truncate(part_pos);
            return suffix;
        }
    }

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

    let (replica_content, outside_content, is_bare) = if let Some(open_pos) = block.find('(') {
        let close_pos = util::find_matching_close_paren(block, open_pos).ok_or_else(|| {
            CompilerError::paren(modifier_index, open_pos, 1)
                .with_context(block[open_pos..block.len().min(open_pos + 40)].to_string())
        })?;
        (&block[open_pos + 1..close_pos], &block[close_pos + 1..], false)
    } else if block.contains(".sd.") || block.contains(".img.") {
        // Bare tier block — all content at same level
        (block, block, true)
    } else {
        return Err(CompilerError::hero_parse(
            modifier_index,
            hero_name.to_string(),
            Some(tier_index),
            0,
            "opening '(' or bare block with .sd./.img.",
            block[..block.len().min(40)].to_string(),
        ));
    };

    // Parse replica content using util functions
    // Unwrap nested parens: ((replica.X...)) -> replica.X...
    let unwrapped = {
        let mut s = replica_content;
        while s.starts_with('(') && s.ends_with(')') {
            s = &s[1..s.len()-1];
        }
        // Also handle leading ( without matching ) (asymmetric nesting)
        while s.starts_with('(') {
            s = &s[1..];
        }
        s
    };

    let template = unwrapped
        .strip_prefix("replica.")
        .or_else(|| unwrapped.strip_prefix("Replica."))
        .and_then(|r| r.find('.').map(|end| r[..end].to_string()))
        .or_else(|| {
            // Non-replica template: first segment before '.' (e.g., "Sparky.n.Rotom" -> "Sparky")
            let end = unwrapped.find('.').unwrap_or(unwrapped.len());
            let name = &unwrapped[..end];
            if name.is_empty() { None } else { Some(name.to_string()) }
        })
        .unwrap_or_default();

    let tier = extract_tier_number(replica_content);
    let hp = util::extract_hp(replica_content, true);
    let sd = util::extract_sd(replica_content, true)
        .map(|s| crate::ir::DiceFaces::parse(&s))
        .unwrap_or_else(|| crate::ir::DiceFaces { faces: vec![] });
    let block_color = util::extract_color(replica_content);
    let img_data = util::extract_img_data(replica_content)
        .or_else(|| util::extract_img_data(outside_content));

    // Abilitydata/triggerhpdata can be inside OR outside the replica parens
    let abilitydata = util::extract_nested_prop(replica_content, ".abilitydata.")
        .or_else(|| util::extract_nested_prop(outside_content, ".abilitydata."))
        .map(|s| crate::ir::AbilityData::parse(&s));
    let triggerhpdata = util::extract_nested_prop(replica_content, ".triggerhpdata.")
        .or_else(|| util::extract_nested_prop(outside_content, ".triggerhpdata."))
        .map(|s| crate::ir::TriggerHpDef::parse(&s));
    let hue = extract_depth0_simple_prop(replica_content, ".hue.")
        .or_else(|| extract_depth0_simple_prop(outside_content, ".hue."));

    let modifier_chain = util::extract_modifier_chain(replica_content)
        .map(|s| crate::ir::ModifierChain::parse(&s));
    let facades = modifier_chain.as_ref()
        .map(|c| c.facades())
        .unwrap_or_default();
    let items_inside: Option<crate::ir::ModifierChain> = None;

    // For bare blocks, speech/name/doc are in replica_content (same level)
    let speech = util::extract_simple_prop(outside_content, ".speech.");
    let name = {
        let n = extract_display_name(outside_content);
        if n.is_empty() { extract_display_name(unwrapped) } else { n }
    };
    let doc = util::extract_simple_prop(outside_content, ".doc.");
    let items_outside = extract_items_outside(outside_content)
        .map(|s| crate::ir::ModifierChain::parse(&s));

    let sprite = crate::authoring::SpriteId::owned(
        name.clone(),
        img_data.clone().unwrap_or_default(),
    );

    Ok(HeroBlock {
        template,
        tier,
        hp,
        sd,
        bare: is_bare,
        color: block_color,
        sprite,
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

/// Extract a simple property value at paren depth 0.
/// Unlike `util::extract_simple_prop`, this skips occurrences inside nested parens.
/// The value ends at the next property boundary or `)`.
fn extract_depth0_simple_prop(content: &str, marker: &str) -> Option<String> {
    let pos = util::find_at_depth0(content, marker)?;
    let val_start = pos + marker.len();
    let remaining = &content[val_start..];
    // Value ends at next property boundary, `)`, or end
    let mut end = remaining.len();
    let boundary = util::find_next_prop_boundary(remaining);
    if boundary < end { end = boundary; }
    // Also stop at close paren at depth 0 (belongs to outer scope)
    let mut depth: i32 = 0;
    for (i, ch) in remaining.char_indices() {
        if i >= end { break; }
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
    }
    let val = &remaining[..end];
    if val.is_empty() { None } else { Some(val.to_string()) }
}

/// Extract `.tier.N` value at depth 0.
fn extract_tier_number(content: &str) -> Option<u8> {
    let pos = util::find_at_depth0(content, ".tier.")?;
    let val_start = pos + ".tier.".len();
    let b = content.as_bytes().get(val_start)?;
    if b.is_ascii_digit() { Some(b - b'0') } else { None }
}

/// Extract display name from last `.n.NAME` at depth 0 in outside content.
/// NAME ends at the next property marker, `&`, `+`, or end of string.
fn extract_display_name(outside: &str) -> String {
    if let Some(pos) = util::find_last_at_depth0(outside, ".n.") {
        let val_start = pos + 3;
        let remaining = &outside[val_start..];
        let val_end = util::find_next_prop_boundary(remaining)
            .min(remaining.find('&').unwrap_or(remaining.len()))
            .min(remaining.find('+').unwrap_or(remaining.len()));
        return remaining[..val_end].to_string();
    }
    String::new()
}

/// Extract items outside the replica block (like `.i.self.Something`).
/// Must be paren-aware: items can contain parenthesized content like
/// `.i.self.Summon.(wolf.n.Name.hp.4.sd.FACES.img.DATA)`.
fn extract_items_outside(outside: &str) -> Option<String> {
    let non_item_markers = [
        ".col.", ".tier.", ".hp.", ".sd.", ".img.", ".abilitydata.", ".triggerhpdata.",
        ".doc.", ".hue.", ".speech.", ".n.", ".mn.", ".part.", ".bal.",
    ];

    let bytes = outside.as_bytes();
    let mut items: Vec<&str> = Vec::new();
    let mut i = 0;
    let mut outer_depth: i32 = 0;

    while i < bytes.len() {
        // Track paren depth in the outer scan so `.i.` inside a nested paren
        // group (e.g., `.abilitydata.(Fey.sd....i.left.k.X.i.left.k.Y...)`)
        // is NOT mistaken for a top-level item marker.
        match bytes[i] {
            b'(' => { outer_depth += 1; i += 1; continue; }
            b')' => { outer_depth -= 1; i += 1; continue; }
            _ => {}
        }

        // Stop scanning at the display-name marker `.n.` at depth 0 — anything
        // after that is the block's name, not an item (handles heroes whose
        // display name contains `.i.`, e.g., `.n.i.t.Barrel`).
        if outer_depth == 0 && i + 3 <= bytes.len() && &outside[i..i + 3] == ".n." {
            break;
        }

        // Only match `.i.` at depth 0.
        if outer_depth == 0 && i + 3 <= bytes.len() && &outside[i..i + 3] == ".i." {
            // Reject false matches: `.col.X.i.NEXT_PROP` — the `.i.` is really the
            // `.` between the color value `X` and the next property, not a chain marker.
            // Detect by checking whether the content after `.i.` starts with a
            // non-item property marker.
            let after = &outside[i + 2..]; // starts with the trailing `.`
            let is_false = non_item_markers.iter().any(|m| after.starts_with(m));
            if is_false {
                i += 1;
                continue;
            }

            let item_start = i;
            i += 3; // skip ".i."
            let mut depth: i32 = 0;

            while i < bytes.len() {
                match bytes[i] {
                    b'(' => { depth += 1; i += 1; }
                    b')' => {
                        depth -= 1;
                        if depth < 0 {
                            // Hit a close paren that belongs to outer scope — stop before it
                            outer_depth -= 1;
                            break;
                        }
                        i += 1;
                    }
                    b'.' if depth == 0 => {
                        // Check if this is a non-item property marker
                        let is_non_item = non_item_markers.iter().any(|m| {
                            i + m.len() <= bytes.len() && &outside[i..i + m.len()] == *m
                        });
                        if is_non_item {
                            break;
                        }
                        // Check if this is a new .i. — that starts a new item
                        if i + 3 <= bytes.len() && &outside[i..i + 3] == ".i." {
                            break;
                        }
                        i += 1;
                    }
                    _ => { i += 1; }
                }
            }

            items.push(&outside[item_start..i]);
        } else {
            i += 1;
        }
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
