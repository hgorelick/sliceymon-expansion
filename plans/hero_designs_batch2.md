# Hero Designs — Batch 2 (7 Evolution Lines)

## Balance Reference Summary
- T1: HP 3-6, 2-3 blanks, 1-2 pip values per active face
- T2: HP 6-9, 1-2 blanks, 2-3 pip values per active face
- T3: HP 8-13, 0-1 blanks, 3-6 pip values per active face
- Spells: 5 faces in abilitydata (not 6)
- 1 damage pip = 1.4 shield pips in value
- Face position: Strongest face on bottom (position 6) to protect from petrify

## Key Comparisons Used
- **Scyther T1**: HP 5, sd=29-2:29-2:0:0:171-1:0 (2 dmg, 1 cleave, 3 blanks)
- **Kleavor T3**: HP 10, sd=17-3:30-3:175-3:175-3:17-2:30-2 (6 active faces, engage+cruel+critical)
- **Ralts T1**: HP 3, sd=85-1:124:84-1:84-1:0:0 (mana trio + dodge cantrip + 2 blanks)
- **Gardevoir T3**: HP 8, sd=85-3:85-2:108-1:108-1:134-2:130 (mana trio + heal + weaken + reuse)
- **Happiny T1**: HP 9 (healer baseline), sd=41-3:103-4:111-1:111-1:103-2:103-2
- **Blissey T3a**: HP 12, sd=110-7:106-3:109-3:109-3:106-2:0 (1 blank even at T3 — healer budget)
- **Trubbish T1**: HP 5, sd=54-1:119-1:0:0:145:0 (2 active + poison add + 3 blanks)
- **Garbodor T3a**: HP 13, sd=55-0:91-2:62-4:62-4:0 (high HP, poison, 1 blank)
- **Sunkern T1**: HP 6, sd=79-2:113-2:112-1:112-1:113-1:151 (growth mana + heal growth + add growth)
- **Varoom T1**: HP 5, sd=53-1:15-1:17-1:17-1 (4 active from Statue template)
- **Roggenrola T1**: HP 5, sd=160-3:118-2:133-1:133-1:65-2 (redirect + shield pristine)
- **Magikarp T1**: HP 5, sd=96-1:6:6:6:6:6 (1 face + 5 stasis — extreme blank pattern)

---

## 1. Treecko -> Grovyle -> Sceptile (color l P2, replaces Sunkern)

**Template**: Primrose
**Speech**: "Treecko!" / "Grovyle!" / "Sceptile!"
**Doc**: (inherits Primrose growth mechanic — gains +1 pip to sides over time)

**Role**: Grass physical DPS with crits. Fast hard-hitting blade attacker with Growth scaling. NOT a caster — the Growth mechanic via Primrose template + Add Growth (151) face provides scaling damage, not spell support. Treecko's speed translates to Engage (17); blade attacks translate to Critical (175).

**Design rationale**: Replaces Sunkern's supportive growth role with offensive growth. Sunkern grew heals; Sceptile grows damage. Same Primrose template, completely different identity. Competes with Kleavor (critical DPS) but differentiates via Growth scaling — Sceptile starts weaker but scales harder over long fights.

### T1 Treecko: HP 5
- sd: 17-1:175-1:0:0:113-1:151
- Faces:
  1. Damage Engage 1 — Quick Attack, strikes first
  2. Damage Critical 1 — Leaf Blade training, lucky hits
  3. Blank — still a hatchling
  4. Blank — still learning
  5. Heal Growth 1 — Absorb, minor sustain that scales
  6. Add Growth — seeds the growth engine (Primrose signature)

**Budget check**: 2 blanks, 3 active + 1 utility. HP 5 is standard T1 DPS. Damage output low (1-1 pips) but Growth means it scales. Comparable to Scyther T1 (HP 5, 2 dmg faces + 1 cleave + 3 blanks) but trades immediate damage for growth potential.

### T2a Grovyle: HP 7
- sd: 17-2:175-2:113-2:113-2:0:151
- Faces:
  1. Damage Engage 2 — Leaf Blade slash, strikes first
  2. Damage Critical 2 — Leaf Blade, crits emerge
  3. Heal Growth 2 — Mega Drain sustain
  4. Heal Growth 2 — Giga Drain sustain
  5. Blank — still evolving
  6. Add Growth — continues growth engine

**Budget check**: 1 blank, 4 active + 1 utility. HP 7 is mid-range T2. The Growth heals keep it alive while damage scales. Comparable to Scyther T2a (HP 8, sd=29-4:29-4:29-3:29-3:171-1:171-1) — Grovyle has less raw damage but Growth upside.

### T2b Grovyle: HP 6
- sd: 17-2:175-2:17-1:17-1:0:151
- Faces:
  1. Damage Engage 2 — Quick Attack
  2. Damage Critical 2 — Leaf Blade
  3. Damage Engage 1 — Pursuit slash
  4. Damage Engage 1 — Pursuit slash
  5. Blank
  6. Add Growth — growth engine

**Budget check**: All-in on damage variant. Lower HP (6) + no heals = glass cannon with growth. 1 blank. More aggressive than T2a but less sustain.

### T3a Sceptile: HP 9
- sd: 17-3:175-3:175-3:175-3:113-2:151
- Faces:
  1. Damage Engage 3 — Leaf Blade, strikes first with authority
  2. Damage Critical 3 — Leaf Blade crit
  3. Damage Critical 3 — Night Slash crit
  4. Damage Critical 3 — X-Scissor crit
  5. Heal Growth 2 — Giga Drain sustain
  6. Add Growth — final growth engine

**Spell**: Leaf Blade — abilitydata: (Fey.sd.175-4:175-3:0-0:0-0:17-2.i.left.k.focus.img.slice.hsv.40:0:0.n.Leaf Blade)
- Spell faces: Critical 4, Critical 3, Blank, Blank, Engage 2 (5 faces)

**Budget check**: 0 blanks, HP 9. Six active faces with 3 criticals is very strong but offset by no shields/self-protection. Compare Kleavor T3 (HP 10, sd=17-3:30-3:175-3:175-3:17-2:30-2) — Kleavor has Cruel for conditional bonus; Sceptile has Growth scaling + Heal Growth for sustain. Different DPS flavors: Kleavor = burst, Sceptile = scaling. Spell adds a mana-cost burst option.

### T3b Sceptile: HP 8
- sd: 17-4:175-4:30-2:30-2:17-2:151
- Faces:
  1. Damage Engage 4 — Leaf Storm, massive first strike
  2. Damage Critical 4 — Leaf Blade maximum power
  3. Damage Cruel 2 — Pursuit, finisher instinct
  4. Damage Cruel 2 — Pursuit, finisher instinct
  5. Damage Engage 2 — Quick Attack follow-up
  6. Add Growth — growth engine

**Budget check**: 0 blanks, HP 8 (lower for pure glass cannon). No heals at all — pure damage variant. The Cruel faces reward finishing wounded enemies. Growth still scales all faces. Compare Kleavor T3 (HP 10) — T3b Sceptile trades HP for higher peak pips (4 engage, 4 critical).

---

## 2. Chikorita -> Bayleef -> Meganium (color l P1, replaces Fomantis)

**Template**: replica.Lost
**Speech**: "Chiko!" / "Bay!" / "Mega!"
**Doc**: none

**Role**: Grass party healer/buffer. Pure AoE sustain support. Heal All (107) provides team-wide healing; Shield Cleanse (71) removes debuffs; Heal Vitality (105) provides efficient single-target heals. The gentle Grass-type medic. Distinct from Happiny/Blissey (which is targeted rescue/critical healing) — Chikorita spreads moderate healing to everyone.

**Design rationale**: Replaces Fomantis's exert/growth DPS identity with pure support. Competes with Blissey (r P2) but differentiates: Blissey = big single-target heals + Rescue; Meganium = moderate team-wide heals + Cleanse. Meganium is the AoE healer, Blissey is the clutch healer.

### T1 Chikorita: HP 4
- sd: 107-1:103-1:0:0:71-1:0
- Faces:
  1. Heal All 1 — Synthesis aura, heals the whole party for 1
  2. Heal Basic 1 — Razor Leaf graze (minor heal flavor)
  3. Blank
  4. Blank
  5. Shield Cleanse 1 — Aromatherapy preview, removes a debuff
  6. Blank

**Budget check**: 3 blanks, 3 active. HP 4 is low for a healer (Happiny is 9) but Chikorita is a Grass starter, not a dedicated nurse — it grows into the role. Heal All 1 is the signature. Compare Fomantis T1 (HP 4, sd=23-3:113-1:16-1:16-1:114-1 — 5 faces, damage-oriented). Chikorita is weaker early but has clearer healer identity.

### T2a Bayleef: HP 7
- sd: 107-2:105-2:71-2:71-2:0:103-2
- Faces:
  1. Heal All 2 — Synthesis, team heal
  2. Heal Vitality 2 — Mega Drain, efficient single heal
  3. Shield Cleanse 2 — Safeguard
  4. Shield Cleanse 2 — Aromatherapy
  5. Blank
  6. Heal Basic 2 — Giga Drain

**Budget check**: 1 blank, 5 active. HP 7 mid-range T2. Strong support spread. Compare Chansey T2a (HP 9, sd=105-3:39-3:109-2:109-2 — 4 faces, no blanks in 4-face format). Bayleef has more face variety but lower individual pips.

### T2b Bayleef: HP 8
- sd: 107-2:105-3:0:0:71-2:105-2
- Faces:
  1. Heal All 2 — Synthesis
  2. Heal Vitality 3 — stronger single-target focus
  3. Blank
  4. Blank
  5. Shield Cleanse 2 — Safeguard
  6. Heal Vitality 2 — Giga Drain

**Budget check**: 2 blanks but higher HP (8) and higher pip values. Tankier but less consistent variant. Trades T2a's extra active faces for survivability + bigger heal spikes.

### T3a Meganium: HP 10
- sd: 107-3:105-3:71-3:71-3:105-2:103-3
- Faces:
  1. Heal All 3 — Synthesis, full team heal
  2. Heal Vitality 3 — Aromatherapy burst heal
  3. Shield Cleanse 3 — Safeguard, cleanse + protect
  4. Shield Cleanse 3 — Light Screen, cleanse + protect
  5. Heal Vitality 2 — Giga Drain
  6. Heal Basic 3 — Mega Drain

**Spell**: Aromatherapy — abilitydata: (Fey.sd.111-3:107-2:0-0:0-0:107-2.img.sprout.hsv.40:0:0.n.Aromatherapy)
- Spell faces: Heal Cleanse 3, Heal All 2, Blank, Blank, Heal All 2 (5 faces)

**Budget check**: 0 blanks, HP 10. Six heal/shield faces — Meganium heals every single roll. But all healing, no damage — pure support. Compare Blissey T3a (HP 12, sd=110-7:106-3:109-3:109-3:106-2:0) — Blissey has higher pips and 1 blank but focuses on Regen/Rescue for clutch saves. Meganium has Heal All for team spread + Cleanse for debuff removal. Different healer niches. Spell provides emergency team cleanse+heal.

### T3b Meganium: HP 9
- sd: 107-3:107-2:105-4:105-4:71-3:112-2
- Faces:
  1. Heal All 3 — Synthesis
  2. Heal All 2 — Petal Dance aura
  3. Heal Vitality 4 — massive single-target heal
  4. Heal Vitality 4 — massive single-target heal
  5. Shield Cleanse 3 — Safeguard
  6. Heal ManaGain 2 — Ancient Power, heals and gains mana for spells

**Budget check**: 0 blanks, HP 9. More offensive healing variant — the ManaGain face enables spell use. Higher Vitality pips (4) for huge single heals. Trades T3a's consistency (6 spread faces) for spike healing + mana generation.

---

## 3. Bulbasaur -> Ivysaur -> Venusaur (color p P2, replaces Trubbish)

**Template**: replica.Statue
**Speech**: "Bulba!" / "Ivy!" / "Venusaur!"
**Doc**: "Start Poisoned"

**Role**: Grass/Poison sustain tank. Starts poisoned (like Trubbish) but uses Poison Plague (55) to spread poison to enemies while self-sustaining with Heal Regen (110) and Shield Repel (119). Leech Seed fantasy — wears enemies down passively while refusing to die. Solar Beam spell as a charged finisher.

**Design rationale**: Replaces Trubbish's poison tank identity with a more offensive poison sustain design. Trubbish was a passive poison sponge; Venusaur is an active poison spreader who heals through the poison. Same replica.Statue template, same "Start Poisoned" doc. Competes with Duskull/Dusknoir (p P1) but Dusknoir is a Ghost utility tank; Venusaur is a Poison sustain bruiser.

### T1 Bulbasaur: HP 5
- sd: 53-1:110-1:0:0:145:0
- Faces:
  1. Damage Poison 1 — Poison Powder, poisons the target
  2. Heal Regen 1 — Leech Seed, regenerating heal
  3. Blank
  4. Blank
  5. Add Poison — Toxic Spikes setup, inflicts poison
  6. Blank

**Budget check**: 3 blanks, 2 active + 1 utility. HP 5 standard. Compare Trubbish T1 (HP 5, sd=54-1:119-1:0:0:145:0) — nearly identical structure. Bulbasaur trades Trubbish's shield for Regen heal (Leech Seed) and Poison Damage for Damage Poison. Both have Add Poison.

### T2a Ivysaur: HP 8
- sd: 55-1:110-2:119-2:119-2:0:53-2
- Faces:
  1. Damage Poison Plague 1 — Toxic, spreads poison to adjacent enemies
  2. Heal Regen 2 — Leech Seed, sustained regeneration
  3. Shield Repel 2 — Vine Whip deflection
  4. Shield Repel 2 — Vine Whip deflection
  5. Blank
  6. Damage Poison 2 — Sludge Bomb

**Budget check**: 1 blank, 5 active. HP 8, matching Trubbish T2a (HP 8). Poison Plague appears at T2 (strong keyword — appropriate for mid-tier). Shield Repel provides tank capability. Balanced sustain tank spread.

### T2b Ivysaur: HP 7
- sd: 55-1:110-3:53-2:53-2:119-1:145
- Faces:
  1. Damage Poison Plague 1 — Toxic
  2. Heal Regen 3 — stronger Leech Seed sustain
  3. Damage Poison 2 — Sludge Bomb
  4. Damage Poison 2 — Sludge Bomb
  5. Shield Repel 1 — minor Vine Whip
  6. Add Poison — more poison spreading

**Budget check**: 0 blanks (5 active + Add Poison utility). HP 7 (lower). More aggressive poison variant — more Damage Poison faces, plus Add Poison for extra poison application. Trades HP and shield pips for damage output.

### T3a Venusaur: HP 11
- sd: 55-2:110-4:119-3:119-3:110-2:53-3
- Faces:
  1. Damage Poison Plague 2 — Toxic, devastating poison spread
  2. Heal Regen 4 — Leech Seed maximum sustain
  3. Shield Repel 3 — Vine Whip wall
  4. Shield Repel 3 — Vine Whip wall
  5. Heal Regen 2 — Synthesis backup heal
  6. Damage Poison 3 — Sludge Bomb

**Spell**: Solar Beam — abilitydata: (Statue.sd.42-5:15-3:0:0:15-3.i.left.k.focus.img.beam.hsv.40:0:0.n.Solar Beam)
- Spell faces: Damage Charged 5, Damage 3, Blank, Blank, Damage 3 (5 faces)
- Charged keyword means it hits harder with mana investment. Solar Beam fantasy = big charged finisher.

**Budget check**: 0 blanks, HP 11. Strong sustain tank with 2 Repel shields (4.2 damage equivalent at 3 pips each) + 2 Regen heals + Plague + Poison damage. Compare Garbodor T3a (HP 13, sd=55-0:91-2:62-4:62-4:0) — Garbodor has higher HP and some faces from Statue defaults; Venusaur has more active faces with better pips but lower HP. Solar Beam spell provides burst damage option rare for a tank.

### T3b Venusaur: HP 10
- sd: 55-3:110-3:53-3:53-3:119-2:110-3
- Faces:
  1. Damage Poison Plague 3 — Toxic maximum spread
  2. Heal Regen 3 — Leech Seed
  3. Damage Poison 3 — Sludge Bomb
  4. Damage Poison 3 — Venoshock
  5. Shield Repel 2 — Vine Whip
  6. Heal Regen 3 — Synthesis

**Budget check**: 0 blanks, HP 10. More aggressive variant — higher Plague pips (3!) for massive poison spread, more Damage Poison faces. Less shield but more healing and poison damage. T3b is the "offensive poison" Venusaur vs T3a's "defensive wall" Venusaur.

---

## 4. Mudkip -> Marshtomp -> Swampert (color g P2, replaces Varoom)

**Template**: replica.Statue
**Speech**: "Mudkip!" / "Marshtomp!" / "Swampert!"
**Doc**: none

**Role**: Water/Ground defensive anchor. Shield Rescue (64) saves dying allies; Redirect SelfShield (118) forces attacks onto Swampert and shields itself; Heavy (39) provides conditional big hits. The sturdy party protector who absorbs damage so teammates don't have to. No spell — pure physical.

**Design rationale**: Replaces Varoom's engine/poison DPS with a completely different identity — defensive anchor. Same replica.Statue template. Competes with Slakoth/Slaking (g P1) for color slot, but Slaking is an aggressive DPS with Pain drawback; Swampert is a protective tank. Perfect color pairing: one hits hard (Slaking), one protects the team (Swampert).

### T1 Mudkip: HP 6
- sd: 64-1:39-1:0:0:56-1:0
- Faces:
  1. Shield Rescue 1 — Protect, saves an ally at 1 HP
  2. Damage Heavy 1 — Tackle, conditional bonus vs high HP
  3. Blank
  4. Blank
  5. Shield Basic 1 — Mud-Slap, minor protection
  6. Blank

**Budget check**: 3 blanks, 3 active. HP 6 (slightly high for T1, justified as a tank — Larvitar is HP 8 at T1 as a tank). Compare Varoom T1 (HP 5, sd=53-1:15-1:17-1:17-1 — 4 active from Statue). Mudkip has fewer active faces but higher HP and Rescue is a premium support keyword.

### T2a Marshtomp: HP 8
- sd: 64-2:39-2:118-2:118-2:0:56-2
- Faces:
  1. Shield Rescue 2 — Protect
  2. Damage Heavy 2 — Mud Shot
  3. Redirect SelfShield 2 — Muddy Water, draws attacks + shields self
  4. Redirect SelfShield 2 — Muddy Water
  5. Blank
  6. Shield Basic 2 — Rock Slide defense

**Budget check**: 1 blank, 5 active. HP 8 solid T2 tank. Two Redirect faces = very reliable party protection. Compare Varoom T2 (HP 7, sd=17-2:17-1:28-1:17-1:181-5 — DPS-oriented). Marshtomp is a fundamentally different hero: defensive anchor.

### T2b Marshtomp: HP 7
- sd: 64-2:39-2:39-1:39-1:0:118-2
- Faces:
  1. Shield Rescue 2 — Protect
  2. Damage Heavy 2 — Earthquake
  3. Damage Heavy 1 — Rock Slide
  4. Damage Heavy 1 — Rock Slide
  5. Blank
  6. Redirect SelfShield 2 — Muddy Water

**Budget check**: 1 blank, 5 active. HP 7 (lower). More Heavy damage, less Redirect — bruiser variant. Still has 1 Rescue and 1 Redirect for core tanking, but 3 damage faces lean offensive.

### T3a Swampert: HP 11
- sd: 64-3:39-3:118-3:118-3:56-3:39-2
- Faces:
  1. Shield Rescue 3 — Protect, saves allies with a big shield
  2. Damage Heavy 3 — Earthquake
  3. Redirect SelfShield 3 — Muddy Water, absorbs big hits
  4. Redirect SelfShield 3 — Muddy Water, absorbs big hits
  5. Shield Basic 3 — Avalanche defense wall
  6. Damage Heavy 2 — Rock Slide

**No spell** — Swampert is pure physical.

**Budget check**: 0 blanks, HP 11. Very sturdy. Two Redirect 3 faces = reliable damage absorption. Shield Rescue 3 saves dying allies. Shield 3 for passive protection. Heavy 3 + Heavy 2 for damage output. Compare Slaking T3a (HP 12, sd=23-10:23-10:28-3:28-3:187-10) — Slaking is a damage monster with huge pips; Swampert is the opposite, a damage sponge. Shield value: (3+3+3)*1.4 = 12.6 damage-equivalent in shields, which is strong but appropriate for a 0-blank T3 tank.

### T3b Swampert: HP 10
- sd: 64-3:39-4:118-3:118-3:39-3:52-2
- Faces:
  1. Shield Rescue 3 — Protect
  2. Damage Heavy 4 — Earthquake, massive conditional hit
  3. Redirect SelfShield 3 — Muddy Water
  4. Redirect SelfShield 3 — Muddy Water
  5. Damage Heavy 3 — Hammer Arm
  6. Damage SelfHeal 2 — Waterfall, heals self on hit

**Budget check**: 0 blanks, HP 10. More offensive variant — Damage Heavy 4 is a huge face. SelfHeal provides sustain without dedicated shield faces. Two Redirect still anchors the tank role. Compare T3a: T3b trades Shield 3 + Heavy 2 for Heavy 4 + Heavy 3 + SelfHeal 2. More damage, less party protection.

---

## 5. Totodile -> Croconaw -> Feraligatr (color j P1, NEW)

**Template**: replica.Lost
**Speech**: "Totodile!" / "Croconaw!" / "Feraligatr!"
**Doc**: none

**Role**: Aggressive water berserker. Pure offense with Engage (17) + Cruel (30) + Heavy (39). Just bites things. No heals, no shields, no utility — raw aggression with big jaws. No spell — pure physical.

**Design rationale**: Brand new color j, P1. Paired with Poliwag (j P2). Totodile is the pure aggro pick — if you want maximum damage and don't care about survival, pick the croc. Competes with Garchomp (a P1) for "aggressive physical DPS" niche but differentiates: Garchomp has AoE cleave; Feraligatr has single-target burst via Cruel + Heavy.

### T1 Totodile: HP 5
- sd: 17-1:30-1:0:0:39-1:0
- Faces:
  1. Damage Engage 1 — Bite, strikes first
  2. Damage Cruel 1 — Scratch, bonus vs wounded
  3. Blank
  4. Blank
  5. Damage Heavy 1 — Water Gun, bonus vs high HP
  6. Blank

**Budget check**: 3 blanks, 3 active. HP 5 standard DPS T1. Three different damage keywords at 1 pip each — establishes the "many attack styles" identity early. Compare Gible T1 (HP 5, sd=170-3:158-1:158-1:158-1:43:0) — Gible has higher pips on fewer keywords; Totodile has lower pips on more diverse keywords. Both are 3-blank T1s.

### T2a Croconaw: HP 7
- sd: 17-2:30-2:39-2:39-2:0:17-2
- Faces:
  1. Damage Engage 2 — Ice Fang, strikes first
  2. Damage Cruel 2 — Crunch, finisher
  3. Damage Heavy 2 — Aqua Tail
  4. Damage Heavy 2 — Aqua Tail
  5. Blank
  6. Damage Engage 2 — Bite

**Budget check**: 1 blank, 5 active. HP 7 standard T2. Five damage faces at 2 pips — very aggressive. No shields or heals at all. Compare Scyther T2a (HP 8, sd=29-4:29-4:29-3:29-3:171-1:171-1 — 6 active, higher pips). Croconaw has less raw damage but more keyword variety (Engage/Cruel/Heavy each provide conditional bonuses).

### T2b Croconaw: HP 6
- sd: 17-2:30-2:30-1:30-1:0:39-2
- Faces:
  1. Damage Engage 2 — Bite
  2. Damage Cruel 2 — Crunch
  3. Damage Cruel 1 — Scratch
  4. Damage Cruel 1 — Scratch
  5. Blank
  6. Damage Heavy 2 — Aqua Tail

**Budget check**: 1 blank, 5 active. HP 6 (glass cannon). More Cruel-focused — excels at finishing wounded enemies. Lower HP forces a riskier playstyle. T2b is the "assassin" variant vs T2a's balanced aggression.

### T3a Feraligatr: HP 9
- sd: 17-3:30-3:39-3:39-3:17-2:30-2
- Faces:
  1. Damage Engage 3 — Ice Fang, big first strike
  2. Damage Cruel 3 — Crunch, devastating finisher
  3. Damage Heavy 3 — Aqua Tail
  4. Damage Heavy 3 — Waterfall
  5. Damage Engage 2 — Bite follow-up
  6. Damage Cruel 2 — Thrash

**No spell** — pure physical berserker.

**Budget check**: 0 blanks, HP 9. SIX damage faces — every single roll is an attack. Total raw pips = 3+3+3+3+2+2 = 16 before keywords. With Engage (first-strike bonus), Cruel (finisher bonus), and Heavy (bonus vs high HP), the conditional damage output is enormous. But HP 9 with zero shields/heals = paper tiger in hard content. Compare Kleavor T3 (HP 10, sd=17-3:30-3:175-3:175-3:17-2:30-2) — nearly identical structure! Feraligatr swaps Critical for Heavy. Heavy is better vs bosses (high HP targets), Critical is better for random spikes. Both are pure damage T3s at similar budgets.

### T3b Feraligatr: HP 10
- sd: 17-4:30-3:39-4:39-4:30-2:51-2
- Faces:
  1. Damage Engage 4 — Dragon Dance boosted Bite
  2. Damage Cruel 3 — Crunch
  3. Damage Heavy 4 — Earthquake
  4. Damage Heavy 4 — Superpower
  5. Damage Cruel 2 — Thrash
  6. Damage SelfShield 2 — Liquidation, shields self on hit

**Budget check**: 0 blanks, HP 10 (higher than T3a). Sacrifices some Engage for bigger Heavy hits (4 pips!). SelfShield face provides a touch of survivability. T3b is the "bulkier berserker" — still pure damage but can take a hit via SelfShield and higher HP.

---

## 6. Poliwag -> Poliwhirl -> Poliwrath / Politoed (color j P2, NEW)

**Template**: replica.Statue
**Speech**: "Poli!" / "Whirl!" / "Wrath!" or "Toed!"
**Doc**: none (Politoed T3b: "Rain Dance: +1 to all heals this fight")

**Role**: BRANCHING evolution. T1-T2 is a Water generalist. T3a Poliwrath = Water/Fighting bruiser (Heavy + Engage + SelfShield). T3b Politoed = Water support (Heal All + Shield ManaGain + Rain Dance spell). Two completely different end-states from the same base.

**Design rationale**: Brand new color j, P2. Paired with Totodile (j P1). Poliwag's branching evolution mirrors the games — King's Rock → Politoed, or level-up → Poliwrath. This creates a true draft decision: pick Poliwag and choose DPS bruiser or support healer later. Poliwrath competes with Feraligatr for damage but adds self-defense; Politoed competes with Meganium/Blissey for healing but adds mana support.

### T1 Poliwag: HP 4
- sd: 56-1:39-1:0:0:103-1:0
- Faces:
  1. Shield Basic 1 — Bubble, minor protection
  2. Damage Heavy 1 — Water Gun
  3. Blank
  4. Blank
  5. Heal Basic 1 — Rain Dance (minor heal)
  6. Blank

**Budget check**: 3 blanks, 3 active. HP 4 (low, it's a tadpole). Generalist spread — shield, damage, heal — hints at the branching future. Compare Varoom T1 (HP 5, sd=53-1:15-1:17-1:17-1). Poliwag is more defensive with shield+heal but less damage.

### T2a Poliwhirl: HP 7
- sd: 56-2:39-2:17-2:17-2:0:103-2
- Faces:
  1. Shield Basic 2 — Bubble Beam defense
  2. Damage Heavy 2 — Waterfall
  3. Damage Engage 2 — Brick Break, fighting preview
  4. Damage Engage 2 — Brick Break
  5. Blank
  6. Heal Basic 2 — Rain Dance heal

**Budget check**: 1 blank, 5 active. HP 7. Leans toward the Poliwrath (fighter) path with 2 Engage faces but keeps 1 heal. Balanced T2.

### T2b Poliwhirl: HP 8
- sd: 61-2:103-2:107-1:107-1:0:56-2
- Faces:
  1. Shield ManaGain 2 — Hypnosis focus, gains mana
  2. Heal Basic 2 — Water Pulse heal
  3. Heal All 1 — Rain Dance, team aura
  4. Heal All 1 — Rain Dance, team aura
  5. Blank
  6. Shield Basic 2 — Bubble Beam

**Budget check**: 1 blank, 5 active. HP 8. Leans toward the Politoed (support) path with Heal All and ManaGain. More defensive T2 option.

### T3a Poliwrath: HP 10
- sd: 17-3:39-3:51-3:51-3:39-2:17-2
- Faces:
  1. Damage Engage 3 — Dynamic Punch, strikes first
  2. Damage Heavy 3 — Waterfall
  3. Damage SelfShield 3 — Brick Break, shields self on impact
  4. Damage SelfShield 3 — Close Combat, shields self on impact
  5. Damage Heavy 2 — Earthquake
  6. Damage Engage 2 — Ice Punch

**No spell** — Poliwrath is pure physical fighter.

**Budget check**: 0 blanks, HP 10. Six damage faces with two SelfShield (3 pips each = 4.2 shield equivalent) for self-sustain. Compare Feraligatr T3a (HP 9, sd=17-3:30-3:39-3:39-3:17-2:30-2) — both are aggressive Water fighters. Poliwrath trades Cruel for SelfShield = less burst damage but better durability. This is the intended differentiation: Feraligatr = glass cannon, Poliwrath = bruiser.

### T3b Politoed: HP 9
- sd: 107-3:61-3:105-3:105-3:107-2:112-2
- Faces:
  1. Heal All 3 — Rain Dance, heals entire team
  2. Shield ManaGain 3 — Hypnosis focus, shields + generates mana
  3. Heal Vitality 3 — Perish Song sustain
  4. Heal Vitality 3 — Perish Song sustain
  5. Heal All 2 — Drizzle aura
  6. Heal ManaGain 2 — Helping Hand, heals + mana

**Spell**: Rain Dance — abilitydata: (Statue.sd.72-3:107-2:0:0:61-2.img.flick.hsv.-50:-30:0.n.Rain Dance)
- Spell faces: Shield to All 3, Heal All 2, Blank, Blank, Shield ManaGain 2 (5 faces)
- Rain Dance = team-wide protection + healing + mana generation. The ultimate support spell.

**Budget check**: 0 blanks, HP 9. Pure support — 4 heal faces + 2 Shield ManaGain. Compare Blissey T3a (HP 12, sd=110-7:106-3:109-3:109-3:106-2:0) — Blissey has higher HP, higher pips, and 1 blank. Politoed has lower stats but Heal All for team spread + ManaGain for spell support. Blissey saves one hero; Politoed sustains the whole team. Rain Dance spell adds team-wide shielding. Different enough from Meganium too: Meganium has Cleanse (debuff removal), Politoed has ManaGain (spell enabling).

---

## 7. Wailmer -> Wailord (color m P2, replaces Roggenrola)

**Template**: replica.Lost
**Speech**: "Wailmer!" / "Wailord!"
**Doc**: none

**Role**: Massive HP sponge. Highest HP in the mod at T3 (13-14). Shield to All (72) provides team protection; Heavy (39) provides respectable damage from the enormous whale. SPECIAL pattern: T1 has very high HP for its tier but many blank faces (like Magikarp pattern but with shields instead of stasis). Wailmer is immediately durable but does almost nothing — then Wailord becomes an immovable wall.

**Design rationale**: Replaces Roggenrola's "1 damage to all" gimmick with a pure HP sponge identity. Wailord IS the biggest Pokemon — it should have the biggest HP. The blank-heavy early game creates the Magikarp-style "invest now, payoff later" dynamic, but less extreme: Wailmer can at least shield (Magikarp had stasis blanks that did nothing).

**SPECIAL NOTE**: Wailmer is a 2-stage line (no middle evo in the games), but Slice & Dice needs T1/T2/T3 tiers. Solution: T1 = Wailmer (small), T2 = Wailmer (bigger), T3 = Wailord. The T2 "bigger Wailmer" is mechanically distinct even if it's the same Pokemon — it represents Wailmer growing before evolving.

### T1 Wailmer: HP 6
- sd: 72-1:56-1:0:0:0:0
- Faces:
  1. Shield to All 1 — Water Spout, protects entire team for 1
  2. Shield Basic 1 — Brine, minor self-protection
  3. Blank — enormous body, slow to act
  4. Blank — enormous body
  5. Blank — enormous body
  6. Blank — enormous body

**Budget check**: 4 blanks, 2 active. HP 6 (high for T1 — only Sunkern at 6 and Larvitar at 8 are comparable). The 4 blanks are deliberately punishing — this is the "investment" phase. Wailmer has the worst face ratio in T1 (4 blanks!) but compensates with Shield to All (a T3-tier keyword appearing at T1, albeit at 1 pip) and high HP. Compare Magikarp T1 (HP 5, sd=96-1:6:6:6:6:6 — 5 stasis blanks!). Wailmer is less extreme: 4 regular blanks (not stasis) and 2 usable faces.

### T2a Wailmer: HP 9
- sd: 72-2:56-2:39-2:39-2:0:0
- Faces:
  1. Shield to All 2 — Water Spout, team protection
  2. Shield Basic 2 — Brine defense
  3. Damage Heavy 2 — Body Slam
  4. Damage Heavy 2 — Rollout
  5. Blank — still growing
  6. Blank — still growing

**Budget check**: 2 blanks, 4 active. HP 9 (top of T2 range). Still blank-heavy for T2 (most T2s have 1 blank). The double blank is the ongoing "investment tax" — Wailmer is huge but still slow. Shield to All 2 is powerful team protection at T2.

### T2b Wailmer: HP 8
- sd: 72-2:56-3:39-2:39-2:56-1:0
- Faces:
  1. Shield to All 2 — Water Spout
  2. Shield Basic 3 — Brine, thick blubber
  3. Damage Heavy 2 — Body Slam
  4. Damage Heavy 2 — Rollout
  5. Shield Basic 1 — Dive
  6. Blank — still growing

**Budget check**: 1 blank, 5 active. HP 8 (trades HP for consistency). More shield-focused variant — 3 shield faces. Less extreme than T2a but lower HP.

### T3a Wailord: HP 14
- sd: 72-3:39-3:56-4:56-4:39-2:72-2
- Faces:
  1. Shield to All 3 — Water Spout, massive team protection
  2. Damage Heavy 3 — Body Slam from the largest Pokemon
  3. Shield Basic 4 — Dive, thick blubber wall
  4. Shield Basic 4 — Aqua Ring, thick blubber wall
  5. Damage Heavy 2 — Heavy Slam
  6. Shield to All 2 — Surf, team aura protection

**No spell** — Wailord doesn't need magic, it IS the wall.

**Budget check**: 0 blanks, HP 14 (!). This is the highest HP in the mod (Garbodor T3a is 13, Gyarados T3 is 13). The extreme HP is justified by the terrible T1-T2 curve (4 blanks at T1, 2 blanks at T2a). At T3, Wailord finally pays off: 2 Shield to All faces provide team protection, 2 Shield Basic 4 faces create a personal wall (4*1.4=5.6 damage equivalent each), and 2 Heavy faces provide respectable damage. Shield value: (3+4+4+2)*1.4 = 18.2 damage-equivalent in shields. This is very high but Wailord earned it with the worst early game. Compare Garbodor T3a (HP 13, 1 blank) — similar ceiling but Wailord has Shield to All (team utility) and higher HP. The investment payoff is clear: T1 Wailmer is one of the worst heroes in the game; T3 Wailord is one of the most durable.

### T3b Wailord: HP 13
- sd: 72-3:39-4:56-3:56-3:52-2:39-3
- Faces:
  1. Shield to All 3 — Water Spout
  2. Damage Heavy 4 — Body Slam, colossal impact
  3. Shield Basic 3 — Dive
  4. Shield Basic 3 — Aqua Ring
  5. Damage SelfHeal 2 — Waterfall, self-sustain
  6. Damage Heavy 3 — Heavy Slam

**Budget check**: 0 blanks, HP 13. More offensive variant — Heavy 4 is the biggest Heavy pip in the mod. SelfHeal provides sustain without dedicated heal faces. Only 1 Shield to All (vs T3a's 2) = less team protection, more personal output. T3b is the "bruiser whale" vs T3a's "wall whale."

---

## Design Verification Checklist

### Face ID Validation
All Face IDs used in this document are from the approved reference:
- 17 (Engage), 30 (Cruel), 39 (Heavy), 42 (Charged), 51 (SelfShield), 52 (SelfHeal), 53 (Poison), 55 (Poison Plague), 175 (Critical) -- Damage family
- 56 (Basic Shield), 61 (ManaGain Shield), 64 (Rescue Shield), 71 (Cleanse Shield), 72 (Shield to All), 119 (Repel Shield) -- Shield family
- 103 (Basic Heal), 105 (Vitality Heal), 107 (Heal All), 110 (Regen Heal), 111 (Cleanse Heal), 112 (ManaGain Heal), 113 (Growth Heal) -- Heal family
- 0 (Blank), 118 (Redirect SelfShield), 145 (Add Poison), 151 (Add Growth) -- Utility
- 15 (Basic Damage) -- used only in Solar Beam spell

### Tier Budget Summary

| Hero | T1 HP | T1 Blanks | T2 HP | T2 Blanks | T3 HP | T3 Blanks | Spell? |
|------|-------|-----------|-------|-----------|-------|-----------|--------|
| Treecko→Sceptile | 5 | 2 | 7/6 | 1/1 | 9/8 | 0/0 | Leaf Blade |
| Chikorita→Meganium | 4 | 3 | 7/8 | 1/2 | 10/9 | 0/0 | Aromatherapy |
| Bulbasaur→Venusaur | 5 | 3 | 8/7 | 1/0 | 11/10 | 0/0 | Solar Beam |
| Mudkip→Swampert | 6 | 3 | 8/7 | 1/1 | 11/10 | 0/0 | None |
| Totodile→Feraligatr | 5 | 3 | 7/6 | 1/1 | 9/10 | 0/0 | None |
| Poliwag→Poliwrath/Politoed | 4 | 3 | 7/8 | 1/1 | 10/9 | 0/0 | Rain Dance (Politoed) |
| Wailmer→Wailord | 6 | 4 | 9/8 | 2/1 | 14/13 | 0/0 | None |

### Role Uniqueness Check
- **Sceptile** (l P2): Grass physical crit DPS with Growth scaling. No other hero combines Critical + Growth.
- **Meganium** (l P1): Grass AoE healer with Cleanse. Distinct from Blissey (rescue/regen) and Politoed (mana support).
- **Venusaur** (p P2): Poison sustain tank. Only hero with Plague + Regen + Repel combo.
- **Swampert** (g P2): Water Redirect tank. Only Water-type that serves as party protector.
- **Feraligatr** (j P1): Pure physical berserker. 6 damage faces at T3 with zero support.
- **Poliwrath** (j P2 T3a): Water/Fighting bruiser with SelfShield. Bridges DPS and tank.
- **Politoed** (j P2 T3b): Water support healer with ManaGain. Enables spell-heavy teams.
- **Wailord** (m P2): Extreme HP sponge with Shield to All. Highest HP in the mod, worst T1.

### Premium Keyword Usage
- **Cantrip**: Not used (reserved for truly premium heroes)
- **Rampage**: Not used (reserved for dragons)
- **Revive**: Not used
- **QuadUse**: Not used
- **Critical**: Used on Sceptile only (appropriate — Leaf Blade is famous for high crit rate)
- **Poison Plague**: Used on Venusaur only (appropriate — Toxic/Venoshock identity)
- **Shield to All**: Used on Wailord (justified by terrible early game curve)
- **Redirect**: Used on Swampert (appropriate — party protector role)

### Power Budget Verification (T3a comparison)
| Hero | Total Raw Pips | Active Faces | HP | Keywords |
|------|---------------|--------------|-----|----------|
| Sceptile T3a | 3+3+3+3+2+0 = 14 | 5+utility | 9 | Engage, Critical, Growth Heal, Add Growth |
| Meganium T3a | 3+3+3+3+2+3 = 17 | 6 | 10 | Heal All, Vitality, Cleanse (all healing = lower effective value than damage) |
| Venusaur T3a | 2+4+3+3+2+3 = 17 | 6 | 11 | Plague, Regen, Repel, Poison |
| Swampert T3a | 3+3+3+3+3+2 = 17 | 6 | 11 | Rescue, Heavy, Redirect, Shield |
| Feraligatr T3a | 3+3+3+3+2+2 = 16 | 6 | 9 | Engage, Cruel, Heavy (pure damage) |
| Poliwrath T3a | 3+3+3+3+2+2 = 16 | 6 | 10 | Engage, Heavy, SelfShield |
| Politoed T3b | 3+3+3+3+2+2 = 16 | 6 | 9 | Heal All, ManaGain, Vitality |
| Wailord T3a | 3+3+4+4+2+2 = 18 | 6 | 14 | Shield to All, Heavy, Shield (high total justified by worst T1/T2) |
| *Kleavor T3 (ref)* | 3+3+3+3+2+2 = 16 | 6 | 10 | Engage, Cruel, Critical |
| *Blissey T3a (ref)* | 7+3+3+3+2+0 = 18 | 5 | 12 | Regen, Rescue, Cleave (1 blank) |
| *Gardevoir T3 (ref)* | 3+2+1+1+2+0 = 9 | 5 | 8 | Mana Trio, Heal, Weaken, Reuse (spell-focused, lower face pips) |

All T3 heroes fall within the 14-18 raw pip range established by existing T3 heroes. Wailord's 18 pips with HP 14 is the ceiling, justified by the 4-blank T1 investment.
