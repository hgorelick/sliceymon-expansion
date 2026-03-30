# Sliceymon Expansion Plan — v3

## Context
Expanding the Sliceymon textmod with 21 new hero evolution lines, new monsters, new bosses across Gen 3/4/5, and targeted capture changes. All changes are direct modifications to `/Users/hgorelick/Documents/slice-and-dice/textmod.txt`.

**Reference doc**: `/Users/hgorelick/Documents/slice-and-dice/SLICEYMON_AUDIT.md`

---

## Part 1: Hero Additions (21 brand-new heroes + Turtwig move + Tyranitar redesign = 23 total hero changes)

### Complete Color Pairing Map

**Full-pair colors (both P1 and P2 are new):**

| Color | P1 (new) | P2 (new) | Theme |
|-------|----------|----------|-------|
| z | **Charmander→Charizard** — Fire AoE DPS: Damage (34) + Cleave (36) + Rampage (137) + Mana Pain (82) for spell support | **Beldum→Metagross** — Steel/Psychic bruiser-tank: Steel Shield (63) + Heavy Damage (39) + Redirect (118). Slow but devastating | Power evolution |
| l | **Chikorita→Meganium** — Grass party healer: Heal All (107) + Shield Cleanse (71) + Heal Vitality (105). Aromatherapy spell | **Treecko→Sceptile** — Grass physical DPS with crits: Damage Critical (175) + Engage (17) + Add Growth (151). Hard-hitting physical attacker. Growth via Primrose template + Add Growth (151) face | Grass duo |
| w | **Bagon→Salamence** — Dragon rampage berserker: Rampage (137) + Cleave (36) + Pain (19). Shelgon = defensive cocoon, Salamence = AoE devastation | **Dratini→Dragonite** — Dragon all-rounder powerhouse: Heavy Damage (39) + Shield (56) + Engage (17). Balanced dragon that does everything well | Dragon duo |
| e (NEW) | **Machop→Machamp** — Fighting multi-striker: T1-T2 use DoubleUse (24) + Engage (17) + SelfShield (51). T3 Machamp upgrades to QuadUse (25) — premium keyword tier-gated to T3 only. Hits often, hits hard | **Riolu→Lucario** — Fighting/Steel precision warrior: Steel Damage (41) + Engage (17) + Charged (42). Aura Sphere spell | Fighting duo |
| j (NEW) | **Totodile→Feraligatr** — Aggressive water berserker: Engage (17) + Cruel (30) + Heavy (39). Pure offense with powerful jaws. **Rampage removed** — Cruel + Engage is already a strong offensive budget; Heavy represents raw jaw power | **Poliwag→Poliwrath/Politoed** — Branching water: Poliwrath = Heavy (39) + Engage + SelfShield. Politoed = Heal All + Shield ManaGain + Rain Dance spell | Water duo |

**Single-slot replacements (new hero paired with existing keeper):**

| Color | Phase | New Hero | Paired With (stays) | Theme |
|-------|-------|----------|-------------------|-------|
| q | P2 | **Cyndaquil→Typhlosion** — Fire eruption caster: Mana Pain (82) + AoE spell damage. Eruption spell | Litten→Incineroar (P1) | Fire + Fire |
| p | P2 | **Bulbasaur→Venusaur** — Grass/Poison sustain: Poison Plague (55) + Heal Regen (110) + Shield Repel (119). Wears down enemies while self-healing. Solar Beam spell | Duskull→Dusknoir (P1) | Poison themes |
| g | P2 | **Mudkip→Swampert** — Water/Ground defensive anchor: Shield Rescue (64) + Redirect SelfShield (118) + Heavy (39). Party protector | Slakoth→Slaking (P1) | Utility tanks |
| m | P2 | **Wailmer→Wailord** — Massive HP sponge: T3 HP 13-14 (mod ceiling), Shield to All (72) + Heavy (39). T1 mostly blanks (3-4 blank faces), T2 still blank-heavy. Drawback: very few active faces at all tiers to justify high HP | ★ Turtwig→Torterra (P1, moved from n) | Grass/Water tanks |
| t | P2 | **Weedle→Beedrill** — Bug/Poison speed DPS: Damage Poison (53) + Damage Engage (17) + Ranged (46). Fast poison striker | Scyther→Scizor/Kleavor (P1) | Bug + Bug |
| x | P2 | **Pichu→Pikachu→Raichu** — Electric burst: Damage Charged (42) + SU Charged (88) + Mana (76). Thunderbolt spell | Rotom (P1) | Electric + Electric |
| k | P2 | **Torchic→Blaziken** — Fire/Fighting speed assassin: Damage Defy (174) + Critical (175) + Engage (17) + DoubleUse (24). Blaze Kick spell. **No Cantrip** — premium keyword removed; DoubleUse represents Speed Boost (hits twice) | Litwick→Chandelure (P1) | Fire + Fire |
| r | P1 | **Togepi→Togetic→Togekiss** — Fairy healer/support: Heal Rescue (106) + Heal Cleave (109) + Shield ManaGain (61). Dazzling Gleam / Wish spell | Happiny→Blissey (P2) | Healer + Healer |
| u | P2 | **Cleffa→Clefairy→Clefable** — Fairy support/utility: Metronome (random/copycat effects), Moonlight heal (110), Shield (56). Whimsical wildcard fairy | Ralts→Gardevoir/Gallade (P1) | Fairy + Fairy/Psychic |

### Heroes Removed (14 lines)
| Line | Was | Replaced By |
|------|-----|-------------|
| L53 | Agumon→SkullGreymon (z, P1) | Charmander→Charizard |
| L97 | Tentomon→MegaKabuterimon (z, P2) | Beldum→Metagross |
| L27 | Fomantis→Lurantis (l, P1) | Chikorita→Meganium |
| L71 | Sunkern→Sunflora (l, P2) | Treecko→Sceptile |
| L99 | Ditto (w, P1+P2) — REMOVED ENTIRELY | Bagon→Salamence (P1) + Dratini→Dragonite (P2) |
| L81 | Slugma→Magcargo (q, P2) | Cyndaquil→Typhlosion |
| L79 | Trubbish→Garbodor (p, P2) | Bulbasaur→Venusaur |
| L61 | Varoom→Revavroom (g, P2) | Mudkip→Swampert |
| L73 | Roggenrola→Gigalith (m, P2) | Wailmer→Wailord |
| L87 | Burmy→Wormadam/Mothim (t, P2) | Weedle→Beedrill |
| L93 | Joltik→Galvantula (x, P2) | Pichu→Pikachu→Raichu |
| L69 | Espurr→Meowstic (k, P2) | Torchic→Blaziken |
| L39 | Darumaka→Darmanitan (r, P1) | Togepi→Togekiss |
| L89 | Tinkatink→Tinkaton (u, P2) | Cleffa→Clefable |
| L29 | Rockruff→Lycanroc (m, P1) | Turtwig→Torterra (moved from n, P2) |
| L31 | Applin→Flapple/Appletun/Hydrapple (n, P1) | Nidoran♀→Nidoqueen |
| (n, P2) | Turtwig→Torterra (moved to m, P1) | Nidoran♂→Nidoking |
| — | (new line) color e | Machop→Machamp (P1) + Riolu→Lucario (P2) |
| — | (new line) color j | Totodile→Feraligatr (P1) + Poliwag→Poliwrath/Politoed (P2) |

### Heroes That Stay (NOT replaced)
Noibat→Noivern (v, P1), Trapinch→Flygon (o, P2), Feebas→Milotic (c, P2), Porygon→Porygon-Z (i, P2), Goomy→Goodra (v, P2), Spheal→Walrein (b, P2), Axew→Haxorus (a, P2), Happiny→Blissey (r, P2), Slowpoke→Slowbro/Slowking (s, P2), Pawniard→Kingambit (y, P2), Lillipup→Stoutland (h, P2), Turtwig→Torterra (moved to m, P1), and all other existing P1 heroes not listed above.

### Complete Color Pairing Overview (all 24 colors)

`★` = new hero replacing an existing one. `✦` = brand new color/slot.

| Color | Name | P1 Hero | P2 Hero |
|-------|------|---------|---------|
| a | amber | Gible→Garchomp | Axew→Haxorus |
| b | blue | Vanillite→Vanilluxe | Spheal→Walrein |
| c | cyan | Magikarp→Gyarados | Feebas→Milotic |
| d | dark | Missingno (hidden) | — |
| **e** | **fighting** | **✦ Machop→Machamp** | **✦ Riolu→Lucario** |
| g | grey | Slakoth→Slaking | ★ **Mudkip→Swampert** (was Varoom) |
| h | huish | Larvitar→Tyranitar **(REDESIGN — remove poison, add Rock/Dark: Heavy + Cruel + Steel Shield + Sandstorm facade: 1 dmg to all enemies/turn)** | Lillipup→Stoutland |
| i | iuish | Eevee→Eeveelutions | Porygon→Porygon-Z |
| **j** | **water** | **✦ Totodile→Feraligatr** | **✦ Poliwag→Poliwrath/Politoed** |
| k | kuish | Litwick→Chandelure | ★ **Torchic→Blaziken** (was Espurr) |
| l | lime | ★ **Chikorita→Meganium** (was Fomantis) | ★ **Treecko→Sceptile** (was Sunkern) |
| m | mahogany | ★ **Turtwig→Torterra** (was Rockruff; moved from n) | ★ **Wailmer→Wailord** (was Roggenrola) |
| n | green | ★ **Nidoran♀→Nidoqueen** (was Applin) — Poison/Ground defensive: Poison mechanics (start poisoned, immune to poison, poison synergy) + Shield Repel (119) + Heavy (39). Inherits Tyranitar's current poison design | ★ **Nidoran♂→Nidoking** (was Turtwig) — Poison/Ground offensive: Poison Plague (55) + Engage (17) + Cruel (30). Sheer Force poison DPS. **Rampage removed** — Cruel better represents Sheer Force (bonus damage) and keeps keyword budget in check |
| o | orange | Larvesta→Volcarona | Trapinch→Flygon |
| p | purple | Duskull→Dusknoir | ★ **Bulbasaur→Venusaur** (was Trubbish) |
| q | quish | Litten→Incineroar | ★ **Cyndaquil→Typhlosion** (was Slugma) |
| r | red | ★ **Togepi→Togekiss** (was Darumaka) | Happiny→Blissey |
| s | sea | Squirtle→Blastoise | Slowpoke→Slowbro/Slowking |
| t | tuish | Scyther→Scizor/Kleavor | ★ **Weedle→Beedrill** (was Burmy) |
| u | uuish | Ralts→Gardevoir/Gallade | ★ **Cleffa→Clefable** (was Tinkatink) |
| v | violet | Noibat→Noivern | Goomy→Goodra |
| w | white | ★ **Bagon→Salamence** (was Ditto) | ★ **Dratini→Dragonite** (was Ditto) |
| x | xuish | Rotom (5 forms) | ★ **Pichu→Raichu** (was Joltik) |
| y | yellow | Honedge→Aegislash | Pawniard→Kingambit |
| z | zuish | ★ **Charmander→Charizard** (was Agumon) | ★ **Beldum→Metagross** (was Tentomon) |

**Summary:** 21 new heroes across 17 modified lines + 2 new colors. 23 existing heroes unchanged.

---

### Dice Design Principles
Each hero follows its **own Pokemon identity**, not the role of what it replaces:
- **Fire types** (Charmander, Torchic, Cyndaquil): Damage to All (34), Damage Cruel (30), Damage Engage (17), Mana Pain (82), fire spells. Note: Face 15 is generic Damage — use typed faces (34, 30, 17) for flavor
- **Water types** (Mudkip, Totodile, Wailmer, Poliwag): Shield (56, 64), Heavy (39), SelfHeal (52), Engage (17)
- **Grass physical** (Treecko): Damage Critical (175), Engage (17), Add Growth (151) — NOT a caster. Growth mechanic via Primrose template
- **Grass support** (Chikorita, Bulbasaur): Heal (103-113), Shield Cleanse (71), Poison (53, 55)
- **Fighting** (Machop, Riolu, Blaziken): Heavy (39), Engage (17), DoubleUse (24, T1-T2) / QuadUse (25, T3 Machamp only), Defy (174). **No Cantrip on Blaziken** — DoubleUse (24) for Speed Boost instead
- **Steel** (Beldum): Steel Shield (63), Heavy (39), Redirect (118)
- **Electric** (Pikachu): Charged (42, 88), Mana (76)
- **Dragon** (Bagon, Dratini): Rampage (137), Cleave (36), Engage (17), Heavy (39). Note: Rampage is budget-appropriate here because dragons have Pain (19) as drawback
- **Fairy healer** (Togepi): Heal Rescue (106), Heal Cleave (109), Shield ManaGain (61). Distinct from Chikorita: Togepi = targeted rescue/clutch healing, Chikorita = AoE sustain
- **Bug/Poison** (Weedle): Damage Poison (53), Ranged (46), Engage (17)
- **Fairy utility** (Cleffa): Copycat (27), Heal Regen (110), Shield (56) — Metronome = Copycat face approximation (copies face above on die). Note: not true randomness, but thematic
- **Poison/Ground tank** (Nidoran-F→Nidoqueen): Inherit Tyranitar's current poison design (start poisoned, immune to poison, poison synergy) + Shield Repel (119) + Heavy (39). The poison-themed tank the Larvitar line currently is, but on the correct Pokemon
- **Poison/Ground DPS** (Nidoran-M→Nidoking): Poison Plague (55), Engage (17), Cruel (30) — Sheer Force poison offense. **Rampage removed** — Cruel represents Sheer Force bonus damage
- **Rock/Dark redesign** (Larvitar→Tyranitar): REDESIGN existing line. Remove poison theme. Replace with Rock/Dark identity: Damage Heavy (39) + Damage Cruel (30) + Shield Steel (63) + Sandstorm via `.facade.` (1 damage to all enemies each turn — mirrors the Gigalith "1 damage to all" pattern from L73 audit, re-flavored as sandstorm chip). Tyranitar = bulky dark bruiser with massive Heavy hits and passive environmental damage

### Templates
- `replica.Lost` — default for most heroes (DPS, utility)
- `replica.Statue` — for tanks/casters that need blank-slate faces (Beldum, Cyndaquil, Wailmer)
- `Primrose` — for Treecko (growth mechanic, replaces Sunkern which used Primrose)
- `replica.Healer` — for Togepi (healer, replaces Darumaka which was on r but Happiny on r uses Healer)

### Pixel Art
- Source PMD sprites from **pmdcollab.org** (credited by original mod)
- Encode using **tann.fun/things/dice-img**
- ~70+ sprites needed (hero T1/T2/T3 + monsters + captures + bosses + spells)

---

## Part 2: Capture System Changes

### Remove 6 Captures
| Pokemon | Reason | Line |
|---------|--------|------|
| Ivysaur | Becomes hero (Bulbasaur line) | 111 |
| Pikachu | Becomes hero | 111 |
| Charizard | Becomes hero (Charmander line) | 111 |
| Metagross | Becomes hero (Beldum line) | 111 |
| Poliwag | Becomes hero | 111 |
| Arceus | Becomes Gen 4 Floor 20 boss | 63, 111 |

### Upgrade Existing Captures to Final Evolutions
| Current | Replace With | Ball (keep same) |
|---------|-------------|-----------------|
| Caterpie | **Butterfree** | Nest Ball |
| Sneasel | **Weavile** | Fast Ball |
| Barboach | **Whiscash** | Dive Ball |
| Electrike | **Manectric** | Quick Ball |

### Other Capture Replacements (different Pokemon, same ball slot)
| Current | Replace With | Ball | Reason |
|---------|-------------|------|--------|
| Rattata | **Skarmory** (Steel/Flying, single-stage) | Level Ball | Not an evolution — entirely different Pokemon replacing the slot |
| Furret | **Lapras** (Water/Ice, single-stage, Gen 1 iconic) | Friend Ball | Thematic upgrade |
| Alcremie | **Arcanine** | Premier Ball | Better Gen 1 representation |
| Zubat | **REMOVED** (now enemy monster) | Dusk Ball freed | Zubat moves to monster pool |

### Keep Unchanged (5 fully unchanged + 4 upgraded + 3 replaced = 12 remaining)
- **Fully unchanged**: Delcatty (Moon Ball), Lilligant (Luxury Ball), Wobbuffet (Timer Ball), Snorlax (Dream Ball), Mewtwo (Master Ball)
- **All 24 ball types are unique** — no duplicates across kept, upgraded, replaced, and new captures
- **Upgraded** (same ball, evolved Pokemon): Butterfree, Weavile, Whiscash, Manectric
- **Replaced** (same ball, different Pokemon): Skarmory, Lapras, Arcanine
- **Total after removals**: 19 original - 7 removed = 12 remaining captures
- **Legendary items total**: 10 (4 existing: Ho-Oh, Lugia, Kyogre, Groudon + 6 new: Rayquaza, Latias, Latios, Suicune, Entei, Raikou)

### Add 8 New Captures + 6 New Legendary Items
| Pokemon | Item Type | Tier |
|---------|-----------|------|
| **Mew** | Poke Ball | 3 | *Freed from Pikachu removal* |
| **Jirachi** | Great Ball | 4 | *Freed from Ivysaur removal* |
| **Kangaskhan** | Safari Ball | 5 | *New ball type* |
| **Heracross** | Net Ball | 6 | *New ball type* |
| **Greninja** | Dusk Ball | 6 | *Freed from Zubat removal. Dark type = dusk themed* |
| **Electivire** | Ultra Ball | 6 | *Freed from Charizard removal* |
| **Magmortar** | Lure Ball | 6 | *Freed from Poliwag removal* |
| **Latias** | **Legendary item** (Soul Dew) | 7 | Legendary summon. Defensive Eon twin: Shield to All (72) + Heal Cleanse (111) + Dodge (123). Mist Ball = weaken facade on enemies. Flees turn 7. |
| **Latios** | **Legendary item** (Eon Flute) | 7 | Legendary summon. Offensive Eon twin: Damage Ranged (46) + Damage Cleave (36) + Mana Pain (82). Luster Purge = vulnerable facade on enemies. Flees turn 7. |
| **Suicune** | **Legendary item** (Clear Bell) | 7 | Legendary summon. Water defensive/cleanse support: Shield (56) + Heal (103) + Heal Cleanse (111). Purifies and protects. Flees turn 7. |
| **Entei** | **Legendary item** (Flame Plate) | 7 | Legendary summon. Fire damage + AoE: Damage (15) + Damage to All (34) + Heavy. Sacred Fire eruption. Flees turn 7. |
| **Raikou** | **Legendary item** (Zap Plate) | 7 | Legendary summon. Electric charged burst + speed: Damage Charged (42) + Damage Engage (17). Lightning-fast strikes. Flees turn 7. |
| **Rhyperior** | Heavy Ball | 7 | *Freed from Metagross removal* |
| **Rayquaza** | **Legendary item** (Jade Orb / Sky Pillar) | 8 | Same system as Ho-Oh/Lugia/Kyogre/Groudon |

**Ball conflict resolution**: Dive Ball (Whiscash), Quick Ball (Manectric), and Timer Ball (Wobbuffet) are already in use. New captures use alternative ball types to avoid engine conflicts. Lure Ball and Heavy Ball are freed by Poliwag/Metagross becoming heroes.

---

## Part 3: Monster Additions

### Regular Monsters (appended with `.part.1`)

**IMPORTANT: Monster Face ID Rules**
- Monsters use **170** (enemy-style Damage) instead of 15 (hero Damage)
- Monsters use **171** (enemy-style Damage Cleave) instead of 36 (hero Cleave)
- Monsters do NOT get Cantrip (126) — they don't reroll
- Monsters do NOT get `.abilitydata.` spells
- Use facade-based keywords (weaken, petrify, inflict-exert) via `.facade.` syntax, not raw face IDs for status effects
- Study existing monster templates (Slimelet, Bee, Bones) on L119 for exact format

**New Monsters — Floor 1-3 (Line 119):**
| Monster | HP | Dice Concept |
|---------|----|-------------|
| Zubat | 3 | Enemy Damage (170) + facade for Confuse Ray (weaken/petrify) + small pips |
| Tentacool | 4 | Enemy Damage (170) + facade for Poison (inflict-poison via facade, NOT face 53) |
| Carvanha | 3 | High Enemy Damage (170-3) + facade for Engage — glass cannon |
| Chinchou | 4 | Enemy Damage (170) + small self-heal (52) |

**Floor 9-11 (Line 121):**
| Monster | HP | Dice Concept |
|---------|----|-------------|
| Golbat | 6 | Enemy Damage (170) + facade for Confuse Ray (weaken/inflict-exert) + SelfHeal (52) — drain |
| Tentacruel | 8 | Enemy Damage (170) + facade for AoE Poison + Enemy Cleave (171) |
| Sharpedo | 7 | Very high Enemy Damage (170-4) + facade for Engage + Cruel — pure aggression |
| Lanturn | 7 | Enemy Damage (170) + Shield to All monsters (72) — monster support |
| Wild Steelix | 10 | Shield Steel (63) + Enemy Damage (170) + Repel (119) — wall. **Named "Wild Steelix" to differentiate from Gen 3 boss Steelix** |

**Floor 17-19 (Line 123):**
| Monster | HP | Dice Concept |
|---------|----|-------------|
| Crobat | 9 | Enemy Damage (170) + facade for Poison Plague + Confuse Ray (inflict-exert/petrify) — elite |
| Elite Steelix | 12 | Enhanced Shield Steel (63) + Enemy Damage (170) + Repel (119) — fortress. **Named "Elite Steelix"** |
| Absol | 8 | Enemy Damage (170) + facade for Critical + Cruel — elite assassin |

**Note: Monsters do NOT get Cantrip** (they don't reroll). Use facade-based Engage + high enemy-style damage (170) pips for aggressive monsters.

---

## Part 4: Boss Fights

### Complete Boss Layout (5 Gen Paths + Random Mix)

| Floor | Gen 3 (NEW) | Gen 4 (NEW) | Gen 5 (NEW) | Gen 6 (existing) | Gen 7 (existing) |
|-------|-------------|-------------|-------------|-------------------|-------------------|
| **4** | **Golem** + Geodude/Graveler | Quagsire | Quagsire | Quagsire | Quagsire |
| **8** | **Alpha Steelix** + Onix swarm | Exeggutor | **Serperior + Emboar + Samurott** (Unova starters) | Exeggutor | Exeggutor |
| **12** | **Random: Regi Trio OR Legendary Birds** | **Palkia** | **Random: Swords of Justice OR Forces of Nature** | Xerneas | Necrozma P1 |
| **16** | **Regigigas** + Regi Guardians (Regirock/Regice/Registeel) | **Dialga** | **Reshiram/Zekrom** | Zygarde | Necrozma P2 |
| **20** | **Deoxys** (4 forms) | **Arceus** (FINAL BOSS) | **Kyurem** (B/W forms) | Hoopa | Necrozma P3 |

**Random Mix option**: Expand existing random selection to pull from ALL 5 gens per floor independently.

### Boss Fight Designs
*All designs modeled after existing patterns. Study: Quagsire (L131), Exeggutor (L133), Xerneas (L137), Zygarde (L139/141), Hoopa (L143/145), Necrozma (L147), and base S&D bosses (Troll King, Hexia, Dragon, Slime Queen) for HP values, templates, minion mechanics, phase structures.*

**Gen 3:**
- **Golem (F4):** Model after Quagsire. HP ~11-12 (Quagsire is HP 11 — do NOT exceed). Geodude minions (HP 3) + Graveler (HP 5-6). Self-destruct faces on Geodudes (single-use damage to all).
- **Alpha Steelix + Onix (F8):** Model after Exeggutor multi-body. Alpha Steelix HP ~18 + Onix (HP 6-8, x2). Massive shields + heavy damage. **Named "Alpha Steelix"** to differentiate from Wild Steelix/Elite Steelix regular monsters.
- **Regi Trio (F12):** Model after Xerneas + Florges. Three bosses: Regirock (HP ~12, heavy+shields), Regice (HP ~12, damage+weaken), Registeel (HP ~12, steel shields+repel). Total ~36 matches F12 ceiling (Xerneas 25 + Florges 10 = 35). **HP reduced from 15 to 12 each** to stay within budget. The "puzzle" = three distinct threats requiring different counter-strategies simultaneously.
- **Legendary Birds (F12, alternative):** Articuno (HP ~12, ice shields+weaken), Zapdos (HP ~12, electric cruel+first damage), Moltres (HP ~12, fire AoE+pain). Total ~36 matches Regi Trio. Randomly selected as alternative to Regi Trio at Gen 3 F12 — each run picks Birds or Regis.
- **Regigigas (F16):** Model after Zygarde. HP ~20 + weakened Regi guardians: Regirock (HP 7), Regice (HP 6), Registeel (HP 8). Total ~41. "Slow Start" = stasis-like first turns, then massive Rampage. The Regi trio serves as Regigigas's guardians (he is their creator/master) — these are weaker versions than their F12 boss forms (HP 6-8 vs HP 12).
- **Deoxys (F20):** Model after Necrozma phases. Form changes: Normal (HP ~10, balanced) → Attack (HP ~8, massive damage but fragile) → Defense (HP ~12, heavy shields+repel) → Speed (HP ~8, high engage+dodge). Total HP ~38. The "puzzle" = each form demands different counter-strategy (burst the Attack form fast, chip through Defense, etc.). NOT a stat check — form variety forces adaptation.

**Gen 4:**
- **Palkia (F12):** HP ~25 + support minions (e.g., Bronzong HP ~8, Spatial distortion hazards HP ~5). Spatial Rend = Ranged (46) high damage. Model after Xerneas+Florges (boss + support pattern). **Solo boss at HP 25 with no minions would be a stat check, not a puzzle** — needs minions that create interesting targeting decisions.
- **Dialga (F16):** HP ~25 + Time distortion minions (e.g., Bronzong HP ~8 with Stasis faces, temporal anomalies that buff/debuff). Roar of Time = massive Damage (170) + Weaken (131). Model after Zygarde+Cells (boss + regenerating adds pattern). The "puzzle" = managing temporal adds that grant Dialga extra turns or stasis your heroes.
- **Arceus (F20):** HP 40+. Judgment = Damage to All (34-style). Type-shifting each turn changes Arceus's damage type AND resistances (e.g., Fire type = immune to fire damage, deals fire AoE; Steel type = heavy shields, reduced incoming). Model after Necrozma phases but with turn-by-turn shifts rather than HP thresholds. The "puzzle" = predicting the next type shift and choosing which heroes to commit. **Must be beatable at Normal difficulty** — type shifts should have tells/patterns, not be purely random.

**Gen 5:**
- **Serperior + Emboar + Samurott (F8):** Three Unova starters as bosses (HP ~12 each). Serperior (grass + shields), Emboar (fire damage + rampage), Samurott (water damage + cleave). Model after Exeggutor/Swords of Justice pattern.
- **Swords of Justice (F12, random path A):** Cobalion (HP ~12, steel), Terrakion (HP ~12, rock), Virizion (HP ~12, grass). Total ~36 matches F12 ceiling. **Keldeo removed** — adding HP 8 would push total to 44, far exceeding F12 budget. Keldeo could appear as a capture instead.
- **Forces of Nature (F12, random path B):** Tornadus (HP ~12, AoE+dodge), Thundurus (HP ~12, charged), Landorus (HP ~12, heavy). Total ~36 matches Xerneas+Florges (~35). **HP reduced from 15 to 12 each** — 15x3=45 was far too high for F12.
- **Reshiram/Zekrom (F16):** One main boss (HP ~25) + the other as environmental hazard (passive AoE damage or buff to boss each turn, NOT a targetable enemy). Reshiram = fire AoE + weaken; Zekrom = electric burst + charged. Model after Zygarde (L139/L141 X/Y variant pattern). The "puzzle" = managing the environmental hazard while fighting the main boss. Which dragon appears could be random per run (like Zygarde X vs Y).
- **Kyurem (F20):** Base Kyurem (HP ~15, ice damage + weaken) → absorbs Reshiram OR Zekrom mid-fight → Black Kyurem (HP ~20, fire+ice hybrid, massive damage) OR White Kyurem (HP ~20, electric+ice hybrid, AoE). Total HP ~35. The "puzzle" = which fusion form appears changes the counter-strategy needed. Phase transition should visually telegraph which form is coming.

---

## Part 5: Supporting Changes

### A. Character Selection System (Line 11)
- Update all 14 replacement hero entries (names, sprites, descriptions)
- Add 2 new color entries: **e** (Fighting duo) and **j** (Water duo)
- Expand the draft picker to include colors e and j

### B. Ditto Removal (Line 99)
Delete entire Line 99 (66K chars). Remove Ditto from color w draft. Replace with Dragon duo (Bagon P1 + Dratini P2).

### C. Boss Selection Menu (Line 135)
Expand from "Gen 6 / Gen 7 / Random" to "Gen 3 / Gen 4 / Gen 5 / Gen 6 / Gen 7 / Random Mix"
Random Mix = each floor independently picks a random gen's boss.

### D. Togepi + Riolu Eggs (Line 109)
Remove Togepi egg items (x6) AND Riolu egg items (x2) — both are now selectable heroes. Keep Bonsly eggs (x2) and Mystery Egg.

### E. Line 63 Cleanup
Remove Arceus capture from Line 63 (Lillipup compound line). Keep Caterpie→Butterfree capture and Lillipup hero.

### F. Porygon Helper (Line 67)
Keep — Porygon is NOT being replaced.

### G. Pixel Art
- Source PMD sprites from **pmdcollab.org**
- Encode using **tann.fun/things/dice-img**
- ~70+ sprites needed

---

## Part 6: Implementation Chunks

### Global Constraints
- **Source of truth**: `/Users/hgorelick/Documents/slice-and-dice/textmod.txt` (152 lines, ~415KB)
- **Reference doc**: `/Users/hgorelick/Documents/slice-and-dice/SLICEYMON_AUDIT.md`
- **Do NOT** modify any line not explicitly listed in a chunk's "Lines touched" field
- **Do NOT** change any existing hero marked "stays" in the Complete Color Pairing Overview
- **Use ONLY** face IDs from the Key Face IDs Quick Reference in SLICEYMON_AUDIT.md
- **Use ONLY** templates listed in the Template Reference section of the audit
- **Every new hero** MUST follow the exact `sd=ID-PIPS:ID-PIPS:...` format visible in existing heroes on adjacent lines
- **Every chunk** must be verified by pasting textmod.txt into Slice & Dice (Modes > Creative > Custom > Resources) before proceeding to the next chunk

### Parallel Execution Map

```
PHASE 0 (BLOCKING — Human task):
  [C0] Pixel Art Sourcing ──────────────────────────────────────────────────────►

PHASE 1 (Sequential — each chunk depends on prior):
  [C1] Ditto Removal (L99) ─► [C2a-C2e] Hero Replacements (parallel batch) ─► [C3] New Colors e,j ─► [C4] Char Select + Eggs

PHASE 2 (Sequential, independent of Phase 1 completion):
  [C5] Capture Removals ─► [C6] Capture Upgrades + Replacements ─► [C7] New Ball Captures ─► [C8] New Legendary Items (Latias, Latios, Rayquaza, Suicune, Entei, Raikou)

PHASE 3 (Independent of Phase 1 and 2):
  [C9] Regular Monsters

PHASE 4 (Sequential, independent of Phase 1-3):
  [C10] Boss Menu Expansion ─► [C11] Gen 3 Bosses ─► [C12] Gen 4 Bosses ─► [C13] Gen 5 Bosses

PHASE 5 (Depends on ALL above):
  [C14] Larvitar Redesign ─► [C15] Nidoran Poison Inheritance ─► [C16] Final Integration Test

Note: Phases 1-4 can execute in parallel EXCEPT where noted.
C14 (Larvitar redesign) must complete BEFORE C15 (Nidoran inherits Tyranitar's OLD poison design which moves to Nidoqueen).
```

---

### PHASE 0: Human-Only Prerequisite

#### Chunk C0: Pixel Art Sourcing and Encoding
- **Owner**: Human (NOT AI-executable)
- **Task**: Source ~70+ PMD sprites from pmdcollab.org, encode via tann.fun/things/dice-img
- **Sprites needed**: All 21 new hero lines (T1/T2/T3 forms), new monsters, new captures, new bosses, spell icons
- **Output**: A sprite reference file mapping each Pokemon name to its encoded `.img.` string
- **BLOCKS**: All hero/monster/boss/capture chunks that need `.img.` data. Chunks can be written with placeholder `img.PLACEHOLDER_POKEMONNAME` strings and sprites inserted later.
- **If blocked**: Proceed with all chunks using `img.PLACEHOLDER_POKEMONNAME` placeholders. Do a find-and-replace pass after sprites are encoded.

---

### PHASE 1: Hero Changes

#### Chunk C1: Ditto Removal (Line 99)
- **Concern**: Remove the entire Ditto line to free color w for Dragon duo
- **Lines touched**: L99 (66,309 chars — the largest line), L11 (character selection — remove Ditto from w draft)
- **Source of truth**: SLICEYMON_AUDIT.md "Line 99 | Color w (white) | Ditto"
- **Steps**:
  1. Delete the ENTIRE contents of Line 99 (do NOT delete the line itself — keep it as blank or the line numbering shifts)
  2. In Line 11, find the `w` color draft entry referencing "Ditto" and remove it (it will be re-added in C3 with Dragon duo)
- **Pattern reference**: Study how Line 99 is structured in the audit: `replica.Lost`, T1 Ditto HP 3, contains ~130+ Pokemon form copies
- **Verification**:
  - Line 99 is empty or contains only whitespace
  - Line 11 no longer references Ditto or color w
  - Paste textmod into game: selecting color w in draft should show nothing or error gracefully
  - Total file size decreased by ~66KB
- **If blocked**: If Line 99 cannot be cleanly emptied (encoding issues), replace its entire content with a single comment placeholder: `=` (empty definition)

#### Chunk C2a: Hero Replacements — Fire types (z, q, k)
- **Concern**: Replace Digimon (Agumon/Tentomon) and non-Pokemon (Slugma, Espurr) with fire-themed Pokemon
- **Lines touched**: L53, L97, L81, L69 (4 lines, max 5)
- **Source of truth**: Audit entries for Lines 53, 97, 81, 69
- **Pattern reference**: Copy the EXACT structural format from the line being replaced. For L53 (Agumon), note: `replica.Statue`, T1 HP 5, T2 HP 8, T3 HP 8-10. New Charmander line should use same template and similar HP curve.
- **Steps**:
  1. **L53**: Replace Agumon→SkullGreymon with Charmander→Charizard. Template: `replica.Statue`. Face palette: Damage to All (34) + Cleave (36) + Rampage (137) + Mana Pain (82). **Tier-gate the AoE**: T1 Charmander should use basic Damage Cruel (30) + Mana Pain (82) with 2-3 blanks. T2 Charmeleon adds Cleave (36). T3 Charizard gets Damage to All (34) + Rampage (137) — the full AoE devastation. T1 HP 5, T2 HP 7-8, T3 HP 9-10. Keep `col.z`.
  2. **L97**: Replace Tentomon→MegaKabuterimon with Beldum→Metagross. Template: `replica.Statue`. Faces: Shield Steel (63) + Heavy Damage (39) + Redirect (118). T1 Beldum HP 5, T2 Metang HP 7-8, T3 Metagross HP 10-12. Keep `col.z`.
  3. **L81**: Replace Slugma→Magcargo with Cyndaquil→Typhlosion. Template: `replica.Statue`. Faces: Mana Pain (82) + AoE spell damage. T1 Cyndaquil HP 4, T2 Quilava HP 6-7, T3 Typhlosion HP 9. Keep `col.q`. Include Eruption spell via `.abilitydata.`
  4. **L69**: Replace Espurr→Meowstic with Torchic→Blaziken. Template: `replica.Lost`. Faces: Damage Defy (174) + Critical (175) + Engage (17) + DoubleUse (24). T1 Torchic HP 4, T2 Combusken HP 7, T3 Blaziken HP 9. Keep `col.k`. Include Blaze Kick spell. **No Cantrip** — DoubleUse (24) represents Speed Boost (hits twice). Cantrip was removed as it made Blaziken grossly overbudget (Defy + Critical + Engage + Cantrip = 4 premium keywords).
- **Explicit constraints**:
  - Do NOT change the color assignment (`col.z`, `col.q`, `col.k`)
  - Do NOT modify the line's position (L53 stays at line 53)
  - Each T3 form MUST have exactly 2 variants (matching the replaced hero's structure)
  - All face IDs MUST exist in SLICEYMON_AUDIT.md Key Face IDs table
- **Verification**:
  - Paste textmod: draft color z shows Charmander (P1) and Beldum (P2)
  - Charmander evolves: Charmander→Charmeleon→Charizard with 2 T3 forms
  - Beldum evolves: Beldum→Metang→Metagross with 2 T3 forms
  - Cyndaquil appears under color q P2, evolves through Typhlosion
  - Torchic appears under color k P2, evolves through Blaziken
  - No crashes on any evolution path
- **If blocked**: If the replaced line has a structure too complex to replicate (e.g., spells, summoning), implement the basic hero (T1/T2/T3 dice + HP) first, then add spells in a follow-up sub-chunk.

#### Chunk C2b: Hero Replacements — Grass types (l) + Bulbasaur (p)
- **Concern**: Replace Fomantis/Sunkern with Chikorita/Treecko, replace Trubbish with Bulbasaur
- **Lines touched**: L27, L71, L79 (3 lines)
- **Source of truth**: Audit entries for Lines 27 (Fomantis, `replica.Lost`), 71 (Sunkern, `Primrose` template), 79 (Trubbish, `replica.Statue`)
- **Pattern reference**: L71 uses the UNIQUE `Primrose` template — Treecko MUST use `Primrose` to inherit its growth mechanic. L27 uses `replica.Lost`. L79 uses `replica.Statue`.
- **Steps**:
  1. **L27**: Replace Fomantis→Lurantis with Chikorita→Meganium. Template: `replica.Lost`. Faces: Heal All (107) + Shield Cleanse (71) + Heal Vitality (105). T1 Chikorita HP 4, T2 Bayleef HP 6-7, T3 Meganium HP 8-9. Keep `col.l`. Include Aromatherapy spell.
  2. **L71**: Replace Sunkern→Sunflora with Treecko→Sceptile. Template: `Primrose` (MUST keep — unique growth template). Faces: Damage Critical (175) + Engage (17) + Add Growth (151). T1 Treecko HP 5, T2 Grovyle HP 7, T3 Sceptile HP 9. Keep `col.l`. **Note: Face ID 16 does not exist** — use 151 (Add Growth) for the growth mechanic. The Primrose template provides additional growth scaling.
  3. **L79**: Replace Trubbish→Garbodor with Bulbasaur→Venusaur. Template: `replica.Statue`. Faces: Poison Plague (55) + Heal Regen (110) + Shield Repel (119). T1 Bulbasaur HP 5, T2 Ivysaur HP 7-8, T3 Venusaur HP 10. Keep `col.p`. Include Solar Beam spell.
- **Explicit constraints**:
  - L71 MUST use `Primrose` template, NOT `replica.Lost`
  - Treecko is a PHYSICAL DPS — do NOT give it healing or mana faces
  - Bulbasaur is Grass/Poison sustain — MUST have both poison AND heal faces
- **Verification**:
  - Color l draft: Chikorita (P1) heals party, Treecko (P2) deals physical damage with crits
  - Treecko has growth mechanic from Primrose template
  - Color p draft: Bulbasaur (P2) alongside Duskull (P1, unchanged)
  - Venusaur has both poison and healing faces on T3
- **If blocked**: If Primrose template interaction is unclear, study Sunkern line (L71) character-by-character before replacing.

#### Chunk C2c: Hero Replacements — Dragons (w) + Wailmer (m) + Mudkip (g)
- **Concern**: Fill color w with Dragon duo (post-Ditto removal), replace Roggenrola and Varoom
- **Lines touched**: L99 (now empty from C1 — write Bagon P1 + Dratini P2), L73, L61 (3 lines)
- **Depends on**: C1 (Ditto removal) MUST be complete
- **Source of truth**: Audit entries for Lines 99, 73 (Roggenrola, `replica.Lost`), 61 (Varoom, `replica.Statue`)
- **Steps**:
  1. **L99**: Write NEW Dragon duo line. Bagon→Salamence (P1): `replica.Lost`, Rampage (137) + Cleave (36) + Pain (19). T1 Bagon HP 5, T2 Shelgon HP 8 (defensive cocoon — more shields), T3 Salamence HP 10. Dratini→Dragonite (P2): `replica.Lost`, Heavy Damage (39) + Shield (56) + Engage (17). T1 Dratini HP 5, T2 Dragonair HP 7, T3 Dragonite HP 10. Both `col.w`.
  2. **L73**: Replace Roggenrola→Gigalith with Wailmer→Wailord. Template: `replica.Statue`. Faces: Shield to All (72) + Heavy (39). T1 Wailmer HP 7 (4+ blank faces), T2 Wailmer HP 10 (3 blank faces), T3 Wailord HP 13-14 (1-2 blank faces). **HP capped at 13-14** — the mod ceiling is 13 (Gyarados/Tyranitar). Wailord's identity comes from high HP + heavy blanks as drawback, not breaking the HP scale. Keep `col.m`.
  3. **L61**: Replace Varoom→Revavroom with Mudkip→Swampert. Template: `replica.Statue`. Faces: Shield Rescue (64) + Redirect SelfShield (118) + Heavy (39). T1 Mudkip HP 5, T2 Marshtomp HP 7-8, T3 Swampert HP 10. Keep `col.g`.
- **Explicit constraints**:
  - L99 MUST contain TWO complete hero definitions (P1 + P2) on the SAME line, separated by the standard heropool delimiter
  - Study how other 2-hero lines (e.g., L13 for Gible+Axew) structure the P1/P2 split
  - Wailord T3 HP MUST be 13-14 (highest in the mod alongside Gyarados/Tyranitar). Compensate with 4+ blank faces at T1 and heavy blank economy throughout — Wailord's identity is "huge HP, few useful faces"
  - Shelgon (Bagon T2) should have MORE shield faces than Bagon T1 (cocoon fantasy)
- **Verification**:
  - Color w draft: Bagon (P1) and Dratini (P2) appear (no Ditto)
  - Bagon→Shelgon→Salamence evolves with Shelgon being tankier
  - Wailord has 13-14 HP at T3 with heavy blank economy
  - Swampert has Redirect + Shield Rescue (party protector role)
  - No remnants of Ditto, Roggenrola, or Varoom in any draft or gameplay
- **If blocked**: If L99 two-hero-per-line format is unclear, study L13 (Gible P1 + Axew P2) byte-by-byte to understand the P1/P2 delimiter before writing.

#### Chunk C2d: Hero Replacements — Fairy/Healer types (r, u) + Electric (x) + Bug (t)
- **Concern**: Replace Darumaka, Tinkatink, Joltik, Burmy with Togepi, Cleffa, Pichu, Weedle
- **Lines touched**: L39, L89, L93, L87 (4 lines)
- **Source of truth**: Audit entries for Lines 39 (Darumaka, `replica.Statue`), 89 (Tinkatink, `replica.Lost`), 93 (Joltik, `replica.Lost`), 87 (Burmy, `replica.Lost`)
- **Pattern reference**: L39 replaces into `replica.Healer` slot (color r) — Togepi MUST use `replica.Healer` since it pairs with Happiny→Blissey (L83, the only other Healer template user)
- **Steps**:
  1. **L39**: Replace Darumaka→Darmanitan with Togepi→Togetic→Togekiss. Template: `replica.Healer`. Faces: Heal Rescue (106) + Heal Cleave (109) + Shield ManaGain (61). T1 Togepi HP 4, T2 Togetic HP 7, T3 Togekiss HP 9. Keep `col.r`. Include Dazzling Gleam and/or Wish spell.
  2. **L89**: Replace Tinkatink→Tinkaton with Cleffa→Clefairy→Clefable. Template: `replica.Lost`. Faces: Damage Copycat (27) + Heal Regen (110) + Shield (56). T1 Cleffa HP 4, T2 Clefairy HP 7, T3 Clefable HP 9. Keep `col.u`. Metronome = copycat mechanic. Include Moonlight spell.
  3. **L93**: Replace Joltik→Galvantula with Pichu→Pikachu→Raichu. Template: `replica.Lost`. Faces: Damage Charged (42) + Damage SU Charged (88) + Mana (76). T1 Pichu HP 4, T2 Pikachu HP 6, T3 Raichu HP 9. Keep `col.x`. Include Thunderbolt spell.
  4. **L87**: Replace Burmy→Wormadam/Mothim with Weedle→Kakuna→Beedrill. Template: `replica.Lost`. Faces: Damage Poison (53) + Damage Engage (17) + Damage Ranged (46). T1 Weedle HP 4, T2 Kakuna HP 6 (defensive — more shields/blanks), T3 Beedrill HP 8. Keep `col.t`.
- **Explicit constraints**:
  - Togepi MUST use `replica.Healer` template (matches Happiny on same color r). NOTE: Darumaka (being replaced) used `replica.Statue` — this is an INTENTIONAL template change, not a copy error.
  - Clefable's Metronome = Copycat (face ID 27) — do NOT invent new mechanic
  - Pichu is a 3-stage evo: ensure T1=Pichu, T2=Pikachu, T3=Raichu (not 2-stage)
  - Pichu T1 HP should be 4 (not 3) — HP 3 is too fragile for a T1 with active faces. Only Ditto (HP 3, pure copycat) and Ralts (HP 3, caster with blanks) are that low, and both have specific reasons.
  - Kakuna (Weedle T2) should be defensive/cocoon-like (more blanks/shields), Beedrill = fast offense
- **Verification**:
  - Color r: Togepi (P1) is a healer, evolves to Togekiss
  - Color u: Cleffa (P2) has copycat faces, Clefable has Moonlight heal
  - Color x: Pichu (P2) → Pikachu → Raichu, electric burst damage
  - Color t: Weedle (P2) → Kakuna (tanky) → Beedrill (fast poison DPS)

#### Chunk C2e: Hero Replacements — Turtwig move (m/n) + Nidoran pair (n)
- **Concern**: Move Turtwig from color n to color m (P1), replace Applin with Nidoqueen, replace Turtwig's old slot with Nidoking
- **Lines touched**: L75 (Turtwig, currently n/P2), L29 (Rockruff, currently m/P1), L31 (Applin, currently n/P1) (3 lines)
- **Depends on**: None (independent of C2a-C2d)
- **Source of truth**: Audit entries for Lines 75 (Turtwig, `replica.Lost`), 29 (Rockruff, `replica.Lost`), 31 (Applin, `replica.Statue`)
- **Steps**:
  1. **L29**: Replace Rockruff→Lycanroc with Turtwig→Torterra. COPY the existing Turtwig line from L75, change `col.n` to `col.m`. Keep all existing Turtwig mechanics (Starly summons, growth). Template stays `replica.Lost`.
  2. **L75**: Replace the OLD Turtwig→Torterra content with Nidoran-M→Nidorino→Nidoking. Template: `replica.Lost`. Faces: Poison Plague (55) + Engage (17) + Cruel (30). T1 Nidoran-M HP 5, T2 Nidorino HP 7, T3 Nidoking HP 10. Set `col.n`. **No Rampage** — Cruel represents Sheer Force.
  3. **L31**: Replace Applin→Flapple/Appletun/Hydrapple with Nidoran-F→Nidorina→Nidoqueen. Template: `replica.Statue`. Faces: Poison design inherited from Tyranitar AFTER C14 redesign + Shield Repel (119) + Heavy (39). T1 Nidoran-F HP 5, T2 Nidorina HP 7, T3 Nidoqueen HP 10. Set `col.n`.
- **Explicit constraints**:
  - Turtwig's move MUST preserve ALL existing mechanics (Starly summons, spells, keywords) — only the color changes
  - Nidoqueen's poison design is a PLACEHOLDER until C14 (Larvitar redesign) defines what moves off Tyranitar. Write the basic structure now, refine in C15.
  - Both Nidoran lines MUST be `col.n`
  - Applin had 3 branching T3 forms — Nidoqueen does NOT need 3 forms. Use standard 2-form T3.
- **Verification**:
  - Color m: Turtwig (P1) + Wailmer (P2, from C2c) — Turtwig still summons Starly
  - Color n: Nidoran-F (P1) + Nidoran-M (P2) — both poison/ground themed
  - No remnants of Rockruff, Applin, or old Turtwig position
- **If blocked**: If copying Turtwig's full definition between lines causes syntax issues, rebuild the Turtwig line from scratch on L29 using the audit's Line 75 details as reference.

#### Chunk C3: New Color Lines (e and j)
- **Concern**: Add two brand-new hero color lines
- **Lines touched**: Requires inserting new content — study how the textmod handles additional heropool entries. Likely appended to the heropool section or inserted at a specific position. Check Lines 13-99 structure.
- **Depends on**: Understanding of line structure from C1/C2 work
- **Source of truth**: SLICEYMON_AUDIT.md Lines 13-99 section, character selection system (L11)
- **CRITICAL NOTE**: The textmod has exactly 152 lines with even lines as blank spacers. Adding new lines may break the structure. Two options:
  - Option A: Find unused/blank lines in the 13-99 range to repurpose
  - Option B: Append after L99 if the game engine supports it (check if L101's "hidden level-up trigger" is positional)
  - **If neither works**: Embed new heroes into existing lines using the compound-line pattern (like L63 which contains multiple definitions)
- **Steps**:
  1. Determine insertion strategy (A, B, or compound) by studying how the game parses heropool lines
  2. Write Machop→Machamp (color e, P1): `replica.Lost`. Faces: T1-T2 use DoubleUse (24) + Engage (17) + SelfShield (51). T3 Machamp upgrades key faces to QuadUse (25). T1 Machop HP 5, T2 Machoke HP 8, T3 Machamp HP 10. **QuadUse is premium — only on T3.**
  3. Write Riolu→Lucario (color e, P2): `replica.Lost`. Faces: Damage Steel (41) + Engage (17) + Charged (42). T1 Riolu HP 4, T2 Riolu HP 7, T3 Lucario HP 9. Include Aura Sphere spell.
  4. Write Totodile→Feraligatr (color j, P1): `replica.Lost`. Faces: Engage (17) + Cruel (30) + Heavy (39). T1 Totodile HP 5, T2 Croconaw HP 7, T3 Feraligatr HP 10. **No Rampage** — Cruel + Engage + Heavy is the offensive budget.
  5. Write Poliwag→Poliwrath/Politoed (color j, P2): Branching T3. Poliwrath: Heavy (39) + Engage (17) + SelfShield (51). Politoed: Heal All (107) + Shield ManaGain (61) + Rain Dance spell. T1 Poliwag HP 4, T2 Poliwhirl HP 7, T3 HP 9.
- **Explicit constraints**:
  - Do NOT shift existing line numbers — this breaks ALL other line references
  - If inserting, use the blank even-numbered spacer pattern
  - Riolu appears in L109 as egg items ("Riolu x2") — those eggs must be removed in C4
- **Verification**:
  - Colors e and j appear in game draft
  - All 4 new heroes evolve correctly through all tiers
  - Poliwag branches into Poliwrath OR Politoed at T3
  - No existing heroes displaced or broken
- **If blocked**: If new lines cannot be added without breaking structure, use the compound-line approach on an existing line with spare capacity.

#### Chunk C4: Character Selection + Egg Cleanup
- **Concern**: Update draft picker and remove eggs for heroes that are now selectable
- **Lines touched**: L11 (character selection), L109 (items/eggs) (2 lines)
- **Depends on**: C1, C2a-C2e, C3 ALL complete
- **Source of truth**: SLICEYMON_AUDIT.md "Character Selection Draft System (Line 11)" and "Line 109 | 56 advanced items + eggs"
- **Steps**:
  1. **L11**: Update ALL 14 replacement hero entries (new names, new sprites, new descriptions). Add colors e and j to the draft picker with correct phase assignments.
  2. **L11**: Ensure color w now shows Bagon (P1) + Dratini (P2) instead of Ditto
  3. **L109**: Remove Togepi egg items (x6) — Togepi is now a selectable hero
  4. **L109**: Remove Riolu egg items (x2) — Riolu is now a selectable hero
  5. **L109**: Keep Bonsly eggs (x2) and Mystery Egg — these are NOT becoming heroes
- **Explicit constraints**:
  - Do NOT remove Bonsly or Mystery Egg items from L109
  - Every hero name in L11 must EXACTLY match the T1 name used in its heropool line
  - Colors e and j must appear in BOTH Phase 1 and Phase 2 draft pools (matching the P1/P2 pattern)
- **Verification**:
  - Draft picker shows all 24 colors with correct Pokemon names
  - No Togepi or Riolu eggs appear in item pool
  - Bonsly eggs and Mystery Egg still appear
  - Selecting every color in draft produces the correct P1 and P2 hero

#### Checkpoint 1: Hero Verification
After completing C1-C4, perform a FULL hero verification:
- [ ] All 24 colors selectable in draft
- [ ] All 21 new heroes evolve T1→T2→T3 without crashes
- [ ] All 23 unchanged heroes still work correctly
- [ ] No Ditto remnants anywhere
- [ ] No Togepi/Riolu eggs in item pool
- [ ] Turtwig still summons Starly under color m
- [ ] File loads without errors in Slice & Dice

---

### PHASE 2: Capture Changes

#### Chunk C5: Capture Removals
- **Concern**: Remove captures for Pokemon that are now heroes or bosses
- **Lines touched**: L111, L63 (2 lines)
- **Source of truth**: SLICEYMON_AUDIT.md "Capture/Summon System" table
- **Steps**:
  1. **L111**: Remove these captures: Ivysaur (Great Ball), Pikachu (Poke Ball), Charizard (Ultra Ball), Metagross (Heavy Ball), Poliwag (Lure Ball)
  2. **L63**: Remove Arceus/Master Ball capture from the compound line. KEEP Caterpie/Nest Ball capture AND Lillipup hero definition on this line.
  3. **L111**: Remove Arceus reference if present (audit shows Arceus on both L63 and L111)
- **Explicit constraints**:
  - L63 is a COMPOUND LINE (10,412 chars) — do NOT delete the entire line. Surgically remove only the Arceus capture definition.
  - KEEP all other captures on L111 (Snorlax, Mewtwo, Sneasel, Furret, Rattata, Delcatty, etc.)
  - After removal, L111 should still be valid syntax
- **Verification**:
  - Arceus no longer appears as a capturable Pokemon in item shops
  - Ivysaur, Pikachu, Charizard, Metagross, Poliwag no longer appear as captures
  - Snorlax, Mewtwo, Delcatty, and all other kept captures still work
  - Lillipup hero on L63 still functions
  - Caterpie capture on L63 still works

#### Chunk C6: Capture Upgrades and Replacements
- **Concern**: Upgrade 4 existing captures to final evolutions, replace 3 with different Pokemon, remove 1
- **Lines touched**: L111, L113, L63 (3 lines — L63 shared with C5; C5 MUST complete first since it removes Arceus from L63, then C6 upgrades Caterpie→Butterfree on same line)
- **Source of truth**: Capture pools in SLICEYMON_AUDIT.md
- **Steps**:
  1. **Upgrades** (keep same ball type, evolve to final form):
     - Caterpie → **Butterfree** (Nest Ball) — on L63 and/or L113
     - Sneasel → **Weavile** (Fast Ball) — L111
     - Barboach → **Whiscash** (Dive Ball) — L113
     - Electrike → **Manectric** (Quick Ball) — L113
  2. **Replacements** (same ball, entirely different Pokemon):
     - Rattata → **Skarmory** (Level Ball) — L111
     - Furret → **Lapras** (Friend Ball) — L111
     - Alcremie → **Arcanine** (Premier Ball) — L113
  3. **Removal**:
     - Remove Zubat capture (Dusk Ball) — L113. Zubat is now an enemy monster.
- **Explicit constraints**:
  - Ball types MUST stay the same for upgrades (the ball IS the item — changing it changes the item identity)
  - Skarmory replaces Rattata but is a completely different Pokemon — update all dice/HP/sprites
  - Lapras replaces Furret — update all dice/HP/sprites for Water/Ice typing
  - Each replacement needs new `.img.` sprite data (or placeholder)
- **Verification**:
  - Finding a Nest Ball gives Butterfree (not Caterpie)
  - Finding a Fast Ball gives Weavile (not Sneasel)
  - Skarmory appears via Level Ball with Steel/Flying stats
  - Lapras appears via Friend Ball with Water/Ice stats
  - Arcanine appears via Premier Ball
  - No Zubat capture exists (Zubat only appears as floor monster)
  - Total capture count: was 19, removed 6 (C5) + 1 (Zubat) = 12 remaining, after upgrades/replacements still 12
- **If blocked**: If a capture upgrade breaks (e.g., Butterfree's dice don't fit Caterpie's old slot), keep the old Pokemon temporarily and flag for manual fix. Do NOT skip — each upgrade is independent.

#### Chunk C7: New Ball Captures
- **Concern**: Add 8 new ball captures. Legendary items (Latias, Latios, Rayquaza, Suicune, Entei, Raikou) handled in C8.
- **Lines touched**: L111 and/or L113 (ball captures) (2 lines)
- **Source of truth**: Part 2 "Add 8 New Captures + 6 New Legendary Items" table
- **Steps**: Add each new ball capture with correct item, tier, dice design, and sprites:
  1. Mew (Poke Ball, T3) — L111/L113
  2. Jirachi (Great Ball, T4) — L111/L113
  3. Kangaskhan (Safari Ball, T5) — L111/L113
  4. Heracross (Net Ball, T6) — L111/L113
  5. Greninja (Dusk Ball, T6) — L111/L113
  6. Electivire (Ultra Ball, T6) — L111/L113
  7. Magmortar (Lure Ball, T6) — L111/L113
  8. Rhyperior (Heavy Ball, T7) — L111/L113
- **Explicit constraints**:
  - Study existing capture format on L111/L113 to match EXACTLY
  - Each capture needs: ball item definition + Pokemon hero definition (with dice, HP, sprites)
  - Tier determines when the ball appears in item shops — match existing tier patterns
  - All ball types are unique — no duplicates across existing and new captures
- **Verification**:
  - All 8 new ball captures appear at correct shop tiers
  - Each capture produces the correct Pokemon with appropriate stats
  - No ball type conflicts cause issues (test duplicate ball types)
  - Total ball captures after C5+C6+C7: 12 kept + 8 new = 20 ball captures. Legendary items (Latias, Latios, Rayquaza, Suicune, Entei, Raikou) are handled in C8.
- All ball types are now unique across the entire mod. No duplicates. Freed balls (Poke, Great, Dusk, Ultra, Lure, Heavy) reused for new captures. New ball types (Safari, Net) added for remaining.

#### Chunk C8: New Legendary Items (Latias, Latios, Rayquaza, Suicune, Entei, Raikou)
- **Concern**: Add 6 new legendary summons using the same system as Ho-Oh/Lugia/Kyogre/Groudon
- **Lines touched**: L115 and/or L117 (legendary summons) or new adjacent line (1-3 lines)
- **Source of truth**: SLICEYMON_AUDIT.md Lines 115, 117 (legendary summon format). Existing mechanics: Ho-Oh = "Revives 3 topmost defeated allies/turn, flees turn 7"; Kyogre = "Flees turn 7"; Groudon = "Has Geyser sub-form".
- **Pattern reference**: Study L115 (Ho-Oh/Rainbow Wing, Lugia/Silver Wing) and L117 (Kyogre/Blue Orb, Groudon/Red Orb) for EXACT format
- **Steps**:
  1. **Latias** (Soul Dew, T7): Defensive Eon twin. Faces: Shield to All (72) + Heal Cleanse (111) + Dodge (123). Mist Ball = weaken facade on enemies. Flees turn 7.
  2. **Latios** (Eon Flute, T7): Offensive Eon twin. Faces: Damage Ranged (46) + Damage Cleave (36) + Mana Pain (82). Luster Purge = vulnerable facade on enemies. Flees turn 7.
  3. **Rayquaza** (Jade Orb / Sky Pillar, T8): Dragon/Flying. Faces: Damage to All (34) + Damage Rampage (137) + Engage (17). Air Lock = removes enemy buffs via facade. Flees turn 7.
  4. **Suicune** (Clear Bell, T7): Water defensive/cleanse support. Faces: Shield (56) + Heal (103) + Heal Cleanse (111). HP 10. Purifies and protects the party. Flees turn 7.
  5. **Entei** (Flame Plate, T7): Fire damage + AoE. Faces: Damage (15) + Damage to All (34) + Heavy keyword. HP 9. Sacred Fire eruption. Flees turn 7.
  6. **Raikou** (Zap Plate, T7): Electric charged burst + speed. Faces: Damage Charged (42) + Damage Engage (17). HP 9. Lightning-fast strikes. Flees turn 7.
  7. Model all six after Kyogre/Groudon pattern: hero definition + summon trigger + flee timer
- **Explicit constraints**:
  - ALL six MUST follow the exact same structural pattern as L115/L117 legendary summons
  - These are NOT ball captures — they use legendary item slots (Soul Dew, Eon Flute, Jade Orb, Clear Bell, Flame Plate, Zap Plate), NOT the L111/L113 capture pool
  - These are HERO summons (fight on your side) — use hero-style Face IDs (15, 56, 103, etc.), NOT enemy-style (170/171)
  - Latias/Latios/Suicune/Entei/Raikou are T7, Rayquaza is T8
  - All six flee after ~7 turns like other legendaries
  - Latias is defensive (shields/heals/dodge), Latios is offensive (ranged damage/cleave) — must feel distinct from each other
  - Suicune is defensive/cleanse, Entei is offensive/AoE, Raikou is burst/speed — must feel distinct from each other and from the Eon twins
- **Verification**:
  - Soul Dew, Eon Flute, Jade Orb/Sky Pillar, Clear Bell, Flame Plate, and Zap Plate items appear in shops at correct tiers
  - Using each item summons the correct legendary as a temporary ally
  - Latias provides party-wide shields and healing; Latios deals AoE damage
  - Suicune provides shields + heals + cleanse; Entei deals damage + AoE; Raikou deals charged burst damage
  - All six flee after expected number of turns
  - Existing legendaries (Ho-Oh, Lugia, Kyogre, Groudon) still work correctly

#### Checkpoint 2: Capture Verification
After completing C5-C8:
- [ ] Total captures: 20 Pokemon via balls + 10 legendaries via orbs/wings/items (Ho-Oh, Lugia, Kyogre, Groudon, Rayquaza, Latias, Latios, Suicune, Entei, Raikou)
- [ ] No duplicate-ball conflicts causing crashes
- [ ] All ball items appear at correct shop tiers
- [ ] All legendary summons function correctly
- [ ] No removed captures still appearing

---

### PHASE 3: Monsters

#### Chunk C9: Regular Monster Additions
- **Concern**: Add new monsters to floor pools
- **Lines touched**: L119 (floors 1-3), L121 (floors 9-11), L123 (floors 17-19) (3 lines)
- **Source of truth**: Part 3 monster tables + SLICEYMON_AUDIT.md Monster Roster
- **Pattern reference**: Study existing monsters on L119 to see exact `.part.1` append format. Each monster needs: name, HP via `.hp.`, dice via `.sd=`, sprite via `.img.`
- **Steps**:
  1. **L119** (Floors 1-3): Append with `.part.1`: Zubat (HP 3), Tentacool (HP 4), Carvanha (HP 3), Chinchou (HP 4)
  2. **L121** (Floors 9-11): Append with `.part.1`: Golbat (HP 6), Tentacruel (HP 8), Sharpedo (HP 7), Lanturn (HP 7), Wild Steelix (HP 10)
  3. **L123** (Floors 17-19): Append with `.part.1`: Crobat (HP 9), Elite Steelix (HP 12), Absol (HP 8)
- **Explicit constraints**:
  - **ALL monster damage faces MUST use Face ID 170** (enemy-style Damage), NOT Face ID 15 (hero Damage). Cleave faces use 171 (enemy-style), NOT 36 (hero-style).
  - Monsters do NOT get Cantrip (face ID 126) — they don't reroll
  - Monsters do NOT get `.abilitydata.` spells
  - Use facade-based Engage + high pips on 170 for aggressive monsters instead of Cantrip
  - Status effects (poison, weaken, petrify, confuse) are applied via `.facade.` syntax, not via hero-style face IDs like 53/55
  - Append using `.part.1` — do NOT replace existing monsters
  - Study existing monster templates on L119 (Slimelet, Bee, Bones) for exact structural format before implementing
  - Steelix monsters named "Wild Steelix" (F9-11) and "Elite Steelix" (F17-19) to differentiate from Gen 3 boss "Alpha Steelix"
  - Zubat was removed as a capture in C6 — it is ONLY a monster now
- **Verification**:
  - Floors 1-3: Zubat, Tentacool, Carvanha, Chinchou appear as enemies
  - Floors 9-11: Golbat, Tentacruel, Sharpedo, Lanturn, Wild Steelix appear
  - Floors 17-19: Crobat, Elite Steelix, Absol appear
  - Existing monsters on all floors still spawn
  - No monster has Cantrip face
  - Zubat appears ONLY as monster, never as capture

#### Checkpoint 3: Monster Verification
- [ ] All 12 new monsters spawn at correct floor ranges
- [ ] No existing monsters removed
- [ ] Monster difficulty scales appropriately (HP 3-4 early, HP 7-10 mid, HP 8-12 late)

---

### PHASE 4: Bosses

#### Chunk C10: Boss Selection Menu Expansion
- **Concern**: Expand boss selection from 3 options to 6
- **Lines touched**: L135 (1 line)
- **Source of truth**: SLICEYMON_AUDIT.md "Line 135 | Boss selection menu"
- **Pattern reference**: Study L135 current format: "Gen 6 / Gen 7 / Random" selection system
- **Steps**:
  1. Expand L135 selection menu to: "Gen 3 / Gen 4 / Gen 5 / Gen 6 / Gen 7 / Random Mix"
  2. Each selection must route to the correct boss lines for floors 4, 8, 12, 16, 20
  3. "Random Mix" = each floor independently picks a random gen's boss
- **Explicit constraints**:
  - Do NOT break existing Gen 6 / Gen 7 routing — they must still work exactly as before
  - Gen 3/4/5 selections can initially route to placeholder bosses (existing Quagsire/Exeggutor for shared floors)
  - The menu MUST support 6 options — study how the current 3-option menu is structured
- **Verification**:
  - Boss selection shows 6 options
  - Selecting Gen 6 or Gen 7 produces the same bosses as before
  - Selecting Gen 3/4/5 does not crash (may show placeholders initially)
- **If blocked**: If the menu system cannot support 6 options, implement as nested selection (Region: Hoenn/Sinnoh/Unova/Kalos/Alola, then confirm).

#### Chunk C11: Gen 3 Boss Fights
- **Concern**: Implement all 5 Gen 3 boss encounters
- **Lines touched**: New boss definition lines. **Resolution strategy**: First check if `.part.1` appends work on L131/L133/L137/L139/L143 (preferred — no new lines needed). If not, study L147 (Necrozma uses a SINGLE line for ALL phases across floors) as an alternative pattern — all Gen 3 bosses could fit on 1-2 lines. Estimate 1-3 lines.
- **Depends on**: C10 (menu must route to these bosses)
- **Source of truth**: Part 4 "Gen 3" section + existing boss patterns on L131-L147
- **Pattern reference**: CRITICAL — Study each of these lines before writing ANY boss:
  - L131: Quagsire (Floor 4 format, minion pattern, HP values)
  - L133: Exeggutor (Floor 8 format, multi-body pattern)
  - L137: Xerneas (Floor 12 format, support enemy pattern)
  - L139/L141: Zygarde (Floor 16 format, cell mechanics)
  - L143/L145: Hoopa (Floor 20 format, phase/portal pattern)
  - L147: Necrozma (multi-phase across floors pattern)
- **Steps**:
  1. **Golem (F4)**: HP ~11-12 + Geodude minions (HP 3) + Graveler (HP 5-6). Self-destruct on Geodudes (single-use damage to all). Model after Quagsire (L131). **Quagsire is HP 11** — do not exceed this baseline for F4.
  2. **Alpha Steelix + Onix (F8)**: Alpha Steelix HP ~18 + Onix x2 (HP 6-8). Heavy shields + damage. Model after Exeggutor (L133). **Named "Alpha Steelix"** to differentiate from regular floor monsters.
  3. **Regi Trio OR Legendary Birds (F12, random selection)**: Either Regi Trio (Regirock/Regice/Registeel, HP ~12 each, total ~36) OR Legendary Birds (Articuno/Zapdos/Moltres, HP ~12 each, total ~36). Randomly selected per run. Both match F12 ceiling. Model after Xerneas+Florges (L137). Implement random selection similar to Gen 5 F12 Swords of Justice vs Forces of Nature pattern.
  4. **Regigigas (F16)**: HP ~20 + weakened Regi guardians: Regirock (HP 7), Regice (HP 6), Registeel (HP 8). Total ~41. Slow Start mechanic (stasis-like early turns, then Rampage). Regi trio serves as Regigigas's guardians (he is their creator). Model after Zygarde (L139/L141).
  5. **Deoxys (F20)**: Form changes — Normal/Attack/Defense/Speed. Model after Necrozma phases (L147).
- **Explicit constraints**:
  - Each boss MUST fit within the floor's expected difficulty curve (study existing boss HP at each floor)
  - Minions MUST use monster-style faces (170-series damage), NOT hero-style faces
  - Boss definitions MUST include `.flee.` or fight-ending conditions matching existing patterns
  - Geodude self-destruct = use single-use damage to all (similar to existing explosion mechanics)
- **Verification**:
  - Select Gen 3 in boss menu → Floor 4 shows Golem + Geodude/Graveler
  - Floor 8: Alpha Steelix + Onix swarm
  - Floor 12: Either all three Regis OR all three Legendary Birds appear (randomly selected)
  - Floor 16: Regigigas starts slow with Regi guardians (Regirock/Regice/Registeel), then rampages
  - Floor 20: Deoxys changes forms between phases
  - All 5 fights are completable (not impossibly hard or trivially easy)
- **If blocked**: Implement simpler versions first (single boss, no minions, no phase changes), then add complexity.

#### Chunk C12: Gen 4 Boss Fights
- **Concern**: Implement 3 Gen 4 boss encounters (Floors 12, 16, 20)
- **Lines touched**: Use same resolution strategy as C11. Prefer `.part.1` appends or single-line-all-phases (Necrozma L147 pattern). Estimate 1-2 lines.
- **Depends on**: C10 (menu routing)
- **Note**: Gen 4 Floors 4 and 8 intentionally reuse existing Quagsire and Exeggutor. This is a deliberate design choice — Quagsire/Exeggutor serve as "shared starter bosses" across all gen paths, providing a consistent early-game foundation before gen-specific bosses diverge at F12+.
- **Pattern reference**: Same as C11 — L137 (F12), L139/141 (F16), L143/145 (F20)
- **Steps**:
  1. **Palkia (F12)**: HP ~25 + support minions (Bronzong HP ~8, spatial distortion hazards HP ~5). Spatial Rend = Ranged (46) high damage. Model after Xerneas+Florges (L137). Solo boss with no minions would be a stat check — needs adds for targeting decisions.
  2. **Dialga (F16)**: HP ~25 + temporal minions (Bronzong HP ~8 with Stasis, temporal anomalies). Roar of Time = massive damage + Weaken (131). Model after Zygarde+Cells (L139/L141). Puzzle = managing adds that grant Dialga extra turns or stasis heroes.
  3. **Arceus (F20)**: HP 40+. Judgment = Damage to All (34-style). Type-shifting each turn changes damage type AND resistances. Model after Necrozma (L147) but with turn-by-turn shifts. Shifts should have tells/patterns, not be random. FINAL BOSS tier.
- **Explicit constraints**:
  - Arceus was removed as a capture in C5 — ensure NO capture remnants interfere with boss definition
  - Arceus type-shifting: study how Rotom (L49) handles form changes for inspiration, but implement as boss phases
  - Arceus HP 40+ makes it the highest-HP boss — ensure it's beatable at Normal difficulty
  - Palkia and Dialga MUST have minions/adds — solo bosses at F12/F16 with no minions are stat checks, not puzzles
- **Verification**:
  - Gen 4 path: Quagsire (F4) → Exeggutor (F8) → Palkia (F12) → Dialga (F16) → Arceus (F20)
  - Arceus type-shifts each turn with visible form change
  - All 3 fights completable at Normal difficulty
- **If blocked**: Implement Palkia and Dialga first (simpler single-boss fights). Arceus type-shifting is the hardest mechanic — if form changes are not achievable, implement Arceus as a single high-HP form with varied face types instead.

#### Chunk C13: Gen 5 Boss Fights
- **Concern**: Implement 4+ Gen 5 boss encounters (Floors 8, 12, 16, 20)
- **Lines touched**: Use same resolution strategy as C11/C12. Gen 5 has the most bosses (4+) so may need 2 lines (one for F8/F12, one for F16/F20). Estimate 1-3 lines.
- **Depends on**: C10 (menu routing)
- **Note**: Gen 5 Floor 4 reuses existing Quagsire
- **Pattern reference**: Same as C11/C12
- **Steps**:
  1. **Unova Starters (F8)**: Serperior (HP ~12, grass+shields) + Emboar (HP ~12, fire+rampage) + Samurott (HP ~12, water+cleave). Three simultaneous bosses. Model after Exeggutor/multi-body (L133).
  2. **Swords of Justice OR Forces of Nature (F12, random)**:
     - Path A: Cobalion + Terrakion + Virizion (HP ~12 each). **No Keldeo** — removed to stay within F12 HP budget.
     - Path B: Tornadus + Thundurus + Landorus (HP ~12 each). **HP reduced from 15 to 12** — 15x3=45 exceeds F12 budget.
     - Random selection between A and B each run
  3. **Reshiram/Zekrom (F16)**: One main boss (HP ~25), other as environmental effect. Model after Zygarde (L139/L141).
  4. **Kyurem (F20)**: Base Kyurem → Black/White Kyurem phase shift. Total HP ~35+. Model after Necrozma phases (L147).
- **Explicit constraints**:
  - F12 random selection between SoJ and FoN must be implemented as a branching path (study how Gen 6 X/Y variants work on L139 vs L141)
  - Three-boss fights (Unova starters, SoJ, FoN) total HP should not exceed F12 Xerneas+Florges total HP significantly
  - Reshiram/Zekrom environmental effect = passive buff/debuff, NOT a second boss the player fights
- **Verification**:
  - Gen 5 path: Quagsire (F4) → Unova Starters (F8) → SoJ or FoN (F12) → Reshiram or Zekrom (F16) → Kyurem (F20)
  - F12 randomly picks between Swords of Justice and Forces of Nature
  - Kyurem form-shifts mid-fight
  - All fights completable at Normal difficulty
- **If blocked**: Implement Unova Starters and Kyurem first (most unique). SoJ/FoN and Reshiram/Zekrom can follow as sub-chunks.

#### Checkpoint 4: Boss Verification
After completing C10-C13:
- [ ] All 6 boss menu options work (Gen 3/4/5/6/7/Random Mix)
- [ ] Gen 3: 5 unique boss fights across floors 4-20 (F12 randomly selects Regi Trio or Legendary Birds)
- [ ] Gen 4: 3 unique + 2 shared boss fights
- [ ] Gen 5: 4 unique + 1 shared boss fights
- [ ] Gen 6 and Gen 7: unchanged, still fully functional
- [ ] Random Mix: each floor independently picks from all 5 gens
- [ ] No boss fight crashes or soft-locks

---

### PHASE 5: Redesigns and Integration

#### Chunk C14: Larvitar→Tyranitar Redesign
- **Concern**: Remove poison theme from Tyranitar, replace with Rock/Dark identity
- **Lines touched**: L21 (1 line)
- **Depends on**: Should be done BEFORE C15 (Nidoqueen inherits the poison design being removed)
- **Source of truth**: SLICEYMON_AUDIT.md "Line 21 | Color h (huish) | Larvitar → Tyranitar"
- **Current state**: Tyranitar has poison, acidic, regen, duel, ego keywords. Doc says "Start Poisoned 1", "Immune to Poison".
- **Steps**:
  1. Remove ALL poison-related keywords and faces from L21: `poison`, `acidic`, face IDs 53, 54, 55, 145
  2. Remove "Start Poisoned 1" and "Immune to Poison" from doc/facade
  3. Replace with Rock/Dark identity:
     - Damage Heavy (39) + Damage Cruel (30) + Shield Steel (63)
     - Add Sandstorm via `.facade.` — "1 damage to all enemies each turn" (same pattern as Gigalith's "1 damage to all heroes and monsters each turn" on L73, but enemy-only). This is the passive chip damage that replaces the old poison-self-damage identity.
     - Keywords: `cruel`, `heavy`, `steel`
  4. Keep template `replica.Statue`, keep `col.h`, keep HP values similar (T1: 8, T2: varies, T3: 13/9)
- **Explicit constraints**:
  - Do NOT change Larvitar/Pupitar/Tyranitar names or evolution stages
  - Do NOT change the color assignment (col.h)
  - Do NOT change Lillipup→Stoutland (P2 on same color, different line L63)
  - SAVE the removed poison design details — they will be applied to Nidoqueen in C15
- **Verification**:
  - Tyranitar no longer starts poisoned
  - Tyranitar has Heavy + Cruel + Steel Shield faces
  - Tyranitar feels like a Rock/Dark bruiser, not a poison tank
  - Larvitar T1 and Pupitar T2 also updated (no poison on early stages)
  - Lillipup (P2) completely unaffected

#### Chunk C15: Nidoqueen Poison Inheritance
- **Concern**: Apply the poison design removed from Tyranitar onto Nidoqueen
- **Lines touched**: L31 (Nidoqueen, written in C2e with placeholder) (1 line)
- **Depends on**: C14 (must know exactly what poison mechanics were removed) AND C2e (Nidoqueen exists)
- **Steps**:
  1. Take the EXACT poison keywords and face patterns removed from Tyranitar in C14
  2. Apply to Nidoqueen's T3 forms: "Start Poisoned 1", "Immune to Poison", poison synergy faces
  3. Combine with Nidoqueen's existing Shield Repel (119) + Heavy (39) from C2e
  4. Ensure T1 Nidoran-F and T2 Nidorina also have appropriate poison flavor (lighter version)
- **Explicit constraints**:
  - Nidoqueen is Poison/GROUND — keep the Heavy (39) and Shield Repel (119) for ground identity
  - The poison design should feel like a TRANSFER, not a copy — Nidoqueen gets the tank-poison identity Tyranitar is losing
  - Do NOT give Nidoking (L75) the same poison tank design — Nidoking is offensive (Poison Plague + Engage + Cruel)
- **Verification**:
  - Nidoqueen starts poisoned, is immune to poison (like old Tyranitar)
  - Nidoqueen has both poison AND ground (heavy/repel) faces
  - Nidoking is distinctly different (offensive vs defensive)
  - Tyranitar (C14) no longer has ANY of these poison mechanics

#### Chunk C16: Final Integration Test
- **Concern**: Full end-to-end verification of all changes
- **Lines touched**: None (read-only verification)
- **Depends on**: ALL previous chunks (C1-C15)
- **Steps**:
  1. Paste FINAL textmod.txt into Slice & Dice
  2. Run through each verification item below
  3. Document any failures and trace back to the responsible chunk
- **Final Verification Checklist**:
  - [ ] **Heroes (21 new)**: Charmander, Beldum, Chikorita, Treecko, Bagon, Dratini, Machop, Riolu, Totodile, Poliwag, Cyndaquil, Bulbasaur, Mudkip, Wailmer, Weedle, Pichu, Torchic, Togepi, Cleffa, Nidoran-F, Nidoran-M — all evolve T1→T2→T3
  - [ ] **Heroes (1 redesign)**: Larvitar→Tyranitar now Rock/Dark (no poison)
  - [ ] **Turtwig**: Moved to color m, still summons Starly
  - [ ] **Colors**: All 24 colors (a-z minus d,f) appear in draft, including new e and j
  - [ ] **Removed**: No Ditto, no Digimon (Agumon/Tentomon), no Fomantis/Sunkern, no Trubbish/Garbodor, etc.
  - [ ] **Captures (20 ball total)**: 12 kept/upgraded + 8 new ball captures. Correct ball types. Correct tiers.
  - [ ] **Legendaries (10)**: Ho-Oh, Lugia, Kyogre, Groudon, Rayquaza, Latias, Latios, Suicune, Entei, Raikou — all summon correctly
  - [ ] **Monsters (12 new)**: Spawn at correct floor ranges. No Cantrip. Zubat only as monster.
  - [ ] **Bosses**: All 6 menu options work. Gen 3 (5 fights), Gen 4 (3+2 shared), Gen 5 (4+1 shared), Gen 6 (unchanged), Gen 7 (unchanged), Random Mix.
  - [ ] **Eggs**: No Togepi/Riolu eggs. Bonsly + Mystery Egg remain.
  - [ ] **Difficulty**: Test at Heaven, Normal, Hell — no crashes at any difficulty
  - [ ] **File integrity**: textmod.txt has correct line count (152 or 152+new lines), even lines are blank spacers, no encoding corruption
