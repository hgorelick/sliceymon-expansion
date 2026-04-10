use textmod_compiler::extractor::classifier::{classify, ModifierType};
use textmod_compiler::extractor::splitter::split_modifiers;

fn load_mod(name: &str) -> String {
    let path = format!("../working-mods/{}.txt", name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", path, e))
}

fn split_and_classify(name: &str) -> Vec<(usize, ModifierType, String)> {
    let text = load_mod(name);
    let mods = split_modifiers(&text).unwrap();
    mods.iter()
        .enumerate()
        .map(|(i, m)| {
            let t = classify(m, i).unwrap();
            let preview = if m.len() > 80 { m[..80].to_string() } else { m.to_string() };
            (i, t, preview)
        })
        .collect()
}

#[test]
fn classify_sliceymon_hero() {
    let text = load_mod("sliceymon");
    let mods = split_modifiers(&text).unwrap();
    // Sliceymon hero lines contain "heropool" and "replica."
    let gible = mods.iter().enumerate().find(|(_, m)| {
        m.contains("heropool") && m.contains("replica.") && m.contains(".mn.Gible")
    });
    assert!(gible.is_some(), "Could not find Gible hero modifier");
    let (idx, m) = gible.unwrap();
    assert_eq!(classify(m, idx).unwrap(), ModifierType::Hero);
}

#[test]
fn classify_pansaer_hero() {
    let text = load_mod("pansaer");
    let mods = split_modifiers(&text).unwrap();
    // Pansaer uses "Heropool.(replica." with capital H
    let hero = mods.iter().enumerate().find(|(_, m)| {
        m.contains("heropool") || m.contains("Heropool")
    });
    assert!(hero.is_some(), "Could not find any pansaer hero modifier");
    let (idx, m) = hero.unwrap();
    let mtype = classify(m, idx).unwrap();
    // Could be Hero or HeroPoolBase depending on whether it has replica
    assert!(
        mtype == ModifierType::Hero || mtype == ModifierType::HeroPoolBase,
        "Expected Hero or HeroPoolBase, got {:?}",
        mtype
    );
}

#[test]
fn classify_sliceymon_capture_pool_as_itempool() {
    let text = load_mod("sliceymon");
    let mods = split_modifiers(&text).unwrap();
    // Capture pools are compound itempools with #alternatives — classified as ItemPool
    let capture_pool = mods.iter().enumerate().find(|(_, m)| {
        let lower = m.to_lowercase();
        lower.starts_with("itempool.") && lower.contains("hat.replica")
    });
    assert!(capture_pool.is_some(), "Could not find capture pool modifier");
    let (idx, m) = capture_pool.unwrap();
    assert_eq!(classify(m, idx).unwrap(), ModifierType::ItemPool);
}

#[test]
fn classify_sliceymon_monster_pools_as_itempool() {
    let text = load_mod("sliceymon");
    let mods = split_modifiers(&text).unwrap();
    // Sliceymon monsters are compound pools (start with "(") — classified as ItemPool
    let monster_pool = mods.iter().enumerate().find(|(_, m)| {
        m.to_lowercase().contains("monsterpool.")
    });
    assert!(monster_pool.is_some(), "Could not find monster pool modifier");
    let (idx, m) = monster_pool.unwrap();
    assert_eq!(classify(m, idx).unwrap(), ModifierType::ItemPool);
}

#[test]
fn classify_simple_monster() {
    // Punpuns has simple (non-wrapped) monsters
    let text = load_mod("punpuns");
    let mods = split_modifiers(&text).unwrap();
    let monster = mods.iter().enumerate().find(|(_, m)| {
        m.to_lowercase().contains("monsterpool.") && !m.starts_with('(')
    });
    assert!(monster.is_some(), "Could not find simple monster modifier in punpuns");
    let (idx, m) = monster.unwrap();
    assert_eq!(classify(m, idx).unwrap(), ModifierType::Monster);
}

#[test]
fn classify_sliceymon_boss() {
    let text = load_mod("sliceymon");
    let mods = split_modifiers(&text).unwrap();
    let boss = mods.iter().enumerate().find(|(_, m)| {
        m.to_lowercase().contains(".fight.")
    });
    assert!(boss.is_some(), "Could not find boss modifier");
    let (idx, m) = boss.unwrap();
    assert_eq!(classify(m, idx).unwrap(), ModifierType::Boss);
}

#[test]
fn classify_all_sliceymon_no_unknowns() {
    let classified = split_and_classify("sliceymon");
    let unknowns: Vec<_> = classified
        .iter()
        .filter(|(_, t, _)| *t == ModifierType::Unknown)
        .collect();
    if !unknowns.is_empty() {
        for (i, _, preview) in &unknowns {
            eprintln!("Unknown modifier #{}: {}", i, preview);
        }
    }
    assert!(
        unknowns.is_empty(),
        "Found {} unknown modifiers in sliceymon (see stderr for details)",
        unknowns.len()
    );
}

#[test]
fn classify_all_punpuns_no_panics() {
    let classified = split_and_classify("punpuns");
    // Just verify no panics -- some unknowns are acceptable for punpuns
    assert!(!classified.is_empty());
    let mut counts = std::collections::HashMap::new();
    for (_, t, _) in &classified {
        *counts.entry(format!("{:?}", t)).or_insert(0) += 1;
    }
    eprintln!("Punpuns classification summary: {:?}", counts);
}

#[test]
fn classify_all_pansaer_no_panics() {
    let classified = split_and_classify("pansaer");
    assert!(!classified.is_empty());
    let mut counts = std::collections::HashMap::new();
    for (_, t, _) in &classified {
        *counts.entry(format!("{:?}", t)).or_insert(0) += 1;
    }
    eprintln!("Pansaer classification summary: {:?}", counts);
}
