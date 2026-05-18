//! Library-reachability + invocability smoke test for the harvester pure core.
//!
//! Compile-time: the bound crate-external paths `textmod_compiler::harvester::
//! harvest_face_ids` and `harvest_sprites` resolve; their signatures match the
//! shapes the `harvester-pure-core` chunk plan binds (`&[(&'static str, String)]`
//! input; `Vec<(String, String)>` and `Vec<(String, String, String)>` returns
//! respectively, the latter's third element carrying the per-entry
//! `<mod_name>:<line_no>` provenance suffix consumed by `compiler/build.rs`'s
//! provenance comment block emission).
//!
//! Runtime: smoke-test scope is invocability + signature + ordering, not per-
//! entry count audit (the SHA-256 ledger captured in the chunk's PR is the
//! byte-identity gate; majority-content regressions surface there).

use textmod_compiler::harvester::{harvest_face_ids, harvest_sprites};

/// Build the corpus as `Vec<(&'static str, String)>` in `WORKING_MOD_ORDER`
/// priority (sliceymon > pansaer > punpuns > community) from `include_str!`-
/// baked working-mod sources. The `.to_string()` per-mod allocation matches
/// today's `compiler/build.rs` callsite element-type shape.
fn corpus() -> Vec<(&'static str, String)> {
    vec![
        ("sliceymon", include_str!("../../working-mods/sliceymon.txt").to_string()),
        ("pansaer", include_str!("../../working-mods/pansaer.txt").to_string()),
        ("punpuns", include_str!("../../working-mods/punpuns.txt").to_string()),
        ("community", include_str!("../../working-mods/community.txt").to_string()),
    ]
}

#[test]
fn harvest_face_ids_returns_at_least_eighteen_pairs() {
    // The 18-floor catches a catastrophic empty-return regression. Today's
    // KNOWN_FACE_NAMES table seeds 18 curated entries and the four-mod
    // corpus contributes ~170 additional distinct FaceIDs, so the floor is
    // far below the natural-passing output size — it fires when `harvest_face_ids`
    // returns essentially nothing (e.g., the BTreeMap aggregation regressed
    // to an unreachable branch). Curated-loop drift (dropping the
    // KNOWN_FACE_NAMES belt-and-suspenders seeding) is caught by the
    // SHA-256 ledger on face_id_generated.rs, not by this assertion.
    let mods = corpus();
    let face_ids: Vec<(String, String)> = harvest_face_ids(&mods);
    assert!(
        face_ids.len() >= 18,
        "harvest_face_ids returned {} pairs; expected at least 18 (catastrophic empty-return floor)",
        face_ids.len()
    );
}

#[test]
fn harvest_face_ids_first_elements_are_unique() {
    // Deterministic-presence smoke: source aggregation is `BTreeMap<u16, _>`,
    // and the encoding-of-the-first-element is the implementer's residual
    // choice (u16-as-string OR FaceIdMeta-derived name). Uniqueness rules out
    // a HashMap intermediate that collapsed distinct u16 keys to the same
    // first-element string, without binding the encoding choice. A lex-
    // monotone assertion would underspecify the encoding because "107"
    // lex-sorts before "13" on decimal-prefix forms.
    let mods = corpus();
    let face_ids: Vec<(String, String)> = harvest_face_ids(&mods);
    let mut keys: Vec<&str> = face_ids.iter().map(|(k, _)| k.as_str()).collect();
    keys.sort();
    let count_before = keys.len();
    keys.dedup();
    assert_eq!(
        count_before,
        keys.len(),
        "harvest_face_ids first elements must be unique; got {} pairs with {} distinct keys",
        count_before,
        keys.len()
    );
}

#[test]
fn harvest_sprites_returns_at_least_one_triple() {
    // No numeric floor on harvest_sprites: an unverified floor would either
    // ship the smoke test RED on count-mismatch unrelated to harvester
    // correctness, or admit a silent regression below the hand-estimated
    // floor. Majority-drop content regression surfaces via the SHA-256
    // ledger captured in this chunk's PR; this assertion catches the
    // catastrophic empty-return regression.
    let mods = corpus();
    let sprites: Vec<(String, String, String)> = harvest_sprites(&mods);
    assert!(
        !sprites.is_empty(),
        "harvest_sprites returned an empty Vec; corpus contains at least one .img. site"
    );
}

#[test]
fn harvest_sprites_first_elements_are_lex_ascending_and_unique() {
    // Source aggregation is `BTreeMap<String, _>` per §Conventions "Vec
    // ordering preservation"; `BTreeMap::into_iter` over String keys
    // produces lex-ascending output and the first-element of each tuple
    // IS the String key, so this property is automatic under the bound
    // collection shape. The assertion is the smoke detector for a
    // post-collect reordering or HashMap intermediate.
    let mods = corpus();
    let sprites: Vec<(String, String, String)> = harvest_sprites(&mods);
    let keys: Vec<&str> = sprites.iter().map(|(k, _, _)| k.as_str()).collect();
    for pair in keys.windows(2) {
        assert!(
            pair[0] < pair[1],
            "harvest_sprites first elements must be strictly lex-ascending; found `{}` >= `{}`",
            pair[0],
            pair[1]
        );
    }
}

#[test]
fn harvest_sprites_provenance_suffix_is_non_empty_and_well_shaped() {
    // The third element is consumed byte-verbatim by `compiler/build.rs`
    // glue into the `// Sprite provenance (stable order):` comment block
    // above the phf static. An empty third element would silently corrupt
    // that block (single-space gap after `←`); a malformed shape (missing
    // colon, non-numeric line number, leading zero) would land bytes in
    // the generated file that do not match today's `emit_sprite_registry`
    // output, REJECTing via the SHA ledger. The regex floor here is a
    // runtime check that fails the smoke test BEFORE the SHA ledger
    // fires, giving a sharper failure mode than ledger divergence on
    // this column.
    let mods = corpus();
    let sprites: Vec<(String, String, String)> = harvest_sprites(&mods);
    for (key, _, provenance) in &sprites {
        assert!(
            !provenance.is_empty(),
            "harvest_sprites provenance suffix for `{}` is empty",
            key
        );
        // Regex `^[a-z][a-z_]*:[1-9][0-9]*$` — non-empty lowercase-and-
        // underscore mod-name, colon, non-zero-leading positive line
        // number. Inline-verified rather than pulling in a regex
        // dependency: the harvester pure-core's type universe forbids
        // crate imports beyond the std prelude-equivalents, but this
        // test file is downstream of that ban.
        let (mod_name, line_no) = provenance
            .split_once(':')
            .unwrap_or_else(|| panic!(
                "harvest_sprites provenance for `{}` missing `:` separator: {:?}",
                key, provenance
            ));
        assert!(
            !mod_name.is_empty()
                && mod_name.chars().next().is_some_and(|c| c.is_ascii_lowercase())
                && mod_name.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
            "harvest_sprites provenance for `{}` has malformed mod-name `{}`; expected lowercase + underscore",
            key,
            mod_name
        );
        assert!(
            !line_no.is_empty()
                && !line_no.starts_with('0')
                && line_no.chars().all(|c| c.is_ascii_digit()),
            "harvest_sprites provenance for `{}` has malformed line-number `{}`; expected non-zero-leading positive integer",
            key,
            line_no
        );
    }
}
