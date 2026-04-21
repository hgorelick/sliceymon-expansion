//! Roundtrip diagnostic: for each mod, extract → build → re-extract,
//! and print count deltas and per-StructuralType breakdowns.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use textmod_compiler::ir::{BossFormat, ModIR};

fn count_structural_by_type(ir: &ModIR) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for s in &ir.structural {
        *map.entry(format!("{:?}", s.modifier_type)).or_insert(0) += 1;
    }
    map
}

fn count_boss_formats(ir: &ModIR) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for b in &ir.bosses {
        let key = match b.format {
            BossFormat::Standard => "Standard",
            BossFormat::Event => "Event",
            BossFormat::Encounter => "Encounter",
        };
        *map.entry(key.to_string()).or_insert(0) += 1;
    }
    map
}

fn diff_maps(a: &BTreeMap<String, usize>, b: &BTreeMap<String, usize>) -> Vec<(String, i64, i64)> {
    let mut keys: Vec<&String> = a.keys().chain(b.keys()).collect();
    keys.sort();
    keys.dedup();
    keys.into_iter().map(|k| {
        let av = *a.get(k).unwrap_or(&0) as i64;
        let bv = *b.get(k).unwrap_or(&0) as i64;
        (k.clone(), av, bv)
    }).collect()
}

fn report(name: &str, path: &PathBuf) {
    println!("\n========== {} ==========", name);
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => { println!("  READ ERROR: {}", e); return; }
    };

    let ir1 = match textmod_compiler::extract(&text) {
        Ok(ir) => ir,
        Err(e) => { println!("  EXTRACT1 ERROR: {}", e); return; }
    };

    let s1 = count_structural_by_type(&ir1);
    let b1 = count_boss_formats(&ir1);

    let rebuilt = match textmod_compiler::build(&ir1) {
        Ok(t) => t,
        Err(e) => {
            println!("  BUILD ERROR: {}", e);
            println!("  IR1: heroes={} replicas={} monsters={} bosses={} structural={}",
                ir1.heroes.len(), ir1.replica_items.len(), ir1.monsters.len(),
                ir1.bosses.len(), ir1.structural.len());
            println!("  Structural by type:");
            for (k, v) in &s1 { println!("    {:20} {}", k, v); }
            return;
        }
    };

    let ir2 = match textmod_compiler::extract(&rebuilt) {
        Ok(ir) => ir,
        Err(e) => { println!("  EXTRACT2 ERROR: {}", e); return; }
    };

    let s2 = count_structural_by_type(&ir2);
    let b2 = count_boss_formats(&ir2);

    let dh = ir2.heroes.len() as i64 - ir1.heroes.len() as i64;
    let dr = ir2.replica_items.len() as i64 - ir1.replica_items.len() as i64;
    let dm = ir2.monsters.len() as i64 - ir1.monsters.len() as i64;
    let db = ir2.bosses.len() as i64 - ir1.bosses.len() as i64;
    let ds = ir2.structural.len() as i64 - ir1.structural.len() as i64;

    let pass = dh == 0 && dr == 0 && dm == 0 && db == 0 && ds == 0;
    println!("  Status: {}", if pass { "ROUNDTRIP OK" } else { "ROUNDTRIP FAIL" });
    println!("  Heroes    ir1={:4} ir2={:4} delta={:+}", ir1.heroes.len(), ir2.heroes.len(), dh);
    println!("  Replicas  ir1={:4} ir2={:4} delta={:+}", ir1.replica_items.len(), ir2.replica_items.len(), dr);
    println!("  Monsters  ir1={:4} ir2={:4} delta={:+}", ir1.monsters.len(), ir2.monsters.len(), dm);
    println!("  Bosses    ir1={:4} ir2={:4} delta={:+}", ir1.bosses.len(), ir2.bosses.len(), db);
    println!("  Structural ir1={:4} ir2={:4} delta={:+}", ir1.structural.len(), ir2.structural.len(), ds);

    println!("  Boss formats:");
    for (k, a, b) in diff_maps(&b1, &b2) {
        println!("    {:12} ir1={:3} ir2={:3} delta={:+}", k, a, b, b - a);
    }

    println!("  Structural by type:");
    for (k, a, b) in diff_maps(&s1, &s2) {
        println!("    {:20} ir1={:3} ir2={:3} delta={:+}", k, a, b, b - a);
    }
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("working-mods");
    for mod_name in &["sliceymon", "punpuns", "pansaer", "community"] {
        let path = root.join(format!("{}.txt", mod_name));
        report(mod_name, &path);
    }
}
