//! Build-time FaceID harvester.
//!
//! Scans `working-mods/*.txt` for `.sd.` dice-face declarations, collects every
//! distinct FaceID into a `BTreeMap<u16, Meta>` (deterministic iteration), and
//! emits `$OUT_DIR/face_id_generated.rs` — a file the authoring-layer newtype
//! `include!`s at compile time to populate `pub const FACE_NAME: FaceId = ...`
//! consts.
//!
//! Determinism contract: no wall-clock, PID, or environment input; the same
//! input corpus yields a byte-identical output file. `BTreeMap` (not `HashMap`)
//! guarantees ordered iteration. Consts emit in ascending `u16` order.

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// Fixed corpus scan order — affects doc-comment provenance only (first-seen wins),
/// not correctness. Ordered by authoring authority (primary corpus first).
const WORKING_MOD_ORDER: &[&str] = &[
    "sliceymon.txt",
    "pansaer.txt",
    "punpuns.txt",
    "community.txt",
];

#[derive(Debug)]
struct FaceIdMeta {
    /// Human-readable mechanic name (from KNOWN_FACE_NAMES), else `None`.
    name: Option<&'static str>,
    /// `(mod_name, line_number)` of the first occurrence — stable across builds.
    first_seen: (String, usize),
}

/// Curated FaceID → mechanic name mapping. Sourced from `reference/textmod_guide.md`
/// (cheat-sheet and keyword cards) and `compiler/src/constants.rs::UNTARGETED_FACE_IDS`.
/// Anything not in this table gets the generic `FACE_{id}` const name.
///
/// Keep this list conservative — only add entries whose mechanic the guide
/// documents unambiguously. Guessed names are worse than no name: the authoring
/// layer would propagate the guess.
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

fn main() {
    // Cargo should re-run this script whenever any working-mod changes.
    println!("cargo:rerun-if-changed=build.rs");
    let workspace_root = workspace_root();
    let working_mods_dir = workspace_root.join("working-mods");
    println!(
        "cargo:rerun-if-changed={}",
        working_mods_dir.display()
    );

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR must be set by cargo");
    let out_dir = Path::new(&out_dir);

    // Single-pass read: both harvesters scan the same file contents, so we
    // read each working-mod exactly once and hand each pass the string it
    // needs. Keeps the build script I/O-cheap and ordered deterministically.
    let mod_contents = read_working_mods(&working_mods_dir);

    let ids = harvest_face_ids(&mod_contents);
    let face_path = out_dir.join("face_id_generated.rs");
    fs::write(&face_path, emit_generated(&ids))
        .unwrap_or_else(|e| panic!("write {}: {}", face_path.display(), e));

    let sprites = harvest_sprites(&mod_contents);
    let sprite_path = out_dir.join("sprite_registry_generated.rs");
    fs::write(&sprite_path, emit_sprite_registry(&sprites))
        .unwrap_or_else(|e| panic!("write {}: {}", sprite_path.display(), e));
}

/// Read each working-mod exactly once, in the `WORKING_MOD_ORDER` sequence.
/// Missing files are silently skipped so the build doesn't break while a mod
/// is in flux (same policy as the face-id harvester's previous inline loop).
fn read_working_mods(dir: &Path) -> Vec<(&'static str, String)> {
    let mut out = Vec::with_capacity(WORKING_MOD_ORDER.len());
    for mod_name in WORKING_MOD_ORDER {
        let path = dir.join(mod_name);
        if !path.exists() {
            continue;
        }
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
        out.push((*mod_name, contents));
    }
    out
}

/// The compiler/ Cargo.toml lives one level below the repo root. Walk up from
/// CARGO_MANIFEST_DIR to find the `working-mods/` sibling.
fn workspace_root() -> PathBuf {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be set by cargo");
    PathBuf::from(manifest_dir)
        .parent()
        .expect("compiler/ must have a parent directory")
        .to_path_buf()
}

fn harvest_face_ids(mods: &[(&'static str, String)]) -> BTreeMap<u16, FaceIdMeta> {
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
    ids
}

/// Extract FaceIDs from every `.sd.<...>` segment in a single line.
/// After `.sd.`, a value runs until the next textmod marker (`.X.` where X is
/// alphabetic) or a non-sd-shaped char. Split the value on `:` and parse
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

fn emit_generated(ids: &BTreeMap<u16, FaceIdMeta>) -> String {
    let mut out = String::new();
    out.push_str("// @generated by compiler/build.rs — DO NOT EDIT.\n");
    out.push_str("// Harvested from working-mods/*.txt. Determinism: BTreeMap, no I/O side-effects.\n");
    out.push_str("// Consts live inside the `authoring::face_id` module so `FaceId(N)` can\n");
    out.push_str("// access the newtype's private tuple field.\n\n");
    out.push_str("impl FaceId {\n");
    for (id, meta) in ids {
        let const_name = meta.name.map(str::to_string).unwrap_or_else(|| format!("FACE_{id}"));
        let provenance = if meta.first_seen.1 == 0 {
            "(curated)".to_string()
        } else {
            format!("{}:{}", meta.first_seen.0, meta.first_seen.1)
        };
        out.push_str(&format!(
            "    /// FaceID {id} — first seen {provenance}. See reference/textmod_guide.md.\n"
        ));
        out.push_str(&format!(
            "    pub const {const_name}: FaceId = FaceId({id});\n"
        ));
    }
    out.push_str("}\n\n");

    // Emit a slice of every known FaceID so xref can distinguish Known-vs-Unknown
    // without relying on curated names alone.
    out.push_str("/// Every FaceID harvested from the corpus or curated by name,\n");
    out.push_str("/// in ascending order. Used by the `FaceIdValue::try_new` lookup.\n");
    out.push_str(&format!(
        "pub const KNOWN_FACE_IDS: &[u16] = &[\n"
    ));
    for id in ids.keys() {
        out.push_str(&format!("    {id},\n"));
    }
    out.push_str("];\n");
    out
}

// -- Sprite harvester ---------------------------------------------------------
//
// For each `.img.<payload>` in the corpus, pair it with the nearest `.mn.` or
// `.n.` name at the same paren depth. The result is a `BTreeMap<String, ...>`
// (keyed by display name, first-write-wins in `WORKING_MOD_ORDER` — sliceymon
// highest priority). Empty or pathological lines produce no pairs; they don't
// error. The output is a `phf::Map<&'static str, SpriteId>` static whose value
// expressions construct `SpriteId` via its private fields, which is why the
// generated file is `include!`d into `authoring/sprite.rs` rather than exposed
// as a submodule.

#[derive(Debug)]
struct SpriteEntry {
    img_data: String,
    /// `(mod_name, line_number)` of the first occurrence — stable across builds.
    first_seen: (String, usize),
}

fn harvest_sprites(mods: &[(&'static str, String)]) -> BTreeMap<String, SpriteEntry> {
    let mut sprites: BTreeMap<String, SpriteEntry> = BTreeMap::new();
    for (mod_name, contents) in mods {
        for (line_idx, line) in contents.lines().enumerate() {
            for (name, img) in scan_entity_sprites(line) {
                // First-write-wins in WORKING_MOD_ORDER. `sliceymon` iterates
                // first, so its sprites stick — matching the plan's "priority
                // order" rule. Later mods that reuse the same name are skipped.
                sprites.entry(name).or_insert_with(|| SpriteEntry {
                    img_data: img,
                    first_seen: ((*mod_name).to_string(), line_idx + 1),
                });
            }
        }
    }
    sprites
}

/// Walk a line tracking paren depth; collect `(.img. position, payload, depth)`
/// and `(name position, name, depth, is_mn)` tuples; then for each `.img.`
/// site pick the nearest name at the same depth. `.mn.` beats `.n.` on tie.
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
        // Names at the img's own depth or an outer depth are candidates. The
        // common shape in sliceymon is an `.img.` at depth 1 inside a
        // `(replica.Template...)` group with the entity's `.n.NAME` at depth 0
        // immediately after the closing paren — e.g.
        //   !mheropool.(replica.Lost...img.DATA).speech.X.n.Eevee+...
        // Scoring: (fewer-outer-hops first, then shorter distance, then `.mn.` wins).
        let mut best: Option<(u32, usize, &str, bool)> = None;
        for (npos, name, ndepth, is_mn) in &name_sites {
            if ndepth > idepth {
                // Names at deeper depths belong to nested entities, not this one.
                continue;
            }
            let hops = (*idepth - *ndepth) as u32;
            let dist = if *npos >= *ipos { *npos - *ipos } else { *ipos - *npos };
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
/// - Bare: end at the next `.X.` property marker (`.` followed by an alphabetic
///   char), `(`, `)`, or end-of-line. `=` and `%` are valid payload characters
///   and do NOT terminate the value.
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

/// Termination rule for a `.n.NAME` or `.mn.NAME` value. Names terminate at
/// any char that can start the next structural element: `.` (next property),
/// `(`, `)`, `+`, `=`, `&`, `@`, `,`, or newline. This matches the union of
/// `util::extract_last_n_name` and `util::extract_mn_name` terminators closely
/// enough for the registry-harvest use case — we only need to capture the
/// entity's display name, not every byte of it verbatim.
fn find_name_end(line: &str, start: usize) -> usize {
    let bytes = line.as_bytes();
    let mut j = start;
    while j < bytes.len() {
        let c = bytes[j];
        if matches!(c, b'.' | b'(' | b')' | b'+' | b'=' | b'&' | b'@' | b',' | b'\n') {
            return j;
        }
        j += 1;
    }
    j
}

fn emit_sprite_registry(sprites: &BTreeMap<String, SpriteEntry>) -> String {
    let mut out = String::new();
    out.push_str("// @generated by compiler/build.rs — DO NOT EDIT.\n");
    out.push_str("// Harvested from working-mods/*.txt. Determinism: BTreeMap-ordered phf build.\n\n");

    // Build the phf map. Keys are &str borrowed from the BTreeMap (lives for
    // the rest of this function), values are Rust expression strings that
    // construct `SpriteId` through its private fields. The `include!` in
    // `authoring/sprite.rs` puts the generated static inside that module's
    // privacy scope, so the field access is legal.
    let mut map: phf_codegen::Map<&str> = phf_codegen::Map::new();
    // Hold value-expression strings for the lifetime of `map.build()`.
    let value_exprs: Vec<(String, String, (String, usize))> = sprites
        .iter()
        .map(|(name, entry)| {
            let expr = format!(
                "SpriteId {{ name: ::std::borrow::Cow::Borrowed({name:?}), img_data: ::std::borrow::Cow::Borrowed({img:?}) }}",
                name = name,
                img = entry.img_data,
            );
            (name.clone(), expr, entry.first_seen.clone())
        })
        .collect();
    for (name, expr, _) in &value_exprs {
        map.entry(name.as_str(), expr.as_str());
    }

    // Provenance block — human-readable, stable across builds because the
    // BTreeMap iterates in sorted order and first_seen is deterministic.
    out.push_str("// Sprite provenance (stable order):\n");
    for (name, _, (mod_name, line_no)) in &value_exprs {
        let _ = writeln!(&mut out, "//   {name} ← {mod_name}:{line_no}");
    }
    out.push('\n');

    out.push_str("/// Corpus-derived sprite registry. Keys are entity display names\n");
    out.push_str("/// (`.mn.` preferred, `.n.` fallback) harvested from `working-mods/*.txt`.\n");
    out.push_str("/// First-write-wins in the `sliceymon > pansaer > punpuns > community` priority order.\n");
    out.push_str("pub static SPRITE_REGISTRY: ::phf::Map<&'static str, SpriteId> = ");
    let _ = write!(&mut out, "{}", map.build());
    out.push_str(";\n");
    out
}
