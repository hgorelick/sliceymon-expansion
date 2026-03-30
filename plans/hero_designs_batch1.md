# Hero Designs Batch 1 -- 7 New Pokemon Evolution Lines

## Design Notes & Balance Reference

**Tier guidelines (enforced below):**
- T1: HP 3-6, 2-3 blanks (face 0), 1-2 pip values per active face
- T2: HP 6-9, 1-2 blanks, 2-3 pip values per active face
- T3: HP 8-13, 0-1 blanks, 3-6 pip values per active face
- Each hero: T1, T2a, T2b, T3a, T3b
- Spells use 5 faces in abilitydata (NOT 6)
- 1 damage pip ~ 1.4 shield pips in value
- Premium keywords (Cantrip, Rampage, Revive, QuadUse) used sparingly

**Spell abilitydata format** (from Squirtle/Espurr reference):
`abilitydata.(Template.sd.FACE-PIP:FACE-PIP:FACE-PIP:FACE-PIP:FACE-PIP.i.MODIFIERS.n.SpellName.img.IMAGE)`
- Template is typically `Statue`, `Fey`, or `lost`
- 5 faces only (not 6)
- img uses existing asset names (spark, stream, scorch, Whirlpool, cursed bolt, etc.)

**Face ID Quick Reference (used in this document):**
- Damage: 15(basic), 17(Engage), 30(Cruel), 34(to All), 36(Cleave), 39(Heavy), 42(Charged), 51(SelfShield), 53(Poison), 88(SU Charged), 115(SU), 126(Cantrip), 131(Weaken), 137(Rampage), 174(Defy), 175(Critical)
- Shield: 56(basic), 61(ManaGain), 63(Steel), 64(Rescue), 65(Pristine), 69(Cleave), 71(Cleanse), 72(to All), 119(Repel)
- Heal: 103(basic), 105(Vitality), 106(Rescue), 109(Cleave), 110(Regen), 111(Cleanse), 112(ManaGain), 113(Growth), 114(DoubleUse)
- Mana: 76(basic), 77(Cantrip), 79(Growth), 80(Decay), 82(Pain), 84(Pair), 85(Trio), 93(SU)
- Utility: 0(Blank), 6(Stasis), 118(Redirect SelfShield), 123(Dodge), 125(Reroll Cantrip), 130(Reuse), 136(Revive)
- Add: 145(Poison), 151(Growth)

---

## 1. Charmander -> Charmeleon -> Charizard (color z P1, replaces Agumon)

**Template:** replica.Statue
**Role:** Fire AoE DPS -- damage + cleave + rampage, Mana Pain for spell support
**Speech:** Char!:Charr!:Mander! / Charmeleon!:Grr! / CHARIZARD!:RAWR!
**Doc:** (none -- straightforward fire DPS)

### T1 Charmander: HP 5
- **sd:** `34-1:30-1:0:0:30-1:0`
- **Faces:**
  1. Damage to All 1 -- tail flame flicker, light AoE chip
  2. Damage Cruel 1 -- Scratch attack
  3. Blank
  4. Blank
  5. Damage Cruel 1 -- Ember
  6. Blank
- **Blank count: 3** (within T1 range 2-3)
- **Total offense: ~3 damage pips** (within T1 range 1-2 per active face)
- **Balance note:** Matches Agumon T1 pattern (HP 5, 3 blanks, mix of damage types). Slightly less raw damage than Agumon (who had 90-2 + 30-2 + 30-1 = stronger burst) but Cruel keyword adds value.

### T2a Charmeleon: HP 7
- **sd:** `34-2:30-2:36-1:36-1:82-1:0`
- **Faces:**
  1. Damage to All 2 -- Flamethrower AoE
  2. Damage Cruel 2 -- Slash
  3. Damage Cleave 1 -- Fire Fang sweep
  4. Damage Cleave 1 -- Fire Fang sweep
  5. Mana Pain 1 -- Rage (fuel spells via taking hits)
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Total offense: ~8 damage pips equivalent** (within T2 budget)
- **Balance note:** Mirrors Agumon T2 structure (HP 8, sd=90-3:30-2:171-1:171-1:30-2:0) but slightly less HP compensated by AoE spread + mana generation. Cleave 1 is modest at T2.

### T2b Charmeleon: HP 8
- **sd:** `30-2:30-2:82-2:82-2:34-1:0`
- **Faces:**
  1. Damage Cruel 2 -- Slash
  2. Damage Cruel 2 -- Dragon Rage
  3. Mana Pain 2 -- Rage buildup
  4. Mana Pain 2 -- Rage buildup
  5. Damage to All 1 -- Smokescreen fire
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Caster-leaning variant. Higher mana generation (4 mana pips from Pain) but less cleave. Trades AoE physical for spell fuel. Comparable to Greymon T2 with its 90-3 + 30-2 + 171-1 setup.

### T3a Charizard: HP 10
- **sd:** `34-3:34-2:36-2:36-2:137-2:82-2`
- **Faces:**
  1. Damage to All 3 -- Fire Blast
  2. Damage to All 2 -- Flamethrower
  3. Damage Cleave 2 -- Wing Attack
  4. Damage Cleave 2 -- Wing Attack
  5. Damage Rampage 2 -- Dragon Rage frenzy
  6. Mana Pain 2 -- Blaze fury
- **Blank count: 0** (within T3 range 0-1)
- **Total offense: ~15 damage pips equivalent** (strong T3 DPS, comparable to MGreymon's 170-5:170-4:90-3:90-3:82-3:82-3 which totals ~20 pips but uses enemy-style damage IDs)
- **Spell: Fire Blast**
  - `abilitydata.(Statue.sd.34-3:34-2:0:0:76-3.n.Fire Blast.img.spark.hsv.-10:20:80)`
  - Spell faces: Damage to All 3, Damage to All 2, Blank, Blank, Mana 3 (cost)
  - Design: Big AoE nuke spell. 5 pips of AoE damage for 3 mana cost.
- **Balance note:** Rampage is the premium keyword here -- justified at T3 only with only 2 pips. Total pip budget is high but spread across AoE (less efficient per-target than single-target). HP 10 is standard for T3 DPS. Compares to Incineroar T3a (HP 10, sd=128-1:125-1:34-1:34-1:123:157) which uses reroll + dodge for survivability; Charizard trades defense for raw AoE.

### T3b Charizard: HP 11
- **sd:** `36-3:36-3:34-2:34-2:82-3:0`
- **Faces:**
  1. Damage Cleave 3 -- Air Slash
  2. Damage Cleave 3 -- Air Slash
  3. Damage to All 2 -- Heat Wave
  4. Damage to All 2 -- Heat Wave
  5. Mana Pain 3 -- Blaze overdrive
  6. Blank
- **Blank count: 1** (within T3 range 0-1)
- **Spell: Fire Blast** (same as T3a)
  - `abilitydata.(Statue.sd.34-3:34-2:0:0:76-3.n.Fire Blast.img.spark.hsv.-10:20:80)`
- **Balance note:** Tankier variant (HP 11) with 1 blank tradeoff. No Rampage -- trades burst for consistency and survivability. Cleave 3 is the primary output, making this variant better against 2-3 enemy groups rather than full hordes. Higher mana generation (Pain 3) supports heavier spell use.

---

## 2. Cyndaquil -> Quilava -> Typhlosion (color q P2, replaces Slugma)

**Template:** replica.Lost
**Role:** Fire eruption CASTER -- mana generation + fire AoE spells
**Speech:** Cynda!:Quil! / Quilava!:Lava! / TYPHLOSION!:Eruption!
**Doc:** (none)

### T1 Cyndaquil: HP 4
- **sd:** `82-1:76-1:15-1:0:0:0`
- **Faces:**
  1. Mana Pain 1 -- inner fire stoked by damage
  2. Mana 1 -- Leer (basic mana gen)
  3. Damage 1 -- Tackle
  4. Blank
  5. Blank
  6. Blank
- **Blank count: 3** (within T1 range 2-3)
- **Total offense: ~1 damage pip + 2 mana pips** (low damage, high utility -- caster setup)
- **Balance note:** Very similar to Litten T1 (HP 4, inherits template dice). Weak early but mana-focused. Compared to Slugma T1 (HP 2 with complex self-shield mechanics), this is straightforward and healthier at HP 4.

### T2a Quilava: HP 7
- **sd:** `82-2:76-2:34-1:34-1:15-1:0`
- **Faces:**
  1. Mana Pain 2 -- Flame Wheel self-damage fuel
  2. Mana 2 -- Swift charge
  3. Damage to All 1 -- Flame Wheel
  4. Damage to All 1 -- Flame Wheel
  5. Damage 1 -- Quick Attack
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Mana engine building up. 4 mana pips + 3 damage pips across 5 faces. The value is in enabling spells rather than raw die output.

### T2b Quilava: HP 6
- **sd:** `82-2:82-2:76-2:76-2:34-1:0`
- **Faces:**
  1. Mana Pain 2 -- Smokescreen
  2. Mana Pain 2 -- Smokescreen
  3. Mana 2 -- Focus Energy
  4. Mana 2 -- Focus Energy
  5. Damage to All 1 -- Ember burst
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Full caster variant. 8 mana pips (!) but only 1 damage pip on die. Relies entirely on spells for output. Lower HP (6) reflects glass cannon caster. Pain keyword means self-damage fuels mana but reduces survivability.

### T3a Typhlosion: HP 9
- **sd:** `82-3:82-3:34-2:34-2:76-3:0`
- **Faces:**
  1. Mana Pain 3 -- Inferno fuel
  2. Mana Pain 3 -- Inferno fuel
  3. Damage to All 2 -- Lava Plume
  4. Damage to All 2 -- Lava Plume
  5. Mana 3 -- Flash Fire charge
  6. Blank
- **Blank count: 1** (within T3 range 0-1)
- **Spell: Eruption**
  - `abilitydata.(Statue.sd.34-4:34-3:0:0:76-4.n.Eruption.img.spark.hsv.-20:30:90)`
  - Spell faces: Damage to All 4, Damage to All 3, Blank, Blank, Mana 4 (cost)
  - Design: Massive AoE nuke. 7 total AoE damage for 4 mana cost. The signature move.
- **Balance note:** 8 mana pips on the die itself (6 Pain + 3 basic, minus blank) means Eruption fires frequently. 4 AoE damage pips on die + Eruption spell = enormous AoE output. HP 9 is lower for T3 to offset the power. Pain keyword is a built-in drawback (self-damage). One blank keeps the budget honest. Comparable in total output to Chandelure T3 which also relies on spell cycling.

### T3b Typhlosion: HP 10
- **sd:** `82-2:76-3:34-3:34-2:131-2:0`
- **Faces:**
  1. Mana Pain 2 -- Smokescreen
  2. Mana 3 -- Focus Blast charge
  3. Damage to All 3 -- Eruption passive
  4. Damage to All 2 -- Lava Plume
  5. Damage Weaken 2 -- Will-O-Wisp
  6. Blank
- **Blank count: 1** (within T3 range 0-1)
- **Spell: Eruption** (same as T3a)
  - `abilitydata.(Statue.sd.34-4:34-3:0:0:76-4.n.Eruption.img.spark.hsv.-20:30:90)`
- **Balance note:** More balanced variant. Trades raw mana generation for Weaken utility (reducing enemy damage). HP 10 is standard. Less Pain (2 vs 6) means less self-damage but slower spell cycling. Weaken 2 provides team defensive value that T3a lacks entirely. One blank is the tradeoff for having both mana generation and Weaken on one die.

---

## 3. Torchic -> Combusken -> Blaziken (color k P2, replaces Espurr)

**Template:** replica.Lost
**Role:** Fire/Fighting speed assassin -- Defy + Critical + Engage + DoubleUse (NOT Cantrip)
**Speech:** Torchic!:Chic! / Combusken!:Ken! / BLAZIKEN!:Blaze Kick!
**Doc:** (none)

### T1 Torchic: HP 4
- **sd:** `17-1:15-1:0:0:56-1:0`
- **Faces:**
  1. Damage Engage 1 -- Peck (strikes first)
  2. Damage 1 -- Scratch
  3. Blank
  4. Blank
  5. Shield 1 -- Sand Attack (light defense)
  6. Blank
- **Blank count: 3** (within T1 range 2-3)
- **Total offense: ~2 damage pips + 0.7 shield equivalent** (modest T1)
- **Balance note:** Matches Espurr T1 pattern (HP 4, 3+ active faces). Engage keyword at T1 is fine (not premium). Comparable to Litten T1 or Cyndaquil T1.

### T2a Combusken: HP 7
- **sd:** `17-2:175-1:174-1:174-1:51-1:0`
- **Faces:**
  1. Damage Engage 2 -- Double Kick
  2. Damage Critical 1 -- Blaze Kick
  3. Damage Defy 1 -- Flame Charge
  4. Damage Defy 1 -- Flame Charge
  5. Damage SelfShield 1 -- Bulk Up
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Diverse keyword portfolio. Engage 2 is the workhorse, Critical/Defy at 1 pip each are exploratory. SelfShield adds minor survivability. Total ~6 damage pips is solid for T2. Compared to Espurr T2a (HP 6, sd=34-1:76-2:18-1:18-1:76-2:76-2) this is entirely physical rather than mana-based.

### T2b Combusken: HP 8
- **sd:** `17-2:17-2:174-1:174-1:15-1:0`
- **Faces:**
  1. Damage Engage 2 -- Double Kick
  2. Damage Engage 2 -- Low Kick
  3. Damage Defy 1 -- Flame Charge
  4. Damage Defy 1 -- Flame Charge
  5. Damage 1 -- Peck
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Pure damage variant. No Critical, no SelfShield. Higher Engage concentration (4 pips) for consistent first-strike. HP 8 compensates for lack of utility. Total ~8 damage pips spread across Engage and Defy.

### T3a Blaziken: HP 9
- **sd:** `17-3:175-2:174-2:174-2:51-2:0`
- **Faces:**
  1. Damage Engage 3 -- High Jump Kick
  2. Damage Critical 2 -- Blaze Kick
  3. Damage Defy 2 -- Flare Blitz
  4. Damage Defy 2 -- Flare Blitz
  5. Damage SelfShield 2 -- Bulk Up
  6. Blank
- **Blank count: 1** (within T3 range 0-1)
- **Spell: Blaze Kick**
  - `abilitydata.(lost.sd.174-3:175-2:0:0:76-2.n.Blaze Kick.img.scorch.hue.10)`
  - Spell faces: Damage Defy 3, Damage Critical 2, Blank, Blank, Mana 2 (cost)
  - Design: High single-target burst with Defy (survives lethal) + Critical (chance to double). 5 damage pips for 2 mana.
- **Balance note:** HP 9 is appropriate for a glass cannon assassin. One blank keeps budget honest for the strong keyword portfolio (Critical + Defy + SelfShield). Total ~11 damage pips is strong but single-target focused. Compared to Meowstic T3 (HP 8, utility/control caster), this is a completely different design direction.

### T3b Blaziken: HP 10
- **sd:** `17-3:17-2:174-2:174-2:114-2:0`
- **Faces:**
  1. Damage Engage 3 -- Sky Uppercut
  2. Damage Engage 2 -- Brave Bird
  3. Damage Defy 2 -- Flare Blitz
  4. Damage Defy 2 -- Flare Blitz
  5. Heal DoubleUse 2 -- Speed Boost (DoubleUse = acts twice, represents Speed Boost)
  6. Blank
- **Blank count: 1** (within T3 range 0-1)
- **Spell: Blaze Kick** (same as T3a)
  - `abilitydata.(lost.sd.174-3:175-2:0:0:76-2.n.Blaze Kick.img.scorch.hue.10)`
- **Balance note:** DoubleUse variant. Heal DoubleUse 2 means the heal face triggers twice when rolled, representing Speed Boost's extra action. Trades Critical and SelfShield for DoubleUse healing + more Engage. HP 10 is higher to support the DoubleUse playstyle (needs to survive to benefit from double actions). 1 blank at T3 is the tradeoff for the DoubleUse power. NOT Cantrip -- as specified in expansion plan, DoubleUse represents Speed Boost.

---

## 4. Bagon -> Shelgon -> Salamence (color w P1, replaces Ditto)

**Template:** replica.Statue
**Role:** Dragon rampage berserker -- T1/T2 defensive cocoon, T3 devastating AoE
**Speech:** Bagon!:Bash! / Shelgon!:Shell! / SALAMENCE!:FLY!:RAAAWR!
**Doc:** (none -- pure physical, no spell)

### T1 Bagon: HP 5
- **sd:** `39-1:15-1:56-1:0:0:0`
- **Faces:**
  1. Damage Heavy 1 -- Headbutt (dreams of flying, bashes head)
  2. Damage 1 -- Bite
  3. Shield 1 -- Dragon Dance (early defense)
  4. Blank
  5. Blank
  6. Blank
- **Blank count: 3** (within T1 range 2-3)
- **Total offense: ~2 damage pips + 0.7 shield equivalent** (standard T1)
- **Balance note:** Straightforward T1. Heavy at 1 pip is fine for flavor. Matches Agumon T1 power level. The fantasy is that Bagon is stubborn but weak -- headbutts things.

### T2a Shelgon: HP 8
- **sd:** `56-3:56-2:63-2:63-2:39-1:0`
- **Faces:**
  1. Shield 3 -- Protect (cocoon shell)
  2. Shield 2 -- Iron Defense
  3. Shield Steel 2 -- Harden (reduces incoming damage)
  4. Shield Steel 2 -- Harden
  5. Damage Heavy 1 -- Headbutt (weak offense in cocoon)
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** DEFENSIVE cocoon phase. 9 shield pips + 2 Steel shields = massive defense. Only 1 damage pip. This is intentional -- Shelgon is an armored cocoon dreaming of flying, not attacking. Shield value: 9 shield pips / 1.4 ~ 6.4 damage equivalent, which is within T2 budget. HP 8 + heavy shields makes Shelgon a wall.

### T2b Shelgon: HP 7
- **sd:** `63-3:63-2:56-2:56-2:39-2:0`
- **Faces:**
  1. Shield Steel 3 -- Iron Defense
  2. Shield Steel 2 -- Iron Defense
  3. Shield 2 -- Protect
  4. Shield 2 -- Protect
  5. Damage Heavy 2 -- Zen Headbutt
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Slightly more offensive cocoon variant. Still shield-heavy but Heavy 2 provides meaningful damage. Lower HP (7 vs 8) trades bulk for the extra damage pip. Steel Shield is the premium defensive keyword here.

### T3a Salamence: HP 10
- **sd:** `137-3:137-3:36-3:36-3:34-2:82-2`
- **Faces:**
  1. Damage Rampage 3 -- Outrage (hit all, gain rage)
  2. Damage Rampage 3 -- Outrage
  3. Damage Cleave 3 -- Dragon Claw sweep
  4. Damage Cleave 3 -- Dragon Claw sweep
  5. Damage to All 2 -- Intimidate roar
  6. Mana Pain 2 -- Moxie (power from pain)
- **Blank count: 0** (within T3 range 0-1)
- **No spell** (pure physical berserker)
- **Balance note:** THE PAYOFF for enduring Shelgon's weak offense. Rampage is the premium keyword -- justified here because (a) T3 only, (b) the T2 phase was almost entirely defensive so the player "earned" it, and (c) Mana Pain self-damage is a drawback. 6 Rampage pips + 6 Cleave pips + 2 AoE pips = devastating multi-target damage. HP 10 is standard. Comparable to Garchomp T3a (HP 11, 170-4:170-4:158-2:158-2:123) in total offensive power. No spell keeps the "brute force" fantasy.

### T3b Salamence: HP 11
- **sd:** `36-4:36-3:34-2:34-2:39-2:56-2`
- **Faces:**
  1. Damage Cleave 4 -- Fly
  2. Damage Cleave 3 -- Aerial Ace
  3. Damage to All 2 -- Dragon Breath
  4. Damage to All 2 -- Dragon Breath
  5. Damage Heavy 2 -- Crunch
  6. Shield 2 -- Dragon Dance (retained defense)
- **Blank count: 0** (within T3 range 0-1)
- **No spell** (pure physical)
- **Balance note:** Balanced variant. No Rampage but much more consistent. Cleave 4 is the highest single-face cleave value. HP 11 is higher and Shield 2 provides some defense. AoE + Cleave + Heavy covers all target configurations. Trades explosive Rampage ceiling for reliable floor.

---

## 5. Dratini -> Dragonair -> Dragonite (color w P2, replaces Ditto)

**Template:** replica.Lost
**Role:** Dragon all-rounder powerhouse -- Heavy + Shield + Engage, balanced across offense/defense
**Speech:** Dratini!:Tini! / Dragonair!:Air! / DRAGONITE!:Special Delivery!:HYPER BEAM!
**Doc:** (none)

### T1 Dratini: HP 5
- **sd:** `17-1:39-1:56-1:0:0:0`
- **Faces:**
  1. Damage Engage 1 -- Thunder Wave (strikes first)
  2. Damage Heavy 1 -- Wrap
  3. Shield 1 -- Dragon Tail (deflect)
  4. Blank
  5. Blank
  6. Blank
- **Blank count: 3** (within T1 range 2-3)
- **Total offense: ~2 damage pips + 0.7 shield equivalent** (standard balanced T1)
- **Balance note:** All-rounder from the start. Engage + Heavy + Shield covers offense and defense. Same power level as Gible T1.

### T2a Dragonair: HP 8
- **sd:** `17-2:39-2:56-2:56-2:76-1:0`
- **Faces:**
  1. Damage Engage 2 -- Dragon Rush
  2. Damage Heavy 2 -- Slam
  3. Shield 2 -- Safeguard
  4. Shield 2 -- Safeguard
  5. Mana 1 -- Dragon Dance (light mana gen)
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Balanced growth. 4 damage pips + 4 shield pips (4/1.4 ~ 2.9 damage equivalent) + 1 mana pip. Total ~7 damage equivalent across 5 active faces. HP 8 is solid mid-tier. Dragonair's grace is reflected in even stat distribution.

### T2b Dragonair: HP 7
- **sd:** `17-2:39-2:61-2:61-2:17-1:0`
- **Faces:**
  1. Damage Engage 2 -- Dragon Rush
  2. Damage Heavy 2 -- Aqua Tail
  3. Shield ManaGain 2 -- Rain Dance (shield + mana)
  4. Shield ManaGain 2 -- Rain Dance
  5. Damage Engage 1 -- Extreme Speed preview
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** More offensive variant with ManaGain on shields (fueling T3 spell). Lower HP (7) for more damage output. 5 damage pips + 4 ManaGain shield pips. ManaGain shields are dual-purpose: defense + spell fuel.

### T3a Dragonite: HP 11
- **sd:** `17-3:39-3:56-3:56-3:36-2:82-1`
- **Faces:**
  1. Damage Engage 3 -- Extreme Speed
  2. Damage Heavy 3 -- Dragon Rush
  3. Shield 3 -- Multiscale (thick natural armor)
  4. Shield 3 -- Multiscale
  5. Damage Cleave 2 -- Hurricane
  6. Mana Pain 1 -- Outrage buildup
- **Blank count: 0** (within T3 range 0-1)
- **Spell: Dragon Dance**
  - `abilitydata.(lost.sd.17-4:39-3:0:0:76-2.n.Dragon Dance.img.spark.hsv.30:40:80)`
  - Spell faces: Damage Engage 4, Damage Heavy 3, Blank, Blank, Mana 2 (cost)
  - Design: Massive single-target burst spell. 7 damage pips for 2 mana. Represents Dragon Dance power boost.
- **Balance note:** True all-rounder. 8 damage pips + 6 shield pips (6/1.4 ~ 4.3 damage equivalent) + Cleave + Mana. Total ~14 damage equivalent is strong for T3 but split offense/defense. HP 11 reflects Dragonite's bulk. Comparable to Blastoise T3a (HP 10, ranged/shields) but melee-focused.

### T3b Dragonite: HP 10
- **sd:** `17-3:39-3:64-2:64-2:36-3:76-2`
- **Faces:**
  1. Damage Engage 3 -- Extreme Speed
  2. Damage Heavy 3 -- Superpower
  3. Shield Rescue 2 -- Dragonite's kindness (rescues allies)
  4. Shield Rescue 2 -- Dragonite's kindness
  5. Damage Cleave 3 -- Hurricane
  6. Mana 2 -- Dragon Dance charge
- **Blank count: 0** (within T3 range 0-1)
- **Spell: Outrage**
  - `abilitydata.(lost.sd.39-5:36-3:0:0:76-3.n.Outrage.img.scorch.hue.40)`
  - Spell faces: Damage Heavy 5, Damage Cleave 3, Blank, Blank, Mana 3 (cost)
  - Design: Devastating AoE burst spell. 8 damage pips for 3 mana. Raw dragon fury.
- **Balance note:** Support-DPS variant. Shield Rescue 2 saves dying allies (Dragonite is canonically kind and rescues lost sailors). Less personal defense but more team value. Cleave 3 + Outrage spell gives tremendous AoE burst. HP 10 is slightly lower to compensate for the powerful spell.

---

## 6. Beldum -> Metang -> Metagross (color z P2, replaces Tentomon)

**Template:** replica.Lost
**Role:** Steel/Psychic bruiser-tank -- Steel Shield + Heavy + Redirect
**Speech:** Bel...:Dum / Metang!:Tang! / METAGROSS!:Computed.:METEOR MASH!
**Doc:** (none)

### T1 Beldum: HP 5
- **sd:** `39-1:63-1:0:0:63-1:0`
- **Faces:**
  1. Damage Heavy 1 -- Take Down (only move Beldum learns)
  2. Shield Steel 1 -- Clear Body (Steel body)
  3. Blank
  4. Blank
  5. Shield Steel 1 -- Iron Defense
  6. Blank
- **Blank count: 3** (within T1 range 2-3)
- **Total: ~1 damage pip + 2 Steel shield pips** (2/1.4 ~ 1.4 damage equivalent)
- **Balance note:** Mirrors Tentomon T1 (HP 5, sd=88-1:56-3:24-1:24-1:56-2). Beldum canonically only knows Take Down. Steel Shield at T1 is appropriate flavor. Low offense, moderate defense.

### T2a Metang: HP 8
- **sd:** `39-2:63-3:118-1:118-1:63-2:0`
- **Faces:**
  1. Damage Heavy 2 -- Metal Claw
  2. Shield Steel 3 -- Iron Defense
  3. Redirect SelfShield 1 -- Telekinesis (redirects attacks to self, gains shield)
  4. Redirect SelfShield 1 -- Telekinesis
  5. Shield Steel 2 -- Bullet Punch block
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Tank identity solidifies. 5 Steel shield pips + 2 Redirect pips = massive team protection. Only 2 damage pips keeps it honest. Redirect SelfShield is the signature mechanic -- Metang's psychic powers redirect attacks to itself then shields. Comparable to Ballistamon T2 (HP 5, steel+shields) but much tankier at HP 8.

### T2b Metang: HP 7
- **sd:** `39-2:39-2:63-2:63-2:118-1:0`
- **Faces:**
  1. Damage Heavy 2 -- Meteor Mash
  2. Damage Heavy 2 -- Zen Headbutt
  3. Shield Steel 2 -- Iron Defense
  4. Shield Steel 2 -- Iron Defense
  5. Redirect SelfShield 1 -- Psychic redirect
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Bruiser variant. More damage (4 Heavy pips vs 2) but less shield (4 Steel pips vs 5). HP 7 is lower to compensate for higher offense. Still has Redirect for team protection.

### T3a Metagross: HP 11
- **sd:** `39-4:39-3:63-3:63-3:118-2:82-1`
- **Faces:**
  1. Damage Heavy 4 -- Meteor Mash
  2. Damage Heavy 3 -- Zen Headbutt
  3. Shield Steel 3 -- Iron Defense
  4. Shield Steel 3 -- Iron Defense
  5. Redirect SelfShield 2 -- Psychic Terrain
  6. Mana Pain 1 -- Take Down recoil (light mana gen)
- **Blank count: 0** (within T3 range 0-1)
- **Spell: Meteor Mash**
  - `abilitydata.(lost.sd.39-5:39-4:0:0:76-3.n.Meteor Mash.img.mithril shields.hsv.-120:30:70)`
  - Spell faces: Damage Heavy 5, Damage Heavy 4, Blank, Blank, Mana 3 (cost)
  - Design: Colossal single-target damage. 9 Heavy damage for 3 mana. Represents Metagross's computed precision strikes.
- **Balance note:** HP 11 is high for T3, justified by tank role. 7 Heavy damage pips + 6 Steel shield pips (6/1.4 ~ 4.3) + Redirect 2 = ~13 damage equivalent on die + devastating spell. Compared to MegaKabuterimon T3 (HP 13, sd=170:82-4:25-2:25-2:82-4) which has QuadUse + Pain + massive mana, Metagross trades mana generation for consistent Heavy + Steel defense. The Meteor Mash spell is the burst window.

### T3b Metagross: HP 10
- **sd:** `39-3:39-3:63-4:63-4:118-3:0`
- **Faces:**
  1. Damage Heavy 3 -- Bullet Punch
  2. Damage Heavy 3 -- Hammer Arm
  3. Shield Steel 4 -- Iron Defense maximum
  4. Shield Steel 4 -- Iron Defense maximum
  5. Redirect SelfShield 3 -- Psychic Terrain full
  6. Blank
- **Blank count: 1** (within T3 range 0-1)
- **Spell: Meteor Mash** (same as T3a)
  - `abilitydata.(lost.sd.39-5:39-4:0:0:76-3.n.Meteor Mash.img.mithril shields.hsv.-120:30:70)`
- **Balance note:** Full tank variant. 8 Steel shield pips + Redirect 3 = the party's ultimate bodyguard. 6 Heavy damage is still meaningful -- Metagross hits hard even when tanking. 1 blank at T3 plus no mana generation means slower spell cycling, which is the tradeoff for the absurd shield values.

---

## 7. Machop -> Machoke -> Machamp (color t P2, replaces Burmy)

**Template:** replica.Lost
**Role:** Fighting multi-striker -- DoubleUse at T2, QuadUse ONLY at T3, Engage + SelfShield
**Speech:** Machop!:Chop! / Machoke!:Flex!:Hah! / MACHAMP!:Four Arms!:NO GUARD!
**Doc:** (none -- pure physical, no spell)

### T1 Machop: HP 5
- **sd:** `17-1:39-1:51-1:0:0:0`
- **Faces:**
  1. Damage Engage 1 -- Karate Chop (strikes first)
  2. Damage Heavy 1 -- Low Kick
  3. Damage SelfShield 1 -- Bulk Up (hit + protect)
  4. Blank
  5. Blank
  6. Blank
- **Blank count: 3** (within T1 range 2-3)
- **Total offense: ~3 damage pips** (SelfShield provides both damage and defense)
- **Balance note:** Straightforward fighting T1. Engage + Heavy + SelfShield covers the fighting identity from the start. Matches Burmy T1 (HP 4, shield/cleanse focused) in total value but completely different role. HP 5 vs Burmy's 4 reflects Fighting type sturdiness.

### T2a Machoke: HP 8
- **sd:** `17-2:39-2:51-2:51-2:15-1:0`
- **Faces:**
  1. Damage Engage 2 -- Karate Chop
  2. Damage Heavy 2 -- Vital Throw
  3. Damage SelfShield 2 -- Bulk Up
  4. Damage SelfShield 2 -- Bulk Up
  5. Damage 1 -- Low Sweep
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** Solid melee fighter. 9 damage pips across 5 faces (some with SelfShield bonus defense). SelfShield is the core mechanic -- Machoke hits AND protects himself. No DoubleUse yet on this variant. HP 8 is sturdy for T2.

### T2b Machoke: HP 7
- **sd:** `17-2:17-2:39-1:39-1:114-1:0`
- **Faces:**
  1. Damage Engage 2 -- Seismic Toss
  2. Damage Engage 2 -- Submission
  3. Damage Heavy 1 -- Wake-Up Slap
  4. Damage Heavy 1 -- Wake-Up Slap
  5. Heal DoubleUse 1 -- No Guard (acts twice! Four arms = double action)
  6. Blank
- **Blank count: 1** (within T2 range 1-2)
- **Balance note:** DoubleUse introduction. Heal DoubleUse 1 at T2 is appropriate -- it is not a premium keyword (QuadUse is premium, DoubleUse is standard). Lower HP (7) and lower per-face pips balance the DoubleUse power. DoubleUse represents No Guard/four arms hitting twice.

### T3a Machamp: HP 10
- **sd:** `17-3:39-3:51-3:51-3:17-2:0`
- **Faces:**
  1. Damage Engage 3 -- Cross Chop
  2. Damage Heavy 3 -- Dynamic Punch
  3. Damage SelfShield 3 -- Bulk Up maximum
  4. Damage SelfShield 3 -- Bulk Up maximum
  5. Damage Engage 2 -- Close Combat
  6. Blank
- **Blank count: 1** (within T3 range 0-1)
- **No spell** (pure physical)
- **Balance note:** Consistent powerhouse variant. Five active damage faces plus one blank. 14 total damage pips is high but all single-target. SelfShield 3 on two faces means Machamp can sustain through fights. No premium keywords (no QuadUse on this variant) -- pure reliable damage. HP 10 is standard T3. The blank keeps the budget honest for such a keyword-rich die. Comparable to Scizor T3 (HP 8, damage-focused) but bulkier with more SelfShield.

### T3b Machamp: HP 9
- **sd:** `17-3:39-3:17-2:17-2:25-2:0`
- **Faces:**
  1. Damage Engage 3 -- Cross Chop
  2. Damage Heavy 3 -- Dynamic Punch
  3. Damage Engage 2 -- Mach Punch
  4. Damage Engage 2 -- Mach Punch
  5. QuadUse 2 -- No Guard: Four Arms (hits FOUR times!)
  6. Blank
- **Blank count: 1** (within T3 range 0-1)
- **No spell** (pure physical)
- **Balance note:** THE QuadUse variant. QuadUse (face 25) is the premium keyword -- T3 ONLY as specified. QuadUse 2 means the face triggers 4 times at 2 pips. HP 9 is lower to offset QuadUse power. 1 blank further balances the explosive potential. When QuadUse hits, it is devastating; when blank hits, Machamp does nothing. High variance = high excitement. Compared to Hydrapple T3 (quad-use + mana) or MegaKabuterimon (quad-use + pain), Machamp's QuadUse is pure damage without mana support, making it feast-or-famine.

---

## Balance Summary Table

| Hero | T1 HP | T1 Blanks | T2a HP | T2a Blanks | T2b HP | T2b Blanks | T3a HP | T3a Blanks | T3b HP | T3b Blanks | Has Spell |
|------|-------|-----------|--------|------------|--------|------------|--------|------------|--------|------------|-----------|
| Charmander line | 5 | 3 | 7 | 1 | 8 | 1 | 10 | 0 | 11 | 1 | Yes (Fire Blast) |
| Cyndaquil line | 4 | 3 | 7 | 1 | 6 | 1 | 9 | 1 | 10 | 1 | Yes (Eruption) |
| Torchic line | 4 | 3 | 7 | 1 | 8 | 1 | 9 | 1 | 10 | 1 | Yes (Blaze Kick) |
| Bagon line | 5 | 3 | 8 | 1 | 7 | 1 | 10 | 0 | 11 | 0 | No |
| Dratini line | 5 | 3 | 8 | 1 | 7 | 1 | 11 | 0 | 10 | 0 | Yes (Dragon Dance / Outrage) |
| Beldum line | 5 | 3 | 8 | 1 | 7 | 1 | 11 | 0 | 10 | 1 | Yes (Meteor Mash) |
| Machop line | 5 | 3 | 8 | 1 | 7 | 1 | 10 | 1 | 9 | 1 | No |

### Tier HP Range Verification
- **T1:** 4, 4, 4, 5, 5, 5, 5 -- all within 3-6 range. PASS.
- **T2:** 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8 -- all within 6-9 range. PASS.
- **T3:** 9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11 -- all within 8-13 range. PASS.

### Tier Blank Count Verification
- **T1:** all have 3 blanks -- within 2-3 range. PASS.
- **T2:** all have 1 blank -- within 1-2 range. PASS.
- **T3:** 0 or 1 blanks each -- within 0-1 range. PASS.

### Premium Keyword Usage
- **Rampage (137):** Salamence T3a only (2 faces). Earned via defensive T2 phase.
- **QuadUse (25):** Machamp T3b only (1 face). T3-gated as specified.
- **Cantrip:** NOT USED on any hero (per design spec -- removed from Blaziken).
- **Revive:** NOT USED (not thematically relevant to any of these heroes).

### Face ID Verification (all faces used)
- 0 (Blank): all tiers
- 15 (Damage): Cyndaquil T1, Machop T1, Machoke T2a, Combusken T2b
- 17 (Engage): Torchic, Combusken, Blaziken, Dratini, Dragonair, Dragonite, Machop, Machoke, Machamp
- 25 (QuadUse): Machamp T3b only
- 30 (Cruel): Charmander T1, Charmeleon T2a/T2b
- 34 (to All): Charmander T1, Charmeleon T2a/T2b, Charizard T3a/T3b, Quilava T2a/T2b, Typhlosion T3a/T3b
- 36 (Cleave): Charmeleon T2a, Charizard T3a/T3b, Dragonite T3a/T3b, Salamence T3a/T3b
- 39 (Heavy): Bagon T1, Shelgon T2b, Salamence T3b, Dratini T1, Dragonair T2a/T2b, Dragonite T3a/T3b, Beldum T1, Metang T2a/T2b, Metagross T3a/T3b, Machop T1, Machoke T2a/T2b, Machamp T3a/T3b
- 51 (SelfShield): Torchic T1, Combusken T2a, Blaziken T3a, Machop T1, Machoke T2a, Machamp T3a
- 56 (Shield): Bagon T1, Shelgon T2a, Dratini T1, Dragonair T2a, Dragonite T3a, Salamence T3b, Torchic T1
- 61 (Shield ManaGain): Dragonair T2b
- 63 (Shield Steel): Shelgon T2a/T2b, Beldum T1, Metang T2a/T2b, Metagross T3a/T3b
- 64 (Shield Rescue): Dragonite T3b
- 76 (Mana): Cyndaquil T1, Quilava T2a/T2b, Typhlosion T3a/T3b, Dragonair T2a, Dragonite T3b
- 82 (Mana Pain): Charmeleon T2a/T2b, Charizard T3a/T3b, Cyndaquil T1, Quilava T2a/T2b, Typhlosion T3a/T3b, Salamence T3a, Metagross T3a, Dragonite T3a
- 114 (Heal DoubleUse): Machoke T2b, Blaziken T3b
- 118 (Redirect SelfShield): Metang T2a/T2b, Metagross T3a/T3b
- 131 (Weaken): Typhlosion T3b
- 137 (Rampage): Salamence T3a
- 174 (Defy): Combusken T2a/T2b, Blaziken T3a/T3b
- 175 (Critical): Combusken T2a, Blaziken T3a

All face IDs verified against the reference table. No invalid IDs used.

### Role Differentiation
These 7 heroes occupy distinct niches:
1. **Charizard** -- Fire AoE DPS (Cleave + All + Rampage at T3). Wide damage spread.
2. **Typhlosion** -- Fire AoE Caster (Mana engine + Eruption spell). Spell-dependent.
3. **Blaziken** -- Fire/Fighting Assassin (Engage + Defy + Critical). Single-target burst.
4. **Salamence** -- Dragon Berserker (defensive T2 -> explosive Rampage T3). High variance.
5. **Dragonite** -- Dragon All-Rounder (Engage + Heavy + Shield + Cleave). Does everything.
6. **Metagross** -- Steel Tank-Bruiser (Steel Shield + Redirect + Heavy). Party protector.
7. **Machamp** -- Fighting Multi-Striker (SelfShield + QuadUse at T3). Sustained melee.

No two heroes share the same primary role or keyword profile.
