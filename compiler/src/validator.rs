/// Textmod validator — validates a raw textmod string against structural rules
/// derived from 3 known-working mods (pansaer, punpuns, sliceymon).
///
/// Pure library module: no std::fs or I/O. WASM-compatible.
use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::CompilerError;
use crate::extractor::classifier::{classify, ModifierType};
use crate::extractor::splitter::split_modifiers;
use crate::util::{
    extract_color, extract_mn_name, find_matching_close_paren,
    split_at_depth0, verify_ascii_only,
};

// ---------------------------------------------------------------------------
// Rule ID constants
// ---------------------------------------------------------------------------

const E001: &str = "E001";
const E002: &str = "E002";
const E003: &str = "E003";
const E004: &str = "E004";
const E005: &str = "E005";
const E006: &str = "E006";
const E007: &str = "E007";
const E008: &str = "E008";
const E009: &str = "E009";
const E010: &str = "E010";
const E011: &str = "E011";
const E012: &str = "E012";

const E013: &str = "E013";
const E014: &str = "E014";
const E015: &str = "E015";
const E016: &str = "E016";

const W001: &str = "W001";
const W002: &str = "W002";
const W003: &str = "W003";
const W004: &str = "W004";
const W005: &str = "W005";
const W006: &str = "W006";
const W007: &str = "W007";
const W008: &str = "W008";
const W009: &str = "W009";
const W010: &str = "W010";
const W011: &str = "W011";

const E017: &str = "E017";
const E018: &str = "E018";
const E019: &str = "E019";
const E020: &str = "E020";
const E021: &str = "E021";

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
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Context helper
// ---------------------------------------------------------------------------

/// Extract ~20 chars before and after `pos` from `s` for error context.
fn snippet(s: &str, pos: usize) -> String {
    let start = pos.saturating_sub(20);
    let end = (pos + 20).min(s.len());
    s[start..end].to_string()
}

// ---------------------------------------------------------------------------
// Custom validators (cannot reuse util.rs versions — need Finding output)
// ---------------------------------------------------------------------------

/// E001: Check parenthesis balance, returning a Finding with position and context.
fn check_paren_balance(modifier: &str, idx: usize, mn: &Option<String>) -> Option<Finding> {
    let mut depth: i32 = 0;
    for (i, ch) in modifier.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Some(Finding {
                        rule_id: E001.to_string(),
                        severity: Severity::Error,
                        modifier_index: Some(idx),
                        modifier_name: mn.clone(),
                        position: Some(i),
                        context: Some(snippet(modifier, i)),
                        message: format!("Unmatched closing paren at position {} (depth {})", i, depth),
                        ..Default::default()
                    });
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        let pos = modifier.len().saturating_sub(1);
        return Some(Finding {
            rule_id: E001.to_string(),
            severity: Severity::Error,
            modifier_index: Some(idx),
            modifier_name: mn.clone(),
            position: Some(pos),
            context: Some(snippet(modifier, pos)),
            message: format!("Unbalanced parens: depth {} at end", depth),
            ..Default::default()
        });
    }
    None
}

/// Parse a single face entry like "0", "34-1", "76--2", "76-1#triggerhpdata".
/// Returns (face_id, Option<pips>) on success, or error message.
fn parse_face_entry(entry: &str) -> Result<(u16, Option<i16>), String> {
    if entry.is_empty() {
        return Err("empty face entry".to_string());
    }

    // Strip #suffix if present (free-form game metadata)
    let numeric_part = entry.split('#').next().unwrap_or(entry);

    // Handle N--P (negative pips): split on first '-'
    if let Some(dash_pos) = numeric_part.find('-') {
        let face_str = &numeric_part[..dash_pos];
        let pips_str = &numeric_part[dash_pos + 1..];

        let face_id: u16 = face_str
            .parse()
            .map_err(|_| format!("invalid face ID '{}'", face_str))?;

        // pips_str might be "-N" (for N--P pattern) or "N"
        let pips: i16 = pips_str
            .parse()
            .map_err(|_| format!("invalid pips '{}'", pips_str))?;

        Ok((face_id, Some(pips)))
    } else {
        // Bare N (blank face or just face ID)
        let face_id: u16 = numeric_part
            .parse()
            .map_err(|_| format!("invalid face entry '{}'", numeric_part))?;
        Ok((face_id, None))
    }
}

/// Extract `.tier.X` value from a tier block content string.
fn extract_tier(content: &str) -> Option<u8> {
    let marker = ".tier.";
    let pos = content.find(marker)?;
    let start = pos + marker.len();
    let bytes = content.as_bytes();
    if start < bytes.len() && bytes[start].is_ascii_digit() {
        Some(bytes[start] - b'0')
    } else {
        None
    }
}

/// Extract face data from `.sd.` in a tier block, handling #suffix.
/// Returns the list of raw face entry strings (split on ':').
fn extract_sd_faces(content: &str) -> Option<Vec<String>> {
    let marker = ".sd.";
    let pos = content.find(marker)?;
    let start = pos + marker.len();
    let bytes = content.as_bytes();

    // Read chars that are part of face data: digits, ':', '-', '#', and alphanumeric suffix chars
    // We stop at '.' or ')' which indicate next property or end of block
    let mut end = start;
    while end < bytes.len() {
        let b = bytes[end];
        // Stop at property delimiter or block boundary
        if b == b'.' || b == b')' {
            break;
        }
        end += 1;
    }

    if end <= start {
        return None;
    }

    let sd_raw = &content[start..end];
    let faces: Vec<String> = sd_raw.split(':').map(|s| s.to_string()).collect();
    if faces.is_empty() {
        None
    } else {
        Some(faces)
    }
}

// ---------------------------------------------------------------------------
// Validation phases
// ---------------------------------------------------------------------------

/// Phase 1: Global ASCII check (E002).
fn phase_global(text: &str, report: &mut ValidationReport) {
    if let Err(msg) = verify_ascii_only(text) {
        // Find the exact position for the finding
        for (i, ch) in text.char_indices() {
            if !ch.is_ascii() {
                report.errors.push(Finding {
                    rule_id: E002.to_string(),
                    severity: Severity::Error,
                    modifier_index: None,
                    modifier_name: None,
                    position: Some(i),
                    context: Some(snippet(text, i)),
                    message: msg.clone(),
                    ..Default::default()
                });
                break;
            }
        }
    }
}

/// Phase 3: Per-modifier checks (E001, mn extraction, W005, W004).
/// Returns the list of (modifier_index, modifier_string, Option<mn_name>) for later phases.
fn phase_per_modifier(
    modifiers: &[String],
    report: &mut ValidationReport,
) -> Vec<(usize, String, Option<String>)> {
    let mut result = Vec::new();

    for (idx, modifier) in modifiers.iter().enumerate() {
        let mn = extract_mn_name(modifier);

        // E001: paren balance
        if let Some(finding) = check_paren_balance(modifier, idx, &mn) {
            report.errors.push(finding);
        }

        // W005: missing .mn.
        if mn.is_none() {
            report.warnings.push(Finding {
                rule_id: W005.to_string(),
                severity: Severity::Warning,
                modifier_index: Some(idx),
                modifier_name: None,
                position: None,
                context: None,
                message: "Modifier lacks depth-0 .mn.".to_string(),
                ..Default::default()
            });
        }

        // W004: .img. present but empty
        if let Some(img_pos) = modifier.find(".img.") {
            let after = img_pos + 5;
            let next_delim = modifier[after..]
                .find(['.', ')', '+', '&'])
                .map(|p| p + after)
                .unwrap_or(modifier.len());
            if next_delim == after {
                report.warnings.push(Finding {
                    rule_id: W004.to_string(),
                    severity: Severity::Warning,
                    modifier_index: Some(idx),
                    modifier_name: mn.clone(),
                    position: None,
                    context: None,
                    message: ".img. data is empty".to_string(),
                    ..Default::default()
                });
            }
        }

        result.push((idx, modifier.clone(), mn));
    }

    result
}

/// Phase 4: Hero-specific checks (E004-E012, W001, W002).
/// Only called for ph.b heroes (full hero definitions, not vanilla replacements).
fn phase_hero(
    idx: usize,
    modifier: &str,
    mn: &Option<String>,
    report: &mut ValidationReport,
) {
    let lower = modifier.to_lowercase();

    // E004: Hero prefix must match pattern (case-insensitive)
    // Expected: hidden&temporary&ph.b[a-z]+;\d+;!mheropool.\(
    let prefix_ok = lower.starts_with("hidden&temporary&ph.b")
        && lower.contains(";")
        && lower.contains("!mheropool.");
    if !prefix_ok {
        report.errors.push(Finding {
            rule_id: E004.to_string(),
            severity: Severity::Error,
            modifier_index: Some(idx),
            modifier_name: mn.clone(),
            position: None,
            context: None,
            message: "Hero prefix doesn't match: expected hidden&temporary&ph.b[name];N;!mheropool."
                .to_string(),
            ..Default::default()
        });
    }

    // E005: Hero suffix must end with .part.1&hidden.mn.NAME@2!m(skip&hidden&temporary)
    // Case-insensitive for .part.1&hidden
    let has_part1_hidden = lower.contains(".part.1&hidden");
    let has_mn_suffix = lower.contains(".mn.") && lower.contains("@2!m(skip&hidden&temporary)");
    if !has_part1_hidden || !has_mn_suffix {
        report.errors.push(Finding {
            rule_id: E005.to_string(),
            severity: Severity::Error,
            modifier_index: Some(idx),
            modifier_name: mn.clone(),
            position: None,
            context: None,
            message: "Hero suffix doesn't match: expected .part.1&hidden.mn.NAME@2!m(skip&hidden&temporary)"
                .to_string(),
            ..Default::default()
        });
    }

    // Extract the hero body (between !mheropool. and .part.1&hidden) — case-insensitive search
    let body_start = lower.find("!mheropool.").map(|p| p + "!mheropool.".len());
    let body_end = lower.find(".part.1&hidden");
    let body = match (body_start, body_end) {
        (Some(s), Some(e)) if s < e => &modifier[s..e],
        _ => return, // Can't parse body — prefix/suffix errors already reported
    };

    // E006: >= 3 tier parts (split on '+' at depth 0)
    let tier_parts = split_at_depth0(body, '+');
    if tier_parts.len() < 3 {
        report.errors.push(Finding {
            rule_id: E006.to_string(),
            severity: Severity::Error,
            modifier_index: Some(idx),
            modifier_name: mn.clone(),
            position: None,
            context: None,
            message: format!(
                "Hero has {} tier parts, expected >= 3 (T1 + at least 2 evolution paths)",
                tier_parts.len()
            ),
            ..Default::default()
        });
    }

    // Per-tier checks
    for (tier_idx, tier_part) in tier_parts.iter().enumerate() {
        // E007: Each tier block opens with ( and has matching )
        // Relaxed from strict (replica. to accept (Replica., (ClassName., etc.
        // Bare tier parts (no opening paren) are valid — some heroes use them for overlays.
        let trimmed = tier_part.trim();
        if trimmed.starts_with('(') {
            // Has opening paren — check for matching close
            if find_matching_close_paren(trimmed, 0).is_none() {
                report.errors.push(Finding {
                    rule_id: E007.to_string(),
                    severity: Severity::Error,
                    modifier_index: Some(idx),
                    modifier_name: mn.clone(),
                    position: None,
                    context: None,
                    message: format!("Tier {} has unmatched opening paren", tier_idx),
                    ..Default::default()
                });
            }
        }
        // Skip further checks for bare tier parts (no parens) — they are valid overlays

        // E008/E009/W001/W002: Face validation
        if let Some(faces) = extract_sd_faces(trimmed) {
            // W001: face count
            if faces.len() < 4 || faces.len() > 6 {
                report.warnings.push(Finding {
                    rule_id: W001.to_string(),
                    severity: Severity::Warning,
                    modifier_index: Some(idx),
                    modifier_name: mn.clone(),
                    position: None,
                    context: None,
                    message: format!(
                        "Tier {} .sd. has {} faces (expected 4-6)",
                        tier_idx,
                        faces.len()
                    ),
                    ..Default::default()
                });
            }

            for face_str in &faces {
                match parse_face_entry(face_str) {
                    Ok((face_id, pips)) => {
                        // E009: face ID range 0-187
                        if face_id > 187 {
                            report.errors.push(Finding {
                                rule_id: E009.to_string(),
                                severity: Severity::Error,
                                modifier_index: Some(idx),
                                modifier_name: mn.clone(),
                                position: None,
                                context: Some(face_str.clone()),
                                message: format!(
                                    "Face ID {} out of range 0-187 in tier {}",
                                    face_id, tier_idx
                                ),
                                ..Default::default()
                            });
                        }
                        // W002: pip value > 25
                        if let Some(p) = pips {
                            if p.unsigned_abs() > 25 {
                                report.warnings.push(Finding {
                                    rule_id: W002.to_string(),
                                    severity: Severity::Warning,
                                    modifier_index: Some(idx),
                                    modifier_name: mn.clone(),
                                    position: None,
                                    context: Some(face_str.clone()),
                                    message: format!(
                                        "Pip value {} exceeds 25 in tier {}",
                                        p, tier_idx
                                    ),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                    Err(msg) => {
                        // E008: bad face format
                        report.errors.push(Finding {
                            rule_id: E008.to_string(),
                            severity: Severity::Error,
                            modifier_index: Some(idx),
                            modifier_name: mn.clone(),
                            position: None,
                            context: Some(face_str.clone()),
                            message: format!("Bad face format in tier {}: {}", tier_idx, msg),
                            ..Default::default()
                        });
                    }
                }
            }
        }

        // E010: .hp. value is digits, range 1-999
        // extract_hp returns None if .hp. absent — that's OK per Non-Rules
        if let Some(hp_pos) = trimmed.find(".hp.") {
            let hp_start = hp_pos + 4;
            let bytes = trimmed.as_bytes();
            let mut hp_end = hp_start;
            while hp_end < bytes.len() && bytes[hp_end].is_ascii_digit() {
                hp_end += 1;
            }
            if hp_end == hp_start {
                // .hp. present but no digits follow
                report.errors.push(Finding {
                    rule_id: E010.to_string(),
                    severity: Severity::Error,
                    modifier_index: Some(idx),
                    modifier_name: mn.clone(),
                    position: None,
                    context: None,
                    message: format!("Tier {} .hp. value is not numeric", tier_idx),
                    ..Default::default()
                });
            } else {
                let hp_str = &trimmed[hp_start..hp_end];
                if let Ok(hp_val) = hp_str.parse::<u16>() {
                    if hp_val == 0 || hp_val > 999 {
                        report.errors.push(Finding {
                            rule_id: E010.to_string(),
                            severity: Severity::Error,
                            modifier_index: Some(idx),
                            modifier_name: mn.clone(),
                            position: None,
                            context: None,
                            message: format!(
                                "Tier {} .hp. value {} out of range 1-999",
                                tier_idx, hp_val
                            ),
                            ..Default::default()
                        });
                    }
                } else {
                    report.errors.push(Finding {
                        rule_id: E010.to_string(),
                        severity: Severity::Error,
                        modifier_index: Some(idx),
                        modifier_name: mn.clone(),
                        position: None,
                        context: None,
                        message: format!("Tier {} .hp. value '{}' is not a valid number", tier_idx, hp_str),
                        ..Default::default()
                    });
                }
            }
        }

        // E011: .col. value is single lowercase letter
        if trimmed.contains(".col.") && extract_color(trimmed).is_none() {
            report.errors.push(Finding {
                rule_id: E011.to_string(),
                severity: Severity::Error,
                modifier_index: Some(idx),
                modifier_name: mn.clone(),
                position: None,
                context: None,
                message: format!("Tier {} .col. value is not a single lowercase letter", tier_idx),
                ..Default::default()
            });
        }

        // E012: .tier. value is single digit 0-9
        if trimmed.contains(".tier.") && extract_tier(trimmed).is_none() {
            report.errors.push(Finding {
                rule_id: E012.to_string(),
                severity: Severity::Error,
                modifier_index: Some(idx),
                modifier_name: mn.clone(),
                position: None,
                context: None,
                message: format!("Tier {} .tier. value is not a single digit 0-9", tier_idx),
                ..Default::default()
            });
        }
    }
}

/// Phase 5: Cross-modifier checks (E003, W003).
fn phase_cross_modifier(
    annotated: &[(usize, String, Option<String>)],
    report: &mut ValidationReport,
) {
    // E003: duplicate .mn. names
    let mut seen_names: HashSet<String> = HashSet::new();
    for (idx, _modifier, mn) in annotated {
        if let Some(ref name) = mn {
            if !seen_names.insert(name.clone()) {
                report.errors.push(Finding {
                    rule_id: E003.to_string(),
                    severity: Severity::Error,
                    modifier_index: Some(*idx),
                    modifier_name: mn.clone(),
                    position: None,
                    context: None,
                    message: format!("Duplicate .mn. name: {}", name),
                    ..Default::default()
                });
            }
        }
    }

    // W003: unknown modifier type
    for (idx, modifier, mn) in annotated {
        let mod_type = classify(modifier, *idx).unwrap_or(ModifierType::Unknown);
        if mod_type == ModifierType::Unknown {
            report.warnings.push(Finding {
                rule_id: W003.to_string(),
                severity: Severity::Warning,
                modifier_index: Some(*idx),
                modifier_name: mn.clone(),
                position: None,
                context: None,
                message: "Modifier classifies as Unknown type".to_string(),
                ..Default::default()
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Shared content validation (generalized from phase_hero)
// ---------------------------------------------------------------------------

/// Validate .sd. and .hp. content in any modifier type.
/// Applies E008 (face format), E009 (face ID range), E010 (HP range),
/// W001 (face count), W002 (pip values).
fn phase_content_blocks(
    content: &str,
    idx: usize,
    mn: &Option<String>,
    report: &mut ValidationReport,
) {
    // Face validation
    if let Some(faces) = extract_sd_faces(content) {
        if faces.len() < 4 || faces.len() > 6 {
            report.warnings.push(Finding {
                rule_id: W001.to_string(),
                severity: Severity::Warning,
                modifier_index: Some(idx),
                modifier_name: mn.clone(),
                position: None,
                context: None,
                message: format!(".sd. has {} faces (expected 4-6)", faces.len()),
                ..Default::default()
            });
        }

        for face_str in &faces {
            match parse_face_entry(face_str) {
                Ok((face_id, pips)) => {
                    if face_id > 187 {
                        report.errors.push(Finding {
                            rule_id: E009.to_string(),
                            severity: Severity::Error,
                            modifier_index: Some(idx),
                            modifier_name: mn.clone(),
                            position: None,
                            context: Some(face_str.clone()),
                            message: format!("Face ID {} out of range 0-187", face_id),
                            ..Default::default()
                        });
                    }
                    if let Some(p) = pips {
                        if p.unsigned_abs() > 25 {
                            report.warnings.push(Finding {
                                rule_id: W002.to_string(),
                                severity: Severity::Warning,
                                modifier_index: Some(idx),
                                modifier_name: mn.clone(),
                                position: None,
                                context: Some(face_str.clone()),
                                message: format!("Pip value {} exceeds 25", p),
                                ..Default::default()
                            });
                        }
                    }
                }
                Err(msg) => {
                    report.errors.push(Finding {
                        rule_id: E008.to_string(),
                        severity: Severity::Error,
                        modifier_index: Some(idx),
                        modifier_name: mn.clone(),
                        position: None,
                        context: Some(face_str.clone()),
                        message: format!("Bad face format: {}", msg),
                        ..Default::default()
                    });
                }
            }
        }
    }

    // HP validation
    if let Some(hp_pos) = content.find(".hp.") {
        let hp_start = hp_pos + 4;
        let bytes = content.as_bytes();
        let mut hp_end = hp_start;
        while hp_end < bytes.len() && bytes[hp_end].is_ascii_digit() {
            hp_end += 1;
        }
        if hp_end > hp_start {
            if let Ok(hp_val) = content[hp_start..hp_end].parse::<u16>() {
                if hp_val == 0 || hp_val > 999 {
                    report.errors.push(Finding {
                        rule_id: E010.to_string(),
                        severity: Severity::Error,
                        modifier_index: Some(idx),
                        modifier_name: mn.clone(),
                        position: None,
                        context: None,
                        message: format!(".hp. value {} out of range 1-999", hp_val),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

/// E013: Monster floor range format validation (N-M where N <= M).
fn check_monster_floor_range(modifier: &str, idx: usize, mn: &Option<String>, report: &mut ValidationReport) {
    let content = modifier.trim_start_matches('(');
    let bytes = content.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
    if i == 0 || i >= bytes.len() || bytes[i] != b'-' { return; }
    let n_str = &content[..i];
    i += 1;
    let start2 = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
    if i == start2 { return; }
    let m_str = &content[start2..i];
    if let (Ok(n), Ok(m)) = (n_str.parse::<u16>(), m_str.parse::<u16>()) {
        if n > m {
            report.errors.push(Finding {
                rule_id: E013.to_string(),
                severity: Severity::Error,
                modifier_index: Some(idx),
                modifier_name: mn.clone(),
                position: None,
                context: Some(format!("{}-{}", n, m)),
                message: format!("Monster floor range {}-{} invalid: start > end", n, m),
                ..Default::default()
            });
        }
    }
}

/// E014: Boss level range (1-20).
fn check_boss_level(modifier: &str, idx: usize, mn: &Option<String>, report: &mut ValidationReport) {
    let marker = "ch.om";
    if let Some(pos) = modifier.find(marker) {
        let start = pos + marker.len();
        let remaining = &modifier[start..];
        let mut end = 0;
        for ch in remaining.chars() {
            if ch.is_ascii_digit() { end += 1; } else { break; }
        }
        if end > 0 {
            if let Ok(level) = remaining[..end].parse::<u8>() {
                if level == 0 || level > 20 {
                    report.errors.push(Finding {
                        rule_id: E014.to_string(),
                        severity: Severity::Error,
                        modifier_index: Some(idx),
                        modifier_name: mn.clone(),
                        position: None,
                        context: Some(format!("ch.om{}", level)),
                        message: format!("Boss level {} out of range 1-20", level),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

/// E015: Abilitydata internal face validation.
fn check_abilitydata_faces(modifier: &str, idx: usize, mn: &Option<String>, report: &mut ValidationReport) {
    let marker = ".abilitydata.";
    let mut search_from = 0;
    while let Some(pos) = modifier[search_from..].find(marker) {
        let abs = search_from + pos;
        let after = abs + marker.len();
        if after < modifier.len() && modifier.as_bytes()[after] == b'(' {
            if let Some(close) = find_matching_close_paren(modifier, after) {
                let inner = &modifier[after + 1..close];
                // Only validate if .sd. is present inside abilitydata
                if let Some(faces) = extract_sd_faces(inner) {
                    for face_str in &faces {
                        // Skip entries with alphabetic chars — not face data
                        if face_str.chars().any(|c| c.is_ascii_alphabetic()) {
                            continue;
                        }
                        match parse_face_entry(face_str) {
                            Ok((face_id, _)) => {
                                if face_id > 187 {
                                    report.errors.push(Finding {
                                        rule_id: E015.to_string(),
                                        severity: Severity::Error,
                                        modifier_index: Some(idx),
                                        modifier_name: mn.clone(),
                                        position: None,
                                        context: Some(face_str.clone()),
                                        message: format!(
                                            "Face ID {} out of range 0-187 inside abilitydata",
                                            face_id
                                        ),
                                        ..Default::default()
                                    });
                                }
                            }
                            Err(msg) => {
                                report.errors.push(Finding {
                                    rule_id: E015.to_string(),
                                    severity: Severity::Error,
                                    modifier_index: Some(idx),
                                    modifier_name: mn.clone(),
                                    position: None,
                                    context: Some(face_str.clone()),
                                    message: format!(
                                        "Bad face format inside abilitydata: {}",
                                        msg
                                    ),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
                search_from = close + 1;
                continue;
            }
        }
        search_from = abs + marker.len();
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Validate a raw textmod string, returning a report of all findings.
///
/// Returns `Err(CompilerError)` only if splitting fails (globally unmatched parens).
/// All other issues are reported as `Finding`s inside the `ValidationReport`.
pub fn validate(textmod: &str) -> Result<ValidationReport, CompilerError> {
    let mut report = ValidationReport::default();

    // Phase 1: Global ASCII check
    phase_global(textmod, &mut report);

    // Phase 2: Split into modifiers
    let modifiers = split_modifiers(textmod)?;

    // Phase 3: Per-modifier checks
    let annotated = phase_per_modifier(&modifiers, &mut report);

    // Phase 4: Type-specific checks
    for (idx, modifier, mn) in &annotated {
        let mod_type = classify(modifier, *idx).unwrap_or(ModifierType::Unknown);
        let lower = modifier.to_lowercase();

        match mod_type {
            ModifierType::Hero if lower.contains("ph.b") => {
                // Sliceymon heroes: full structural + content checks
                phase_hero(*idx, modifier, mn, &mut report);
                check_abilitydata_faces(modifier, *idx, mn, &mut report);
            }
            ModifierType::Hero => {
                // Grouped heroes: content checks only (no wrapper rules)
                phase_content_blocks(modifier, *idx, mn, &mut report);
                check_abilitydata_faces(modifier, *idx, mn, &mut report);
            }
            ModifierType::ReplicaItem => {
                phase_content_blocks(modifier, *idx, mn, &mut report);
                // W006: replica item missing sprite
                if !modifier.contains(".img.") && !modifier.contains(".n.") {
                    report.warnings.push(Finding {
                        rule_id: W006.to_string(),
                        severity: Severity::Warning,
                        modifier_index: Some(*idx),
                        modifier_name: mn.clone(),
                        position: None,
                        context: None,
                        message: "Replica item missing sprite data (.img. or .n.)".to_string(),
                        ..Default::default()
                    });
                }
            }
            ModifierType::ReplicaItemWithAbility => {
                phase_content_blocks(modifier, *idx, mn, &mut report);
                check_abilitydata_faces(modifier, *idx, mn, &mut report);
                // W007: replica item with ability missing abilitydata
                if !modifier.contains(".abilitydata.") {
                    report.warnings.push(Finding {
                        rule_id: W007.to_string(),
                        severity: Severity::Warning,
                        modifier_index: Some(*idx),
                        modifier_name: mn.clone(),
                        position: None,
                        context: None,
                        message: "Replica item with ability missing .abilitydata.".to_string(),
                        ..Default::default()
                    });
                }
            }
            ModifierType::Monster => {
                phase_content_blocks(modifier, *idx, mn, &mut report);
                check_monster_floor_range(modifier, *idx, mn, &mut report);
            }
            ModifierType::Boss => {
                // Skip phase_content_blocks for bosses — .sd. data is inside
                // .fight.(...) blocks where '+' separates fight units, not faces.
                // extract_sd_faces isn't depth-aware and produces false positives.
                check_boss_level(modifier, *idx, mn, &mut report);
            }
            _ => {}
        }
    }

    // Phase 5: Cross-modifier checks
    phase_cross_modifier(&annotated, &mut report);

    // Info: summary
    report.info.push(Finding {
        rule_id: "I000".to_string(),
        severity: Severity::Info,
        modifier_index: None,
        modifier_name: None,
        position: None,
        context: None,
        message: format!("{} modifiers analyzed", modifiers.len()),
        ..Default::default()
    });

    Ok(report)
}

/// Validate cross-type references in a structured ModIR.
///
/// Checks that string references between types resolve correctly:
/// - Hero names in HeroPoolBase.hero_refs must exist in ModIR.heroes
/// - Hero names in PoolReplacement.hero_names must exist in ModIR.heroes
/// - Party member names in PartyConfig.members must exist in ModIR.heroes
///
/// Uses case-insensitive comparison (hero_refs may be Title Case while
/// internal_name is lowercase for grouped heroes).
pub fn validate_cross_references(ir: &crate::ir::ModIR) -> ValidationReport {
    use crate::ir::StructuralContent;

    let mut report = ValidationReport::default();

    // Build a set of known hero internal_names (lowercase for case-insensitive match)
    let hero_names: HashSet<String> = ir
        .heroes
        .iter()
        .map(|h| h.internal_name.to_lowercase())
        .collect();

    for (s_idx, s) in ir.structural.iter().enumerate() {
        match &s.content {
            StructuralContent::HeroPoolBase { hero_refs, .. } => {
                for href in hero_refs {
                    if !hero_names.contains(&href.to_lowercase()) {
                        report.errors.push(Finding {
                            rule_id: E016.to_string(),
                            severity: Severity::Error,
                            modifier_index: Some(s_idx),
                            modifier_name: s.name.clone(),
                            position: None,
                            context: Some(href.clone()),
                            message: format!(
                                "HeroPoolBase references hero '{}' which does not exist in heroes",
                                href
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
            StructuralContent::PoolReplacement { hero_names: pool_names, .. } => {
                for pname in pool_names {
                    if !hero_names.contains(&pname.to_lowercase()) {
                        report.errors.push(Finding {
                            rule_id: E016.to_string(),
                            severity: Severity::Error,
                            modifier_index: Some(s_idx),
                            modifier_name: s.name.clone(),
                            position: None,
                            context: Some(pname.clone()),
                            message: format!(
                                "PoolReplacement references hero '{}' which does not exist in heroes",
                                pname
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
            StructuralContent::PartyConfig { members, .. } => {
                for member in members {
                    if !hero_names.contains(&member.to_lowercase()) {
                        report.errors.push(Finding {
                            rule_id: E016.to_string(),
                            severity: Severity::Error,
                            modifier_index: Some(s_idx),
                            modifier_name: s.name.clone(),
                            position: None,
                            context: Some(member.clone()),
                            message: format!(
                                "PartyConfig references member '{}' which does not exist in heroes",
                                member
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
            _ => {}
        }
    }

    report
}

// ---------------------------------------------------------------------------
// Semantic validation (IR-based) — E017-E021, W008-W011
// ---------------------------------------------------------------------------

/// Known game templates — hero classes, item containers, monster bases.
/// Case-insensitive check (templates in IR may be lowercase).
fn is_known_template(name: &str) -> bool {
    // Normalize to lowercase for comparison
    let lower = name.to_lowercase();
    matches!(lower.as_str(),
        // Hero classes (from working mods + base game)
        "ace" | "alien" | "alloy" | "assassin" | "barbarian" | "bard" | "bash" |
        "brawler" | "brigand" | "buckle" | "captain" | "clumsy" | "collector" |
        "dabble" | "dabbler" | "dabblest" | "dancer" | "defender" | "eccentric" |
        "fencer" | "fey" | "fighter" | "gambler" | "gladiator" | "guardian" |
        "healer" | "herbalist" | "hoarder" | "housecat" | "keeper" | "knight" |
        "leader" | "lost" | "ludus" | "mage" | "mimic" | "monk" | "ninja" |
        "pilgrim" | "primrose" | "prodigy" | "ranger" | "reflection" | "rogue" |
        "roulette" | "ruffian" | "scoundrel" | "scrapper" | "sharpshot" |
        "sinew" | "soldier" | "sorcerer" | "spade" | "sphere" | "stalwart" |
        "statue" | "tank" | "thief" | "trapper" | "valkyrie" | "vampire" |
        "sparky" | "tainted" |
        "venom" | "veteran" | "wallop" | "wanderer" | "whirl" |
        // Monster templates
        "slimelet" | "slime" | "skeleton" | "zombie" | "wolf" | "ghost" |
        "spider" | "rat" | "bat" | "snake" | "goblin" | "bandit" | "witch" |
        "ogre" | "dragon" | "wyrm" | "imp" | "demon" |
        // Item/replica templates
        "hat" | "ball" | "dagger" | "ring" | "amulet" | "sword" |
        "bow" | "staff" | "wand" | "potion" | "scroll" | "armor" | "boots" |
        "gloves" | "cloak" | "helmet" | "lantern" | "charm" | "tome"
    )
}

/// Rejected face IDs per template — blocklist approach.
/// Returns None for templates without a defined table (those skip E017/E021).
/// Uses blocklist instead of allowlist to avoid false positives on real mods
/// while still catching clearly inappropriate face assignments.
fn rejected_faces_for_template(template: &str) -> Option<&'static [u16]> {
    let lower = template.to_lowercase();
    match lower.as_str() {
        "fey" => Some(&[
            // Physically-themed faces inappropriate for magical Fey spells
            42, // Damage Charged (Brawler mechanic)
            39, // Damage Heavy (Tank mechanic)
            41, // Damage Steel (armor/physical)
            43, // Stun Bully (physical bully)
        ]),
        _ => None,
    }
}

/// Expected HP range per tier: (min, max).
fn hp_range_for_tier(tier: u8) -> (u16, u16) {
    match tier {
        1 => (1, 12),
        2 => (2, 15),
        3 => (2, 30),
        _ => (1, 999),
    }
}

/// Validate a ModIR for semantic correctness.
///
/// Checks: E017 (face ID per template), E018 (template existence),
/// E019 (color uniqueness), E020 (cross-category names),
/// E021 (spell face IDs), W008-W011 (HP/face count/item warnings).
pub fn validate_ir(ir: &crate::ir::ModIR) -> ValidationReport {
    use crate::ir::DiceFace;
    use std::collections::HashMap;

    let mut report = ValidationReport::default();

    // E019: Hero color uniqueness (warning — some mods intentionally share colors)
    {
        let mut seen_colors: HashMap<char, String> = HashMap::new();
        for hero in &ir.heroes {
            if let Some(existing) = seen_colors.get(&hero.color) {
                report.warnings.push(Finding {
                    rule_id: E019.to_string(),
                    severity: Severity::Warning,
                    modifier_index: None,
                    modifier_name: Some(hero.mn_name.clone()),
                    position: None,
                    context: Some(format!("color '{}' also used by '{}'", hero.color, existing)),
                    message: format!(
                        "Hero '{}' uses color '{}' which is already used by '{}'",
                        hero.mn_name, hero.color, existing
                    ),
                    ..Default::default()
                });
            } else {
                seen_colors.insert(hero.color, hero.mn_name.clone());
            }
        }
    }

    // E020: Cross-category name uniqueness
    {
        let mut names: HashMap<String, &str> = HashMap::new();
        for hero in &ir.heroes {
            let lower = hero.mn_name.to_lowercase();
            if let Some(category) = names.get(&lower) {
                report.errors.push(Finding {
                    rule_id: E020.to_string(),
                    severity: Severity::Error,
                    modifier_index: None,
                    modifier_name: Some(hero.mn_name.clone()),
                    position: None,
                    context: None,
                    message: format!("Name '{}' conflicts: exists as {} and hero", hero.mn_name, category),
                    ..Default::default()
                });
            } else {
                names.insert(lower, "hero");
            }
        }
        for item in &ir.replica_items {
            let lower = item.name.to_lowercase();
            if let Some(category) = names.get(&lower) {
                report.errors.push(Finding {
                    rule_id: E020.to_string(),
                    severity: Severity::Error,
                    modifier_index: None,
                    modifier_name: Some(item.name.clone()),
                    position: None,
                    context: None,
                    message: format!("Name '{}' conflicts: exists as {} and replica item", item.name, category),
                    ..Default::default()
                });
            } else {
                names.insert(lower, "replica item");
            }
        }
        for monster in &ir.monsters {
            let lower = monster.name.to_lowercase();
            if let Some(category) = names.get(&lower) {
                report.errors.push(Finding {
                    rule_id: E020.to_string(),
                    severity: Severity::Error,
                    modifier_index: None,
                    modifier_name: Some(monster.name.clone()),
                    position: None,
                    context: None,
                    message: format!("Name '{}' conflicts: exists as {} and monster", monster.name, category),
                    ..Default::default()
                });
            } else {
                names.insert(lower, "monster");
            }
        }
        for boss in &ir.bosses {
            let lower = boss.name.to_lowercase();
            if let Some(category) = names.get(&lower) {
                report.errors.push(Finding {
                    rule_id: E020.to_string(),
                    severity: Severity::Error,
                    modifier_index: None,
                    modifier_name: Some(boss.name.clone()),
                    position: None,
                    context: None,
                    message: format!("Name '{}' conflicts: exists as {} and boss", boss.name, category),
                    ..Default::default()
                });
            } else {
                names.insert(lower, "boss");
            }
        }
    }

    // Per-hero-block validation
    for (hero_idx, hero) in ir.heroes.iter().enumerate() {
        for (block_idx, block) in hero.blocks.iter().enumerate() {
            let ctx_name = format!("{}[{}]", hero.mn_name, block_idx);

            // E018: Template existence
            if !block.template.is_empty() && !is_known_template(&block.template) {
                report.errors.push(Finding {
                    rule_id: E018.to_string(),
                    severity: Severity::Error,
                    modifier_index: None,
                    modifier_name: Some(ctx_name.clone()),
                    position: None,
                    context: Some(block.template.clone()),
                    field_path: Some(format!("heroes[{}].blocks[{}].template", hero_idx, block_idx)),
                    suggestion: Some("Check template name against known game templates (Lost, Statue, Thief, Fey, ...)".into()),
                    message: format!(
                        "Unknown template '{}' in hero '{}' block {}",
                        block.template, hero.mn_name, block_idx
                    ),
                });
            }

            // E017: Face ID per template (blocklist — flags clearly inappropriate faces)
            if let Some(rejected) = rejected_faces_for_template(&block.template) {
                for face in &block.sd.faces {
                    if let DiceFace::Active { face_id, .. } = face {
                        if rejected.contains(face_id) {
                            report.errors.push(Finding {
                                rule_id: E017.to_string(),
                                severity: Severity::Error,
                                modifier_index: None,
                                modifier_name: Some(ctx_name.clone()),
                                position: None,
                                context: Some(format!("face_id={}", face_id)),
                                field_path: Some(format!("heroes[{}].blocks[{}].sd", hero_idx, block_idx)),
                                suggestion: Some(format!("Face ID {} is a physical mechanic inappropriate for template '{}'", face_id, block.template)),
                                message: format!(
                                    "Face ID {} inappropriate for template '{}' in hero '{}' block {}",
                                    face_id, block.template, hero.mn_name, block_idx
                                ),
                            });
                        }
                    }
                }
            }

            // E021: Spell face IDs (blocklist for spell template)
            if let Some(ref ability) = block.abilitydata {
                if let Some(rejected) = rejected_faces_for_template(&ability.template) {
                    for face in &ability.sd.faces {
                        if let DiceFace::Active { face_id, .. } = face {
                            if rejected.contains(face_id) {
                                report.errors.push(Finding {
                                    rule_id: E021.to_string(),
                                    severity: Severity::Error,
                                    modifier_index: None,
                                    modifier_name: Some(ctx_name.clone()),
                                    position: None,
                                    context: Some(format!("spell face_id={}", face_id)),
                                    field_path: Some(format!("heroes[{}].blocks[{}].abilitydata.sd", hero_idx, block_idx)),
                                    suggestion: Some(format!("Face ID {} is inappropriate for spell template '{}'", face_id, ability.template)),
                                    message: format!(
                                        "Spell face ID {} inappropriate for spell template '{}' in hero '{}' block {}",
                                        face_id, ability.template, hero.mn_name, block_idx
                                    ),
                                });
                            }
                        }
                    }
                }
            }

            // W008: HP range per tier (skip if hp is None — inherits from template)
            if let (Some(tier), Some(hp)) = (block.tier, block.hp) {
                let (min_hp, max_hp) = hp_range_for_tier(tier);
                if hp < min_hp || hp > max_hp {
                    report.warnings.push(Finding {
                        rule_id: W008.to_string(),
                        severity: Severity::Warning,
                        modifier_index: None,
                        modifier_name: Some(ctx_name.clone()),
                        position: None,
                        context: Some(format!("tier={}, hp={}", tier, hp)),
                        field_path: Some(format!("heroes[{}].blocks[{}].hp", hero_idx, block_idx)),
                        suggestion: Some(format!("Expected HP {}-{} for tier {}", min_hp, max_hp, tier)),
                        message: format!(
                            "Hero '{}' block {} has HP {} for tier {} (expected {}-{})",
                            hero.mn_name, block_idx, hp, tier, min_hp, max_hp
                        ),
                    });
                }
            }

            // W009: SD face count
            if block.sd.faces.len() != 6 {
                report.warnings.push(Finding {
                    rule_id: W009.to_string(),
                    severity: Severity::Warning,
                    modifier_index: None,
                    modifier_name: Some(ctx_name.clone()),
                    position: None,
                    context: Some(format!("count={}", block.sd.faces.len())),
                    field_path: Some(format!("heroes[{}].blocks[{}].sd", hero_idx, block_idx)),
                    suggestion: Some("Dice should have exactly 6 faces".into()),
                    message: format!(
                        "Hero '{}' block {} has {} dice faces (expected 6)",
                        hero.mn_name, block_idx, block.sd.faces.len()
                    ),
                });
            }
        }
    }

    // Per-replica-item validation
    for item in &ir.replica_items {
        // W010: Unknown template
        if !is_known_template(&item.template) {
            report.warnings.push(Finding {
                rule_id: W010.to_string(),
                severity: Severity::Warning,
                modifier_index: None,
                modifier_name: Some(item.name.clone()),
                position: None,
                context: Some(item.template.clone()),
                message: format!("Replica item '{}' uses unknown template '{}'", item.name, item.template),
                ..Default::default()
            });
        }

        // W011: High HP on item with ability
        if item.abilitydata.is_some() {
            if let Some(hp) = item.hp {
                if hp > 20 {
                    report.warnings.push(Finding {
                        rule_id: W011.to_string(),
                        severity: Severity::Warning,
                        modifier_index: None,
                        modifier_name: Some(item.name.clone()),
                        position: None,
                        context: Some(format!("hp={}", hp)),
                        message: format!(
                            "Replica item '{}' has HP {} with ability (typical range 1-20)",
                            item.name, hp
                        ),
                        ..Default::default()
                    });
                }
            }
        }
    }

    report
}
