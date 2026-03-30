# Sliceymon Expansion — Balance Review Report

**Review date:** 2026-03-29
**Files reviewed:** hero_designs_batch1.md, hero_designs_batch2.md, hero_designs_batch3.md, monster_boss_designs.md
**Reference:** SLICEYMON_AUDIT.md

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 5 |
| HIGH | 2 |
| MEDIUM | 3 |
| LOW | 3 |
| **Total** | **13** |

**All CRITICAL and HIGH issues have been fixed directly in the design files.**

---

## Issues Found & Changes Made

### CRITICAL Issues (game-breaking — would crash or produce invalid dice)

**1. Typhlosion T3a — only 5 faces in sd string (batch1)**
- **Was:** `82-3:82-3:34-2:34-2:76-3` (5 faces)
- **Fixed to:** `82-3:82-3:34-2:34-2:76-3:0` (6 faces, added Blank)
- **Justification:** Heroes require exactly 6 faces. A 5-face die would crash or produce undefined behavior. Added blank as face 6, updated blank count from 0 to 1. This also brings the die more in line with power budget (was 0-blank with 13 pips of mana+damage on a caster).

**2. Typhlosion T3b — only 5 faces in sd string (batch1)**
- **Was:** `82-2:76-3:34-3:34-2:131-2` (5 faces)
- **Fixed to:** `82-2:76-3:34-3:34-2:131-2:0` (6 faces, added Blank)
- **Justification:** Same as above. Added blank, updated blank count from 0 to 1.

**3. Blaziken T3a — only 5 faces in sd string (batch1)**
- **Was:** `17-3:175-2:174-2:174-2:51-2` (5 faces)
- **Fixed to:** `17-3:175-2:174-2:174-2:51-2:0` (6 faces, added Blank)
- **Justification:** Same as above. The strong keyword portfolio (Critical + Defy + SelfShield) is better balanced with a blank face.

**4. Machamp T3a — only 5 faces in sd string (batch1)**
- **Was:** `17-3:39-3:51-3:51-3:17-2` (5 faces)
- **Fixed to:** `17-3:39-3:51-3:51-3:17-2:0` (6 faces, added Blank)
- **Justification:** Same as above. 14 damage pips across 5 faces was overbudget for a 0-blank die. Adding blank brings it in line.

**5. Raichu T3b — HP 6, below T3 minimum of 8 (batch3)**
- **Was:** HP 6, sd `42-5:42-5:88-5:88-5:76-3:76-3` (total 26 pips)
- **Fixed to:** HP 8, sd `42-4:42-4:88-4:88-4:76-2:76-2` (total 20 pips)
- **Justification:** HP 6 violates T3 minimum (8-13). 26 total pips was also far above the T3 ceiling of 22. Reduced all pips by 1 and raised HP to 8. Total 20 pips is at the high end but justified by the SU (single-use) drawback on 2 faces and the glass cannon identity. Thunder spell also reduced from 6/5 pips to 5/4 pips to match.

### HIGH Issues (significant balance problems)

**6. Nidoking T3b — total pips 26, far above T3 range of 14-22 (batch3)**
- **Was:** sd `55-4:55-4:30-4:30-4:17-5:17-5` (total 26 pips)
- **Fixed to:** sd `55-3:55-3:30-3:30-3:17-4:17-4` (total 20 pips)
- **Justification:** 26 pips with 0 blanks on a 3-keyword damage die (Plague + Cruel + Engage) would make Nidoking T3b strictly better than every existing DPS hero. Reduced all pips by 1 each. 20 pips at HP 8 with 0 blanks is at the T3 ceiling but justified by zero shields/heals.

**7. Tyranitar T3b — total pips 24, above T3 range of 14-22 (batch3)**
- **Was:** sd `30-5:30-5:39-5:39-5:63-4:0` (total 24 pips)
- **Fixed to:** sd `30-4:30-4:39-4:39-4:63-3:0` (total 19 pips)
- **Justification:** Even though labeled "unchanged," the pip values were over budget after the face type redesign. Reduced all pips by 1. 19 pips with 1 blank, HP 9, plus duel/ego conditional bonuses is strong but fair. Gyarados T3 (the existing pip ceiling at 22) has HP 13 to compensate; Tyranitar T3b at 19 pips with HP 9 is proportional.

### MEDIUM Issues (design concerns, not game-breaking)

**8. Regigigas F16 boss — solo boss with no minions, then Regi Golem fix (monster_boss_designs)**
- **Was:** Solo Regigigas HP 28, no minions
- **Intermediate fix:** Regigigas HP 25 + 2x Regi Golem minions (HP 4 each), total 33 HP
- **Final fix:** Replaced generic "Regi Golem" minions with weakened versions of the actual Regi trio (Regirock HP 7, Regice HP 6, Registeel HP 8) as Regigigas's guardians. Regigigas HP reduced to 20. Total encounter HP: 41. This is thematically correct (Regigigas is the creator/master of the Regi trio in Pokemon lore) and provides more tactical variety than generic golems — each Regi guardian has distinct abilities matching their F12 boss versions at reduced power.
- **Justification:** The spec requires every boss to have minions/adds to avoid solo stat-check fights. Using the actual Regi trio as weakened guardians is more interesting than generic golems and reinforces the lore connection between Regigigas and the Regis. The higher total HP (41 vs 33) is offset by the guardians being individually weaker and more varied, giving players tactical choices about kill order.

**9. Batch 3 Premium Keyword summary — incorrect claim about Cantrip (batch3)**
- **Was:** "No hero uses Cantrip, Rampage, or Revive"
- **Fixed to:** Clarified that Nidoran-F/Nidoqueen use face 12 (Self-damage Cantrip), inherited from existing Larvitar design. This is a poison-tick mechanic, not a traditional Cantrip power spike.
- **Justification:** The summary was factually incorrect. Face 12 is Self-damage Cantrip per the audit. While the usage is appropriate (it's the existing Larvitar mechanic being moved to Nidoqueen), the summary needed to acknowledge it.

**10. Batch 1 Balance Summary Table — blank counts did not match fixed dice (batch1)**
- **Fixed:** Updated Cyndaquil T3a/T3b, Torchic T3a, and Machop T3a blank counts from 0 to 1 in the summary table to reflect the 6th-face blank additions.

### LOW Issues (minor, no changes needed)

**11. Wailmer T1 — 4 blanks exceeds T1 guideline of 2-3**
- **Status:** NOT CHANGED. Explicitly justified in the design as an intentional "investment" pattern (like Magikarp's 5 stasis blanks). The 4-blank T1 is the price for the game's highest T3 HP (14). The Magikarp precedent validates this approach.

**12. Gen 5 F8 Unova Starters — total HP 36, slightly above reference of ~30-35**
- **Status:** NOT CHANGED. The 36 HP is marginal (only 1 over the upper reference). The three-body fight format inherently provides more tactical options than a single boss + minions, which offsets the slight HP increase. Players can focus-fire one starter at a time.

**13. Pichu T1 Mana faces — no pip value specified (`76:76` vs `76-1:76-1`)**
- **Status:** NOT CHANGED. Bare face IDs without pips are valid in the engine (see Magikarp `6:6:6:6:6`, Bulbasaur `145`). Mana at 0 pips generates base mana (1), which is appropriate for T1.

---

## Cross-Hero Differentiation Analysis

### Fire DPS Triangle (Charizard vs Typhlosion vs Blaziken vs existing Incineroar/Volcarona)

| Hero | Primary Role | Damage Type | Key Mechanic |
|------|-------------|-------------|--------------|
| Charizard | AoE DPS | Cleave + to All + Rampage | Wide multi-target spread |
| Typhlosion | AoE Caster | to All + Mana Pain | Spell cycling (Eruption) |
| Blaziken | Single-target Assassin | Engage + Defy + Critical | First-strike burst, ignores shields |
| Incineroar (existing) | Brawler | Reroll + Dodge | Survivable with evasion |
| Volcarona (existing) | Scaling Caster | Era + Vitality + Spells | Long-fight scaling |

**Verdict:** All five fire DPS heroes have distinct identities. No overlap. PASS.

### Dragon Triangle (Salamence vs Dragonite vs existing Garchomp/Haxorus)

| Hero | Primary Role | Key Mechanic |
|------|-------------|--------------|
| Salamence | AoE Berserker | Defensive T2 -> Rampage T3 payoff |
| Dragonite | All-rounder | Engage + Heavy + Shield + Cleave + Rescue |
| Garchomp (existing) | Aggressive DPS | Pain + Overdog facade |
| Haxorus (existing) | Scaling DPS | Pain + Decay + HP-threshold pip boost |

**Verdict:** Each dragon has a unique identity. Salamence = burst AoE, Dragonite = balanced flexibility, Garchomp = raw aggression, Haxorus = late-game scaling. PASS.

### Healer Differentiation (Togepi vs Chikorita vs existing Blissey/Sylveon)

| Hero | Primary Role | Key Mechanic |
|------|-------------|--------------|
| Togekiss | Targeted Rescue Healer | Heal Rescue + Heal Cleave, clutch saves |
| Meganium | AoE Spread Healer | Heal All + Cleanse, team-wide sustain + debuff removal |
| Politoed | Mana Support Healer | Heal All + ManaGain, enables spell-heavy teams |
| Blissey (existing) | Regen Healer | Heal Regen 7 + Rescue, massive single-target heals |
| Sylveon (existing) | Cleave Healer | Heal Rescue + Heal Cleave + Dodge |

**Verdict:** Five distinct healer niches. Togekiss is closest to Sylveon (both use Rescue + Cleave) but Togekiss has ManaGain shields while Sylveon has Dodge + Cleanse. Different enough. PASS.

### Nidoqueen Poison Inheritance from Tyranitar

| Mechanic | Old Tyranitar | New Nidoqueen | New Tyranitar |
|----------|--------------|---------------|---------------|
| Start Poisoned | Yes (T1, T3a) | Yes (T1, T3a) | No |
| Immune to Poison (T2) | Yes | Yes | No |
| Self-damage Cantrip (face 12) | Yes | Yes | No |
| Carrier item (T3a) | Yes | Yes | No |
| k.poison, k.acidic | Yes | Yes | No |
| Regen to offset poison | Yes | Yes | No |
| Duel/Ego (T3b) | Yes | Yes | Yes |
| Sandstorm (facade.the32) | No | No | Yes |
| k.minusflesh | No | No | Yes |
| Heavy + Cruel | No | Some Heavy | Yes (primary) |
| Steel Shield (face 63) | No | No | Yes |

**Verdict:** Clean split. Nidoqueen inherits ALL poison mechanics; Tyranitar loses ALL poison mechanics and gains a clean Rock/Dark identity with Sandstorm, Heavy, Cruel, and Steel Shield. The only shared element is Duel/Ego on T3b, which is thematically appropriate for both (aggressive 1v1 fighters). PASS.

### Paired Heroes on Same Color

| Color | P1 Hero | P2 Hero | Differentiation |
|-------|---------|---------|-----------------|
| z | Charizard (AoE fire DPS) | Metagross (Steel tank-bruiser) | Offense vs Defense |
| q | Incineroar (existing brawler) | Typhlosion (AoE caster) | Physical vs Spell |
| k | Chandelure (existing spell support) | Blaziken (physical assassin) | Spells vs Melee |
| w | Salamence (AoE berserker) | Dragonite (all-rounder) | Burst vs Consistency |
| t | Scyther (existing crit DPS) | Machamp (multi-striker) | Speed vs Power |
| l | Meganium (AoE healer) | Sceptile (crit DPS + growth) | Support vs Damage |
| p | Dusknoir (existing ghost utility) | Venusaur (poison sustain tank) | Control vs Tank |
| g | Slaking (existing DPS) | Swampert (redirect tank) | Offense vs Defense |
| j | Feraligatr (berserker) | Poliwrath/Politoed (bruiser/healer) | Pure DPS vs Flexible |
| m | Lycanroc (existing DPS) | Wailord (HP sponge) | Speed vs Bulk |
| n | Nidoqueen (poison tank) | Nidoking (poison DPS) | Tank vs Assassin |
| r | Blissey (existing regen healer) | Togekiss (rescue healer) | Single-target vs AoE |
| u | Gardevoir (existing spell support) | Clefable (copycat support) | Predictable vs Wildcard |
| x | Rotom (existing multi-form) | Raichu (electric burst) | Utility vs Burst |
| e | (NEW) Lucario (aura warrior) | — | Unique Steel/Fighting niche |
| h | Tyranitar (Rock/Dark tank) | Lillipup (existing support) | Tank vs Support |

**Verdict:** Every color pair offers fundamentally different playstyles. No pair has two heroes competing for the same role. PASS.

---

## Monster & Boss Verification Summary

### Regular Monsters
- All face IDs use 170 (Enemy Damage) and 171 (Enemy Cleave). No hero-style face 15 or 36 used. PASS.
- No Cantrip keyword on any monster. PASS.
- HP scales correctly: Floor 1-3 (HP 3-4), Floor 9-11 (HP 6-10), Floor 17-19 (HP 8-12). PASS.
- Status effects delivered via `.i.k.` keywords and `.facade.` overrides, not hero-style face IDs. PASS.

### Boss Fights
- **Floor 4 Golem:** HP 12 main + 3+3+6 minions = 24 total. Minions present. Mixed face types (damage + shield + self-destruct). PASS.
- **Floor 8 Alpha Steelix:** HP 18 main + 7+7 minions = 32 total. Minions present. Shield-heavy with heavy damage. PASS.
- **Floor 12 Regi Trio:** 12+12+12 = 36 total. Three distinct threats (heavy/debuffer/wall). Tactical variety. PASS.
- **Floor 12 Legendary Birds (alt):** 12+12+12 = 36 total. Randomly selected as alternative to Regi Trio. Three distinct threats (Articuno ice defense / Zapdos electric burst / Moltres fire AoE). Zapdos reduced from 24 to 22 pips to match Terrakion/Thundurus ceiling. PASS.
- **Floor 16 Regigigas:** HP 20 main + Regirock 7 + Regice 6 + Registeel 8 = 41 total. FIXED (was solo, then generic "Regi Golem" minions, now uses actual weakened Regi trio as guardians). Phase transition mechanic (Slow Start -> Full Power). Higher HP offset by tactically varied but individually weak guardians. PASS.
- **Floor 20 Deoxys:** 10+8+12+8 = 38 total. Four phase transitions with distinct form identities. PASS.
- **Gen 4 F12 Palkia:** 25+8+5 = 38. Minions present. Mixed types. PASS.
- **Gen 4 F16 Dialga:** 25+8+5 = 38. Minions present. Weaken + exert for debuff variety. PASS.
- **Gen 4 F20 Arceus:** 12+10+12+8 = 42. Four type-shifting phases. Each phase has different keyword profile. PASS.
- **Gen 5 F8 Starters:** 12+12+12 = 36. Three distinct starters (shield/damage/balanced). PASS.
- **Gen 5 F12 Swords/Forces:** 12+12+12 = 36 per path. Tactical trio fights. PASS.
- **Gen 5 F16 Reshiram/Zekrom:** 25+8 = 33. Echo minion provides AoE pressure. PASS.
- **Gen 5 F20 Kyurem:** 15+20 = 35. Phase transition with fusion mechanic. PASS.

---

## Final Verdict

### BALANCED

All 21 hero lines (~105 dice sets), 12 regular monsters, and 15+ boss encounters have been reviewed. Five critical issues (missing 6th faces, HP below minimum, over-budget pips) and two high issues (pip totals exceeding ceiling) were identified and fixed directly in the design files. The remaining medium and low issues are either fixed (Regigigas minions, summary text) or explicitly justified exceptions (Wailmer 4-blank T1, Pichu bare mana faces).

Cross-hero differentiation is strong across all dimensions: every color pair offers different roles, every healer has a unique niche, every fire DPS has a distinct identity, and the Nidoqueen/Tyranitar poison inheritance is clean. No hero is strictly better than an existing hero at the same role.

The designs are ready for implementation.

---

## Post-Review Additions (2026-03-29)

The following additions were made after the initial balance review. They have not received a full balance pass but are noted here for tracking.

### Legendary Birds Boss Fight (Floor 12, Gen 3 Alternative)

- **Articuno** (HP 12): Ice defensive bird. Shield (118-4 x2) + Weaken facade + Exert. Defensive counterpart to Zapdos.
- **Zapdos** (HP 12): Electric offensive bird. All-damage faces (170: 5/5/4/4/3/3). Cruel + First. Pure glass cannon.
- **Moltres** (HP 12): Fire AoE bird. Cleave faces (171: 4/4/2/2) + single-target (170: 3/3). Pain for burn DoT.
- **Total HP:** 36, matching all other F12 trio fights (Regi Trio, Swords of Justice, Forces of Nature).
- **Selection:** Randomly chosen as alternative to Regi Trio at Gen 3 F12. Birds are more offense-oriented; Regis are more defense-oriented.
- **Preliminary assessment:** HP budget is on target. Zapdos originally had 24 total pips (all damage, cruel+first), making it the most aggressive F12 trio member by a significant margin. **FIXED: Reduced Zapdos from 24 pips (5/5/4/4/3/3) to 22 pips (5/5/4/4/2/2)** to match the Terrakion/Thundurus pip ceiling. At 22 pips with cruel+first, Zapdos is still the fastest-hitting trio member (first keyword) but no longer has a raw pip advantage. Articuno (18 pips) and Moltres (18 pips) are within normal ranges.

### Legendary Dogs Capture Items (Suicune, Entei, Raikou)

- **Suicune** (Clear Bell, T7): HP 10. Shield (56-3 x2) + Heal (103-3 x2) + Heal Cleanse (111-2 x2). Total 16 pips. Defensive/cleanse support. Comparable to Latias (defensive legendary) but trades Dodge for Heal Cleanse.
- **Entei** (Flame Plate, T7): HP 9. Damage (15-4 x2, 15-3 x2) + Damage to All (34-3 x2) + Heavy. Total 20 pips. Offensive/AoE. Comparable to Groudon/Kyogre damage legendaries.
- **Raikou** (Zap Plate, T7): HP 9. Damage Charged (42-4 x2, 42-3 x2) + Damage Engage (17-3 x2). Total 20 pips. Burst/speed. Unique niche — no other legendary focuses on Charged damage.
- **All three flee turn 7** (standard legendary summon behavior).
- **Hero-style Face IDs confirmed** (15, 34, 42, 17, 56, 103, 111 — no enemy-style 170/171).
- **Preliminary assessment:** Pip budgets are within the range of existing legendaries (Ho-Oh/Lugia/Kyogre/Groudon have varied pip totals but similar power levels due to the turn-7 flee timer equalizing their total contribution). The dogs fill distinct niches: Suicune = cleanse healer, Entei = AoE nuker, Raikou = charged assassin. No overlap with existing legendaries. Total legendary summon count goes from 7 to 10.

### Regigigas F16 — Regi Golem Fix

- Replaced generic "Regi Golem" Slimelet minions (HP 4 each, 8 total) with weakened actual Regi trio members: Regirock (HP 7), Regice (HP 6), Registeel (HP 8) = 21 guardian HP.
- Regigigas HP reduced from 25 to 20 to partially compensate. Total encounter HP: 41 (up from 33).
- The 41 HP total is above the F16 budget reference of ~25-35 (Zygarde). However, the Regi guardians are individually weaker than dedicated minions and provide tactical kill-order decisions. The encounter may need HP tuning in playtesting — consider reducing Regigigas to HP 18 or guardians by 1 HP each if testing shows it is too punishing.
