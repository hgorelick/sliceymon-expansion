use proptest::prelude::*;
use textmod_compiler::*;

// ---------------------------------------------------------------------------
// Property: Phase parse-then-emit round-trips
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn prop_phase_message_roundtrip(text in "[a-zA-Z0-9 ]{1,50}") {
        let input = format!("ph.4{}", text);
        let phase = parse_phase(&input).unwrap();
        let emitted = emit_phase(&phase).unwrap();
        prop_assert_eq!(input, emitted);
    }

    #[test]
    fn prop_phase_position_swap_roundtrip(a in 0u8..5, b in 0u8..5) {
        let input = format!("ph.8{}{}", a, b);
        let phase = parse_phase(&input).unwrap();
        let emitted = emit_phase(&phase).unwrap();
        prop_assert_eq!(input, emitted);
    }
}

// ---------------------------------------------------------------------------
// Property: DiceFaces round-trip
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn prop_dice_faces_roundtrip(
        faces in prop::collection::vec(
            prop::strategy::Union::new(vec![
                Just("0".to_string()).boxed(),
                (1u16..200, 1u8..10).prop_map(|(id, pips)| format!("{}-{}", id, pips)).boxed(),
            ]),
            6..=6
        )
    ) {
        let input = faces.join(":");
        let df = ir::DiceFaces::parse(&input);
        let emitted = df.emit();
        let reparsed = ir::DiceFaces::parse(&emitted);
        prop_assert_eq!(df, reparsed);
    }
}

// ---------------------------------------------------------------------------
// Build invariant: emitted chains always have balanced parentheses
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn prop_chain_emitted_balanced_parens(
        entries in prop::collection::vec("[a-zA-Z]{3,10}", 1..5)
    ) {
        use textmod_compiler::ir::*;
        let chain_entries: Vec<ChainEntry> = entries.iter()
            .map(|name| ChainEntry::ItemRef { name: name.clone(), position: None })
            .collect();
        let chain = ModifierChain {
            segments: vec![ChainSegment::Item { sub_entries: chain_entries }],
        };
        let emitted = chain.emit();
        let mut depth: i32 = 0;
        for c in emitted.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
            prop_assert!(depth >= 0, "Paren depth went negative at char in: {}", emitted);
        }
        prop_assert_eq!(depth, 0, "Unbalanced parens in: {}", emitted);
    }
}

// ---------------------------------------------------------------------------
// Property: DiceFaces emit is always ASCII-only
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn prop_dice_faces_emit_ascii(
        faces in prop::collection::vec(
            prop::strategy::Union::new(vec![
                Just("0".to_string()).boxed(),
                (1u16..200, 1u8..10).prop_map(|(id, pips)| format!("{}-{}", id, pips)).boxed(),
            ]),
            6..=6
        )
    ) {
        let input = faces.join(":");
        let df = ir::DiceFaces::parse(&input);
        let emitted = df.emit();
        for ch in emitted.chars() {
            prop_assert!(ch.is_ascii(), "Non-ASCII char in DiceFaces emit: {:?}", ch);
        }
    }
}

// ---------------------------------------------------------------------------
// Property: phase parse never panics on valid-looking input
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn prop_phase_parse_no_panic(code in "[!0-9a-z]", content in "[a-zA-Z0-9 ]{0,30}") {
        let input = format!("ph.{}{}", code, content);
        // Should return Ok or Err, never panic
        let _ = parse_phase(&input);
    }
}

// ---------------------------------------------------------------------------
// Property: reward tag round-trip for simple modifier tags
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn prop_reward_tag_modifier_roundtrip(name in "[a-zA-Z]{1,20}") {
        let input = format!("m({})", name);
        let tag = parse_reward_tag(&input).unwrap();
        let emitted = emit_reward_tag(&tag);
        prop_assert_eq!(input, emitted);
    }

    #[test]
    fn prop_reward_tag_item_roundtrip(name in "[a-zA-Z]{1,20}") {
        let input = format!("i({})", name);
        let tag = parse_reward_tag(&input).unwrap();
        let emitted = emit_reward_tag(&tag);
        prop_assert_eq!(input, emitted);
    }
}
