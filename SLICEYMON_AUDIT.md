# Sliceymon Textmod - Complete Audit Reference

## File Structure (153 lines, ~415KB)
Even-numbered lines are blank spacers. All data on odd lines.

### Section Map
| Lines | Content |
|-------|---------|
| 1 | Party definition (starting heroes: "Berder" + "Curry") |
| 3 | Welcome message from Berder |
| 5 | Credits message from Curry |
| 7 | Art/community credits |
| 9 | Missingno (hidden hero, replica.Thief, col.d) |
| 11 | Character selection system (draft picker, 4 phases) |
| 13-99 | **44 heropool lines** (43 selectable colors + Missingno) |
| 101 | Hidden level-up trigger |
| 103 | Clear itempool command |
| 105 | 23 TM items |
| 107 | 88 consumable/held items |
| 109 | 56 advanced items + eggs (Togepi x6, Riolu x2, Bonsly x2, Mystery Egg) |
| 111 | Capture pool 1: 11 Pokemon via ball items |
| 113 | Capture pool 2: 8 Pokemon via ball items |
| 115 | Legendary summons: Ho-Oh (Rainbow Wing), Lugia (Silver Wing) |
| 117 | Legendary summons: Kyogre (Blue Orb), Groudon (Red Orb) |
| 119 | Monster pool: Floors 1-3 |
| 121 | Monster pool: Floors 9-11 |
| 123 | Monster pool: Floors 17-19 |
| 125 | Monster pool: Elites (Mareep, Gliscor) |
| 127 | Diglett monster (floors 5-7) |
| 129 | Diglett monster (floors 9-11) |
| 131 | Boss L4: Quagsire (Alpha) + Wooper/Seviper/Zangose/Ariados/Spinarak |
| 133 | Boss L8: Exeggutor + Sandile/Krookodile/Krokorok/Golett/Cofagrigus |
| 135 | Boss selection menu (Gen 6 / Gen 7 / Random) |
| 137 | Boss L12 Gen 6: Xerneas + Florges/Floette/Zoroark |
| 139 | Boss L16 Gen 6 X-variant: Zygarde + Cells |
| 141 | Boss L16 Gen 6 Y-variant: Zygarde + Cells |
| 143 | Boss L20 Gen 6 X-variant: Hoopa + Hand + Portal summons |
| 145 | Boss L20 Gen 6 Y-variant: Hoopa + Hand + Portal summons |
| 147 | Boss Gen 7: Necrozma (ALL phases L12/L16/L20) + Minior |
| 149 | Boss mods (no flee L16, horde L20) |
| 151 | Difficulty selection (Heaven/Easy/Normal/Hard/Unfair/Brutal/Hell) |
| 153 | End screen / "Send Teams" message |

---

## All 44 Hero Evolution Lines

### Character Selection Draft System (Line 11)
Players pick heroes by color letter. Phase 1 = first 5 party slots, Phase 2 = next 5 (different pool). Phases 3/4 repeat 1/2.

| Color Code | Color Name | Phase 1 (T1) | Phase 2 (T1) |
|------------|-----------|---------------|---------------|
| a | amber | Gible | Axew |
| b | blue | Vanillite | Spheal |
| c | cyan | Magikarp | Feebas |
| d | dark | Skip | — |
| g | grey | Slakoth | Varoom |
| h | huish | Larvitar | Lillipup |
| i | iuish | Eevee | Porygon |
| k | kuish | Litwick | Espurr |
| l | lime | Fomantis | Sunkern |
| m | mahogany | Rockruff | Roggenrola |
| n | green | Applin | Turtwig |
| o | orange | Larvesta | Trapinch |
| p | purple | Duskull | Trubbish |
| q | quish | Litten | Slugma |
| r | red | Darumaka | Happiny |
| s | sea | Squirtle | Slowpoke |
| t | tuish | Scyther | Burmy |
| u | uuish | Ralts | Tinkatink |
| v | violet | Noibat | Goomy |
| w | white | Ditto | Ditto |
| x | xuish | Rotom | Joltik |
| y | yellow | Honedge | Pawniard |
| z | zuish | Agumon (Digimon) | Tentomon (Digimon) |

### Detailed Hero Lines

#### Line 13 | Color a (amber) | Gible → Garchomp
- **Template**: replica.Lost
- **T1 Gible**: HP 5, sd=170-3:158-1:158-1:158-1:43:0
- **T2 Gabite** (2 forms): HP 7/8
- **T3 Garchomp** (2 forms): HP 11/9
- Keywords: scared, pain, overdog (via facade)
- Doc: "Rough Skin - On-Hit damage the attacker for 2"
- Speech: Gib!/Gab!/Raugh!

#### Line 15 | Color b (blue) | Vanillite → Vanilluxe
- **Template**: replica.Statue
- **T1 Vanillite**: HP 4, sd=93-3:93-3:93-3:93-3:93-2:93-2 (all mana single-use)
- **T2 Vanillish** (2 forms): HP 7
- **T3 Vanilluxe** (2 forms): HP 8/10
- Has 5 abilitydata spells (Icy Wind, Ice Beam, Hail, Sheer Cold, Blizzard)
- Doc: "[plus] Pips Minimum of 1"

#### Line 17 | Color c (cyan) | Magikarp → Gyarados
- **Template**: replica.Lost
- **T1 Magikarp**: HP 5, sd=96-1:6:6:6:6:6 (1 face + 5 stasis blanks!)
- **T2 Magikarp** (still Magikarp!): HP 6, nearly all blanks
- **T3 Gyarados** (2 forms): HP 13/13 - MASSIVE payoff
  - Form 1: 170-5:170-5:137-3:137-3:20-3:20-3
  - Form 2: 21-10:21-10:23-4:23-4:27-2:27-2
- Keywords: fierce (via facade)

#### Line 19 | Color g (grey) | Slakoth → Slaking
- **Template**: replica.Lost
- **T1 Slakoth**: HP 5, sd=170-3:170-3:170-2:170-2:56-2:56-2
- **T2 Vigoroth** (2 forms): HP 7/8
- **T3 Slaking** (2 forms): HP 12/11
- Keywords: exert, cantrip
- Doc: "Plus 1 to sides turn one", "Takes a Quick Nap"

#### Line 21 | Color h (huish) | Larvitar → Tyranitar
- **Template**: replica.Statue
- **T1 Larvitar**: HP 8, sd=39-1:39-1:56-1:56-1:0:12-0
- **T2 Pupitar** (2 forms)
- **T3 Tyranitar** (2 forms): HP 13/9
- Keywords: poison, acidic, regen, duel, ego (via facades)
- Doc: "[plus] Start Poisoned 1", "[plus] Immune to Poison"

#### Line 23 | Color i (iuish) | Eevee → 8 Eeveelutions
- **Template**: replica.Lost
- **LARGEST hero line** (9,537 chars)
- **T1 Eevee**: HP 5 (8 different T1 variants)
- **T3 forms** (8 total):
  - Jolteon: HP 9, damage+heal+dodge
  - Flareon: damage+shield+dodge
  - Vaporeon: HP 11, heal+heal regen+dodge (+ Aqua Ring spell)
  - Umbreon: damage+shield+dodge
  - Espeon: damage+pain (+ Cosmic Power spell)
  - Leafeon: HP 9, mana+shield+heal cleanse (+ Grass Knot spell)
  - Glaceon: death+redirect+shield+revive
  - Sylveon: heal rescue+heal cleave+heal cleanse+dodge (+ Skill Swap spell)
- 28 dice sets total, 5 abilitydata spells

#### Line 25 | Color k (kuish) | Litwick → Chandelure
- **Template**: replica.Statue
- **T1 Litwick**: HP 5
- **T2 Lampent** (2 forms): HP 7
- **T3 Chandelure** (2 forms): HP 9
- Has spells: Rejuvenate, Sacrifice, Safeguard, Curse, Spite
- Keywords: inflictsingleuse, vigil, damage, pain, inflictdeath
- Doc: "[red]All Monsters:[white] Gain 1 mana on Death"

#### Line 27 | Color l (lime) | Fomantis → Lurantis
- **Template**: replica.Lost
- **T1 Fomantis**: HP 4
- **T2 Fomantis** (2 forms): HP 6
- **T3 Lurantis** (2 forms): HP 8
- Keywords: exert, growth (via facades)
- Has abilitydata spells

#### Line 29 | Color m (mahogany) | Rockruff → Lycanroc
- **Template**: replica.Lost
- **T1 Rockruff**: HP 5
- **T2 Rockruff** (2 forms): HP 8/7
- **T3 Lycanroc** (3 forms - Midday/Midnight/Dusk): HP 9
- Keywords: run, selfheal, pain

#### Line 31 | Color n (green) | Applin → Flapple/Appletun/Hydrapple
- **Template**: replica.Statue
- **T1 Applin** (3 variants): HP 5/7
- **T3 forms** (3 branching):
  - Flapple: HP 8, damage+poison+mana
  - Appletun: HP 11, heal cleanse+heal vitality+heal rescue
  - Hydrapple: HP 9, quad-use+mana+damage selfheal
- Keywords: duel, critical, duplicate, quin, cleave
- Has spells: Apple, Grav Apple, Pie, Dragon Cheer

#### Line 33 | Color o (orange) | Larvesta → Volcarona
- **Template**: replica.Thief
- **T1 Larvesta**: HP 5
- **T2 Larvesta** (2 forms): HP 9/6
- **T3 Volcarona** (2 forms): HP 9/12
- Keywords: era, vitality, singleuse, inflictpain
- Has spells: Cocoon, Warmth, Quiver Dance, Flamethrower

#### Line 35 | Color p (purple) | Duskull → Dusknoir
- **Template**: replica.Lost
- **T1 Duskull**: HP 5
- **T2 Dusclops** (2 forms): HP 6
- **T3 Dusknoir** (2 forms): HP 8/9
- Keywords: possessed, terminal, selfheal, picky, managain
- Has spells: Payback, Cull, Shadow Ball, Reap, Pain Split
- 13 dice sets - most complex hero

#### Line 37 | Color q (quish) | Litten → Incineroar
- **Template**: replica.Statue
- **T1 Litten**: HP 4
- **T2 Torracat** (2 forms): HP 7
- **T3 Incineroar** (2 forms): HP 10
- Keywords: moxie
- Facade: bas34:8:20:20

#### Line 39 | Color r (red) | Darumaka → Darmanitan
- **Template**: replica.Statue
- **T1 Darumaka**: HP 6
- **T2 forms**: Darumaka (HP 9), DarumakaG/Galarian
- **T3 forms**: Darmanitan (HP 10), DarmanitnG/Galarian (HP 10)
- Keywords: fumble, singleuse, onesie

#### Line 41 | Color s (sea) | Squirtle → Blastoise
- **Template**: replica.Lost
- **T1 Squirtle**: HP 5
- **T2 Wartortle** (2 forms): HP 8
- **T3 Blastoise** (2 forms): HP 10/9
- Has spells: Bubble, Water Gun, Shell Smash, Hydro Pump, Surf
- 10 dice sets (alternating primary/spell)

#### Line 43 | Color t (tuish) | Scyther → Scizor/Kleavor
- **Template**: replica.Lost
- **T1 Scyther**: HP 5
- **T2 Scyther** (2 forms): HP 8/7
- **T3 forms** (3): Scizor (HP 8), Scyther stays (HP 9), Kleavor (HP 10)
- Keywords: damage (via facade)

#### Line 45 | Color u (uuish) | Ralts → Gardevoir/Gallade
- **Template**: replica.Statue
- **T1 Ralts**: HP 3
- **T2 Kirlia** (2 forms): HP 6
- **T3 forms**: Gallade (HP 9), Gardevoir (HP 8)
- Has spells: Confusion, Disable, Helping Hand, Psycho Cut, Future Sight
- Keywords: sticky, inflictsingleuse, duplicate, ranged, focus

#### Line 47 | Color v (violet) | Noibat → Noivern
- **Template**: replica.Lost
- **T1 Noibat**: HP 1 (echo mechanic - some forms 1 HP)
- **T2 Noibat** (multiple echo forms): HP 5
- **T3 Noivern** (2 forms + echoes): HP 7
- Keywords: damage, ranged, underdog, echo, resonate

#### Line 49 | Color x (xuish) | Rotom → 5 Appliance Forms
- **Template**: replica.Lost
- **T1 Rotom**: HP 4
- **T2 Rotom** (Chef/Sparky): HP 6
- **T3 forms** (5):
  - RTM Frost (Blizzard): HP 10
  - RTM Mow (Leaf Storm): HP 9
  - RTM Heat (Overheat): HP 9
  - RTM Wash (Hydropump): HP 9
  - RTM Fan (Air Slash): HP 6
- Keywords: focus, cleave, weaken, wither, vulnerable

#### Line 51 | Color y (yellow) | Honedge → Aegislash
- **Template**: replica.Thief
- **T1 Honedge**: HP 5
- **T2 Doublade** (2 forms): HP 7/5
- **T3 Aegislash** (2 forms, stance-switching): HP 9
- Keywords: cantrip, generous, possessed, stasis, bloodlust, exert, deathwish, inspired, rescue
- Doc: "Switches Between Forms Every Turn"

#### Line 53 | Color z (zuish) | Agumon → MGreymon/SkullGreymon (DIGIMON)
- **Template**: replica.Statue
- **T1 Agumon**: HP 5
- **T2 Greymon**: HP 8
- **T3 forms**: MGreymon (HP 8/10), SkullGreymon (HP 9)
- Keywords: damage, cruel, managain, selfheal
- Doc: "All Monsters Except Bones: Upon Death: summon a bones"

#### Line 55 | Color a (amber) | Axew → Haxorus
- **Template**: replica.Lost
- **T1 Axew**: HP 5
- **T2 Fraxure** (2 forms): HP 8
- **T3 Haxorus** (2 forms): HP 10
- Keywords: pain, decay
- Doc: "[plus] (every 5th hp) +1 pip to all sides this fight"

#### Line 57 | Color b (blue) | Spheal → Walrein
- **Template**: replica.Sphere (UNIQUE - only Sphere user!)
- **T1 Spheal**: HP 6
- **T2 Sealeo** (2 forms): HP 7
- **T3 Walrein** (2 forms): HP 10
- Keywords: hypergrowth, fumble, lead, cleave

#### Line 59 | Color c (cyan) | Feebas → Milotic
- **Template**: replica.Lost
- **T1 Feebas**: HP 5, sd=93-2:6:6:6:6:6 (1 face + 5 stasis blanks!)
- **T2 Feebas** (still Feebas): HP 6, nearly all blanks
- **T3 Milotic** (2 forms): HP 8/10 - strong payoff
- Keywords: pristine, damage, doubleuse
- Doc: "[plus] +1 pip to all sides this fight (every 5th hp)"

#### Line 61 | Color g (grey) | Varoom → Revavroom
- **Template**: replica.Statue
- **T1 Varoom**: HP 5
- **T2 Varoom**: HP 7
- **T3 Revavroom** (2 forms): HP 10/8
- Keywords: engine, heal, selfheal, poison, descend, era

#### Line 63 | Color h (huish) | Lillipup → Stoutland + Items/Arceus
- **Template**: replica.Lost
- **THE COMPOUND LINE** (10,412 chars) - contains items, Arceus capture, AND Lillipup line
- **Lillipup line**: HP 7 → Herdier HP 7 → Stoutland HP 10
- Keywords: fierce, cantrip, lead, underdog, overdog
- "Pickup" ability provides random items
- Also contains: R4r3 C4ndy items, TMs, Arceus/Master Ball, Caterpie/Nest Ball

#### Line 65 | Color i (iuish) | Porygon → Porygon-Z
- **Template**: replica.Statue
- **T1 Porygon**: HP 4
- **T2 Porygon** (2 forms): HP 6
- **T3 Porygon-Z** (+ Hyper Beam spell): HP 7
- 15 facades! Most facades of any hero
- Items: Golden Cup

#### Line 67 | Porygon Helper (NOT a heropool)
- Hidden helper providing Upgrade item for Porygon T3

#### Line 69 | Color k (kuish) | Espurr → Meowstic
- **Template**: replica.Lost
- **T1 Espurr**: HP 4
- **T2 Espurr** (2 forms): HP 6
- **T3 Meowstic** (2 forms): HP 8
- Has spells: Reflect, Disarming Voice, Extrasensory
- Keywords: lucky, singleuse, managain, manacost, cruel

#### Line 71 | Color l (lime) | Sunkern → Sunflora
- **Template**: Primrose (UNIQUE - not a replica!)
- **T1 Sunkern**: HP 6
- **T2 Sunkern** (2 forms): HP 6
- **T3 Sunflora** (2 forms): HP 7
- Has spells: Growth, Leafer, Cultivate, Reap
- Keywords: era, boost, groupgrowth, growth

#### Line 73 | Color m (mahogany) | Roggenrola → Gigalith
- **Template**: replica.Lost
- **T1 Roggenrola**: HP 5
- **T2 Boldore** (2 forms): HP 6
- **T3 Gigalith** (2 forms): HP 7
- Keywords: death, swapcruel, deathwish
- Doc: "Reduce damage taken by 1; 1 damage to all heroes and monsters each turn"

#### Line 75 | Color n (green) | Turtwig → Torterra (+Starly summons)
- **Template**: replica.Lost
- **T1 Turtwig**: HP 5
- **T2 Grotle** (2 forms): HP 7/8
- **T3 Torterra** (2 forms): HP 10
- **Summons Starly** birds! (HP 18/14)
- Keywords: singleuse, growth, undergrowth, eliminate
- Doc: "Pokemon like to nest on Torterra"

#### Line 77 | Color o (orange) | Trapinch → Flygon
- **Template**: replica.Lost
- **T1 Trapinch**: HP 6
- **T2 Vibrava** (2 forms): HP 7/6
- **T3 Flygon** (2 forms): HP 8
- Keywords: treble, echo
- Doc: "Digs underground every other turn"

#### Line 79 | Color p (purple) | Trubbish → Garbodor
- **Template**: replica.Statue
- **T1 Trubbish**: HP 5
- **T2 Trubbish** (2 forms): HP 8/12
- **T3 Garbodor** (2 forms): HP 13/12
- Keywords: poison, onesie, plague, selfrepel
- Doc: "Start Poisoned one/Two"

#### Line 81 | Color q (quish) | Slugma → Magcargo
- **Template**: replica.Lost
- **T1 Slugma**: HP 2 (LOWEST!)
- **T2 Slugma** (2 forms): HP 2/5
- **T3 Magcargo** (2 forms): HP 2/8
- Keywords: steel, cruel, pain
- Doc: "+1 to incoming shields", "self-shield 4 each turn"
- Complex triggerhpdata mechanics

#### Line 83 | Color r (red) | Happiny → Blissey
- **Template**: replica.Healer (UNIQUE - only Healer user!)
- **T1 Happiny**: HP 9
- **T2 Chansey** (2 forms): HP 8/12
- **T3 Blissey** (2 forms): HP 12
- Keywords: critical (via facades)
- Pure healer line

#### Line 85 | Color s (sea) | Slowpoke → Slowbro/Slowking
- **Template**: replica.Lost
- **T1 Slowpoke**: HP 6 (7 "Slack Off" spell variants!)
- **T2 Slowpoke** (2 forms): HP 8
- **T3 forms**: Slowbro (HP 28/3!), Slowking (HP 10/8)
- 11 abilitydata spells! Most spells of any hero
- Keywords: era, cleanse, singleuse, focus, plague, poison

#### Line 87 | Color t (tuish) | Burmy → Wormadam/Mothim
- **Template**: replica.Lost
- **T1 Burmy**: HP 4
- **T2 Burmy** (3 cloak forms): HP 7
- **T3 forms** (4): Wormadam Plant/Sandy/Trash (HP 10), Mothim (HP 8)
- Keywords: fumble, decay, critical, poison

#### Line 89 | Color u (uuish) | Tinkatink → Tinkaton
- **Template**: replica.Lost
- **T1 Tinkatink**: HP 5
- **T2 Tinkatuff** (2 forms): HP 8
- **T3 Tinkaton** (2 forms): HP 10
- Keywords: scared, permaboost, undergrowth, fumble, selfshield, ranged
- Doc: "[plus] Keep unused shields"

#### Line 91 | Color v (violet) | Goomy → Goodra
- **Template**: replica.Statue
- **T1 Goomy**: HP 4
- **T2 Sliggoo**: HP 8 (+ SliggooH/Hisuian: HP 9)
- **T3 forms**: Goodra (HP 10), GoodraH/Hisuian (HP 12)
- Keywords: sticky, cleanse, selfshield
- Has spells: Hydration, Rain Dance

#### Line 93 | Color x (xuish) | Joltik → Galvantula
- **Template**: replica.Lost
- **T1 Joltik**: HP 4
- **T2 Joltik** (2 forms): HP 6/9
- **T3 Galvantula** (2 forms): HP 6
- Keywords: pain, singleuse, manacost, inflictexert
- Has spells: Stringshot, Thunder Wave, Sticky Web, Discharge

#### Line 95 | Color y (yellow) | Pawniard → Kingambit
- **Template**: replica.Lost
- **T1 Pawniard**: HP 9 (high for T1!)
- **T2 Bisharp** (2 forms): HP 11/9
- **T3 forms**: Bisharp (HP 11), Kingambit
- **Pawniard minion summoning mechanic**
- Keywords: halvedeathwish, duel, rescue, rite, vigil
- Doc: "Bisharp would rather die than live with defeat"

#### Line 97 | Color z (zuish) | Tentomon → MegaKabuterimon (DIGIMON)
- **Template**: replica.Lost
- **T1 Tentomon**: HP 5
- **T2 forms**: Ballistamon (HP 5), Kabuterimon (HP 9)
- **T3 forms**: MetalKabuterimon (HP 5), MegaKabuterimon (HP 13)
- Keywords: steel, picky
- Has spells: Protect, Essence Storm, Disperse, Electron Cannon, Super Charge

#### Line 99 | Color w (white) | Ditto (WILDCARD)
- **Template**: replica.Lost
- **THE LARGEST LINE** (66,309 chars!)
- **T1 Ditto**: HP 3
- Doc: "Copy the above hero's top and bottom sides and the below hero's middle side"
- Contains **copies of ~130+ Pokemon forms** (every other hero's T3 forms)
- All copies have [g] grey speech prefixes and "?" suffixed names

---

## Capture/Summon System

### 19 Capturable Pokemon (via Ball Items)
| Pokemon | Ball Type | Item Tier | Source Line |
|---------|-----------|-----------|------------|
| Arceus | Master Ball? | 0 | 63, 111 |
| Caterpie | Nest Ball | 1 | 63, 113 |
| Pikachu | Poke Ball | 3 | 111 |
| Snorlax | Dream Ball | 4 | 113 |
| Ivysaur | Great Ball | 5 | 111 |
| Barboach | Dive Ball | 6 | 113 |
| Lilligant | Luxury Ball | 6 | 113 |
| Wobbuffet | Timer Ball | 6 | 113 |
| Zubat | Dusk Ball | 6 | 113 |
| Electrike | Quick Ball | 6 | 113 |
| Charizard | Ultra Ball | 7 | 111 |
| Alcremie | Premier Ball | 8 | 113 |
| Mewtwo | Master Ball | 9 | 111 |
| Sneasel | Fast Ball | special | 111 |
| Furret | Friend Ball | special | 111 |
| Metagross | Heavy Ball | special | 111 |
| Rattata | Level Ball | special | 111 |
| Poliwag | Lure Ball | special | 111 |
| Delcatty | Moon Ball | special | 111 |

### 4 Legendary Summons (via Wing/Orb Items)
| Pokemon | Item | Line | Notes |
|---------|------|------|-------|
| Ho-Oh | Rainbow Wing | 115 | Revives 3 topmost defeated allies/turn, flees turn 7 |
| Lugia | Silver Wing | 115 | |
| Kyogre | Blue Orb | 117 | Flees turn 7 |
| Groudon | Red Orb | 117 | Has Geyser sub-form |

---

## Monster Roster (~50+ unique types)

### Floor 1-3 Pool (Line 119)
Wooper, Skorupi, Dreepy, Rattata, Pincurchin, Grunt, Raticate, Primeape, Gastly, Scrafty, Simipour, Orthworm, Vigoroth, Carbink, Ekans, Stonjourner, Magnezone, Bewear, Rhyhorn, Swalot, Gulpin

### Floor 9-11 Pool (Line 121)
All of above PLUS: Cubone (with grave sub-form), Impidimp, Haunter, Drakloak, Skuntank, Annihilape, Parasect, Banette, Ferrothorn, Alakazam, Fearow, Golett, Mightyena, Poochyena, Stunfisk, Druddigon, Grimmsnarl, Hydreigon, Aggron

### Floor 17-19 Pool (Line 123)
Gastly (Shade), Drakloak, Duskull, Annihilape, Orthworm, Parasect, Banette, Stunfisk, Druddigon, Grimmsnarl, Impidimp, Hydreigon, Ferrothorn, Aggron, Alakazam, Cubone

### Elite Monsters (Line 125)
Mareep (petrify), Gliscor (HP 12, very powerful)

### Special Monsters
Diglett (floors 5-11, Lines 127/129)

---

## Boss Encounters

| Floor | Boss | Supporting Enemies |
|-------|------|--------------------|
| 4 | Quagsire (Alpha, HP 11) | Wooper, Seviper, Zangose, Ariados, Spinarak, Pond |
| 8 | Exeggutor (multi-neck forms) | Sandile, Krookodile, Krokorok, Golett, Cofagrigus |
| 12 (Gen 6) | Xerneas (HP 25) | Florges (HP 10), Floette, Zoroark (HP 18, disguised as Yveltal) |
| 12 (Gen 7) | Necrozma Phase 1 | Minior minions |
| 16 (Gen 6) | Zygarde (HP 25) | Zygarde Cells (HP 8-10) |
| 16 (Gen 7) | Necrozma Phase 2 | Minior minions |
| 20 (Gen 6) | Hoopa (HP 30) | Hand entities, Portal summons |
| 20 (Gen 7) | Necrozma Phase 3 "The Blinding One" | Minior, escalating forms |

---

## Template Reference

| Template | Base Color | Default HP | Notes | Used By |
|----------|-----------|-----------|-------|---------|
| replica.Lost | Orange | 3 | Most flexible generic base, override everything | 24 heroes (most common) |
| replica.Statue | Grey | 20 | All-blank faces, pure customization canvas | 12 heroes |
| replica.Thief | Orange | 4 | Ranged damage base | 4 heroes |
| replica.Sphere | Blue | varies | Mana-focused | 1 hero (Spheal only) |
| replica.Prodigy | Blue | varies | Single-use mana burst | Within some heroes |
| replica.Fighter | Yellow | 5 | Balanced damage+shield | 2 heroes |
| Primrose | Green | 5 | Growth-focused, NOT a replica | 1 hero (Sunkern only) |
| replica.Housecat | Green | 3 | Never-levels mechanic | Within Ditto copies |
| replica.Healer | Red | 5 | Healing+mana with Mend spell | 1 hero (Happiny only) |

---

## Key Face IDs Quick Reference

| ID | Type | ID | Type |
|----|------|----|------|
| 0 | Blank | 103 | Heal |
| 6 | Blank (Stasis) | 105 | Heal Vitality |
| 12 | Self-damage Cantrip | 106 | Heal Rescue |
| 15 | Damage | 107 | Heal All |
| 17 | Damage Engage | 108 | Heal Boost |
| 18 | Damage ManaGain | 109 | Heal Cleave |
| 19 | Damage Pain | 110 | Heal Regen |
| 20 | Damage Deathwish | 111 | Heal Cleanse |
| 21 | Damage Death | 112 | Heal ManaGain |
| 23 | Damage Exert | 113 | Heal Growth |
| 24 | Damage DoubleUse | 114 | Heal DoubleUse |
| 25 | Damage QuadUse | 115 | Damage Single-use |
| 27 | Damage Copycat | 117 | Undying |
| 28 | Damage Pristine | 118 | Redirect SelfShield |
| 29 | Damage Guilt | 119 | Shield Repel |
| 30 | Damage Cruel | 123 | Dodge |
| 31 | Damage Shifter | 124 | Dodge Cantrip |
| 34 | Damage to All | 125 | Reroll Cantrip |
| 36 | Damage Cleave | 126 | Damage Cantrip |
| 37 | Damage Descend | 130 | Reuse |
| 38 | Damage Cleave Chain | 131 | Damage Weaken |
| 39 | Damage Heavy | 132 | Damage Duplicate |
| 41 | Damage Steel | 133 | Shield Duplicate |
| 42 | Damage Charged | 134 | Mana Duplicate |
| 43 | Stun Bully | 135 | Damage Ranged Engage |
| 44 | Damage Vulnerable | 136 | Revive |
| 45 | Damage Era | 137 | Damage Rampage |
| 46 | Damage Ranged | 145 | Add Poison |
| 47 | Damage Ranged Poison | 151 | Add Growth |
| 48 | Damage Ranged Duplicate | 158 | Damage to ALL Rampage |
| 50 | Damage Ranged Copycat | 160 | Damage to ALL Charged ManaCost |
| 51 | Damage SelfShield | 162 | Heal Boost InflictPain |
| 52 | Damage SelfHeal | 168 | Damage Eliminate |
| 53 | Damage Poison | 170 | Damage (Enemy-style) |
| 54 | Damage to ALL Poison | 171 | Damage Cleave (Enemy-style) |
| 55 | Damage Poison Plague | 174 | Damage Defy |
| 56 | Shield | 175 | Damage Critical |
| 57 | Shield Flesh | 176-187 | Targeting faces |
| 58 | Shield Growth | 181 | Target Enemy (Pips) |
| 61 | Shield ManaGain | 187 | Target Self (Pips) |
| 62 | Shield DoubleUse | | |
| 63 | Shield Steel | | |
| 64 | Shield Rescue | | |
| 65 | Shield Pristine | | |
| 66 | Shield Cantrip | | |
| 67 | Shield Copycat | | |
| 69 | Shield Cleave | | |
| 70 | Shield Charged | | |
| 71 | Shield Cleanse | | |
| 72 | Shield to All | | |
| 76 | Mana | | |
| 77 | Mana Cantrip | | |
| 79 | Mana Growth | | |
| 80 | Mana Decay | | |
| 81 | Mana Death | | |
| 82 | Mana Pain | | |
| 84 | Mana Pair | | |
| 85 | Mana Trio | | |
| 87 | Heal Shield ManaGain | | |
| 88 | Damage Single-use Charged | | |
| 89 | Damage SU InflictPain | | |
| 90 | Damage SU Cruel | | |
| 91 | Damage SU Poison | | |
| 92 | Damage SU SelfHeal | | |
| 93 | Mana Single-use | | |
| 94 | Shield SU PermaBoost | | |
| 95 | Damage SU Weaken | | |
| 96 | Damage SU Fierce | | |
| 97 | Damage SU Echo | | |
| 100 | Stun Single-use | | |

---

## Balance Guidelines

### HP Ranges in Sliceymon
| Tier | Low | Typical | High | Extreme |
|------|-----|---------|------|---------|
| T1 | 1 (Noibat) | 4-5 | 8 (Larvitar) | 9 (Happiny, Pawniard) |
| T2 | 2 (Slugma) | 6-8 | 9 (Larvesta) | 12 (Trubbish, Chansey) |
| T3 | 2 (Magcargo) | 8-10 | 12-13 (Gyarados, Tyranitar, Garbodor) | 28 (Slowbro!) |

### Blank Face Guidelines
- 0 blanks: Very powerful, needs drawback keywords (Pain, Exert, Death)
- 1 blank: Strong, standard for T3
- 2 blanks: Average, standard for T2
- 3 blanks: Weak early, standard for T1
- 4+ blanks: Very weak, needs massive payoff (Magikarp→Gyarados, Feebas→Milotic)
- ALL blanks: Statue-style, relies on items

### Dice Design Patterns from Existing Heroes
- **DPS heroes**: Mostly damage faces (15, 17, 30, 34, 46) + 0-2 blanks
- **Tanks**: Shield faces (56, 63, 65, 119) + Redirect (118) + heavy damage (39)
- **Healers**: Heal faces (103, 105, 109, 110, 111) + Mana (76) + shield
- **Spellcasters**: Mana faces (76, 82, 84, 93) + spell via .abilitydata.
- **Utility/Green**: Mix of everything + unique mechanics (growth, summons, echo)
