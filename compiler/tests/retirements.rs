//! Chunk 8A retirement greps + stub source-preservation + transitional NonSummon
//! round-trip. Per parent plan §5, retirement greps live in an integration test
//! file, not build.rs (which is forbidden — coupling `cargo build` success to
//! retirement absence drifts the WASM build surface).
//!
//! Tests shipped here:
//! - T12  — `ReplicaItemContainer` enum fully retired.
//! - T12a — `ModifierType::ReplicaItem{,WithAbility}` fully retired.
//! - T12b — Stub `extract_from_itempool` preserves body bytes verbatim
//!          (source-vs-IR divergence guard — the only 8a test that
//!          distinguishes "source bytes preserved" from "canonical shape
//!          recomputed").
//! - T13  — `parse_legendary` / `parse_simple` / `parse_with_ability` retired.
//! - T14  — `ModifierType::Legendary` retired.
//! - T25a — `authoring/replica_item.rs` does not expose any `AbilityData`
//!          field or method (parent §1.1: cast.sthief.abilitydata bodies
//!          have zero depth-0 `.n.<spell_name>`).
//! - T30  — Transitional `NonSummon { name: "", tier: None, content }` round-
//!          trips byte-equal via `emit_itempool` (stub-sentinel emitter path).

use std::fs;
use std::path::{Path, PathBuf};

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Walk every `*.rs` under `root` (relative to the crate dir) and collect
/// `(relative_path, line_number)` for each line containing `pattern`. Pure
/// std — no external crates. Skips `target/`.
///
/// If `root_rel` resolves to a FILE (not a directory), grep that single file
/// directly. Without this branch, `walk` would call `fs::read_dir(dir)` on a
/// file path — which fails — and silently return zero hits, making the test
/// vacuous.
fn recursive_grep(root_rel: &Path, pattern: &str) -> Vec<(String, usize)> {
    let root = crate_dir().join(root_rel);
    let mut hits: Vec<(String, usize)> = Vec::new();
    if root.is_file() {
        grep_file(&root, &root, pattern, &mut hits);
    } else {
        walk(&root, &root, pattern, &mut hits);
    }
    hits
}

fn grep_file(root: &Path, path: &Path, pattern: &str, hits: &mut Vec<(String, usize)>) {
    if path.extension().and_then(|s| s.to_str()) != Some("rs") {
        return;
    }
    if path.file_name().and_then(|s| s.to_str()) == Some("retirements.rs") {
        return;
    }
    let Ok(text) = fs::read_to_string(path) else {
        return;
    };
    for (i, line) in text.lines().enumerate() {
        if line.contains(pattern) {
            let rel = path.strip_prefix(root).unwrap_or(path).display().to_string();
            hits.push((rel, i + 1));
        }
    }
}

fn walk(root: &Path, dir: &Path, pattern: &str, hits: &mut Vec<(String, usize)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|s| s.to_str()) == Some("target") {
                continue;
            }
            walk(root, &path, pattern, hits);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            // Skip this file itself — the retirement test bodies carry the
            // retired pattern strings as match targets, and counting them
            // would flag this test as a violation of its own rule.
            if path.file_name().and_then(|s| s.to_str()) == Some("retirements.rs") {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            for (i, line) in text.lines().enumerate() {
                if line.contains(pattern) {
                    let rel = path.strip_prefix(root).unwrap_or(&path).display().to_string();
                    hits.push((rel, i + 1));
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// Retirement greps (T12 / T12a / T13 / T14)
// ---------------------------------------------------------------------

#[test]
fn grep_crate_for_replica_item_container_enum() {
    // T12
    let src_hits = recursive_grep(Path::new("src"), "ReplicaItemContainer");
    let test_hits = recursive_grep(Path::new("tests"), "ReplicaItemContainer");
    let total = src_hits.len() + test_hits.len();
    assert_eq!(
        total, 0,
        "ReplicaItemContainer still referenced: src={:?} tests={:?}",
        src_hits, test_hits
    );
}

#[test]
fn grep_crate_for_parse_legendary() {
    // T13
    let src_hits = recursive_grep(Path::new("src"), "parse_legendary");
    let test_hits = recursive_grep(Path::new("tests"), "parse_legendary");
    let total = src_hits.len() + test_hits.len();
    assert_eq!(
        total, 0,
        "parse_legendary still referenced: src={:?} tests={:?}",
        src_hits, test_hits
    );
}

#[test]
fn grep_crate_for_modifier_type_legendary() {
    // T14
    let hits = recursive_grep(Path::new("src"), "ModifierType::Legendary");
    assert_eq!(
        hits.len(),
        0,
        "ModifierType::Legendary still referenced: {:?}",
        hits
    );
}

#[test]
fn grep_crate_for_modifier_type_replica_item_variants() {
    // T12a — guards against upstream-retired variants being reintroduced.
    let hits_a = recursive_grep(Path::new("src"), "ModifierType::ReplicaItem ");
    let hits_b = recursive_grep(Path::new("src"), "ModifierType::ReplicaItemWithAbility");
    let total = hits_a.len() + hits_b.len();
    assert_eq!(
        total, 0,
        "ModifierType::ReplicaItem{{,WithAbility}} still referenced: a={:?} b={:?}",
        hits_a, hits_b
    );
}

#[test]
fn grep_crate_for_item_pool_entry_type() {
    // 8A-specific: the retired `ItemPoolEntry` struct must be gone.
    let src_hits = recursive_grep(Path::new("src"), "ItemPoolEntry");
    let test_hits = recursive_grep(Path::new("tests"), "ItemPoolEntry");
    let total = src_hits.len() + test_hits.len();
    assert_eq!(
        total, 0,
        "ItemPoolEntry still referenced: src={:?} tests={:?}",
        src_hits, test_hits
    );
}

// ---------------------------------------------------------------------
// T25a — authoring replica_item builder carries no AbilityData surface.
// ---------------------------------------------------------------------

#[test]
fn grep_authoring_replica_item_for_abilitydata() {
    // The builder deliberately exposes no `abilitydata()` method and no
    // `AbilityData` field — cast.sthief.abilitydata bodies have zero
    // depth-0 `.n.<spell_name>` (parent plan §1.1). This test makes the
    // absence load-bearing under `cargo test`, not just PR review.
    let lower = recursive_grep(Path::new("src/authoring/replica_item.rs"), "abilitydata");
    let upper = recursive_grep(Path::new("src/authoring/replica_item.rs"), "AbilityData");
    assert_eq!(
        lower.len() + upper.len(),
        0,
        "AbilityData re-introduced in authoring/replica_item.rs: lower={:?} upper={:?}",
        lower,
        upper
    );
}

// ---------------------------------------------------------------------
// T12b — source-vs-IR divergence guard (MANDATORY per hook rule).
//
// The 8a stub is trivially idempotent (stub extractor emits NonSummon with
// content=body; stub emitter re-emits that content verbatim), so every
// roundtrip-equality test passes even if the stub silently normalizes /
// drops / corrupts bytes. This test catches "the stub reached for a
// derived / canonical / registry source instead of the input bytes."
// ---------------------------------------------------------------------

#[test]
fn stub_preserves_itempool_body_byte_equal() {
    use textmod_compiler::extractor::replica_item_parser::extract_from_itempool;
    use textmod_compiler::ir::ItempoolItem;

    for body in [
        "(ritemx.1697d.part.0).n.A.tier.1",
        "(ritemx.1697d.part.0).n.a.tier.1",   // case-diff: a canonicalizer would fold
        "(ritemx.1697d.part.0).n.A.tier.1 ",  // trailing space: a normalizer would trim
    ] {
        let got = extract_from_itempool(body, 0, 0).expect("stub must never fail");
        assert_eq!(
            got.new_replica_items.len(),
            0,
            "8a stub never populates Summon entries"
        );
        assert_eq!(
            got.items.len(),
            1,
            "8a stub always emits exactly one NonSummon per pool"
        );
        match &got.items[0] {
            ItempoolItem::NonSummon { content, .. } => {
                assert_eq!(
                    content, body,
                    "NonSummon.content must be BYTE-EQUAL to input. Divergence \
                     means the stub reached for a derived / normalized source — \
                     this is the exact failure mode the hook rule exists to catch."
                );
            }
            ItempoolItem::Summon(_) => panic!("8a stub must never produce Summon"),
        }
    }
}

// ---------------------------------------------------------------------
// T30 — transitional NonSummon round-trip via raw `content`.
// ---------------------------------------------------------------------

#[test]
fn non_summon_transitional_raw_passthrough_roundtrips() {
    use textmod_compiler::builder::replica_item_emitter::emit_itempool;
    use textmod_compiler::ir::ItempoolItem;

    // Corpus-sourced fragment from sliceymon line 67 Upgrade Pool (abbreviated
    // img for fixture readability; byte-equality is the property under test,
    // not the specific bytes).
    let body = "((ritemx.1697d.part.0)#(ocular amulet)#(Citrine Ring)).n.Upgrade.tier.3.img.SHORTSPRITE";
    let items = vec![ItempoolItem::NonSummon {
        name: String::new(),
        tier: None,
        content: body.to_string(),
    }];
    let replica_items: Vec<textmod_compiler::ir::ReplicaItem> = Vec::new();
    let emitted = emit_itempool(&items, &replica_items, "Test Pool");
    assert!(
        emitted.contains(body),
        "transitional NonSummon raw-passthrough must preserve content byte-equal; got: {}",
        emitted
    );
}
