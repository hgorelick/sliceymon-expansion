use crate::error::CompilerError;
use crate::ir::{ModIR, Source};

/// Merge a base ModIR with an overlay ModIR.
///
/// Merge rules:
/// - Heroes: overlay heroes REPLACE base heroes with the same `internal_name`. New heroes ADDED.
///   Heroes with `removed: true` are filtered out.
/// - ReplicaItems: match by `name`, replace or add.
/// - Monsters: match by `name`, replace or add.
/// - Bosses: match by `name`, replace or add.
/// - Structural: overlay structural modifiers REPLACE base modifiers with matching
///   `(modifier_type, name)` pair. Unknown types are appended.
///
/// All overlay items are marked with `Source::Overlay`.
pub fn merge(mut base: ModIR, overlay: ModIR) -> Result<ModIR, CompilerError> {
    // Heroes: replace by internal_name, add new, remove marked
    for mut hero in overlay.heroes {
        hero.source = Source::Overlay;
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

    // Replica items: replace by name, add new
    for mut item in overlay.replica_items {
        item.source = Source::Overlay;
        if let Some(pos) = base.replica_items.iter().position(|r| r.name == item.name) {
            base.replica_items[pos] = item;
        } else {
            base.replica_items.push(item);
        }
    }

    // Monsters: replace by name, add new
    for mut mon in overlay.monsters {
        mon.source = Source::Overlay;
        if let Some(pos) = base.monsters.iter().position(|m| m.name == mon.name) {
            base.monsters[pos] = mon;
        } else {
            base.monsters.push(mon);
        }
    }

    // Bosses: replace by name, add new
    for mut boss in overlay.bosses {
        boss.source = Source::Overlay;
        if let Some(pos) = base.bosses.iter().position(|b| b.name == boss.name) {
            base.bosses[pos] = boss;
        } else {
            base.bosses.push(boss);
        }
    }

    // Structural: replace by (modifier_type, name) pair, append otherwise.
    for mut s in overlay.structural {
        s.source = Source::Overlay;
        if let Some(pos) = base
            .structural
            .iter()
            .position(|bs| bs.modifier_type == s.modifier_type && bs.name == s.name)
        {
            base.structural[pos] = s;
        } else {
            base.structural.push(s);
        }
    }

    Ok(base)
}
