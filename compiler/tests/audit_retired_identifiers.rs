//! Source-tree literal-string guards: each test greps for a retired
//! pattern marker / comment sentinel across `compiler/src/**/*.rs` and
//! asserts zero hits.
//!
//! Patterned after CLAUDE.md's "Retiring a public identifier" three-step
//! protocol but bounded to the source tree. The CLAUDE.md protocol's
//! two-surface (markdown + source) grep requirement is the contract for
//! retiring a public identifier (function, type, field, enum variant,
//! file path, CLI subcommand); the guards currently in this suite retire
//! code-level pattern markers / inline comments, not public identifiers,
//! so the markdown surface is intentionally out of scope (documentation
//! that historically quotes a retired marker is description, not
//! reintroduction). If a future retirement adds a public identifier to
//! this suite, extend the walker to cover the markdown surface per
//! CLAUDE.md step 2.

use std::fs;
use std::path::{Path, PathBuf};

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

fn collect_lib_rs_files() -> Vec<PathBuf> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = PathBuf::from(manifest_dir).join("src");
    let mut files = Vec::new();
    walk_rs(&src_dir, &mut files);
    files.sort();
    assert!(
        !files.is_empty(),
        "no .rs files found under {}",
        src_dir.display()
    );
    files
}

fn assert_literal_absent(needle: &str, files: &[PathBuf]) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut violations: Vec<String> = Vec::new();
    for path in files {
        let src = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
        let rel = path.strip_prefix(manifest_dir).unwrap_or(path);
        for (idx, line) in src.lines().enumerate() {
            if line.contains(needle) {
                violations.push(format!("{}:{}: {}", rel.display(), idx + 1, line.trim()));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "retired identifier `{}` must return zero hits under compiler/src/**/*.rs.\n\
         Offending sites ({}):\n{}",
        needle,
        violations.len(),
        violations.join("\n")
    );
}

/// `// PoolReplacement regenerator deferred.` was the inline marker for the
/// `_ => {}` wildcard sink in `regenerate_derived_kinds` before the
/// complete-regenerators chunk replaced the sink with a typed dispatch over
/// `DerivedKind`. Reintroducing the marker (or the deferred sink it
/// described) reopens the residual the chunk closed.
#[test]
fn pool_replacement_deferred_comment_retired() {
    let files = collect_lib_rs_files();
    assert_literal_absent("// PoolReplacement regenerator deferred.", &files);
}
