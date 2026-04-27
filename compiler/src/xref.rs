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

use crate::authoring::FaceIdValue;
use crate::ir::{Boss, DiceFace, DiceFaces, FightUnit, Hero, ModIR, Source, StructuralContent};

pub use crate::finding::{Finding, Severity};

// ---------------------------------------------------------------------------
// Rule ID constants
// ---------------------------------------------------------------------------

const V016: &str = "V016";
const V019: &str = "V019";
const V020: &str = "V020";
pub const X003: &str = "X003";
pub const X016: &str = "X016";
pub const X017: &str = "X017";

/// X016 — template-restricted FaceID table. The authoritative source is
/// `reference/textmod_guide.md`. Today the guide does not make a per-FaceID
/// template-restriction claim, so this table is intentionally empty —
/// populating it with corpus-derived guesses would violate the plan's
/// "no hardcoded lists based on game-design persona claims" directive
/// (PLATFORM_FOUNDATIONS_PLAN.md §F3). Add entries here only when the guide
/// documents a restriction.
///
/// Each entry: `(face_id, &[allowed_template_prefix, ...])`. An entry with
/// an empty `allowed_template_prefix` slice means the FaceID is blocked on
/// every template — useful for "enemy-only" faces once the guide lists any.
pub const X016_TEMPLATE_RESTRICTIONS: &[(u16, &[&str])] = &[];

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Source-aware severity promotion — every xref rule that visits a sourced
/// entity runs its literal `severity` through this helper so the promotion
/// policy lives in one place (per PLATFORM_FOUNDATIONS_PLAN §F5 and the
/// Chunk 3b lesson on not duplicating the same incantation across N sites).
///
/// Policy:
/// - `Some(Source::Base)` → `Severity::Warning` (base content is load-bearing;
///   violations are informative, not blocking).
/// - `Some(Source::Custom | Source::Overlay)` → `Severity::Error` (author-added
///   violations should block).
/// - `None` → `base` unchanged (global findings carry the rule's native severity).
pub fn promote_severity(base: Severity, src: Option<Source>) -> Severity {
    match src {
        Some(Source::Base) => Severity::Warning,
        Some(Source::Custom) | Some(Source::Overlay) => Severity::Error,
        None => base,
    }
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
    check_duplicate_pokemon_buckets(ir, &mut report);
    check_hero_pool_refs(ir, &mut report);
    check_face_template_compat(ir, &mut report);
    check_face_unknown(ir, &mut report);

    report
}

// ---------------------------------------------------------------------------
// X003: No duplicate Pokemon across heroes / legendaries / monsters
// ---------------------------------------------------------------------------

/// SPEC §6.3 — "A Pokemon may exist in at most one of: heroes, replica items
/// (captures / legendaries), monsters."
///
/// Post-Chunk-8A, `ReplicaItem` models trigger-based summons
/// (`SummonTrigger::SideUse` and `SummonTrigger::Cast`) — both bucketed under
/// the `"legendary"` label here so X003 collects buckets as
/// `{hero, legendary, monster}`. The label is preserved across the 8A rewrite
/// for stability of the paired test (`x003_duplicate_pokemon_across_kinds`
/// asserts `message.contains("legendary")`) and the SPEC §6.3 prose surface;
/// 8B unifies it to `"replica_item"` per `plans/CHUNK_8B…` §9. The bucket set
/// remains narrower than V020's `{hero, replica_item, monster, boss}` — not
/// more granular: `legendary` and `replica_item` carry the same information
/// one-to-one, and X003 deliberately excludes `boss` per SPEC §6.3.
fn check_duplicate_pokemon_buckets(ir: &ModIR, report: &mut ValidationReport) {
    // (bucket_label, original_name) pairs keyed by lowercase name.
    let mut owners: HashMap<String, Vec<(String, &'static str)>> = HashMap::new();

    for hero in &ir.heroes {
        if hero.removed {
            continue;
        }
        owners
            .entry(hero.mn_name.to_lowercase())
            .or_default()
            .push((hero.mn_name.clone(), "hero"));
    }
    for item in &ir.replica_items {
        // Bucket label remains "legendary" in 8A — unifying both owner-map
        // sites to "replica_item" is 8B's scope (see plans/CHUNK_8B §9).
        // Unilaterally renaming here would break a paired test
        // (x003_duplicate_pokemon_across_kinds asserts
        // `message.contains("legendary")`) and a SPEC prose surface in one
        // atomic commit that 8A does not own.
        owners
            .entry(item.target_name.to_lowercase())
            .or_default()
            .push((item.target_name.clone(), "legendary"));
    }
    for monster in &ir.monsters {
        owners
            .entry(monster.name.to_lowercase())
            .or_default()
            .push((monster.name.clone(), "monster"));
    }

    // Sort keys so finding order is deterministic when multiple duplicates
    // exist (HashMap iteration order is randomized per-process).
    let mut keys: Vec<&String> = owners.keys().collect();
    keys.sort();

    for key in keys {
        let entries = &owners[key];
        let mut buckets: Vec<&str> = entries.iter().map(|(_, b)| *b).collect();
        buckets.sort();
        buckets.dedup();
        // X003 is a *cross-bucket* check per SPEC §6.3 ("a Pokemon may exist
        // in at most one of"). Intra-bucket duplicates (e.g., two heroes with
        // the same name) are V-rule territory; don't emit a confusing
        // `buckets: [hero]` finding here.
        if buckets.len() < 2 {
            continue;
        }
        let display_name = &entries[0].0;
        // X003 is a global cross-bucket finding: there is no single offending
        // entity whose `source` would be authoritative. Mirrors the Chunk 4
        // precedent for V020's `check_cross_category_names` (plan §F5 —
        // "global — source: None because there is no single offending entity;
        // severity stays Error"). `promote_severity(Error, None)` = Error.
        push_finding(report, Finding {
            rule_id: X003.to_string(),
            severity: promote_severity(Severity::Error, None),
            message: format!(
                "Pokemon '{}' appears in multiple buckets: [{}] (SPEC §6.3)",
                display_name,
                buckets.join(", ")
            ),
            field_path: Some(format!("pokemon_buckets[{}]", display_name)),
            modifier_name: Some(display_name.clone()),
            suggestion: Some(format!(
                "Rename one of the colliding '{}' entries so the name appears in at \
                 most one of these buckets: {}",
                display_name,
                buckets.join(" / ")
            )),
            source: None,
            ..Default::default()
        });
    }
}

// ---------------------------------------------------------------------------
// X016 / X017: dice-face rules
// ---------------------------------------------------------------------------

/// Walk every `DiceFaces` in the IR, yielding `(field_path, &DiceFaces, template)`
/// tuples so face-level rules don't have to reimplement the traversal.
///
/// Covered surfaces (every `DiceFaces` field reachable from `ModIR`):
/// - `heroes[].blocks[].sd`
/// - `heroes[].blocks[].abilitydata.sd`
/// - `heroes[].blocks[].triggerhpdata.sd`
/// - `replica_items[].sd`
/// - `monsters[].sd`
/// - `bosses[].fights[].enemies[].sd` (recursively into `nested_units`)
fn iter_dice_faces<'a>(ir: &'a ModIR) -> Vec<(String, &'a DiceFaces, &'a str, Source)> {
    let mut out: Vec<(String, &'a DiceFaces, &'a str, Source)> = Vec::new();
    for hero in &ir.heroes {
        if hero.removed {
            continue;
        }
        for (i, block) in hero.blocks.iter().enumerate() {
            out.push((
                format!("heroes[{}].blocks[{}].sd", hero.mn_name, i),
                &block.sd,
                block.template.as_str(),
                hero.source,
            ));
            if let Some(ability) = &block.abilitydata {
                out.push((
                    format!("heroes[{}].blocks[{}].abilitydata.sd", hero.mn_name, i),
                    &ability.sd,
                    ability.template.as_str(),
                    hero.source,
                ));
            }
            if let Some(trigger) = &block.triggerhpdata {
                if let Some(sd) = &trigger.sd {
                    out.push((
                        format!("heroes[{}].blocks[{}].triggerhpdata.sd", hero.mn_name, i),
                        sd,
                        trigger.template.as_str(),
                        hero.source,
                    ));
                }
            }
        }
    }
    for item in &ir.replica_items {
        // 8A stubs face-template-compat's `template` key to the lowercase
        // literal `"thief"` — the retired template field on ReplicaItem
        // carried `"thief"` for every corpus summon (chunk-impl §3.3). 8B's xref
        // bucket-routing rewrite resolves the capital-Thief vs lowercase-thief
        // asymmetry between this lookup key and the emitter's `"Thief"` literal.
        // Dice access routes through the shared accessor — no variant branching.
        out.push((
            format!("replica_items[{}].sd", item.target_name),
            item.trigger.dice_faces(),
            "thief",
            item.source,
        ));
    }
    for monster in &ir.monsters {
        if let Some(sd) = &monster.sd {
            out.push((
                format!("monsters[{}].sd", monster.name),
                sd,
                monster.base_template.as_str(),
                monster.source,
            ));
        }
    }
    for boss in &ir.bosses {
        for (fi, fight) in boss.fights.iter().enumerate() {
            for (ei, enemy) in fight.enemies.iter().enumerate() {
                let base = format!(
                    "bosses[{}].fights[{}].enemies[{}]",
                    boss.name, fi, ei
                );
                collect_fight_unit_faces(enemy, &base, boss.source, &mut out);
            }
        }
    }
    out
}

/// Recursive helper — yields `(path, &DiceFaces, template, source)` for a
/// `FightUnit` and any `nested_units` below it. `FightUnit` has no provenance
/// of its own, so the source is inherited from the owning `Boss`.
fn collect_fight_unit_faces<'a>(
    unit: &'a FightUnit,
    path_prefix: &str,
    source: Source,
    out: &mut Vec<(String, &'a DiceFaces, &'a str, Source)>,
) {
    if let Some(sd) = &unit.sd {
        out.push((format!("{}.sd", path_prefix), sd, unit.template.as_str(), source));
    }
    if let Some(nested) = &unit.nested_units {
        for (ni, child) in nested.iter().enumerate() {
            let child_prefix = format!("{}.nested_units[{}]", path_prefix, ni);
            collect_fight_unit_faces(child, &child_prefix, source, out);
        }
    }
}

/// X016 — flag a `FaceIdValue::Known` face whose ID is template-restricted by
/// `reference/textmod_guide.md` when used on a disallowed template. Operates
/// on `Known` only; `Unknown` is handled by X017.
///
/// Today `X016_TEMPLATE_RESTRICTIONS` is empty (the guide does not make
/// per-FaceID template-restriction claims), so this rule fires only when the
/// table is populated by a future guide update. See the table's doc-comment
/// for why this is intentional.
fn check_face_template_compat(ir: &ModIR, report: &mut ValidationReport) {
    check_face_template_compat_with_table(ir, X016_TEMPLATE_RESTRICTIONS, report);
}

/// Testable variant of [`check_face_template_compat`] that accepts an injected
/// restriction table. Used by unit tests to verify the rule framework fires
/// when the table is populated, without hardcoding production claims.
pub(crate) fn check_face_template_compat_with_table(
    ir: &ModIR,
    restrictions: &[(u16, &[&str])],
    report: &mut ValidationReport,
) {
    if restrictions.is_empty() {
        return;
    }
    for (field_path, sd, template, source) in iter_dice_faces(ir) {
        for (face_idx, face) in sd.faces.iter().enumerate() {
            if let DiceFace::Active { face_id: FaceIdValue::Known(id), .. } = face {
                let raw = id.raw();
                if let Some((_, allowed)) = restrictions.iter().find(|(rid, _)| *rid == raw) {
                    let template_ok = allowed.iter().any(|prefix| template.starts_with(prefix));
                    if !template_ok {
                        push_finding(report, Finding {
                            rule_id: X016.to_string(),
                            severity: promote_severity(Severity::Error, Some(source)),
                            message: format!(
                                "FaceID {} is template-restricted by reference/textmod_guide.md; \
                                 template '{}' is not in the allowed set",
                                raw, template
                            ),
                            field_path: Some(format!("{}.faces[{}]", field_path, face_idx)),
                            suggestion: Some(format!(
                                "Use FaceID {} only on templates matching one of: [{}]",
                                raw,
                                allowed.join(", ")
                            )),
                            source: Some(source),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }
}

/// X017 — warn on a `FaceIdValue::Unknown(raw)`, i.e. a FaceID that extracted
/// successfully (permissive path per SPEC §3.6) but is not in the corpus
/// whitelist. Severity is Warning — extraction still round-trips byte-for-byte.
fn check_face_unknown(ir: &ModIR, report: &mut ValidationReport) {
    for (field_path, sd, _template, source) in iter_dice_faces(ir) {
        for (face_idx, face) in sd.faces.iter().enumerate() {
            if let DiceFace::Active { face_id: FaceIdValue::Unknown(raw), .. } = face {
                push_finding(report, Finding {
                    rule_id: X017.to_string(),
                    severity: promote_severity(Severity::Warning, Some(source)),
                    message: format!(
                        "FaceID {} is not in the corpus whitelist; roundtrip is preserved \
                         but the authoring layer has no typed const for this ID",
                        raw
                    ),
                    field_path: Some(format!("{}.faces[{}]", field_path, face_idx)),
                    suggestion: Some(format!(
                        "If FaceID {} is a legitimate game FaceID, add it to the \
                         build.rs curated table; otherwise verify the source textmod",
                        raw
                    )),
                    source: Some(source),
                    ..Default::default()
                });
            }
        }
    }
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
                severity: promote_severity(Severity::Warning, Some(hero.source)),
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
                source: Some(hero.source),
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
        let key = item.target_name.to_lowercase();
        name_owners
            .entry(key)
            .or_default()
            .push((item.target_name.clone(), "replica_item"));
    }

    for monster in &ir.monsters {
        let key = monster.name.to_lowercase();
        name_owners.entry(key).or_default().push((monster.name.clone(), "monster"));
    }

    for boss in &ir.bosses {
        let key = boss.name.to_lowercase();
        name_owners.entry(key).or_default().push((boss.name.clone(), "boss"));
    }

    for entries in name_owners.values() {
        if entries.len() > 1 {
            // Use the first original name for display
            let display_name = &entries[0].0;
            let cats: Vec<&str> = entries.iter().map(|(_, cat)| *cat).collect();
            let cats_str = cats.join(", ");
            push_finding(report, Finding {
                rule_id: V020.to_string(),
                severity: promote_severity(Severity::Error, None),
                message: format!(
                    "Name '{}' appears in multiple categories: [{}]",
                    display_name, cats_str
                ),
                field_path: Some(format!("cross_category[{}]", display_name)),
                suggestion: Some(format!(
                    "Rename the duplicate '{}' so each category has a unique name",
                    display_name
                )),
                source: None,
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
                        severity: promote_severity(Severity::Error, Some(structural.source)),
                        message: format!(
                            "Hero pool reference '{}' does not resolve to any hero",
                            href
                        ),
                        field_path: Some("structural[HeroPoolBase].hero_refs".to_string()),
                        suggestion: Some(format!(
                            "Add a hero with internal_name '{}' or remove the reference",
                            href
                        )),
                        source: Some(structural.source),
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
                severity: promote_severity(Severity::Warning, Some(hero.source)),
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
                source: Some(hero.source),
                ..Default::default()
            });
        }
    }

    // Cross-category name conflict
    let lower = hero.mn_name.to_lowercase();

    if ir.replica_items.iter().any(|r| r.target_name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: promote_severity(Severity::Error, Some(hero.source)),
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
            source: Some(hero.source),
            ..Default::default()
        });
    }

    if ir.monsters.iter().any(|m| m.name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: promote_severity(Severity::Error, Some(hero.source)),
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
            source: Some(hero.source),
            ..Default::default()
        });
    }

    if ir.bosses.iter().any(|b| b.name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: promote_severity(Severity::Error, Some(hero.source)),
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
            source: Some(hero.source),
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
            severity: promote_severity(Severity::Error, Some(boss.source)),
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
            source: Some(boss.source),
            ..Default::default()
        });
    }

    if ir.replica_items.iter().any(|r| r.target_name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: promote_severity(Severity::Error, Some(boss.source)),
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
            source: Some(boss.source),
            ..Default::default()
        });
    }

    if ir.monsters.iter().any(|m| m.name.to_lowercase() == lower) {
        push_finding(&mut report, Finding {
            rule_id: V020.to_string(),
            severity: promote_severity(Severity::Error, Some(boss.source)),
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
            source: Some(boss.source),
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
    ///
    /// Defaults to `Source::Base` so severity-promotion (per §F5) leaves the
    /// rule's native severity intact; tests that exercise the Custom/Overlay
    /// escalation set `source` explicitly.
    fn make_hero(name: &str, color: char) -> Hero {
        Hero {
            internal_name: name.to_lowercase(),
            mn_name: name.to_string(),
            color,
            format: HeroFormat::default(),
            blocks: vec![],
            removed: false,
            source: Source::Base,
        }
    }

    /// Helper: create a minimal replica item for testing.
    ///
    /// Chunk 8A: trigger-IR shape. Dice are blank (one `DiceFace::Blank`)
    /// because the legacy helper's `sd: DiceFaces { faces: vec![DiceFace::Blank] }`
    /// shape is used by X017 `x017_silent_when_all_face_ids_known` negative
    /// assertions — the shape must still produce an all-blank `DiceFaces`
    /// so nothing in the face iteration flags a Known or Unknown face.
    fn make_replica_item(name: &str) -> ReplicaItem {
        ReplicaItem {
            container_name: "Test Ball".to_string(),
            target_name: name.to_string(),
            trigger: SummonTrigger::SideUse {
                dice: DiceFaces { faces: vec![DiceFace::Blank] },
                dice_location: DiceLocation::OuterPreface,
            },
            enemy_template: "Wolf".to_string(),
            team_template: "housecat".to_string(),
            tier: None,
            hp: Some(4),
            color: None,
            sprite: crate::authoring::SpriteId::owned(name.to_lowercase(), ""),
            sticker_stack: None,
            speech: None,
            doc: None,
            toggle_flags: None,
            item_modifiers: None,
            source: Source::Base,
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
            source: Source::Base,
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
            source: Source::Base,
        }
    }

    /// Test helper: every finding in the report regardless of severity lane.
    /// Used by tests that assert a rule fires without caring about the §F5
    /// severity-promotion policy (which routes findings to different lanes
    /// based on the offending entity's `source`).
    fn all_findings(report: &ValidationReport) -> Vec<&Finding> {
        report
            .errors
            .iter()
            .chain(report.warnings.iter())
            .chain(report.info.iter())
            .collect()
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

        // X003 (per-bucket) also fires on this IR; filter to V020 alone here.
        let v020: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "V020").collect();
        assert_eq!(v020.len(), 1, "Expected exactly one V020 finding for cross-category duplicate");
        assert!(
            v020[0].message.contains("Pikachu"),
            "Error should mention the conflicting name"
        );
        assert!(
            v020[0].field_path.is_some(),
            "field_path must be populated"
        );
        assert!(
            v020[0].suggestion.is_some(),
            "suggestion must be populated"
        );
    }

    #[test]
    fn test_v020_case_insensitive() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("PIKACHU", 'a'));
        ir.replica_items.push(make_replica_item("pikachu"));

        let report = check_references(&ir);
        let v020: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "V020").collect();
        assert_eq!(v020.len(), 1, "Cross-category check should be case-insensitive");
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
            sprite: None,
            color: None,
            doc: None,
            modifier_chain: None,
            balance: None,
            source: Source::Custom,
        });

        let report = check_references(&ir);
        assert!(report.errors.is_empty(), "No errors when all names are unique");
    }

    // -- X003: No duplicate Pokemon across hero/legendary/monster buckets --
    //
    // Post-Chunk-8A, `ReplicaItem` models trigger-based summons (SideUse /
    // Cast) bucketed under the "legendary" label. The former `capture`
    // bucket was retired upstream per chunk-impl rule 3 — no corpus instance.

    #[test]
    fn x003_duplicate_pokemon_across_kinds() {
        // Pikachu as both Hero and replica item (legendary bucket) — X003
        // must fire even though V020 also catches this case. X003 reports
        // per-bucket granularity. The bucket label "legendary" is preserved
        // across the 8A rewrite per the function-doc on
        // check_duplicate_pokemon_buckets; the IR itself is no longer
        // Legendary-only (SummonTrigger::SideUse / Cast).
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Pikachu", 'a'));
        ir.replica_items.push(make_replica_item("Pikachu"));

        let report = check_references(&ir);

        let x003: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "X003").collect();
        assert_eq!(x003.len(), 1, "expected one X003 finding");
        assert!(x003[0].message.contains("Pikachu"));
        assert!(x003[0].message.contains("hero"));
        assert!(x003[0].message.contains("legendary"));
        assert_eq!(x003[0].modifier_name.as_deref(), Some("Pikachu"));
    }

    #[test]
    fn x003_silent_when_names_unique() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));
        ir.replica_items.push(make_replica_item("Pikachu"));
        ir.replica_items.push(make_replica_item("Mew"));

        let report = check_references(&ir);
        let x003: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "X003").collect();
        assert!(x003.is_empty(), "unique names across buckets must produce no X003");
    }

    #[test]
    fn x003_finding_is_global_source_none() {
        // X003 is a cross-bucket finding: no single offending entity, so
        // `source` stays None and severity stays Error — mirrors V020's
        // cross-category behavior (Chunk 4 §F5). Pin this so a future
        // "retrofit source on every X-rule" sweep doesn't silently attribute
        // the collision to one entity's bucket.
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Pikachu", 'a'));
        ir.replica_items.push(make_replica_item("Pikachu"));

        let report = check_references(&ir);
        let x003: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "X003").collect();
        assert_eq!(x003.len(), 1);
        assert_eq!(x003[0].source, None, "global X003 finding carries source=None");
        assert_eq!(x003[0].severity, Severity::Error);
    }

    /// X003's `suggestion` string must enumerate only buckets the rule can
    /// report — never a hypothetical `capture` bucket (deleted upstream per
    /// chunk-impl rule 3, no corpus instance) nor `boss` (V020's territory
    /// per SPEC §6.3). Pins the fix for the Round-12 finding where the
    /// user-facing advice listed `capture` as a valid rename target;
    /// source-vs-IR divergent by construction because no ModIR this rule
    /// can see contains a `capture` bucket, so the suggestion must not
    /// reach for one. (Post-8A bucket set is `{hero, legendary, monster}`
    /// per the function-doc on `check_duplicate_pokemon_buckets`.)
    #[test]
    fn x003_suggestion_only_enumerates_live_buckets() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Pikachu", 'a'));
        ir.replica_items.push(make_replica_item("Pikachu"));

        let report = check_references(&ir);
        let x003: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "X003").collect();
        assert_eq!(x003.len(), 1);
        let suggestion = x003[0].suggestion.as_ref().expect("X003 must populate suggestion");
        assert!(
            !suggestion.contains("capture"),
            "suggestion must not reference the deleted `capture` bucket: {}",
            suggestion,
        );
        assert!(
            !suggestion.contains("boss"),
            "suggestion must not imply `boss` as a rename target — boss collisions route through V020: {}",
            suggestion,
        );
        // Positive: the suggestion must mirror the buckets X003 actually found.
        assert!(
            suggestion.contains("hero") && suggestion.contains("legendary"),
            "suggestion must mirror the colliding buckets the message reports: {}",
            suggestion,
        );
    }

    #[test]
    fn x003_silent_on_intra_bucket_duplicate() {
        // Two replica items with the same name — both land in the `legendary`
        // bucket. Post-8A, `ReplicaItem` models trigger-based summons
        // (`SummonTrigger::SideUse` / `Cast`); the `legendary` bucket label
        // is preserved for X003-message stability per the function-doc on
        // `check_duplicate_pokemon_buckets`. X003 is a cross-bucket check
        // per SPEC §6.3; intra-bucket duplicates are V-rule territory
        // (V019 etc.), not a Pokemon-bucket collision.
        let mut ir = ModIR::empty();
        ir.replica_items.push(make_replica_item("Pikachu"));
        ir.replica_items.push(make_replica_item("Pikachu"));

        let report = check_references(&ir);
        let x003: Vec<_> = report.errors.iter().filter(|f| f.rule_id == "X003").collect();
        assert!(
            x003.is_empty(),
            "X003 must not fire for intra-bucket duplicates (two legendaries of the same name)",
        );
    }

    // -- V016: Hero pool references --

    #[test]
    fn test_v016_unresolved_hero_pool_ref() {
        let mut ir = ModIR::empty();
        ir.heroes.push(make_hero("Charmander", 'a'));
        ir.structural.push(make_hero_pool(vec!["charmander", "nonexistent"]));

        let report = check_references(&ir);

        // Base-sourced V016 promotes Error → Warning per §F5.
        let v016: Vec<_> = all_findings(&report)
            .into_iter()
            .filter(|f| f.rule_id == "V016")
            .collect();
        assert_eq!(v016.len(), 1, "Expected one V016 finding");
        assert_eq!(v016[0].severity, Severity::Warning, "Base source demotes V016 to Warning");
        assert_eq!(v016[0].source, Some(Source::Base));
        assert!(v016[0].message.contains("nonexistent"));
        assert!(v016[0].field_path.is_some());
        assert!(v016[0].suggestion.is_some());
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

        // Author-added hero (Source::Custom) — §F5 keeps V020's Error severity.
        let mut new_hero = make_hero("Pikachu", 'a');
        new_hero.source = Source::Custom;
        let report = check_hero_in_context(&new_hero, &ir);

        assert_eq!(report.errors.len(), 1, "Should detect cross-category name conflict");
        assert_eq!(report.errors[0].rule_id, "V020");
        assert_eq!(report.errors[0].source, Some(Source::Custom));
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

        // Author-added boss (Source::Custom) — §F5 keeps V020's Error severity.
        let mut boss = make_boss("Pikachu");
        boss.source = Source::Custom;
        let report = check_boss_in_context(&boss, &ir);

        assert_eq!(report.errors.len(), 1, "Should detect cross-category name conflict");
        assert_eq!(report.errors[0].rule_id, "V020");
        assert_eq!(report.errors[0].source, Some(Source::Custom));
    }

    #[test]
    fn test_check_boss_in_context_no_conflicts() {
        let ir = ModIR::empty();

        let boss = make_boss("Floor8");
        let report = check_boss_in_context(&boss, &ir);

        assert!(report.errors.is_empty());
        assert!(report.warnings.is_empty());
    }

    // -- X016: face-template compatibility --

    /// Verifies the rule framework fires when the restriction table is
    /// populated. The production table (`X016_TEMPLATE_RESTRICTIONS`) is
    /// empty today because `reference/textmod_guide.md` does not make
    /// per-FaceID template-restriction claims; the test injects a synthetic
    /// `[(170, &["Wolf"])]` table to exercise the matching logic without
    /// hardcoding a production claim.
    ///
    /// Guide context (verbatim, `reference/textmod_guide.md` line 626):
    /// > "Enemy effects may be repeated for different enemy sizes. Also note
    /// > that many enemy effects may behave strangely; the game doesn't
    /// > expect heroes to use them."
    ///
    /// The guide documents FaceID 170 as "Wolf Bite | sd.170 (wolf damage)"
    /// (line 639) — the synthetic restriction mirrors that guidance for test
    /// purposes only.
    #[test]
    fn x016_flags_template_restricted_face() {
        use crate::authoring::{FaceId, FaceIdValue, Pips};

        let mut ir = ModIR::empty();
        let mut hero = make_hero("Pikachu", 'a');
        hero.blocks.push(HeroBlock {
            template: "Goblin".to_string(),
            tier: Some(0),
            hp: Some(4),
            sd: DiceFaces {
                faces: vec![DiceFace::Active {
                    // FaceId::try_from is the corpus-whitelisted constructor;
                    // 170 is curated in authoring/face_id.rs.
                    face_id: FaceIdValue::Known(FaceId::try_from(170u16).unwrap()),
                    pips: Pips::new(3),
                }],
            },
            bare: false,
            color: None,
            sprite: crate::authoring::SpriteId::owned("pikachu", ""),
            speech: String::new(),
            name: "Pikachu".to_string(),
            doc: None,
            abilitydata: None,
            triggerhpdata: None,
            hue: None,
            modifier_chain: None,
            facades: vec![],
            items_inside: None,
            items_outside: None,
        });
        ir.heroes.push(hero);

        let synthetic: &[(u16, &[&str])] = &[(170, &["Wolf"])];
        let mut report = ValidationReport::default();
        check_face_template_compat_with_table(&ir, synthetic, &mut report);

        // Base-sourced hero demotes X016 to Warning per §F5.
        let x016: Vec<_> = all_findings(&report)
            .into_iter()
            .filter(|f| f.rule_id == "X016")
            .collect();
        assert_eq!(x016.len(), 1, "expected one X016 finding");
        assert_eq!(x016[0].severity, Severity::Warning);
        assert_eq!(x016[0].source, Some(Source::Base));
        assert!(x016[0].message.contains("170"));
        assert!(x016[0].message.contains("Goblin"));
    }

    #[test]
    fn x016_silent_when_production_table_empty() {
        assert!(
            X016_TEMPLATE_RESTRICTIONS.is_empty(),
            "production table must stay empty until the guide documents a restriction"
        );
    }

    // -- X017: unknown FaceID warning --

    #[test]
    fn x017_flags_unknown_face_id_as_warning() {
        use crate::authoring::{FaceIdValue, Pips};

        let mut ir = ModIR::empty();
        ir.replica_items.push(ReplicaItem {
            container_name: "Test Ball".to_string(),
            target_name: "UnknownFaceItem".to_string(),
            trigger: SummonTrigger::SideUse {
                dice: DiceFaces {
                    faces: vec![DiceFace::Active {
                        face_id: FaceIdValue::try_new(9999),
                        pips: Pips::new(1),
                    }],
                },
                dice_location: DiceLocation::OuterPreface,
            },
            enemy_template: "Wolf".to_string(),
            team_template: "housecat".to_string(),
            tier: None,
            hp: Some(4),
            color: None,
            sprite: crate::authoring::SpriteId::owned("unknown", ""),
            sticker_stack: None,
            speech: None,
            doc: None,
            toggle_flags: None,
            item_modifiers: None,
            source: Source::Base,
        });

        let report = check_references(&ir);

        // Base source: §F5 policy keeps X017's native Warning severity.
        assert_eq!(report.errors.len(), 0, "X017 on Base source must stay Warning");
        let x017: Vec<_> = report.warnings.iter().filter(|f| f.rule_id == "X017").collect();
        assert_eq!(x017.len(), 1);
        assert_eq!(x017[0].severity, Severity::Warning);
        assert_eq!(x017[0].source, Some(Source::Base));
        assert!(x017[0].message.contains("9999"));
    }

    /// Regression: X017 must fire on every `DiceFaces`-bearing IR surface,
    /// not only `ReplicaItem`. This test wires an Unknown FaceID into each
    /// of the five other surfaces `iter_dice_faces` walks and asserts one
    /// warning per surface, plus correct `field_path` strings.
    #[test]
    fn x017_fires_on_every_dice_faces_surface() {
        use crate::authoring::{FaceIdValue, Pips};
        use crate::ir::{AbilityData, FightDefinition, FightUnit, TriggerHpDef};

        fn unknown_faces(id: u16) -> DiceFaces {
            DiceFaces {
                faces: vec![DiceFace::Active {
                    face_id: FaceIdValue::try_new(id),
                    pips: Pips::new(1),
                }],
            }
        }

        let mut ir = ModIR::empty();

        // Surface 1: HeroBlock.sd  (id 9001)
        // Surface 2: HeroBlock.abilitydata.sd  (id 9002)
        // Surface 3: HeroBlock.triggerhpdata.sd  (id 9003)
        let mut hero = make_hero("Pikachu", 'a');
        hero.blocks.push(HeroBlock {
            template: "Slime".to_string(),
            tier: Some(0),
            hp: Some(4),
            sd: unknown_faces(9001),
            bare: false,
            color: None,
            sprite: crate::authoring::SpriteId::owned("pikachu", ""),
            speech: String::new(),
            name: "Pikachu".to_string(),
            doc: None,
            abilitydata: Some(AbilityData {
                template: "Statue".to_string(),
                sd: unknown_faces(9002),
                sprite: None,
                name: "Infuse".to_string(),
                modifier_chain: None,
                hsv: None,
                ability_type: None,
            }),
            triggerhpdata: Some(TriggerHpDef {
                template: "Egg".to_string(),
                hp: Some(1),
                sd: Some(unknown_faces(9003)),
                color: None,
                modifier_chain: None,
                sprite: None,
                name: None,
                tier: None,
                part: None,
            }),
            hue: None,
            modifier_chain: None,
            facades: vec![],
            items_inside: None,
            items_outside: None,
        });
        ir.heroes.push(hero);

        // Surface 4: Monster.sd  (id 9004)
        ir.monsters.push(crate::ir::Monster {
            name: "Gastly".to_string(),
            base_template: "Goblin".to_string(),
            floor_range: "1-3".to_string(),
            hp: Some(3),
            sd: Some(unknown_faces(9004)),
            sprite: None,
            color: None,
            doc: None,
            modifier_chain: None,
            balance: None,
            source: Source::Base,
        });

        // Surfaces 5 & 6: FightUnit.sd (id 9005) + FightUnit.nested_units[].sd (id 9006)
        let nested = FightUnit {
            template: "Egg".to_string(),
            name: "EggSpawn".to_string(),
            sd: Some(unknown_faces(9006)),
            ..Default::default()
        };
        let enemy = FightUnit {
            template: "Wolf".to_string(),
            name: "Alpha".to_string(),
            sd: Some(unknown_faces(9005)),
            nested_units: Some(vec![nested]),
            ..Default::default()
        };
        let fight = FightDefinition {
            level: Some(8),
            enemies: vec![enemy],
            name: None,
            trigger: None,
        };
        let mut boss = make_boss("Floor8");
        boss.fights.push(fight);
        ir.bosses.push(boss);

        let report = check_references(&ir);
        let x017: Vec<_> =
            report.warnings.iter().filter(|f| f.rule_id == "X017").collect();

        // 6 surfaces × 1 Unknown each = 6 warnings.
        assert_eq!(x017.len(), 6, "every DiceFaces surface must emit X017");

        // Spot-check each surface's field_path prefix — this pins the walker's
        // path shape so a regression that drops a surface fails loudly.
        let paths: Vec<_> = x017.iter().map(|f| f.field_path.clone().unwrap()).collect();
        assert!(paths.iter().any(|p| p.starts_with("heroes[Pikachu].blocks[0].sd")));
        assert!(paths.iter().any(|p| p.starts_with("heroes[Pikachu].blocks[0].abilitydata.sd")));
        assert!(paths.iter().any(|p| p.starts_with("heroes[Pikachu].blocks[0].triggerhpdata.sd")));
        assert!(paths.iter().any(|p| p.starts_with("monsters[Gastly].sd")));
        assert!(paths.iter().any(|p| p.starts_with("bosses[Floor8].fights[0].enemies[0].sd")));
        assert!(paths
            .iter()
            .any(|p| p.starts_with("bosses[Floor8].fights[0].enemies[0].nested_units[0].sd")));
    }

    #[test]
    fn x017_silent_when_all_face_ids_known() {
        use crate::authoring::{FaceId, FaceIdValue, Pips};

        let mut ir = ModIR::empty();
        ir.replica_items.push(ReplicaItem {
            container_name: "Test Ball".to_string(),
            target_name: "KnownFaceItem".to_string(),
            trigger: SummonTrigger::SideUse {
                dice: DiceFaces {
                    faces: vec![DiceFace::Active {
                        face_id: FaceIdValue::Known(FaceId::DAMAGE_BASIC),
                        pips: Pips::new(2),
                    }],
                },
                dice_location: DiceLocation::OuterPreface,
            },
            enemy_template: "Wolf".to_string(),
            team_template: "housecat".to_string(),
            tier: None,
            hp: Some(4),
            color: None,
            sprite: crate::authoring::SpriteId::owned("known", ""),
            sticker_stack: None,
            speech: None,
            doc: None,
            toggle_flags: None,
            item_modifiers: None,
            source: Source::Custom,
        });

        let report = check_references(&ir);
        let x017: Vec<_> = report.warnings.iter().filter(|f| f.rule_id == "X017").collect();
        assert!(x017.is_empty(), "no X017 on corpus-known FaceIDs");
    }
}
