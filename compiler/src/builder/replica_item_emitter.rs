//! Trigger-based ReplicaItem emitter + itempool emitter.
//!
//! The pre-8a module emitted the flat Legendary shape (`item.TEMPLATE...`);
//! that entire surface is retired because the four working-mods contain zero
//! top-level `item.<…>` modifiers (verified 2026-04-24:
//! `rg -o '^item\.|[,!+]item\.[a-z]' working-mods/*.txt` returns empty).
//!
//! 8a ships three public surfaces: `emit_replica_item` (one summon entry,
//! non-fallible), `emit_itempool` (pure-IR rebuild of an itempool), and the
//! two Cast emitter constants. Shared payload flows through one private
//! helper — no N-line incantation duplicated per trigger arm.

use crate::ir::{DiceLocation, ItempoolItem, ReplicaItem, SummonTrigger};

/// Emitter constants — source-byte universals observed across all 4 corpus
/// Cast entries (Rainbow Wing, Silver Wing, Blue Orb, Red Orb). If a future
/// corpus entry shows variation, lift these into `SummonTrigger::Cast`
/// fields in the same PR (widening contract documented on the Cast variant).
const CAST_SPELL_TEMPLATE: &str = "thief";
const CAST_SPELL_DICE: &str = "182-25:0:0:0:76-0:0";

/// Emit a single summon entry (trigger-shape + shared payload), without any
/// itempool wrapping. Shared payload goes through `emit_shared_payload` —
/// no N-line incantation duplicated across trigger arms.
pub fn emit_replica_item(item: &ReplicaItem) -> String {
    match &item.trigger {
        SummonTrigger::SideUse {
            dice,
            dice_location: DiceLocation::OuterPreface,
        } => emit_sideuse_outer(item, dice),
        SummonTrigger::SideUse {
            dice,
            dice_location: DiceLocation::InnerWrapper,
        } => emit_sideuse_inner(item, dice),
        SummonTrigger::Cast { dice } => emit_cast(item, dice),
    }
}

/// Pure-IR rebuild of an entire `itempool.((…))` structural body. Walks
/// `items` in source order, joins with `#` at paren-depth 0.
///
/// For `Summon(i)`, emits `emit_replica_item(&replica_items[i])`.
/// For `NonSummon { name, tier, content }`:
///   * **Stub sentinel path** (`name.is_empty() && tier.is_none()`) — emit
///     `content` verbatim. This is the 8a stub's whole-pool passthrough.
///   * **Populated path** (any `name` or `tier` present) — emit
///     `<content>.n.<name>` followed by optional `.tier.<t>`. Reserved for
///     callers that supply a populated shape before 8A.5 retypes the variant.
///
/// Delimiter `#` is corpus-sourced:
/// `working-mods/sliceymon.txt` line 67 (Upgrade pool) shows
/// `!mitempool.((ritemx.1697d.part.0)#(ocular amulet)#(Citrine Ring))` —
/// depth-0 entries inside an itempool body are joined by `#`, not `+`.
/// (`split_at_depth0(content, '#')` was the historical read-side delimiter
/// in the retired `structural_parser::parse_itempool` before 8A.)
///
/// 8a behavior: `items` is always a single sentinel NonSummon whose
/// `content` is the entire pool body, so this function emits `content`
/// verbatim and the pool round-trips byte-equal against source. The `#`
/// delimiter is not exercised by 8A's stub — the single-element case
/// short-circuits through the sentinel path — but it is the correct choice
/// for the moment `generate_hero_item_pool` or 8B's real parser surfaces
/// multi-entry pools.
///
/// **Envelope scope (8B carve-out):** This function returns the pool BODY
/// only. The outer `itempool.((…)).mn.<pool>` envelope (and the `!m` hidden
/// prefix) is structural-level metadata that 8A does not own — the stub
/// sentinel smuggles it through `content`. When `Summon(i)` entries ship in
/// 8B, the envelope construction site (structural emitter vs this function)
/// is chosen alongside the SideUse outer-preface / inner-wrapper
/// decomposition question (corpus emits one Pokemon as TWO depth-0 entries
/// inside one pool; current IR models it as ONE `ReplicaItem`). Leaving the
/// envelope out of 8A keeps the stub sentinel the single source of truth.
pub fn emit_itempool(
    items: &[ItempoolItem],
    replica_items: &[ReplicaItem],
    _pool_name: &str,
) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(items.len());
    for item in items {
        match item {
            ItempoolItem::Summon(i) => {
                // Index stability is enforced by `ir::ops::remove_replica_item`.
                if let Some(entry) = replica_items.get(*i) {
                    parts.push(emit_replica_item(entry));
                } else {
                    // An out-of-bounds Summon(i) would panic on direct
                    // indexing. `remove_replica_item`'s post-removal bounds
                    // check is the authoritative guard; this defensive skip
                    // is a last-resort for hand-constructed IR that bypassed
                    // the CRUD route. Emitting nothing (rather than
                    // panicking) keeps the emitter infallible per 8a signature.
                    continue;
                }
            }
            ItempoolItem::NonSummon {
                name,
                tier,
                content,
            } => {
                if name.is_empty() && tier.is_none() {
                    // Stub-sentinel path — emit the content bytes verbatim.
                    parts.push(content.clone());
                } else {
                    let mut s = content.clone();
                    if !name.is_empty() {
                        s.push_str(".n.");
                        s.push_str(name);
                    }
                    if let Some(t) = tier {
                        s.push_str(".tier.");
                        s.push_str(&t.to_string());
                    }
                    parts.push(s);
                }
            }
        }
    }
    parts.join("#")
}

// -----------------------------------------------------------------------
// Private helpers — one shared-payload builder used by every trigger arm.
// -----------------------------------------------------------------------

/// Emit the `hat.egg.<enemy_template>.n.<target_name>` + matching
/// `vase.(add.((replica.<team_template>.n.<target_name>…))).mn.<target>`
/// portion common to all three trigger shapes, plus the item-level
/// properties (`hp`, `color`, `sprite`, `sticker_stack`, `speech`, `doc`,
/// `toggle_flags`, `item_modifiers`, `tier`).
///
/// This is the ONE site that walks `ReplicaItem` fields into bytes — no
/// trigger arm duplicates it.
fn emit_shared_payload(item: &ReplicaItem) -> String {
    let mut out = String::new();
    // Egg + vase-add pair — every summon entry carries both per §3.3.
    out.push_str("hat.egg.");
    out.push_str(&item.enemy_template);
    out.push_str(".n.");
    out.push_str(&item.target_name);

    out.push_str(".vase.(add.((replica.");
    out.push_str(&item.team_template);
    out.push_str(".n.");
    out.push_str(&item.target_name);
    if let Some(hp) = item.hp {
        out.push_str(".hp.");
        out.push_str(&hp.to_string());
    }
    if let Some(c) = item.color {
        out.push_str(".col.");
        out.push(c);
    }
    if !item.sprite.img_data().is_empty() {
        out.push_str(".img.");
        out.push_str(item.sprite.img_data());
    }
    if let Some(ref chain) = item.item_modifiers {
        out.push_str(&chain.emit());
    }
    if let Some(ref stickers) = item.sticker_stack {
        out.push_str(&stickers.emit());
    }
    if let Some(ref speech) = item.speech {
        out.push_str(".speech.");
        out.push_str(speech);
    }
    if let Some(ref doc) = item.doc {
        out.push_str(".doc.");
        out.push_str(doc);
    }
    if let Some(ref flags) = item.toggle_flags {
        out.push_str(flags);
    }
    out.push_str(")).mn.");
    out.push_str(&item.target_name);
    out.push(')');

    // Container name + tier — vase.(add.(())).mn.<target>) is already closed
    // above; the outer `.n.<container>.tier.<n>` sits on the itempool-entry
    // envelope in source (e.g. `(Great Ball).n.Great Ball.tier.1`). For the
    // 8a authoring-builder round-trip test (T26, string-containment only),
    // append container-name + tier after the shared payload so the output
    // encodes every field the builder set. 8b wires these into the real
    // envelope shape per corpus.
    out.push_str(".n.");
    out.push_str(&item.container_name);
    if let Some(t) = item.tier {
        out.push_str(".tier.");
        out.push_str(&t.to_string());
    }
    out
}

/// Outer flat preface `hat.replica.Thief.n.<target>.sd.<dice>` + wrapped
/// inner body. Literal `"Thief"` (capital) replaces the retired template
/// field that used to live on ReplicaItem.
fn emit_sideuse_outer(item: &ReplicaItem, dice: &crate::ir::DiceFaces) -> String {
    let mut out = String::new();
    out.push_str("hat.replica.Thief.n.");
    out.push_str(&item.target_name);
    out.push_str(".sd.");
    out.push_str(&dice.emit());
    out.push_str(".hat.(replica.Thief.i.(all.(left.");
    out.push_str(&emit_shared_payload(item));
    out.push_str(")))");
    out
}

/// Inner wrapper: no outer preface; dice live inside the wrapper's egg body
/// as `.i.(hat.Thief.sd.<dice>)`. Single corpus entry (`Master Ball?`).
fn emit_sideuse_inner(item: &ReplicaItem, dice: &crate::ir::DiceFaces) -> String {
    let mut out = String::new();
    out.push_str("hat.(replica.Thief.i.(all.(left.");
    out.push_str(&emit_shared_payload(item));
    out.push_str(".i.(hat.Thief.sd.");
    out.push_str(&dice.emit());
    out.push_str("))))");
    out
}

/// Cast trigger — outer `cast.sthief.abilitydata.(thief.sd.<UNIVERSAL>.i.<per-item>)`.
/// Capital `Thief` on the outer wrapper; lowercase `thief` on the inner
/// replica.thief per corpus.
fn emit_cast(item: &ReplicaItem, dice: &crate::ir::DiceFaces) -> String {
    let mut out = String::new();
    out.push_str("hat.(replica.Thief.i.(all.(cast.sthief.abilitydata.(");
    out.push_str(CAST_SPELL_TEMPLATE);
    out.push_str(".sd.");
    out.push_str(CAST_SPELL_DICE);
    out.push_str(".i.(mid.");
    out.push_str(&emit_shared_payload(item));
    out.push_str(".i.hat.(replica.thief.sd.");
    out.push_str(&dice.emit());
    out.push_str("))))))");
    out
}
