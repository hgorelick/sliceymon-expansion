//! Roundtrip baseline pins — regression signal while PIPELINE_FIDELITY_PLAN
//! is in flight.
//!
//! The full-mod `extract -> build -> extract` cycle is known-red today on all
//! four reference mods. PIPELINE_FIDELITY_PLAN.md owns the fix. Until that
//! lands we pin the *current* shape of the failure (per-entity-collection
//! equality + counts) so any *change* — better or worse — trips a red test.
//!
//! When drift intentionally shifts (a chunk of PIPELINE_FIDELITY_PLAN lands),
//! re-bless with:
//!
//!   UPDATE_BASELINES=1 cargo test --test roundtrip_baseline
//!
//! and commit the updated files under tests/baselines/roundtrip/.

use std::fs;
use std::path::PathBuf;

use textmod_compiler::{build_complete, extract};

#[test]
fn baseline_sliceymon() {
    check("sliceymon");
}
#[test]
fn baseline_pansaer() {
    check("pansaer");
}
#[test]
fn baseline_punpuns() {
    check("punpuns");
}
#[test]
fn baseline_community() {
    check("community");
}

fn check(name: &str) {
    let mod_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("working-mods")
        .join(format!("{}.txt", name));
    let baseline_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("baselines")
        .join("roundtrip")
        .join(format!("{}.baseline", name));

    let text = fs::read_to_string(&mod_path)
        .unwrap_or_else(|e| panic!("reading {}: {}", mod_path.display(), e));
    let observed = summarize(&text);

    if std::env::var_os("UPDATE_BASELINES").is_some() {
        if let Some(p) = baseline_path.parent() {
            fs::create_dir_all(p).expect("creating baseline dir");
        }
        fs::write(&baseline_path, &observed).expect("writing baseline");
        return;
    }

    let expected = fs::read_to_string(&baseline_path).unwrap_or_else(|e| {
        panic!(
            "reading baseline {}: {}.\n\nObserved:\n{}\n\n\
             To create the baseline, run:\n  \
             UPDATE_BASELINES=1 cargo test --test roundtrip_baseline",
            baseline_path.display(),
            e,
            observed
        )
    });

    assert_eq!(
        expected.trim(),
        observed.trim(),
        "\nroundtrip baseline drift for `{}`.\n\
         If the drift is intentional, re-bless with:\n  \
         UPDATE_BASELINES=1 cargo test --test roundtrip_baseline\n",
        name
    );
}

fn summarize(text: &str) -> String {
    let ir = match extract(text) {
        Ok(ir) => ir,
        Err(e) => return format!("phase: extract1\nresult: err\nerror: {}\n", e),
    };
    let built = match build_complete(&ir) {
        Ok(t) => t,
        Err(e) => return format!("phase: build\nresult: err\nerror: {}\n", e),
    };
    let reextracted = match extract(&built) {
        Ok(ir2) => ir2,
        Err(e) => return format!("phase: extract2\nresult: err\nerror: {}\n", e),
    };
    let mut out = String::new();
    out.push_str("phase: complete\n");
    out.push_str(&format!(
        "heroes.count: {} -> {}\n",
        ir.heroes.len(),
        reextracted.heroes.len()
    ));
    out.push_str(&format!(
        "heroes.equal: {}\n",
        ir.heroes == reextracted.heroes
    ));
    out.push_str(&format!(
        "replica_items.count: {} -> {}\n",
        ir.replica_items.len(),
        reextracted.replica_items.len()
    ));
    out.push_str(&format!(
        "replica_items.equal: {}\n",
        ir.replica_items == reextracted.replica_items
    ));
    out.push_str(&format!(
        "monsters.count: {} -> {}\n",
        ir.monsters.len(),
        reextracted.monsters.len()
    ));
    out.push_str(&format!(
        "monsters.equal: {}\n",
        ir.monsters == reextracted.monsters
    ));
    out.push_str(&format!(
        "bosses.count: {} -> {}\n",
        ir.bosses.len(),
        reextracted.bosses.len()
    ));
    out.push_str(&format!(
        "bosses.equal: {}\n",
        ir.bosses == reextracted.bosses
    ));
    out.push_str(&format!("roundtrip.equal: {}\n", ir == reextracted));
    out
}
