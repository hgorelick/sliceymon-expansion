//! The 2026-04-22 "library code panic-free" ruling / SPEC §8 audit: lib code
//! must not contain `.unwrap()`, `.expect(...)`, `panic!(...)`,
//! `unimplemented!(...)`, or `todo!(...)` outside of `#[cfg(test)]` /
//! `#[test]` blocks.
//!
//! This is an integration test (not a unit test) so it can walk the source
//! tree rooted at `CARGO_MANIFEST_DIR` — unit tests inside `compiler/src/**`
//! cannot reliably grep the workspace at runtime.
//!
//! If this test fails, the fix is to replace the offending site with `?`
//! propagation returning `CompilerError`. The ruling is the full policy.

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
///
/// Supported test-gate shapes (file `path` is passed only for error messages):
///   - `#[cfg(test)]` / `#[test]` / `#[cfg(... test ...)]` immediately followed
///     (same line or any number of blank/attribute lines later) by a `{`-opened
///     block item (`mod NAME {`, `fn NAME() {`, `impl ... {`, etc.). The stripper
///     brace-matches to the closing `}` and excludes the whole span.
///
/// Rejected shapes — the stripper panics rather than silently guess, so future
/// drift fails loudly:
///   - Single-line items between the attribute and the next `{` (e.g. an
///     intervening `use foo;` or `const N: u32 = 1;`). These would cause the
///     brace-match to latch onto the NEXT item's `{` (which belongs to non-
///     test code) and silently strip production code — defeating the
///     2026-04-22 "library code panic-free" ruling.
///   - `#[cfg(not(test))]` — the `contains("test")` heuristic would misfire;
///     today no such attribute exists in `src/`, but if one is introduced the
///     audit panics instead of falsely test-gating production code.
fn lines_outside_test_gates<'a>(src: &'a str, path: &str) -> Vec<(usize, &'a str)> {
    let lines: Vec<&str> = src.lines().collect();
    let n = lines.len();
    let mut kept = vec![true; n];

    let mut i = 0;
    while i < n {
        let trimmed = lines[i].trim_start();
        if trimmed.starts_with("#[cfg(not(test))]") {
            panic!(
                "{}:{}: `#[cfg(not(test))]` is unsupported by the audit stripper — the \
                 `starts_with(\"#[cfg(\") && contains(\"test\")` heuristic would falsely \
                 treat it as a test gate and strip non-test code. Replace with an explicit \
                 `#[cfg(any(not(test)))]` or rework the audit.",
                path,
                i + 1
            );
        }
        let is_test_gate = trimmed.starts_with("#[cfg(test)]")
            || trimmed.starts_with("#[test]")
            || (trimmed.starts_with("#[cfg(") && trimmed.contains("test"));
        if is_test_gate {
            // Walk forward from the attribute line until we find a `{` that opens
            // the test item's block. Every line in between must be part of the
            // same item declaration — attribute lines (`#[...]`), continuation
            // of a multi-line signature, or the `mod`/`fn`/`impl` keyword — and
            // NOT a complete single-line item ending in `;`. A single-line item
            // here would mean the `#[cfg(test)]` gates something like `use foo;`
            // or `const N: u32 = 1;`, and the next `{` belongs to the FOLLOWING
            // non-test item — brace-matching that would silently strip
            // production code. Panic instead so drift fails loudly.
            let mut j = i;
            while j < n && !lines[j].contains('{') {
                let t = lines[j].trim();
                // Permit: blank lines, attribute lines (`#[...]`), doc comments,
                // line comments, and `//`-style notes between the attribute and
                // the block-open. Anything else that ends with `;` is a single-
                // line item we cannot handle.
                let is_permitted_between =
                    t.is_empty()
                        || t.starts_with("#[")
                        || t.starts_with("//")
                        || t.starts_with("/*")
                        || t.starts_with("*")
                        || (j == i); // the attribute line itself
                if !is_permitted_between && t.ends_with(';') {
                    panic!(
                        "{}:{}: audit stripper cannot handle single-line `#[cfg(test)]` / \
                         `#[test]` items (line ends with `;` before any `{{` is found). \
                         The stripper would latch onto the following item's `{{` and \
                         silently strip production code — defeating the 2026-04-22 \"library code panic-free\" ruling. Wrap the \
                         test-gated item in a `#[cfg(test)] mod ... {{ ... }}` block, or \
                         extend the stripper to recognize the new shape.",
                        path,
                        i + 1
                    );
                }
                j += 1;
            }
            if j >= n {
                // Attribute line with no following `{` (e.g. at EOF with a stray
                // comment). Nothing to strip; advance past the attribute.
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
    // Mirror the rg pattern from the 2026-04-22 "library code panic-free" ruling:
    //   \.unwrap\(\)|\.expect\(|panic!\(|unimplemented!|todo!\(
    line.contains(".unwrap()")
        || line.contains(".expect(")
        || line.contains("panic!(")
        || line.contains("unimplemented!(")
        || line.contains("todo!(")
}

#[test]
#[should_panic(expected = "audit stripper cannot handle single-line")]
fn stripper_panics_on_cfg_test_use_item() {
    // R3 regression: a `#[cfg(test)] use foo;` pattern between `#[cfg(test)]`
    // and the next item's `{` would cause the stripper to latch onto the
    // FOLLOWING (non-test) item's brace and silently strip production code.
    // The stripper must panic instead.
    let src = "#[cfg(test)]\nuse some_panicky_helper;\n\nfn production_fn() {\n    panic!(\"oh no\");\n}\n";
    let _ = lines_outside_test_gates(src, "synthetic.rs");
}

#[test]
#[should_panic(expected = "audit stripper cannot handle single-line")]
fn stripper_panics_on_cfg_test_const_item() {
    // Same class of drift, const variant.
    let src = "#[cfg(test)]\nconst TEST_ONLY: u32 = 1;\n\nfn production() { panic!(\"leaks\"); }\n";
    let _ = lines_outside_test_gates(src, "synthetic.rs");
}

#[test]
#[should_panic(expected = "`#[cfg(not(test))]` is unsupported")]
fn stripper_panics_on_cfg_not_test() {
    // The `starts_with("#[cfg(") && contains("test")` heuristic would falsely
    // treat `#[cfg(not(test))]` as a test gate and strip production code.
    let src = "#[cfg(not(test))]\nmod production_only {\n    fn leaks() { panic!(); }\n}\n";
    let _ = lines_outside_test_gates(src, "synthetic.rs");
}

#[test]
fn stripper_strips_standard_mod_tests_block() {
    // Positive: the normal `#[cfg(test)] mod tests { ... }` shape is stripped
    // cleanly; production code above/below is preserved.
    let src = "fn keep_me() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn inner() { panic!(\"stripped\"); }\n}\n\nfn also_keep_me() {}\n";
    let kept: Vec<&str> = lines_outside_test_gates(src, "synthetic.rs")
        .into_iter()
        .map(|(_, t)| t)
        .collect();
    let joined = kept.join("\n");
    assert!(joined.contains("fn keep_me"), "pre-test production preserved");
    assert!(joined.contains("fn also_keep_me"), "post-test production preserved");
    assert!(!joined.contains("stripped"), "test-block body stripped");
}

#[test]
fn stripper_allows_attribute_and_comment_lines_between_gate_and_brace() {
    // `#[cfg(test)]` followed by doc comments and additional attributes before
    // the `mod tests {` line is a normal shape in real code — must NOT panic.
    let src = "#[cfg(test)]\n// A comment about the tests below.\n#[allow(clippy::pedantic)]\nmod tests {\n    fn inner() {}\n}\n";
    let _ = lines_outside_test_gates(src, "synthetic.rs");
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
        let rel = path.strip_prefix(manifest_dir).unwrap_or(path);
        let rel_str = rel.display().to_string();
        for (ln, text) in lines_outside_test_gates(&src, &rel_str) {
            if contains_forbidden(text) {
                violations.push(format!("{}:{}: {}", rel_str, ln, text.trim()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "2026-04-22 \"library code panic-free\" ruling violation: lib code must not contain .unwrap()/.expect()/panic!()/unimplemented!()/todo!() outside #[cfg(test)] blocks.\n\
         Replace each with ? propagation returning CompilerError.\n\n\
         Offending sites ({}):\n{}",
        violations.len(),
        violations.join("\n")
    );
}
