//! Smoke tests for SPEC amendments that Chunk 2 made.
//!
//! These tests exist to prevent silent rollback of SPEC wording that
//! Chunk 2 of `plans/PLATFORM_FOUNDATIONS_PLAN.md` required — the
//! permissive-whitelist ruling (SPEC §3.6) and the `Pips: i16` annotation.

use std::fs;
use std::path::PathBuf;

fn read_spec() -> String {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("compiler/ has a parent")
        .to_path_buf();
    fs::read_to_string(repo_root.join("SPEC.md"))
        .expect("SPEC.md must exist at repo root")
}

#[test]
fn spec_section_3_6_mentions_unknown_variant() {
    let spec = read_spec();
    assert!(
        spec.contains("### 3.6 Make invalid states unrepresentable"),
        "SPEC §3.6 heading must still exist"
    );
    assert!(
        spec.contains("Unknown(raw)"),
        "SPEC §3.6 must document the `Unknown(raw)` escape hatch for \
         corpus-derived whitelists (Chunk 2 amendment)"
    );
    assert!(
        spec.contains("Severity::Warning"),
        "SPEC §3.6 must name the warning severity for unknown-whitelist values"
    );
}

#[test]
fn spec_section_3_6_pips_is_i16() {
    let spec = read_spec();
    // Pre-Chunk 2 wording was `pips: u8`; amended to `Pips` newtype around `i16`.
    assert!(
        !spec.contains("pips: u8"),
        "SPEC §3.6 must not retain the old `pips: u8` annotation"
    );
    assert!(
        spec.contains("i16"),
        "SPEC §3.6 must annotate pips as `i16` (corpus contains negative pips)"
    );
}
