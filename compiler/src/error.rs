use std::fmt;

#[derive(Debug, Clone)]
pub enum CompilerError {
    /// Modifier splitting failed
    SplitError {
        raw_position: usize,
        message: String,
    },

    /// Modifier could not be classified
    ClassifyError {
        modifier_index: usize,
        preview: String,
        message: String,
    },

    /// Hero parsing failed
    HeroParseError {
        modifier_index: usize,
        hero_name: String,
        tier_index: Option<usize>,
        position: usize,
        expected: String,
        found: String,
    },

    /// Parenthesis balance error (critical -- game silently rejects)
    ParenError {
        modifier_index: usize,
        position: usize,
        depth: i32,
        context: String,
    },

    /// Builder emission error
    BuildError {
        component: String,
        message: String,
    },

    /// Overlay merge conflict
    MergeConflict {
        key: String,
        base_value: String,
        overlay_value: String,
    },

    /// Sprite resolution failed
    SpriteNotFound {
        sprite_name: String,
        hero_name: String,
        tier_index: usize,
    },

    /// IR validation error
    ValidationError {
        message: String,
    },

    /// Duplicate name across categories (hero, replica item, monster, boss)
    DuplicateName {
        name: String,
        existing_category: String,
        new_category: String,
    },

    /// Duplicate hero color
    DuplicateColor {
        color: char,
        existing_hero: String,
    },

    /// Item not found for remove/update
    NotFound {
        type_name: String,
        key: String,
    },

    /// Chain entry parse error
    ChainParseError {
        content: String,
        position: usize,
        expected: String,
        found: String,
    },

    /// Phase parse error
    PhaseParseError {
        phase_code: Option<char>,
        content: String,
        expected: String,
        found: String,
    },

    /// Reward tag parse error
    RewardParseError {
        content: String,
        expected: String,
        found: String,
    },

    /// IO error (CLI only)
    IoError(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::SplitError { raw_position, message } => {
                write!(f, "SplitError at position {}: {}", raw_position, message)
            }
            CompilerError::ClassifyError { modifier_index, preview, message } => {
                write!(
                    f,
                    "ClassifyError: modifier #{} could not be classified: {}\n  preview: \"{}\"",
                    modifier_index, message, preview
                )
            }
            CompilerError::HeroParseError {
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
                )
            }
            CompilerError::ParenError {
                modifier_index,
                position,
                depth,
                context,
            } => {
                write!(
                    f,
                    "ParenError: unbalanced parentheses at position {} (depth {})\n  \
                     context: \"...{}...\"\n  modifier index: {}\n  \
                     hint: check for missing closing ')' or extra '('",
                    position, depth, context, modifier_index
                )
            }
            CompilerError::BuildError { component, message } => {
                write!(f, "BuildError in {}: {}", component, message)
            }
            CompilerError::MergeConflict {
                key,
                base_value,
                overlay_value,
            } => {
                write!(
                    f,
                    "MergeConflict: key \"{}\" has conflicting values\n  \
                     base: \"{}\"\n  overlay: \"{}\"\n  \
                     hint: resolve by setting an explicit value in the overlay",
                    key, base_value, overlay_value
                )
            }
            CompilerError::SpriteNotFound {
                sprite_name,
                hero_name,
                tier_index,
            } => {
                write!(
                    f,
                    "SpriteNotFound: sprite \"{}\" not found for hero \"{}\", tier {}\n  \
                     hint: check sprite_encodings.json for available sprite names",
                    sprite_name, hero_name, tier_index
                )
            }
            CompilerError::ValidationError { message } => {
                write!(f, "ValidationError: {}", message)
            }
            CompilerError::DuplicateName { name, existing_category, new_category } => {
                write!(
                    f,
                    "DuplicateName: '{}' already exists as {} (cannot add as {})",
                    name, existing_category, new_category
                )
            }
            CompilerError::DuplicateColor { color, existing_hero } => {
                write!(
                    f,
                    "DuplicateColor: color '{}' already used by hero '{}'",
                    color, existing_hero
                )
            }
            CompilerError::NotFound { type_name, key } => {
                write!(f, "NotFound: {} '{}' does not exist", type_name, key)
            }
            CompilerError::ChainParseError { content, position, expected, found } => {
                write!(
                    f,
                    "ChainParseError at position {}: expected \"{}\", found \"{}\"\n  content: \"{}\"",
                    position, expected, found, content
                )
            }
            CompilerError::PhaseParseError { phase_code, content, expected, found } => {
                let code_str = match phase_code {
                    Some(c) => format!(" (phase code '{}')", c),
                    None => String::new(),
                };
                write!(
                    f,
                    "PhaseParseError{}: expected \"{}\", found \"{}\"\n  content: \"{}\"",
                    code_str, expected, found, content
                )
            }
            CompilerError::RewardParseError { content, expected, found } => {
                write!(
                    f,
                    "RewardParseError: expected \"{}\", found \"{}\"\n  content: \"{}\"",
                    expected, found, content
                )
            }
            CompilerError::IoError(msg) => {
                write!(f, "IoError: {}", msg)
            }
        }
    }
}

impl std::error::Error for CompilerError {}

impl From<std::io::Error> for CompilerError {
    fn from(e: std::io::Error) -> Self {
        CompilerError::IoError(e.to_string())
    }
}

impl From<serde_json::Error> for CompilerError {
    fn from(e: serde_json::Error) -> Self {
        CompilerError::BuildError {
            component: "json".to_string(),
            message: e.to_string(),
        }
    }
}
