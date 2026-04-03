# OVERHAUL PLAN: Sliceymon+ Template Migration & Completion

## Executive Summary

This plan covers the migration of 13 heroes to new templates (11 to never-before-used templates: Eccentric, Stalwart, Dancer, Fencer, Alloy, Fighter, Guardian; plus 2 proven-template corrections: Treecko to Primrose, Chikorita to Healer), plus verification and cleanup of the remaining 12 heroes already on correct proven templates (Lost, Statue, Thief), the Aron/Lillipup swap, monster changes (Aggron to Probopass), and capture changes.

**Out of scope**: Boss design/implementation, new monster designs beyond Probopass rename, gameplay balancing beyond what is specified in this plan, UI/aesthetic changes beyond sprite swaps.

**Current state**: 22 hero lines are generated in `/generated/` using `replica.Lost` and `replica.Statue`. The FULL_ROSTER.md mandates 11 of these heroes use new templates from the base game that exist in `pansaer.txt` but have never been used in Sliceymon.

**Core risk**: The new templates (Eccentric, Stalwart, Dancer, Fencer, Alloy, Fighter, Guardian) have unknown default properties. Since we override `.hp.`, `.sd.`, `.col.`, `.img.`, `.speech.`, `.n.` on every tier, the template choice mainly affects: (a) the base die frame/visual, (b) any inherited abilitydata infrastructure, and (c) default keywords/items. We must test each new template in isolation before mass-deploying.

---

## PART 0: Template Research (MUST DO FIRST)

### Why This Matters

The pansaer.txt working mod uses all 7 new templates. Each template has base properties that we inherit unless we override them. For `replica.Lost` and `replica.Statue`, we know exactly what they provide:
- Lost: Orange color, HP 3, basic damage die
- Statue: Grey color, HP 20, all-blank faces

For the new templates, we need to extract from `pansaer.txt`:
- Default HP (we override, but need to know)
- Default color (we override)
- Default sd faces (we override)
- Any inherited abilitydata (CRITICAL -- if the template has a built-in spell, our heroes will inherit it unless we explicitly clear it)
- Any inherited keywords, facades, triggerhpdata, doc strings

### Action Items

For each of these 7 templates, extract from `/Users/hgorelick/Documents/slice-and-dice/working-mods/pansaer.txt`:

| Template | Target Heroes | What to Extract |
|----------|--------------|-----------------|
| `replica.Eccentric` | Snorunt (B P1) | Default HP, sd, keywords, abilitydata, doc |
| `replica.Stalwart` | Aron (H P2), Wailmer (M P2) | Default HP, sd, keywords, abilitydata, doc |
| `replica.Dancer` | Togepi (R P1) | Default HP, sd, keywords, abilitydata, doc |
| `replica.Fencer` | Cleffa (U P2) | Default HP, sd, keywords, abilitydata, doc |
| `replica.Alloy` | Beldum (Z P2) | Default HP, sd, keywords, abilitydata, doc |
| `replica.Fighter` | Machop (F P1), Totodile (J P1), Poliwag (J P2) | Default HP, sd, keywords, abilitydata, doc |
| `replica.Guardian` | Mudkip (G P2), Bulbasaur (P P2) | Default HP, sd, keywords, abilitydata, doc |

**Method**: Parse pansaer.txt to find each `replica.TEMPLATE` usage, extract the properties set on each tier, and identify which properties come from template defaults vs explicit overrides.

**Deliverable**: A template properties reference document listing exactly what each template provides by default and what we must override.

---

## PART 1: Hero Overhaul -- Complete Specifications

### Removed Heroes (implicit in line replacements)

The following existing heroes are being CUT from the mod, replaced by new heroes on their lines:

| Removed Hero | Was On | Replaced By | New Template |
|-------------|--------|-------------|--------------|
| Vanillite/Vanilluxe | Line 15, B P1 | Snorunt/Glalie/Froslass | Eccentric |
| Varoom/Revavroom | Line 61, G P2 | Mudkip/Swampert | Guardian |
| Lillipup/Stoutland | Line 63, H P2 | Aron/Aggron | Stalwart |
| Roggenrola/Gigalith | Line 73, M P2 | Wailmer/Wailord | Stalwart |
| Trubbish/Garbodor | Line 79, P P2 | Bulbasaur/Venusaur | Guardian |
| Tinkatink/Tinkaton | Line 89, U P2 | Cleffa/Clefable | Fencer |
| Darumaka/Darmanitan | Line 39, R P1 | Togepi/Togekiss | Dancer |
| Tentomon/MKabuterimon | Line 97, Z P2 | Beldum/Metagross | Alloy |
| Rockruff/Lycanroc | Line 29, M P1 | Turtwig/Torterra (moved from N) | Lost |
| Slugma/Magcargo | Line 81, Q P2 | Cyndaquil/Typhlosion | Lost |
| Fomantis/Lurantis | Line 27, L P1 | Chikorita/Meganium | Healer |
| Applin/Flapple/etc. | Line 31, N P1 | NidoranF/Nidoqueen | Statue |

**CRITICAL -- Ditto Impact**: Ditto copy line (Line 99, 66K chars) contains copies of every hero T3 forms. ALL 12 removed heroes T3 copies must be removed from Line 99, and ALL new heroes T3 copies must be added. This must be addressed during Chunk 7/8 assembly or as a separate step.

### Section 1A: Heroes on NEW Templates (11 heroes -- highest risk)

These heroes need their template changed from the current Lost/Statue to the new template, then re-tested. The dice stats, HP, keywords, speech, and sprites remain the same -- only the template wrapper changes.

---

#### 1. Snorunt -> Glalie/Froslass (Color B, P1, Line 15, replaces Vanillite)

**Template**: `replica.Eccentric` (was: `replica.Statue` on Vanillite)
**Justification**: Eccentric = unpredictable/quirky base. Snorunt's branching evolution (Glalie vs Froslass) is the definition of eccentric -- one path goes brutal physical, the other goes ghostly special. Vanillite also used a unique mana-burst pattern that Eccentric's base may support.
**Color**: B
**Line**: 15

**IMPORTANT**: Vanillite has 5 abilitydata spells (Icy Wind, Ice Beam, Hail, Sheer Cold, Blizzard). Snorunt's design must account for this spell infrastructure. If `replica.Eccentric` has built-in abilitydata, we may need to clear or override it.

**T1 Snorunt: HP 4**
- sd: `93-2:93-2:56-1:56-1:0:0`
- Faces: Mana SU 2, Mana SU 2, Shield 1, Shield 1, Blank, Blank
- Keywords: none
- Speech: `Sno!:Runt!:[i]shiver`
- Doc: none
- Balance: 2 blanks, 4 active. Mana SU faces mirror Vanillite's identity but at lower pips. Shields represent ice armor. HP 4 matches Vanillite T1.

**T2a Snorunt: HP 7**
- sd: `93-3:56-2:56-2:76-1:131-1:0`
- Faces: Mana SU 3, Shield 2, Shield 2, Mana 1, Damage Weaken 1, Blank
- Keywords: none
- Speech: `Sno!:Runt!:[i]chill`
- Balance: 1 blank, 5 active. Growing into ice caster. Weaken represents Icy Wind debuff.

**T2b Snorunt: HP 6**
- sd: `93-3:93-2:76-2:76-2:131-1:0`
- Faces: Mana SU 3, Mana SU 2, Mana 2, Mana 2, Damage Weaken 1, Blank
- Keywords: none
- Speech: `Sno!:Runt!:[i]chill`
- Balance: 1 blank, 5 active. Full caster variant. Heavy mana generation for spells.

**T3a Glalie: HP 9**
- sd: `34-3:34-3:131-2:131-2:93-3:0`
- Faces: Damage to All 3, Damage to All 3, Damage Weaken 2, Damage Weaken 2, Mana SU 3, Blank
- Keywords: none
- Speech: `Glalie!:[i]SHEER COLD`
- Spell -- **Sheer Cold**: `abilitydata.(Fey.sd.34-5:0-0:0-0:0-0:76-5.i.Ritemx.132fb.part.1.i.left.k.weaken.img.light.hsv.-120:0:40.n.Sheer Cold)`
  - Spell faces: Damage to All 5, Blank, Blank, Blank, Mana 5 (cost)
  - Fey template: face 34 is valid for Fey (in approved list: 15,32,34,36,37,44,56,72,76,95,103,105,107,108,109,111,116,117,119,131,136,150,162,177,180,181,182,183). Face 76 valid.
  - Singlecast + Weaken. Per SPELL_REDESIGN.md: 5 AoE dmg / 5 mana is Glalie's fight-ending nuke.
- Balance: 1 blank, HP 9. AoE + Weaken = offensive ice bruiser. Comparable to Charizard T3b in AoE pattern.

**T3b Froslass: HP 8**
- sd: `131-3:131-3:34-2:34-2:76-3:76-3`
- Faces: Damage Weaken 3, Damage Weaken 3, Damage to All 2, Damage to All 2, Mana 3, Mana 3
- Keywords: none
- Speech: `Froslass!:[i]ghostly chill`
- Spell -- **Ice Beam**: `abilitydata.(Fey.sd.131-3:0-0:0-0:0-0:76-3.i.Ritemx.132fb.part.1.i.left.k.inflictsingleuse.img.light.hsv.-120:0:40.n.Ice Beam)`
  - Face 131 valid for Fey. Face 76 valid. Singlecast + inflictsingleuse per SPELL_REDESIGN.md. Weaken 3 (not 4) matches balance review.
- Balance: 0 blanks, HP 8. All faces active. Ghost/Ice caster variant. Weaken-heavy = debuff-focused support DPS. Lower HP, more mana generation. Distinct from Glalie: Glalie = AoE burst, Froslass = debuff caster.

---

#### 2. Mudkip -> Swampert (Color G, P2, Line 61, replaces Varoom)

**Template**: `replica.Guardian` (was: `replica.Statue` in generated file)
**Justification**: Guardian = party protector base. Swampert's entire identity is Redirect SelfShield + Shield Rescue. Guardian's defaults likely include shield/redirect infrastructure, which is perfect.
**Color**: G
**Line**: 61

**Dice stats**: KEEP EXACTLY as designed in batch 2 and generated in `line_61_mudkip.txt`. Only change `replica.Statue` to `replica.Guardian` in the template reference.

**T1 Mudkip: HP 6**
- sd: `64-1:39-1:0:0:56-1:0`
- Faces: Shield Rescue 1, Damage Heavy 1, Blank, Blank, Shield 1, Blank
- Keywords: none
- Speech: `Mud!:Kip!:[i]splash`

**T2a Marshtomp: HP 8**
- sd: `64-2:39-2:118-2:118-2:0:56-2`
- Speech: `Marsh!:[i]stomp`

**T2b Marshtomp: HP 7**
- sd: `64-2:39-2:39-1:39-1:0:118-2`
- Speech: `Marsh!:[i]stomp`

**T3a Swampert: HP 11**
- sd: `64-3:39-3:118-3:118-3:56-3:39-2`
- No spell.
- Speech: `Swam!:Pert!:[i]EARTHQUAKE`

**T3b Swampert: HP 10**
- sd: `64-3:39-4:118-3:118-3:39-3:52-2`
- No spell.
- Speech: `Swam!:Pert!:[i]EARTHQUAKE`

---

#### 3. Bulbasaur -> Venusaur (Color P, P2, Line 79, replaces Trubbish)

**Template**: `replica.Guardian` (was: `replica.Statue` in generated file)
**Justification**: Guardian = defensive anchor. Venusaur is a sustain tank with Poison Plague + Regen + Repel. Guardian's protective base fits.
**Color**: P
**Line**: 79

**Dice stats**: KEEP EXACTLY as generated in `line_79_bulbasaur.txt`. Change template to `replica.Guardian`.

**T1 Bulbasaur: HP 5**
- sd: `53-1:110-1:0:0:145:0`
- Doc: `Start Poisoned`
- Speech: `Bulba!:[i]saur`

**T2a Ivysaur: HP 8**
- sd: `55-1:110-2:119-2:119-2:0:53-2`
- Speech: `Ivy!:[i]saur`

**T2b Ivysaur: HP 7**
- sd: `55-1:110-3:53-2:53-2:119-1:145`
- Speech: `Ivy!:[i]saur`

**T3a Venusaur: HP 11**
- sd: `55-2:110-4:119-3:119-3:110-2:53-3`
- Spell -- **Solar Beam**: `abilitydata.(Statue.sd.42-5:15-3:0:0:76-3.i.left.k.focus.img.beam.hsv.40:0:0.n.Solar Beam)`
  - Statue template: face 15 valid (in approved list: 15,32,34,76,103,111,177). Face 76 valid. Face 42 is NOT in the Statue approved list.
  - **FIXED**: Replaced `42-5` with `15-4`: `abilitydata.(Statue.sd.15-4:0:0:0:76-4.i.left.k.vulnerable.img.beam.hsv.40:0:0.n.Solar Beam)`. Face 15 valid for Statue. Single keyword (vulnerable) per DEFINITIVE SPELL REFERENCE -- removed redundant `.i.left.k.focus`.
- Speech: `Venusaur!:[i]SOLAR BEAM`

**T3b Venusaur: HP 10**
- sd: `55-3:110-3:53-3:53-3:119-2:110-3`
- Same spell as T3a.
- Speech: `Venusaur!:[i]SOLAR BEAM`

---

#### 4. Aron -> Aggron (Color H, P2, Line 63 compound, replaces Lillipup)

**Template**: `replica.Stalwart` (NEW -- never used in Sliceymon)
**Justification**: Stalwart = heavy armored fighter. Aron/Aggron is the quintessential Steel/Rock tank -- heavy armor, massive defense, crushing blows. Stalwart captures the \"immovable object\" fantasy.
**Color**: H
**Line**: 63 (the compound line currently containing Lillipup + items + Arceus capture)

**CRITICAL**: This line is the most complex in the mod (10,412 chars). The Lillipup hero data must be surgically replaced with Aron data while preserving all items, Caterpie/Butterfree capture, and any other non-hero content on this line. The Arceus capture is being removed per the expansion plan (Arceus becomes a boss).

**T1 Aron: HP 6**
- sd: `63-1:63-1:39-1:0:0:0`
- Faces: Steel Shield 1, Steel Shield 1, Damage Heavy 1, Blank, Blank, Blank
- Keywords: none
- Speech: `Ar!:Ron!:[i]clank`
- Doc: none
- Balance: 3 blanks, 3 active. HP 6 is appropriate for a baby Steel tank (Mudkip is also HP 6 at T1 as a tank). Two Steel Shields establish the armored identity immediately.

**T2a Lairon: HP 9**
- sd: `63-3:63-2:39-2:39-2:119-1:0`
- Faces: Steel Shield 3, Steel Shield 2, Damage Heavy 2, Damage Heavy 2, Shield Repel 1, Blank
- Keywords: none
- Speech: `Lair!:Ron!:[i]rumble`
- Balance: 1 blank, 5 active. HP 9 (top T2 range -- justified as a tank). Heavy Steel Shields + emerging Heavy damage + Repel.

**T2b Lairon: HP 8**
- sd: `63-2:63-2:39-3:39-2:0:0`
- Faces: Steel Shield 2, Steel Shield 2, Damage Heavy 3, Damage Heavy 2, Blank, Blank
- Keywords: none
- Speech: `Lair!:Ron!:[i]rumble`
- Balance: 2 blanks, 4 active. Lower HP, more offense. Bruiser Lairon variant.

**T3a Aggron: HP 12**
- sd: `63-4:63-3:39-4:39-3:119-2:118-2`
- Faces: Steel Shield 4, Steel Shield 3, Damage Heavy 4, Damage Heavy 3, Shield Repel 2, Redirect SelfShield 2
- Keywords: none
- Speech: `AGGRON!:[i]IRON TAIL`
- Spell -- **Iron Tail**: `abilitydata.(Statue.sd.15-4:0:0:0:76-3.i.left.k.focus.n.Iron Tail.img.mithril shields.hsv.0:0:0)` (per DEFINITIVE SPELL REFERENCE)
  - Face 15 valid for Statue. Face 76 valid for Statue. Damage 4 / 3 mana + focus keyword. Focus represents Aggron's precision steel strike.
- Balance: 0 blanks, HP 12. Full armored fortress. 7 Steel Shield pips (7/1.4 = 5 damage equivalent) + 7 Heavy damage pips + Repel 2 + Redirect 2. Compares to Metagross T3a (HP 11, 7 Heavy + 6 Steel Shield + Redirect 2) -- similar but Aggron has Repel instead of Mana Pain, making it more purely defensive.

**T3b Aggron: HP 10**
- sd: `63-3:63-3:39-4:39-4:39-3:119-3`
- Faces: Steel Shield 3, Steel Shield 3, Damage Heavy 4, Damage Heavy 4, Damage Heavy 3, Shield Repel 3
- Keywords: none
- Speech: `AGGRON!:[i]IRON TAIL`
- Same spell as T3a.
- Balance: 0 blanks, HP 10. Offensive Aggron. More Heavy damage (3 faces at 4/4/3 = 11 pips), less shielding (6 Steel pips vs 7), strong Repel. The \"crusher\" variant.

---

#### 5. Wailmer -> Wailord (Color M, P2, Line 73, replaces Roggenrola)

**Template**: `replica.Stalwart` (was: `replica.Lost` in generated file)
**Justification**: Stalwart = immovable heavy. Wailord is the biggest Pokemon, the ultimate HP sponge. Stalwart's heavy-armor base fits the \"wall of blubber\" fantasy.
**Color**: M
**Line**: 73

**Dice stats**: KEEP EXACTLY as designed in batch 2 and generated in `line_73_wailmer.txt`. Change template to `replica.Stalwart`.

**T1 Wailmer: HP 6**
- sd: `72-1:56-1:0:0:0:0`
- 4 blanks (EXCEPTION: exceeds T1 guideline of 2-3 blanks. Justified by Magikarp-style delayed payoff pattern -- worst T1 face ratio compensated by Shield to All keyword and high HP. See batch 2 design doc for full rationale). Speech: `Wailmer!:[i]splash`

**T2a Wailmer: HP 9**
- sd: `72-2:56-2:39-2:39-2:0:0`
- 2 blanks. Speech: `Wailmer!:[i]bigger splash`

**T2b Wailmer: HP 8**
- sd: `72-2:56-3:39-2:39-2:56-1:0`
- 1 blank. Speech: `Wailmer!:[i]bigger splash`

**T3a Wailord: HP 14**
- sd: `72-3:39-3:56-4:56-4:39-2:72-2`
- 0 blanks. No spell. Speech: `WAILORD!:[i]WATER SPOUT`
- HP EXCEPTION: 14 exceeds T3 guideline of 8-13. Justified by the worst T1/T2 curve in the game (4 blanks at T1, 2 blanks at T2a). Same design logic as Magikarp->Gyarados (HP 5 with 5 stasis blanks -> HP 13). The extreme delayed payoff earns the extreme HP.

**T3b Wailord: HP 13**
- sd: `72-3:39-4:56-3:56-3:52-2:39-3`
- 0 blanks. No spell. Speech: `WAILORD!:[i]WATER SPOUT`

---

#### 6. Togepi -> Togekiss (Color R, P1, Line 39, replaces Darumaka)

**Template**: `replica.Dancer` (was: `replica.Statue` in generated file)
**Justification**: Dancer = graceful support/healer. Togekiss's Serene Grace ability and aerial dance fits the Dancer template perfectly. Dancer likely provides heal/support infrastructure that aligns with Togekiss's rescue healer role.
**Color**: R
**Line**: 39

**Dice stats**: KEEP EXACTLY as generated in `line_39_togepi.txt`. Change template to `replica.Dancer`.

**T1 Togepi: HP 5**
- sd: `106-1:103-1:61-1:0:0:0`
- Speech: `Toge!:Pi!:[i]chirp`

**T2a Togetic: HP 7**
- sd: `106-2:109-1:109-1:61-2:103-1:0`
- Speech: `Toge!:Tic!:[i]flutter`

**T2b Togetic: HP 6**
- sd: `106-2:106-2:61-1:61-1:109-1:0`
- Speech: `Toge!:Tic!:[i]flutter`

**T3a Togekiss: HP 9**
- sd: `106-3:106-3:109-3:109-3:61-2:0`
- Spell -- **Wish**: `abilitydata.(Fey.sd.107-3:0-0:0-0:0-0:76-3.img.light.n.Wish)` (per DEFINITIVE SPELL REFERENCE)
  - Face 107 valid for Fey. Face 76 valid. Heal All 3 / 3 mana -- clean, fair team heal. Reduced from 4/4 per balance review.
- Speech: `Kiss!:[i]DAZZLING GLEAM`

**T3b Togekiss: HP 8**
- sd: `106-4:109-4:109-4:61-3:61-3:123`
- Spell -- **Dazzling Gleam**: `abilitydata.(Fey.sd.34-3:0-0:0-0:0-0:76-4.i.left.k.vulnerable.img.light.n.Dazzling Gleam)` (per DEFINITIVE SPELL REFERENCE)
  - Face 34 valid for Fey. Face 76 valid. AoE 3 / 4 mana + vulnerable. Dazzling = blinding = vulnerable debuff.
- Speech: `Kiss!:[i]DAZZLING GLEAM`

---

#### 7. Cleffa -> Clefable (Color U, P2, Line 89, replaces Tinkatink)

**Template**: `replica.Fencer` (was: `replica.Lost` in generated file)
**Justification**: Fencer = parry/riposte style, which maps to Clefable's Metronome/Copycat mechanic -- reacting to what others do. The \"copy the opponent's move\" aspect of Copycat (face 27) is thematically a fencer's riposte.
**Color**: U
**Line**: 89

**Dice stats**: KEEP EXACTLY as generated in `line_89_cleffa.txt`. Change template to `replica.Fencer`.

**T1 Cleffa: HP 4**
- sd: `27-1:110-1:56-1:0:0:0`
- Speech: `Cleff!:Fa!`

**T2a Clefairy: HP 7**
- sd: `27-2:27-1:110-2:56-2:56-1:0`
- Speech: `Fairy!:Clefairy!:Clef clef!`

**T2b Clefairy: HP 8**
- sd: `27-1:110-2:110-1:56-2:56-2:0`
- Speech: `Fairy!:Clefairy!:Clef clef!`

**T3a Clefable: HP 9**
- sd: `27-3:27-3:110-3:110-3:56-2:0`
- Spell -- **Metronome**: `abilitydata.(Fey.sd.136-4:0-0:0-0:0-0:76-4.img.light.n.Metronome)`
  - **FIXED**: Face 27 (Copycat) is NOT in the Fey approved list. Replaced with face 136 (Revive) which IS valid for Fey: `abilitydata.(Fey.sd.136-2:0-0:0-0:0-0:76-4.i.Ritemx.132fb.part.1.img.light.n.Metronome)`. Revive 2 (not 4) per SPELL_REDESIGN.md balance review -- Revive is Premium tier, 4 mana + singlecast keeps it expensive. Rejuvenate precedent is Revive 1/3 mana singlecast; Metronome at Revive 2/4 mana is a meaningful upgrade without being game-breaking.
- Speech: `Clefable!:[i]Moonlight:[i]wiggles fingers`

**T3b Clefable: HP 10**
- sd: `27-2:27-2:110-4:110-4:56-3:56-3`
- Spell -- **Moonlight**: `abilitydata.(Fey.sd.107-3:0-0:0-0:0-0:76-3.i.left.k.cleanse.img.light.n.Moonlight)` (per DEFINITIVE SPELL REFERENCE)
  - Face 107 valid for Fey. Face 76 valid. Heal All 3 / 3 mana + cleanse keyword. Purifying moonlight removes debuffs.
- Speech: `Clefable!:[i]Moonlight:[i]wiggles fingers`

---

#### 8. Beldum -> Metagross (Color Z, P2, Line 97, replaces Tentomon)

**Template**: `replica.Alloy` (was: `replica.Lost` in generated file)
**Justification**: Alloy = steel/metal base. Metagross is literally a Steel/Psychic supercomputer made of metal alloys. Perfect thematic fit.
**Color**: Z
**Line**: 97

**Dice stats**: KEEP EXACTLY as generated in `line_97_beldum.txt`. Change template to `replica.Alloy`.

**T1 Beldum: HP 5**
- sd: `39-1:63-1:0:0:63-1:0`
- Speech: `Bel...:Dum`

**T2a Metang: HP 8**
- sd: `39-2:63-3:118-1:118-1:63-2:0`
- Speech: `Metang!:Tang!`

**T2b Metang: HP 7**
- sd: `39-2:39-2:63-2:63-2:118-1:0`
- Speech: `Metang!:Tang!`

**T3a Metagross: HP 11**
- sd: `39-4:39-3:63-3:63-3:118-2:82-1`
- Spell -- **Meteor Mash**: `abilitydata.(lost.sd.39-5:39-4:0:0:76-3.n.Meteor Mash.img.mithril shields.hsv.-120:30:70)`
  - **VALIDATION**: Face 39 is NOT in the `lost` approved list (lost: 8,15,36,69,74,76,103,108,177,185). Face 76 IS valid.
  - **FIXED**: `abilitydata.(lost.sd.15-5:0:0:0:76-4.i.left.k.focus.n.Meteor Mash.img.mithril shields.hsv.-120:30:70)`. Face 15 (Damage) valid for lost. Face 76 valid. Damage 5 + focus keyword for 4 mana cost. Per SPELL_REDESIGN.md: Focus represents Metagross's calculated precision (same pattern as Psycho Cut).
- Speech: `METAGROSS!:Computed.:[i]METEOR MASH`

**T3b Metagross: HP 10**
- sd: `39-3:39-3:63-4:63-4:118-3:0`
- Same corrected spell.
- Speech: `METAGROSS!:Computed.:[i]METEOR MASH`

---

#### 9. Machop -> Machamp (Color F, P1, NEW Line)

**Template**: `replica.Fighter` (was: `replica.Lost` in generated file)
**Justification**: Fighter = combat-focused base. The base game already has 2 heroes on Fighter template (per audit). Machamp IS a fighter -- four arms, martial arts, No Guard. Perfect match.
**Color**: F
**Line**: NEW (appended after existing hero lines)

**Dice stats**: KEEP EXACTLY as generated in `line_new_machop.txt`. Change template to `replica.Fighter`.

**WARNING**: A stale file `generated/line_87_machop.txt` also exists in the generated directory (Machop was originally planned for line 87 but moved to a NEW line). Use `line_new_machop.txt` only. Delete or ignore `line_87_machop.txt` during implementation.

**T1 Machop: HP 5**
- sd: `17-1:39-1:51-1:0:0:0`
- Speech: `Chop!:[i]flex`

**T2a Machoke: HP 8**
- sd: `17-2:39-2:51-2:51-2:15-1:0`
- Speech: `Choke!:[i]punch`

**T2b Machoke: HP 7**
- sd: `17-2:17-2:39-1:39-1:114-1:0`
- Speech: `Choke!:[i]punch`

**T3a Machamp: HP 10**
- sd: `17-3:39-3:51-3:51-3:17-2:0`
- No spell. Speech: `Champ!:[i]DYNAMIC PUNCH`

**T3b Machamp: HP 9**
- sd: `17-3:39-3:17-2:17-2:25-2:0`
- No spell. Speech: `Champ!:[i]DYNAMIC PUNCH`

---

#### 10. Totodile -> Feraligatr (Color J, P1, NEW Line)

**Template**: `replica.Fighter` (was: `replica.Lost` in generated file)
**Justification**: Fighter for the water berserker. Feraligatr is a physical fighting croc with Engage + Cruel + Heavy. Fighter template base suits its aggressive melee identity.
**Color**: J
**Line**: NEW

**Dice stats**: KEEP EXACTLY as generated in `line_new_totodile.txt`. Change template to `replica.Fighter`.

**T1 Totodile: HP 5**
- sd: `17-1:30-1:0:0:39-1:0`
- Speech: `Toto!:[i]chomp`

**T2a Croconaw: HP 7**
- sd: `17-2:30-2:39-2:39-2:0:17-2`
- Speech: `Croc!:[i]BITE`

**T2b Croconaw: HP 6**
- sd: `17-2:30-2:30-1:30-1:0:39-2`
- Speech: `Croc!:[i]BITE`

**T3a Feraligatr: HP 9**
- sd: `17-3:30-3:39-3:39-3:17-2:30-2`
- No spell. Speech: `Feral!:[i]CRUNCH`

**T3b Feraligatr: HP 10**
- sd: `17-4:30-3:39-4:39-4:30-2:51-2`
- No spell. Speech: `Feral!:[i]CRUNCH`

---

#### 11. Poliwag -> Poliwrath/Politoed (Color J, P2, NEW Line)

**Template**: `replica.Fighter` (was: `replica.Statue` in generated file)
**Justification**: Fighter for the branching water line. Poliwrath is Fighting-type; even Politoed benefits from Fighter's balanced base since it provides a framework that works for both evolution paths.
**Color**: J
**Line**: NEW (same new color j line as Totodile, P2 slot)

**Dice stats**: KEEP EXACTLY as generated in `line_new_poliwag.txt`. Change template to `replica.Fighter`.

**T1 Poliwag: HP 4**
- sd: `56-1:39-1:0:0:103-1:0`
- Speech: `Poli!:[i]wiggle`

**T2a Poliwhirl: HP 7**
- sd: `56-2:39-2:17-2:17-2:0:103-2`
- Speech: `Whirl!:[i]spin`

**T2b Poliwhirl: HP 8**
- sd: `61-2:103-2:107-1:107-1:0:56-2`
- Speech: `Whirl!:[i]spin`

**T3a Poliwrath: HP 10**
- sd: `17-3:39-3:51-3:51-3:39-2:17-2`
- No spell. Speech: `Wrath!:[i]DYNAMIC PUNCH`

**T3b Politoed: HP 9**
- sd: `107-3:61-3:105-3:105-3:107-2:112-2`
- Spell -- **Rain Dance**: `abilitydata.(Fey.sd.107-2:0-0:0-0:0-0:76-3.i.Ritemx.62e8.i.left.k.cleanse.img.flick.hsv.-50:-30:0.n.Rain Dance)` (per DEFINITIVE SPELL REFERENCE)
  - Face 107 valid for Fey. Face 76 valid. Heal All 2 / 3 mana + Ritemx.62e8 (sustain) + cleanse. Matches original Goodra Rain Dance from base Sliceymon. Redesigned from 2 active faces to 1 active face + keywords for cleaner design.
- Speech: `Toed!:[i]RAIN DANCE`

---

### Section 1B: Heroes on PROVEN Templates (14 heroes -- lower risk)

These heroes use templates already working in Sliceymon. The generated files should work with minimal changes. Listed for completeness with any validation issues flagged.

---

#### 12. Torchic -> Blaziken (Color K, P2, Line 69, Thief template)

**Template**: `replica.Thief` (CHANGE from Lost in generated file)
**Justification per FULL_ROSTER**: Thief template. Blaziken's speed assassin role (Defy + Critical + Engage) maps to Thief's ranged damage base. The EXPANSION_PLAN originally specified Lost but FULL_ROSTER.md specifies Thief.
**Color**: K, Line 69

**Template discrepancy resolved**: FULL_ROSTER.md says Thief; batch 1 design doc and generated file say Lost. FULL_ROSTER is the authoritative document. **DECISION**: Use `replica.Thief`. Generated file needs template update.

**Spell validation for Thief template**:
- Original: `abilitydata.(lost.sd.174-3:175-2:0:0:76-2.n.Blaze Kick.img.scorch.hue.10)` -- faces 174 (Defy) and 175 (Critical) are NOT valid for lost (8,15,36,69,74,76,103,108,177,185) or Thief (76,136) spell templates.
- **FIXED**: Use Fey spell template. `abilitydata.(Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.inflictpain.img.scorch.hue.10.n.Blaze Kick)`. Face 15 valid for Fey, face 76 valid. Single active face follows Fey convention. Damage 4 for 3 mana cost + inflictpain debuff.

**T1-T3 stats**: Same as batch 1 designs and `line_69_torchic.txt`. Template change only.

---

#### 13. Treecko -> Sceptile (Color L, P2, Line 71, Primrose template)

**Template**: `Primrose` (CHANGE from Lost in generated file)
**Justification per FULL_ROSTER**: Primrose template. Sceptile's Growth mechanic (Overgrow) maps to Primrose's scaling base.

**Template discrepancy resolved**: FULL_ROSTER.md says Primrose; batch 2 design doc says Primrose. Generated file `line_71_treecko.txt` currently uses `replica.Lost`. **DECISION**: Use `Primrose`. Generated file needs template update.

**Stats**: Same as batch 2 and `line_71_treecko.txt`. Template and spell fixes needed.

---

#### 14. Pichu -> Raichu (Color X, P2, Line 93, Thief template)

**Template**: `replica.Thief` (FULL_ROSTER specifies Thief)
**Currently generated as**: `replica.Lost` in `line_93_pikachu.txt`
**Color**: X, Line 93

**Spell validation for Thief template** (spell template independent):
- T3a Thunderbolt: `abilitydata.(Fey.sd.42-4:0-0:0-0:0-0:76-4.img.IMG.n.Thunderbolt)`
  - **FIXED**: Face 42 (Charged) NOT in Fey list. Per DEFINITIVE SPELL REFERENCE: `abilitydata.(Fey.sd.15-3:0-0:0-0:0-0:76-3.i.Ritemx.132fb.part.1.i.left.k.inflictexert.img.spark.hsv.-60:0:40.n.Thunderbolt)`. Damage 3 / 3 mana + singlecast + inflictexert. Paralysis one-shot.
- T3b Thunder: `abilitydata.(Fey.sd.88-5:0-0:0-0:0-0:76-4.img.IMG.n.Thunder)`
  - **FIXED**: Face 88 (SU Charged) NOT in Fey list. Per DEFINITIVE SPELL REFERENCE: `abilitydata.(Fey.sd.34-4:0-0:0-0:0-0:76-4.i.Ritemx.132fb.part.1.i.left.k.weaken.img.spark.hsv.-60:0:40.n.Thunder)`. AoE 4 / 4 mana + singlecast + weaken. Devastating storm.

**Stats**: Same as batch 3 and `line_93_pikachu.txt`. Template and spell fixes only.

---

#### 15. Chikorita -> Meganium (Color L, P1, Line 27, Healer template)

**Template**: `replica.Healer` (FULL_ROSTER specifies Healer)
**Currently generated as**: `replica.Lost` in `line_27_chikorita.txt`
**Color**: L, Line 27

**Spell validation**:
- Aromatherapy: `abilitydata.(Fey.sd.111-3:107-2:0-0:0-0:107-2.img.sprout.hsv.40:0:0.n.Aromatherapy)`
  - **FIXED**: Last face was `107-2` (Heal All) instead of mana cost. Per DEFINITIVE SPELL REFERENCE: `abilitydata.(Fey.sd.111-3:0-0:0-0:0-0:76-3.i.left.k.cleanse.img.sprout.hsv.40:0:0.n.Aromatherapy)`. Face 111 valid for Fey. Face 76 valid. Heal Cleanse 3 / 3 mana + cleanse keyword.

**Stats**: Same as batch 2 and `line_27_chikorita.txt`. Template and spell fixes.

---

#### 16. Larvitar -> Tyranitar (Color H, P1, Line 21, Statue template -- REDESIGN)

**Template**: `replica.Statue` (KEEP)
**Color**: H, Line 21
**Status**: Already generated in `line_21_larvitar.txt`

**HP EXCEPTIONS (pre-existing in mod, retained in redesign)**:
- T1 Larvitar HP 8: Exceeds T1 guideline of 3-6. Justified as a slow Rock/Dark tank with 2 blanks -- matches existing mod balance. Similar to Happiny (HP 9 at T1).
- T2 Pupitar HP 12: Exceeds T2 guideline of 6-9. Justified as a cocoon phase (like Shelgon) with heavy shields and the minusflesh damage reduction keyword. Pre-existing in mod.
- T3a Tyranitar HP 13: Within T3 guideline of 8-13. No exception needed.

**Stats**: Same as batch 3 redesign. Rock/Dark identity replacing poison. See batch 3 for full details.

---

#### 17. Charmander -> Charizard (Color Z, P1, Line 53, Statue template)

**Template**: `replica.Statue` (KEEP per both FULL_ROSTER and generated file)
**Color**: Z, Line 53
**Already generated**: `line_53_charmander.txt` -- VERIFIED WORKING in textmod.

**Spell validation**:
- Fire Blast: `abilitydata.(Statue.sd.34-3:34-2:0:0:76-3.n.Fire Blast.img.spark.hsv.-10:20:80)`
  - Face 34 valid for Statue. Face 76 valid. 2 active faces (Statue tolerates 2). PASSES.

**No changes needed.**

---

#### 18. Cyndaquil -> Typhlosion (Color Q, P2, Line 81, Lost template)

**Template**: `replica.Lost` (FULL_ROSTER says Lost)
**Color**: Q, Line 81
**Already generated**: `line_81_cyndaquil.txt`

**Spell validation**:
- Eruption: `abilitydata.(Statue.sd.34-4:34-3:0:0:76-4.i.Ritemx.132fb.part.1.n.Eruption.img.spark.hsv.-20:30:90)`
  - Face 34 valid for Statue. Face 76 valid. 2 active faces (Statue ok). PASSES.

**No changes needed.**

---

#### 19. NidoranF -> Nidoqueen (Color N, P1, Line 31, Statue template)

**Template**: `replica.Statue` (KEEP)
**Color**: N, Line 31
**Already generated**: `line_31_nidoranf.txt`

**Spell validation**:
- Earth Power: `abilitydata.(Fey.sd.39-4:0-0:0-0:0-0:76-4.img.IMG.n.Earth Power)`
  - **FIXED**: Face 39 (Heavy) NOT in Fey list. Replaced: `abilitydata.(Fey.sd.15-4:0-0:0-0:0-0:76-4.i.left.k.weaken.img.spark.hsv.20:30:0.n.Earth Power)`

---

#### 20. NidoranM -> Nidoking (Color N, P2, Line 75, Lost template)

**Template**: `replica.Lost` (KEEP)
**Color**: N, Line 75
**Already generated**: `line_75_nidoranm.txt`
**No spell** -- pure physical. No validation needed.

---

#### 21. Riolu -> Lucario (Color F, P2, NEW Line, Lost template)

**Template**: `replica.Lost` (KEEP per both FULL_ROSTER and generated file)
**Color**: F, NEW line
**Already generated**: `line_new_riolu.txt`

**Spell validation**:
- T3a Aura Sphere: `abilitydata.(Fey.sd.46-4:0-0:0-0:0-0:76-4.i.left.k.focus.img.IMG.n.Aura Sphere)`
  - **FIXED**: Face 46 (Ranged) NOT in Fey list. Replaced: `abilitydata.(Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.focus.img.light.hsv.-120:20:0.n.Aura Sphere)`
- T3b Close Combat: `abilitydata.(Fey.sd.174-5:0-0:0-0:0-0:76-3.img.IMG.n.Close Combat)`
  - **FIXED**: Face 174 (Defy) NOT in Fey list. Replaced: `abilitydata.(Fey.sd.15-5:0-0:0-0:0-0:76-4.i.left.k.vulnerable.i.left.togfri.img.spark.hsv.-120:20:0.n.Close Combat)`

---

#### 22. Dratini -> Dragonite (Color W, P2, NEW Line, Lost template)

**Template**: `replica.Lost` (KEEP)
**Color**: W
**Already generated**: `line_new_dratini.txt`

**Spell validation**:
- Dragon Dance: `abilitydata.(lost.sd.17-4:39-3:0:0:76-2.n.Dragon Dance.img.spark.hsv.30:40:80)`
  - **FIXED**: Faces 17, 39 NOT in lost list. Replaced with pure Focus sticker buff per SPELL_REDESIGN.md: `abilitydata.(lost.i.left.sticker.k.focus.sd.0:0:0:0:76-4.n.Dragon Dance.img.spark.hsv.30:40:80)`. Dragon Dance is a SETUP move -- the value comes from empowering all future attacks, not from direct damage. 4 mana matches Psych Up precedent.
- Outrage: `abilitydata.(lost.sd.39-5:36-3:0:0:76-3.n.Outrage.img.scorch.hue.40)`
  - **FIXED**: Face 39 NOT in lost list (face 36 IS valid). Replaced: `abilitydata.(lost.sd.36-5:0:0:0:76-4.n.Outrage.img.scorch.hue.40)`. Cleave 5 / 4 mana -- simple devastating cleave. Only 1 active face (cleaner design per SPELL_REDESIGN.md).

---

#### 23. Bagon -> Salamence (Color W, P1, Line 99, Statue template)

**Template**: `replica.Statue` (KEEP)
**Color**: W, Line 99
**Already generated**: `line_99_bagon.txt`
**No spell** -- pure physical. No validation needed.

---

#### 24. Weedle -> Beedrill (Color T, P2, Line 87, Lost template)

**Template**: `replica.Lost` (KEEP)
**Color**: T, Line 87
**Already generated**: `line_87_weedle.txt`
**No spell** -- pure physical. No validation needed.

---

#### 25. Turtwig -> Torterra (Color M, P1 -- MOVE from N to M, REPLACES Rockruff)

**Template**: `replica.Lost` (KEEP existing)
**Color**: M (was N), Line 29 (currently Rockruff/Lycanroc)
**Status**: Turtwig moves from Color N Line 75 to Color M Line 29. This REPLACES Rockruff/Lycanroc entirely -- Rockruff is not in FULL_ROSTER.md and is being cut from the mod.
**Action**:
1. Replace Rockruff hero data on Line 29 with Turtwig hero data (from current Line 75)
2. Update `.col.` from n to m on the Turtwig data
3. Remove the now-empty Turtwig data from Line 75 (Line 75 will be used by NidoranM per FULL_ROSTER: Color N P2 = NidoranM)
4. Also update Line 11 character selection to reflect color changes
**WARNING**: Rockruff/Lycanroc removal means any references to Rockruff in Ditto's copy line (Line 99, the massive 66K line) must also be removed or will cause stale references.

---

### Section 1C: Spell Validation Summary

**ALL spells must be re-validated against the face ID restriction lists.**

**NOTE**: This table shows the ORIGINAL (invalid) spell faces and the initial fix applied. The DEFINITIVE SPELL REFERENCE at the end of this plan supersedes all fixes below -- it includes balance changes (pip adjustments, added keywords, singlecast modifiers) applied during the spell redesign review. Always use the DEFINITIVE SPELL REFERENCE for implementation.

| Hero | Spell | Template | Original Faces | Valid? | Fix (see DEFINITIVE SPELL REFERENCE for final version) |
|------|-------|----------|---------------|--------|-----|
| Charizard | Fire Blast | Statue | 34-3:34-2:0:0:76-3 | YES | None |
| Typhlosion | Eruption | Statue | 34-4:34-3:0:0:76-4 | YES | None (singlecast added in DEFINITIVE) |
| Blaziken | Blaze Kick | Fey | 174-3:175-2:0:0:76-2 | NO (174,175 invalid) | Face fix + inflictpain. See DEFINITIVE. |
| Dragonite | Dragon Dance | lost | 17-4:39-3:0:0:76-2 | NO (17,39 invalid) | Redesigned to pure Focus sticker buff (no damage faces). See DEFINITIVE. |
| Dragonite | Outrage | lost | 39-5:36-3:0:0:76-3 | NO (39 invalid) | `36-5:0:0:0:76-4` (single active face, 4 mana). See DEFINITIVE. |
| Metagross | Meteor Mash | lost | 39-5:39-4:0:0:76-3 | NO (39 invalid) | `15-5:0:0:0:76-4` + focus keyword. See DEFINITIVE. |
| Sceptile | Leaf Blade | Fey | 175-4:175-3:0-0:0-0:17-2 | NO (175,17 invalid; missing mana cost) | `15-4:0-0:0-0:0-0:76-3` + focus keyword. Damage 4 / 3 mana + focus (high crit → Growth synergy). See DEFINITIVE. |
| Meganium | Aromatherapy | Fey | 111-3:107-2:0-0:0-0:107-2 | NO (no mana cost) | `111-3:0-0:0-0:0-0:76-3` + cleanse keyword. See DEFINITIVE. |
| Venusaur | Solar Beam | Statue | 42-5:15-3:0:0:76-3 | NO (42 invalid) | `15-4:0:0:0:76-4` + vulnerable keyword. See DEFINITIVE. |
| Togekiss | Wish | Fey | 107-4:0-0:0-0:0-0:76-5 | YES | Rebalanced to `107-3:0-0:0-0:0-0:76-3` (3 heal / 3 mana). See DEFINITIVE. |
| Togekiss | Dazzling Gleam | Fey | 34-3:0-0:0-0:0-0:76-4 | YES | Added vulnerable keyword. See DEFINITIVE. |
| Clefable | Metronome | Fey | 27-4:0-0:0-0:0-0:76-4 | NO (27 invalid) | `136-2:0-0:0-0:0-0:76-4` (Revive 2, singlecast). See DEFINITIVE. |
| Clefable | Moonlight | Fey | 107-3:0-0:0-0:0-0:76-5 | YES | Rebalanced to `107-3:0-0:0-0:0-0:76-3` + cleanse. See DEFINITIVE. |
| Raichu | Thunderbolt | Fey | 42-4:0-0:0-0:0-0:76-4 | NO (42 invalid) | `15-3:0-0:0-0:0-0:76-3` (singlecast + inflictexert). See DEFINITIVE. |
| Raichu | Thunder | Fey | 88-5:0-0:0-0:0-0:76-4 | NO (88 invalid) | `34-4:0-0:0-0:0-0:76-4` (AoE 4, singlecast + weaken). See DEFINITIVE. |
| Glalie | Sheer Cold | Fey | 34-4:0-0:0-0:0-0:76-4 | YES | Rebalanced to `34-5:0-0:0-0:0-0:76-5` (singlecast + weaken). See DEFINITIVE. |
| Froslass | Ice Beam | Fey | 131-4:0-0:0-0:0-0:76-3 | YES | Rebalanced to `131-3:0-0:0-0:0-0:76-3` (singlecast + inflictsingleuse). See DEFINITIVE. |
| Nidoqueen | Earth Power | Fey | 39-4:0-0:0-0:0-0:76-4 | NO (39 invalid) | `15-4:0-0:0-0:0-0:76-4` + weaken. See DEFINITIVE. |
| Lucario | Aura Sphere | Fey | 46-4:0-0:0-0:0-0:76-4 | NO (46 invalid) | `15-4:0-0:0-0:0-0:76-3` + focus. See DEFINITIVE. |
| Lucario | Close Combat | Fey | 174-5:0-0:0-0:0-0:76-3 | NO (174 invalid) | `15-5:0-0:0-0:0-0:76-4` + vulnerable + togfri. See DEFINITIVE. |
| Aggron | Iron Tail | Statue | 15-5:0-0:0-0:0-0:76-3 | YES | Rebalanced to `15-4:0:0:0:76-3` + focus. See DEFINITIVE. |
| Politoed | Rain Dance | Fey | 72-3:107-2:0-0:0-0:76-2 | YES | Redesigned to `107-2:0-0:0-0:0-0:76-3` + Ritemx.62e8 + cleanse. See DEFINITIVE. |

**Fix count**: 12 spells need face ID corrections. All 21 spells were rebalanced during the spell redesign review -- see DEFINITIVE SPELL REFERENCE for final abilitydata strings.

---

### Section 1D: Budget Exception Summary

The following heroes intentionally exceed standard tier guidelines. Each exception is justified by a specific design pattern with precedent in the existing mod:

| Hero | Stat | Value | Guideline | Justification | Precedent |
|------|------|-------|-----------|---------------|-----------|
| Wailmer T1 | Blanks | 4 | 2-3 | Delayed payoff pattern (worst T1 -> strongest T3) | Magikarp T1: 5 stasis blanks |
| Wailmer T2a | Blanks | 2 | 1-2 | Ongoing investment tax | Magikarp T2 pattern |
| Wailord T3a | HP | 14 | 8-13 | Earned by worst T1/T2 face ratios in the game | Gyarados T3: HP 13 |
| Larvitar T1 | HP | 8 | 3-6 | Pre-existing in mod; slow tank archetype | Happiny T1: HP 9 |
| Pupitar T2a | HP | 12 | 6-9 | Pre-existing; cocoon phase with minusflesh | Slowbro T3: HP 28/3! |
| Pupitar T2b | HP | 12 | 6-9 | Pre-existing; cocoon phase | Same as T2a |

All other heroes are within standard guidelines.

---

## PART 2: Monster Changes

### 2A: Replace Aggron Monster with Probopass

**Reason**: Aron is now a hero (Color H, P2). Having both a hero named Aggron and a monster named Aggron would cause confusion and potential `.n.` conflicts.

**Current state**: \"Aggron\" is a base-game rename of the Troll monster template, appearing in:
- Floor 9-11 pool (Line 121)
- Floor 17-19 pool (Line 123)

**Action**: Find-and-replace `.n.Aggron` with `.n.Probopass` on Lines 121 and 123. Keep all dice stats, HP, and keywords identical (it is a base-game Troll with a cosmetic rename).

**Sprite**: Download Probopass sprite from PMDCollab, encode with `tools/encode_sprite.js`, update the `.img.` property on the renamed monster entries. Add \"Probopass\" to `tools/batch_sprites.js` POKEMON_LIST and re-run.

**Probopass justification**: Steel/Rock type that fits the Troll template's tanky high-HP high-shield identity. Probopass is a defensive wall with magnets -- thematically similar to what the Troll (now Aggron) monster does. It does not conflict with any hero, capture, or boss in the roster.

**FULL_ROSTER discrepancy**: `plans/FULL_ROSTER.md` Monsters table still lists "Aggron" (line 157). After this rename is implemented, FULL_ROSTER.md must be updated to show "Probopass" instead. This is a documentation-only fix and does not block implementation.

**FULL_ROSTER discrepancy**: `plans/FULL_ROSTER.md` Monsters table still lists "Aggron" (line 157). After this rename is implemented, FULL_ROSTER.md must be updated to show "Probopass" instead. This is a documentation-only fix and does not block implementation.

### 2B: Alakazam Monster (No Change)

Per the requirements: \"Replace Alakazam monster rename with Alakazam (keep as-is, it's just a Wizz rename).\" The Alakazam monster is already a cosmetic rename of the Wizz base-game monster. No action needed.

---

## PART 3: Capture Changes

**FULL_ROSTER discrepancy**: `plans/FULL_ROSTER.md` Captures table still lists Charizard (Ultra Ball, ORIG) and Pikachu (Poke Ball, ORIG) even though both are now hero evolution lines and must be removed as captures. It also still shows Barboach instead of Whiscash upgrade. After capture changes are implemented, FULL_ROSTER.md Captures table must be updated to reflect removals, upgrades, and additions. This is documentation-only and does not block implementation.

**FULL_ROSTER discrepancy**: `plans/FULL_ROSTER.md` Captures table still lists Charizard (Ultra Ball, ORIG) and Pikachu (Poke Ball, ORIG) even though both are now hero evolution lines and must be removed as captures. After capture changes are implemented, FULL_ROSTER.md Captures table must be updated. This is documentation-only and does not block implementation.

### 3A: Remove Ivysaur from Capture Pool

**Reason**: Bulbasaur is now a hero line. Ivysaur cannot be both a capturable and a hero evolution.
**Action**: Remove Ivysaur capture entry from Line 111. Find the ball item entry referencing Ivysaur and delete it.

### 3B: Upgrade Barboach to Whiscash (Dive Ball)

**Current**: Barboach capture with Dive Ball.
**Action**: Replace Barboach sprite with Whiscash sprite. Change `.n.Barboach` to `.n.Whiscash`. Keep Dive Ball item type. Update dice stats for Whiscash (higher HP, better faces as a final evolution).
**NOTE**: Whiscash dice stats, HP, and keywords are not yet specified. These must be designed before implementation -- study Barboach's current stats and scale up appropriately for a Water/Ground final evolution (expect HP ~8-10, add Heavy damage or Shield faces to reflect Ground typing). Flag for user decision if stat design is needed during Chunk 10.

### 3C: Other Capture Changes (per EXPANSION_PLAN.md)

| Change | Action | Line |
|--------|--------|------|
| Remove Pikachu capture | Delete from L111 (now hero) | 111 |
| Remove Charizard capture | Delete from L111 (now hero) | 111 |
| Remove Metagross capture | Delete from L111 (now hero) | 111 |
| Remove Poliwag capture | Delete from L111 (now hero) | 111 |
| Remove Arceus capture | Delete from L63 and L111 (now boss) | 63, 111 |
| Upgrade Caterpie to Butterfree | Replace sprite + name, keep Nest Ball | 111 |
| Upgrade Sneasel to Weavile | Replace sprite + name, keep Fast Ball | 111 |
| Upgrade Electrike to Manectric | Replace sprite + name, keep Quick Ball | 111 |
| Replace Rattata with Skarmory | New Pokemon, keep Level Ball | 111 |
| Replace Furret with Lapras | New Pokemon, keep Friend Ball | 111 |
| Replace Alcremie with Arcanine | New Pokemon, keep Premier Ball | 111 |
| Remove Zubat capture | Delete (now monster-only) | 111 |
| Add Mew (Poke Ball) | New capture | 111 |
| Add Jirachi (Great Ball) | New capture | 111 |
| Add Kangaskhan (Safari Ball) | New capture | 111 |
| Add Heracross (Net Ball) | New capture | 111 |
| Add Greninja (Dusk Ball) | New capture | 111 |
| Add Electivire (Ultra Ball) | New capture | 111 |
| Add Magmortar (Lure Ball) | New capture | 111 |
| Add Rhyperior (Heavy Ball) | New capture | 111 |

### 3C-1: Capture Dice Designs

All captures use `replica.Thief` template. All new/upgraded capture dice use hero-style Face IDs (15, 36, etc.), NOT monster-style (170, 171). Existing untouched captures (Rattata, Furret, Snorlax, Wobbuffet, Lilligant, Mewtwo) keep their current stats.

#### Upgraded Captures (keep ball type, improve stats)

| Pokemon | Ball | HP | sd | Face Summary | Design Rationale |
|---------|------|----|----|--------------|-----------------|
| Whiscash | Dive | 6 | `15-3:15-2:76-2:76-1:0:0` | Dmg 3, Dmg 2, Mana 2, Mana 1, Blank×2 | Ground/Water upgrade from Barboach's pure mana. Adds damage while keeping mana utility. |
| Butterfree | Nest | 4 | `53-2:53-1:131-1:131-1:0:0` | Poison 2, Poison 1, Weaken 1×2, Blank×2 | Bug/Flying status spreader. Compound Eyes → poison + debuff. Upgrade from Caterpie's single face. |
| Weavile | Fast | 4 | `15-3:15-3:30-2:30-2:0:0` | Dmg 3×2, Cruel 2×2, Blank×2 | Dark/Ice assassin. Pressure + Technician → consistent damage with Cruel (Dark keyword). Upgrade from Sneasel's 170 faces. |
| Manectric | Quick | 7 | `15-3:15-3:46-2:46-2:0:0` | Dmg 3×2, Ranged 2×2, Blank×2 | Electric ranged attacker. Lightning Rod → damage at distance. Upgrade from Electrike's 170/171 faces. |

#### Replacement Captures (new Pokemon, keep ball type from replaced capture)

| Pokemon | Ball | HP | sd | Face Summary | Design Rationale |
|---------|------|----|----|--------------|-----------------|
| Skarmory | Luxury | 8 | `63-3:63-2:119-2:119-1:0:0` | Steel Shield 3, Steel Shield 2, Repel 2, Repel 1, Blank×2 | Steel/Flying wall. Sturdy + Whirlwind → pure defense, shields + repel. Replaces Lilligant. |
| Lapras | Moon | 10 | `56-3:56-3:103-2:103-2:0:0` | Shield 3×2, Heal 2×2, Blank×2 | Water/Ice transport. Shell Armor → support tank with shields + heals. HP 10 = bulky. Replaces Delcatty. |
| Arcanine | Premier | 8 | `15-4:15-3:17-3:17-2:0:0` | Dmg 4, Dmg 3, Engage 3, Engage 2, Blank×2 | Fire Legendary (in-lore). Intimidate + Extreme Speed → aggressive multi-strike. Tier 8 power. Replaces Alcremie. |

#### New Captures

| Pokemon | Ball | HP | sd | Face Summary | Design Rationale |
|---------|------|----|----|--------------|-----------------|
| Mew | Poke | 5 | `15-2:56-2:103-2:76-1:0:0` | Dmg 2, Shield 2, Heal 2, Mana 1, Blank×2 | Psychic mythical. Transform → jack-of-all-trades, does everything at low pips. Tier 3 power. |
| Jirachi | Great | 6 | `107-3:107-2:56-2:76-1:0:0` | Heal All 3, Heal All 2, Shield 2, Mana 1, Blank×2 | Steel/Psychic wish-granter. Serene Grace → heal-focused support. Tier 5 power. |
| Kangaskhan | Safari | 7 | `15-3:15-3:51-2:51-2:0:0` | Dmg 3×2, SelfShield 2×2, Blank×2 | Normal. Parental Bond → hits hard + self-protects. Tier 5 power. |
| Heracross | Net | 6 | `17-3:17-3:15-2:15-2:0:0` | Engage 3×2, Dmg 2×2, Blank×2 | Bug/Fighting. Megahorn + Close Combat → aggressive Engage attacker. Tier 6 power. |
| Greninja | Dusk | 5 | `46-3:46-2:15-2:15-2:0:0` | Ranged 3, Ranged 2, Dmg 2×2, Blank×2 | Water/Dark ninja. Protean + Water Shuriken → ranged assassin. Tier 6 power. |
| Electivire | Ultra | 7 | `15-4:15-3:15-3:15-2:0:0` | Dmg 4, Dmg 3×2, Dmg 2, Blank×2 | Electric. Motor Drive → pure damage powerhouse. Tier 7 power, replaces Charizard's Ultra Ball slot. |
| Magmortar | Lure | 7 | `34-3:34-2:15-3:15-2:0:0` | AoE 3, AoE 2, Dmg 3, Dmg 2, Blank×2 | Fire. Flame Body → fire cannon with AoE splash. Tier 6 power, replaces Poliwag's Lure Ball slot. |
| Rhyperior | Heavy | 10 | `39-4:39-3:63-3:63-2:0:0` | Heavy 4, Heavy 3, Steel Shield 3, Steel Shield 2, Blank×2 | Ground/Rock. Solid Rock → heavy tank-DPS hybrid. HP 10 = beefy. Replaces Metagross's Heavy Ball slot. |

### 3D: New Legendary Items

| Pokemon | Item | Action |
|---------|------|--------|
| Latias | Soul Dew | New legendary summon item |
| Latios | Eon Flute | New legendary summon item |
| Suicune | Clear Bell | New legendary summon item |
| Entei | Flame Plate | New legendary summon item |
| Raikou | Zap Plate | New legendary summon item |
| Rayquaza | Jade Orb | New legendary summon item |

### 3D-1: Legendary Dice Designs

All legendaries use `replica.Thief` template, **HP 70** (fixed), and **flee on turn 7** (matching existing Ho-Oh/Lugia/Kyogre/Groudon). Legendaries are powerful temporary summons -- their pip values are intentionally extreme (8-12 range) to justify the flee timer. Study existing legendary implementations on Lines 115 and 117 of textmod.txt for the flee mechanic encoding.

**Pip budget reference** (existing legendaries): Ho-Oh = 70 total pips + revive mechanic, Lugia = 58 pips (defensive), Kyogre = 20 pips + facade, Groudon = 30 pips + facade. New legendaries without facade systems target 40-60 total pips.

| Pokemon | Item | HP | sd | Face Summary | Total Pips | Design Rationale |
|---------|------|----|----|--------------|------------|-----------------|
| Latias | Soul Dew | 70 | `56-12:56-12:107-8:107-8:123-0:123-0` | Shield 12×2, Heal All 8×2, Dodge×2 | 40 + Dodge | Psychic/Dragon defensive twin. Mist Ball → shields + heals + evasion. Lower pips offset by Dodge utility. |
| Latios | Eon Flute | 70 | `15-12:15-12:36-8:36-8:46-5:46-5` | Dmg 12×2, Cleave 8×2, Ranged 5×2 | 50 | Psychic/Dragon offensive twin. Luster Purge → damage + cleave + ranged. Mirrors Latias as offense vs defense. |
| Suicune | Clear Bell | 70 | `56-12:56-12:71-8:71-8:107-5:107-5` | Shield 12×2, Shield Cleanse 8×2, Heal All 5×2 | 50 | Water aurora beast. Purifying aura → shields + cleanses + heals. Anti-debuff legendary. |
| Entei | Flame Plate | 70 | `15-12:15-12:34-8:34-8:15-5:15-5` | Dmg 12×2, AoE 8×2, Dmg 5×2 | 50 | Fire volcano beast. Sacred Fire → raw single-target damage + AoE. Pure offense. |
| Raikou | Zap Plate | 70 | `15-12:15-12:15-10:15-10:36-5:36-5` | Dmg 12×2, Dmg 10×2, Cleave 5×2 | 54 | Electric thunder beast. Thunderbolt → consistent high damage + cleave. Highest single-target DPS legendary. |
| Rayquaza | Jade Orb | 70 | `34-12:34-12:36-10:36-10:15-8:15-8` | AoE 12×2, Cleave 10×2, Dmg 8×2 | 60 | Dragon/Flying sky sovereign. Air Lock → ultimate AoE powerhouse. Highest pip budget = final legendary. |

**Role differentiation across all 10 legendaries**:
- Pure Offense: Latios, Entei, Raikou, Rayquaza
- Pure Defense: Latias, Suicune, Lugia
- Unique Mechanic: Ho-Oh (revive), Kyogre (facade), Groudon (facade + sub-form)

**Speech and doc strings**: To be designed during Chunk 10 implementation -- these are cosmetic and don't affect gameplay balance or dice mechanics.

---

## PART 4: Testing Plan

### Phase 0: Template Isolation Tests (HIGHEST PRIORITY)

Test each new template in complete isolation before combining:

1. Create a minimal textmod with ONLY one hero using each new template
2. For each template, add a single T1 hero entry to the heropool on a spare line
3. Paste into game, verify:
   - Hero appears in draft
   - Die faces display correctly
   - HP is correct (not inheriting template default)
   - No garbled name
   - No inherited spells we didn't want

**Test order** (by risk level):
1. `replica.Fighter` -- already used by 2 heroes in base Sliceymon, lowest risk
2. `replica.Stalwart` -- used in pansaer.txt, medium risk
3. `replica.Guardian` -- used in pansaer.txt, medium risk
4. `replica.Dancer` -- used in pansaer.txt, medium risk
5. `replica.Fencer` -- used in pansaer.txt, medium risk
6. `replica.Alloy` -- used in pansaer.txt, medium risk
7. `replica.Eccentric` -- used in pansaer.txt, medium risk

### Phase 1: Proven-Template Heroes (14 heroes, lower risk)

Test heroes that use Lost, Statue, Thief, Healer, Primrose templates:

**Batch 1a -- Already generated and tested** (verify spell fixes only):
1. Charmander (Statue, L53) -- no spell fix needed
2. Cyndaquil (Lost, L81) -- no spell fix needed  
3. Bagon (Statue, L99) -- no spell needed
4. Weedle (Lost, L87) -- no spell needed
5. NidoranM (Lost, L75) -- no spell needed
6. Larvitar redesign (Statue, L21) -- no spell needed

**Batch 1b -- Need spell fixes before testing**:
7. Treecko (Primrose, L71) -- fix Leaf Blade spell faces
8. Chikorita (Healer, L27) -- fix Aromatherapy mana cost
9. NidoranF (Statue, L31) -- fix Earth Power spell faces
10. Dragonite (Lost, new) -- fix Dragon Dance and Outrage spell faces
11. Riolu (Lost, new) -- fix Aura Sphere and Close Combat spell faces

### Phase 2: New-Template Heroes (11 heroes, higher risk)

Test after Phase 0 template isolation passes:

**Batch 2a -- Simple heroes (no spells)**:
1. Machop/Fighter (F P1) -- no spell
2. Totodile/Fighter (J P1) -- no spell
3. Wailmer/Stalwart (M P2) -- no spell

**Batch 2b -- Heroes with spells**:
4. Poliwag/Fighter (J P2) -- Politoed has Rain Dance spell (validated OK)
5. Togepi/Dancer (R P1) -- Wish and Dazzling Gleam (validated OK)
6. Mudkip/Guardian (G P2) -- no spell
7. Bulbasaur/Guardian (P P2) -- Solar Beam spell (needs fix)
8. Snorunt/Eccentric (B P1) -- Sheer Cold and Ice Beam spells (validated OK)
9. Aron/Stalwart (H P2) -- Iron Tail spell (validated OK)
10. Cleffa/Fencer (U P2) -- Metronome and Moonlight spells (Metronome needs fix)
11. Beldum/Alloy (Z P2) -- Meteor Mash spell (needs fix)

**Batch 2c -- Template + spell fix heroes (test last)**:
- Torchic/Thief (K P2) -- Blaze Kick spell needs fix
- Pichu/Thief (X P2) -- Thunderbolt and Thunder spells need fix

### Phase 3: Monster & Capture Changes

Test after all heroes pass:
1. Probopass monster rename (quick find-replace test)
2. Capture removals (test draft doesn't show removed captures)
3. Capture upgrades (test Whiscash appears instead of Barboach)
4. New captures (test new ball items appear)

### Phase 4: Integration Test

Full textmod with all changes:
1. Play a complete run on Normal difficulty
2. Verify all 25 color draft options work
3. Verify branching evolutions display correctly
4. Verify spells cast and display correctly
5. Verify monsters appear on correct floors
6. Verify Probopass appears instead of Aggron

---

## Checkpoint Configuration

- **Total chunks**: 11
- **Checkpoint frequency**: At critical gates only (not after every chunk -- most chunks are mechanical and self-verifying)
- **Critical checkpoints**:
  - After Chunk 1 (template research -- all subsequent work depends on this)
  - After Chunk 5 (L63 compound line surgery -- high-risk edit, verify before proceeding)
  - After Chunk 6 (template isolation tests -- GATE: new-template heroes cannot proceed until user confirms templates work in-game)
  - After Chunk 11 (final integration -- full playthrough verification)
- **Self-verifying chunks** (no checkpoint pause needed): Chunks 2, 3, 4, 7, 8, 9, 10 -- these have automated validation (`validate_textmod.js`, grep checks, paren balance) that confirm success without human review

---

## Parallel Execution Map

```
Foundation (sequential):
  Chunk 1: Template Research

Parallel Group A (after Chunk 1):
  Chunk 2: Spell Fixes
  Chunk 4: Snorunt Design (New Hero)
  Chunk 5: Aron + L63 Compound Line Surgery (depends on template research only)

Sequential (after Chunk 2):
  Chunk 3: Template Migration (depends on Chunk 2 -- shared files)

Gate Checkpoint (after Chunks 3, 4, 5 all complete):
  Chunk 6: Template Isolation Tests

Sequential (after Chunk 6 passes):
  Chunk 7: Proven-Template Hero Assembly + Line 11 Sort

Sequential (after Chunk 7):
  Chunk 8: New-Template Hero Assembly (depends on Chunk 7 textmod)

Parallel Group B (after Chunk 8):
  Chunk 9: Monster Changes (modifies textmod_heroes_only.txt)
  Chunk 10: Capture Changes (modifies textmod_heroes_only.txt -- different lines than Chunk 9)

Sequential (after Chunks 9 + 10):
  Chunk 11: Full Integration Test
```

**Minimum wall-clock rounds**: 7 (vs 11 sequential)
**Critical path**: 1 -> 2 -> 3 -> 6 -> 7 -> 8 -> 9/10 -> 11

**NOTE**: Chunks 9 and 10 modify different sections of `textmod_heroes_only.txt` (monster pool lines vs capture pool lines), so they can safely run in parallel. If the AI executor cannot guarantee non-overlapping edits, run them sequentially (9 then 10).

---



---

## DEFINITIVE SPELL REFERENCE (supersedes all other spell specifications in this plan)

All spells have been reviewed and balanced by the slice-and-dice-design persona. Use these exact abilitydata strings.

| Hero | Spell | Abilitydata String |
|------|-------|--------------------|
| Charizard | Fire Blast | `(Statue.sd.34-3:34-2:0:0:76-3.n.Fire Blast.img.spark.hsv.-10:20:80)` |
| Typhlosion | Eruption | `(Statue.sd.34-4:34-3:0:0:76-4.i.Ritemx.132fb.part.1.n.Eruption.img.spark.hsv.-20:30:90)` |
| Blaziken | Blaze Kick | `(Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.inflictpain.img.scorch.hue.10.n.Blaze Kick)` |
| Metagross | Meteor Mash | `(lost.sd.15-5:0:0:0:76-4.i.left.k.focus.n.Meteor Mash.img.mithril shields.hsv.-120:30:70)` |
| Venusaur | Solar Beam | `(Statue.sd.15-4:0:0:0:76-4.i.left.k.vulnerable.n.Solar Beam.img.beam.hsv.40:0:0)` |
| Meganium | Aromatherapy | `(Fey.sd.111-3:0-0:0-0:0-0:76-3.i.left.k.cleanse.img.sprout.hsv.40:0:0.n.Aromatherapy)` |
| Nidoqueen | Earth Power | `(Fey.sd.15-4:0-0:0-0:0-0:76-4.i.left.k.weaken.img.spark.hsv.20:30:0.n.Earth Power)` |
| Glalie | Sheer Cold | `(Fey.sd.34-5:0-0:0-0:0-0:76-5.i.Ritemx.132fb.part.1.i.left.k.weaken.img.light.hsv.-120:0:40.n.Sheer Cold)` |
| Froslass | Ice Beam | `(Fey.sd.131-3:0-0:0-0:0-0:76-3.i.Ritemx.132fb.part.1.i.left.k.inflictsingleuse.img.light.hsv.-120:0:40.n.Ice Beam)` |
| Togekiss T3a | Wish | `(Fey.sd.107-3:0-0:0-0:0-0:76-3.img.light.n.Wish)` |
| Togekiss T3b | Dazzling Gleam | `(Fey.sd.34-3:0-0:0-0:0-0:76-4.i.left.k.vulnerable.img.light.n.Dazzling Gleam)` |
| Clefable T3a | Metronome | `(Fey.sd.136-2:0-0:0-0:0-0:76-4.i.Ritemx.132fb.part.1.img.light.n.Metronome)` |
| Clefable T3b | Moonlight | `(Fey.sd.107-3:0-0:0-0:0-0:76-3.i.left.k.cleanse.img.light.n.Moonlight)` |
| Raichu T3a | Thunderbolt | `(Fey.sd.15-3:0-0:0-0:0-0:76-3.i.Ritemx.132fb.part.1.i.left.k.inflictexert.img.spark.hsv.-60:0:40.n.Thunderbolt)` |
| Raichu T3b | Thunder | `(Fey.sd.34-4:0-0:0-0:0-0:76-4.i.Ritemx.132fb.part.1.i.left.k.weaken.img.spark.hsv.-60:0:40.n.Thunder)` |
| Dragonite T3a | Dragon Dance | `(lost.i.left.sticker.k.focus.sd.0:0:0:0:76-4.n.Dragon Dance.img.spark.hsv.30:40:80)` |
| Dragonite T3b | Outrage | `(lost.sd.36-5:0:0:0:76-4.n.Outrage.img.scorch.hue.40)` |
| Lucario T3a | Aura Sphere | `(Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.focus.img.light.hsv.-120:20:0.n.Aura Sphere)` |
| Lucario T3b | Close Combat | `(Fey.sd.15-5:0-0:0-0:0-0:76-4.i.left.k.vulnerable.i.left.togfri.img.spark.hsv.-120:20:0.n.Close Combat)` |
| Aggron | Iron Tail | `(Statue.sd.15-4:0:0:0:76-3.i.left.k.focus.n.Iron Tail.img.mithril shields.hsv.0:0:0)` |
| Politoed | Rain Dance | `(Fey.sd.107-2:0-0:0-0:0-0:76-3.i.Ritemx.62e8.i.left.k.cleanse.img.flick.hsv.-50:-30:0.n.Rain Dance)` |
| Sceptile T3a | Leaf Blade | `(Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.focus.img.spark.hsv.60:0:0.n.Leaf Blade)` |

See `plans/SPELL_REDESIGN.md` for full design rationale, mechanic explanations, and balance review.


## Implementation Chunks (Execution Order)

### Chunk 1: Template Research [CRITICAL CHECKPOINT]

**Scope**: Extract all 7 new template defaults from the working pansaer.txt mod to understand what properties each template provides by default.
**Dependencies**: None
**Concern**: Template property extraction

**Files** (read-only):
1. `/Users/hgorelick/Documents/slice-and-dice/working-mods/pansaer.txt`

**Files** (create):
2. `/Users/hgorelick/Documents/slice-and-dice/plans/TEMPLATE_PROPERTIES.md`

**Source of truth**: `working-mods/pansaer.txt`

**Requirements**:
- Parse pansaer.txt to find every `replica.Eccentric`, `replica.Stalwart`, `replica.Dancer`, `replica.Fencer`, `replica.Alloy`, `replica.Fighter`, `replica.Guardian` usage
- For each template, document: default HP, default sd faces, default color, default keywords, default abilitydata (spells), default doc strings, default facades, default triggerhpdata
- Flag any template that has built-in abilitydata (spells) -- these MUST be explicitly cleared or overridden in our heroes
- Flag any template that has built-in keywords that conflict with our designs

**Verification**:
- [ ] All 7 templates documented
- [ ] Each template entry lists all default properties found
- [ ] Any built-in abilitydata flagged with CRITICAL warning
- [ ] Output saved to `plans/TEMPLATE_PROPERTIES.md`

**If blocked**: If pansaer.txt does not contain a template (e.g., the template name is different), search for partial matches. If truly absent, flag for user and proceed with remaining templates.

**Test cases**:
- Confirm each template name appears at least once in pansaer.txt
- Confirm extracted HP values are reasonable (1-20 range)
- Confirm no template is missed by cross-referencing the 7-template list

---

### Chunk 2: Spell Fixes [PARALLEL GROUP A]

**Scope**: Apply the correct spell abilitydata strings from the DEFINITIVE SPELL REFERENCE to all heroes with spells. This includes fixing 12 originally-invalid face IDs and applying balance changes (pip adjustments, keywords, singlecast modifiers) from the spell redesign review.
**Dependencies**: None (spell fixes use the validation table in this plan, not template research)
**Concern**: Spell face ID validation and correction
**Parallel with**: Chunk 4

**Files** (modify):
1. `generated/line_69_torchic.txt` -- Blaze Kick spell fix
2. `generated/line_new_dratini.txt` -- Dragon Dance + Outrage spell fixes
3. `generated/line_97_beldum.txt` -- Meteor Mash spell fix
4. `generated/line_71_treecko.txt` -- Leaf Blade spell ADDITION (generated file has NO abilitydata -- add `abilitydata.(Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.focus.img.spark.hsv.60:0:0.n.Leaf Blade)` per DEFINITIVE SPELL REFERENCE)
5. `generated/line_27_chikorita.txt` -- Aromatherapy mana cost fix

**Files** (modify, continued in same concern):
6. `generated/line_79_bulbasaur.txt` -- Solar Beam spell ADDITION (generated file has NO abilitydata -- add per DEFINITIVE SPELL REFERENCE)
7. `generated/line_89_cleffa.txt` -- Metronome spell fix (face 27 -> 136)
8. `generated/line_93_pikachu.txt` -- Thunderbolt + Thunder spell fixes (faces 42,88 -> 15)
9. `generated/line_31_nidoranf.txt` -- Earth Power spell fix (face 39 -> 15)
10. `generated/line_new_riolu.txt` -- Aura Sphere + Close Combat spell fixes (faces 46,174 -> 15)
11. `generated/line_39_togepi.txt` -- Wish rebalanced (107-4->107-3, 76-5->76-3) + Dazzling Gleam add vulnerable keyword
12. `generated/line_81_cyndaquil.txt` -- Eruption add singlecast modifier (i.Ritemx.132fb.part.1)
13. `generated/line_new_poliwag.txt` -- Rain Dance complete redesign (Statue->Fey, face 103->107, add Ritemx+cleanse)

**NOTE**: This chunk modifies 13 files, exceeding the 5-file soft limit. This is acceptable because every change is the same mechanical pattern (find abilitydata string, replace with the correct version from the DEFINITIVE SPELL REFERENCE). The concern is singular and the changes are formulaic. Splitting into two chunks would add overhead without benefit.

**Source of truth**: DEFINITIVE SPELL REFERENCE table at the end of this plan (supersedes Section 1C)

**Requirements**:
- Apply exactly the spell abilitydata strings from the DEFINITIVE SPELL REFERENCE table
- For each file, find the `abilitydata.(` string and replace the invalid face IDs
- Do NOT change any other part of the generated files (HP, sd, keywords, speech, sprites)
- Also fix any `.img.IMG` placeholders in spell strings with appropriate spell icons (e.g., `spark`, `light`, `beam`, `scorch`)

**Verification**:
- [ ] All spells in every modified file match the DEFINITIVE SPELL REFERENCE exactly
- [ ] Every spell face ID in every generated file is in the approved list for its spell template
- [ ] No other properties were changed
- [ ] Run `node tools/validate_textmod.js` on each modified file -- paren balance = 0

**If blocked**: If a generated file's abilitydata format doesn't match expected pattern, read the file to understand its actual structure before applying fixes.

**Test cases**:
- For each modified file, extract the `abilitydata.(...)` string and verify it matches the DEFINITIVE SPELL REFERENCE entry for that hero character-for-character. NOTE: face IDs like 39 and 17 appear legitimately on hero sd faces -- only check inside abilitydata strings, not the entire file.
- Grep for `.img.IMG` in abilitydata strings -- should find zero matches after fixes (all IMG placeholders replaced with real icon names)
- Paren balance check on every modified file

---

### Chunk 3: Template Migration

**Scope**: Update the 13 heroes that need template changes from Lost/Statue to their new templates (11 new-template heroes + 2 proven-template heroes whose generated files have the wrong template).
**Dependencies**: Chunk 2 (Chunks 2 and 3 share 9 files -- `line_69_torchic.txt`, `line_97_beldum.txt`, `line_79_bulbasaur.txt`, `line_89_cleffa.txt`, `line_93_pikachu.txt`, `line_71_treecko.txt`, `line_27_chikorita.txt`, `line_39_togepi.txt`, `line_new_poliwag.txt` -- so Chunk 3 must run after Chunk 2 to avoid conflicting edits on the same files)
**Concern**: Template string replacement in generated files

**Files** (modify):
1. `generated/line_61_mudkip.txt` -- `replica.Statue` -> `replica.Guardian`
2. `generated/line_79_bulbasaur.txt` -- `replica.Statue` -> `replica.Guardian`
3. `generated/line_73_wailmer.txt` -- `replica.Lost` -> `replica.Stalwart`
4. `generated/line_39_togepi.txt` -- `replica.Statue` -> `replica.Dancer`
5. `generated/line_89_cleffa.txt` -- `replica.Lost` -> `replica.Fencer`

**Files** (modify, continued):
6. `generated/line_97_beldum.txt` -- `replica.Lost` -> `replica.Alloy`
7. `generated/line_new_machop.txt` -- `replica.Lost` -> `replica.Fighter`
8. `generated/line_new_totodile.txt` -- `replica.Lost` -> `replica.Fighter`
9. `generated/line_new_poliwag.txt` -- `replica.Statue` -> `replica.Fighter`
10. `generated/line_69_torchic.txt` -- `replica.Lost` -> `replica.Thief`
11. `generated/line_93_pikachu.txt` -- `replica.Lost` -> `replica.Thief`
12. `generated/line_71_treecko.txt` -- `replica.Lost` -> `Primrose`
13. `generated/line_27_chikorita.txt` -- `replica.Lost` -> `replica.Healer`

**NOTE**: 13 files, same justification as Chunk 2 -- every change is the identical mechanical pattern (find-replace template name). Single concern. Files 12-13 are proven-template heroes whose generated files currently use `replica.Lost` instead of their FULL_ROSTER-specified templates.

**Source of truth**: Section 1A and 1B template assignments in this plan; `plans/FULL_ROSTER.md` Heroes table

**Requirements**:
- For each file, replace ALL occurrences of the old `replica.TEMPLATE` with the new one (templates appear once per tier, so 5 replacements per file typically)
- Do NOT change any other property
- Snorunt (Chunk 4) and Aron (Chunk 5) are handled separately -- do NOT touch those here
- If Chunk 1 template research revealed any built-in abilitydata or keywords that need clearing, apply those overrides here too

**Verification**:
- [ ] Each of the 13 files uses the correct new template per the plan
- [ ] No file still contains the old template name
- [ ] Paren balance = 0 on every file
- [ ] Run `node tools/validate_textmod.js` on each

**If blocked**: If a file's template string has an unexpected format (e.g., `Replica.Lost` with capital R), search case-insensitively.

**Test cases**:
- Grep `generated/` for `replica.Lost` and `replica.Statue` -- only files NOT in this chunk's list should still have them (Larvitar=Statue, Charmander=Statue, Cyndaquil=Lost, Bagon=Statue, NidoranF=Statue, NidoranM=Lost, Weedle=Lost, Dratini=Lost, Riolu=Lost)
- Count template occurrences per file -- should be 5 (one per tier: T1, T2a, T2b, T3a, T3b)

---

### Chunk 4: Snorunt Design (New Hero) [PARALLEL GROUP A]

**Scope**: Create the full Snorunt->Glalie/Froslass hero -- the ONLY hero not yet generated.
**Dependencies**: None (can run in parallel with Chunk 2)
**Concern**: New hero generation
**Parallel with**: Chunk 2

**Files** (create):
1. `tools/hero_configs/snorunt.json` -- hero config for generate_hero.js
2. `generated/line_15_snorunt.txt` -- generated hero line

**Files** (read):
3. `tools/generate_hero.js` -- generator tool
4. `tools/sprite_encodings.json` -- sprite data (must contain "Snorunt", "Glalie", "Froslass")

**Source of truth**: Section 1A hero #1 (Snorunt -> Glalie/Froslass) specifications in this plan (search for "#### 1. Snorunt")

**Requirements**:
- Build config JSON matching the Snorunt spec: Eccentric template, color B, 5 tiers (T1 Snorunt, T2a, T2b, T3a Glalie, T3b Froslass)
- T3a Glalie has Sheer Cold spell (already validated OK)
- T3b Froslass has Ice Beam spell (already validated OK)
- Use `replica.Eccentric` as template (pending Chunk 1 research -- if Eccentric has problematic defaults, flag at checkpoint)
- Run `node tools/generate_hero.js tools/hero_configs/snorunt.json > generated/line_15_snorunt.txt`
- Validate paren balance and structure

**Verification**:
- [ ] Generated file exists and is non-empty
- [ ] Paren balance = 0
- [ ] Contains `.mn.Snorunt@2`
- [ ] Contains all 5 tier names: Snorunt (x3), Glalie, Froslass
- [ ] Both spells present in abilitydata strings
- [ ] Sprite encodings resolve for all 3 Pokemon names

**If blocked**: If "Snorunt", "Glalie", or "Froslass" are missing from `sprite_encodings.json`, run `node tools/encode_sprite.js` to add them first.

**Test cases**:
- `node tools/validate_textmod.js` passes on the generated line
- The line starts with `hidden&temporary&ph.b` and ends with `,`
- No bare `.k.` without `.i.` prefix

---

### Chunk 5: Aron Design + L63 Compound Line Surgery [CRITICAL CHECKPOINT]

**Scope**: Create the Aron->Aggron hero AND perform surgery on the compound Line 63 to replace Lillipup with Aron and remove the Arceus capture data.
**Dependencies**: Chunk 1 (need Stalwart template properties)
**Concern**: New hero generation + compound line editing

**Files** (create):
1. `tools/hero_configs/aron.json` -- hero config
2. `generated/line_63_aron.txt` -- generated Aron hero data (just the hero portion)

**Files** (modify):
3. `textmod.txt` line 63 -- surgical replacement (OR handled via rebuild_textmod.js)

**Files** (read):
4. `tools/generate_hero.js`
5. `tools/sprite_encodings.json` (must contain "Aron", "Lairon", "Aggron")

**Source of truth**: Section 1A hero #4 (Aron -> Aggron) specifications in this plan (search for "#### 4. Aron")

**Requirements**:
- Build config JSON for Aron: Stalwart template, color H, 5 tiers
- T3a and T3b Aggron have Iron Tail spell (already validated OK -- face 15 and 76 valid for Statue)
- Generate the Aron hero line with `generate_hero.js`
- **L63 SURGERY**: Line 63 is the compound line (10,412 chars) containing: Lillipup hero data + item data + Caterpie/Butterfree capture + Arceus capture. Must:
  - Remove Lillipup hero data (everything from `ph.b` to the closing `.mn.Lillipup@2!m(skip&hidden&temporary),`)
  - Insert Aron hero data in the same position
  - Remove Arceus capture data (Arceus becomes a boss per FULL_ROSTER)
  - Preserve ALL item data and Caterpie/Butterfree capture data
- **WARNING**: Arceus capture removal on L63 overlaps with Chunk 10 (Capture Changes). This chunk handles the L63-specific Arceus removal. Chunk 10 handles L111 Arceus removal and all other captures.

**Verification**:
- [ ] Aron hero line generated with paren balance = 0
- [ ] L63 still has valid structure after surgery
- [ ] Lillipup name no longer appears on L63
- [ ] Arceus capture no longer on L63
- [ ] Caterpie/Butterfree capture data preserved on L63
- [ ] Item data preserved on L63
- [ ] `.mn.Aron@2` present on L63 (or wherever Aron is placed)

**If blocked**: If L63 structure is too complex to parse programmatically, extract Lillipup boundaries manually by finding the `ph.b` marker and `.mn.Lillipup@2` marker. If Arceus capture boundaries are unclear, flag for user review at checkpoint.

**Test cases**:
- Paren balance of entire L63 = 0
- No `Lillipup` string remains on L63
- No `Arceus` string remains on L63 (capture removed)
- `Caterpie` or `Butterfree` string still present on L63
- Aron hero data validates independently

---

### Chunk 6: Template Isolation Tests [GATE CHECKPOINT]

**Scope**: Test each of the 7 new templates in complete isolation before mass-deploying.
**Dependencies**: Chunks 1, 2, 3, 4, 5 all complete
**Concern**: Template validation

**Files** (create -- NOTE: `tests/` directory does not exist yet, create it first):
1. `tests/template_test_fighter.txt` -- minimal textmod with one Fighter hero
2. `tests/template_test_stalwart.txt` -- minimal textmod with one Stalwart hero
3. `tests/template_test_guardian.txt` -- minimal textmod with one Guardian hero
4. `tests/template_test_dancer.txt` -- minimal textmod with one Dancer hero
5. `tests/template_test_eccentric_fencer_alloy.txt` -- remaining 3 templates

**Files** (read):
- `textmod.txt` (base for minimal test mods)
- Generated files from Chunks 3-5

**Source of truth**: Phase 0 Testing Plan in this document (search for "### Phase 0: Template Isolation Tests")

**Requirements**:
- For each template, create a minimal textmod containing the original mod PLUS one hero using that template
- Use the simplest hero for each template (no-spell heroes preferred: Machop for Fighter, Wailmer for Stalwart, Mudkip for Guardian, etc.)
- User pastes each test mod into the game and reports: hero appears in draft, die faces display, HP correct, no garbled name, no inherited spells
- If a template fails, document the failure and determine if the template defaults need explicit overrides

**Verification**:
- [ ] All 7 test mods created
- [ ] Each test mod contains exactly 1 new-template hero plus all original content
- [ ] User confirms each template works in-game (REQUIRES USER INPUT)

**If blocked**: If user cannot test immediately, proceed to Chunks 7-8 with a warning that template failures may require rework. Mark as provisional.

**Test cases**:
- Each test textmod passes `node tools/validate_textmod.js`
- Each test textmod has correct line count (original + 0 or +1 lines)
- User in-game verification (manual)

---

### Chunk 7: Proven-Template Hero Assembly + Line 11 Sort

**Scope**: Assemble all 14 proven-template heroes into the textmod and fix Line 11 alphabetical ordering.
**Dependencies**: Chunk 6 (template tests pass)
**Concern**: Assembly + character selection ordering
**Parallel with**: Chunk 9

**Files** (modify):
1. `textmod_heroes_only.txt` -- output textmod
2. `tools/rebuild_textmod.js` -- update REPLACEMENTS map and NEW_LINES array if needed

**Files** (read):
3. All 14 proven-template generated files in `generated/`
4. `textmod.txt` (original base)

**Source of truth**: `plans/FULL_ROSTER.md` Heroes table; `plans/OVERHAUL_NOTES.md` (Line 11 ordering)

**Requirements**:
- Run `node tools/rebuild_textmod.js` to assemble the textmod with all proven-template heroes
- **LINE 11 FIX** (from OVERHAUL_NOTES.md): The character selection screen (Line 11) must show hero colors in alphabetical order (A, B, C, ..., X, Y, Z). Currently new colors E/F and J are appended at the end instead of sorted. Fix the rebuild_textmod.js assembler to sort Line 11 entries by color letter across all three pick rounds (party, add, add).
- Verify all 14 heroes appear in the output
- Verify line count is correct

**Verification**:
- [ ] All 14 proven-template heroes present in output
- [ ] Line 11 has colors in alphabetical order for all pick rounds
- [ ] `node tools/validate_textmod.js textmod_heroes_only.txt` passes
- [ ] No proven-template hero has errors
- [ ] Turtwig correctly at col.m (moved from col.n)

**If blocked**: If rebuild_textmod.js doesn't handle Line 11 sorting, implement the sort manually: parse the pick entries, sort by color letter, reassemble.

**Test cases**:
- Grep Line 11 for color sequence -- must be alphabetical
- Count `.mn.` entries in output -- should match expected hero count
- All hero lines have paren balance = 0

---

### Chunk 8: New-Template Hero Assembly

**Scope**: Add all 11 new-template heroes to the textmod assembled in Chunk 7.
**Dependencies**: Chunk 7 (need the base textmod with proven heroes)
**Concern**: Assembly of new-template heroes

**Files** (modify):
1. `textmod_heroes_only.txt` -- add new-template heroes
2. `tools/rebuild_textmod.js` -- update to include new-template heroes

**Files** (read):
3. All 11 new-template generated files in `generated/`
4. `generated/line_15_snorunt.txt` (from Chunk 4)
5. `generated/line_63_aron.txt` or modified L63 (from Chunk 5)

**Source of truth**: `plans/FULL_ROSTER.md` Heroes table

**Requirements**:
- Add all 11 new-template heroes to the textmod
- Update Line 11 character selection to include all 25 colors in alphabetical order
- **DITTO LINE UPDATE (Line 99)**: Remove T3 form copies for all 12 removed heroes (Vanillite, Varoom, Lillipup, Roggenrola, Trubbish, Tinkatink, Darumaka, Tentomon, Rockruff, Slugma, Fomantis, Applin and all their evolutions). Add T3 form copies for all new heroes. This line is 66K chars -- use programmatic search/replace, not manual editing. The rebuild_textmod.js tool should handle this if properly configured.
- Verify line count, hero count, paren balance

**Verification**:
- [ ] All 25 hero colors present in Line 11 (A through Z, minus unused letters)
- [ ] All 25 hero lines present in the textmod body
- [ ] `node tools/validate_textmod.js textmod_heroes_only.txt` passes with 0 errors
- [ ] Total hero count matches FULL_ROSTER (25 colors x 2 heroes = ~46 hero lines, some on same line)
- [ ] Ditto line (Line 99) contains T3 copies for all new heroes and none for removed heroes

**If blocked**: If any new-template hero causes validation failures, isolate it by adding heroes one at a time.

**Test cases**:
- Hero count in output matches FULL_ROSTER count
- Every `.mn.NAME` in the output corresponds to a name in FULL_ROSTER
- No duplicate `.mn.` names
- User in-game paste test (REQUIRES USER INPUT)

---

### Chunk 9: Monster Changes [PARALLEL WITH CHUNK 10]

**Scope**: Rename Aggron monster to Probopass.
**Dependencies**: Chunk 7 (Chunk 9 modifies `textmod_heroes_only.txt` which Chunk 7 creates. Monster changes are independent of hero templates -- the dependency is on the output file existing, not on template validation.)
**Concern**: Monster renaming
**Parallel with**: Chunk 7

**Files** (modify):
1. `textmod_heroes_only.txt` -- monster pool lines containing `.n.Aggron` (originally Lines 121 and 123 in textmod.txt, but line numbers shift after hero additions -- use content-based search, NOT hardcoded line numbers)

**Files** (read/create):
2. `tools/sprite_encodings.json` -- needs Probopass sprite encoding
3. `tools/batch_sprites.js` -- add Probopass to POKEMON_LIST if missing

**Source of truth**: Part 2 (Monster Changes) of this plan; `plans/FULL_ROSTER.md` Monsters table

**Requirements**:
- Add "Probopass" to `tools/batch_sprites.js` POKEMON_LIST if not present
- Run sprite download and encoding for Probopass
- **IMPORTANT**: Do NOT use hardcoded line numbers. Grep for `.n.Aggron` in the monster pool section of `textmod_heroes_only.txt` to find the correct lines (they were originally L121 and L123 in textmod.txt but shift after hero additions in Chunks 7-8)
- On the found lines, replace `.n.Aggron` with `.n.Probopass`
- On the found lines, replace Aggron's `.img.` sprite with Probopass's `.img.` sprite
- Alakazam monster: NO CHANGE needed (already confirmed in plan)

**Verification**:
- [ ] `.n.Probopass` appears on the two monster pool lines (grep to find)
- [ ] `.n.Aggron` does NOT appear on monster lines (only on hero line L63)
- [ ] Probopass sprite encoding exists in sprite_encodings.json
- [ ] Paren balance unchanged on modified lines

**If blocked**: If Probopass sprite cannot be downloaded/encoded, use Aggron sprite as placeholder and flag for user.

**Test cases**:
- Grep for `.n.Aggron` -- should appear ONLY in hero context (L63), never in monster context
- Grep for `.n.Probopass` -- should appear exactly twice (L121, L123)

---

### Chunk 10: Capture Changes

**Scope**: Apply all capture removals, upgrades, replacements, and additions.
**Dependencies**: Chunk 8 (need assembled textmod with all heroes). Parallel with Chunk 9 (monster changes modify different lines of `textmod_heroes_only.txt`).
**Concern**: Capture pool modifications

**Files** (modify):
1. `textmod_heroes_only.txt` -- capture pool line (originally L111 in textmod.txt, but line numbers shift after hero additions -- use content-based search for the capture/ball item section)
2. `tools/sprite_encodings.json` -- new Pokemon sprites needed

**Files** (read):
3. `plans/FULL_ROSTER.md` Captures table
4. `tools/batch_sprites.js`

**Source of truth**: Part 3 (Capture Changes) of this plan; `plans/FULL_ROSTER.md` Captures table

**Requirements**:
- **IMPORTANT**: Do NOT use hardcoded line numbers. The capture pool was originally on L111 in textmod.txt but shifts after hero additions. Search for the capture/ball section by content (e.g., grep for existing capture names like `.n.Snorlax` or ball item markers).
- **Removals**: Remove Pikachu, Charizard, Metagross, Poliwag, Zubat, Arceus, Ivysaur captures (these Pokemon are now heroes/bosses/hero evolutions)
- **NOTE**: Arceus removal from L63 was already handled in Chunk 5. This chunk removes Arceus from the capture pool line only.
- **Upgrades**: Barboach->Whiscash (keep Dive Ball), Caterpie->Butterfree (keep Nest Ball), Sneasel->Weavile (keep Fast Ball), Electrike->Manectric (keep Quick Ball)
- **Replacements**: Rattata->Skarmory (Level Ball), Furret->Lapras (Friend Ball), Alcremie->Arcanine (Premier Ball)
- **Additions**: Mew (Poke Ball), Jirachi (Great Ball), Kangaskhan (Safari Ball), Heracross (Net Ball), Greninja (Dusk Ball), Electivire (Ultra Ball), Magmortar (Lure Ball), Rhyperior (Heavy Ball)
- **Legendary items** (Part 3D): Add Latias/Soul Dew, Latios/Eon Flute, Suicune/Clear Bell, Entei/Flame Plate, Raikou/Zap Plate, Rayquaza/Jade Orb
- Download and encode sprites for ALL new Pokemon

**Verification**:
- [ ] Removed Pokemon do not appear in capture pool
- [ ] Upgraded Pokemon show new names but same ball types
- [ ] New captures appear with correct ball types
- [ ] Legendary items present
- [ ] `node tools/validate_textmod.js` passes
- [ ] No duplicate captures (per mod design rules)

**If blocked**: If capture line format is unclear, study existing capture entries on the capture pool line to understand the pattern before making changes. If sprites are missing for new Pokemon, flag and use placeholder sprites.

**Test cases**:
- Grep capture pool line for removed Pokemon names -- zero matches
- Count total captures -- should match FULL_ROSTER Captures table count
- Each new capture has a valid `.img.` sprite string (not empty or placeholder)

---

### Chunk 11: Full Integration Test

**Scope**: Complete playthrough verification of the full assembled textmod.
**Dependencies**: All previous chunks (1-10)
**Concern**: End-to-end validation

**Files** (read):
1. `textmod_heroes_only.txt` -- final assembled output
2. `plans/FULL_ROSTER.md` -- verification reference

**Files** (create):
3. `TEST_RESULTS.md` -- integration test results log

**Source of truth**: Phase 4 Testing Plan in this document; `plans/FULL_ROSTER.md`

**Requirements**:
- Run `node tools/validate_textmod.js textmod_heroes_only.txt` -- must pass with 0 errors
- User pastes full textmod into game (REQUIRES USER INPUT)
- Verify all 25 color draft options appear in character selection
- Verify character selection is in alphabetical order (Line 11 fix from Chunk 7)
- Verify branching evolutions display correctly (T3a vs T3b)
- Verify spells cast and display correctly
- Verify monsters appear on correct floors
- Verify Probopass appears instead of Aggron in monster encounters
- Verify captures: removed Pokemon absent, new Pokemon present, upgraded Pokemon correct
- Play a complete run on Normal difficulty

**Verification**:
- [ ] Validator passes with 0 errors
- [ ] 0/N modifiers fail on paste (game acceptance)
- [ ] All 25 colors appear in draft
- [ ] Draft order is alphabetical by color
- [ ] At least 5 different heroes tested through full evolution chain
- [ ] At least 2 spells tested (cast and resolve correctly)
- [ ] Probopass encountered as monster (not Aggron)
- [ ] Captures verified (spot-check 5+ captures)

**If blocked**: If paste shows failures, use binary search: split textmod in half, test each half, narrow down to failing lines. Document failures in TEST_RESULTS.md.

**Test cases**:
- `node tools/validate_textmod.js textmod_heroes_only.txt` returns 0 errors
- Modifier failure count on paste = 0
- Full Normal-difficulty run completable

---

## Decisions (All Resolved)

1. **Treecko template**: RESOLVED -- Use `Primrose`. FULL_ROSTER.md, design docs, and user all confirm. Growth mechanic justifies Primrose over Lost. Template migration scheduled in Chunk 3.
2. **Trapinch duplicate**: RESOLVED -- Copy error. Removed standalone row (was line 149) and duplicate Global Roster entry from FULL_ROSTER.md. Trapinch is O P2 with Larvesta as O P1, already correct at line 137.
3. **Metagross spell final form**: RESOLVED -- Using DEFINITIVE version: `abilitydata.(lost.sd.15-5:0:0:0:76-4.i.left.k.focus.n.Meteor Mash.img.mithril shields.hsv.-120:30:70)` (Damage 5 + focus keyword for 4 mana).
4. **Blaziken spell template**: RESOLVED -- Using `Fey` template per DEFINITIVE SPELL REFERENCE: `(Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.inflictpain.img.scorch.hue.10.n.Blaze Kick)`.
5. **Sceptile Leaf Blade spell**: RESOLVED -- User chose option (a). Designed as `(Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.focus.img.spark.hsv.60:0:0.n.Leaf Blade)`. Damage 4 / 3 mana + focus keyword. Focus synergizes with Sceptile's Growth mechanic on Primrose (growing faces crit harder). Added to DEFINITIVE SPELL REFERENCE.
6. **Capture/Legendary dice designs**: RESOLVED -- User chose option (a), design now as prerequisite. All 15 capture designs (4 upgrades, 3 replacements, 8 new) and 6 legendary designs added to Part 3 sections 3C-1 and 3D-1. Chunk 10 can now execute with concrete stat specifications.

---

## Critical Files for Implementation

| File | Purpose | Used In Chunks |
|------|---------|---------------|
| `tools/generate_hero.js` | Hero line generator | 4, 5 |
| `tools/rebuild_textmod.js` | Textmod assembler | 7, 8 |
| `tools/validate_textmod.js` | Validation | 2, 3, 6, 7, 8, 10, 11 |
| `tools/sprite_encodings.json` | Sprite data | 4, 5, 9, 10 |
| `tools/batch_sprites.js` | Sprite downloader | 9, 10 |
| `tools/encode_sprite.js` | Single sprite encoder | 9, 10 |
| `working-mods/pansaer.txt` | Template reference | 1 |
| `textmod.txt` | Original unmodified mod | 5, 6, 7 |
| `textmod_heroes_only.txt` | Output textmod | 7, 8, 9, 10, 11 |
| `plans/FULL_ROSTER.md` | Authoritative roster | All chunks (verification) |
| `plans/OVERHAUL_NOTES.md` | Implementation notes | 7 (Line 11 sort) |
| `plans/SPELL_REDESIGN.md` | Spell design rationale | 2 (reference) |
| `tools/hero_configs/` | Hero config JSONs | 4, 5 |
| `generated/*.txt` | Individual hero lines | 2, 3, 4, 5, 7, 8 |
