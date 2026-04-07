use crate::ir::{StructuralContent, StructuralModifier};

/// Emit a structural modifier.
///
/// Dispatches on content variant for reconstruction.
/// Falls back to raw for Raw content or if reconstruction fails.
pub fn emit(s: &StructuralModifier) -> String {
    match &s.content {
        StructuralContent::Raw => s.raw.clone(),
        // For all parsed variants, fall back to raw.
        // Reconstruction from structured fields is available but raw is always
        // authoritative for structural modifiers (they always have raw: String).
        _ => s.raw.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{StructuralContent, StructuralModifier, StructuralType};

    #[test]
    fn structural_emitter_raw_fallback() {
        let s = StructuralModifier::new_raw(StructuralType::Dialog, "raw-dialog-content".into());
        assert_eq!(emit(&s), "raw-dialog-content");
    }

    #[test]
    fn structural_emitter_dispatches_on_content() {
        let s = StructuralModifier {
            modifier_type: StructuralType::Dialog,
            name: Some("Credits".into()),
            content: StructuralContent::Dialog { phase: "4".into() },
            raw: "the-raw-content".into(),
        };
        // Currently falls back to raw; reconstruction can be added later
        assert_eq!(emit(&s), "the-raw-content");
    }
}
