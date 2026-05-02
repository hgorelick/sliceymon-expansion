//! Well-formedness and addressing gate for the doc-invariant carve-out
//! registry.
//!
//! `compiler/tests/doc_invariants_carveouts.toml` is a fixture: the
//! cross-grep guard test that deserializes it via a typed `CarveOut`
//! struct + filters the doc-violation hit set lands in a later chunk.
//! Without this test, an edit that breaks TOML well-formedness or
//! smuggles in a stray top-level key — or appends a `[[carveout]]`
//! whose addressing target is dead, ambiguous, duplicate, or escapes
//! the repo root — would land silently; `cargo test` would not catch
//! it because nothing else parses the file.
//!
//! Contract enforced (the scaffold's append-discipline at the top of
//! `tests/doc_invariants_carveouts.toml` is the canonical enumeration
//! of the failure modes per-entry uniqueness catches):
//!
//!   1. The file parses as TOML.
//!   2. The only permitted top-level key is `carveout` (an array of
//!      tables — TOML's `[[carveout]]` array-of-tables form is the
//!      canonical surface syntax in the scaffold; the inline form
//!      `carveout = [{ ... }]` parses to the same value and is
//!      accepted but discouraged in the append-discipline).
//!   3. If `carveout` is present, every entry is a table with
//!      required string fields `path` and `pattern` present
//!      (missing, non-string, or empty is "unknown shape, fail fast"
//!      rather than a silent skip). Field-shape validation for
//!      remaining fields per the scaffold's schema-by-construction
//!      example block is the future cross-grep guard test's job.
//!   4. **Per-file pattern uniqueness.** Every entry's `pattern` MUST
//!      appear EXACTLY ONCE in the file at `path`. Per-file, not
//!      global — identical patterns across distinct `path` values are
//!      independent carve-outs. Two distinct patterns whose unique
//!      matches happen to land on the same line are also legitimate
//!      (addressing is per-pattern, not per-line, because line numbers
//!      are deliberately not stored).
//!   5. **Pair uniqueness on canonical paths.** No two entries share
//!      a `(canonicalized_path, pattern)` tuple — a duplicate would
//!      attach two contradictory rationales to the same carved-out
//!      site, silent rot the per-entry count check cannot detect
//!      (each entry's count is 1 independently). The path is
//!      canonicalized BEFORE the ledger lookup so two entries
//!      addressing the same file via different spellings collapse to
//!      one canonical key: segment-syntax variants (`SPEC.md` vs
//!      `./SPEC.md` vs `compiler/../SPEC.md`) AND case-only spellings
//!      on case-insensitive filesystems (macOS APFS-default,
//!      NTFS-default), because `Path::canonicalize` resolves to the
//!      on-disk leaf-name spelling — `path = "spec.md"` and `path =
//!      "SPEC.md"` canonicalize to the same `PathBuf` when the file's
//!      actual on-disk name is `SPEC.md`, so the duplicate cannot be
//!      smuggled in by path-spelling drift.
//!   6. **Repo-root containment.** Every `path`, after canonicalization
//!      from the repo root, stays inside the repo. Absolute paths and
//!      `..`-traversal that escapes the repo root are rejected so the
//!      gate's per-file uniqueness count is taken against bytes inside
//!      the repo (a registry entry whose uniqueness "evidence" is
//!      bytes the audit doesn't own would defeat the contract; chunk
//!      8's cross-grep guard then inherits that guarantee).

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn doc_invariants_carveouts_registry_is_well_formed_and_addressing_is_sound() {
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
    // so we resolve them against `<manifest_dir>/..`. Canonicalize once
    // up front so the per-entry containment check has a stable root to
    // compare against (canonicalized paths normalize `..`/symlinks the
    // same way on both sides).
    let repo_root = PathBuf::from(manifest_dir)
        .parent()
        .unwrap_or_else(|| panic!("CARGO_MANIFEST_DIR has no parent: {manifest_dir}"))
        .to_path_buf();
    let repo_root_canonical = repo_root.canonicalize().unwrap_or_else(|e| {
        panic!("repo root {} does not canonicalize: {e}", repo_root.display())
    });

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
        // Pair-uniqueness ledger keyed on (canonical_path, pattern).
        // Two entries sharing this key would attach contradictory
        // rationales to the same carved-out site — silent rot the
        // per-entry count==1 check cannot see (each entry's count is 1
        // independently). Keying on the canonicalized path collapses
        // spelling variants (`SPEC.md` vs `./SPEC.md` vs `compiler/../SPEC.md`)
        // to one identity, so the duplicate cannot be smuggled in by
        // path-spelling drift.
        let mut seen: HashSet<(PathBuf, String)> = HashSet::new();
        for (i, entry) in arr.iter().enumerate() {
            let table = entry.as_table().unwrap_or_else(|| {
                panic!("carveout[{i}] must be a `[[carveout]]` table, got {entry:?}")
            });

            // `path` and `pattern` are the two fields the addressing
            // contract reads. Missing, non-string, or empty is
            // "unknown shape, fail fast" — not a silent skip. Remaining
            // schema fields are validated against the scaffold's
            // schema-by-construction by the future cross-grep guard test.
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
            // Empty-`path` reject. `Path::new("").is_absolute()` is
            // false, so the absolute-path check below would let an
            // empty path through. `repo_root.join("")` returns
            // `repo_root` (a directory), `canonicalize` succeeds on
            // it, `starts_with(&repo_root_canonical)` passes, then
            // `fs::read_to_string` panics with "Is a directory" —
            // diagnosing the wrong defect class (read failure, not
            // empty-field). Catch it here so the panic names the real
            // mistake. Symmetric with the empty-`pattern` reject below
            // and with clause 3 of the contract docstring at the top
            // of this file (which names BOTH `path` and `pattern` for
            // "missing, non-string, or empty is fail-fast").
            assert!(
                !entry_path.is_empty(),
                "carveout[{i}] `path` must be a non-empty repo-relative file path \
                 (an empty path joins to the repo root, which is a directory and not an addressing target)"
            );
            // Empty-pattern reject. `String::matches("")` returns one
            // match per byte boundary, so an empty pattern would
            // incidentally fire the count==1 assert below with a
            // count proportional to the file size — the panic message
            // would name the wrong defect ("observed N occurrences" for
            // a pattern that matches nothing meaningful). Catch the real
            // defect ("pattern field is empty") here, before the count
            // check, so the diagnostic points at the actual mistake.
            assert!(
                !entry_pattern.is_empty(),
                "carveout[{i}] `pattern` must be a non-empty literal substring \
                 (an empty pattern matches every byte position and is not an addressing key)"
            );

            // Repo-root containment, two checks. (a) Reject absolute
            // paths up front so the panic message names the right defect
            // class — `PathBuf::join` discards the left side when the
            // right is absolute, so without this check `path = "/etc/x"`
            // would silently read `/etc/x` outside the repo. (b) Resolve
            // the relative path and canonicalize; assert the canonical
            // form stays inside the canonicalized repo root, which
            // catches `..`-traversal that lands outside the repo even
            // though `is_absolute()` returns false for it.
            assert!(
                !Path::new(entry_path).is_absolute(),
                "carveout[{i}] `path` must be repo-relative, not absolute (got {entry_path:?})"
            );
            let target_path = repo_root.join(entry_path);
            let canonical_target = target_path.canonicalize().unwrap_or_else(|e| {
                panic!(
                    "carveout[{i}] `path = {entry_path:?}` does not resolve to a readable file at {}: {e}",
                    target_path.display()
                )
            });
            assert!(
                canonical_target.starts_with(&repo_root_canonical),
                "carveout[{i}] `path = {entry_path:?}` escapes the repo root \
                 (resolved to {}, expected to start with {}); registry paths must stay inside the repo \
                 so the per-file uniqueness count is taken against bytes inside the audit's scope",
                canonical_target.display(),
                repo_root_canonical.display()
            );

            // Pair uniqueness on canonical path. Performed AFTER
            // canonicalize so the ledger key is the resolved file
            // identity, not the author-supplied spelling — two entries
            // addressing the same canonical site via different path
            // spellings collapse to one key and the duplicate fires.
            assert!(
                seen.insert((canonical_target.clone(), entry_pattern.to_string())),
                "carveout[{i}] duplicates an earlier entry on canonical path: \
                 `(path = {entry_path:?}, pattern = {entry_pattern:?})` resolves to {} which \
                 is already registered with the same pattern. Two entries claiming the same \
                 site can carry contradictory rationales — collapse them into one entry, or \
                 differentiate by lengthening one `pattern` to address a sibling site.",
                canonical_target.display()
            );

            // Per-file uniqueness: count occurrences of `pattern` as a
            // case-sensitive literal substring of the file at `path`.
            // count != 1 is the guard-test failure mode the registry's
            // addressing contract depends on. Reading via the canonical
            // path so the panic-context display matches what the
            // containment check verified above.
            let target_contents = fs::read_to_string(&canonical_target).unwrap_or_else(|e| {
                panic!(
                    "carveout[{i}] `path = {entry_path:?}` could not be read at {}: {e}",
                    canonical_target.display()
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
