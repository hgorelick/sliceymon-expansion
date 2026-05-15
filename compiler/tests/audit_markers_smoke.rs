//! Smoke test: the `InternalStateEnum` opt-out vehicle marker trait resolves at
//! its bound crate-external path `textmod_compiler::InternalStateEnum`, accepts
//! an empty-impl-block attachment on a no-derive fixture enum, and the trait's
//! declaration site `compiler/src/audit_markers.rs` carries the shape constraints
//! bound by the audit-marker chunk's Factoring Contract (decisions.md 2026-05-13
//! "Audit chunk implementation form" + decisions.md 2026-05-14 "Vehicle marker
//! semantics" + decisions.md 2026-05-15 outer-rustdoc-attestation refinements).
//!
//! If `impl_attachment_smoke_compiles` fails to compile, one of three contracts
//! is broken:
//!   (a) the trait fails to resolve at `textmod_compiler::InternalStateEnum`
//!       (the mandatory crate-root re-export `pub use audit_markers::InternalStateEnum;`
//!       in `compiler/src/lib.rs` was omitted or relocated),
//!   (b) an accidental super-trait bound rejected the no-derive fixture, or
//!   (c) the empty `impl ... for X {}` body itself fails to type-check.
//! These are the three failure modes `audit-marker-retrofit` and `audit-harness`
//! would otherwise hit at their impl-attachment time; the smoke test surfaces
//! them within this chunk's PR.

use std::fs;
use std::path::PathBuf;

#[test]
fn impl_attachment_smoke_compiles() {
    // Fixture enum carries no derives — the marker trait must not impose any
    // super-trait bound that would reject a bare enum declaration.
    enum DummyInternalState {
        A,
        B,
    }

    // Attach the marker via the canonical crate-root re-export path
    // (decisions.md 2026-05-15 point 3). If the re-export is omitted from
    // `compiler/src/lib.rs`, this impl fails to resolve and the file does not
    // compile, which is the assertion.
    impl textmod_compiler::InternalStateEnum for DummyInternalState {}

    // Anchor the impl so it is reachable from the test entry point.
    let _ = DummyInternalState::A;
    let _ = DummyInternalState::B;
}

/// Read `compiler/src/audit_markers.rs` once for the shape-guard checks below.
fn read_audit_markers_source() -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
    path.push("audit_markers.rs");
    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!("read {}: {}", path.display(), e);
    })
}

/// Strip every blank line and every line whose first non-whitespace character is
/// `/` (line comment `//`, outer doc-comment `///`, inner doc-comment `//!`).
/// Returns the residue lines. The comment-stripping step is what lets the
/// downstream checks treat `use `, `pub mod`, etc. as start-of-non-whitespace
/// code patterns without false-positives from rustdoc prose containing English
/// words like "use", "misuse", or "because".
///
/// Does NOT strip `/* ... */` block-comment continuations whose continuation
/// lines begin with ` *`. The file's bound shape uses `///` outer
/// doc-comments exclusively (no block comments today), so the limitation is
/// dormant at HEAD. A future contributor introducing a block comment to
/// `audit_markers.rs` would need to extend this helper before doing so, or
/// the residue check could either false-positive on continuation prose or
/// fail to flag a real banned item co-located with one.
fn strip_comments_and_blanks(src: &str) -> Vec<&str> {
    src.lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.is_empty() && !trimmed.starts_with('/')
        })
        .collect()
}

#[test]
fn audit_markers_residue_contains_exactly_one_trait_declaration() {
    let src = read_audit_markers_source();
    let residue = strip_comments_and_blanks(&src);
    let trait_decl_lines: Vec<&&str> = residue
        .iter()
        .filter(|line| line.trim_start().starts_with("pub trait InternalStateEnum"))
        .collect();
    assert_eq!(
        trait_decl_lines.len(),
        1,
        "audit_markers.rs residue must contain exactly one `pub trait InternalStateEnum` declaration; got {} (residue: {:?})",
        trait_decl_lines.len(),
        residue
    );
}

#[test]
fn audit_markers_residue_carries_no_other_items() {
    let src = read_audit_markers_source();
    let residue = strip_comments_and_blanks(&src);
    // Each banned pattern, when found at start-of-non-whitespace on the
    // comment-stripped residue, indicates a code item beyond the bare trait
    // declaration — `use`, `pub fn`, `pub struct`, etc. The trait-declaration
    // line itself is matched separately by the trait-count check above; here we
    // assert every banned pattern returns zero hits on the residue.
    let banned: &[&str] = &[
        "pub fn",
        "pub struct",
        "pub enum",
        "pub mod",
        "mod ",
        "use ",
        "pub use ",
        "#[derive(",
        "extern crate",
    ];
    for pattern in banned {
        let hits: Vec<&&str> = residue
            .iter()
            .filter(|line| line.trim_start().starts_with(pattern))
            .collect();
        assert!(
            hits.is_empty(),
            "audit_markers.rs residue must not contain any `{}` lines; got {} (hits: {:?})",
            pattern,
            hits.len(),
            hits
        );
    }
}

#[test]
fn audit_markers_full_file_carries_no_disallowed_std_imports() {
    let src = read_audit_markers_source();
    // SPEC §3.4 hygiene check: the audit-markers module is a leaf in the
    // library tree and must not pull in `std::fs`, `std::process`,
    // environment access, or wall-clock. The check is on the FULL file (not
    // the stripped residue) to also catch any `extern crate`-style alternative
    // routes into these symbols.
    let banned_imports: &[&str] = &[
        "std::fs",
        "std::process",
        "std::env",
        "std::time::Instant",
        "SystemTime",
    ];
    for symbol in banned_imports {
        assert!(
            !src.contains(symbol),
            "audit_markers.rs must not import or reference `{}` (SPEC §3.4 hygiene); found a reference in:\n{}",
            symbol,
            src
        );
    }
}
