//! Structured findings — errors, warnings, info — produced by xref checks,
//! merge-time validation, and build-time derived-structural policy.
//!
//! Lives at the crate root rather than in `xref/` because both `ir/` (for
//! `ModIR.warnings`) and `xref/` depend on `Finding`; keeping it out of `xref`
//! avoids a circular dependency when `ir` sidecars findings.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ir::Source;

/// Severity level for a validation finding.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum Severity {
    #[default]
    Error,
    Warning,
    Info,
}

/// A single validation finding (error, warning, or info).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
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
    /// Provenance of the offending entity. `None` for global findings that
    /// don't bind to a single sourced entity (e.g. cross-category name clashes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}
