//! CLAUDE.md "Retiring a public identifier" guard suite: literal-string
//! greps over `compiler/src/**/*.rs` asserting that retired identifiers,
//! retired comment markers, and retired sentinel prose return zero hits
//! across the library source tree.
//!
//! Each guard pins the third step of CLAUDE.md's three-step retirement
//! protocol (the rest — retirement comment dated to the chunk + atomic
//! removal of every doc reference — is enforced at chunk-land time; this
//! suite is the permanent CI floor that prevents reintroduction).

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
