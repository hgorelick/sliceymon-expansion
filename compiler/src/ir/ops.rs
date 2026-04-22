//! CRUD operations on ModIR.
//!
//! Provides add/remove/update methods for heroes, replica items, monsters, and bosses.
//! Enforces cross-category name uniqueness and hero color uniqueness.

use crate::error::CompilerError;
use super::{ModIR, Hero, ReplicaItem, Monster, Boss, Source};

impl ModIR {
    /// Check if a name is already used by any content type.
    /// Returns the category name if found.
    fn find_name_category(&self, name: &str) -> Option<&'static str> {
        let lower = name.to_lowercase();
        if self.heroes.iter().any(|h| h.mn_name.to_lowercase() == lower) {
            return Some("hero");
        }
        if self.replica_items.iter().any(|r| r.name.to_lowercase() == lower) {
            return Some("replica item");
        }
        if self.monsters.iter().any(|m| m.name.to_lowercase() == lower) {
            return Some("monster");
        }
        if self.bosses.iter().any(|b| b.name.to_lowercase() == lower) {
            return Some("boss");
        }
        None
    }

    /// Add a hero to the IR. Checks for duplicate color and cross-category name conflicts.
    pub fn add_hero(&mut self, hero: Hero) -> Result<(), CompilerError> {
        // Check cross-category name uniqueness
        if let Some(category) = self.find_name_category(&hero.mn_name) {
            return Err(CompilerError::duplicate_name(
                hero.mn_name.clone(),
                category.to_string(),
                "hero",
            ));
        }

        // Check color uniqueness
        if let Some(existing) = self.heroes.iter().find(|h| h.color == hero.color) {
            return Err(CompilerError::duplicate_color(
                hero.color,
                existing.mn_name.clone(),
            ));
        }

        let mut hero = hero;
        hero.source = Source::Custom;
        self.heroes.push(hero);
        Ok(())
    }

    /// Remove a hero by mn_name. Returns error if not found.
    pub fn remove_hero(&mut self, mn_name: &str) -> Result<(), CompilerError> {
        let lower = mn_name.to_lowercase();
        let pos = self.heroes.iter().position(|h| h.mn_name.to_lowercase() == lower);
        match pos {
            Some(i) => { self.heroes.remove(i); Ok(()) }
            None => Err(CompilerError::not_found("hero", mn_name.to_string())),
        }
    }

    /// Update a hero by mn_name (replace in-place). Returns error if not found.
    /// Checks color uniqueness against other heroes (excluding the one being updated).
    pub fn update_hero(&mut self, hero: Hero) -> Result<(), CompilerError> {
        let lower = hero.mn_name.to_lowercase();
        let pos = self.heroes.iter().position(|h| h.mn_name.to_lowercase() == lower);
        match pos {
            Some(i) => {
                // Check color uniqueness (excluding the hero being updated)
                if let Some(existing) = self.heroes.iter().enumerate()
                    .find(|(j, h)| *j != i && h.color == hero.color)
                    .map(|(_, h)| h)
                {
                    return Err(CompilerError::duplicate_color(
                        hero.color,
                        existing.mn_name.clone(),
                    ));
                }
                self.heroes[i] = hero;
                Ok(())
            }
            None => Err(CompilerError::not_found("hero", hero.mn_name.clone())),
        }
    }

    /// Add a replica item. Checks cross-category name uniqueness.
    pub fn add_replica_item(&mut self, item: ReplicaItem) -> Result<(), CompilerError> {
        if let Some(category) = self.find_name_category(&item.name) {
            return Err(CompilerError::duplicate_name(
                item.name.clone(),
                category.to_string(),
                "replica item",
            ));
        }
        let mut item = item;
        item.source = Source::Custom;
        self.replica_items.push(item);
        Ok(())
    }

    /// Remove a replica item by name.
    pub fn remove_replica_item(&mut self, name: &str) -> Result<(), CompilerError> {
        let lower = name.to_lowercase();
        let pos = self.replica_items.iter().position(|r| r.name.to_lowercase() == lower);
        match pos {
            Some(i) => { self.replica_items.remove(i); Ok(()) }
            None => Err(CompilerError::not_found("replica item", name.to_string())),
        }
    }

    /// Add a monster. Checks cross-category name uniqueness.
    pub fn add_monster(&mut self, monster: Monster) -> Result<(), CompilerError> {
        if let Some(category) = self.find_name_category(&monster.name) {
            return Err(CompilerError::duplicate_name(
                monster.name.clone(),
                category.to_string(),
                "monster",
            ));
        }
        let mut monster = monster;
        monster.source = Source::Custom;
        self.monsters.push(monster);
        Ok(())
    }

    /// Remove a monster by name.
    pub fn remove_monster(&mut self, name: &str) -> Result<(), CompilerError> {
        let lower = name.to_lowercase();
        let pos = self.monsters.iter().position(|m| m.name.to_lowercase() == lower);
        match pos {
            Some(i) => { self.monsters.remove(i); Ok(()) }
            None => Err(CompilerError::not_found("monster", name.to_string())),
        }
    }

    /// Add a boss. Checks cross-category name uniqueness.
    pub fn add_boss(&mut self, boss: Boss) -> Result<(), CompilerError> {
        if let Some(category) = self.find_name_category(&boss.name) {
            return Err(CompilerError::duplicate_name(
                boss.name.clone(),
                category.to_string(),
                "boss",
            ));
        }
        let mut boss = boss;
        boss.source = Source::Custom;
        self.bosses.push(boss);
        Ok(())
    }

    /// Remove a boss by name.
    pub fn remove_boss(&mut self, name: &str) -> Result<(), CompilerError> {
        let lower = name.to_lowercase();
        let pos = self.bosses.iter().position(|b| b.name.to_lowercase() == lower);
        match pos {
            Some(i) => { self.bosses.remove(i); Ok(()) }
            None => Err(CompilerError::not_found("boss", name.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{DiceFaces, HeroBlock, HeroFormat, ReplicaItemContainer};

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
                sprite: crate::authoring::SpriteId::owned(name.to_string(), "test"),
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
                bare: false,
            }],
            removed: false,
            source: Source::Base,
        }
    }

    fn make_monster(name: &str) -> Monster {
        Monster {
            name: name.into(),
            base_template: "Slimelet".into(),
            floor_range: "1-3".into(),
            hp: Some(3),
            sd: Some(DiceFaces::parse("0:0:0:0:0:0")),
            sprite: Some(crate::authoring::SpriteId::owned(name.to_string(), "")),
            color: None,
            doc: None,
            modifier_chain: None,
            balance: None,
            source: Source::Base,
        }
    }

    fn make_boss(name: &str) -> Boss {
        Boss {
            name: name.into(),
            level: Some(4),
            format: crate::ir::BossFormat::Standard,
            encounter_id: None,
            fights: vec![],
            doc: None,
            modifier_chain: None,
            source: Source::Base,
            event_phases: None,
        }
    }

    fn make_replica_item(name: &str) -> ReplicaItem {
        ReplicaItem {
            name: name.into(),
            container: ReplicaItemContainer::Capture { name: "Ball".into() },
            template: "Hat".into(),
            hp: None,
            sd: DiceFaces::parse("0:0:0:0:0:0"),
            sprite: crate::authoring::SpriteId::owned(name.to_string(), ""),
            color: None,
            tier: None,
            doc: None,
            speech: None,
            abilitydata: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            source: Source::Base,
        }
    }

    #[test]
    fn add_hero_to_ir() {
        let mut ir = ModIR::empty();
        ir.add_hero(make_hero("Gible", 'a')).unwrap();
        assert_eq!(ir.heroes.len(), 1);
    }

    #[test]
    fn add_hero_duplicate_color_errors() {
        let mut ir = ModIR::empty();
        ir.add_hero(make_hero("Gible", 'a')).unwrap();
        let result = ir.add_hero(make_hero("Axew", 'a'));
        assert!(result.is_err());
    }

    #[test]
    fn remove_hero_by_name() {
        let mut ir = ModIR::empty();
        ir.add_hero(make_hero("Gible", 'a')).unwrap();
        ir.remove_hero("Gible").unwrap();
        assert_eq!(ir.heroes.len(), 0);
    }

    #[test]
    fn remove_hero_not_found_errors() {
        let mut ir = ModIR::empty();
        let result = ir.remove_hero("NonExistent");
        assert!(result.is_err());
    }

    #[test]
    fn update_hero_by_name() {
        let mut ir = ModIR::empty();
        ir.add_hero(make_hero("Gible", 'a')).unwrap();
        let mut updated = make_hero("Gible", 'a');
        updated.blocks[0].hp = Some(99);
        ir.update_hero(updated).unwrap();
        assert_eq!(ir.heroes[0].blocks[0].hp, Some(99));
    }

    #[test]
    fn add_replica_item_duplicate_name_errors() {
        let mut ir = ModIR::empty();
        ir.add_hero(make_hero("Charmander", 'a')).unwrap();
        let result = ir.add_replica_item(make_replica_item("Charmander"));
        assert!(result.is_err());
    }

    #[test]
    fn remove_replica_item_by_name() {
        let mut ir = ModIR::empty();
        ir.add_replica_item(make_replica_item("Pikachu")).unwrap();
        ir.remove_replica_item("Pikachu").unwrap();
        assert_eq!(ir.replica_items.len(), 0);
    }

    #[test]
    fn add_monster() {
        let mut ir = ModIR::empty();
        ir.add_monster(make_monster("Wooper")).unwrap();
        assert_eq!(ir.monsters.len(), 1);
    }

    #[test]
    fn remove_monster() {
        let mut ir = ModIR::empty();
        ir.add_monster(make_monster("Wooper")).unwrap();
        ir.remove_monster("Wooper").unwrap();
        assert_eq!(ir.monsters.len(), 0);
    }

    #[test]
    fn add_boss() {
        let mut ir = ModIR::empty();
        ir.add_boss(make_boss("Mewtwo")).unwrap();
        assert_eq!(ir.bosses.len(), 1);
    }

    #[test]
    fn remove_boss() {
        let mut ir = ModIR::empty();
        ir.add_boss(make_boss("Mewtwo")).unwrap();
        ir.remove_boss("Mewtwo").unwrap();
        assert_eq!(ir.bosses.len(), 0);
    }

    #[test]
    fn crud_preserves_other_types() {
        let mut ir = ModIR::empty();
        ir.add_replica_item(make_replica_item("Pikachu")).unwrap();
        ir.add_monster(make_monster("Wooper")).unwrap();
        ir.add_boss(make_boss("Mewtwo")).unwrap();
        ir.add_hero(make_hero("Gible", 'a')).unwrap();
        assert_eq!(ir.replica_items.len(), 1);
        assert_eq!(ir.monsters.len(), 1);
        assert_eq!(ir.bosses.len(), 1);
        assert_eq!(ir.heroes.len(), 1);
    }
}
