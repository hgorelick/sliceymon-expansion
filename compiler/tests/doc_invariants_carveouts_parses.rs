//! Well-formedness gate for the doc-invariant carve-out registry.
//!
//! `compiler/tests/doc_invariants_carveouts.toml` is a fixture: the
//! cross-grep guard test that deserializes it via a typed `CarveOut`
//! struct + filters the doc-violation hit set lands in a later chunk.
//! Without this test, an edit that breaks TOML well-formedness or
//! smuggles in a stray top-level key — or appends a `[[carveout]]`
//! whose addressing target is dead — would land silently; `cargo test`
//! would not catch it because nothing would parse the file.
//!
//! This integration test fills the gap with the structural-and-
//! addressing contract:
//!   1. The file parses as TOML.
//!   2. The only permitted top-level key is `carveout` (an array of
//!      `[[carveout]]` tables).
//!   3. If `carveout` is present, it is an array of tables.
//!   4. **Pattern-uniqueness addressing.** Every `[[carveout]]`
//!      entry's `pattern` MUST appear EXACTLY ONCE in the file at
//!      `path`. This is the load-bearing addressing contract: per-file
//!      uniqueness locates the carved-out site without a stored line
//!      number, so collision (count >= 2), removal (count == 0), and
//!      ambiguity-on-growth (count >= 2) all surface here as
//!      guard-test failures rather than silent rot. Required fields
//!      `path` and `pattern` must be strings; missing them is "unknown
//!      shape, fail fast" rather than a silent skip. Field-shape
//!      validation against the rest of the schema (`rationale`,
//!      `invariant_not_violated`) is the future cross-grep guard
//!      test's job; uniqueness lives here because it is a property of
//!      the registry-against-source mapping, no grep needed.
//!
//! Field-name authority is the scaffold (`tests/
//! doc_invariants_carveouts.toml`'s example `[[carveout]]` block);
//! this gate reads `path` and `pattern` by name (the two fields the
//! addressing contract depends on) and treats unrecognized shapes as
//! errors.

use std::fs;
use std::path::PathBuf;

#[test]
fn doc_invariants_carveouts_toml_is_well_formed() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let registry_path =
        PathBuf::from(manifest_dir).join("tests/doc_invariants_carveouts.toml");
    let contents = fs::read_to_string(&registry_path)
        .unwrap_or_else(|e| panic!("read {}: {}", registry_path.display(), e));
    let parsed: toml::Table = toml::from_str(&contents)
        .unwrap_or_else(|e| panic!("registry must parse as TOML: {}", e));

    // Top-level keys: only `carveout` is permitted (an array-of-tables).
    // Any other top-level key is a sign the file picked up stray content
    // that the comment-block header would not have caught.
    for key in parsed.keys() {
        assert_eq!(
            key, "carveout",
            "unexpected top-level key {key:?} in {}",
            registry_path.display()
        );
    }

    // The repo root is the parent of `compiler/`; entry `path` fields
    // are repo-relative (e.g. `"SPEC.md"`, `"personas/architecture.md"`),
    // so we resolve them against `<manifest_dir>/..`. This keeps the
    // gate CWD-independent the same way `manifest_dir` does for the
    // registry path itself.
    let repo_root = PathBuf::from(manifest_dir)
        .parent()
        .unwrap_or_else(|| panic!("CARGO_MANIFEST_DIR has no parent: {manifest_dir}"))
        .to_path_buf();

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
            let table = entry.as_table().unwrap_or_else(|| {
                panic!("carveout[{i}] must be a `[[carveout]]` table, got {entry:?}")
            });

            // `path` and `pattern` are the two fields the addressing
            // contract reads. Missing or non-string is "unknown shape,
            // fail fast" — not a silent skip. The remaining schema
            // fields (`rationale`, `invariant_not_violated`) are the
            // future cross-grep guard test's responsibility.
            let entry_path = table
                .get("path")
                .unwrap_or_else(|| panic!("carveout[{i}] missing required `path` field"))
                .as_str()
                .unwrap_or_else(|| {
                    panic!("carveout[{i}] `path` must be a string, got {:?}", table.get("path"))
                });
            let entry_pattern = table
                .get("pattern")
                .unwrap_or_else(|| panic!("carveout[{i}] missing required `pattern` field"))
                .as_str()
                .unwrap_or_else(|| {
                    panic!(
                        "carveout[{i}] `pattern` must be a string, got {:?}",
                        table.get("pattern")
                    )
                });

            // Per-file uniqueness: count occurrences of `pattern` as a
            // case-sensitive literal substring of the file at `path`.
            // Anything other than exactly 1 is a guard-test failure —
            // count == 0 means the pattern was removed (carve-out is
            // dead) and count >= 2 means the pattern collided or grew
            // ambiguous; in either case the carve-out's locator is
            // unsound and the author must restore the line, lengthen
            // the pattern, or drop the entry.
            let target_path = repo_root.join(entry_path);
            let target_contents = fs::read_to_string(&target_path).unwrap_or_else(|e| {
                panic!(
                    "carveout[{i}] `path = \"{entry_path}\"` does not resolve to a readable file at {}: {e}",
                    target_path.display()
                )
            });
            let count = target_contents.matches(entry_pattern).count();
            assert_eq!(
                count, 1,
                "carveout[{i}] `pattern` must appear EXACTLY ONCE in `{entry_path}` \
                 (per-file uniqueness is the addressing contract); observed {count} occurrences. \
                 pattern = {entry_pattern:?}"
            );
        }
    }
}
