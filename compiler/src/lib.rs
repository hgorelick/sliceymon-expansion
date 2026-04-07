pub mod error;
pub mod ir;
pub mod extractor;
pub mod builder;
pub mod util;
pub mod validator;

pub use validator::{validate, validate_cross_references, Finding, ValidationReport};

use std::collections::HashMap;

use error::CompilerError;
use ir::ModIR;

/// Extract a textmod string into a structured ModIR.
pub fn extract(textmod: &str) -> Result<ModIR, CompilerError> {
    extractor::extract(textmod)
}

/// Build a textmod string from a ModIR and sprite mappings.
pub fn build(ir: &ModIR, sprites: &HashMap<String, String>) -> Result<String, CompilerError> {
    builder::build(ir, sprites)
}

/// Merge a base ModIR with an overlay ModIR.
pub fn merge(base: ModIR, overlay: ModIR) -> Result<ModIR, CompilerError> {
    ir::merge::merge(base, overlay)
}

/// Serialize a ModIR to JSON.
pub fn ir_to_json(ir: &ModIR) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(ir)
}

/// Deserialize a ModIR from JSON.
pub fn ir_from_json(json: &str) -> Result<ModIR, serde_json::Error> {
    serde_json::from_str(json)
}
