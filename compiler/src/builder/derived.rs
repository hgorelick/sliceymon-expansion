//! Derived structural modifiers — auto-generated from IR content.
//!
//! Character selection (Selector) and hero pool base (HeroPoolBase) can be
//! derived from the hero list. These are generated during build if no explicit
//! structural of that type exists in the IR.

use crate::ir::{Hero, Source, StructuralContent, StructuralModifier, StructuralType};

/// Generate a character selection Selector from the hero list.
/// Options are sorted by hero color alphabetically.
pub fn generate_char_selection(heroes: &[Hero]) -> StructuralModifier {
    let mut sorted: Vec<&Hero> = heroes.iter().collect();
    sorted.sort_by_key(|h| h.color);

    let mut body = String::from("1.ph.s");
    let mut options = Vec::new();

    for hero in &sorted {
        body.push_str(&format!("@1{}", hero.mn_name));
        options.push(hero.mn_name.clone());
    }

    StructuralModifier {
        modifier_type: StructuralType::Selector,
        name: None,
        content: StructuralContent::Selector { body, options },
        derived: true,
        source: Source::Base,
    }
}

/// Generate a HeroPoolBase from the hero list.
/// Lists hero internal_names as pool references.
pub fn generate_hero_pool_base(heroes: &[Hero]) -> StructuralModifier {
    let hero_refs: Vec<String> = heroes.iter().map(|h| h.internal_name.clone()).collect();
    let body = format!("heropool.{}", hero_refs.join("."));

    StructuralModifier {
        modifier_type: StructuralType::HeroPoolBase,
        name: None,
        content: StructuralContent::HeroPoolBase {
            body,
            hero_refs,
        },
        derived: true,
        source: Source::Base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{DiceFaces, HeroBlock, HeroFormat};

    fn make_hero(name: &str, color: char) -> Hero {
        Hero {
            internal_name: name.to_lowercase(),
            mn_name: name.to_string(),
            color,
            format: HeroFormat::Sliceymon,
            blocks: vec![HeroBlock {
                template: "Lost".into(),
                tier: Some(1),
                hp: Some(5),
                sd: DiceFaces::parse("0:0:0:0:0:0"),
                color: None,
                sprite_name: name.into(),
                speech: "!".into(),
                name: name.into(),
                doc: None,
                abilitydata: None,
                triggerhpdata: None,
                hue: None,
                modifier_chain: None,
                facades: vec![],
                items_inside: None,
                items_outside: None,
                img_data: Some("test".into()),
                bare: false,
            }],
            removed: false,
            source: Source::Base,
        }
    }

    #[test]
    fn generate_char_selection_from_heroes() {
        let heroes = vec![
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
            make_hero("Gamma", 'c'),
        ];
        let sel = generate_char_selection(&heroes);
        assert_eq!(sel.modifier_type, StructuralType::Selector);
        assert!(sel.derived);
        if let StructuralContent::Selector { options, .. } = &sel.content {
            assert_eq!(options.len(), 3);
            assert_eq!(options[0], "Alpha");
            assert_eq!(options[1], "Beta");
            assert_eq!(options[2], "Gamma");
        } else {
            panic!("Expected Selector content");
        }
    }

    #[test]
    fn generate_char_selection_alphabetical() {
        let heroes = vec![
            make_hero("Gamma", 'c'),
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
        ];
        let sel = generate_char_selection(&heroes);
        if let StructuralContent::Selector { options, .. } = &sel.content {
            assert_eq!(options, &["Alpha", "Beta", "Gamma"]);
        } else {
            panic!("Expected Selector content");
        }
    }

    #[test]
    fn generate_hero_pool_base_from_heroes() {
        let heroes = vec![
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
            make_hero("Gamma", 'c'),
        ];
        let pool = generate_hero_pool_base(&heroes);
        assert_eq!(pool.modifier_type, StructuralType::HeroPoolBase);
        assert!(pool.derived);
        if let StructuralContent::HeroPoolBase { hero_refs, .. } = &pool.content {
            assert_eq!(hero_refs, &["alpha", "beta", "gamma"]);
        } else {
            panic!("Expected HeroPoolBase content");
        }
    }

    #[test]
    fn char_selection_updates_on_add_hero() {
        let mut heroes = vec![
            make_hero("Alpha", 'a'),
            make_hero("Beta", 'b'),
            make_hero("Gamma", 'c'),
        ];
        heroes.push(make_hero("Delta", 'd'));
        let sel = generate_char_selection(&heroes);
        if let StructuralContent::Selector { options, .. } = &sel.content {
            assert_eq!(options.len(), 4);
        } else {
            panic!("Expected Selector content");
        }
    }

    #[test]
    fn char_selection_updates_on_remove_hero() {
        let heroes = vec![
            make_hero("Alpha", 'a'),
            make_hero("Gamma", 'c'),
        ];
        let sel = generate_char_selection(&heroes);
        if let StructuralContent::Selector { options, .. } = &sel.content {
            assert_eq!(options.len(), 2);
        } else {
            panic!("Expected Selector content");
        }
    }

    #[test]
    fn derived_flag_set_on_generated() {
        let heroes = vec![make_hero("Alpha", 'a')];
        assert!(generate_char_selection(&heroes).derived);
        assert!(generate_hero_pool_base(&heroes).derived);
    }

    #[test]
    fn builder_auto_generates_derived_structurals() {
        use std::collections::HashMap;
        use crate::ir::ModIR;

        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Alpha", 'a'));
        ir.heroes.push(make_hero("Beta", 'b'));
        ir.heroes.push(make_hero("Gamma", 'c'));

        let sprites: HashMap<String, String> = [
            ("Alpha".into(), "testimg".into()),
            ("Beta".into(), "testimg".into()),
            ("Gamma".into(), "testimg".into()),
        ].into();

        let output = crate::builder::build_complete(&ir, &sprites).unwrap();
        // Should contain auto-generated selector and hero pool
        assert!(output.contains("@1Alpha"), "missing char selection option Alpha");
        assert!(output.contains("@1Beta"), "missing char selection option Beta");
        assert!(output.contains("heropool."), "missing hero pool base");
    }
}
