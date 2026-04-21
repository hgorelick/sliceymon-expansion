//! Emit Phase IR back to textmod string form.
//!
//! Handles recursive nesting for Linked, Boolean, Seq, and LevelEnd phases.
//! Round-trips with `extractor::phase_parser::parse_phase`.

use crate::constants::MAX_PHASE_DEPTH;
use crate::error::CompilerError;
use crate::ir::*;

/// Emit a Phase to its textmod string representation.
pub fn emit_phase(phase: &Phase) -> Result<String, CompilerError> {
    emit_phase_at_depth(phase, 0)
}

fn emit_phase_at_depth(phase: &Phase, depth: usize) -> Result<String, CompilerError> {
    if depth > MAX_PHASE_DEPTH {
        return Err(CompilerError::phase_parse(
            None,
            format!("(emitter depth {})", depth),
            format!("phase nesting depth <= {}", MAX_PHASE_DEPTH),
            format!("depth {}", depth),
        ));
    }

    let mut out = String::new();

    // Emit level scope prefix if present
    if let Some(ref scope) = phase.level_scope {
        out.push_str(&crate::extractor::level_scope_parser::emit_level_scope(scope));
        out.push('.');
    }

    // Emit ph.X prefix
    out.push_str("ph.");
    match phase.phase_type {
        PhaseType::SimpleChoice => out.push('!'),
        PhaseType::PlayerRolling => out.push('0'),
        PhaseType::Targeting => out.push('1'),
        PhaseType::LevelEnd => out.push('2'),
        PhaseType::EnemyRolling => out.push('3'),
        PhaseType::Message => out.push('4'),
        PhaseType::HeroChange => out.push('5'),
        PhaseType::Reset => out.push('6'),
        PhaseType::ItemCombine => out.push('7'),
        PhaseType::PositionSwap => out.push('8'),
        PhaseType::Challenge => out.push('9'),
        PhaseType::Boolean => out.push('b'),
        PhaseType::Choice => out.push('c'),
        PhaseType::Damage => out.push('d'),
        PhaseType::RunEnd => out.push('e'),
        PhaseType::PhaseGenerator => out.push('g'),
        PhaseType::Linked => out.push('l'),
        PhaseType::RandomReveal => out.push('r'),
        PhaseType::Seq => out.push('s'),
        PhaseType::Trade => out.push('t'),
        PhaseType::Boolean2 => out.push('z'),
    }

    // Emit phase-specific content
    match &phase.content {
        PhaseContent::Message { text, button_text } => {
            out.push_str(text.as_str());
            if let Some(btn) = button_text {
                out.push(';');
                out.push_str(btn);
            }
        }
        PhaseContent::SimpleChoice { title, rewards } => {
            if let Some(t) = title {
                out.push_str(t);
                out.push(';');
            }
            let reward_strs: Vec<&str> = rewards.iter().map(|r| r.content.as_str()).collect();
            out.push_str(&reward_strs.join("@3"));
        }
        PhaseContent::Boolean { value_name, threshold, if_true, if_false } => {
            if phase.phase_type == PhaseType::Boolean2 {
                // Boolean2 uses @6/@7 delimiters
                out.push_str(value_name);
                out.push_str("@6");
                out.push_str(&threshold.to_string());
                out.push_str("@6");
                let true_str = emit_phase_at_depth(if_true, depth + 1)?;
                let false_str = emit_phase_at_depth(if_false, depth + 1)?;
                out.push_str(&format!("!m({})", true_str));
                out.push_str("@7");
                out.push_str(&format!("!m({})", false_str));
            } else {
                // Standard Boolean uses ;/@2 delimiters
                out.push_str(value_name);
                out.push(';');
                out.push_str(&threshold.to_string());
                out.push(';');
                let true_str = emit_phase_at_depth(if_true, depth + 1)?;
                let false_str = emit_phase_at_depth(if_false, depth + 1)?;
                out.push_str(&format!("!m({})", true_str));
                out.push_str("@2");
                out.push_str(&format!("!m({})", false_str));
            }
        }
        PhaseContent::Linked { phases } => {
            let strs: Result<Vec<String>, _> = phases
                .iter()
                .map(|p| emit_phase_at_depth(p, depth + 1).map(|s| format!("({})", s)))
                .collect();
            out.push_str(&strs?.join("@1"));
        }
        PhaseContent::Seq { message, options } => {
            out.push_str(message.as_str());
            for opt in options {
                out.push_str("@1");
                out.push_str(&opt.button_text);
                for p in &opt.phases {
                    out.push_str("@2");
                    let s = emit_phase_at_depth(p, depth + 1)?;
                    out.push_str(&format!("!m({})", s));
                }
            }
        }
        PhaseContent::Trade { rewards } => {
            let strs: Vec<&str> = rewards.iter().map(|r| r.content.as_str()).collect();
            out.push_str(&strs.join("@3"));
        }
        PhaseContent::Choice { choice_type, rewards } => {
            match choice_type {
                ChoiceType::PointBuy { budget } => {
                    out.push_str(&format!("PointBuy#{}", budget));
                }
                ChoiceType::Number { count } => {
                    out.push_str(&format!("Number#{}", count));
                }
                ChoiceType::UpToNumber { max } => {
                    out.push_str(&format!("UpToNumber#{}", max));
                }
                ChoiceType::Optional => out.push_str("Optional"),
            }
            if !rewards.is_empty() {
                out.push(';');
                let strs: Vec<&str> = rewards.iter().map(|r| r.content.as_str()).collect();
                out.push_str(&strs.join("@3"));
            }
        }
        PhaseContent::HeroChange { hero_position, change_type } => {
            out.push_str(&hero_position.to_string());
            match change_type {
                HeroChangeType::RandomClass => out.push('0'),
                HeroChangeType::GeneratedHero => out.push('1'),
            }
        }
        PhaseContent::PositionSwap { first, second } => {
            out.push_str(&first.to_string());
            out.push_str(&second.to_string());
        }
        PhaseContent::ItemCombine { combine_type } => {
            out.push_str(combine_type);
        }
        PhaseContent::Challenge { reward, .. } => {
            let strs: Vec<&str> = reward.iter().map(|r| r.content.as_str()).collect();
            out.push_str(&strs.join("@3"));
        }
        PhaseContent::RandomReveal { reward } => {
            out.push_str(&reward.content);
        }
        PhaseContent::PhaseGenerator { gen_type } => match gen_type {
            PhaseGenType::Hero => out.push('h'),
            PhaseGenType::Item => out.push('i'),
        },
        PhaseContent::LevelEnd { phases } => {
            if !phases.is_empty() {
                let strs: Result<Vec<String>, _> = phases
                    .iter()
                    .map(|p| {
                        emit_phase_at_depth(p, depth + 1)
                            .map(|s| format!("!m({})", s))
                    })
                    .collect();
                out.push_str(&strs?.join("@2"));
            }
        }
        PhaseContent::RunEnd
        | PhaseContent::Reset
        | PhaseContent::PlayerRolling
        | PhaseContent::Targeting
        | PhaseContent::EnemyRolling
        | PhaseContent::Damage => {}
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_message_phase() {
        let phase = Phase {
            phase_type: PhaseType::Message,
            level_scope: None,
            content: PhaseContent::Message {
                text: RichText::new("Hello World"),
                button_text: None,
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.4Hello World");
    }

    #[test]
    fn test_emit_message_with_button() {
        let phase = Phase {
            phase_type: PhaseType::Message,
            level_scope: None,
            content: PhaseContent::Message {
                text: RichText::new("Hello"),
                button_text: Some("OK".to_string()),
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.4Hello;OK");
    }

    #[test]
    fn test_emit_run_end() {
        let phase = Phase {
            phase_type: PhaseType::RunEnd,
            level_scope: None,
            content: PhaseContent::RunEnd,
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.e");
    }

    #[test]
    fn test_emit_position_swap() {
        let phase = Phase {
            phase_type: PhaseType::PositionSwap,
            level_scope: None,
            content: PhaseContent::PositionSwap { first: 1, second: 3 },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.813");
    }

    #[test]
    fn test_emit_simple_choice() {
        let phase = Phase {
            phase_type: PhaseType::SimpleChoice,
            level_scope: None,
            content: PhaseContent::SimpleChoice {
                title: None,
                rewards: vec![
                    RewardTag {
                        tag_type: RewardTagType::Modifier,
                        content: "m(skip)".to_string(),
                    },
                    RewardTag {
                        tag_type: RewardTagType::Modifier,
                        content: "m(skip2)".to_string(),
                    },
                ],
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.!m(skip)@3m(skip2)");
    }

    #[test]
    fn test_emit_with_level_scope() {
        let phase = Phase {
            phase_type: PhaseType::Message,
            level_scope: Some(LevelScope {
                start: 5,
                end: None,
                interval: None,
                offset: None,
            }),
            content: PhaseContent::Message {
                text: RichText::new("Hello"),
                button_text: None,
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "5.ph.4Hello");
    }

    #[test]
    fn test_emit_reset() {
        let phase = Phase {
            phase_type: PhaseType::Reset,
            level_scope: None,
            content: PhaseContent::Reset,
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.6");
    }

    #[test]
    fn test_emit_player_rolling() {
        let phase = Phase {
            phase_type: PhaseType::PlayerRolling,
            level_scope: None,
            content: PhaseContent::PlayerRolling,
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.0");
    }

    #[test]
    fn test_emit_targeting() {
        let phase = Phase {
            phase_type: PhaseType::Targeting,
            level_scope: None,
            content: PhaseContent::Targeting,
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.1");
    }

    #[test]
    fn test_emit_enemy_rolling() {
        let phase = Phase {
            phase_type: PhaseType::EnemyRolling,
            level_scope: None,
            content: PhaseContent::EnemyRolling,
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.3");
    }

    #[test]
    fn test_emit_damage() {
        let phase = Phase {
            phase_type: PhaseType::Damage,
            level_scope: None,
            content: PhaseContent::Damage,
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.d");
    }

    #[test]
    fn test_emit_hero_change() {
        let phase = Phase {
            phase_type: PhaseType::HeroChange,
            level_scope: None,
            content: PhaseContent::HeroChange {
                hero_position: 2,
                change_type: HeroChangeType::GeneratedHero,
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.521");
    }

    #[test]
    fn test_emit_item_combine() {
        let phase = Phase {
            phase_type: PhaseType::ItemCombine,
            level_scope: None,
            content: PhaseContent::ItemCombine {
                combine_type: "SecondHighestToTierThrees".to_string(),
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.7SecondHighestToTierThrees");
    }

    #[test]
    fn test_emit_phase_generator_hero() {
        let phase = Phase {
            phase_type: PhaseType::PhaseGenerator,
            level_scope: None,
            content: PhaseContent::PhaseGenerator {
                gen_type: PhaseGenType::Hero,
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.gh");
    }

    #[test]
    fn test_emit_phase_generator_item() {
        let phase = Phase {
            phase_type: PhaseType::PhaseGenerator,
            level_scope: None,
            content: PhaseContent::PhaseGenerator {
                gen_type: PhaseGenType::Item,
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.gi");
    }

    #[test]
    fn test_emit_random_reveal() {
        let phase = Phase {
            phase_type: PhaseType::RandomReveal,
            level_scope: None,
            content: PhaseContent::RandomReveal {
                reward: RewardTag {
                    tag_type: RewardTagType::Modifier,
                    content: "m(Sword)".to_string(),
                },
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.rm(Sword)");
    }

    #[test]
    fn test_emit_trade() {
        let phase = Phase {
            phase_type: PhaseType::Trade,
            level_scope: None,
            content: PhaseContent::Trade {
                rewards: vec![
                    RewardTag {
                        tag_type: RewardTagType::Modifier,
                        content: "m(item1)".to_string(),
                    },
                    RewardTag {
                        tag_type: RewardTagType::Modifier,
                        content: "m(item2)".to_string(),
                    },
                ],
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.tm(item1)@3m(item2)");
    }

    #[test]
    fn test_emit_choice_point_buy() {
        let phase = Phase {
            phase_type: PhaseType::Choice,
            level_scope: None,
            content: PhaseContent::Choice {
                choice_type: ChoiceType::PointBuy { budget: 3 },
                rewards: vec![RewardTag {
                    tag_type: RewardTagType::Modifier,
                    content: "m(thing)".to_string(),
                }],
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.cPointBuy#3;m(thing)");
    }

    #[test]
    fn test_emit_choice_optional_no_rewards() {
        let phase = Phase {
            phase_type: PhaseType::Choice,
            level_scope: None,
            content: PhaseContent::Choice {
                choice_type: ChoiceType::Optional,
                rewards: vec![],
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.cOptional");
    }

    #[test]
    fn test_emit_simple_choice_with_title() {
        let phase = Phase {
            phase_type: PhaseType::SimpleChoice,
            level_scope: None,
            content: PhaseContent::SimpleChoice {
                title: Some("Pick one".to_string()),
                rewards: vec![RewardTag {
                    tag_type: RewardTagType::Modifier,
                    content: "m(A)".to_string(),
                }],
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.!Pick one;m(A)");
    }

    #[test]
    fn test_emit_challenge() {
        let phase = Phase {
            phase_type: PhaseType::Challenge,
            level_scope: None,
            content: PhaseContent::Challenge {
                reward: vec![RewardTag {
                    tag_type: RewardTagType::Modifier,
                    content: "m(reward)".to_string(),
                }],
                extra_monsters: vec![],
            },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.9m(reward)");
    }

    #[test]
    fn test_emit_linked_phases() {
        let phase = Phase {
            phase_type: PhaseType::Linked,
            level_scope: None,
            content: PhaseContent::Linked {
                phases: vec![
                    Phase {
                        phase_type: PhaseType::Message,
                        level_scope: None,
                        content: PhaseContent::Message {
                            text: RichText::new("First"),
                            button_text: None,
                        },
                    },
                    Phase {
                        phase_type: PhaseType::RunEnd,
                        level_scope: None,
                        content: PhaseContent::RunEnd,
                    },
                ],
            },
        };
        assert_eq!(
            emit_phase(&phase).unwrap(),
            "ph.l(ph.4First)@1(ph.e)"
        );
    }

    #[test]
    fn test_emit_boolean_phase() {
        let phase = Phase {
            phase_type: PhaseType::Boolean,
            level_scope: None,
            content: PhaseContent::Boolean {
                value_name: "score".to_string(),
                threshold: 5,
                if_true: Box::new(Phase {
                    phase_type: PhaseType::Message,
                    level_scope: None,
                    content: PhaseContent::Message {
                        text: RichText::new("You win"),
                        button_text: None,
                    },
                }),
                if_false: Box::new(Phase {
                    phase_type: PhaseType::Message,
                    level_scope: None,
                    content: PhaseContent::Message {
                        text: RichText::new("Try again"),
                        button_text: None,
                    },
                }),
            },
        };
        assert_eq!(
            emit_phase(&phase).unwrap(),
            "ph.bscore;5;!m(ph.4You win)@2!m(ph.4Try again)"
        );
    }

    #[test]
    fn test_emit_boolean2_phase() {
        let phase = Phase {
            phase_type: PhaseType::Boolean2,
            level_scope: None,
            content: PhaseContent::Boolean {
                value_name: "hp".to_string(),
                threshold: 10,
                if_true: Box::new(Phase {
                    phase_type: PhaseType::Message,
                    level_scope: None,
                    content: PhaseContent::Message {
                        text: RichText::new("Alive"),
                        button_text: None,
                    },
                }),
                if_false: Box::new(Phase {
                    phase_type: PhaseType::Message,
                    level_scope: None,
                    content: PhaseContent::Message {
                        text: RichText::new("Dead"),
                        button_text: None,
                    },
                }),
            },
        };
        assert_eq!(
            emit_phase(&phase).unwrap(),
            "ph.zhp@610@6!m(ph.4Alive)@7!m(ph.4Dead)"
        );
    }

    #[test]
    fn test_emit_seq_phase() {
        let phase = Phase {
            phase_type: PhaseType::Seq,
            level_scope: None,
            content: PhaseContent::Seq {
                message: RichText::new("Choose"),
                options: vec![
                    SeqOption {
                        button_text: "Option A".to_string(),
                        phases: vec![Phase {
                            phase_type: PhaseType::Message,
                            level_scope: None,
                            content: PhaseContent::Message {
                                text: RichText::new("You chose A"),
                                button_text: None,
                            },
                        }],
                    },
                ],
            },
        };
        assert_eq!(
            emit_phase(&phase).unwrap(),
            "ph.sChoose@1Option A@2!m(ph.4You chose A)"
        );
    }

    #[test]
    fn test_emit_level_end_with_sub_phases() {
        let phase = Phase {
            phase_type: PhaseType::LevelEnd,
            level_scope: None,
            content: PhaseContent::LevelEnd {
                phases: vec![
                    Phase {
                        phase_type: PhaseType::Message,
                        level_scope: None,
                        content: PhaseContent::Message {
                            text: RichText::new("Level complete"),
                            button_text: None,
                        },
                    },
                    Phase {
                        phase_type: PhaseType::Reset,
                        level_scope: None,
                        content: PhaseContent::Reset,
                    },
                ],
            },
        };
        assert_eq!(
            emit_phase(&phase).unwrap(),
            "ph.2!m(ph.4Level complete)@2!m(ph.6)"
        );
    }

    #[test]
    fn test_emit_level_end_empty() {
        let phase = Phase {
            phase_type: PhaseType::LevelEnd,
            level_scope: None,
            content: PhaseContent::LevelEnd { phases: vec![] },
        };
        assert_eq!(emit_phase(&phase).unwrap(), "ph.2");
    }

    #[test]
    fn test_emit_with_range_level_scope() {
        let phase = Phase {
            phase_type: PhaseType::Reset,
            level_scope: Some(LevelScope {
                start: 3,
                end: Some(7),
                interval: None,
                offset: None,
            }),
            content: PhaseContent::Reset,
        };
        assert_eq!(emit_phase(&phase).unwrap(), "3-7.ph.6");
    }

    #[test]
    fn test_emit_with_interval_level_scope() {
        let phase = Phase {
            phase_type: PhaseType::Damage,
            level_scope: Some(LevelScope {
                start: 0,
                end: None,
                interval: Some(2),
                offset: None,
            }),
            content: PhaseContent::Damage,
        };
        assert_eq!(emit_phase(&phase).unwrap(), "e2.ph.d");
    }

    #[test]
    fn test_emit_with_interval_offset_level_scope() {
        let phase = Phase {
            phase_type: PhaseType::PlayerRolling,
            level_scope: Some(LevelScope {
                start: 0,
                end: None,
                interval: Some(3),
                offset: Some(1),
            }),
            content: PhaseContent::PlayerRolling,
        };
        assert_eq!(emit_phase(&phase).unwrap(), "e3.1.ph.0");
    }

    #[test]
    fn test_emit_depth_overflow_errors() {
        // Construct a phase at the surface and call emit_phase_at_depth
        // with depth > MAX_PHASE_DEPTH
        let phase = Phase {
            phase_type: PhaseType::Message,
            level_scope: None,
            content: PhaseContent::Message {
                text: RichText::new("deep"),
                button_text: None,
            },
        };
        let result = emit_phase_at_depth(&phase, MAX_PHASE_DEPTH + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip_message() {
        let input = "ph.4Hello World";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_message_with_button() {
        let input = "ph.4Hello;OK";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_run_end() {
        let input = "ph.e";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_position_swap() {
        let input = "ph.813";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_with_level_scope() {
        let input = "5.ph.4Hello";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_boolean() {
        let input = "ph.bscore;5;!m(ph.4You win)@2!m(ph.4Try again)";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_linked() {
        let input = "ph.l(ph.4First)@1(ph.e)";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_trade() {
        let input = "ph.tm(item1)@3m(item2)";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_item_combine() {
        let input = "ph.7SecondHighestToTierThrees";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_phase_generator() {
        let input = "ph.gh";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_simple_choice() {
        let input = "ph.!m(skip)@3m(skip2)";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_hero_change() {
        let input = "ph.521";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }

    #[test]
    fn test_roundtrip_seq() {
        let input = "ph.sChoose@1Option A@2!m(ph.4You chose A)";
        let parsed = crate::extractor::phase_parser::parse_phase(input).unwrap();
        let emitted = emit_phase(&parsed).unwrap();
        assert_eq!(emitted, input);
    }
}
