use textmod_compiler::extractor::classifier::{classify, ModifierType};
use textmod_compiler::extractor::splitter::split_modifiers;

fn load_mod(name: &str) -> String {
    let path = format!("../working-mods/{}.txt", name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to load {}: {}", path, e))
}

#[test]
fn replica_item_pools_classified_as_itempool() {
    let text = load_mod("sliceymon");
    let mods = split_modifiers(&text).unwrap();
    // All replica item pools are classified as ItemPool
    let itempools: Vec<_> = mods.iter().enumerate()
        .filter(|(i, m)| classify(m, *i).unwrap() == ModifierType::ItemPool)
        .collect();
    assert!(itempools.len() >= 2, "Should have at least 2 itempools (replica item pools)");

    // None should be classified as ReplicaItem or ReplicaItemWithAbility
    for (i, m) in mods.iter().enumerate() {
        let t = classify(m, i).unwrap();
        assert_ne!(t, ModifierType::ReplicaItem, "No modifiers should classify as ReplicaItem");
        assert_ne!(t, ModifierType::ReplicaItemWithAbility, "No modifiers should classify as ReplicaItemWithAbility");
    }
}

#[test]
fn replica_item_parser_works_on_simple_input() {
    // The replica item parser still works for CRUD-created individual entries
    use textmod_compiler::extractor::replica_item_parser;
    let simple = "itempool.((hat.replica.Thief.sd.34-1:0:0:0:0:0.n.Pikachu)).n.PokeBall.mn.Pikachu";
    let item = replica_item_parser::parse_simple(simple, 0);
    assert_eq!(item.name, "Pikachu");
    assert_eq!(item.container_name, "PokeBall");
    assert!(!item.sd.faces.is_empty());
}

#[test]
fn replica_item_with_ability_parser_works() {
    use textmod_compiler::extractor::replica_item_parser;
    let simple = "itempool.((hat.(replica.Alpha.sd.34-3:0:0:0:0:0.n.Mewtwo.cast.(Fey.sd.34-1:0.img.spark.n.Psychic)))).n.MasterBall.mn.Mewtwo";
    let item = replica_item_parser::parse_with_ability(simple, 0);
    assert_eq!(item.name, "Mewtwo");
    assert_eq!(item.container_name, "MasterBall");
}
