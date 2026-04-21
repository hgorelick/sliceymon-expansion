use std::fmt;

#[derive(Debug, Clone)]
pub struct CompilerError {
    // Boxed to keep `Result<T, CompilerError>` small (clippy::result_large_err).
    // The largest `ErrorKind` variants carry up to six String/Option fields; inlining
    // them bloats every Err slot to ~176 bytes even on success paths.
    pub kind: Box<ErrorKind>,
    pub field_path: Option<String>,
    pub suggestion: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Split {
        raw_position: usize,
        message: String,
    },
    Classify {
        modifier_index: usize,
        preview: String,
        message: String,
    },
    HeroParse {
        modifier_index: usize,
        hero_name: String,
        tier_index: Option<usize>,
        position: usize,
        expected: String,
        found: String,
    },
    Paren {
        modifier_index: usize,
        position: usize,
        depth: i32,
    },
    Build {
        component: String,
        message: String,
    },
    MergeConflict {
        key: String,
        base_value: String,
        overlay_value: String,
    },
    SpriteNotFound {
        sprite_name: String,
        hero_name: Option<String>,
        tier_index: Option<usize>,
    },
    FaceIdInvalid {
        raw: u16,
        template: Option<String>,
    },
    DerivedStructuralAuthored {
        modifier_type: String,
    },
    Validation {
        message: String,
    },
    DuplicateName {
        name: String,
        existing_category: String,
        new_category: String,
    },
    DuplicateColor {
        color: char,
        existing_hero: String,
    },
    NotFound {
        type_name: String,
        key: String,
    },
    ChainParse {
        content: String,
        position: usize,
        expected: String,
        found: String,
    },
    PhaseParse {
        phase_code: Option<char>,
        content: String,
        expected: String,
        found: String,
    },
    RewardParse {
        content: String,
        expected: String,
        found: String,
    },
    Io {
        kind: std::io::ErrorKind,
        message: String,
    },
    Json {
        line: Option<usize>,
        column: Option<usize>,
        message: String,
    },
}

impl CompilerError {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind: Box::new(kind),
            field_path: None,
            suggestion: None,
            context: None,
        }
    }

    pub fn with_field_path(mut self, path: impl Into<String>) -> Self {
        self.field_path = Some(path.into());
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn split(raw_position: usize, message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Split {
            raw_position,
            message: message.into(),
        })
    }

    pub fn classify(
        modifier_index: usize,
        preview: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::Classify {
            modifier_index,
            preview: preview.into(),
            message: message.into(),
        })
    }

    pub fn hero_parse(
        modifier_index: usize,
        hero_name: impl Into<String>,
        tier_index: Option<usize>,
        position: usize,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::HeroParse {
            modifier_index,
            hero_name: hero_name.into(),
            tier_index,
            position,
            expected: expected.into(),
            found: found.into(),
        })
    }

    pub fn paren(modifier_index: usize, position: usize, depth: i32) -> Self {
        Self::new(ErrorKind::Paren {
            modifier_index,
            position,
            depth,
        })
    }

    pub fn build(component: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Build {
            component: component.into(),
            message: message.into(),
        })
    }

    pub fn merge_conflict(
        key: impl Into<String>,
        base_value: impl Into<String>,
        overlay_value: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::MergeConflict {
            key: key.into(),
            base_value: base_value.into(),
            overlay_value: overlay_value.into(),
        })
    }

    pub fn sprite_not_found(
        sprite_name: impl Into<String>,
        hero_name: Option<String>,
        tier_index: Option<usize>,
    ) -> Self {
        Self::new(ErrorKind::SpriteNotFound {
            sprite_name: sprite_name.into(),
            hero_name,
            tier_index,
        })
    }

    pub fn face_id_invalid(raw: u16, template: Option<String>) -> Self {
        Self::new(ErrorKind::FaceIdInvalid { raw, template })
    }

    pub fn derived_structural_authored(modifier_type: impl Into<String>) -> Self {
        Self::new(ErrorKind::DerivedStructuralAuthored {
            modifier_type: modifier_type.into(),
        })
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Validation {
            message: message.into(),
        })
    }

    pub fn duplicate_name(
        name: impl Into<String>,
        existing_category: impl Into<String>,
        new_category: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::DuplicateName {
            name: name.into(),
            existing_category: existing_category.into(),
            new_category: new_category.into(),
        })
    }

    pub fn duplicate_color(color: char, existing_hero: impl Into<String>) -> Self {
        Self::new(ErrorKind::DuplicateColor {
            color,
            existing_hero: existing_hero.into(),
        })
    }

    pub fn not_found(type_name: impl Into<String>, key: impl Into<String>) -> Self {
        Self::new(ErrorKind::NotFound {
            type_name: type_name.into(),
            key: key.into(),
        })
    }

    pub fn chain_parse(
        content: impl Into<String>,
        position: usize,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::ChainParse {
            content: content.into(),
            position,
            expected: expected.into(),
            found: found.into(),
        })
    }

    pub fn phase_parse(
        phase_code: Option<char>,
        content: impl Into<String>,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::PhaseParse {
            phase_code,
            content: content.into(),
            expected: expected.into(),
            found: found.into(),
        })
    }

    pub fn reward_parse(
        content: impl Into<String>,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::RewardParse {
            content: content.into(),
            expected: expected.into(),
            found: found.into(),
        })
    }

    pub fn io(kind: std::io::ErrorKind, message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Io {
            kind,
            message: message.into(),
        })
    }

    pub fn json(
        line: Option<usize>,
        column: Option<usize>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(ErrorKind::Json {
            line,
            column,
            message: message.into(),
        })
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind.as_ref() {
            ErrorKind::Split { raw_position, message } => {
                write!(f, "SplitError at position {}: {}", raw_position, message)?;
            }
            ErrorKind::Classify { modifier_index, preview, message } => {
                write!(
                    f,
                    "ClassifyError: modifier #{} could not be classified: {}\n  preview: \"{}\"",
                    modifier_index, message, preview
                )?;
            }
            ErrorKind::HeroParse {
                modifier_index,
                hero_name,
                tier_index,
                position,
                expected,
                found,
            } => {
                let tier_str = match tier_index {
                    Some(t) => format!(", tier {}", t),
                    None => String::new(),
                };
                write!(
                    f,
                    "HeroParseError: hero \"{}\"{}: expected \"{}\" at position {}, found \"{}\"\n  \
                     hint: property order must be template -> col -> tier -> hp -> sd -> img -> ...\n  \
                     modifier index: {}",
                    hero_name, tier_str, expected, position, found, modifier_index
                )?;
            }
            ErrorKind::Paren { modifier_index, position, depth } => {
                write!(
                    f,
                    "ParenError: unbalanced parentheses at position {} (depth {})\n  \
                     modifier index: {}\n  \
                     hint: check for missing closing ')' or extra '('",
                    position, depth, modifier_index
                )?;
            }
            ErrorKind::Build { component, message } => {
                write!(f, "BuildError in {}: {}", component, message)?;
            }
            ErrorKind::MergeConflict { key, base_value, overlay_value } => {
                write!(
                    f,
                    "MergeConflict: key \"{}\" has conflicting values\n  \
                     base: \"{}\"\n  overlay: \"{}\"\n  \
                     hint: resolve by setting an explicit value in the overlay",
                    key, base_value, overlay_value
                )?;
            }
            ErrorKind::SpriteNotFound { sprite_name, hero_name, tier_index } => {
                let hero_str = match hero_name {
                    Some(h) => format!(" for hero \"{}\"", h),
                    None => String::new(),
                };
                let tier_str = match tier_index {
                    Some(t) => format!(", tier {}", t),
                    None => String::new(),
                };
                write!(
                    f,
                    "SpriteNotFound: sprite \"{}\" not found{}{}\n  \
                     hint: check sprite_encodings.json for available sprite names",
                    sprite_name, hero_str, tier_str
                )?;
            }
            ErrorKind::FaceIdInvalid { raw, template } => {
                let tmpl_str = match template {
                    Some(t) => format!(" (template \"{}\")", t),
                    None => String::new(),
                };
                write!(
                    f,
                    "FaceIdInvalid: raw face id {} is not in the whitelist{}",
                    raw, tmpl_str
                )?;
            }
            ErrorKind::DerivedStructuralAuthored { modifier_type } => {
                write!(
                    f,
                    "DerivedStructuralAuthored: \"{}\" is a derived structural and must not be authored directly",
                    modifier_type
                )?;
            }
            ErrorKind::Validation { message } => {
                write!(f, "ValidationError: {}", message)?;
            }
            ErrorKind::DuplicateName { name, existing_category, new_category } => {
                write!(
                    f,
                    "DuplicateName: '{}' already exists as {} (cannot add as {})",
                    name, existing_category, new_category
                )?;
            }
            ErrorKind::DuplicateColor { color, existing_hero } => {
                write!(
                    f,
                    "DuplicateColor: color '{}' already used by hero '{}'",
                    color, existing_hero
                )?;
            }
            ErrorKind::NotFound { type_name, key } => {
                write!(f, "NotFound: {} '{}' does not exist", type_name, key)?;
            }
            ErrorKind::ChainParse { content, position, expected, found } => {
                write!(
                    f,
                    "ChainParseError at position {}: expected \"{}\", found \"{}\"\n  content: \"{}\"",
                    position, expected, found, content
                )?;
            }
            ErrorKind::PhaseParse { phase_code, content, expected, found } => {
                let code_str = match phase_code {
                    Some(c) => format!(" (phase code '{}')", c),
                    None => String::new(),
                };
                write!(
                    f,
                    "PhaseParseError{}: expected \"{}\", found \"{}\"\n  content: \"{}\"",
                    code_str, expected, found, content
                )?;
            }
            ErrorKind::RewardParse { content, expected, found } => {
                write!(
                    f,
                    "RewardParseError: expected \"{}\", found \"{}\"\n  content: \"{}\"",
                    expected, found, content
                )?;
            }
            ErrorKind::Io { kind, message } => {
                write!(f, "IoError ({:?}): {}", kind, message)?;
            }
            ErrorKind::Json { line, column, message } => {
                let loc = match (line, column) {
                    (Some(l), Some(c)) => format!(" at line {}, column {}", l, c),
                    (Some(l), None) => format!(" at line {}", l),
                    _ => String::new(),
                };
                write!(f, "JsonError{}: {}", loc, message)?;
            }
        }

        if let Some(path) = &self.field_path {
            write!(f, "\n  field_path: \"{}\"", path)?;
        }
        if let Some(suggestion) = &self.suggestion {
            write!(f, "\n  suggestion: {}", suggestion)?;
        }
        if let Some(context) = &self.context {
            write!(f, "\n  context: \"{}\"", context)?;
        }
        Ok(())
    }
}

impl std::error::Error for CompilerError {}

impl From<std::io::Error> for CompilerError {
    fn from(e: std::io::Error) -> Self {
        CompilerError::io(e.kind(), e.to_string())
    }
}

impl From<serde_json::Error> for CompilerError {
    fn from(e: serde_json::Error) -> Self {
        // serde_json reports 0 for unknown/EOF positions; normalize to None.
        let line = Some(e.line()).filter(|&l| l > 0);
        let column = Some(e.column()).filter(|&c| c > 0);
        CompilerError::json(line, column, e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_includes_field_path() {
        let err = CompilerError::validation("bad thing")
            .with_field_path("heroes[0].color");
        let text = err.to_string();
        assert!(
            text.contains("field_path: \"heroes[0].color\""),
            "Display output missing field_path line:\n{}",
            text
        );
    }

    #[test]
    fn test_display_includes_suggestion() {
        let err = CompilerError::validation("bad thing")
            .with_suggestion("try setting it to 'r'");
        let text = err.to_string();
        assert!(
            text.contains("suggestion: try setting it to 'r'"),
            "Display output missing suggestion line:\n{}",
            text
        );
    }

    #[test]
    fn test_existing_variants_migrate_cleanly() {
        // Every ErrorKind variant must be constructable via its matching helper.
        let _: CompilerError = CompilerError::split(5, "msg");
        let _: CompilerError = CompilerError::classify(1, "prev", "msg");
        let _: CompilerError =
            CompilerError::hero_parse(1, "x", Some(0), 2, "exp", "found");
        let _: CompilerError = CompilerError::paren(1, 2, 3);
        let _: CompilerError = CompilerError::build("c", "m");
        let _: CompilerError = CompilerError::merge_conflict("k", "b", "o");
        let _: CompilerError =
            CompilerError::sprite_not_found("s", Some("h".into()), Some(0));
        let _: CompilerError = CompilerError::face_id_invalid(42, Some("tpl".into()));
        let _: CompilerError = CompilerError::derived_structural_authored("heropool");
        let _: CompilerError = CompilerError::validation("m");
        let _: CompilerError = CompilerError::duplicate_name("n", "a", "b");
        let _: CompilerError = CompilerError::duplicate_color('r', "h");
        let _: CompilerError = CompilerError::not_found("hero", "x");
        let _: CompilerError = CompilerError::chain_parse("c", 0, "e", "f");
        let _: CompilerError = CompilerError::phase_parse(Some('q'), "c", "e", "f");
        let _: CompilerError = CompilerError::reward_parse("c", "e", "f");
        let _: CompilerError = CompilerError::io(std::io::ErrorKind::NotFound, "io");
        let _: CompilerError = CompilerError::json(Some(3), Some(12), "bad json");
    }

    #[test]
    fn test_with_context_appears_in_display() {
        let err = CompilerError::paren(0, 5, 1).with_context("...(abc...");
        let text = err.to_string();
        assert!(
            text.contains("context: \"...(abc...\""),
            "Display output missing context line:\n{}",
            text
        );
    }
}
