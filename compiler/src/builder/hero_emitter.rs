use std::collections::HashMap;

use crate::error::CompilerError;
use crate::ir::{Hero, HeroFormat};

/// Emit a Hero struct as a modifier string.
///
/// If the hero has a `raw` field, emits that directly (passthrough mode).
/// Otherwise, reconstructs the modifier from parsed fields based on format.
pub fn emit(hero: &Hero, sprites: &HashMap<String, String>) -> Result<String, CompilerError> {
    // Raw passthrough
    if let Some(ref raw) = hero.raw {
        return Ok(raw.clone());
    }

    match hero.format {
        HeroFormat::Sliceymon => emit_sliceymon(hero, sprites),
        HeroFormat::Grouped | HeroFormat::Unknown => emit_grouped(hero, sprites),
    }
}

/// Emit a Sliceymon-format hero (ph.b prefix, !mheropool.).
fn emit_sliceymon(hero: &Hero, sprites: &HashMap<String, String>) -> Result<String, CompilerError> {
    if hero.blocks.is_empty() {
        return Err(CompilerError::BuildError {
            component: format!("hero:{}", hero.internal_name),
            message: "hero has no blocks".to_string(),
        });
    }

    let mut out = String::new();

    // Prefix: hidden&temporary&ph.b{name};1;!mheropool.
    out.push_str("hidden&temporary&ph.b");
    out.push_str(&hero.internal_name);
    out.push_str(";1;!mheropool.");

    // Emit each block
    for (i, block) in hero.blocks.iter().enumerate() {
        if i > 0 {
            out.push('+');
        }

        // Resolve sprite
        let img_data = sprites.get(&block.sprite_name).ok_or_else(|| {
            CompilerError::SpriteNotFound {
                sprite_name: block.sprite_name.clone(),
                hero_name: hero.internal_name.clone(),
                tier_index: i,
            }
        })?;

        // Open replica block
        out.push_str("(replica.");
        out.push_str(&block.template);

        // Color: use block color if set, otherwise hero color
        out.push_str(".col.");
        out.push(block.color.unwrap_or(hero.color));

        // Tier number (omit for T1 / None)
        if let Some(t) = block.tier {
            out.push_str(".tier.");
            out.push_str(&t.to_string());
        }

        // HP
        out.push_str(".hp.");
        out.push_str(&block.hp.to_string());

        // Modifier chain (.i./.k./.facade. sequences) - inside replica block
        if let Some(ref chain) = block.modifier_chain {
            out.push_str(chain);
        }

        // SD (dice faces)
        out.push_str(".sd.");
        out.push_str(&block.sd);

        // IMG (sprite encoding)
        out.push_str(".img.");
        out.push_str(img_data);

        // Close replica block
        out.push(')');

        // Abilitydata (outside replica parens)
        if let Some(ref ability) = block.abilitydata {
            out.push_str(".abilitydata.");
            out.push_str(ability);
        }

        // Triggerhpdata (outside replica parens)
        if let Some(ref thp) = block.triggerhpdata {
            out.push_str(".triggerhpdata.");
            out.push_str(thp);
        }

        // Speech (outside replica parens)
        out.push_str(".speech.");
        out.push_str(&block.speech);

        // Items outside replica
        if let Some(ref items) = block.items_outside {
            out.push_str(items);
        }

        // Doc (outside replica parens)
        if let Some(ref doc) = block.doc {
            out.push_str(".doc.");
            out.push_str(doc);
        }

        // Display name (always last before + or suffix)
        out.push_str(".n.");
        out.push_str(&block.name);
    }

    // Suffix
    out.push_str(".part.1&hidden");
    out.push_str(".mn.");
    out.push_str(&hero.mn_name);
    out.push_str("@2!m(skip&hidden&temporary)");

    Ok(out)
}

/// Emit a grouped-format hero (pansaer/punpuns/community: heropool.Name+...replica blocks).
fn emit_grouped(hero: &Hero, sprites: &HashMap<String, String>) -> Result<String, CompilerError> {
    if hero.blocks.is_empty() {
        return Err(CompilerError::BuildError {
            component: format!("hero:{}", hero.internal_name),
            message: "hero has no blocks".to_string(),
        });
    }

    let mut out = String::new();

    // Grouped format: heropool.Name+(replica.Template...)..
    out.push_str("heropool.");

    for (i, block) in hero.blocks.iter().enumerate() {
        if i > 0 {
            out.push('+');
        }

        // Resolve sprite
        let img_data = sprites.get(&block.sprite_name).ok_or_else(|| {
            CompilerError::SpriteNotFound {
                sprite_name: block.sprite_name.clone(),
                hero_name: hero.internal_name.clone(),
                tier_index: i,
            }
        })?;

        // Open replica block
        out.push_str("(replica.");
        out.push_str(&block.template);

        // Color
        let c = block.color.unwrap_or(hero.color);
        if c != '?' {
            out.push_str(".col.");
            out.push(c);
        }

        // Tier number
        if let Some(t) = block.tier {
            out.push_str(".tier.");
            out.push_str(&t.to_string());
        }

        // HP
        out.push_str(".hp.");
        out.push_str(&block.hp.to_string());

        // Modifier chain
        if let Some(ref chain) = block.modifier_chain {
            out.push_str(chain);
        }

        // SD
        out.push_str(".sd.");
        out.push_str(&block.sd);

        // IMG
        out.push_str(".img.");
        out.push_str(img_data);

        // Close replica
        out.push(')');

        // Abilitydata
        if let Some(ref ability) = block.abilitydata {
            out.push_str(".abilitydata.");
            out.push_str(ability);
        }

        // Triggerhpdata
        if let Some(ref thp) = block.triggerhpdata {
            out.push_str(".triggerhpdata.");
            out.push_str(thp);
        }

        // Speech
        if !block.speech.is_empty() {
            out.push_str(".speech.");
            out.push_str(&block.speech);
        }

        // Items outside
        if let Some(ref items) = block.items_outside {
            out.push_str(items);
        }

        // Doc
        if let Some(ref doc) = block.doc {
            out.push_str(".doc.");
            out.push_str(doc);
        }

        // Display name
        out.push_str(".n.");
        out.push_str(&block.name);
    }

    // Grouped format: .mn.Name at end (no sliceymon suffix)
    if !hero.mn_name.is_empty() {
        out.push_str(".mn.");
        out.push_str(&hero.mn_name);
    }

    Ok(out)
}

/// Check that emitted output has balanced parentheses.
pub fn verify_paren_balance(output: &str) -> Result<(), String> {
    let mut depth: i32 = 0;
    for (i, ch) in output.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Err(format!(
                        "Negative paren depth at position {}: ...{}...",
                        i,
                        &output[i.saturating_sub(20)..output.len().min(i + 20)]
                    ));
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        Err(format!("Unbalanced parens: depth {} at end of string", depth))
    } else {
        Ok(())
    }
}

/// Check that all '+' tier separators are at paren depth 0.
pub fn verify_tier_separators(output: &str) -> Result<(), String> {
    // Find heropool content
    let hp_start = output.find("heropool.").map(|p| p + "heropool.".len());
    if hp_start.is_none() {
        return Ok(()); // No heropool = no tier separators to check
    }
    let start = hp_start.unwrap();
    let content = &output[start..];

    let mut depth: i32 = 0;
    for (i, ch) in content.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            '+' if depth != 0 => {
                return Err(format!(
                    "'+' at paren depth {} (position {}): ...{}...",
                    depth,
                    start + i,
                    &content[i.saturating_sub(20)..content.len().min(i + 20)]
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

/// Check that `.n.NAME` is the last property before each `+` or end of heropool content.
pub fn verify_name_last(output: &str) -> Result<(), String> {
    let hp_start = output.find("heropool.").map(|p| p + "heropool.".len());
    if hp_start.is_none() {
        return Ok(());
    }

    // Split heropool content at depth-0 '+'
    let content = &output[hp_start.unwrap()..];
    let mut depth: i32 = 0;
    let mut tier_start = 0;

    for (i, ch) in content.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            '+' if depth == 0 => {
                let tier_block = &content[tier_start..i];
                if !tier_ends_with_name(tier_block) {
                    return Err(format!(
                        "Tier block does not end with .n.NAME: ...{}",
                        &tier_block[tier_block.len().saturating_sub(40)..]
                    ));
                }
                tier_start = i + 1;
            }
            _ => {}
        }
    }
    Ok(())
}

fn tier_ends_with_name(block: &str) -> bool {
    // The block should have .n.NAME as the last property
    // (possibly followed by .part. or .mn. suffix on the last tier)
    if let Some(last_n) = block.rfind(".n.") {
        let after_n = &block[last_n + 3..];
        // After .n.NAME, there should be nothing or suffix markers
        !after_n.contains(".speech.")
            && !after_n.contains(".sd.")
            && !after_n.contains(".img.")
            && !after_n.contains(".hp.")
    } else {
        false
    }
}

/// Check that output contains only ASCII characters.
pub fn verify_ascii_only(output: &str) -> Result<(), String> {
    for (i, ch) in output.char_indices() {
        if !ch.is_ascii() {
            return Err(format!(
                "Non-ASCII character '{}' (U+{:04X}) at position {}",
                ch, ch as u32, i
            ));
        }
    }
    Ok(())
}
