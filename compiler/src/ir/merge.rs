use crate::error::CompilerError;
use crate::ir::ModIR;

/// Merge a base ModIR with an overlay ModIR.
///
/// Merge rules:
/// - Heroes: overlay heroes REPLACE base heroes with the same `internal_name`. New heroes ADDED.
///   Heroes with `removed: true` are filtered out.
/// - Captures: match by `pokemon` name, replace or add.
/// - Legendaries: match by `pokemon` name, replace or add.
/// - Monsters: match by `name`, replace or add.
/// - Bosses: match by `name`, replace or add.
/// - Structural: overlay structural modifiers REPLACE base modifiers with matching
///   `(modifier_type, name)` pair. Unknown types are appended.
pub fn merge(mut base: ModIR, overlay: ModIR) -> Result<ModIR, CompilerError> {
    // Heroes: replace by internal_name, add new, remove marked
    for hero in overlay.heroes {
        if let Some(pos) = base
            .heroes
            .iter()
            .position(|h| h.internal_name == hero.internal_name)
        {
            base.heroes[pos] = hero;
        } else {
            base.heroes.push(hero);
        }
    }
    // Filter removed heroes
    base.heroes.retain(|h| !h.removed);

    // Captures: replace by pokemon name, add new
    for cap in overlay.captures {
        if let Some(pos) = base.captures.iter().position(|c| c.pokemon == cap.pokemon) {
            base.captures[pos] = cap;
        } else {
            base.captures.push(cap);
        }
    }

    // Legendaries: replace by pokemon name, add new
    for leg in overlay.legendaries {
        if let Some(pos) = base
            .legendaries
            .iter()
            .position(|l| l.pokemon == leg.pokemon)
        {
            base.legendaries[pos] = leg;
        } else {
            base.legendaries.push(leg);
        }
    }

    // Monsters: replace by name, add new
    for mon in overlay.monsters {
        if let Some(pos) = base.monsters.iter().position(|m| m.name == mon.name) {
            base.monsters[pos] = mon;
        } else {
            base.monsters.push(mon);
        }
    }

    // Bosses: replace by name, add new
    for boss in overlay.bosses {
        if let Some(pos) = base.bosses.iter().position(|b| b.name == boss.name) {
            base.bosses[pos] = boss;
        } else {
            base.bosses.push(boss);
        }
    }

    // Structural: replace by (modifier_type, name) pair, append Unknown
    for s in overlay.structural {
        if s.modifier_type == crate::ir::StructuralType::Unknown {
            base.structural.push(s);
        } else if let Some(pos) = base
            .structural
            .iter()
            .position(|bs| bs.modifier_type == s.modifier_type && bs.name == s.name)
        {
            base.structural[pos] = s;
        } else {
            base.structural.push(s);
        }
    }

    // Clear original_modifiers -- merged IR uses type-based assembly
    base.original_modifiers = None;

    Ok(base)
}
