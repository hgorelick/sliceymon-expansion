use crate::error::CompilerError;
use crate::ir::{Hero, HeroFormat};

/// Emit a Hero struct as a modifier string.
///
/// Reconstructs the modifier from parsed fields based on format. Sprite data
/// rides on each `HeroBlock.sprite` (SpriteId); an empty `img_data()` means
/// the source block had no `.img.` property, so emission is skipped to
/// preserve roundtrip fidelity.
pub fn emit(hero: &Hero) -> Result<String, CompilerError> {
    match hero.format {
        HeroFormat::Sliceymon => emit_sliceymon(hero),
        HeroFormat::Grouped | HeroFormat::Unknown => emit_grouped(hero),
    }
}

/// Emit a Sliceymon-format hero (ph.b prefix, !mheropool.).
fn emit_sliceymon(hero: &Hero) -> Result<String, CompilerError> {
    if hero.blocks.is_empty() {
        return Err(CompilerError::build(
            format!("hero:{}", hero.internal_name),
            "hero has no blocks",
        ));
    }

    let mut out = String::new();

    // Prefix: hidden&temporary&ph.b{name};1;!mheropool.
    out.push_str("hidden&temporary&ph.b");
    out.push_str(&hero.internal_name);
    out.push_str(";1;!mheropool.");

    // Emit each block
    let mut emitted_sliceymon = 0;
    for (i, block) in hero.blocks.iter().enumerate() {
        // Skip degenerate blocks (vanilla references parsed as empty blocks)
        if is_degenerate_block(block) {
            continue;
        }

        if emitted_sliceymon > 0 {
            out.push('+');
        }
        emitted_sliceymon += 1;

        let resolved_img = resolve_sprite(block);
        let _ = i;

        if block.bare {
            // Bare block: Template.props... (no replica wrapper)
            out.push_str(&block.template);
            if let Some(ref doc) = block.doc {
                out.push_str(".doc.");
                out.push_str(doc);
            }
            if !block.name.is_empty() {
                out.push_str(".n.");
                out.push_str(&block.name);
            }
            if let Some(ref chain) = block.modifier_chain {
                out.push_str(&chain.emit());
            }
            if let Some(t) = block.tier {
                out.push_str(".tier.");
                out.push_str(&t.to_string());
            }
            if let Some(hp) = block.hp {
                out.push_str(".hp.");
                out.push_str(&hp.to_string());
            }
            if let Some(c) = block.color {
                out.push_str(".col.");
                out.push(c);
            }
            if !block.speech.is_empty() {
                out.push_str(".speech.");
                out.push_str(&block.speech);
            }
            out.push_str(".sd.");
            out.push_str(&block.sd.emit());
            if let Some(ref img) = resolved_img {
                out.push_str(".img.");
                out.push_str(img);
            }
        } else {
            // Standard replica-wrapped block
            out.push_str("(replica.");
            out.push_str(&block.template);

            // Color: only emit if the block explicitly has a color.
            // Falling back to hero.color would add .col. where the source had none
            // (e.g., T1 Larvesta inherits Volcarona's 'o' implicitly at runtime).
            if let Some(c) = block.color {
                out.push_str(".col.");
                out.push(c);
            }

            // Tier number (omit for T1 / None)
            if let Some(t) = block.tier {
                out.push_str(".tier.");
                out.push_str(&t.to_string());
            }

            // HP
            if let Some(hp) = block.hp {
                out.push_str(".hp.");
                out.push_str(&hp.to_string());
            }

            // Hue (inside replica block, before modifier chain)
            if let Some(ref hue) = block.hue {
                out.push_str(".hue.");
                out.push_str(hue);
            }

            // Modifier chain (.i./.k./.facade. sequences) - inside replica block
            if let Some(ref chain) = block.modifier_chain {
                out.push_str(&chain.emit());
            }

            // Items inside replica block
            if let Some(ref items) = block.items_inside {
                out.push_str(&items.emit());
            }

            // SD (dice faces)
            out.push_str(".sd.");
            out.push_str(&block.sd.emit());

            // IMG (sprite encoding) — optional
            if let Some(ref img) = resolved_img {
                out.push_str(".img.");
                out.push_str(img);
            }

            // Close replica block
            out.push(')');

            // Abilitydata (outside replica parens)
            if let Some(ref ability) = block.abilitydata {
                out.push_str(".abilitydata.");
                out.push_str(&ability.emit());
            }

            // Triggerhpdata (outside replica parens)
            if let Some(ref thp) = block.triggerhpdata {
                out.push_str(".triggerhpdata.");
                out.push_str(&thp.emit());
            }

            // Speech (outside replica parens)
            out.push_str(".speech.");
            out.push_str(&block.speech);

            // Items outside replica
            if let Some(ref items) = block.items_outside {
                out.push_str(&items.emit());
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
    }

    // Suffix
    out.push_str(".part.1&hidden");
    out.push_str(".mn.");
    out.push_str(&hero.mn_name);
    out.push_str("@2!m(skip&hidden&temporary)");

    Ok(out)
}

/// Emit a grouped-format hero (pansaer/punpuns/community: heropool.Name+...replica blocks).
fn emit_grouped(hero: &Hero) -> Result<String, CompilerError> {
    if hero.blocks.is_empty() {
        return Err(CompilerError::build(
            format!("hero:{}", hero.internal_name),
            "hero has no blocks",
        ));
    }

    let mut out = String::new();

    // Grouped format: heropool.Name+(replica.Template...)..
    out.push_str("heropool.");

    let mut emitted = 0;
    for (i, block) in hero.blocks.iter().enumerate() {
        // Skip degenerate blocks (vanilla references parsed as empty blocks)
        if is_degenerate_block(block) {
            continue;
        }

        if emitted > 0 {
            out.push('+');
        }
        emitted += 1;

        let resolved_img = resolve_sprite(block);
        let _ = i;

        // Bare blocks emit as `TEMPLATE.props...` without the `(replica....)` wrapper.
        // This preserves community's bare-block convention (e.g., `z1Q.n.Timebomb.sd.X.img.Y`).
        if !block.bare {
            out.push_str("(replica.");
        }
        out.push_str(&block.template);

        // Color: only emit if the block explicitly has a color.
        if let Some(c) = block.color {
            out.push_str(".col.");
            out.push(c);
        }

        // Tier number
        if let Some(t) = block.tier {
            out.push_str(".tier.");
            out.push_str(&t.to_string());
        }

        // HP
        if let Some(hp) = block.hp {
            out.push_str(".hp.");
            out.push_str(&hp.to_string());
        }

        // Hue
        if let Some(ref hue) = block.hue {
            out.push_str(".hue.");
            out.push_str(hue);
        }

        // Modifier chain
        if let Some(ref chain) = block.modifier_chain {
            out.push_str(&chain.emit());
        }

        // Items inside
        if let Some(ref items) = block.items_inside {
            out.push_str(&items.emit());
        }

        // SD
        out.push_str(".sd.");
        out.push_str(&block.sd.emit());

        // IMG — optional (skip when source had no explicit .img.)
        if let Some(ref img) = resolved_img {
            out.push_str(".img.");
            out.push_str(img);
        }

        // Close replica (only for non-bare blocks)
        if !block.bare {
            out.push(')');
        }

        // Abilitydata
        if let Some(ref ability) = block.abilitydata {
            out.push_str(".abilitydata.");
            out.push_str(&ability.emit());
        }

        // Triggerhpdata
        if let Some(ref thp) = block.triggerhpdata {
            out.push_str(".triggerhpdata.");
            out.push_str(&thp.emit());
        }

        // Speech
        if !block.speech.is_empty() {
            out.push_str(".speech.");
            out.push_str(&block.speech);
        }

        // Items outside
        if let Some(ref items) = block.items_outside {
            out.push_str(&items.emit());
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

/// Resolve sprite data for emission. Per SPEC §F4/§3.3, every `HeroBlock.sprite`
/// is a complete `SpriteId` carrying its own `img_data`. An empty `img_data()`
/// means the source had no `.img.` at all (e.g. an inherited block), so we
/// return `None` to suppress emission and preserve roundtrip.
fn resolve_sprite(block: &crate::ir::HeroBlock) -> Option<String> {
    let img = block.sprite.img_data();
    if img.is_empty() { None } else { Some(img.to_string()) }
}

/// Check if a block is a degenerate parser output that cannot be emitted.
/// These are vanilla reference names in grouped format that got parsed as empty blocks.
fn is_degenerate_block(block: &crate::ir::HeroBlock) -> bool {
    block.template.is_empty()
        && block.name.is_empty()
        && block.sprite.img_data().is_empty()
}

