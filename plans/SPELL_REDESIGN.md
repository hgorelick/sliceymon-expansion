# Spell Redesign — Rich Mechanics

Based on analysis of all original Sliceymon spells. Each spell now uses keywords, ritemx, stickers, or togfri where thematically appropriate.

## Original Spell Mechanic Reference

| Mechanic | What it does | Example |
|----------|-------------|---------|
| `.i.left.k.weaken` | Adds Weaken debuff to spell | Blizzard |
| `.i.left.k.focus` | Spell can't miss / Focus buff | Psycho Cut |
| `.i.left.k.singleuse` | One-shot powerful effect | Reflect |
| `.i.left.k.inflictpain` | Inflicts Pain on target | Flamethrower |
| `.i.left.k.inflictsingleuse` | Makes target's faces single-use | Spite |
| `.i.left.k.inflictexert` | Exhausts target | Thunder Wave |
| `.i.left.k.vulnerable` | Makes target take more damage | Air Slash |
| `.i.left.k.cleanse` | Removes debuffs | Rain Dance |
| `.i.left.k.damage#k.ranged` | Adds Damage + Ranged keywords | Ice Beam, Hyper Beam |
| `.i.Ritemx.132fb.part.1` | Singlecast modifier | Many spells |
| `.i.unpack.ritemx.644f` | Delayed effect (hits NEXT turn) | Future Sight |
| `.i.left.togfri` | Hits allies too (friendly fire) | Discharge |
| `.i.left.sticker.k.X` | Gives persistent keyword sticker | Cosmic Power, Cultivate |

## Redesigned Spells

### Fire Blast (Charizard) — Statue template
**Identity**: Devastating AoE nuke
```
Statue.sd.34-3:34-2:0:0:76-3.n.Fire Blast.img.spark.hsv.-10:20:80
```
**No change** — already balanced (5 dmg / 3 mana = 1.67). Damage to All faces are thematic. Charizard's spell is straightforward power.

---

### Eruption (Typhlosion) — Statue template
**Identity**: Maximum power one-shot nuke (Eruption = full power at full HP, then gone)
```
Statue.sd.34-4:34-3:0:0:76-4.i.Ritemx.132fb.part.1.n.Eruption.img.spark.hsv.-20:30:90
```
**Fixed**: 7 AoE dmg / 4 mana + **singlecast**. Eruption is a one-shot nuke — you fire it at full power once, then it's spent. Singlecast prevents repeatable 7 AoE from dominating every fight.

---

### Blaze Kick (Blaziken) — Fey template
**Identity**: High Jump Kick — risky, powerful, Speed Boost accelerating
```
Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.inflictpain.img.scorch.hue.10.n.Blaze Kick
```
**Changed**: 4 dmg / 3 mana + **inflictpain** on target. Matches Flamethrower's pattern. Blaziken's kicks burn the opponent. The pain debuff adds value beyond raw damage, justifying the modest 4 damage.

---

### Meteor Mash (Metagross) — lost template
**Identity**: Computed precision strike with self-sustain
```
lost.sd.15-5:0:0:0:76-4.i.left.k.focus.n.Meteor Mash.img.mithril shields.hsv.-120:30:70
```
**Changed**: 5 dmg / 4 mana + **focus** keyword. Meteor Mash in Pokemon has a chance to boost Attack — Focus represents Metagross's calculated precision. Simpler than dmg+heal, more interesting than plain damage. Matches Psycho Cut pattern.

---

### Solar Beam (Venusaur) — Statue template
**Identity**: Charged sun attack — big but needs setup
```
Statue.sd.15-4:0:0:0:76-4.i.left.k.vulnerable.n.Solar Beam.img.beam.hsv.40:0:0
```
**Changed**: 4 dmg / 4 mana + **vulnerable** on target. Solar Beam lowers the enemy's defenses for the rest of the fight. The vulnerability makes follow-up attacks from the whole team hit harder. Matches Air Slash's pattern.

---

### Aromatherapy (Meganium) — Fey template
**Identity**: Team status cleanse + heal
```
Fey.sd.111-3:0-0:0-0:0-0:76-3.i.left.k.cleanse.img.sprout.hsv.40:0:0.n.Aromatherapy
```
**Changed**: Heal Cleanse 3 / 3 mana + **cleanse** keyword. The face itself is already Heal Cleanse (111), and the keyword doubles down — Aromatherapy IS cleansing. Removed the second heal face for cleaner design. Matches Rain Dance's cleanse pattern.

---

### Earth Power (Nidoqueen) — Fey template
**Identity**: Ground eruption that weakens defenses
```
Fey.sd.15-4:0-0:0-0:0-0:76-4.i.left.k.weaken.img.spark.hsv.20:30:0.n.Earth Power
```
**Changed**: 4 dmg / 4 mana + **weaken** debuff. Earth Power in Pokemon lowers SpDef — Weaken is the S&D equivalent. Matches Blizzard's pattern.

---

### Sheer Cold (Glalie) — Fey template
**Identity**: One-hit KO attempt — Glalie's brute force finisher
```
Fey.sd.34-5:0-0:0-0:0-0:76-5.i.Ritemx.132fb.part.1.i.left.k.weaken.img.light.hsv.-120:0:40.n.Sheer Cold
```
**Fixed**: AoE 5 dmg / 5 mana + **singlecast** + **weaken**. Raised damage from 4 to 5 to differentiate from Thunder (AoE 4/4 mana). Sheer Cold is the more expensive, more devastating version — Glalie's brute force identity demands a bigger hit. 5 AoE + weaken for 5 mana singlecast is a true fight-ender.

---

### Ice Beam (Froslass) — Fey template
**Identity**: Freeze ray — locks down an enemy once per fight
```
Fey.sd.131-3:0-0:0-0:0-0:76-3.i.Ritemx.132fb.part.1.i.left.k.inflictsingleuse.img.light.hsv.-120:0:40.n.Ice Beam
```
**Fixed**: Weaken 3 / 3 mana + **singlecast** + **inflictsingleuse**. Added singlecast to match Spite's pattern — a double-debuff spell this powerful must be one-shot. The freeze locks one enemy down hard, but you only get one shot at it.

---

### Wish (Togekiss) — Fey template
**Identity**: Gentle team heal
```
Fey.sd.107-3:0-0:0-0:0-0:76-3.img.light.n.Wish
```
**Fixed**: Heal All 3 / 3 mana, no extras. Removed Ritemx.62e8 — Heal All 3 for 3 mana is already the best pure healing spell. Clean and fair without the sustain modifier stacking additional value.

---

### Dazzling Gleam (Togekiss) — Fey template
**Identity**: Fairy AoE that dazzles enemies
```
Fey.sd.34-3:0-0:0-0:0-0:76-4.i.left.k.vulnerable.img.light.n.Dazzling Gleam
```
**Changed**: DmgAll 3 / 4 mana + **vulnerable**. Dazzling = blinding = vulnerable. The AoE + vulnerability combo punishes grouped enemies. Slightly expensive for the damage but the debuff adds team value.

---

### Metronome (Clefable) — Fey template
**Identity**: Random powerful effect (Metronome calls any move)
```
Fey.sd.136-2:0-0:0-0:0-0:76-4.i.Ritemx.132fb.part.1.img.light.n.Metronome
```
**Fixed**: Revive 2 / 4 mana + **singlecast**. Reduced from Revive 3/3 mana to Revive 2/4 mana. Rejuvenate (the precedent) does Revive 1/3 mana singlecast — Metronome at Revive 2/4 mana is a meaningful upgrade but not game-breaking. Revive is Premium tier; must be expensive.

---

### Moonlight (Clefable) — Fey template
**Identity**: Purifying moonlight heal
```
Fey.sd.107-3:0-0:0-0:0-0:76-3.i.left.k.cleanse.img.light.n.Moonlight
```
**Fixed**: Heal All 3 / 3 mana + **cleanse**. Raised mana from 2 to 3. Heal All 3 + cleanse for 2 mana was strictly better than every heal spell in the mod. At 3 mana it's still excellent — best repeatable team heal+cleanse — but doesn't embarrass Life Dew or Slack Off.

---

### Thunderbolt (Raichu) — Fey template
**Identity**: Reliable paralysis strike — one precise shock
```
Fey.sd.15-3:0-0:0-0:0-0:76-3.i.Ritemx.132fb.part.1.i.left.k.inflictexert.img.spark.hsv.-60:0:40.n.Thunderbolt
```
**Fixed**: 3 dmg / 3 mana + **singlecast** + **inflictexert**. Added singlecast — repeatable damage + exert for 3 mana was strictly better than Thunder Wave (pure exert/3 mana, singlecast). Now it's a one-shot paralysis strike: deal damage AND lock the target down, but only once.

---

### Thunder (Raichu) — Fey template
**Identity**: Massive unreliable thunder strike
```
Fey.sd.34-4:0-0:0-0:0-0:76-4.i.Ritemx.132fb.part.1.i.left.k.weaken.img.spark.hsv.-60:0:40.n.Thunder
```
**Changed**: DmgAll 4 / 4 mana + **singlecast** + **weaken**. Thunder is powerful but inaccurate (70% in Pokemon) — singlecast represents you only get one shot. AoE + weaken is devastating when it lands. Similar to Sheer Cold pattern.

---

### Dragon Dance (Dragonite) — lost template
**Identity**: Setup buff that makes you faster and stronger
```
lost.i.left.sticker.k.focus.sd.0:0:0:0:76-4.n.Dragon Dance.img.spark.hsv.30:40:80
```
**Fixed**: Pure Focus sticker buff / 4 mana. Raised from 3 to 4 mana to match Psych Up (identical effect, 4 mana). Same effect = same cost. Dragon Dance is a SETUP move, not a damage spell — the value comes from empowering all future attacks.

---

### Outrage (Dragonite) — lost template
**Identity**: Uncontrollable berserker rampage
```
lost.sd.36-5:0:0:0:76-4.n.Outrage.img.scorch.hue.40
```
**Changed**: Cleave 5 / 4 mana. Big single-target damage that cleaves to adjacent enemies. Simple, devastating, expensive. Outrage is all about raw uncontrollable power — no keywords needed, just massive Cleave.

---

### Aura Sphere (Lucario) — Fey template
**Identity**: Never-miss fighting spirit projectile
```
Fey.sd.15-4:0-0:0-0:0-0:76-3.i.left.k.focus.img.light.hsv.-120:20:0.n.Aura Sphere
```
**Changed**: 4 dmg / 3 mana + **focus**. Aura Sphere NEVER misses in Pokemon — Focus is the perfect translation. Cheap and reliable, matching Lucario's precision warrior identity. Same pattern as Psycho Cut.

---

### Close Combat (Lucario) — Fey template
**Identity**: All-out attack that lowers your own defenses
```
Fey.sd.15-5:0-0:0-0:0-0:76-4.i.left.k.vulnerable.i.left.togfri.img.spark.hsv.-120:20:0.n.Close Combat
```
**Changed**: 5 dmg / 4 mana + **vulnerable** + **togfri** (hits allies). Close Combat lowers your own DEF/SpDef — togfri means the vulnerability hits YOUR team too. High risk, high reward. The self-debuff is the "cost" beyond mana.

---

### Iron Tail (Aggron) — Statue template
**Identity**: Computed precision iron strike
```
Statue.sd.15-4:0:0:0:76-3.i.left.k.focus.n.Iron Tail.img.mithril shields.hsv.0:0:0
```
**Fixed**: 4 dmg / 3 mana + **focus**. Added focus keyword — Aggron's Steel computing power ensures the hit lands. Now matches Aura Sphere's value tier (4 dmg / 3 mana + focus) rather than being strictly worse. Aggron's spell is reliable, not flashy.

---

### Rain Dance (Politoed) — Fey template
**Identity**: Weather support — heals and cleanses the team
```
Fey.sd.107-2:0-0:0-0:0-0:76-3.i.Ritemx.62e8.i.left.k.cleanse.img.flick.hsv.-50:-30:0.n.Rain Dance
```
**Changed**: Heal All 2 / 3 mana + **Ritemx.62e8** (sustain modifier) + **cleanse**. Matches the ORIGINAL Rain Dance spell from Goodra in the base Sliceymon! Rain washes away status and heals the team. Ritemx adds growth/sustain value over time.

---

## Balance Summary (final, post-review)

| Spell | Effect | Mana | Special Mechanic | Notes |
|-------|--------|------|-----------------|-------|
| Fire Blast | 5 AoE dmg | 3 | — | Repeatable AoE nuke |
| Eruption | 7 AoE dmg | 4 | singlecast | One-shot mega nuke |
| Blaze Kick | 4 dmg | 3 | inflictpain | Repeatable burn strike |
| Meteor Mash | 5 dmg | 4 | focus | Reliable precision hit |
| Solar Beam | 4 dmg | 4 | vulnerable | Softens target for team |
| Aromatherapy | 3 heal cleanse | 3 | cleanse keyword | Team purify + heal |
| Earth Power | 4 dmg | 4 | weaken | Defensive debuff |
| Sheer Cold | 5 AoE dmg | 5 | singlecast + weaken | Fight-ending nuke |
| Ice Beam | 3 weaken | 3 | singlecast + inflictsingleuse | Freeze lock one-shot |
| Wish | 3 heal all | 3 | — | Clean team heal |
| Dazzling Gleam | 3 AoE dmg | 4 | vulnerable | AoE + team amplify |
| Metronome | 2 revive | 4 | singlecast | Emergency clutch revive |
| Moonlight | 3 heal all | 3 | cleanse | Repeatable team purify |
| Thunderbolt | 3 dmg | 3 | singlecast + inflictexert | Paralysis one-shot |
| Thunder | 4 AoE dmg | 4 | singlecast + weaken | Devastating storm |
| Dragon Dance | 0 (buff) | 4 | focus sticker | Setup for sweep |
| Outrage | 5 cleave | 4 | — | Raw power cleave |
| Aura Sphere | 4 dmg | 3 | focus | Never-miss precision |
| Close Combat | 5 dmg | 4 | vulnerable + togfri | High risk high reward |
| Iron Tail | 4 dmg | 3 | focus | Reliable iron strike |
| Rain Dance | 2 heal all | 3 | Ritemx.62e8 + cleanse | Sustain + purify |

All spells reviewed and balanced by slice-and-dice-design persona. Fixes applied:
- Eruption: added singlecast (was repeatable 7 AoE)
- Ice Beam: added singlecast (was strictly better than Spite)
- Wish: removed Ritemx.62e8 (was best heal by far)
- Metronome: reduced to Revive 2 / 4 mana (was Revive 3 / 3 mana)
- Moonlight: raised mana to 3 (was 2, embarrassed all other heals)
- Thunderbolt: added singlecast (was strictly better than Thunder Wave)
- Dragon Dance: raised mana to 4 (was 3, cheaper than identical Psych Up)
- Sheer Cold: raised damage to 5 (was identical to Thunder but more expensive)
- Iron Tail: added focus keyword (was strictly worse than Aura Sphere)
