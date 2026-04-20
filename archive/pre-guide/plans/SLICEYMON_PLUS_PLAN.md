# Implementation Plan: Sliceymon+ Mod

## Overview

Build the complete Sliceymon+ textmod from the original Sliceymon base using the Rust textmod compiler for extraction, patching, validation, and assembly. The mod adds 22 new heroes (replacing 18 originals + 2 new color slots), redesigns Larvitar, renames Aggron monster to Probopass, overhauls captures (removals, upgrades, replacements, additions), adds 6 new legendaries, and implements 16 new boss fights across Gen 3/4/5 paths.

### Source of Truth

| Concern | Document |
|---------|----------|
| Pokemon assignments | `plans/FULL_ROSTER.md` |
| Hero dice designs | `plans/hero_designs_batch{1,2,3}.md`, `plans/OVERHAUL_PLAN.md` Section 1A (Snorunt, Aron) |
| Spells | `plans/OVERHAUL_PLAN.md` DEFINITIVE SPELL REFERENCE |
| Templates | `plans/TEMPLATE_PROPERTIES.md` |
| Capture/Legendary dice | `plans/OVERHAUL_PLAN.md` Sections 3C-1, 3D-1 |
| Monster dice | `plans/monster_boss_designs.md` Part 1 |
| Boss fights | `plans/monster_boss_designs.md` Parts 2-5, 7 |
| Legendary dog items | `plans/monster_boss_designs.md` Part 8 |
| Generated hero files | `generated/*.txt` (26 files) |
| Original mod baseline | `textmod.txt` |
| Compiler | `compiler/` — `textmod-compiler extract|build|validate|patch` |

### Tools

| Tool | Purpose | When |
|------|---------|------|
| `textmod-compiler validate <file>` | Structural + cross-reference validation | After every modification |
| `textmod-compiler extract <file> --output <dir>` | Extract to IR (JSON) | Inspecting mod structure |
| `textmod-compiler build <dir> --output <file>` | Build mod from IR | Round-trip testing |
| `textmod-compiler patch <base> -H <heroes_dir> --output <file>` | Patch heroes into base mod | Hero assembly |
| `node tools/validate_textmod.js <file>` | Legacy JS validation (backup) | Cross-checking |
| `node tools/generate_hero.js <config>` | Generate hero line from JSON config | New hero creation |

---

## Checkpoint Configuration

- Total chunks: 10
- Checkpoint frequency: After each chunk
- Critical checkpoints: After Chunk 3 (hero assembly — first pasteable mod), Chunk 6 (captures — second paste test), Chunk 10 (final integration)
- Gate checkpoints: Chunk 3 requires user paste test before proceeding

---

## Parallel Execution Map

```
Chunk 1: Hero File Fixes (spells + templates)
├── Chunk 2: Aron + Lillipup Compound Modifier Surgery (depends on 1)
├── Chunk 3: Hero Assembly + Validation [GATE — user paste test]
│   ├── Chunk 4: Monster Changes (depends on 3)
│   ├── Chunk 5: Capture Changes (depends on 3)
│   │   └── Chunk 6: Legendary Additions (depends on 5) [USER PASTE TEST]
│   └── Chunk 7: Boss Fights — Gen 3 (depends on 3)
│       ├── Chunk 8: Boss Fights — Gen 4 (depends on 7)
│       └── Chunk 9: Boss Fights — Gen 5 (depends on 7)
│           └── Chunk 10: Final Integration Test [GATE]
```

Parallel groups:
- **After Chunk 3**: Chunks 4, 5, 7 can run in parallel
- **After Chunk 5**: Chunk 6 (legendaries depend on capture line structure)
- **After Chunk 7**: Chunks 8, 9 can run in parallel

Minimum wall-clock rounds: 7 (1→2→3→{4,5,7}→{6,8,9}→10) vs 10 sequential

---

## Chunk 1: Hero File Fixes (Spells + Templates)

**Scope**: Apply spell corrections and template migrations to all 26 generated hero files. These are mechanical find-replace operations on existing files.

**Dependencies**: None

**Concern**: Spell face ID validation + template string replacement

**Files** (modify — all in `generated/`):

Spell fixes (from DEFINITIVE SPELL REFERENCE in OVERHAUL_PLAN.md):
1. `torchic.txt` — Blaze Kick spell fix
2. `dratini.txt` — Dragon Dance + Outrage spell fixes
3. `beldum.txt` — Meteor Mash spell fix
4. `treecko.txt` — Leaf Blade spell ADDITION
5. `chikorita.txt` — Aromatherapy mana cost fix
6. `bulbasaur.txt` — Solar Beam spell ADDITION
7. `cleffa.txt` — Metronome spell fix (face 27→136)
8. `pikachu.txt` — Thunderbolt + Thunder spell fixes
9. `nidoranf.txt` — Earth Power spell fix
10. `riolu.txt` — Aura Sphere + Close Combat spell fixes
11. `togepi.txt` — Wish rebalance + Dazzling Gleam keyword
12. `cyndaquil.txt` — Eruption add singlecast
13. `poliwag.txt` — Rain Dance complete redesign

Template migrations (after spell fixes, same files + more):
14. `mudkip.txt` — Statue→Guardian
15. `bulbasaur.txt` — Statue→Guardian
16. `wailmer.txt` — Lost→Stalwart
17. `togepi.txt` — Statue→Dancer
18. `cleffa.txt` — Lost→Fencer
19. `beldum.txt` — Lost→Alloy
20. `machop.txt` — Lost→Fighter
21. `totodile.txt` — Lost→Fighter
22. `poliwag.txt` — Statue→Fighter
23. `torchic.txt` — Lost→Thief
24. `pikachu.txt` — Lost→Thief
25. `treecko.txt` — Lost→Primrose
26. `chikorita.txt` — Lost→Healer

**Files** (read):
- `plans/OVERHAUL_PLAN.md` — DEFINITIVE SPELL REFERENCE (search for that heading)
- `plans/TEMPLATE_PROPERTIES.md` — template defaults to check for conflicts

**Requirements**:
- Read the DEFINITIVE SPELL REFERENCE table and apply exact abilitydata strings
- For template migrations, replace ALL occurrences of old `replica.TEMPLATE` with new (typically 5 per file — one per tier)
- Do NOT modify Snorunt (`snorunt.txt`) or Aron (`aron.txt`) — they are handled separately
- Do NOT change any property other than abilitydata and replica.TEMPLATE
- After all changes, validate every modified file with `textmod-compiler validate`

**Verification**:
- [ ] Every spell in every file matches DEFINITIVE SPELL REFERENCE exactly
- [ ] Every file uses its correct template per FULL_ROSTER.md
- [ ] No `replica.Lost` or `replica.Statue` remains in files that should have been migrated
- [ ] `grep 'replica\.' generated/*.txt` shows only correct templates
- [ ] Paren balance = 0 on every file

---

## Chunk 2: Aron + Lillipup Compound Modifier Surgery

**Scope**: Prepare Aron hero data and perform surgery on the Lillipup compound modifier to replace Lillipup with Aron while preserving the Pickup mechanic, item data, and Caterpie/Butterfree capture. Remove Arceus capture from this modifier (Arceus is now a boss).

**Dependencies**: Chunk 1 (all hero files must be finalized before assembly)

**Concern**: Compound modifier editing + Pickup mechanic preservation

**Files** (read):
1. `textmod.txt` — the Lillipup compound modifier (search for `.mn.Lillipup` — contains hero data + items + Arceus capture + Caterpie capture + Pickup system)
2. `generated/aron.txt` — Aron hero data
3. `plans/OVERHAUL_PLAN.md` Section 1A hero #4 (Aron specs)

**Files** (modify):
4. `generated/aron.txt` — ensure template is `replica.Stalwart`, verify Aron specs match plan

**Files** (create):
5. A working replacement for the Lillipup modifier content (hero portion only, to be used by the patch command)

**Requirements**:
- Use `textmod-compiler extract textmod.txt` to get the IR, then inspect the Lillipup hero entry to understand the compound modifier structure
- Identify boundaries: Lillipup hero data, Pickup modifier, item pool, Arceus capture, Caterpie capture
- Build Aron replacement that:
  - Replaces Lillipup hero data with Aron hero data
  - Preserves the Pickup item/modifier system (Lillipup's signature mechanic → Aron inherits it)
  - Removes Arceus capture entry
  - Preserves Caterpie/Butterfree capture entry
  - Preserves all item data
- Validate the reconstructed modifier has balanced parens and correct structure

**Verification**:
- [ ] Aron hero data present with `replica.Stalwart` template
- [ ] `.mn.Aron@2` present, `.mn.Lillipup` absent
- [ ] Pickup modifier preserved (`.mn.Pick Up` still present)
- [ ] Arceus capture removed from this modifier
- [ ] Caterpie/Butterfree capture preserved
- [ ] All item data preserved
- [ ] Paren balance = 0 on reconstructed modifier
- [ ] `textmod-compiler validate` passes

---

## Chunk 3: Hero Assembly + Validation [GATE CHECKPOINT]

**Scope**: Assemble all 25 hero colors into a working textmod using the compiler's `patch` command. Update character selection modifier for alphabetical color ordering. This produces the first pasteable test mod.

**Dependencies**: Chunks 1, 2

**Concern**: Assembly + character selection + first paste test

**Files** (read):
1. `textmod.txt` — base mod
2. All 26 files in `generated/` — hero replacement data
3. `plans/FULL_ROSTER.md` — hero roster verification

**Files** (create):
4. `textmod_heroes_only.txt` — assembled heroes-only textmod

**Requirements**:
- Use `textmod-compiler patch textmod.txt -H generated/ --output textmod_heroes_only.txt` to patch all heroes into the base mod
- Verify the patch command correctly matches each generated file to its target line by color
- If the patch command doesn't handle all cases (new color slots E/J, Lillipup compound modifier), supplement with manual assembly
- Fix character selection modifier (search for `.mn.Start` or the pick-phase modifier): sort hero colors alphabetically across all pick rounds
- Remove any remaining traces of replaced heroes (Ditto config if present, old hero names in structural modifiers)
- Run `textmod-compiler validate textmod_heroes_only.txt` — must pass with 0 errors
- Run `textmod-compiler validate textmod_heroes_only.txt --round-trip` — verify IR stability

**Verification**:
- [ ] All 25 colors present in character selection modifier
- [ ] Character selection is alphabetically sorted by color
- [ ] All 46 hero lines present (verify by grepping `.mn.` names against FULL_ROSTER)
- [ ] No removed hero names remain (Vanillite, Fomantis, Rockruff, Applin, Darumaka, Agumon, Varoom, Lillipup, Espurr, Sunkern, Roggenrola, Trubbish, Slugma, Burmy, Tinkatink, Joltik, Tentomon, Ditto)
- [ ] `textmod-compiler validate` = 0 errors
- [ ] Round-trip test passes
- [ ] **USER PASTE TEST**: User pastes mod into game, reports modifier acceptance count

**GATE**: Do not proceed to Chunks 4+ until user confirms paste test passes (0 failed modifiers, heroes appear in draft, basic gameplay works).

---

## Chunk 4: Monster Changes

**Scope**: Rename Aggron monster to Probopass on monster pool lines. Add new monster designs from `monster_boss_designs.md`.

**Dependencies**: Chunk 3 (need assembled textmod)

**Parallel with**: Chunks 5, 7

**Concern**: Monster pool modification

**Files** (modify):
1. `textmod_heroes_only.txt` → rename to `textmod_plus.txt` (or work on copy) — monster pool lines

**Files** (read):
2. `plans/monster_boss_designs.md` Part 1 — monster dice designs
3. `tools/sprite_encodings.json` — Probopass sprite + new monster sprites

**Requirements**:
- Search for `.n.Aggron` in monster pool sections (NOT hero section) — replace with `.n.Probopass`
- Replace Aggron sprite with Probopass sprite from `sprite_encodings.json`
- Add new monster designs from `monster_boss_designs.md` Part 1 using `.part.1` appending (do NOT overwrite existing monster lines)
- Download/encode sprites for any new monsters missing from `sprite_encodings.json`
- Validate after changes

**Verification**:
- [ ] `.n.Probopass` appears on monster pool lines
- [ ] `.n.Aggron` appears ONLY in hero context (Aron's evolution), never in monster context
- [ ] New monster entries have valid `.sd.` and `.img.` data
- [ ] `textmod-compiler validate` passes

---

## Chunk 5: Capture Changes

**Scope**: Apply all capture removals, upgrades, replacements, and additions per OVERHAUL_PLAN.md Part 3 and FULL_ROSTER.md.

**Dependencies**: Chunk 3

**Parallel with**: Chunks 4, 7

**Concern**: Capture pool modification

**Files** (modify):
1. Working textmod — capture pool section (search for existing capture names like `.n.Snorlax` or ball item markers to find the capture modifier)

**Files** (read):
2. `plans/OVERHAUL_PLAN.md` Sections 3A-3C-1 — capture changes + dice designs
3. `plans/FULL_ROSTER.md` Captures section
4. `tools/sprite_encodings.json` — new Pokemon sprites

**Requirements**:
- **Remove**: Pikachu, Charizard, Metagross, Poliwag, Ivysaur, Arceus (from capture pool modifier — Arceus in Lillipup compound modifier already handled in Chunk 2), Zubat captures
- **Upgrade**: Barboach→Whiscash (keep Dive Ball), Caterpie→Butterfree (keep Nest Ball), Sneasel→Weavile (keep Fast Ball), Electrike→Manectric (keep Quick Ball) — replace name + sprite + dice stats per 3C-1 table
- **Replace**: Rattata→Skarmory (Level Ball), Furret→Lapras (Friend Ball), Alcremie→Arcanine (Premier Ball) — new Pokemon + sprite + dice stats per 3C-1 table
- **Add**: Mew (Poke Ball), Jirachi (Great Ball), Kangaskhan (Safari Ball), Heracross (Net Ball), Greninja (Dusk Ball), Electivire (Ultra Ball), Magmortar (Lure Ball), Rhyperior (Heavy Ball) — dice stats per 3C-1 table
- Download/encode sprites for all new/upgraded Pokemon
- All captures use `replica.Thief` template
- Use `.part.1` appending for new capture entries

**Verification**:
- [ ] Removed Pokemon absent from capture pool
- [ ] Upgraded Pokemon show new names, same ball types, new dice stats
- [ ] Replacement Pokemon present with correct ball types
- [ ] New captures present with correct ball types and dice stats
- [ ] All capture dice match 3C-1 table exactly
- [ ] No duplicate Pokemon across hero/capture/monster/boss pools
- [ ] `textmod-compiler validate` passes

---

## Chunk 6: Legendary Additions

**Scope**: Add 6 new legendary summon items (Latias, Latios, Suicune, Entei, Raikou, Rayquaza) to the legendary pool.

**Dependencies**: Chunk 5 (capture line structure must be finalized)

**Concern**: Legendary item addition

**Files** (modify):
1. Working textmod — legendary summon sections (study existing Ho-Oh/Lugia/Kyogre/Groudon entries for format)

**Files** (read):
2. `plans/OVERHAUL_PLAN.md` Section 3D-1 — legendary dice designs
3. `plans/monster_boss_designs.md` Part 8 — legendary dog item designs (Suicune, Entei, Raikou)
4. `textmod.txt` — existing legendary modifiers (search for `.mn.Ho Oh` or `.mn.Lugia` for flee mechanic encoding format)
5. `tools/sprite_encodings.json` — legendary sprites

**Requirements**:
- Study existing legendary entries (Ho-Oh, Lugia, Kyogre, Groudon, Mewtwo) to understand the exact format
- All new legendaries: `replica.Thief`, HP 70, flee on turn 7
- Apply dice stats from 3D-1 table
- Apply item designs from monster_boss_designs.md Part 8 (Suicune/Entei/Raikou)
- Design speech/doc strings for all 6 legendaries
- Download/encode sprites
- Use `.part.1` appending for new legendary entries

**Verification**:
- [ ] All 6 new legendaries present in mod
- [ ] HP 70, flee timer, correct dice stats
- [ ] Sprites resolve from sprite_encodings.json
- [ ] `textmod-compiler validate` passes
- [ ] **USER PASTE TEST**: Verify captures and legendaries work in-game

---

## Chunk 7: Boss Fights — Gen 3

**Scope**: Implement Gen 3 boss path (Golem F4, Alpha Steelix F8, Regi Trio F12, Regigigas F16, Deoxys F20).

**Dependencies**: Chunk 3

**Parallel with**: Chunks 4, 5

**Concern**: Boss fight implementation

**Files** (modify):
1. Working textmod — boss modifier sections

**Files** (read):
2. `plans/monster_boss_designs.md` Parts 2 (Gen 3 bosses) + Part 7 (Legendary Birds F12 alt)
3. `textmod.txt` — existing boss format reference (Quagsire, Exeggutor, Xerneas, etc.)
4. `tools/sprite_encodings.json`

**Requirements**:
- Study existing boss entries to understand format: `.fight.()` blocks, `ch.omN` floor markers, fight unit structure
- Implement each Gen 3 boss using the complete fight strings from `monster_boss_designs.md`
- Each boss needs: `.fight.()` with all fight units, correct `ch.omN` floor, `.mn.` name, sprites
- Also implement Legendary Birds (F12 alternative) from Part 7
- Use `.part.1` appending — do NOT overwrite existing boss lines
- Download/encode boss sprites

**Verification**:
- [ ] All 5 Gen 3 bosses + Legendary Birds present
- [ ] Each boss has correct floor assignment (`ch.omN`)
- [ ] Fight strings match `monster_boss_designs.md` complete fight strings
- [ ] Sprites present for all bosses and minions
- [ ] `textmod-compiler validate` passes
- [ ] Existing bosses (Quagsire, Exeggutor, Xerneas, etc.) unchanged

---

## Chunk 8: Boss Fights — Gen 4

**Scope**: Implement Gen 4 boss path (Palkia F12, Dialga F16, Arceus F20).

**Dependencies**: Chunk 7 (need to understand boss format from Gen 3 implementation)

**Parallel with**: Chunk 9

**Concern**: Boss fight implementation

**Files** (modify):
1. Working textmod — boss modifier sections

**Files** (read):
2. `plans/monster_boss_designs.md` Part 3 (Gen 4 bosses)

**Requirements**:
- Same format as Chunk 7 bosses
- Arceus is the most complex: 4-phase type-shifting final boss
- Complete fight strings from `monster_boss_designs.md`
- Sprites for Palkia, Dialga, Arceus, Bronzong, Spatial Rift, Temporal Anomaly

**Verification**:
- [ ] All 3 Gen 4 bosses present with correct floors
- [ ] Arceus 4-phase fight works (Normal→Fire→Steel→Dragon)
- [ ] Fight strings match `monster_boss_designs.md`
- [ ] `textmod-compiler validate` passes

---

## Chunk 9: Boss Fights — Gen 5

**Scope**: Implement Gen 5 boss path (Unova Starters F8, Swords of Justice F12A, Forces of Nature F12B, Reshiram/Zekrom F16, Kyurem F20).

**Dependencies**: Chunk 7

**Parallel with**: Chunk 8

**Concern**: Boss fight implementation

**Files** (modify):
1. Working textmod — boss modifier sections

**Files** (read):
2. `plans/monster_boss_designs.md` Parts 4-5 (Gen 5 bosses)

**Requirements**:
- Same format as Chunk 7 bosses
- F12 has two paths (A: Swords of Justice, B: Forces of Nature) — implement both
- F16 has two versions (Reshiram OR Zekrom) — implement both
- Kyurem fusion mechanic: base form → absorbs Reshiram or Zekrom for White/Black Kyurem
- Complete fight strings from `monster_boss_designs.md`

**Verification**:
- [ ] All Gen 5 bosses present with correct floors
- [ ] Both F12 paths implemented
- [ ] Both F16 versions implemented
- [ ] Kyurem fusion phases work
- [ ] Fight strings match `monster_boss_designs.md`
- [ ] `textmod-compiler validate` passes

---

## Chunk 10: Final Integration Test [GATE CHECKPOINT]

**Scope**: Complete validation of the full assembled Sliceymon+ textmod.

**Dependencies**: All previous chunks (1-9)

**Concern**: End-to-end validation

**Files** (read):
1. Final assembled textmod
2. `plans/FULL_ROSTER.md` — verification reference

**Files** (create):
3. `textmod_expanded.txt` — final output (renamed from working copy)

**Requirements**:
- Run `textmod-compiler validate textmod_expanded.txt` — 0 errors required
- Run `textmod-compiler validate textmod_expanded.txt --round-trip` — IR stability
- Cross-reference validation: all hero pool refs, party configs, pool replacements resolve
- Verify against FULL_ROSTER.md:
  - All 25 hero colors present in character selection
  - All hero names match FULL_ROSTER
  - Monster names match (Probopass, not Aggron)
  - No removed captures present
  - All new captures/legendaries present
  - All boss fights present
- No duplicate Pokemon across categories
- **USER PASTE TEST**: Full mod pasted into game
  - 0 failed modifiers
  - All 25 colors in draft
  - Draft order alphabetical
  - Test 5+ heroes through evolution
  - Test 2+ spells
  - Verify Probopass monster, new captures, at least 1 new boss
  - Complete a Normal difficulty run

**Verification**:
- [ ] `textmod-compiler validate` = 0 errors
- [ ] Round-trip passes
- [ ] Cross-reference validation clean
- [ ] FULL_ROSTER.md completely matches mod content
- [ ] User paste test: 0 failed modifiers
- [ ] User gameplay test: Normal run completable

---

## Final Verification (After All Chunks)

- [ ] `textmod-compiler validate textmod_expanded.txt` = 0 errors
- [ ] Round-trip: `extract → build → extract` produces identical IR
- [ ] All FULL_ROSTER.md entries accounted for
- [ ] No duplicate Pokemon across hero/capture/monster/boss pools
- [ ] No non-ASCII characters
- [ ] User confirms mod works in-game (paste + play)
- [ ] Commit and push final textmod
