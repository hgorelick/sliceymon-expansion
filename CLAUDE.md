# Sliceymon+ Expansion — Claude Code Configuration

> **Project**: Sliceymon+ — Pokemon-themed textmod expansion for Slice & Dice
> **Stack**: JavaScript (Node.js) tooling, plain text mod format, canvas library for sprites
> **Game**: Slice & Dice (mobile roguelike deck-builder by tann)
> **Mod Base**: Original Sliceymon mod (415KB, 153 lines) — expanded with ~100 new Pokemon

## General Principles

- When implementing plans, start promptly. Do not spend excessive time re-reading plans that were already created in prior sessions. If a plan document exists in `plans/`, read it and begin executing.
- When in plan mode, ALWAYS read and follow `personas/ai-development.md` — use its chunked plan template, parallel execution map, checkpoint protocol, and one-shot design principles to structure the plan.
- **This is a textmod, not a traditional codebase**: The primary output is `textmod_expanded.txt` — a single enormous text file pasted into a mobile game. Every change must produce valid, pasteable output. There is no compiler — the game is the only validator.
- **Preserve what works**: The original Sliceymon mod is battle-tested. When modifying existing lines, change only what's needed. Don't reformat, reorder properties, or "clean up" working lines.

## Communication Style

- Don't say "you're right" or similar validation statements unless you've actually performed analysis
- When presenting options, clearly state which you recommend and why
- Be warm and collaborative while remaining precise
- **Try your hardest to complete assigned work**
- **CRITICAL**: Clearly articulate in summaries any:
  - Incomplete work
  - Deviations from assigned work
  - Simplifications made
  - Assumptions made

## Correctness Over Convenience (Non-Negotiable)

Do NOT simplify, weaken, or replace a design, algorithm, or safety mechanism for reasons of convenience, speed, perceived complexity, or effort.

**"Too complicated", "too much overhead", and "good enough" are invalid justifications.**

If a hero design encodes a real game balance invariant (damage curve, tier progression, keyword budget), that invariant MUST be preserved or strengthened.

### High-Risk Areas (Stricter Scrutiny)
- Parenthesis/bracket balance in hero lines (game silently rejects malformed lines)
- Tier separator structure (`+` at depth 0, never nested)
- Property order within replica blocks
- Face ID assignments (wrong ID = wrong game mechanic)
- `.part.1` appending for captures and new content (must not overwrite existing lines)

## Critical Rules (Never Violate)

1. **Format-first development**: ALWAYS read working original lines in `textmod.txt` before writing new hero/monster/boss lines
2. **Validate after every change**: Run `node tools/validate_textmod.js [file]` after every modification
3. **Design compliance**: Hero dice MUST match designs in `plans/hero_designs_batch*.md` exactly (Face IDs, pips, HP, keywords)
4. **No hallucinated Face IDs**: Use ONLY Face IDs documented in `SLICEYMON_AUDIT.md` and `plans/EXPANSION_PLAN.md`
5. **Sprite compliance**: Use ONLY sprite encodings from `tools/sprite_encodings.json` — never fabricate `.img.` data
6. **Parenthesis discipline**: Every hero line must have balanced parentheses with tier separators (`+`) at depth 0
7. **No duplicate capturables**: A Pokemon cannot appear as both a hero AND a capture item. Check `plans/FULL_ROSTER.md` for assignments.
8. **Dice = FaceID + Pips**: Faces are encoded as `FaceID-Pips` in `.sd.` — never use move names, keywords, or descriptions in the `.sd.` field
9. **Modify existing mod**: All changes build on top of `textmod.txt` — never start from scratch
10. **User picks Pokemon**: Henry chooses which Pokemon to include. Don't suggest additions without being asked.
11. **Fix pre-existing issues (ZERO TOLERANCE)**: If you encounter ANY format error, broken parentheses, wrong Face ID, or structural issue during your work — **STOP and fix it before continuing**. "It was already broken" is NEVER an excuse.

## Self-Review (After Writing Mod Content)

After writing ANY textmod content, review it against these criteria. Fix issues before presenting to user.

### Self-Review Checklist

**Format Check:**
- [ ] Parentheses balanced (depth never negative, 0 at end of line)
- [ ] Tier separators (`+`) at depth 0 (never nested inside parens)
- [ ] `.n.NAME` is LAST before `+` or line end (outside replica parens)
- [ ] `.part.1&hidden` on LAST T3 variant before `.mn.`
- [ ] `.mn.[Name]@2!m(skip&hidden&temporary),` suffix format correct
- [ ] No non-ASCII characters (game rejects em-dashes, smart quotes, etc.)

**Content Check:**
- [ ] All Face IDs exist in `SLICEYMON_AUDIT.md` or expansion plan
- [ ] HP values match design doc for each tier
- [ ] Tier progression follows power budget (T1: 2-3 blanks, T2: 1-2, T3: 0-1)
- [ ] Sprite `.img.` data matches `tools/sprite_encodings.json`
- [ ] Pokemon role matches its design (type, abilities, competitive identity)

**Structural Check:**
- [ ] No duplicate Pokemon across hero/capture/monster/boss pools
- [ ] New content uses `.part.1` appending (not overwriting existing lines)
- [ ] Color assignments match `plans/FULL_ROSTER.md`
- [ ] Line numbers don't collide with existing mod lines

**Balance Check:**
- [ ] Keywords budgeted appropriately (Cantrip is premium, DoubleUse is multiplicative)
- [ ] Damage/shield/heal pips within tier budget
- [ ] Blank faces appropriate for tier
- [ ] No keyword on monsters that requires rerolls (monsters don't reroll)

## Source of Truth Files

| Purpose | File | When to Read |
|---------|------|--------------|
| Original mod (baseline) | `textmod.txt` | Before modifying any line |
| Full expansion (current build) | `textmod_expanded.txt` | Before assembling or testing |
| Heroes-only test build | `textmod_heroes_only.txt` | When debugging paste failures |
| Mod structure audit | `SLICEYMON_AUDIT.md` | Face IDs, property codes, line structure |
| Expansion plan (v3) | `plans/EXPANSION_PLAN.md` | All design work |
| Full Pokemon roster | `plans/FULL_ROSTER.md` | Checking hero/capture/monster/boss assignments |
| Hero designs batch 1 | `plans/hero_designs_batch1.md` | Charmander, Cyndaquil, Torchic, Bagon, Dratini, Beldum, Machop |
| Hero designs batch 2 | `plans/hero_designs_batch2.md` | Treecko, Chikorita, Bulbasaur, Mudkip, Totodile, Poliwag, Wailmer |
| Hero designs batch 3 | `plans/hero_designs_batch3.md` | Pikachu, Weedle, Riolu, Togepi, Cleffa, Nidorans, Tyranitar |
| Monster + boss designs | `plans/monster_boss_designs.md` | All enemy design work |
| Sprite encodings | `tools/sprite_encodings.json` | When writing `.img.` properties |
| Debugging state | `HANDOFF.md` | Before resuming debugging work |
| Game design principles | `personas/slice-and-dice-design.md` | Balance reviews, new designs |
| AI workflow principles | `personas/ai-development.md` | Plan creation, task structuring |

## Before Writing Mod Content

### New Hero Lines
```
1. Read the target line in textmod.txt (the original being replaced)
2. Read the hero design in plans/hero_designs_batch*.md
3. Read SLICEYMON_AUDIT.md for Face IDs and property format
4. Read a working generated line in generated/ for the pattern
5. Look up sprite in tools/sprite_encodings.json
6. Generate the line following the exact format of working originals
7. Validate: node tools/validate_textmod.js [output file]
```

### Captures / Monsters / Bosses
```
1. Read the relevant section of plans/EXPANSION_PLAN.md
2. Read plans/monster_boss_designs.md for enemy designs
3. Read the corresponding section in textmod.txt for format reference
4. Use .part.1 appending for new content (never overwrite)
5. Check plans/FULL_ROSTER.md to avoid duplicate Pokemon assignments
6. Validate after changes
```

### Modifying Existing Lines
```
1. Read the ORIGINAL line in textmod.txt
2. Read the CURRENT line in textmod_expanded.txt (if already modified)
3. Make minimal targeted changes
4. Validate after changes
```

## Textmod Format Reference

### Hero Line Structure
```
hidden&temporary&ph.b[name];1;!mheropool.(TIER1)+(TIER2A)+(TIER2B)+(TIER3A)+(TIER3B).part.1&hidden.mn.[Name]@2!m(skip&hidden&temporary),
```

### Tier Block Structure
```
(replica.[Template].col.[color].tier.[N].hp.[N].sd.[FaceID-Pips:FaceID-Pips:...].img.[sprite]).speech.[cries].n.[Name]
```

### Critical Format Rules
- `.n.NAME` must be LAST before `+` or line end (outside replica parens)
- `.sd.` contains 6 faces as `FaceID-Pips` separated by colons (e.g., `34-1:30-1:0:0:30-1:0`)
- `.img.` for hero sprites is a long base64 string (>50 chars)
- `.img.` for spell icons is a short string (<30 chars, like `.img.spark`)
- Tier separators (`+`) must be at depth 0 (never inside parentheses)
- Only add `.abilitydata.` if the original hero template had it
- T1 often has explicit `.tier.1`, T2 often omits `.tier.`, T3 has explicit `.tier.3`

### Property Codes
| Code | Purpose | Example |
|------|---------|---------|
| `.sd.` | Dice faces (FaceID-Pips) | `.sd.34-1:30-1:0:0:30-1:0` |
| `.hp.` | Hit points | `.hp.6` |
| `.col.` | Color slot | `.col.a` |
| `.tier.` | Evolution tier | `.tier.3` |
| `.img.` | Sprite encoding | `.img.[base64...]` |
| `.speech.` | Battle cries | `.speech.Char!~Char!!` |
| `.n.` | Display name | `.n.Charmander` |
| `.mn.` | Menu/original name | `.mn.Fomantis` |
| `.abilitydata.` | Spell definition | `.abilitydata.(Fey.sd.[faces].img.[icon].n.[SpellName])` |
| `.facade.` | Alt appearance | `.facade.[data]` |
| `.triggerhpdata.` | HP trigger effects | `.triggerhpdata.[data]` |

## Commands

```bash
# Validation (run after EVERY change)
node tools/validate_textmod.js textmod_heroes_only.txt
node tools/validate_textmod.js textmod_expanded.txt

# Build pipeline
node tools/assemble_textmod.js          # Assemble full textmod from parts
node tools/rebuild_textmod.js           # Rebuild with replacements + fixes

# Sprite tools
node tools/encode_sprite.js [image.png] # Single PNG → sprite encoding
node tools/batch_sprites.js             # Download + encode all 121 sprites
node tools/rebuild_sprites.js           # Update sprites in generated files

# Hero generation
node tools/generate_hero.js [config]    # Generate a hero line from config

# Fix tools
node tools/fix_brackets.js              # Fix unbalanced parentheses
node tools/fix_hero_parens.js           # Fix tier separator nesting
```

## Project Patterns

### Adding a New Hero (One-Shot Pattern)
```
1. Create config in tools/hero_configs/[pokemon].json
2. Generate: node tools/generate_hero.js tools/hero_configs/[pokemon].json
3. Output goes to generated/line_[N]_[pokemon].txt
4. Assemble: node tools/assemble_textmod.js
5. Validate: node tools/validate_textmod.js textmod_expanded.txt
6. User pastes into game to test
```

### Debugging Paste Failures
```
1. Run validator to catch format issues
2. Compare failing line against working original (same line number in textmod.txt)
3. Check: property order, parenthesis depth, tier separators, missing fields
4. If unclear, binary search: add heroes one at a time to isolate failures
5. Game is on mobile — user pastes from VSCode
```

### Safe Modification Pattern
```
1. Read original line from textmod.txt
2. Make targeted change
3. Validate
4. Test in game (user pastes)
```

## Hallucination Prevention

### Common AI Mistakes to Catch
| Mistake | Prevention |
|---------|------------|
| Invented Face IDs | Verify ALL Face IDs against SLICEYMON_AUDIT.md |
| Wrong Face ID for mechanic | Face IDs map to specific game mechanics — check the audit |
| Fabricated sprite data | ONLY use encodings from tools/sprite_encodings.json |
| Keyword on monster dice | Monsters don't reroll — Cantrip is meaningless on them |
| Move names in .sd. field | .sd. is FaceID-Pips ONLY, never move names |
| Duplicate Pokemon assignment | Check FULL_ROSTER.md — no Pokemon in two pools |
| Non-ASCII characters | Game rejects em-dashes, smart quotes, etc. — ASCII only |
| Nested tier separators | `+` must be at paren depth 0 |
| Missing .n. at end | .n.NAME must be last property before `+` or line end |
| Wrong tier blank count | T1: 2-3 blanks, T2: 1-2 blanks, T3: 0-1 blanks |

### Verification Checklist
Before completing any mod content task:
- [ ] All Face IDs verified against audit/plan docs
- [ ] Parentheses balanced, tier separators at depth 0
- [ ] Sprites from sprite_encodings.json
- [ ] No duplicate Pokemon across pools
- [ ] Validator passes: `node tools/validate_textmod.js [file]`
- [ ] No non-ASCII characters
- [ ] Design matches hero_designs_batch*.md exactly

## Task Format (For Complex Work)

```markdown
## Task: [Action] [Component]

### Files to Read First
- textmod.txt (original baseline)
- [relevant design doc]
- SLICEYMON_AUDIT.md (Face IDs, format)

### Requirements
- [Specific requirement 1]
- [Specific requirement 2]

### Constraints
- Use ONLY: [Face IDs, sprites, patterns from source files]
- Do NOT: [anti-patterns to avoid]

### Verification
- [ ] Validator passes
- [ ] Format matches working originals
- [ ] Design matches plan docs
```

## Git Conventions

- Branch from `main` for new features
- Commit message format: `type: description` (feat, fix, refactor, docs)
- **NEVER add `Co-Authored-By` lines to commits** — no AI attribution in commit messages
- Don't push while user is using work git account (ask first)
- User's GitHub: hgorelick (NOT hgorelick-scala which is work)

## Session Handoff (Mandatory for Long Sessions)

After completing each major task, update `HANDOFF.md` with current session state. This ensures continuity if context compression kicks in or the session ends unexpectedly.

### Handoff File Contents
```markdown
# Session Handoff — [date]

## Current Task
What you're working on right now

## Completed
- What's been done this session (with commit hashes if applicable)

## In Progress
- Partially completed work and its current state

## Next Steps
- What should happen next, in order

## Key Decisions
- Important choices made and why

## Blockers / Open Questions
- Anything unresolved
```

### When to Write
- After completing a plan chunk or major task
- Before running `/compact`
- When a task spans many files or steps
- Any time the conversation feels long

## When Stuck

1. **Format issue?** → Read a working original line from `textmod.txt` and compare character by character
2. **Face ID unknown?** → Check `SLICEYMON_AUDIT.md` Face ID tables
3. **Balance unclear?** → Read `personas/slice-and-dice-design.md` tier budget tables
4. **Design question?** → Check `plans/hero_designs_batch*.md` or `plans/EXPANSION_PLAN.md`
5. **Pokemon assignment conflict?** → Check `plans/FULL_ROSTER.md`
6. **Paste failure?** → Run validator, then binary search to isolate failing lines
7. **Sprite missing?** → Run `node tools/encode_sprite.js` or check `tools/sprite_encodings.json`
8. **Non-ASCII sneaking in?** → Search for characters outside 0x20-0x7E range
9. **Paren imbalance?** → Run `node tools/fix_brackets.js` or `node tools/fix_hero_parens.js`
10. **What's the current state?** → Read `HANDOFF.md` for latest session handoff

## Personas (Reference for Specialized Tasks)

| Task Type | Persona File |
|-----------|--------------|
| Game balance, dice design, hero/monster/boss mechanics | `personas/slice-and-dice-design.md` |
| AI workflow, plan structure, one-shot task design | `personas/ai-development.md` |
