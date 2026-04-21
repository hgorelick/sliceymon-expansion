//! Per-modifier byte-level drift audit across all 4 working mods.
//! Classifies each drifted/unmatched modifier by kind (hero/boss/monster/heropool/itempool/selector/etc)
//! and surfaces representative examples per (mod, kind) pair.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;

fn modifier_split(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth: i32 = 0;
    let mut buf = String::new();
    for c in text.chars() {
        match c {
            '(' => { depth += 1; buf.push(c); }
            ')' => { depth -= 1; buf.push(c); }
            ',' if depth == 0 => {
                let t = buf.trim().to_string();
                if !t.is_empty() { out.push(t); }
                buf.clear();
            }
            _ => buf.push(c),
        }
    }
    let t = buf.trim().to_string();
    if !t.is_empty() { out.push(t); }
    out
}

fn modifier_key(m: &str) -> String {
    if let Some(idx) = m.find(".mn.") {
        let rest = &m[idx + 4..];
        let end = rest.find(|c: char| c == '@' || c == '!' || c == ',').unwrap_or(rest.len());
        return format!("mn:{}", rest[..end].trim());
    }
    let head: String = m.chars().take(60).collect();
    format!("head:{}", head)
}

/// Classify a modifier by the kind of content it carries.
fn classify(m: &str) -> &'static str {
    let lower = m.to_ascii_lowercase();
    // Order matters: more specific first.
    if lower.contains("!mheropool") || lower.starts_with("heropool.") || lower.contains(";!mheropool") { "heropool" }
    else if lower.contains("!mitempool") || lower.starts_with("itempool.") || lower.contains(";!mitempool") { "itempool" }
    else if lower.contains("!mpartyconfig") || lower.contains("partyconfig.") { "partyconfig" }
    else if lower.contains(".ph.b") { "boss" }
    else if lower.contains(".ph.s") || lower.contains(".ph.c") { "selector" }
    else if lower.contains("difficulty") { "difficulty" }
    else if lower.contains("levelupaction") || lower.contains("levelup") { "levelup" }
    else if lower.contains("bossmodifier") || lower.contains("eventmodifier") || lower.contains("phasemodifier") { "phase/event-modifier" }
    else if lower.contains("artcredits") || lower.contains(".doc.") && lower.starts_with("hidden&temporary&") { "credits/dialog" }
    else if lower.contains("dialog") { "dialog" }
    else if lower.contains(".ph.") { "phase" }
    else if lower.contains("replica.") { "hero-ish" }
    else if lower.contains("rmon.") || lower.contains("ritemx.") || lower.contains("rmod.") { "entity-ref" }
    else if lower.starts_with("=") || lower.starts_with("0.") || lower.starts_with("1.") || lower.starts_with("=skip") { "ph-header" }
    else if lower.starts_with("hidden&temporary") { "hidden-modifier" }
    else { "other" }
}

struct Stat {
    total: usize,
    drifted: usize,
    unmatched: usize,
    examples: Vec<String>,
}

fn audit(name: &str, path: &PathBuf) {
    println!("\n========== {} ==========", name);
    let source = match fs::read_to_string(path) { Ok(t) => t, Err(e) => { println!("READ ERR {}", e); return; } };
    let ir = match textmod_compiler::extract(&source) { Ok(ir) => ir, Err(e) => { println!("EXTRACT ERR {}", e); return; } };
    let rebuilt = match textmod_compiler::build(&ir) { Ok(t) => t, Err(e) => { println!("BUILD ERR {}", e); return; } };

    let src_mods = modifier_split(&source);
    let dst_mods = modifier_split(&rebuilt);
    println!("source mods: {}  rebuilt mods: {}", src_mods.len(), dst_mods.len());

    let mut dst_by_key: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, m) in dst_mods.iter().enumerate() {
        dst_by_key.entry(modifier_key(m)).or_default().push(i);
    }

    let mut by_kind: BTreeMap<&'static str, Stat> = BTreeMap::new();

    for (si, sm) in src_mods.iter().enumerate() {
        let kind = classify(sm);
        let entry = by_kind.entry(kind).or_insert(Stat { total: 0, drifted: 0, unmatched: 0, examples: vec![] });
        entry.total += 1;

        let k = modifier_key(sm);
        let dst_idx = dst_by_key.get(&k).and_then(|v| v.first().copied());
        match dst_idx {
            None => {
                entry.unmatched += 1;
                if entry.examples.len() < 2 {
                    let preview: String = sm.chars().take(140).collect();
                    entry.examples.push(format!("    [{}] UNMATCHED key={} preview={}", si, k, preview));
                }
            }
            Some(di) => {
                let dm = &dst_mods[di];
                if sm == dm { return_ok(); continue; }
                entry.drifted += 1;
                let sb = sm.as_bytes();
                let db = dm.as_bytes();
                let mut pos = 0usize;
                while pos < sb.len() && pos < db.len() && sb[pos] == db[pos] { pos += 1; }
                let from = pos.saturating_sub(20);
                let s_to = (pos + 100).min(sm.len());
                let d_to = (pos + 100).min(dm.len());
                if entry.examples.len() < 2 {
                    entry.examples.push(format!("    [{}] DRIFT key={}\n      src @{}: ...{}\n      dst @{}: ...{}",
                        si, k, pos, &sm[from..s_to], pos, &dm[from..d_to]));
                }
            }
        }
    }

    println!("{:<24} {:>6} {:>8} {:>10}", "kind", "total", "drifted", "unmatched");
    for (k, s) in &by_kind {
        println!("{:<24} {:>6} {:>8} {:>10}", k, s.total, s.drifted, s.unmatched);
    }
    for (k, s) in &by_kind {
        if s.drifted == 0 && s.unmatched == 0 { continue; }
        println!("\n  --- {} examples ---", k);
        for e in &s.examples { println!("{}", e); }
    }
}

fn return_ok() {}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    for name in &["sliceymon", "punpuns", "pansaer", "community"] {
        audit(name, &root.join("working-mods").join(format!("{}.txt", name)));
    }
}
