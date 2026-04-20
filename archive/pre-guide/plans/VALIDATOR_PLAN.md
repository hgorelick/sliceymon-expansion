# Plan: Textmod Validator in Rust

## Context

The compiler can extract, build, and round-trip textmods, but has no way to guarantee a built textmod will work in-game. A recent test paste showed 29/120 failures with no way to diagnose which modifiers failed or why. We need a validator that gives 100% confidence before pasting.

Rules are derived from:
1. Structural analysis of 3 known-working mods (pansaer, punpuns, sliceymon -- 243 modifiers total)
2. The in-game TextMod API reference (Ledger > TextMod screenshots)
3. Face ID tables from SLICEYMON_AUDIT.md

## What Gets Built

New `compiler/src/validator.rs` module with `validate(textmod: &str) -> Result<ValidationReport, CompilerError>`.

Returns `Result` because splitting can fail with a hard `CompilerError::SplitError` (e.g., globally unmatched parens) that prevents any further validation. All other issues are reported as `Finding`s inside `ValidationReport`.

**WASM compatibility**: `validator.rs` is a library module. It MUST NOT use `std::fs` or any platform-specific APIs. All I/O happens in `main.rs`.

## Out of Scope

- Auto-fixing detected issues (separate tool)
- Validating sprite image data correctness (only checks `.img.` is non-empty when present)
- Validating `.speech.` content
- Validating `.abilitydata.` internal structure (only checked via paren balance)
- Performance optimization (243 modifiers validates in <1s; not needed)
- JSON output mode (Display impl is sufficient for v1; serde enables JSON later)
- Moving `split_modifiers` / `classify` to `util.rs` (acknowledged coupling, separate refactor)

## Implementation Order

This is a single-chunk plan (4 files, single concern). Execute sequentially -- no parallelism needed.

1. Create `compiler/src/validator.rs` -- all validation logic
2. Edit `compiler/src/lib.rs` -- add `pub mod validator;` + re-export
3. Edit `compiler/src/main.rs` -- replace Validate command body, add `--round-trip` flag
4. Create `compiler/tests/validator_tests.rs` -- integration + unit tests
5. Run `cargo test` and `cargo clippy -- -D warnings`
6. Run CLI against all 3 working mods -- verify 0 errors each

**Checkpoint**: Single chunk = single checkpoint after step 6.

## If Blocked

- **If a working mod produces false-positive errors**: The rule is wrong, not the mod. All 3 working mods are ground truth. Comment out the offending rule check, add `// TODO: rule EXXX fires on {mod_name}, needs investigation`, and continue. Report the false positive in the checkpoint summary.
- **If `split_modifiers` error format changes**: The validator propagates `CompilerError` via `?`. If the variant changes, `?` still works -- no special handling needed.
- **If `classify` returns `Err` for a modifier**: Treat as `ModifierType::Unknown` and emit W003. Use `.unwrap_or(ModifierType::Unknown)`. Do NOT propagate the error.
- **If a unit test input triggers unexpected rules beyond the target**: Use minimal modifier strings that only trigger the target rule. If unavoidable, assert the target rule is present (`.any(|f| f.rule_id == "EXXX")`) rather than asserting exact error count.
- **If `extract_hp` returns `None` for a tier**: Not an error (`.hp.` is absent in 2-5% of tiers per the Non-Rules). Skip E010 for that tier.

## Files to Read First

| File | Why |
|------|-----|
| `compiler/src/util.rs` | All reusable functions -- verify signatures match plan |
| `compiler/src/extractor/splitter.rs` | `split_modifiers` signature and error type |
| `compiler/src/extractor/classifier.rs` | `classify` signature, `ModifierType` enum, `ModifierType::Unknown` |
| `compiler/src/error.rs` | `CompilerError` variants, especially `ValidationError` and `SplitError` |
| `compiler/src/lib.rs` | Current module declarations and re-exports to extend |
| `compiler/src/main.rs` | Current `Commands::Validate` body and CLI arg structure |
| `compiler/tests/roundtrip_tests.rs` | Pattern for loading working-mod files in tests (`../working-mods/`) |

## Constraints

- **Use ONLY** util.rs functions listed in the Reuse section -- do NOT call any parser (`hero_parser`, `capture_parser`, etc.)
- **Use ONLY** `ModifierType::Hero` and `ModifierType::Unknown` for branching -- other variants are irrelevant to the validator
- **Do NOT** use `unwrap()` or `panic!()` anywhere in `validator.rs` -- return findings for all errors, never crash
- **Do NOT** use `std::fs` or any I/O in `validator.rs` -- pure library module (WASM-compatible)
- **Do NOT** add new dependencies to `Cargo.toml` -- `serde`, `serde_json`, `clap` already available
- **Do NOT** validate inside nested parens (e.g., `.abilitydata.(...)` internals)
- **Do NOT** use the `regex` crate -- parse face entries with `str` methods and manual character checks
- **Do NOT** validate `#suffix` content on face entries -- it is free-form game metadata after the numeric portion

## Validation Rules

### Fatal Errors (game will reject)

| ID | Rule | Evidence |
|----|------|----------|
| E001 | Parentheses balanced per modifier (depth >= 0, ends at 0) | 243/243 mods balanced |
| E002 | ASCII only (no chars > 127) | 0 non-ASCII in 868K chars |
| E003 | Unique `.mn.` names (among modifiers that have `.mn.`) | 0 duplicates across all mods |
| E004 | ph.b hero prefix matches `hidden&temporary&ph.b[a-z]+;\d+;!mheropool.\(` | 47/47 sliceymon |
| E005 | ph.b hero suffix ends `.part.1&hidden.mn.NAME@2!m(skip&hidden&temporary)` | 47/47 |
| E006 | Hero has >= 3 tier parts (T1 + at least 2 evolution paths). Note: the "Explicit Non-Rules" entry about tier count (Eevee=17, Ditto=27) refers to HIGH counts being valid, not low counts. No known working hero has < 3 parts. | 47/47 |
| E007 | Each tier block opens with `(replica.` and has matching `)` | 100% |
| E008 | Face entries in `.sd.` match: bare `N`, `N-N`, `N--N`, `N-N#suffix` | 100% |
| E009 | Face IDs in range 0-187 | All 3 mods |
| E010 | `.hp.` value is digits, range 1-999 | All 3 mods |
| E011 | `.col.` value is single lowercase letter | All 3 mods |
| E012 | `.tier.` value is single digit 0-9 | All 3 mods |

### Warnings (suspicious but may work)

| ID | Rule |
|----|------|
| W001 | Hero `.sd.` has < 4 or > 6 faces |
| W002 | Pip value > 25 on hero dice |
| W003 | Modifier classifies as Unknown type |
| W004 | `.img.` data empty when present |
| W005 | Modifier lacks depth-0 `.mn.` |

### Explicit Non-Rules (do NOT validate)

- Property order within tier blocks (free-form between `)` and `.n.`)
- Exact face count (1-7 all valid)
- `.n.NAME` position relative to `+` (not always last)
- Any single property required in every tier (even `.sd.` absent 2-5%)
- Specific tier count (Eevee=17, Ditto=27 are valid)

## Types

```rust
/// A single validation finding (error, warning, or info).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub rule_id: String,            // "E001", "W003", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_name: Option<String>,  // .mn. value, for human-readable output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,    // ~40 chars surrounding the error position
    pub message: String,
}

/// Full validation report -- errors, warnings, and informational notes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationReport {
    pub errors: Vec<Finding>,
    pub warnings: Vec<Finding>,
    pub info: Vec<Finding>,
}

impl ValidationReport {
    /// Returns true if there are zero errors (warnings are OK).
    pub fn is_ok(&self) -> bool { self.errors.is_empty() }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_ok() {
            writeln!(f, "Validation PASSED: 0 errors, {} warnings", self.warnings.len())?;
        } else {
            writeln!(f, "Validation FAILED: {} errors, {} warnings",
                self.errors.len(), self.warnings.len())?;
        }
        for finding in &self.errors {
            write!(f, "  ERROR [{}]", finding.rule_id)?;
            if let Some(ref name) = finding.modifier_name {
                write!(f, " {}", name)?;
            }
            if let Some(idx) = finding.modifier_index {
                write!(f, " (modifier #{})", idx)?;
            }
            writeln!(f, ": {}", finding.message)?;
            if let Some(ref ctx) = finding.context {
                writeln!(f, "         context: \"...{}...\"", ctx)?;
            }
        }
        for finding in &self.warnings {
            write!(f, "  WARN  [{}]", finding.rule_id)?;
            if let Some(ref name) = finding.modifier_name {
                write!(f, " {}", name)?;
            }
            if let Some(idx) = finding.modifier_index {
                write!(f, " (modifier #{})", idx)?;
            }
            writeln!(f, ": {}", finding.message)?;
        }
        Ok(())
    }
}
```

Note on `rule_id: String`: Using `String` instead of `&'static str` to support `#[derive(Deserialize)]` for WASM JSON round-tripping. Rule ID constants are defined as `const` strings in the module (e.g., `const E001: &str = "E001";`) and `.to_string()` is called when constructing `Finding` values.

Note on `context: Option<String>`: For positional errors (E001, E008, E009), populate with ~20 chars before and after the error position. This mirrors the existing `CompilerError::ParenError.context` field and is essential for debugging paste failures where index alone is insufficient.

## Validation Phases

Phases execute in order. Each phase depends on the prior phase completing.

1. **Global**: ASCII check (E002) on full text. Note: E002 findings have `position` (byte offset in full text) but `modifier_index` and `modifier_name` are `None` since splitting hasn't happened yet. This is acceptable -- the global position is sufficient to locate the offending character.
2. **Split**: Split into modifiers via `split_modifiers`. If this returns `Err`, propagate as `CompilerError` (not a `Finding`) -- the textmod is too broken to analyze further.
3. **Per-modifier**: Paren balance (E001), `.mn.` extraction (for E003 and `modifier_name` on all subsequent findings)
4. **Hero-specific**: For hero-classified modifiers -- prefix (E004), suffix (E005), tier structure (E006, E007), face/dice validation (E008, E009), property checks (E010-E012)
5. **Cross-modifier**: Duplicate `.mn.` (E003), unknown type (W003)

## Reuse (from util.rs)

- `verify_ascii_only` -- E002 (reuse directly, message is sufficient)
- `extract_mn_name` -- E003 and populating `modifier_name` fields. Note: uses `rfind` (not depth-aware), which is correct for heroes where the depth-0 `.mn.` is the final one in the suffix. For non-hero modifiers this is also fine since their `.mn.` is typically at depth 0.
- `split_at_depth0` -- E006 tier splitting (split on `+`)
- `find_at_depth0`, `find_matching_close_paren` -- E007 tier block detection
- `extract_hp` -- E010 hp extraction (reuse, then range-check the result)
- `extract_color` -- E011 color extraction (reuse, presence = valid)

**Cannot reuse (need custom logic in validator):**

- `verify_paren_balance` -- Returns `Result<(), String>` with only a message. E001 needs a `Finding` with `position` populated. Write a `check_paren_balance` in validator.rs that tracks position and returns `Option<Finding>`.
- `extract_sd` -- Only matches `[0-9:-]` characters. Face entries can contain `#suffix` after the numeric portion (e.g., `76-1)#triggerhpdata`, `140-4#k`, `46-1#topbot`). The `#` attaches game metadata to individual face slots. E008/E009 need a custom approach. Implementation strategy:
  1. Find `.sd.` at the appropriate depth (reuse `find_at_depth0`).
  2. Read the numeric face data using the same `[0-9:-]` char class as `extract_sd` (stopping at `#`, `.`, `)`, etc.).
  3. Split the numeric portion on `:` to get individual face entries.
  4. Parse each entry: valid forms are bare `N` (blank = `0`), `N-N` (FaceID-Pips), `N--N` (FaceID with negative pips).
  5. Validate Face IDs are in range 0-187 (E009) and format is correct (E008).
  The `#suffix` content AFTER the numeric portion is NOT validated by E008/E009 -- it is free-form game metadata.
  Implement as `parse_face_entry(entry: &str) -> Result<(u16, Option<i16>), String>` in validator.rs (note `i16` for pips to handle negative values via `--`). No regex crate needed.
- `extract_tier` -- No existing util function for `.tier.` extraction. Write a small `extract_tier(content: &str) -> Option<u8>` in validator.rs.

From `extractor/` (accepted cross-module dependency):
- `splitter::split_modifiers` -- phase 2 splitting. Pure function on `&str`, no IR knowledge.
- `classifier::classify(modifier, index)` -- W003 (note: takes `&str` and `usize` index; index is unused internally but required by signature). Pure function on `&str`, no IR knowledge.

Architecture note: The validator depends on two `extractor/` functions (`split_modifiers`, `classify`). This is an accepted trade-off: both are pure text-analysis functions that operate on `&str` with no IR coupling. Moving them to `util.rs` would be cleaner but is a larger refactor outside this plan's scope. The validator does NOT depend on any parser (hero_parser, capture_parser, etc.) or on IR types.

## Files

| Action | File |
|--------|------|
| Create | `compiler/src/validator.rs` (~350-450 lines) |
| Create | `compiler/tests/validator_tests.rs` |
| Edit | `compiler/src/lib.rs` -- add `pub mod validator;` and re-export: `pub use validator::{validate, ValidationReport, Finding};` |
| Edit | `compiler/src/main.rs` -- replace `Commands::Validate` body with call to `validator::validate`, print report via `Display`. Add `--round-trip` flag that runs the old extract-build-extract IR comparison after validation passes. This preserves the existing round-trip fidelity check as an opt-in behavior. |

## Tests

### Integration tests (working mods produce 0 errors)

Tests load files via `std::fs::read_to_string("../working-mods/{name}.txt")` following the pattern in `roundtrip_tests.rs`.

1. `validate_pansaer_zero_errors` -- `validate(pansaer)` returns `is_ok() == true`, 0 errors
2. `validate_punpuns_zero_errors` -- `validate(punpuns)` returns `is_ok() == true`, 0 errors
3. `validate_sliceymon_zero_errors` -- `validate(sliceymon)` returns `is_ok() == true`, 0 errors

### Unit tests (each error rule has a targeted test)

4. `validate_unbalanced_parens` -- Input: `"hidden&temporary&ph.bfoo;1;!mheropool.((replica.Foo.sd.0:0:0:0:0:0).n.Foo"` (missing close paren). Assert: 1 error with `rule_id == "E001"`, `position == Some(_)`.
5. `validate_non_ascii` -- Input: modifier containing `\u{2014}` (em-dash). Assert: 1 error with `rule_id == "E002"`.
6. `validate_duplicate_mn` -- Input: two modifiers both containing `.mn.Pikachu@`. Assert: 1 error with `rule_id == "E003"`, message contains "Pikachu".
7. `validate_hero_bad_prefix` -- Input: hero modifier missing `hidden&temporary&` prefix. Assert: 1 error with `rule_id == "E004"`.
8. `validate_hero_bad_suffix` -- Input: hero modifier missing `.part.1&hidden` suffix. Assert: 1 error with `rule_id == "E005"`.
9. `validate_hero_too_few_tiers` -- Input: hero with only 2 `+`-separated parts. Assert: 1 error with `rule_id == "E006"`.
10. `validate_tier_block_no_replica` -- Input: tier block without `(replica.` opening. Assert: 1 error with `rule_id == "E007"`.
11. `validate_bad_face_format` -- Input: `.sd.sword:3-1:0:0:0:0` (non-numeric face). Assert: 1 error with `rule_id == "E008"`.
12. `validate_face_id_out_of_range` -- Input: `.sd.999-1:0:0:0:0:0`. Assert: 1 error with `rule_id == "E009"`, message contains "999".
13. `validate_bad_hp` -- Input: `.hp.0` or `.hp.abc`. Assert: 1 error with `rule_id == "E010"`.
14. `validate_bad_color` -- Input: `.col.3` (digit, not lowercase letter). Assert: 1 error with `rule_id == "E011"`.
15. `validate_bad_tier` -- Input: `.tier.x` (non-digit). Assert: 1 error with `rule_id == "E012"`.
16. `validate_report_is_ok` -- Empty report returns `is_ok() == true`. Report with 1 warning and 0 errors still returns `is_ok() == true`.
17. `validate_report_display` -- Report with errors formats via `Display`, output contains "FAILED" and rule IDs.

### Warning tests

18. `validate_unknown_type_warning` -- Input: modifier that doesn't match any classifier pattern. Assert: 1 warning with `rule_id == "W003"`.
19. `validate_missing_mn_warning` -- Input: modifier without `.mn.`. Assert: 1 warning with `rule_id == "W005"`.

### Context field tests

20. `validate_paren_error_has_context` -- Input: modifier with unbalanced parens. Assert: the E001 finding has `context == Some(...)` containing surrounding chars near the error position.

## CLI Changes

The `Validate` subcommand gains a `--round-trip` flag:

```rust
Validate {
    input: PathBuf,
    /// Also run round-trip IR comparison (extract -> build -> extract)
    #[arg(long)]
    round_trip: bool,
},
```

Default behavior: run `validator::validate()`, print the report, exit with code 1 if errors.
With `--round-trip`: additionally run the existing extract-build-extract IR comparison after validation passes. This preserves backward compatibility for compiler fidelity testing.

## Verification

```bash
# All 3 working mods validate clean
cargo run -- validate ../working-mods/sliceymon.txt  # 0 errors
cargo run -- validate ../working-mods/punpuns.txt     # 0 errors
cargo run -- validate ../working-mods/pansaer.txt     # 0 errors

# Round-trip still available
cargo run -- validate --round-trip ../working-mods/sliceymon.txt

# Expansion build validates before paste
cargo run -- validate ../test_output/sliceymon_plus.txt

# All tests pass
cargo test

# Clippy clean
cargo clippy -- -D warnings
```

## Self-Verification Checklist (Implementer Runs Before Reporting Done)

- [ ] All 3 working mods produce 0 errors (ground truth -- if a rule fires on a working mod, the rule is wrong)
- [ ] `cargo test` passes with 0 failures (all 20 tests)
- [ ] `cargo clippy -- -D warnings` passes
- [ ] Every `use` path compiles (no invented module paths)
- [ ] `parse_face_entry` handles all 4 face patterns: `0`, `N-P`, `N--P`, `N-P#suffix`
- [ ] `Finding.rule_id` values match the rule table exactly (E001-E012, W001-W005)
- [ ] No `unwrap()` or `panic!()` in `validator.rs` (return findings, never crash)
- [ ] `classify` errors caught via `.unwrap_or(ModifierType::Unknown)` (not propagated)
- [ ] `split_modifiers` errors propagated as `CompilerError` via `?` (not swallowed)
- [ ] `validator.rs` contains zero `use std::fs` or I/O imports
- [ ] Display impl output matches the format shown in the Types section
- [ ] `--round-trip` flag works and runs the old IR comparison logic
- [ ] Context field populated for positional errors (E001, E008, E009)
