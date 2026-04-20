//! Source-of-truth constant tables derived from Thunder's Undocumented Textmod Guide v3.2.
//! All values trace back to specific sections in the guide. No invented/extrapolated values.
//! WASM-safe: no std::fs or I/O.

// ---------------------------------------------------------------------------
// Phase type codes (the character after "ph.")
// ---------------------------------------------------------------------------

/// All known phase type codes from Thunder's guide.
/// 21 entries: !, 0-9, b, c, d, e, g, l, r, s, t, z
pub const PHASE_CODES: &[char] = &[
    '!', // SimpleChoicePhase (SCPhase)
    '0', // PlayerRolling
    '1', // Targeting
    '2', // LevelEnd
    '3', // EnemyRolling
    '4', // Message
    '5', // HeroChange
    '6', // Reset
    '7', // ItemCombine
    '8', // PositionSwap
    '9', // Challenge
    'b', // BooleanPhase
    'c', // ChoicePhase
    'd', // Damage
    'e', // RunEnd
    'g', // PhaseGenerator
    'l', // LinkedPhase
    'r', // RandomReveal
    's', // SeqPhase
    't', // Trade
    'z', // BooleanPhase2
];

pub fn is_known_phase_code(c: char) -> bool {
    PHASE_CODES.contains(&c)
}

// ---------------------------------------------------------------------------
// Tog item names
// ---------------------------------------------------------------------------

/// All known tog item names (15 entries).
pub const TOG_ITEMS: &[&str] = &[
    "togtime",  // Time-based toggle
    "togtarg",  // Target toggle
    "togfri",   // Friendly toggle
    "togvis",   // Visual toggle
    "togeft",   // Effect toggle
    "togpip",   // Pip toggle
    "togkey",   // Keyword toggle
    "togunt",   // Untargeted toggle
    "togres",   // Restriction copy (basic)
    "togresm",  // Restriction multiply (x2 conditional)
    "togresa",  // Restriction AND
    "togreso",  // Restriction OR
    "togresx",  // Restriction XOR
    "togresn",  // Restriction NOT
    "togress",  // Restriction SWAP
];

pub fn is_known_tog(name: &str) -> bool {
    TOG_ITEMS.contains(&name)
}

// ---------------------------------------------------------------------------
// Reward/choosable tag letters
// ---------------------------------------------------------------------------

/// Single-letter tags used in SCPhase/ChoicePhase rewards (11 entries).
pub const REWARD_TAG_LETTERS: &[(char, &str)] = &[
    ('m', "Modifier"),
    ('i', "Item"),
    ('l', "Levelup"),
    ('g', "Hero"),
    ('r', "Random"),
    ('q', "RandomRange"),
    ('o', "Or"),
    ('e', "Enu"),
    ('v', "Value"),
    ('p', "Replace"),
    ('s', "Skip"),
];

pub fn is_known_reward_tag(c: char) -> bool {
    REWARD_TAG_LETTERS.iter().any(|(ch, _)| *ch == c)
}

pub fn reward_tag_name(c: char) -> Option<&'static str> {
    REWARD_TAG_LETTERS.iter().find(|(ch, _)| *ch == c).map(|(_, name)| *name)
}

// ---------------------------------------------------------------------------
// Hidden modifier names
// ---------------------------------------------------------------------------

/// Known hidden modifiers (12 entries).
pub const HIDDEN_MODIFIER_NAMES: &[&str] = &[
    "Skip",
    "Wish",
    "Clear Party",
    "Missing",
    "Temporary",
    "Hidden",
    "Skip All",
    "Add Fight",
    "Add 10 Fights",
    "Add 100 Fights",
    "Minus Fight",
    "Cursemode Loopdiff",
];

pub fn is_known_hidden_modifier(name: &str) -> bool {
    HIDDEN_MODIFIER_NAMES.contains(&name)
}

// ---------------------------------------------------------------------------
// Hidden items (non-tog)
// ---------------------------------------------------------------------------

/// Known hidden items that are not tog items.
pub const HIDDEN_ITEMS: &[&str] = &[
    "rgreen",
    "clearicon",
    "cleardesc",
    "Idol of Chrzktx",
    "Idol of Aiiu",
    "Idol of Pythagoras",
    "False Idol",
];

pub fn is_known_hidden_item(name: &str) -> bool {
    HIDDEN_ITEMS.contains(&name)
}

// ---------------------------------------------------------------------------
// Side position names (with aliases)
// ---------------------------------------------------------------------------

/// All known side position names and their aliases.
/// The canonical name is listed first, aliases follow.
pub const SIDE_POSITIONS: &[&[&str]] = &[
    &["top"],
    &["mid", "middle"],
    &["bot", "bottom"],
    &["left"],
    &["right"],
    &["topbot"],
    &["rightmost"],
    &["right2"],
    &["all"],
];

pub fn is_known_side(name: &str) -> bool {
    SIDE_POSITIONS.iter().any(|group| group.contains(&name))
}

/// Normalize a side position name to its canonical form.
pub fn normalize_side(name: &str) -> Option<&'static str> {
    SIDE_POSITIONS.iter()
        .find(|group| group.contains(&name))
        .map(|group| group[0])
}

// ---------------------------------------------------------------------------
// Richtext color tags
// ---------------------------------------------------------------------------

/// Known richtext formatting tags from Thunder's guide.
pub const RICHTEXT_TAGS: &[&str] = &[
    "orange",
    "yellow",
    "light",
    "blue",
    "red",
    "cu",   // Close/unset color
    "n",    // Newline
    "nh",   // No highlight
];

pub fn is_known_richtext_tag(tag: &str) -> bool {
    RICHTEXT_TAGS.contains(&tag)
}

// ---------------------------------------------------------------------------
// Choice phase types
// ---------------------------------------------------------------------------

/// Known choice phase type names.
pub const CHOICE_TYPES: &[&str] = &[
    "PointBuy",
    "Number",
    "UpToNumber",
    "Optional",
];

pub fn is_known_choice_type(name: &str) -> bool {
    CHOICE_TYPES.contains(&name)
}

// ---------------------------------------------------------------------------
// Phase delimiters
// ---------------------------------------------------------------------------

/// Phase delimiter table: which delimiter is used by which phase type(s).
pub const PHASE_DELIMITERS: &[(& str, &[&str])] = &[
    ("@1", &["LinkedPhase", "SeqPhase"]),
    ("@2", &["BooleanPhase", "LevelEndPhase", "SeqPhase"]),
    ("@3", &["SimpleChoicePhase", "ChoicePhase", "TradePhase"]),
    ("@4", &["OrTag"]),
    ("@6", &["BooleanPhase2"]),
    ("@7", &["BooleanPhase2"]),
];

// ---------------------------------------------------------------------------
// phi. index table (phase index shortcuts)
// ---------------------------------------------------------------------------

/// phi.N maps to these phase types (0-9).
pub const PHI_INDEX_TABLE: &[(u8, char)] = &[
    (0, '0'), // PlayerRolling
    (1, '1'), // Targeting
    (2, '2'), // LevelEnd
    (3, '3'), // EnemyRolling
    (4, '4'), // Message
    (5, '5'), // HeroChange
    (6, '6'), // Reset
    (7, '7'), // ItemCombine
    (8, '8'), // PositionSwap
    (9, '9'), // Challenge
];

pub fn phi_to_phase_code(index: u8) -> Option<char> {
    PHI_INDEX_TABLE.iter()
        .find(|(i, _)| *i == index)
        .map(|(_, c)| *c)
}

// ---------------------------------------------------------------------------
// ItemCombine types
// ---------------------------------------------------------------------------

/// Known ItemCombine type names.
pub const ITEM_COMBINE_TYPES: &[&str] = &[
    "SecondHighestToTierThrees",
    "ZeroToThreeToSingle",
];

pub fn is_known_item_combine_type(name: &str) -> bool {
    ITEM_COMBINE_TYPES.contains(&name)
}

// ---------------------------------------------------------------------------
// AbilityData side semantics
// ---------------------------------------------------------------------------

/// AbilityData side positions and their semantic meaning.
/// Side indices are 0-based internally but documented as 1-based in Thunder's guide.
pub const ABILITY_SIDE_SEMANTICS: &[(u8, &str)] = &[
    (1, "Primary effect (damage, shield, heal, etc.)"),
    (2, "Secondary untargeted effect (damage all, mana, shield all)"),
    (3, "Tactic cost 1 (if no Side 5)"),
    (4, "Tactic cost 2"),
    (5, "Mana cost (makes it a spell)"),
    (6, "Tactic cost 3"),
];

/// Returns true if the given side index (1-based) indicates spell behavior.
pub fn is_spell_side(side: u8) -> bool {
    side == 5
}

/// Returns true if the given side index (1-based) indicates a tactic cost.
pub fn is_tactic_cost_side(side: u8) -> bool {
    matches!(side, 3 | 4 | 6)
}

// ---------------------------------------------------------------------------
// TriggerHPData HP-to-pip position mapping
// ---------------------------------------------------------------------------

/// Maps HP value to the pip position that triggers.
/// HP 1-21 have specific mappings; HP 22+ uses the formula: position = (hp - 1) % 6.
/// From Thunder's guide: HP values map to which die face activates.
pub const TRIGGER_HP_TABLE: &[(u16, u8)] = &[
    (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6),
    (7, 1), (8, 2), (9, 3), (10, 4), (11, 5), (12, 6),
    (13, 1), (14, 2), (15, 3), (16, 4), (17, 5), (18, 6),
    (19, 1), (20, 2), (21, 3),
];

/// Get the pip position (1-based) for a given HP trigger value.
pub fn trigger_hp_to_pip(hp: u16) -> u8 {
    if hp == 0 {
        return 0;
    }
    // Formula: position = ((hp - 1) % 6) + 1
    (((hp - 1) % 6) + 1) as u8
}

// ---------------------------------------------------------------------------
// Entity wrapper prefixes
// ---------------------------------------------------------------------------

/// Known entity wrapper prefixes.
pub const ENTITY_WRAPPER_PREFIXES: &[(&str, &str)] = &[
    ("orb.s", "Orb (passive on death)"),
    ("vase.", "Vase (death trigger)"),
    ("jinx.", "Jinx (monster death modifier)"),
    ("egg.", "Egg (summon)"),
];

pub fn is_entity_wrapper_prefix(s: &str) -> bool {
    ENTITY_WRAPPER_PREFIXES.iter().any(|(prefix, _)| s.starts_with(prefix))
}

// ---------------------------------------------------------------------------
// Known untargeted effect face IDs
// ---------------------------------------------------------------------------

/// Face IDs that represent untargeted effects (relevant for AbilityData side 2).
pub const UNTARGETED_FACE_IDS: &[(u16, &str)] = &[
    (125, "Reroll"),
    (35,  "Revive"),
    (136, "Revive (alt)"),
    (166, "Revive (alt 2)"),
    (76,  "Mana"),
    (34,  "Damage all"),
    (128, "Damage all (alt)"),
    (54,  "Damage ALL (caps)"),
    (158, "Damage ALL (alt)"),
    (160, "Damage ALL (alt 2)"),
    (72,  "Shield all"),
    (73,  "Shield all (alt)"),
    (107, "Heal all"),
];

pub fn is_untargeted_face_id(id: u16) -> bool {
    UNTARGETED_FACE_IDS.iter().any(|(fid, _)| *fid == id)
}

/// Get the name/description for an untargeted face ID.
pub fn untargeted_face_name(id: u16) -> Option<&'static str> {
    UNTARGETED_FACE_IDS.iter()
        .find(|(fid, _)| *fid == id)
        .map(|(_, name)| *name)
}

// ---------------------------------------------------------------------------
// Entity reference prefixes (ritemx., rmod., rmon.)
// ---------------------------------------------------------------------------

/// Known entity reference type prefixes.
pub const ENTITY_REF_PREFIXES: &[(&str, &str)] = &[
    ("ritemx.", "Item reference"),
    ("rmod.",   "Modifier reference"),
    ("rmon.",   "Monster reference"),
];

pub fn entity_ref_kind(s: &str) -> Option<&'static str> {
    ENTITY_REF_PREFIXES.iter()
        .find(|(prefix, _)| s.starts_with(prefix))
        .map(|(_, kind)| *kind)
}

// ---------------------------------------------------------------------------
// Max phase recursion depth
// ---------------------------------------------------------------------------

pub const MAX_PHASE_DEPTH: usize = 10;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_phase_codes_contains_all_21() {
        assert_eq!(PHASE_CODES.len(), 21);
    }

    #[test]
    fn test_known_tog_names_contains_all_15() {
        assert_eq!(TOG_ITEMS.len(), 15);
    }

    #[test]
    fn test_is_known_phase_code_accepts_valid() {
        assert!(is_known_phase_code('b'));
        assert!(is_known_phase_code('!'));
        assert!(is_known_phase_code('0'));
        assert!(is_known_phase_code('z'));
    }

    #[test]
    fn test_is_known_phase_code_rejects_invalid() {
        assert!(!is_known_phase_code('x'));
        assert!(!is_known_phase_code('a'));
        assert!(!is_known_phase_code('Z'));
    }

    #[test]
    fn test_is_known_tog_name_accepts_togres() {
        assert!(is_known_tog("togres"));
    }

    #[test]
    fn test_is_known_tog_name_accepts_togresm() {
        // togres is a substring of togresm — must not false-match
        assert!(is_known_tog("togresm"));
        assert!(is_known_tog("togres"));
    }

    #[test]
    fn test_is_known_side_position_accepts_aliases() {
        assert!(is_known_side("mid"));
        assert!(is_known_side("middle"));
        assert!(is_known_side("bot"));
        assert!(is_known_side("bottom"));
        assert!(is_known_side("top"));
        assert!(is_known_side("all"));
    }

    #[test]
    fn test_is_known_richtext_tag_accepts_cu() {
        assert!(is_known_richtext_tag("cu"));
    }

    #[test]
    fn test_is_known_richtext_tag_rejects_unknown() {
        assert!(!is_known_richtext_tag("purple"));
        assert!(!is_known_richtext_tag("green"));
    }

    #[test]
    fn test_reward_tag_letter_m_maps_to_modifier() {
        assert_eq!(reward_tag_name('m'), Some("Modifier"));
    }

    #[test]
    fn test_hidden_modifier_names_complete() {
        assert_eq!(HIDDEN_MODIFIER_NAMES.len(), 12);
    }

    #[test]
    fn test_normalize_side_canonical() {
        assert_eq!(normalize_side("middle"), Some("mid"));
        assert_eq!(normalize_side("bottom"), Some("bot"));
        assert_eq!(normalize_side("top"), Some("top"));
        assert_eq!(normalize_side("unknown"), None);
    }

    #[test]
    fn test_trigger_hp_to_pip_formula() {
        assert_eq!(trigger_hp_to_pip(1), 1);
        assert_eq!(trigger_hp_to_pip(6), 6);
        assert_eq!(trigger_hp_to_pip(7), 1);
        assert_eq!(trigger_hp_to_pip(13), 1);
        assert_eq!(trigger_hp_to_pip(22), 4); // (22-1)%6+1 = 4
    }

    #[test]
    fn test_phi_to_phase_code() {
        assert_eq!(phi_to_phase_code(0), Some('0'));
        assert_eq!(phi_to_phase_code(4), Some('4'));
        assert_eq!(phi_to_phase_code(9), Some('9'));
        assert_eq!(phi_to_phase_code(10), None);
    }

    #[test]
    fn test_entity_wrapper_detection() {
        assert!(is_entity_wrapper_prefix("orb.sSlimelet"));
        assert!(is_entity_wrapper_prefix("vase.(something)"));
        assert!(is_entity_wrapper_prefix("jinx.modifier"));
        assert!(is_entity_wrapper_prefix("egg.Zombie"));
        assert!(!is_entity_wrapper_prefix("hat.Entity"));
    }

    #[test]
    fn test_entity_ref_kind_detection() {
        assert_eq!(entity_ref_kind("ritemx.dae9"), Some("Item reference"));
        assert_eq!(entity_ref_kind("rmod.1270"), Some("Modifier reference"));
        assert_eq!(entity_ref_kind("rmon.8"), Some("Monster reference"));
        assert_eq!(entity_ref_kind("other.thing"), None);
    }

    #[test]
    fn test_untargeted_face_ids() {
        assert!(is_untargeted_face_id(125)); // Reroll
        assert!(is_untargeted_face_id(76));  // Mana
        assert!(is_untargeted_face_id(34));  // Damage all
        assert!(!is_untargeted_face_id(1));  // Not untargeted
    }

    #[test]
    fn test_ability_side_semantics() {
        assert!(is_spell_side(5));
        assert!(!is_spell_side(1));
        assert!(is_tactic_cost_side(3));
        assert!(is_tactic_cost_side(4));
        assert!(is_tactic_cost_side(6));
        assert!(!is_tactic_cost_side(5));
    }
}
