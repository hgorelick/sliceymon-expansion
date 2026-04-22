//! Parse phase strings (ph.X...) into typed Phase IR.
//! Handles recursive nesting for LinkedPhase, BooleanPhase, SeqPhase.

use crate::constants::MAX_PHASE_DEPTH;
use crate::error::CompilerError;
use crate::ir::*;

/// Parse a phase string into a Phase struct.
/// Input should start with an optional level scope prefix then `ph.`.
/// Example: `"5.ph.4Hello"` or `"ph.!m(Sword)@3m(Shield)"`
pub fn parse_phase(input: &str) -> Result<Phase, CompilerError> {
    parse_phase_at_depth(input, 0)
}

fn parse_phase_at_depth(input: &str, depth: usize) -> Result<Phase, CompilerError> {
    if depth > MAX_PHASE_DEPTH {
        return Err(CompilerError::phase_parse(
            None,
            input.to_string(),
            format!("phase nesting depth <= {}", MAX_PHASE_DEPTH),
            format!("depth {}", depth),
        ));
    }

    // Strip level scope prefix
    let (level_scope, rest) = crate::extractor::level_scope_parser::parse_level_scope(input);

    // Expect "ph." prefix
    let after_ph = rest.strip_prefix("ph.").ok_or_else(|| {
        CompilerError::phase_parse(
            None,
            input.to_string(),
            "ph.",
            if rest.len() > 20 { rest[..20].to_string() } else { rest.to_string() },
        )
    })?;

    // Extract phase code (first char after ph.)
    let Some(code) = after_ph.chars().next() else {
        return Err(CompilerError::phase_parse(
            None,
            input.to_string(),
            "phase type code after ph.",
            "empty string",
        )
        .with_field_path("phase.type_code")
        .with_suggestion("expected one of: ! 0-9 b c d e g l r s t y z z<digit>"));
    };
    let content_str = &after_ph[code.len_utf8()..];

    let (phase_type, content) = match code {
        '!' => (PhaseType::SimpleChoice, parse_simple_choice(content_str)?),
        '0' => (PhaseType::PlayerRolling, PhaseContent::PlayerRolling),
        '1' => (PhaseType::Targeting, PhaseContent::Targeting),
        '2' => (PhaseType::LevelEnd, parse_level_end(content_str, depth)?),
        '3' => (PhaseType::EnemyRolling, PhaseContent::EnemyRolling),
        '4' => (PhaseType::Message, parse_message(content_str)),
        '5' => (PhaseType::HeroChange, parse_hero_change(content_str)?),
        '6' => (PhaseType::Reset, PhaseContent::Reset),
        '7' => (PhaseType::ItemCombine, parse_item_combine(content_str)),
        '8' => (PhaseType::PositionSwap, parse_position_swap(content_str)?),
        '9' => (PhaseType::Challenge, parse_challenge(content_str)?),
        'b' => (PhaseType::Boolean, parse_boolean(content_str, depth)?),
        'c' => (PhaseType::Choice, parse_choice(content_str)?),
        'd' => (PhaseType::Damage, PhaseContent::Damage),
        'e' => (PhaseType::RunEnd, PhaseContent::RunEnd),
        'g' => (PhaseType::PhaseGenerator, parse_phase_generator(content_str)?),
        'l' => (PhaseType::Linked, parse_linked(content_str, depth)?),
        'r' => (PhaseType::RandomReveal, parse_random_reveal(content_str)?),
        's' => (PhaseType::Seq, parse_seq(content_str, depth)?),
        't' => (PhaseType::Trade, parse_trade(content_str)?),
        'z' => (PhaseType::Boolean2, parse_boolean2(content_str, depth)?),
        _ => {
            return Err(CompilerError::phase_parse(
                Some(code),
                input.to_string(),
                "known phase code (!, 0-9, b-g, l, r, s, t, z)",
                format!("'{}'", code),
            ));
        }
    };

    Ok(Phase { phase_type, level_scope, content })
}

/// Split a string by a delimiter at paren depth 0.
fn split_at_depth0<'a>(s: &'a str, delim: &str) -> Vec<&'a str> {
    let mut parts = Vec::new();
    let mut depth: i32 = 0;
    let mut start = 0;
    let delim_bytes = delim.as_bytes();

    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if depth == 0 && i + delim_bytes.len() <= bytes.len()
                && &bytes[i..i + delim_bytes.len()] == delim_bytes =>
            {
                parts.push(&s[start..i]);
                start = i + delim_bytes.len();
                i = start;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    parts.push(&s[start..]);
    parts
}

/// Split a string by `;` at paren depth 0.
fn split_semicolons(s: &str) -> Vec<&str> {
    split_at_depth0(s, ";")
}

// -- Phase type parsers --

fn parse_message(content: &str) -> PhaseContent {
    // ph.4TEXT[;BUTTON]
    let parts = split_semicolons(content);
    let text = RichText::new(parts[0]);
    let button_text = if parts.len() > 1 && !parts[1].is_empty() {
        Some(parts[1].to_string())
    } else {
        None
    };
    PhaseContent::Message { text, button_text }
}

fn parse_simple_choice(content: &str) -> Result<PhaseContent, CompilerError> {
    // ph.![TITLE;]REWARD[@3REWARD...]
    // Title is before first reward if present
    let parts = split_semicolons(content);
    let (title, rewards_str) = if parts.len() > 1 {
        (Some(parts[0].to_string()), parts[1..].join(";"))
    } else {
        (None, content.to_string())
    };
    let reward_strs = split_at_depth0(&rewards_str, "@3");
    let rewards = reward_strs.iter()
        .filter(|s| !s.is_empty())
        .map(|s| parse_reward_tag_basic(s))
        .collect();
    Ok(PhaseContent::SimpleChoice { title, rewards })
}

fn parse_boolean(content: &str, depth: usize) -> Result<PhaseContent, CompilerError> {
    // ph.bVALUE;THRESHOLD;PHASE_TRUE@2PHASE_FALSE
    let parts = split_semicolons(content);
    if parts.len() < 3 {
        return Err(CompilerError::phase_parse(
            Some('b'),
            content.to_string(),
            "value;threshold;phases",
            format!("{} semicolon-separated parts", parts.len()),
        ));
    }
    let value_name = parts[0].to_string();
    let threshold = parts[1].parse::<i32>().unwrap_or(0);
    let phase_content = parts[2..].join(";");
    let branches = split_at_depth0(&phase_content, "@2");

    let if_true_str = branches.first().unwrap_or(&"");
    let if_false_str = branches.get(1).unwrap_or(&"");

    let if_true = parse_phase_from_reward_wrapper(if_true_str, depth + 1)?;
    let if_false = parse_phase_from_reward_wrapper(if_false_str, depth + 1)?;

    Ok(PhaseContent::Boolean {
        value_name,
        threshold,
        if_true: Box::new(if_true),
        if_false: Box::new(if_false),
    })
}

fn parse_boolean2(content: &str, depth: usize) -> Result<PhaseContent, CompilerError> {
    // ph.z — same as Boolean but uses @6/@7 instead of ;/@2
    let parts = split_at_depth0(content, "@6");
    if parts.len() < 2 {
        return Err(CompilerError::phase_parse(
            Some('z'),
            content.to_string(),
            "value@6threshold@6phases with @7 branch",
            format!("{} @6-separated parts", parts.len()),
        ));
    }
    let value_name = parts[0].to_string();
    let rest = parts[1..].join("@6");
    let rest_parts = split_at_depth0(&rest, "@6");
    let threshold = rest_parts.first().unwrap_or(&"0").parse::<i32>().unwrap_or(0);
    let phase_content = if rest_parts.len() > 1 { rest_parts[1..].join("@6") } else { String::new() };
    let branches = split_at_depth0(&phase_content, "@7");

    let if_true_str = branches.first().unwrap_or(&"");
    let if_false_str = branches.get(1).unwrap_or(&"");

    let if_true = parse_phase_from_reward_wrapper(if_true_str, depth + 1)?;
    let if_false = parse_phase_from_reward_wrapper(if_false_str, depth + 1)?;

    Ok(PhaseContent::Boolean {
        value_name,
        threshold,
        if_true: Box::new(if_true),
        if_false: Box::new(if_false),
    })
}

fn parse_linked(content: &str, depth: usize) -> Result<PhaseContent, CompilerError> {
    // ph.l(phase1)@1(phase2)@1...
    let parts = split_at_depth0(content, "@1");
    let mut phases = Vec::new();
    for part in &parts {
        let trimmed = part.trim();
        if trimmed.is_empty() { continue; }
        let inner = strip_outer_parens(trimmed);
        phases.push(parse_phase_at_depth(inner, depth + 1)?);
    }
    Ok(PhaseContent::Linked { phases })
}

fn parse_seq(content: &str, depth: usize) -> Result<PhaseContent, CompilerError> {
    // ph.sMESSAGE@1BUTTON1@2PHASE1@1BUTTON2@2PHASE2...
    let at1_parts = split_at_depth0(content, "@1");
    let message = RichText::new(*at1_parts.first().unwrap_or(&""));
    let mut options = Vec::new();

    for part in at1_parts.iter().skip(1) {
        let at2_parts = split_at_depth0(part, "@2");
        let button_text = at2_parts.first().unwrap_or(&"").to_string();
        let mut phases = Vec::new();
        for phase_str in at2_parts.iter().skip(1) {
            let trimmed = phase_str.trim();
            if trimmed.is_empty() { continue; }
            let inner = strip_reward_wrapper(trimmed);
            if !inner.is_empty() {
                phases.push(parse_phase_at_depth(inner, depth + 1)?);
            }
        }
        options.push(SeqOption { button_text, phases });
    }

    Ok(PhaseContent::Seq { message, options })
}

fn parse_trade(content: &str) -> Result<PhaseContent, CompilerError> {
    // ph.tREWARD[@3REWARD...]
    let reward_strs = split_at_depth0(content, "@3");
    let rewards = reward_strs.iter()
        .filter(|s| !s.is_empty())
        .map(|s| parse_reward_tag_basic(s))
        .collect();
    Ok(PhaseContent::Trade { rewards })
}

fn parse_choice(content: &str) -> Result<PhaseContent, CompilerError> {
    // ph.cTYPE#N;REWARD[@3REWARD...]
    let parts = split_semicolons(content);
    let type_str = parts.first().unwrap_or(&"");
    let choice_type = parse_choice_type(type_str);
    let rewards_str = if parts.len() > 1 { parts[1..].join(";") } else { String::new() };
    let reward_strs = split_at_depth0(&rewards_str, "@3");
    let rewards = reward_strs.iter()
        .filter(|s| !s.is_empty())
        .map(|s| parse_reward_tag_basic(s))
        .collect();
    Ok(PhaseContent::Choice { choice_type, rewards })
}

fn parse_hero_change(content: &str) -> Result<PhaseContent, CompilerError> {
    // ph.5XY where X=position, Y=type (0=RandomClass, 1=GeneratedHero)
    let bytes = content.as_bytes();
    let hero_position = if !bytes.is_empty() {
        bytes[0].wrapping_sub(b'0')
    } else { 0 };
    let change_type = if bytes.len() > 1 && bytes[1] == b'1' {
        HeroChangeType::GeneratedHero
    } else {
        HeroChangeType::RandomClass
    };
    Ok(PhaseContent::HeroChange { hero_position, change_type })
}

fn parse_position_swap(content: &str) -> Result<PhaseContent, CompilerError> {
    // ph.8XY where X=first position, Y=second position
    let bytes = content.as_bytes();
    let first = if !bytes.is_empty() { bytes[0].wrapping_sub(b'0') } else { 0 };
    let second = if bytes.len() > 1 { bytes[1].wrapping_sub(b'0') } else { 0 };
    Ok(PhaseContent::PositionSwap { first, second })
}

fn parse_item_combine(content: &str) -> PhaseContent {
    // ph.7TYPE
    PhaseContent::ItemCombine { combine_type: content.to_string() }
}

fn parse_challenge(content: &str) -> Result<PhaseContent, CompilerError> {
    // ph.9 — challenge with reward and optional extra monsters
    let reward_strs = split_at_depth0(content, "@3");
    let reward = reward_strs.iter()
        .filter(|s| !s.is_empty())
        .map(|s| parse_reward_tag_basic(s))
        .collect();
    Ok(PhaseContent::Challenge { reward, extra_monsters: vec![] })
}

fn parse_random_reveal(content: &str) -> Result<PhaseContent, CompilerError> {
    // ph.rREWARD
    let reward = parse_reward_tag_basic(content);
    Ok(PhaseContent::RandomReveal { reward })
}

fn parse_phase_generator(content: &str) -> Result<PhaseContent, CompilerError> {
    // ph.gh or ph.gi
    let gen_type = match content.chars().next() {
        Some('h') => PhaseGenType::Hero,
        Some('i') => PhaseGenType::Item,
        _ => PhaseGenType::Hero,
    };
    Ok(PhaseContent::PhaseGenerator { gen_type })
}

fn parse_level_end(content: &str, depth: usize) -> Result<PhaseContent, CompilerError> {
    // ph.2 — may contain sub-phases
    if content.is_empty() {
        return Ok(PhaseContent::LevelEnd { phases: vec![] });
    }
    let parts = split_at_depth0(content, "@2");
    let mut phases = Vec::new();
    for part in &parts {
        let trimmed = part.trim();
        if trimmed.is_empty() { continue; }
        let inner = strip_reward_wrapper(trimmed);
        if inner.contains("ph.") {
            phases.push(parse_phase_at_depth(inner, depth + 1)?);
        }
    }
    Ok(PhaseContent::LevelEnd { phases })
}

fn parse_choice_type(s: &str) -> ChoiceType {
    if let Some(rest) = s.strip_prefix("PointBuy#") {
        let budget = rest.parse::<i32>().unwrap_or(0);
        return ChoiceType::PointBuy { budget };
    }
    if let Some(rest) = s.strip_prefix("Number#") {
        let count = rest.parse::<u8>().unwrap_or(1);
        return ChoiceType::Number { count };
    }
    if let Some(rest) = s.strip_prefix("UpToNumber#") {
        let max = rest.parse::<u8>().unwrap_or(1);
        return ChoiceType::UpToNumber { max };
    }
    if s.starts_with("Optional") {
        return ChoiceType::Optional;
    }
    // Default to Number#1
    ChoiceType::Number { count: 1 }
}

// -- Helpers --

/// Strip outer parens if present and balanced.
fn strip_outer_parens(s: &str) -> &str {
    if s.starts_with('(') && s.ends_with(')') {
        let inner = &s[1..s.len() - 1];
        // Verify the parens are actually balanced (not just happening to start/end with parens)
        let mut depth: i32 = 0;
        for c in inner.chars() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth < 0 { return s; } // Unbalanced — don't strip
                }
                _ => {}
            }
        }
        if depth == 0 { inner } else { s }
    } else {
        s
    }
}

/// Strip the `!m(...)` wrapper that wraps phases in reward context.
fn strip_reward_wrapper(s: &str) -> &str {
    if let Some(inner) = s.strip_prefix("!m(") {
        if inner.ends_with(')') {
            return &inner[..inner.len() - 1];
        }
    }
    s
}

/// Parse a phase from a string that may be wrapped in `!m(...)`.
fn parse_phase_from_reward_wrapper(s: &str, depth: usize) -> Result<Phase, CompilerError> {
    let inner = strip_reward_wrapper(s);
    if inner.is_empty() || !inner.contains("ph.") {
        // Create a minimal phase for empty content
        return Ok(Phase {
            phase_type: PhaseType::Message,
            level_scope: None,
            content: PhaseContent::Message {
                text: RichText::new(s),
                button_text: None,
            },
        });
    }
    parse_phase_at_depth(inner, depth)
}

/// Basic reward tag parse — creates a RewardTag from a tag string.
/// Full structured reward parsing is in Chunk 7 (reward_parser.rs).
fn parse_reward_tag_basic(s: &str) -> RewardTag {
    let tag_type = match s.chars().next() {
        Some('m') => RewardTagType::Modifier,
        Some('i') => RewardTagType::Item,
        Some('l') => RewardTagType::Levelup,
        Some('g') => RewardTagType::Hero,
        Some('r') => RewardTagType::Random,
        Some('q') => RewardTagType::RandomRange,
        Some('o') => RewardTagType::Or,
        Some('e') => RewardTagType::Enu,
        Some('v') => RewardTagType::Value,
        Some('p') => RewardTagType::Replace,
        Some('s') => RewardTagType::Skip,
        _ => RewardTagType::Modifier,
    };
    RewardTag {
        tag_type,
        content: s.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message_phase() {
        let phase = parse_phase("ph.4Hello World").unwrap();
        assert_eq!(phase.phase_type, PhaseType::Message);
        if let PhaseContent::Message { text, button_text } = &phase.content {
            assert_eq!(text.as_str(), "Hello World");
            assert!(button_text.is_none());
        } else {
            panic!("Expected Message");
        }
    }

    #[test]
    fn test_parse_message_with_button() {
        let phase = parse_phase("ph.4Hello;OK").unwrap();
        if let PhaseContent::Message { text, button_text } = &phase.content {
            assert_eq!(text.as_str(), "Hello");
            assert_eq!(button_text.as_deref(), Some("OK"));
        } else {
            panic!("Expected Message");
        }
    }

    #[test]
    fn test_parse_run_end() {
        let phase = parse_phase("ph.e").unwrap();
        assert_eq!(phase.phase_type, PhaseType::RunEnd);
        assert_eq!(phase.content, PhaseContent::RunEnd);
    }

    #[test]
    fn test_parse_reset() {
        let phase = parse_phase("ph.6").unwrap();
        assert_eq!(phase.phase_type, PhaseType::Reset);
        assert_eq!(phase.content, PhaseContent::Reset);
    }

    #[test]
    fn test_parse_position_swap() {
        let phase = parse_phase("ph.813").unwrap();
        assert_eq!(phase.phase_type, PhaseType::PositionSwap);
        if let PhaseContent::PositionSwap { first, second } = &phase.content {
            assert_eq!(*first, 1);
            assert_eq!(*second, 3);
        } else {
            panic!("Expected PositionSwap");
        }
    }

    #[test]
    fn test_parse_with_level_scope() {
        let phase = parse_phase("5.ph.4Hello").unwrap();
        assert!(phase.level_scope.is_some());
        let scope = phase.level_scope.unwrap();
        assert_eq!(scope.start, 5);
        assert_eq!(phase.phase_type, PhaseType::Message);
    }

    #[test]
    fn test_parse_unknown_phase_code_errors() {
        let result = parse_phase("ph.qsomething");
        assert!(result.is_err());
        if let Err(err) = result {
            if let crate::error::ErrorKind::PhaseParse { phase_code, .. } = err.kind.as_ref() {
                assert_eq!(*phase_code, Some('q'));
            } else {
                panic!("Expected PhaseParse kind, got {:?}", err.kind);
            }
        }
    }

    #[test]
    fn test_parse_simple_choice() {
        let phase = parse_phase("ph.!m(skip)@3m(skip2)").unwrap();
        assert_eq!(phase.phase_type, PhaseType::SimpleChoice);
        if let PhaseContent::SimpleChoice { rewards, .. } = &phase.content {
            assert_eq!(rewards.len(), 2);
        } else {
            panic!("Expected SimpleChoice");
        }
    }

    #[test]
    fn test_parse_trade() {
        let phase = parse_phase("ph.tm(item1)@3m(item2)").unwrap();
        assert_eq!(phase.phase_type, PhaseType::Trade);
        if let PhaseContent::Trade { rewards } = &phase.content {
            assert_eq!(rewards.len(), 2);
        } else {
            panic!("Expected Trade");
        }
    }

    #[test]
    fn test_parse_boolean_phase() {
        let phase = parse_phase("ph.bscore;5;!m(ph.4You win)@2!m(ph.4Try again)").unwrap();
        assert_eq!(phase.phase_type, PhaseType::Boolean);
        if let PhaseContent::Boolean { value_name, threshold, .. } = &phase.content {
            assert_eq!(value_name, "score");
            assert_eq!(*threshold, 5);
        } else {
            panic!("Expected Boolean");
        }
    }

    #[test]
    fn test_parse_deeply_nested_errors() {
        // Build a deeply nested linked phase
        let mut s = "ph.4leaf".to_string();
        for _ in 0..12 {
            s = format!("ph.l({})@1(ph.4end)", s);
        }
        let result = parse_phase(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_damage() {
        let phase = parse_phase("ph.d").unwrap();
        assert_eq!(phase.phase_type, PhaseType::Damage);
        assert_eq!(phase.content, PhaseContent::Damage);
    }

    #[test]
    fn test_parse_item_combine() {
        let phase = parse_phase("ph.7SecondHighestToTierThrees").unwrap();
        assert_eq!(phase.phase_type, PhaseType::ItemCombine);
        if let PhaseContent::ItemCombine { combine_type } = &phase.content {
            assert_eq!(combine_type, "SecondHighestToTierThrees");
        } else {
            panic!("Expected ItemCombine");
        }
    }

    #[test]
    fn test_parse_phase_generator() {
        let phase = parse_phase("ph.gh").unwrap();
        assert_eq!(phase.phase_type, PhaseType::PhaseGenerator);
        if let PhaseContent::PhaseGenerator { gen_type } = &phase.content {
            assert_eq!(*gen_type, PhaseGenType::Hero);
        } else {
            panic!("Expected PhaseGenerator");
        }
    }

    #[test]
    fn phase_parser_malformed_propagates_error() {
        // `ph.` with nothing after is the pathological input previously
        // reaching `after_ph.chars().next().unwrap()` at phase_parser.rs:48.
        // It must now return `Err(PhaseParse)` with the original `input`
        // carried on the error — source-vs-IR proof: `content` must equal
        // the source bytes "ph.", not a canonicalized or registry-derived
        // form.
        let err = parse_phase("ph.").expect_err("empty phase code must not parse");
        match err.kind.as_ref() {
            crate::error::ErrorKind::PhaseParse { content, expected, found, .. } => {
                assert_eq!(content, "ph.");
                assert_eq!(expected, "phase type code after ph.");
                assert_eq!(found, "empty string");
            }
            other => panic!("expected PhaseParse, got {:?}", other),
        }
        assert_eq!(err.field_path.as_deref(), Some("phase.type_code"));
        assert!(err.suggestion.is_some());
    }
}
