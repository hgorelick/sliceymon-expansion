//! SPEC §F8 / §8 audit: lib code must not contain `.unwrap()`, `.expect(...)`,
//! `panic!(...)`, `unimplemented!(...)`, or `todo!(...)` outside of
//! `#[cfg(test)]` / `#[test]` blocks.
//!
//! This is an integration test (not a unit test) so it can walk the source
//! tree rooted at `CARGO_MANIFEST_DIR` — unit tests inside `compiler/src/**`
//! cannot reliably grep the workspace at runtime.
//!
//! If this test fails, the fix is to replace the offending site with `?`
//! propagation returning `CompilerError`. See `plans/PLATFORM_FOUNDATIONS_PLAN.md`
//! §F8 for the full policy.

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

/// Strip `#[cfg(test)]` / `#[test]` items from `src`. Returns `(line_number, text)`
/// pairs for lines OUTSIDE test gates. `line_number` is 1-based.
fn lines_outside_test_gates(src: &str) -> Vec<(usize, &str)> {
    let lines: Vec<&str> = src.lines().collect();
    let n = lines.len();
    let mut kept = vec![true; n];

    let mut i = 0;
    while i < n {
        let trimmed = lines[i].trim_start();
        let is_test_gate = trimmed.starts_with("#[cfg(test)]")
            || trimmed.starts_with("#[test]")
            || (trimmed.starts_with("#[cfg(") && trimmed.contains("test"));
        if is_test_gate {
            // Find the first `{` on this or a subsequent line, then brace-match to its end.
            let mut j = i;
            while j < n && !lines[j].contains('{') {
                j += 1;
            }
            if j >= n {
                i += 1;
                continue;
            }
            let mut depth: i32 =
                (lines[j].matches('{').count() as i32) - (lines[j].matches('}').count() as i32);
            let start = i;
            let mut k = j + 1;
            while k < n && depth > 0 {
                depth += lines[k].matches('{').count() as i32
                    - lines[k].matches('}').count() as i32;
                k += 1;
            }
            for m in start..k {
                kept[m] = false;
            }
            i = k;
            continue;
        }
        i += 1;
    }

    (0..n).filter(|idx| kept[*idx]).map(|idx| (idx + 1, lines[idx])).collect()
}

fn contains_forbidden(line: &str) -> bool {
    // Mirror the rg pattern from plan §F8:
    //   \.unwrap\(\)|\.expect\(|panic!\(|unimplemented!|todo!\(
    line.contains(".unwrap()")
        || line.contains(".expect(")
        || line.contains("panic!(")
        || line.contains("unimplemented!(")
        || line.contains("todo!(")
}

#[test]
fn audit_no_lib_panic_or_unwrap() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = PathBuf::from(manifest_dir).join("src");

    let mut files = Vec::new();
    walk_rs(&src_dir, &mut files);
    files.sort();
    assert!(!files.is_empty(), "no .rs files found under {}", src_dir.display());

    let mut violations: Vec<String> = Vec::new();
    for path in &files {
        let src = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
        for (ln, text) in lines_outside_test_gates(&src) {
            if contains_forbidden(text) {
                let rel = path.strip_prefix(manifest_dir).unwrap_or(path);
                violations.push(format!("{}:{}: {}", rel.display(), ln, text.trim()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "SPEC §F8 violation: lib code must not contain .unwrap()/.expect()/panic!()/unimplemented!()/todo!() outside #[cfg(test)] blocks.\n\
         Replace each with ? propagation returning CompilerError.\n\n\
         Offending sites ({}):\n{}",
        violations.len(),
        violations.join("\n")
    );
}
