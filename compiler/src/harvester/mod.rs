//! Build-time pure-core harvester for FaceID + sprite registry generation.
//!
//! Pure-core extraction of the harvester previously inlined in
//! `compiler/build.rs`. Library-reachable without `std::fs` per SPEC §3.4 so
//! the determinism audit (sibling chunk) and any future browser/mobile
//! mod-builder app feature compile can invoke it without a `cargo build`
//! wrapper.
//!
//! Pure-core boundary: the entry points accept pre-read mod contents as
//! `&[(name, contents)]` slices in memory and return owned `Vec`s of
//! tuples. The build-script consumer reads the corpus via `fs::read_to_string`
//! and feeds it in; the I/O shell stays in the build script (a separate
//! compile unit from the library).
//!
//! Determinism contract: no wall-clock, process, or environment input;
//! `BTreeMap` (not `HashMap`) guarantees ordered iteration; the per-entry
//! tuple shapes carry every byte the build-script glue splices into the
//! generated files, so two invocations against the same corpus produce
//! byte-identical output. The recurring CI gate that asserts this
//! property is the determinism-audit sibling chunk; this module is the
//! structural enabler.

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Per-FaceID provenance + curated-name metadata aggregated during the
/// corpus scan.
#[derive(Debug)]
struct FaceIdMeta {
    /// Human-readable mechanic name (from `KNOWN_FACE_NAMES`), else `None`.
    name: Option<&'static str>,
    /// `(mod_name, line_number)` of the first occurrence — stable across
    /// builds. `line_number == 0` is the sentinel for curated entries that
    /// the corpus does not reference.
    first_seen: (String, usize),
}

/// Per-sprite payload + provenance aggregated during the corpus scan.
#[derive(Debug)]
struct SpriteEntry {
    img_data: String,
    /// `(mod_name, line_number)` of the first occurrence — stable across builds.
    first_seen: (String, usize),
}

/// Curated FaceID → mechanic name mapping. Sourced from
/// `reference/textmod_guide.md` (cheat-sheet and keyword cards) and
/// `compiler/src/constants.rs::UNTARGETED_FACE_IDS`. Anything not in this
/// table gets the generic `FACE_{id}` const name.
///
/// Keep this list conservative — only add entries whose mechanic the guide
/// documents unambiguously. Guessed names are worse than no name: the
/// authoring layer would propagate the guess.
const KNOWN_FACE_NAMES: &[(u16, &str)] = &[
    // Damage (guide §cheat-sheet: "Damage pip | sd.15-2 (Basic Damage)")
    (15, "DAMAGE_BASIC"),
    (34, "DAMAGE_ALL_OR_SELF"),
    (54, "DAMAGE_ALL"),
    (128, "DAMAGE_ALL_OR_SELF_ALT"),
    (158, "DAMAGE_ALL_ALT"),
    (160, "DAMAGE_ALL_ALT2"),
    // Shield (guide line 640: "BoostShield | sd.146 (add selfshield)")
    (72, "SHIELD_ALL"),
    (73, "SHIELD_ALL_ALT"),
    (146, "SHIELD_ADD_SELF"),
    // Heal
    (107, "HEAL_ALL"),
    // Mana / utility (guide cheat-sheet + sd. index at line 1165)
    (76, "MANA"),
    (125, "REROLL"),
    // FaceID 13 — guide line 1165: "sd.13 I Die Cantrip".
    (13, "CANTRIP"),
    // Keyword cards (guide line 630, 639-640)
    (137, "DAMAGE_RAMPAGE"),
    (169, "SNAKE_DAMAGE"),
    (170, "WOLF_DAMAGE"),
    (171, "WOLF_CLEAVE"),
    (150, "ENGAGE_ADD"),
];

// -- FaceID harvester ---------------------------------------------------------

/// Harvest FaceIDs from the corpus into per-entry `(u16-as-string, opaque-text)`
/// pairs.
///
/// The second-element opaque text is the four-space-indented `/// FaceID ...`
/// doc-comment line followed by the four-space-indented `pub const ...` line,
/// each terminated by `\n`. The build-script glue splices the second-element
/// string verbatim into the `impl FaceId { ... }` body in
/// `face_id_generated.rs`; column boundaries are the bound integration
/// contract per the chunk plan's §Conventions "Return shape — harvest_face_ids".
///
/// The first element is the `u16`-as-string FaceID; the build-script glue
/// parses it back to `u16` for the `KNOWN_FACE_IDS` slice emission. Pair
/// ordering is `BTreeMap<u16, _>::into_iter()` — numerically ascending by
/// FaceID, which is the property `KNOWN_FACE_IDS` needs to be ascending-
/// sorted as documented at its declaration site.
pub fn harvest_face_ids(mods: &[(&'static str, String)]) -> Vec<(String, String)> {
    let mut ids: BTreeMap<u16, FaceIdMeta> = BTreeMap::new();
    for (mod_name, contents) in mods {
        for (line_idx, line) in contents.lines().enumerate() {
            for raw_id in scan_sd_face_ids(line) {
                if raw_id == 0 {
                    // `0` is the blank-face sentinel, not a FaceID.
                    continue;
                }
                ids.entry(raw_id).or_insert_with(|| FaceIdMeta {
                    name: KNOWN_FACE_NAMES
                        .iter()
                        .find(|(id, _)| *id == raw_id)
                        .map(|(_, n)| *n),
                    first_seen: ((*mod_name).to_string(), line_idx + 1),
                });
            }
        }
    }
    // Belt-and-suspenders: every curated known-name is emitted even if the
    // corpus happens not to reference it today. The generated consts are
    // the authoring surface, not a corpus snapshot.
    for (id, name) in KNOWN_FACE_NAMES {
        ids.entry(*id).or_insert_with(|| FaceIdMeta {
            name: Some(*name),
            first_seen: ("curated".to_string(), 0),
        });
    }

    ids.into_iter()
        .map(|(id, meta)| {
            let const_name = meta
                .name
                .map(str::to_string)
                .unwrap_or_else(|| format!("FACE_{id}"));
            let provenance = if meta.first_seen.1 == 0 {
                "(curated)".to_string()
            } else {
                format!("{}:{}", meta.first_seen.0, meta.first_seen.1)
            };
            let mut block = String::new();
            let _ = writeln!(
                &mut block,
                "    /// FaceID {id} — first seen {provenance}. See reference/textmod_guide.md."
            );
            let _ = writeln!(
                &mut block,
                "    pub const {const_name}: FaceId = FaceId({id});"
            );
            (id.to_string(), block)
        })
        .collect()
}

/// Extract FaceIDs from every `.sd.<...>` segment in a single line.
/// After `.sd.`, a value runs until the next textmod marker (`.X.` where X
/// is alphabetic) or a non-sd-shaped char. Split the value on `:` and parse
/// `FaceID-Pips` or bare `FaceID` from each chunk.
fn scan_sd_face_ids(line: &str) -> Vec<u16> {
    let mut out = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    while i + 4 <= bytes.len() {
        if &bytes[i..i + 4] == b".sd." {
            let start = i + 4;
            let end = find_sd_value_end(line, start);
            let value = &line[start..end];
            for chunk in value.split(':') {
                if let Some(id) = parse_face_id_chunk(chunk) {
                    out.push(id);
                }
            }
            i = end;
        } else {
            i += 1;
        }
    }
    out
}

/// The `.sd.` value ends at the first `.` that begins a new textmod marker
/// (i.e. `.` immediately followed by an alphabetic char) or at end-of-line.
/// Digits, `-`, and `:` are part of the value.
fn find_sd_value_end(line: &str, start: usize) -> usize {
    let bytes = line.as_bytes();
    let mut i = start;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'.' {
            if let Some(&next) = bytes.get(i + 1) {
                if next.is_ascii_alphabetic() {
                    return i;
                }
            }
        }
        if !(c.is_ascii_digit() || c == b'-' || c == b':' || c == b'.') {
            return i;
        }
        i += 1;
    }
    i
}

fn parse_face_id_chunk(chunk: &str) -> Option<u16> {
    let chunk = chunk.trim();
    if chunk.is_empty() {
        return None;
    }
    // Split on the FIRST dash only (pips may be negative: `13--1`).
    let id_str = chunk.split_once('-').map(|(id, _)| id).unwrap_or(chunk);
    id_str.parse::<u16>().ok()
}

// -- Sprite harvester ---------------------------------------------------------
//
// For each `.img.<payload>` in the corpus, pair it with the nearest `.mn.`
// or `.n.` name at the same paren depth. The result is a `BTreeMap<String,
// ...>` (keyed by display name, first-write-wins in `WORKING_MOD_ORDER` —
// sliceymon highest priority). Empty or pathological lines produce no
// pairs; they don't error.

/// Harvest sprite entries from the corpus into per-entry
/// `(sprite_name, sprite_id_value_expression, "<mod>:<line>")` triples.
///
/// The first element is the sprite-name key. The second element is the
/// `SpriteId` value-expression string the build-script glue feeds as
/// opaque text into the compile-time map builder used by the build
/// dependency (Property b per §Conventions). The third element is the
/// `<mod_name>:<line_no>` provenance suffix the build-script glue prefixes
/// with `//   <name> ← ` and emits into the `// Sprite provenance (stable
/// order):` comment block above the perfect-hash static (Property d).
///
/// Pair ordering is `BTreeMap<String, _>::into_iter()` — lex-ascending by
/// sprite name. No post-collect reordering: the compile-time map builder's
/// output bytes depend on insertion order, so any tautological re-sort is
/// banned to prevent a future contributor extending it to a non-
/// tautological form (per §Conventions "harvest_sprites no post-collect
/// reordering").
pub fn harvest_sprites(
    mods: &[(&'static str, String)],
) -> Vec<(String, String, String)> {
    let mut sprites: BTreeMap<String, SpriteEntry> = BTreeMap::new();
    for (mod_name, contents) in mods {
        for (line_idx, line) in contents.lines().enumerate() {
            for (name, img) in scan_entity_sprites(line) {
                // First-write-wins in WORKING_MOD_ORDER. Because `sliceymon`
                // iterates first, its sprites stick — the plan's "mod-priority
                // last-write-wins" phrasing refers to the priority outcome,
                // not iteration direction: equivalent here under forward
                // iteration + first-write. Later mods reusing the same name
                // are skipped.
                sprites.entry(name).or_insert_with(|| SpriteEntry {
                    img_data: img,
                    first_seen: ((*mod_name).to_string(), line_idx + 1),
                });
            }
        }
    }

    sprites
        .into_iter()
        .map(|(name, entry)| {
            let expr = format!(
                "SpriteId {{ name: ::std::borrow::Cow::Borrowed({name:?}), img_data: ::std::borrow::Cow::Borrowed({img:?}) }}",
                name = name,
                img = entry.img_data,
            );
            let provenance_suffix =
                format!("{}:{}", entry.first_seen.0, entry.first_seen.1);
            (name, expr, provenance_suffix)
        })
        .collect()
}

/// Walk a line tracking paren depth; collect `(.img. position, payload,
/// depth)` and `(name position, name, depth, is_mn)` tuples; then for
/// each `.img.` site pick the nearest name at the same depth. `.mn.`
/// beats `.n.` on tie.
fn scan_entity_sprites(line: &str) -> Vec<(String, String)> {
    let bytes = line.as_bytes();
    let mut img_sites: Vec<(usize, String, i32)> = Vec::new();
    // (pos, name, depth, is_mn)
    let mut name_sites: Vec<(usize, String, i32, bool)> = Vec::new();

    let mut depth: i32 = 0;
    let mut i: usize = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'(' {
            depth += 1;
            i += 1;
            continue;
        }
        if c == b')' {
            depth -= 1;
            i += 1;
            continue;
        }
        if c == b'.' && line[i..].starts_with(".img.") {
            let val_start = i + ".img.".len();
            let val_end = find_img_val_end(line, val_start);
            let val = line[val_start..val_end].to_string();
            if !val.is_empty() {
                img_sites.push((i, val, depth));
            }
            i = val_end.max(i + 1);
            continue;
        }
        if c == b'.' && line[i..].starts_with(".mn.") {
            let name_start = i + ".mn.".len();
            let name_end = find_name_end(line, name_start);
            let name = line[name_start..name_end].trim().to_string();
            if !name.is_empty() {
                name_sites.push((i, name, depth, true));
            }
            i = name_end.max(i + 1);
            continue;
        }
        if c == b'.' && line[i..].starts_with(".n.") {
            let name_start = i + ".n.".len();
            let name_end = find_name_end(line, name_start);
            let name = line[name_start..name_end].trim().to_string();
            if !name.is_empty() {
                name_sites.push((i, name, depth, false));
            }
            i = name_end.max(i + 1);
            continue;
        }
        i += 1;
    }

    let mut out: Vec<(String, String)> = Vec::new();
    for (ipos, val, idepth) in &img_sites {
        // Names at the img's own depth or an outer depth are candidates.
        // The common shape in sliceymon is an `.img.` at depth 1 inside a
        // `(replica.Template...)` group with the entity's `.n.NAME` at
        // depth 0 immediately after the closing paren — e.g.
        //   !mheropool.(replica.Lost...img.DATA).speech.X.n.Eevee+...
        // Scoring: (fewer-outer-hops first, then shorter distance, then
        // `.mn.` wins).
        let mut best: Option<(u32, usize, &str, bool)> = None;
        for (npos, name, ndepth, is_mn) in &name_sites {
            if ndepth > idepth {
                // Names at deeper depths belong to nested entities, not this one.
                continue;
            }
            let hops = (*idepth - *ndepth) as u32;
            let dist = (*npos).abs_diff(*ipos);
            let replace = match &best {
                None => true,
                Some((bh, bd, _, bmn)) => {
                    if hops != *bh {
                        hops < *bh
                    } else if *is_mn != *bmn {
                        *is_mn && !*bmn
                    } else {
                        dist < *bd
                    }
                }
            };
            if replace {
                best = Some((hops, dist, name.as_str(), *is_mn));
            }
        }
        if let Some((_, _, name, _)) = best {
            out.push((name.to_string(), val.clone()));
        }
    }
    out
}

/// Termination rule for an `.img.VAL` payload.
/// - Paren-wrapped (`.img.(X)`): include the whole matched group.
/// - Bare: end at the next `.X.` property marker (`.` followed by an
///   alphabetic char), `(`, `)`, or end-of-line. `=` and `%` are valid
///   payload characters and do NOT terminate the value.
fn find_img_val_end(line: &str, start: usize) -> usize {
    let bytes = line.as_bytes();
    if start < bytes.len() && bytes[start] == b'(' {
        let mut d: i32 = 0;
        let mut j = start;
        while j < bytes.len() {
            match bytes[j] {
                b'(' => d += 1,
                b')' => {
                    d -= 1;
                    if d == 0 {
                        return j + 1;
                    }
                }
                _ => {}
            }
            j += 1;
        }
        return bytes.len();
    }
    let mut j = start;
    while j < bytes.len() {
        let c = bytes[j];
        if c == b'(' || c == b')' {
            return j;
        }
        if c == b'.' {
            if let Some(&next) = bytes.get(j + 1) {
                if next.is_ascii_alphabetic() {
                    return j;
                }
            }
        }
        j += 1;
    }
    j
}

/// Termination rule for a `.n.NAME` or `.mn.NAME` value. Names terminate
/// at any char that can start the next structural element: `.` (next
/// property), `(`, `)`, `+`, `=`, `&`, `@`, `,`, or newline. This matches
/// the union of `util::extract_last_n_name` and `util::extract_mn_name`
/// terminators closely enough for the registry-harvest use case — only
/// the entity's display name is captured, not every byte of it verbatim.
fn find_name_end(line: &str, start: usize) -> usize {
    let bytes = line.as_bytes();
    let mut j = start;
    while j < bytes.len() {
        let c = bytes[j];
        if matches!(
            c,
            b'.' | b'(' | b')' | b'+' | b'=' | b'&' | b'@' | b',' | b'\n'
        ) {
            return j;
        }
        j += 1;
    }
    j
}
