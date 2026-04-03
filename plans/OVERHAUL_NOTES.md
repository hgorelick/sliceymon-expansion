# Overhaul Implementation Notes

Captured during planning — must be addressed during implementation.

## Character Selection (Line 11) Ordering

The hero selection screen must show colors in **alphabetical order** (A, B, C, ... X, Y, Z). When we added new colors E/F and J, they appeared at the bottom instead of in order.

**Fix required**: The `rebuild_textmod.js` assembler must sort the character selection entries in line 11 by color letter, not just append new colors at the end. This applies to all three pick rounds (party, add, add).

## Generator Validation Rules

The `generate_hero.js` tool must validate:
1. No bare `.k.` without `.i.POSITION` prefix
2. Spell face IDs valid for their abilitydata template
3. `.n.NAME` is last before `+` or `.part.1`
4. Paren balance = 0
5. All templates use uppercase (Lost, Statue, Thief, Fighter, Guardian, etc.)
