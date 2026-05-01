//! Well-formedness gate for the doc-invariant carve-out registry.
//!
//! `compiler/tests/doc_invariants_carveouts.toml` is a fixture: it has no
//! `.rs` consumer in this PR, and the future doc-invariant guard test that
//! deserializes it (with a `serde::Deserialize` `CarveOut` struct + a
//! cross-grep filter) lands later. Without this test, an edit that breaks
//! TOML well-formedness or smuggles in a stray top-level key would land
//! silently — `cargo test` would not catch it because nothing would parse
//! the file.
//!
//! This integration test fills the gap with the minimum viable contract:
//!   1. The file parses as TOML.
//!   2. The only permitted top-level key is `carveout` (an array of
//!      `[[carveout]]` tables).
//!   3. If `carveout` is present, it is an array.
//!
//! Per-entry schema validation against the scaffold's field set is
//! intentionally NOT in this gate — that is the future guard test's
//! job. This gate covers only the structural invariants the scaffold
//! itself enforces; field-name authority is the scaffold (`tests/
//! doc_invariants_carveouts.toml`'s example `[[carveout]]` block),
//! re-enumerating it here would create a second canonical surface.

use std::fs;
use std::path::PathBuf;

#[test]
fn doc_invariants_carveouts_toml_is_well_formed() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = PathBuf::from(manifest_dir).join("tests/doc_invariants_carveouts.toml");
    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
    let parsed: toml::Table = toml::from_str(&contents)
        .unwrap_or_else(|e| panic!("registry must parse as TOML: {}", e));

    // Top-level keys: only `carveout` is permitted (an array-of-tables).
    // Any other top-level key is a sign the file picked up stray content
    // that the comment-block header would not have caught.
    for key in parsed.keys() {
        assert_eq!(
            key, "carveout",
            "unexpected top-level key {key:?} in {}",
            path.display()
        );
    }

    // If `carveout` is present, it must be an array-of-tables: TOML's
    // `[[carveout]]` syntax. The weaker `is_array()` predicate accepts
    // `carveout = ["string"]` or `carveout = [42]`; those parse as
    // arrays-of-strings or arrays-of-ints, which break the schema's
    // table-per-entry contract that the future doc-invariant guard test
    // (per-entry deserialize via `serde::Deserialize<CarveOut>`) depends
    // on. An empty `carveout` array is allowed — that's how the
    // registry starts and how it stays valid before any entry is
    // appended.
    if let Some(c) = parsed.get("carveout") {
        let arr = c.as_array().unwrap_or_else(|| {
            panic!("`carveout` top-level key must be an array-of-tables, got {c:?}")
        });
        for (i, entry) in arr.iter().enumerate() {
            assert!(
                entry.is_table(),
                "carveout[{i}] must be a `[[carveout]]` table, got {entry:?}"
            );
        }
    }
}
