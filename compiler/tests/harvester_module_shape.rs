//! Module-shape guard test for the harvester pure core.
//!
//! Walks every `.rs` file under `compiler/src/harvester/` recursively and
//! enforces the introduction-side hygiene floor bound by the
//! `harvester-pure-core` chunk plan's §Tests to add "Module-shape guard
//! test" section. The recurring-CI lexical denylist (with the
//! `compiler/build.rs` allowlist) is `harvester-determinism-audit`'s scope;
//! this test is the at-introduction guard that locks in the floor at this
//! chunk's PR.
//!
//! Two scopes:
//!   - Residue scope: comment-and-blanks-stripped lines; bans `use ` (except
//!     `use std::` / `use core::`), `pub use ` (except the two licensed
//!     re-export shapes), `use crate::` / `use super::` / `use textmod_compiler::`,
//!     `extern crate`, `pub mod`, `pub fn` (except the two entry-point fns),
//!     `pub struct` / `pub enum` / `pub const` / `pub static` / `pub trait`.
//!   - Full-file scope: scanned unstripped, catching banned tokens even if
//!     placed inside string literals or doc-comments; bans the wider WASM-
//!     readiness OS-services denylist plus `extern crate ` plus `crate::`
//!     path-prefix.

use std::fs;
use std::path::{Path, PathBuf};

/// Recursive walker mirroring `compiler/tests/audit_lib_panic_free.rs`'s
/// `walk_rs` helper — collect every `.rs` file under `dir`.
fn walk_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("read_dir {}: {}", dir.display(), e)) {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        let file_type = entry.file_type().expect("file_type");
        if file_type.is_dir() {
            walk_rs(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

fn harvester_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("src");
    p.push("harvester");
    p
}

/// Strip every blank line and every line whose first non-whitespace
/// character is `/` (line comment `//`, outer doc-comment `///`, inner
/// doc-comment `//!`). Mirrors the sibling `audit_markers_smoke.rs`
/// helper so residue checks treat `use `, `pub mod`, etc. as start-of-
/// non-whitespace code patterns without false-positives from doc prose.
fn strip_comments_and_blanks(src: &str) -> Vec<&str> {
    src.lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.is_empty() && !trimmed.starts_with('/')
        })
        .collect()
}

/// Per the chunk plan's §Contracts changed: `pub use` re-exports of the
/// two entry-point function names are licensed in two shapes (one
/// decomposition per file, not both). Each shape matches a tight regex;
/// any other `pub use ` line is banned.
///
/// Shape (a) — single-sub-module curly-set form:
///   `pub use <sub_module>::{harvest_face_ids, harvest_sprites};`
///   (either function-name ordering admitted within the curly set)
/// Shape (b) — per-function single-target form:
///   `pub use <sub_module>::harvest_face_ids;`
///   `pub use <other_sub_module>::harvest_sprites;`
///
/// `<sub_module>` is a standard Rust identifier `[a-zA-Z_][a-zA-Z0-9_]*`.
fn pub_use_line_is_licensed(line: &str) -> bool {
    let trimmed = line.trim();
    // Shape (a): pub use IDENT::{A, B};
    if let Some(rest) = trimmed.strip_prefix("pub use ") {
        if let Some(inside) = rest.strip_suffix("};") {
            if let Some((module, set)) = inside.split_once("::{") {
                if is_ident(module) {
                    // The set must contain exactly `harvest_face_ids` and
                    // `harvest_sprites` in either order, separated by `,` and
                    // optional single whitespace.
                    let canonical: Vec<&str> = set
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if canonical.len() == 2 {
                        let mut sorted = canonical.clone();
                        sorted.sort();
                        if sorted == ["harvest_face_ids", "harvest_sprites"] {
                            return true;
                        }
                    }
                }
            }
        }
        // Shape (b): pub use IDENT::harvest_face_ids; / pub use IDENT::harvest_sprites;
        if let Some(inside) = rest.strip_suffix(';') {
            if let Some((module, ident)) = inside.split_once("::") {
                if is_ident(module)
                    && (ident == "harvest_face_ids" || ident == "harvest_sprites")
                {
                    return true;
                }
            }
        }
    }
    false
}

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[test]
fn harvester_directory_exists_with_at_least_one_source_file() {
    // Establishes the test's load-bearing precondition: the chunk creates the
    // `compiler/src/harvester/` directory. Without this assertion the residue
    // and full-file scope checks below would pass vacuously on an empty file
    // set if the chunk regressed the relocation.
    let dir = harvester_dir();
    let mut files: Vec<PathBuf> = Vec::new();
    walk_rs(&dir, &mut files);
    assert!(
        !files.is_empty(),
        "compiler/src/harvester/ must contain at least one .rs file; got 0",
    );
}

#[test]
fn harvester_residue_scope_has_no_banned_patterns() {
    let dir = harvester_dir();
    let mut files: Vec<PathBuf> = Vec::new();
    walk_rs(&dir, &mut files);
    for path in &files {
        let src = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
        let residue = strip_comments_and_blanks(&src);
        for (idx, raw_line) in residue.iter().enumerate() {
            let trimmed = raw_line.trim_start();

            // Ban `use ` UNLESS `use std::` / `use core::`.
            if trimmed.starts_with("use ")
                && !trimmed.starts_with("use std::")
                && !trimmed.starts_with("use core::")
            {
                panic!(
                    "{}:{}: banned `use` reach in residue (only `use std::` / `use core::` admitted): {:?}",
                    path.display(),
                    idx,
                    raw_line
                );
            }

            // Ban `pub use ` UNLESS it matches one of the two licensed re-export shapes.
            if trimmed.starts_with("pub use ") && !pub_use_line_is_licensed(trimmed) {
                panic!(
                    "{}:{}: banned `pub use` shape (only the two licensed re-export forms admitted): {:?}",
                    path.display(),
                    idx,
                    raw_line
                );
            }

            // Ban `extern crate ` (closes the Layer 3 gap on phf_codegen reach via 2015-edition style).
            if trimmed.starts_with("extern crate ") {
                panic!(
                    "{}:{}: `extern crate` is banned in compiler/src/harvester/ (closes the Layer 3 phf_codegen reach gap): {:?}",
                    path.display(),
                    idx,
                    raw_line
                );
            }

            // Ban `pub mod ` (sub-modules MUST be mod-private per §Contracts changed).
            if trimmed.starts_with("pub mod ") {
                panic!(
                    "{}:{}: `pub mod` is banned in compiler/src/harvester/ (sub-modules MUST be mod-private): {:?}",
                    path.display(),
                    idx,
                    raw_line
                );
            }

            // Ban `pub fn ` other than the two entry-point function names.
            if trimmed.starts_with("pub fn ")
                && !trimmed.starts_with("pub fn harvest_face_ids")
                && !trimmed.starts_with("pub fn harvest_sprites")
            {
                panic!(
                    "{}:{}: banned `pub fn` (only `pub fn harvest_face_ids` / `pub fn harvest_sprites` admitted): {:?}",
                    path.display(),
                    idx,
                    raw_line
                );
            }

            // Ban `pub struct` / `pub enum` / `pub const` / `pub static` / `pub trait`
            // (none licensed by §Contracts changed).
            for prefix in ["pub struct ", "pub enum ", "pub const ", "pub static ", "pub trait "] {
                if trimmed.starts_with(prefix) {
                    panic!(
                        "{}:{}: `{}` is banned in compiler/src/harvester/ (no licensed pub item beyond the two entry-point fns): {:?}",
                        path.display(),
                        idx,
                        prefix.trim_end(),
                        raw_line
                    );
                }
            }
        }
    }
}

#[test]
fn harvester_full_file_scope_has_no_banned_patterns() {
    let dir = harvester_dir();
    let mut files: Vec<PathBuf> = Vec::new();
    walk_rs(&dir, &mut files);

    // Engineering-plan-bound denylist + chunk-introduction-side floors.
    // Full-file scope; deliberate strictness — string-literal hits are
    // banned too (today's emitter output strings do not name these
    // symbols, so the scan does not false-positive on emitted byte
    // content).
    let banned_substrings: &[&str] = &[
        // Engineering-plan-bound denylist.
        "std::time::",
        "std::env::",
        "std::fs::",
        "OnceLock",
        "OnceCell",
        "lazy_static!",
        "once_cell::",
        "fs::read_dir",
        "phf_codegen::",
        // SPEC §3.4 binds 'no std::process' as library-side hygiene.
        "std::process::",
        // Decisions.md 2026-05-08 binds 'no Path arguments'.
        "std::path::Path",
        "std::path::PathBuf",
        "AsRef<Path>",
        "Path::new(",
        // Macros that std::env::* prefix scan misses.
        "env!(",
        "option_env!(",
        // WASM-readiness floors.
        "std::thread::",
        "std::sync::mpsc::",
        "std::net::",
        "std::io::Stdin",
        "std::io::Stdout",
        "std::io::Stderr",
        "std::io::stdin",
        "std::io::stdout",
        "std::io::stderr",
        // Layer-3 gap closure (parallels the residue-scope ban).
        "extern crate ",
        // Cross-module reach bans (full-file catches commented-out attempts too).
        "use crate::",
        "use super::",
        "use textmod_compiler::",
    ];

    for path in &files {
        let src = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));

        for needle in banned_substrings {
            if src.contains(needle) {
                let offending_line = src
                    .lines()
                    .enumerate()
                    .find(|(_, l)| l.contains(needle))
                    .map(|(i, l)| format!("line {}: {:?}", i + 1, l))
                    .unwrap_or_else(|| "<unknown line>".to_string());
                panic!(
                    "{}: banned substring `{}` found in full-file scan ({})",
                    path.display(),
                    needle,
                    offending_line
                );
            }
        }

        // `unsafe ` / `unsafe{` — newline carve-out: substring `unsafe ` would
        // also match the word inside doc-comment prose. Apply only at start-of-
        // line on the trimmed-start view to mirror the chunk plan's full-file
        // scope intent (a deliberate strictness floor over rust syntax).
        for (idx, line) in src.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("unsafe ") || trimmed.starts_with("unsafe{") {
                panic!(
                    "{}:{}: `unsafe` is banned in compiler/src/harvester/: {:?}",
                    path.display(),
                    idx + 1,
                    line
                );
            }
        }

        // `crate::` path-prefix at full-file scope. Boundary-anchored to avoid
        // false-matching identifiers like `iterate::`. Doc-comment carriers
        // ARE scanned (deliberate strictness per the sibling
        // `audit_markers_full_file_carries_no_disallowed_std_imports` floor);
        // implementer-side: paraphrase doc-comments rather than writing
        // literal `crate::` substrings.
        let bytes = src.as_bytes();
        let needle = b"crate::";
        let mut i = 0;
        while i + needle.len() <= bytes.len() {
            if &bytes[i..i + needle.len()] == needle {
                let prev_is_ident = i > 0
                    && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
                if !prev_is_ident {
                    let line_no = src[..i].bytes().filter(|b| *b == b'\n').count() + 1;
                    panic!(
                        "{}:{}: `crate::` path-prefix is banned in compiler/src/harvester/ (boundary-anchored)",
                        path.display(),
                        line_no
                    );
                }
            }
            i += 1;
        }
    }
}
