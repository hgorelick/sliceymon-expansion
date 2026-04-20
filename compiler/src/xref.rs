/// Cross-reference (xref) semantic checks over a full ModIR.
///
/// The extractor pipeline IS the structural validation: if extraction succeeds,
/// the textmod is structurally valid. This module handles only cross-reference
/// checks that require seeing the full IR (name uniqueness, color conflicts,
/// hero pool reference resolution, etc.).
///
/// Pure library module: no std::fs or I/O. WASM-compatible.
use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ir::{Boss, Hero, ModIR, StructuralContent};

// ---------------------------------------------------------------------------
// Rule ID constants
// ---------------------------------------------------------------------------

const V016: &str = "V016";
const V019: &str = "V019";
const V020: &str = "V020";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Severity level for a validation finding.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    #[default]
    Error,
    Warning,
    Info,
}

/// A single validation finding (error, warning, or info).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Finding {
    pub rule_id: String,
    #[serde(default)]
    pub severity: Severity,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

/// Full validation report — errors, warnings, and informational notes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationReport {
    pub errors: Vec<Finding>,
    pub warnings: Vec<Finding>,
    pub info: Vec<Finding>,
}

impl ValidationReport {
    /// Returns true if there are zero errors (warnings are OK).
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Merge another report into this one.
    pub fn merge(&mut self, other: ValidationReport) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.info.extend(other.info);
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_ok() {
            writeln!(
                f,
                "Validation PASSED: 0 errors, {} warnings",
                self.warnings.len()
            )?;
        } else {
            writeln!(
                f,
                "Validation FAILED: {} errors, {} warnings",
                self.errors.len(),
                self.warnings.len()
            )?;
        }
        for finding in &self.errors {
            write!(f, "  ERROR [{}]", finding.rule_id)?;
            if let Some(ref name) = finding.modifier_name {
                write!(f, " {}", name)?;
            }
            if let Some(idx) = finding.modifier_index {
                write!(f, " (modifier #{})", idx)?;
            }
            writeln!(f, ": {}", finding.message)?;
            if let Some(ref ctx) = finding.context {
                writeln!(f, "         context: \"...{}...\"", ctx)?;
            }
            if let Some(ref suggestion) = finding.suggestion {
                writeln!(f, "         suggestion: {}", suggestion)?;
            }
        }
        for finding in &self.warnings {
            write!(f, "  WARN  [{}]", finding.rule_id)?;
            if let Some(ref name) = finding.modifier_name {
                write!(f, " {}", name)?;
            }
            if let Some(idx) = finding.modifier_index {
                write!(f, " (modifier #{})", idx)?;
            }
            writeln!(f, ": {}", finding.message)?;
        }
        for finding in &self.info {
            write!(f, "  INFO  [{}]", finding.rule_id)?;
            if let Some(ref name) = finding.modifier_name {
                write!(f, " {}", name)?;
            }
            writeln!(f, ": {}", finding.message)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn push_finding(report: &mut ValidationReport, finding: Finding) {
    match finding.severity {
        Severity::Error => report.errors.push(finding),
        Severity::Warning => report.warnings.push(finding),
        Severity::Info => report.info.push(finding),
    }
}

// ---------------------------------------------------------------------------
// Public API — full IR cross-reference checks
// ---------------------------------------------------------------------------

/// Run all cross-reference integrity checks on a ModIR.
///
/// These checks require seeing the full IR and cannot be performed on
/// individual items in isolation.
pub fn check_references(ir: &ModIR) -> ValidationReport {
    let mut report = ValidationReport::default();

    check_hero_color_uniqueness(ir, &mut report);
    check_cross_category_names(ir, &mut report);
    check_hero_pool_refs(ir, &mut report);

    report
}

// ---------------------------------------------------------------------------
// V019: Hero color uniqueness
// ---------------------------------------------------------------------------

/// Each hero should use a unique color slot. Two heroes sharing a color
/// is a warning (the game allows it but the second hero is unreachable
/// in the character-selection screen).
fn check_hero_color_uniqueness(ir: &ModIR, report: &mut ValidationReport) {
    let mut seen: HashMap<char, &str> = HashMap::new();

    for hero in &ir.heroes {
        if hero.removed {
            continue;
        }
        if let Some(&existing_name) = seen.get(&hero.color) {
            push_finding(report, Finding {
                rule_id: V019.to_string(),
                severity: Severity::Warning,
                message: format!(
                    "Hero '{}' uses color '{}' which is already used by '{}'",
                    hero.mn_name, hero.color, existing_name
                ),
                field_path: Some(format!("heroes[{}].color", hero.mn_name)),
                suggestion: Some(format!(
                    "Assign a unique color to '{}' (current: '{}')",
                    hero.mn_name, hero.color
                )),
                modifier_name: Some(hero.mn_name.clone()),
                ..Default::default()
            });
        } else {
            seen.insert(hero.color, &hero.mn_name);
        }
    }
}

// ---------------------------------------------------------------------------
// V020: Cross-category name uniqueness
// ---------------------------------------------------------------------------

/// A name must not appear in more than one category (hero, replica item,
/// monster, boss). This prevents game-engine confusion where the same
/// identifier resolves to different entities depending on context.
fn check_cross_category_names(ir: &ModIR, report: &mut ValidationReport) {
    // Collect names by category (case-insensitive comparison).
    // Track (original_name, category) pairs keyed by lowercase name.
    let mut name_owners: HashMap<String, Vec<(String, &str)>> = HashMap::new();

    for hero in &ir.heroes {
        if hero.removed {
            continue;
        }
        let key = hero.mn_name.to_lowercase();
        name_owners.entry(key).or_default().push((hero.mn_name.clone(), "hero"));
    }

    for item in &ir.replica_items {
        let key = item.name.to_lowercase();
        name_owners.entry(key).or_default().push((item.name.clone(), "replica_item"));
    }

    for monster in &ir.monsters {
        let key = monster.name.to_lowercase();
        name_owners.entry(key).or_default().push((monster.name.clone(), "monster"));
    }

    for boss in &ir.bosses {
        let key = boss.name.to_lowercase();
        name_owners.entry(key).or_default().push((boss.name.clone(), "boss"));
    }

    for (_key, entries) in &name_owners {
        if entries.len() > 1 {
            // Use the first original name for display
            let display_name = &entries[0].0;
            let cats: Vec<&str> = entries.iter().map(|(_, cat)| *cat).collect();
            let cats_str = cats.join(", ");
            push_finding(report, Finding {
                rule_id: V020.to_string(),
                severity: Severity::Error,
                message: format!(
                    "Name '{}' appears in multiple categories: [{}]",
                    display_name, cats_str
                ),
                field_path: Some(format!("cross_category[{}]", display_name)),
                suggestion: Some(format!(
                    "Rename the duplicate '{}' so each category has a unique name",
                    display_name
                )),
                ..Default::default()
            });
        }
    }
}

// ---------------------------------------------------------------------------
// V016: Hero pool references resolve
// ---------------------------------------------------------------------------

/// Every hero reference in a HeroPoolBase structural modifier must resolve
/// to an actual hero in the IR. Unresolved references mean the hero pool
/// will silently skip that slot at runtime.
fn check_hero_pool_refs(ir: &ModIR, report: &mut ValidationReport) {
    // Build a set of known hero internal_names (lowercase for case-insensitive match)
    let known_heroes: HashSet<String> = ir
        .heroes
        .iter()
        .filter(|h| !h.removed)
        .map(|h| h.internal_name.to_lowercase())
        .collect();

    for structural in &ir.structural {
        if let StructuralContent::HeroPoolBase { ref hero_refs, .. } = structural.content {
            for href in hero_refs {
                let key = href.to_lowercase();
                if !known_heroes.contains(&key) {
                    push_finding(report, Finding {
                        rule_id: V016.to_string(),
                        severity: Severity::Error,
                        message: format!(
                            "Hero pool reference '{}' does not resolve to any hero",
                            href
                        ),
                        field_path: Some("structural[HeroPoolBase].hero_refs".to_string()),
                        suggestion: Some(format!(
                            "Add a hero with internal_name '{}' or remove the reference",
                            href
                        )),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Single-item context checks
// ---------------------------------------------------------------------------

/// Validate a single hero against the rest of the IR.
///
/// Checks whether this hero's color or name conflicts with existing entries.
/// Does NOT add the hero to the IR — the caller is responsible for that.
pub fn check_hero_in_context(hero: &Hero, ir: &ModIR) -> ValidationReport {
    let mut report = ValidationReport::default();

    // Color conflict
    for existing in &ir.heroes {
        if existing.removed {
            continue;
        }
        if existing.color == hero.color && existing.mn_name != hero.mn_name {
            push_finding(&mut report, Finding {
                rule_id: V019.to_string(),
                severity: Severity::Warning,
                message: format!(
                    "Hero '{}' uses color '{}' which is already used by '{}'",
                    hero.mn_name, hero.color, existing.mn_name
                ),
                field_path: Some(format!("hero[{}].color", hero.mn_name)),
                suggestion: Some(format!(
                    "Assign a unique color to '{}' (current: '{}')",
                    hero.mn_name, hero.color
                )),
                modifier_name: Some(hero.mn_name.clone()),
                ..Default::default()
            });
        }
    }

    // Cross-category name conflict
    let lower = hero.mn_name.to_lowercase();

    if ir.replica_items.iter().any(|r| r.name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: Severity::Error,
            message: format!(
                "Hero name '{}' conflicts with an existing replica item",
                hero.mn_name
            ),
            field_path: Some(format!("hero[{}].mn_name", hero.mn_name)),
            suggestion: Some(format!(
                "Rename the hero or replica item so '{}' is unique across categories",
                hero.mn_name
            )),
            modifier_name: Some(hero.mn_name.clone()),
            ..Default::default()
        });
    }

    if ir.monsters.iter().any(|m| m.name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: Severity::Error,
            message: format!(
                "Hero name '{}' conflicts with an existing monster",
                hero.mn_name
            ),
            field_path: Some(format!("hero[{}].mn_name", hero.mn_name)),
            suggestion: Some(format!(
                "Rename the hero or monster so '{}' is unique across categories",
                hero.mn_name
            )),
            modifier_name: Some(hero.mn_name.clone()),
            ..Default::default()
        });
    }

    if ir.bosses.iter().any(|b| b.name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: Severity::Error,
            message: format!(
                "Hero name '{}' conflicts with an existing boss",
                hero.mn_name
            ),
            field_path: Some(format!("hero[{}].mn_name", hero.mn_name)),
            suggestion: Some(format!(
                "Rename the hero or boss so '{}' is unique across categories",
                hero.mn_name
            )),
            modifier_name: Some(hero.mn_name.clone()),
            ..Default::default()
        });
    }

    report
}

/// Validate a single boss against the rest of the IR.
///
/// Checks whether this boss's name conflicts with existing entries.
pub fn check_boss_in_context(boss: &Boss, ir: &ModIR) -> ValidationReport {
    let mut report = ValidationReport::default();

    let lower = boss.name.to_lowercase();

    if ir.heroes.iter().any(|h| !h.removed && h.mn_name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: Severity::Error,
            message: format!(
                "Boss name '{}' conflicts with an existing hero",
                boss.name
            ),
            field_path: Some(format!("boss[{}].name", boss.name)),
            suggestion: Some(format!(
                "Rename the boss or hero so '{}' is unique across categories",
                boss.name
            )),
            modifier_name: Some(boss.name.clone()),
            ..Default::default()
        });
    }

    if ir.replica_items.iter().any(|r| r.name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: Severity::Error,
            message: format!(
                "Boss name '{}' conflicts with an existing replica item",
                boss.name
            ),
            field_path: Some(format!("boss[{}].name", boss.name)),
            suggestion: Some(format!(
                "Rename the boss or replica item so '{}' is unique across categories",
                boss.name
            )),
            modifier_name: Some(boss.name.clone()),
            ..Default::default()
        });
    }

    if ir.monsters.iter().any(|m| m.name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: Severity::Error,
            message: format!(
                "Boss name '{}' conflicts with an existing monster",
                boss.name
            ),
            field_path: Some(format!("boss[{}].name", boss.name)),
            suggestion: Some(format!(
                "Rename the boss or monster so '{}' is unique across categories",
                boss.name
            )),
            modifier_name: Some(boss.name.clone()),
            ..Default::default()
        });
    }

    report
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::*;

    /// Helper: create a minimal hero for testing.
    fn make_hero(name: &str, color: char) -> Hero {
        Hero {
            internal_name: name.to_lowercase(),
            mn_name: name.to_string(),
            color,
            format: HeroFormat::default(),
            blocks: vec![],
            removed: false,
            source: Source::Custom,
        }
    }

    /// Helper: create a minimal replica item for testing.
    fn make_replica_item(name: &str) -> ReplicaItem {
        ReplicaItem {
            name: name.to_string(),
            container_name: name.to_string(),
            template: "Slime".to_string(),
            hp: Some(4),
            sd: DiceFaces { faces: vec![DiceFace::Blank] },
            sprite_name: name.to_lowercase(),
            color: None,
            tier: None,
            doc: None,
            speech: None,
            abilitydata: None,
            item_modifiers: None,
            sticker: None,
            toggle_flags: None,
            img_data: None,
            source: Source::Custom,
        }
    }

    /// Helper: create a minimal boss for testing.
    fn make_boss(name: &str) -> Boss {
        Boss {
            name: name.to_string(),
            level: Some(5),
            format: BossFormat::default(),
            encounter_id: None,
            fights: vec![],
            event_phases: None,
            doc: None,
            modifier_chain: None,
            source: Source::Custom,
        }
    }

    /// Helper: create a HeroPoolBase structural modifier referencing given hero names.
    fn make_hero_pool(refs: Vec<&str>) -> StructuralModifier {
        StructuralModifier {
            modifier_type: StructuralType::HeroPoolBase,
            name: Some("heropool".to_string()),
            content: StructuralContent::HeroPoolBase {
                body: String::new(),
                hero_refs: refs.into_iter().map(|s| s.to_string()).collect(),
            },
            derived: false,
            source: Source::Custom,
        }
    }

    // -- V019: Hero color uniqueness --

    #[test]
    fn test_v019_duplicate_hero_color() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));
        ir.heroes.push(make_hero("Pikachu", 'a'));

        let report = check_references(&ir);

        assert_eq!(report.warnings.len(), 1, "Expected exactly one warning for duplicate color");
        assert_eq!(report.warnings[0].rule_id, "V019");
        assert!(
            report.warnings[0].message.contains("Pikachu"),
            "Warning should mention the duplicate hero"
        );
        assert!(
            report.warnings[0].field_path.is_some(),
            "field_path must be populated"
        );
        assert!(
            report.warnings[0].suggestion.is_some(),
            "suggestion must be populated"
        );
    }

    #[test]
    fn test_v019_unique_colors_no_warning() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));
        ir.heroes.push(make_hero("Pikachu", 'b'));

        let report = check_references(&ir);
        assert!(report.warnings.is_empty(), "No warnings for unique colors");
    }

    #[test]
    fn test_v019_removed_hero_ignored() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));
        let mut removed = make_hero("Pikachu", 'a');
        removed.removed = true;
        ir.heroes.push(removed);

        let report = check_references(&ir);
        assert!(report.warnings.is_empty(), "Removed heroes should not trigger color conflict");
    }

    // -- V020: Cross-category name uniqueness --

    #[test]
    fn test_v020_cross_category_duplicate() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Pikachu", 'a'));
        ir.replica_items.push(make_replica_item("Pikachu"));

        let report = check_references(&ir);

        assert_eq!(report.errors.len(), 1, "Expected exactly one error for cross-category duplicate");
        assert_eq!(report.errors[0].rule_id, "V020");
        assert!(
            report.errors[0].message.contains("Pikachu"),
            "Error should mention the conflicting name"
        );
        assert!(
            report.errors[0].field_path.is_some(),
            "field_path must be populated"
        );
        assert!(
            report.errors[0].suggestion.is_some(),
            "suggestion must be populated"
        );
    }

    #[test]
    fn test_v020_case_insensitive() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("PIKACHU", 'a'));
        ir.replica_items.push(make_replica_item("pikachu"));

        let report = check_references(&ir);
        assert_eq!(report.errors.len(), 1, "Cross-category check should be case-insensitive");
    }

    #[test]
    fn test_v020_different_names_no_error() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));
        ir.replica_items.push(make_replica_item("Pikachu"));
        ir.monsters.push(Monster {
            name: "Weedle".to_string(),
            base_template: "Slime".to_string(),
            floor_range: "1-3".to_string(),
            hp: Some(4),
            sd: None,
            sprite_name: None,
            color: None,
            doc: None,
            modifier_chain: None,
            balance: None,
            img_data: None,
            source: Source::Custom,
        });

        let report = check_references(&ir);
        assert!(report.errors.is_empty(), "No errors when all names are unique");
    }

    // -- V016: Hero pool references --

    #[test]
    fn test_v016_unresolved_hero_pool_ref() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));
        ir.structural.push(make_hero_pool(vec!["charmander", "nonexistent"]));

        let report = check_references(&ir);

        assert_eq!(report.errors.len(), 1, "Expected one error for unresolved hero pool ref");
        assert_eq!(report.errors[0].rule_id, "V016");
        assert!(
            report.errors[0].message.contains("nonexistent"),
            "Error should mention the unresolved reference"
        );
        assert!(
            report.errors[0].field_path.is_some(),
            "field_path must be populated"
        );
        assert!(
            report.errors[0].suggestion.is_some(),
            "suggestion must be populated"
        );
    }

    #[test]
    fn test_v016_all_refs_resolve() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));
        ir.heroes.push(make_hero("Pikachu", 'b'));
        ir.structural.push(make_hero_pool(vec!["charmander", "pikachu"]));

        let report = check_references(&ir);
        let pool_errors: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "V016").collect();
        assert!(pool_errors.is_empty(), "All refs resolve — no V016 errors");
    }

    // -- Empty IR --

    #[test]
    fn test_empty_ir_no_findings() {
        let ir = ModIR::empty();
        let report = check_references(&ir);

        assert!(report.errors.is_empty(), "Empty IR should produce no errors");
        assert!(report.warnings.is_empty(), "Empty IR should produce no warnings");
        assert!(report.info.is_empty(), "Empty IR should produce no info");
    }

    // -- Single-item context checks --

    #[test]
    fn test_check_hero_in_context_color_conflict() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));

        let new_hero = make_hero("Pikachu", 'a');
        let report = check_hero_in_context(&new_hero, &ir);

        assert_eq!(report.warnings.len(), 1, "Should detect color conflict");
        assert_eq!(report.warnings[0].rule_id, "V019");
        assert!(report.warnings[0].message.contains("Pikachu"));
        assert!(report.warnings[0].message.contains("Charmander"));
    }

    #[test]
    fn test_check_hero_in_context_name_conflict_with_replica() {
        let mut ir = ModIR::empty();
        ir.replica_items.push(make_replica_item("Pikachu"));

        let new_hero = make_hero("Pikachu", 'a');
        let report = check_hero_in_context(&new_hero, &ir);

        assert_eq!(report.errors.len(), 1, "Should detect cross-category name conflict");
        assert_eq!(report.errors[0].rule_id, "V020");
    }

    #[test]
    fn test_check_hero_in_context_no_conflicts() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));

        let new_hero = make_hero("Pikachu", 'b');
        let report = check_hero_in_context(&new_hero, &ir);

        assert!(report.errors.is_empty());
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn test_check_boss_in_context_name_conflict_with_hero() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Pikachu", 'a'));

        let boss = make_boss("Pikachu");
        let report = check_boss_in_context(&boss, &ir);

        assert_eq!(report.errors.len(), 1, "Should detect cross-category name conflict");
        assert_eq!(report.errors[0].rule_id, "V020");
    }

    #[test]
    fn test_check_boss_in_context_no_conflicts() {
        let ir = ModIR::empty();

        let boss = make_boss("Floor8");
        let report = check_boss_in_context(&boss, &ir);

        assert!(report.errors.is_empty());
        assert!(report.warnings.is_empty());
    }
}
