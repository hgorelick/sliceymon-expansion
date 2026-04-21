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
    // Mana / utility (guide cheat-sheet)
    (76, "MANA"),
    (125, "REROLL"),
    (126, "CANTRIP"),
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

    let ids = harvest_face_ids(&working_mods_dir);
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR must be set by cargo");
    let out_path = Path::new(&out_dir).join("face_id_generated.rs");
    fs::write(&out_path, emit_generated(&ids))
        .unwrap_or_else(|e| panic!("write {}: {}", out_path.display(), e));
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

fn harvest_face_ids(dir: &Path) -> BTreeMap<u16, FaceIdMeta> {
    let mut ids: BTreeMap<u16, FaceIdMeta> = BTreeMap::new();
    for mod_name in WORKING_MOD_ORDER {
        let path = dir.join(mod_name);
        if !path.exists() {
            // Plan authored against a 4-mod corpus; tolerate missing files so
            // the build doesn't break when a mod is in flux.
            continue;
        }
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
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
