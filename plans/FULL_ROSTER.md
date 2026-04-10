# Sliceymon+ Full Pokemon Roster

> **Source of truth** for all Pokemon assignments in Sliceymon+.
> Last updated: 2026-04-06

## Rules

1. A Pokemon may appear in exactly ONE category (Hero, Monster, Boss, Capture/Legendary)
2. Hero evolutions (e.g. Charizard for Charmander line) count as Hero — not eligible for Capture
3. Monsters do not overlap with Heroes or Captures
4. Bosses do not overlap with Captures (Arceus = Boss, not Capture)

---

## Heroes

| Color | P1 | Template | P2 | Template | Status |
|-------|----|----------|----|----------|--------|
| A | Gible→Garchomp | Lost | Axew→Haxorus | Lost | ORIG |
| B | Snorunt→Glalie/Froslass | Eccentric | Spheal→Walrein | Sphere | NEW / ORIG |
| C | Magikarp→Gyarados | Lost | Feebas→Milotic | Lost | ORIG |
| F | Machop→Machamp | Fighter | Riolu→Lucario | Lost | NEW / NEW |
| G | Slakoth→Slaking | Lost | Mudkip→Swampert | Guardian | ORIG / NEW |
| H | Larvitar→Tyranitar | Statue | Aron→Aggron | Stalwart | CHG / NEW |
| I | Eevee→Eeveelutions | Lost | Porygon→Porygon-Z | Statue | ORIG |
| J | Totodile→Feraligatr | Fighter | Poliwag→Poliwrath/Politoed | Fighter | NEW / NEW |
| K | Litwick→Chandelure | Statue | Torchic→Blaziken | Thief | ORIG / NEW |
| L | Chikorita→Meganium | Healer | Treecko→Sceptile | Primrose | NEW / NEW |
| M | Turtwig→Torterra | Lost | Wailmer→Wailord | Stalwart | CHG / NEW |
| N | NidoranF→Nidoqueen | Statue | NidoranM→Nidoking | Lost | NEW / NEW |
| O | Larvesta→Volcarona | Thief | Trapinch→Flygon | Lost | ORIG |
| P | Duskull→Dusknoir | Lost | Bulbasaur→Venusaur | Guardian | ORIG / NEW |
| Q | Litten→Incineroar | Statue | Cyndaquil→Typhlosion | Lost | ORIG / NEW |
| R | Togepi→Togekiss | Dancer | Happiny→Blissey | Healer | NEW / ORIG |
| S | Squirtle→Blastoise | Lost | Slowpoke→Slowbro/Slowking | Lost | ORIG |
| T | Scyther→Scizor/Kleavor | Lost | Weedle→Beedrill | Lost | ORIG / NEW |
| U | Ralts→Gardevoir/Gallade | Statue | Cleffa→Clefable | Fencer | ORIG / NEW |
| V | Noibat→Noivern | Lost | Goomy→Goodra | Statue | ORIG |
| W | Bagon→Salamence | Statue | Dratini→Dragonite | Lost | NEW / NEW |
| X | Rotom (5 forms) | Lost | Pichu→Pikachu→Raichu | Thief | ORIG / NEW |
| Y | Honedge→Aegislash | Thief | Pawniard→Kingambit | Lost | ORIG |
| Z | Charmander→Charizard | Statue | Beldum→Metagross | Alloy | NEW / NEW |

**25 colors, 46 hero lines** (2 per color, except hidden Missingno at D)

### Heroes Removed from Original Sliceymon

| Color | Was | Replaced By | Generated File |
|-------|-----|-------------|----------------|
| B P1 | Vanillite→Vanilluxe | Snorunt→Glalie/Froslass | `snorunt.txt` |
| L P1 | Fomantis→Lurantis | Chikorita→Meganium | `chikorita.txt` |
| M P1 | Rockruff→Lycanroc | Turtwig→Torterra (moved from N) | (existing hero, color change) |
| N P1 | Applin→Flapple/Hydrapple | NidoranF→Nidoqueen | `nidoranf.txt` |
| R P1 | Darumaka→Darmanitan | Togepi→Togekiss | `togepi.txt` |
| Z P1 | Agumon→SkullGreymon | Charmander→Charizard | `charmander.txt` |
| G P2 | Varoom→Revavroom | Mudkip→Swampert | `mudkip.txt` |
| H P2 | Lillipup→Stoutland | Aron→Aggron | `aron.txt` |
| K P2 | Espurr→Meowstic | Torchic→Blaziken | `torchic.txt` |
| L P2 | Sunkern→Sunflora | Treecko→Sceptile | `treecko.txt` |
| M P2 | Roggenrola→Gigalith | Wailmer→Wailord | `wailmer.txt` |
| P P2 | Trubbish→Garbodor | Bulbasaur→Venusaur | `bulbasaur.txt` |
| Q P2 | Slugma→Magcargo | Cyndaquil→Typhlosion | `cyndaquil.txt` |
| T P2 | Burmy→Wormadam/Mothim | Weedle→Beedrill | `weedle.txt` |
| U P2 | Tinkatink→Tinkaton | Cleffa→Clefable | `cleffa.txt` |
| X P2 | Joltik→Galvantula | Pichu→Pikachu→Raichu | `pikachu.txt` |
| Z P2 | Tentomon→MKabuterimon | Beldum→Metagross | `beldum.txt` |
| W P1+P2 | Ditto | Bagon→Salamence + Dratini→Dragonite | `bagon.txt` + `dratini.txt` |

### Heroes Modified (not replaced)

| Color | Hero | Change |
|-------|------|--------|
| H P1 | Larvitar→Tyranitar | Redesign: remove poison, add Rock/Dark |
| M P1 | Turtwig→Torterra | Moved from color N to color M P1 |
| N P2 | NidoranM→Nidoking | Fills slot vacated by Turtwig move |

### New Color Slots

| Color | Heroes | Lines |
|-------|--------|-------|
| E (new) | Machop→Machamp (P1) + Riolu→Lucario (P2) | new lines |
| J (new) | Totodile→Feraligatr (P1) + Poliwag→Poliwrath/Politoed (P2) | new lines |

### Aron Special: Pickup Mechanic

Aron (H P2) inherits Lillipup's **Pickup** item system. The Lillipup modifier is a compound modifier containing hero data + item pool + captures. Aron must preserve the Pickup structural modifier while replacing Lillipup's hero data.

---

## Monsters

| Pokemon | Floors | Notes |
|---------|--------|-------|
| Probopass | F9-11, F17-19 | Renamed from Aggron (Aron is now a hero) |
| Alakazam | F9-11, F17-19 | ORIG — base game rename |
| Cubone | F9-11, F17-19 | ORIG |
| Diglett | F5-7, F9-11 | ORIG |
| Dreepy | F1-3 | ORIG |
| Gastly | F1-3, F9-11, F17-19 | ORIG — all floors |
| Gliscor | F1-3 | ORIG |
| Impidimp | F9-11, F17-19 | ORIG |
| Mareep | F1-3 | ORIG |
| Raticate | F1-3 | ORIG |
| Rattata | F1-3 | ORIG |
| Skorupi | F1-3 | ORIG |
| Wooper | F1-3 | ORIG |
| Zubat | F1-3 | ORIG — monster only, removed from captures |

Additional base-game monsters appear renamed (Grunt, Primeape, Scrafty, Orthworm, Carbink, Bewear, etc.) but are not Pokemon-specific designs.

### New Monsters (from monster_boss_designs.md)

| Pokemon | Floors | Notes |
|---------|--------|-------|
| Zubat→Golbat→Crobat | F1-3 / F9-11 / F17-19 | Evolution across floor tiers |
| Tentacool→Tentacruel | F1-3 / F9-11 | Water/Poison |
| Carvanha→Sharpedo | F1-3 / F9-11 | Water/Dark |
| Chinchou→Lanturn | F1-3 / F9-11 | Water/Electric |
| Wild Steelix | F9-11 | Steel/Ground miniboss |
| Elite Steelix | F17-19 | Upgraded Steelix |
| Absol | F17-19 | Dark disaster Pokemon |

---

## Bosses

### Existing (in original Sliceymon)

| Floor | Boss | Key Mechanic | Status |
|-------|------|-------------|--------|
| F4 | Quagsire | Intro boss + minions | ORIG |
| F8 | Exeggutor | Multi-body split | ORIG |
| F12 | Xerneas + Florges + Zoroark | Boss + lieutenant | ORIG |
| F12/16/20 | Necrozma | 3-phase story boss | ORIG |
| F16 | Zygarde | Regenerating cells | ORIG |
| F20 | Hoopa | Escalating summons | ORIG |

### Planned — Gen 3 Path (designed in monster_boss_designs.md)

| Floor | Boss | Key Mechanic |
|-------|------|-------------|
| F4 | Golem + Geodude/Graveler | Rock army |
| F8 | Alpha Steelix + Onix | Steel miniboss |
| F12 | Regi Trio (Regirock/Regice/Registeel) | 3-way elemental |
| F16 | Regigigas + Regi Guardians | Slow Start awakening |
| F20 | Deoxys (4 forms) | Form-shifting phases |

### Planned — Gen 4 Path (designed in monster_boss_designs.md)

| Floor | Boss | Key Mechanic |
|-------|------|-------------|
| F12 | Palkia + Bronzong + Spatial Rift | Space distortion |
| F16 | Dialga + Bronzong + Temporal Anomaly | Time distortion |
| F20 | Arceus (4-phase type shifting) | Multi-type final boss |

### Planned — Gen 5 Path (designed in monster_boss_designs.md)

| Floor | Boss | Key Mechanic |
|-------|------|-------------|
| F8 | Serperior + Emboar + Samurott | Unova starters |
| F12A | Swords of Justice (Cobalion/Terrakion/Virizion) | Fighting trio |
| F12B | Forces of Nature (Tornadus/Thundurus/Landorus) | Weather trio |
| F16 | Reshiram OR Zekrom + Echo hazard | Dragon duality |
| F20 | Kyurem (fusion phases) | Absorbs Reshiram/Zekrom |

### Planned — Alternative (designed in monster_boss_designs.md)

| Floor | Boss | Key Mechanic |
|-------|------|-------------|
| F12 | Legendary Birds (Articuno/Zapdos/Moltres) | Gen 3 alternative trio |

---

## Captures

### Existing (unchanged)

| Pokemon | Ball | Status |
|---------|------|--------|
| Delcatty | Moon Ball | ORIG |
| Snorlax | Dream Ball | ORIG |
| Wobbuffet | Timer Ball | ORIG |
| Lilligant | Luxury Ball | ORIG |

### Existing (kept, from original)

| Pokemon | Ball | Status |
|---------|------|--------|
| Groudon | Red Orb | ORIG legendary |
| Ho-Oh | Rainbow Wing | ORIG legendary |
| Kyogre | Blue Orb | ORIG legendary |
| Lugia | Silver Wing | ORIG legendary |
| Mewtwo | Master Ball | ORIG legendary |

### Upgraded (same ball, evolved Pokemon)

| Pokemon | Ball | Was | Status |
|---------|------|-----|--------|
| Whiscash | Dive Ball | Barboach | CHG |
| Butterfree | Nest Ball | Caterpie | CHG |
| Weavile | Fast Ball | Sneasel | CHG |
| Manectric | Quick Ball | Electrike | CHG |

### Replaced (new Pokemon in existing ball slot)

| Pokemon | Ball | Replaces | Status |
|---------|------|----------|--------|
| Skarmory | Level Ball | Rattata | NEW |
| Lapras | Friend Ball | Furret | NEW |
| Arcanine | Premier Ball | Alcremie | NEW |

### New Captures

| Pokemon | Ball | Status |
|---------|------|--------|
| Mew | Poke Ball | NEW |
| Jirachi | Great Ball | NEW |
| Kangaskhan | Safari Ball | NEW |
| Heracross | Net Ball | NEW |
| Greninja | Dusk Ball | NEW |
| Electivire | Ultra Ball | NEW |
| Magmortar | Lure Ball | NEW |
| Rhyperior | Heavy Ball | NEW |

### New Legendaries

| Pokemon | Item | Status |
|---------|------|--------|
| Latias | Soul Dew | NEW |
| Latios | Eon Flute | NEW |
| Suicune | Clear Bell | NEW |
| Entei | Flame Plate | NEW |
| Raikou | Zap Plate | NEW |
| Rayquaza | Jade Orb | NEW |

### Removed from Captures

| Pokemon | Reason |
|---------|--------|
| Pikachu | Now hero evolution (Pichu→Pikachu→Raichu, X P2) |
| Charizard | Now hero evolution (Charmander→Charizard, Z P1) |
| Metagross | Now hero evolution (Beldum→Metagross, Z P2) |
| Poliwag | Now hero (J P2) |
| Ivysaur | Now hero evolution (Bulbasaur→Venusaur, P P2) |
| Arceus | Now boss (F20 Gen 4 path) |
| Zubat | Now monster only |
| Barboach | Upgraded to Whiscash |
| Caterpie | Upgraded to Butterfree |
| Sneasel | Upgraded to Weavile |
| Electrike | Upgraded to Manectric |
| Rattata | Replaced by Skarmory |
| Furret | Replaced by Lapras |
| Alcremie | Replaced by Arcanine |

---

## Quick Counts

| Category | Count |
|----------|-------|
| Heroes | 46 lines across 25 colors |
| Monsters | 14 original + 7 new designs |
| Bosses | 6 existing + 16 planned across 3 gen paths |
| Captures | 4 unchanged + 4 upgraded + 3 replaced + 8 new = 19 |
| Legendaries | 5 existing + 6 new = 11 |
| **Total unique Pokemon** | ~120+ |

---

## Design Document Cross-References

| Content | Design Source |
|---------|-------------|
| Hero dice stats (batch 1) | `plans/hero_designs_batch1.md` — Charmander, Cyndaquil, Torchic, Bagon, Dratini, Beldum, Machop |
| Hero dice stats (batch 2) | `plans/hero_designs_batch2.md` — Treecko, Chikorita, Bulbasaur, Mudkip, Totodile, Poliwag, Wailmer |
| Hero dice stats (batch 3) | `plans/hero_designs_batch3.md` — Pikachu, Weedle, Riolu, Togepi, Cleffa, Nidorans, Tyranitar |
| Snorunt/Aron dice stats | `plans/OVERHAUL_PLAN.md` Section 1A |
| Monster dice designs | `plans/monster_boss_designs.md` Part 1 |
| Boss fight designs (Gen 3) | `plans/monster_boss_designs.md` Part 2 |
| Boss fight designs (Gen 4) | `plans/monster_boss_designs.md` Part 3 |
| Boss fight designs (Gen 5) | `plans/monster_boss_designs.md` Part 4 |
| Legendary Birds | `plans/monster_boss_designs.md` Part 7 |
| Legendary Dogs captures | `plans/monster_boss_designs.md` Part 8 |
| Capture dice designs | `plans/OVERHAUL_PLAN.md` Section 3C-1 |
| Legendary dice designs | `plans/OVERHAUL_PLAN.md` Section 3D-1 |
| Spell reference | `plans/OVERHAUL_PLAN.md` DEFINITIVE SPELL REFERENCE |
| Template properties | `plans/TEMPLATE_PROPERTIES.md` |
| Generated hero files | `generated/*.txt` (26 files) |

---

## Known Discrepancies (to resolve during implementation)

1. **Capture replacement ball mismatch**: OVERHAUL_PLAN Section 3C says Skarmory=Level Ball (replaces Rattata), Lapras=Friend Ball (replaces Furret). But Section 3C-1 dice table says Skarmory=Luxury (replaces Lilligant), Lapras=Moon (replaces Delcatty). **Resolution**: Use 3C assignments (Skarmory=Level, Lapras=Friend, Arcanine=Premier) as authoritative — these are the explicit action items. The 3C-1 table's ball/replacement names appear to be copy errors.
2. **Whiscash dice stats**: OVERHAUL_PLAN notes stats "not yet specified" in 3B but then provides them in 3C-1. Use the 3C-1 table.
