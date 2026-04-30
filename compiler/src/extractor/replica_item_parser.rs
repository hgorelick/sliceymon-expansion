use crate::error::CompilerError;
use crate::ir::{ItempoolItem, ReplicaItem};

/// Stub (transitional) extractor for an `itempool.((…))` body.
///
/// Returns one `ItempoolItem::NonSummon { name: String::new(), tier: None,
/// content: body.to_string() }` per itempool, carrying the ENTIRE body
/// verbatim inside `content`. This preserves byte-equal round-trip with zero
/// `ReplicaItem` extraction.
///
/// This is a KNOWN, TRACKED SPEC §3.2 raw-passthrough violation. The
/// transitional `NonSummon { name, tier, content }` is replaced with a
/// typed `NonSummonEntry` sum before 8b starts.
///
/// The empty `name` and `None` `tier` combined with a non-empty `content`
/// act as the **stub sentinel** that the emitter's `emit_itempool` detects
/// to skip the `.n.<name>.tier.<n>` prefix it would otherwise synthesize.
/// Without the sentinel, the stub would emit a stray `.n..tier.None` prefix
/// and diff every working-mod's itempool on round-trip.
///
/// 8b replaces this with the real per-entry classifier (after 8A.5 retypes
/// the variant).
///
/// The signature is the final 8b shape (not a narrower 8a-only signature)
/// so `extractor/mod.rs` wires against the real surface now — 8b is a body
/// replacement, not a signature change.
pub fn extract_from_itempool(
    body: &str,
    _modifier_index: usize,
    _next_replica_index: usize,
) -> Result<ItempoolExtraction, CompilerError> {
    Ok(ItempoolExtraction {
        new_replica_items: Vec::new(),
        items: vec![ItempoolItem::NonSummon {
            name: String::new(),
            tier: None,
            content: body.to_string(),
        }],
    })
}

/// Output of `extract_from_itempool` — the new `ReplicaItem`s (empty in 8a)
/// to append to `ModIR.replica_items`, and the `ItempoolItem` sum to store
/// on `StructuralContent::ItemPool`.
#[derive(Debug, Clone)]
pub struct ItempoolExtraction {
    pub new_replica_items: Vec<ReplicaItem>,
    pub items: Vec<ItempoolItem>,
}
