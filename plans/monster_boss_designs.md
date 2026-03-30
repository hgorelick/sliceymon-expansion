# Monster & Boss Dice Designs — Sliceymon Expansion

## Reference: Face ID Key

| Face ID | Name | Notes |
|---------|------|-------|
| 0 | Blank | Empty face |
| 118 | Enemy Shield | Self-shield for monsters |
| 158 | Enemy Heal | Self-heal for monsters |
| 170 | Enemy Damage | Single-target damage (replaces hero face 15) |
| 171 | Enemy Cleave | Damage to all heroes (replaces hero face 36) |
| 43 | Special Blank | Used as filler/empty |
| 123 | Lock | Locks a face |

## Reference: Template Dice Counts

| Template | Dice Faces | Used For |
|----------|-----------|----------|
| Wolf | 4 or 6 | Standard monsters (most common) |
| Slimelet | 4 | Small/weak monsters, minions |
| Gnoll | 6 | Tanky monsters (no default sd, uses items) |
| Sarcophagus | 6 | Heavy tanks/shields |
| Troll | 6 | Large monsters |
| Alpha | 6 | Bosses |
| Dragon | 6 | Major bosses |
| Basalt | 6 | Endgame bosses |

## Reference: Keyword Application

- Keywords on monsters: `.i.k.keyword` (e.g., `.i.k.cruel`, `.i.k.pain`, `.i.k.exert`)
- Status effect infliction: `.i.k.keyword` or `.facade.` overrides
- Facade format: `.facade.bas[faceID]:[pip]:[modifier]:[modifier]`
- Facade for enemy shield: `.facade.Ese[faceID]:[pip]`
- Item slots: `.i.left2.`, `.i.topbot.`, `.i.right2.` for face-pair overrides
- Custom text: `#sidesc.[description text]`

---

## PART 1: Regular Monsters

---

### Zubat (Floor 1-3, HP 3)

- **Template base:** Wolf (4 faces)
- **Concept:** Poison bat + Confuse Ray (weaken). Annoying debuffer.
- **sd:** `170-1:170-1:0:0`
- **Items/keywords:**
  - `.i.k.exert` (Confuse Ray — forces hero exertion)
  - `.i.topbot.facade.bas170:0:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]` (facade on blank faces to inflict weaken when rolled)
- **Doc:** `.doc.Screeches in the Dark`
- **Notes:** Low damage, but weaken + exert disrupts heroes. At HP 3 dies fast but annoys while alive.

**Full definition string:**
```
Wolf.n.Zubat.hp.3.doc.Screeches in the Dark.sd.170-1:170-1:0:0.i.k.exert.i.topbot.facade.bas170:0:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].img.PLACEHOLDER
```

---

### Tentacool (Floor 1-3, HP 4)

- **Template base:** Wolf (4 faces)
- **Concept:** Poison + basic damage. Steady poison applicator.
- **sd:** `170-2:170-2:170-1:170-1`
- **Items/keywords:**
  - `.i.k.pain` (Poison Sting — deals 1 extra damage over time via pain)
  - `.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict poison[nokeyword]` (top faces apply poison via facade)
- **Doc:** `.doc.Stings on Contact`
- **Notes:** Reliable 1-2 damage per turn with poison rider. Slightly tankier than Zubat at HP 4.

**Full definition string:**
```
Wolf.n.Tentacool.hp.4.doc.Stings on Contact.sd.170-2:170-2:170-1:170-1.i.k.pain.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict poison[nokeyword].img.PLACEHOLDER
```

---

### Carvanha (Floor 1-3, HP 3)

- **Template base:** Wolf (4 faces)
- **Concept:** Glass cannon. High damage, low HP, dies fast.
- **sd:** `170-2:170-2:170-2:170-2`
- **Items/keywords:**
  - `.i.k.cruel` (Rough Skin — cruel forces heroes to take extra punishment)
- **Doc:** `.doc.Bites First, Thinks Never`
- **Notes:** Every face does 2 damage. With cruel keyword, a terrifying glass cannon at HP 3. Must be prioritized.

**Full definition string:**
```
Wolf.n.Carvanha.hp.3.doc.Bites First, Thinks Never.sd.170-2:170-2:170-2:170-2.i.k.cruel.img.PLACEHOLDER
```

---

### Chinchou (Floor 1-3, HP 4)

- **Template base:** Wolf (4 faces)
- **Concept:** Damage + small self-heal. Slightly durable.
- **sd:** `170-1:170-1:158-1:158-1`
- **Items/keywords:** (none — simple monster)
- **Doc:** `.doc.Glows Ominously`
- **Notes:** 2 damage faces, 2 heal faces. At HP 4, the self-heal makes it annoying to kill but low threat. Teaches new players to focus-fire.

**Full definition string:**
```
Wolf.n.Chinchou.hp.4.doc.Glows Ominously.sd.170-1:170-1:158-1:158-1.img.PLACEHOLDER
```

---

### Golbat (Floor 9-11, HP 6)

- **Template base:** Wolf (6 faces)
- **Concept:** Evolved Zubat. Poison + Confuse Ray (inflict-exert) + drain (self-heal). Multi-threat.
- **sd:** `170-2:170-2:170-1:170-1:158-1:158-1`
- **Items/keywords:**
  - `.i.k.exert` (Confuse Ray — forces hero exertion)
  - `.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict poison[nokeyword]` (top damage faces apply poison)
- **Doc:** `.doc.Drains the Will to Fight`
- **Notes:** Upgraded Zubat. 2-pip damage + poison on top faces, 1-pip damage on mid, self-heal on bottom. The exert keyword + poison + drain makes Golbat a priority target at Floor 9-11.

**Full definition string:**
```
Wolf.n.Golbat.hp.6.doc.Drains the Will to Fight.sd.170-2:170-2:170-1:170-1:158-1:158-1.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict poison[nokeyword].img.PLACEHOLDER
```

---

### Tentacruel (Floor 9-11, HP 8)

- **Template base:** Wolf (6 faces)
- **Concept:** Damage to All + Poison. AoE threat.
- **sd:** `171-3:171-3:170-2:170-2:170-1:170-1`
- **Items/keywords:**
  - `.i.k.pain` (Toxic — pain keyword for damage over time)
  - `.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict poison[nokeyword]`
- **Doc:** `.doc.Tentacles Lash All`
- **Notes:** Top faces do 3-pip cleave (damage all heroes). Mid faces do 2-pip single target. Bottom faces do 1-pip. With pain keyword, every hit stacks. HP 8 makes it a real threat that needs to die.

**Full definition string:**
```
Wolf.n.Tentacruel.hp.8.doc.Tentacles Lash All.sd.171-3:171-3:170-2:170-2:170-1:170-1.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict poison[nokeyword].img.PLACEHOLDER
```

---

### Sharpedo (Floor 9-11, HP 7)

- **Template base:** Wolf (6 faces)
- **Concept:** Very high Engage damage + Cruel. Pure aggression.
- **sd:** `170-4:170-4:170-3:170-3:170-2:170-2`
- **Items/keywords:**
  - `.i.k.cruel` (Rough Skin — cruel punishes heroes further)
  - `.i.k.first` (Speed Boost — acts before slower heroes)
- **Doc:** `.doc.The Bully of the Sea`
- **Notes:** Highest damage output of any Floor 9-11 monster. Every face deals damage (4/4/3/3/2/2). Cruel + first makes it devastating. HP 7 is its weakness — kill it fast or suffer.

**Full definition string:**
```
Wolf.n.Sharpedo.hp.7.doc.The Bully of the Sea.sd.170-4:170-4:170-3:170-3:170-2:170-2.i.k.cruel.i.k.first.img.PLACEHOLDER
```

---

### Lanturn (Floor 9-11, HP 7)

- **Template base:** Wolf (6 faces)
- **Concept:** Damage + Shield to All monsters. Support healer.
- **sd:** `170-2:170-2:118-2:118-2:158-1:158-1`
- **Items/keywords:**
  - `.i.topbot.facade.Ese118:0` (mid faces grant shield to allies — Ese facade for enemy shield behavior)
- **Doc:** `.doc.Lights the Abyss`
- **Notes:** The support monster. Top faces deal damage, mid faces shield all monsters (via Ese facade), bottom faces self-heal. Makes other monsters harder to kill. Priority target in mixed encounters.

**Full definition string:**
```
Wolf.n.Lanturn.hp.7.doc.Lights the Abyss.sd.170-2:170-2:118-2:118-2:158-1:158-1.i.topbot.facade.Ese118:0.img.PLACEHOLDER
```

---

### Wild Steelix (Floor 9-11, HP 10)

- **Template base:** Sarcophagus (6 faces)
- **Concept:** Steel Shield + Heavy Damage + Repel. Immovable wall.
- **sd:** `170-3:170-3:118-4:118-4:118-3:118-3`
- **Items/keywords:**
  - `.i.k.heavy` (Iron Tail — heavy keyword for extra damage impact, pending facade)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword]` (heavy damage on top faces)
  - `.i.right2.facade.Ese118:0` (repel-style shield on bottom faces — enemies can't be targeted easily)
- **Doc:** `.doc.An Iron Serpent Blocks the Path`
- **Notes:** HP 10 tank. 4 shield faces + 2 heavy damage faces. The repel-shields force heroes to deal with it slowly. Designed as a wall that slows the fight, not a damage race.

**Full definition string:**
```
Sarcophagus.n.Wild Steelix.hp.10.doc.An Iron Serpent Blocks the Path.sd.170-3:170-3:118-4:118-4:118-3:118-3.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.right2.facade.Ese118:0.img.PLACEHOLDER
```

---

### Crobat (Floor 17-19, HP 9)

- **Template base:** Wolf (6 faces)
- **Concept:** Poison Plague + Confuse Ray + high damage. Elite debuffer.
- **sd:** `170-4:170-4:170-3:170-3:171-2:171-2`
- **Items/keywords:**
  - `.i.k.exert` (Confuse Ray — forces hero exertion)
  - `.i.k.pain` (Poison — damage over time)
  - `.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict poison[nokeyword]` (top faces apply poison)
  - `.i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword]` (bottom cleave faces apply weaken)
- **Doc:** `.doc.Silent Wings, Venomous Fangs`
- **Notes:** The ultimate bat. 4-pip poison damage on top, 3-pip single-target mid, 2-pip cleave with weaken on bottom. Exert + pain + poison + weaken = devastating if left alive. HP 9 is manageable but not trivial.

**Full definition string:**
```
Wolf.n.Crobat.hp.9.doc.Silent Wings, Venomous Fangs.sd.170-4:170-4:170-3:170-3:171-2:171-2.i.k.exert.i.k.pain.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict poison[nokeyword].i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword].img.PLACEHOLDER
```

---

### Elite Steelix (Floor 17-19, HP 12)

- **Template base:** Sarcophagus (6 faces)
- **Concept:** Enhanced Steel Shield + Heavy Damage + Repel. Fortress.
- **sd:** `170-5:170-5:118-5:118-5:118-4:118-4`
- **Items/keywords:**
  - `.i.k.heavy` (Iron Tail — heavy damage)
  - `.i.k.stasis` (Iron Defense — stasis makes it even harder to crack)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword]`
  - `.i.right2.facade.Ese118:0`
- **Doc:** `.doc.The Mountain That Moves`
- **Notes:** HP 12 fortress. Even more shield than Wild Steelix with stasis making it resist control effects. The 5-pip damage faces punish ignoring it. A true endgame wall.

**Full definition string:**
```
Sarcophagus.n.Elite Steelix.hp.12.doc.The Mountain That Moves.sd.170-5:170-5:118-5:118-5:118-4:118-4.i.k.stasis.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.right2.facade.Ese118:0.img.PLACEHOLDER
```

---

### Absol (Floor 17-19, HP 8)

- **Template base:** Wolf (6 faces)
- **Concept:** Critical damage + Cruel. Elite assassin. Glass cannon.
- **sd:** `170-5:170-5:170-4:170-4:170-3:170-3`
- **Items/keywords:**
  - `.i.k.cruel` (Super Luck — cruel punishes heroes)
  - `.i.k.first` (Quick Attack — acts early)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] critical[nokeyword]` (critical damage on top faces)
- **Doc:** `.doc.Disaster Follows in Its Wake`
- **Notes:** HP 8 but massive damage output (5/5/4/4/3/3). Cruel + first = acts early and punishes. The floor 17-19 "oh no" monster. Must be killed ASAP or it wipes heroes.

**Full definition string:**
```
Wolf.n.Absol.hp.8.doc.Disaster Follows in Its Wake.sd.170-5:170-5:170-4:170-4:170-3:170-3.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] critical[nokeyword].img.PLACEHOLDER
```

---

## PART 2: Boss Fights — Gen 3

---

### Floor 4: Golem + Geodude/Graveler

**Fight structure:** `ch.om4.fight.(MAIN_BOSS + MINIONS)`

#### Golem (Main Boss, HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Heavy damage + basic shields. Rock-type bruiser boss.
- **sd:** `170-3:170-3:118-3:118-3:170-2:170-2`
- **Items/keywords:**
  - `.i.k.rite` (Rock Polish — rite keyword for scaling)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword]` (Earthquake — heavy damage)
  - `.i.topbot.facade.Ese118:0` (Harden — shield faces with enemy shield facade)
- **Doc:** `.doc.The Rolling Boulder`

**Full definition string:**
```
Alpha.n.Golem.hp.12.doc.The Rolling Boulder.sd.170-3:170-3:118-3:118-3:170-2:170-2.i.k.rite.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.Ese118:0.img.PLACEHOLDER
```

#### Geodude (Minion x2, HP 3)

- **Template base:** Slimelet (4 faces)
- **Concept:** Basic damage + self-destruct face (damage + death on use).
- **sd:** `170-1:170-1:170-1:170-1`
- **Items/keywords:**
  - `.i.k.singleuse` (Self-Destruct — singleuse means it dies after the big hit, applied via facade to specific face)
  - `.i.left2.facade.bas170:90:0:0#sidesc.[pips] damage[red] [n]self-destruct (dies after)[nokeyword]`
- **Doc:** `.doc.Might Explode`

**Full definition string:**
```
Slimelet.n.Geodude.hp.3.doc.Might Explode.sd.170-1:170-1:170-1:170-1.i.k.singleuse.i.left2.facade.bas170:90:0:0#sidesc.[pips] damage[red] [n]self-destruct (dies after)[nokeyword].img.PLACEHOLDER
```

#### Graveler (Elite Minion x1, HP 6)

- **Template base:** Wolf (4 faces)
- **Concept:** Shields + damage. Tougher minion.
- **sd:** `170-2:170-2:118-2:118-2`
- **Items/keywords:** (none — straightforward)
- **Doc:** `.doc.Rolling Towards You`

**Full definition string:**
```
Wolf.n.Graveler.hp.6.doc.Rolling Towards You.sd.170-2:170-2:118-2:118-2.img.PLACEHOLDER
```

#### Complete Fight String (Floor 4 — Gen 3):
```
ch.om4.fight.(
  Slimelet.n.Geodude.hp.3.doc.Might Explode.sd.170-1:170-1:170-1:170-1.i.k.singleuse.i.left2.facade.bas170:90:0:0#sidesc.[pips] damage[red] [n]self-destruct (dies after)[nokeyword].img.PLACEHOLDER
  +Slimelet.n.Geodude.hp.3.doc.Might Explode.sd.170-1:170-1:170-1:170-1.i.k.singleuse.i.left2.facade.bas170:90:0:0#sidesc.[pips] damage[red] [n]self-destruct (dies after)[nokeyword].img.PLACEHOLDER
  +Wolf.n.Graveler.hp.6.doc.Rolling Towards You.sd.170-2:170-2:118-2:118-2.img.PLACEHOLDER
  +Alpha.n.Golem.hp.12.doc.The Rolling Boulder.sd.170-3:170-3:118-3:118-3:170-2:170-2.i.k.rite.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.Ese118:0.img.PLACEHOLDER
).mn.Golem
```
**Total encounter HP:** 3 + 3 + 6 + 12 = 24 (within F4 budget of ~27 from Quagsire fight: 11 + 3x3 + other = ~22-27)

---

### Floor 8: Alpha Steelix + Onix

**Fight structure:** `ch.om8.fight.(MINIONS + BOSS)`

#### Alpha Steelix (Main Boss, HP 18)

- **Template base:** Alpha (6 faces)
- **Concept:** Massive Steel Shield + Heavy Damage + Repel.
- **sd:** `170-5:170-5:118-6:118-6:118-4:118-4`
- **Items/keywords:**
  - `.i.k.rite` (Rock Polish — rite for scaling)
  - `.i.k.stasis` (Iron Defense — resists control)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword]` (Iron Tail — heavy damage)
  - `.i.topbot.facade.Ese118:0` (shield faces with repel-like behavior)
  - `.i.right2.facade.Ese118:0` (more shields)
- **Doc:** `.doc.Lord of Iron and Stone`

**Full definition string:**
```
Alpha.n.Alpha Steelix.hp.18.doc.Lord of Iron and Stone.sd.170-5:170-5:118-6:118-6:118-4:118-4.i.k.rite.i.k.stasis.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.Ese118:0.i.right2.facade.Ese118:0.img.PLACEHOLDER
```

#### Onix (Minion x2, HP 7)

- **Template base:** Wolf (6 faces)
- **Concept:** Shields + basic damage. Wall minions.
- **sd:** `170-2:170-2:118-3:118-3:118-2:118-2`
- **Items/keywords:** (none — straightforward walls)
- **Doc:** `.doc.Coils Around the Cavern`

**Full definition string:**
```
Wolf.n.Onix.hp.7.doc.Coils Around the Cavern.sd.170-2:170-2:118-3:118-3:118-2:118-2.img.PLACEHOLDER
```

#### Complete Fight String (Floor 8 — Gen 3):
```
ch.om8.fight.(
  Wolf.n.Onix.hp.7.doc.Coils Around the Cavern.sd.170-2:170-2:118-3:118-3:118-2:118-2.img.PLACEHOLDER
  +Wolf.n.Onix.hp.7.doc.Coils Around the Cavern.sd.170-2:170-2:118-3:118-3:118-2:118-2.img.PLACEHOLDER
  +Alpha.n.Alpha Steelix.hp.18.doc.Lord of Iron and Stone.sd.170-5:170-5:118-6:118-6:118-4:118-4.i.k.rite.i.k.stasis.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.Ese118:0.i.right2.facade.Ese118:0.img.PLACEHOLDER
).mn.Alpha Steelix
```
**Total encounter HP:** 7 + 7 + 18 = 32 (within F8 budget — Exeggutor fight is ~30-35)

---

### Floor 12: Regi Trio

**Fight structure:** `ch.om12.fight.(REGI1 + REGI2 + REGI3)`

#### Regirock (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Heavy damage + Shields. Rock-type bruiser.
- **sd:** `170-4:170-4:170-3:170-3:118-3:118-3`
- **Items/keywords:**
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword]` (Stone Edge — heavy)
  - `.i.right2.facade.Ese118:0`
- **Doc:** `.doc.The Sealed Stone`

**Full definition string:**
```
Alpha.n.Regirock.hp.12.doc.The Sealed Stone.sd.170-4:170-4:170-3:170-3:118-3:118-3.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.right2.facade.Ese118:0.img.PLACEHOLDER
```

#### Regice (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Damage + Weaken + inflict-singleuse. Ice-type debuffer.
- **sd:** `170-4:170-4:171-2:171-2:170-2:170-2`
- **Items/keywords:**
  - `.i.k.exert` (Ice Beam — forces exertion)
  - `.i.k.singleuse` (Blizzard — singleuse on cleave faces for one massive hit)
  - `.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]` (top faces weaken)
  - `.i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict exert[nokeyword]` (mid cleave faces inflict exert)
- **Doc:** `.doc.The Sealed Ice`

**Full definition string:**
```
Alpha.n.Regice.hp.12.doc.The Sealed Ice.sd.170-4:170-4:171-2:171-2:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict exert[nokeyword].img.PLACEHOLDER
```

#### Registeel (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Steel Shield + Repel + Enduring. Defensive wall.
- **sd:** `170-3:170-3:118-5:118-5:118-4:118-4`
- **Items/keywords:**
  - `.i.k.stasis` (Iron Defense — resists control effects)
  - `.i.topbot.facade.Ese118:0` (mid shield faces repel-like)
  - `.i.right2.facade.Ese118:0` (bottom shield faces repel-like)
- **Doc:** `.doc.The Sealed Steel`

**Full definition string:**
```
Alpha.n.Registeel.hp.12.doc.The Sealed Steel.sd.170-3:170-3:118-5:118-5:118-4:118-4.i.k.stasis.i.topbot.facade.Ese118:0.i.right2.facade.Ese118:0.img.PLACEHOLDER
```

#### Complete Fight String (Floor 12 — Gen 3):
```
ch.om12.fight.(
  Alpha.n.Regirock.hp.12.doc.The Sealed Stone.sd.170-4:170-4:170-3:170-3:118-3:118-3.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.right2.facade.Ese118:0.img.PLACEHOLDER
  +Alpha.n.Regice.hp.12.doc.The Sealed Ice.sd.170-4:170-4:171-2:171-2:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict exert[nokeyword].img.PLACEHOLDER
  +Alpha.n.Registeel.hp.12.doc.The Sealed Steel.sd.170-3:170-3:118-5:118-5:118-4:118-4.i.k.stasis.i.topbot.facade.Ese118:0.i.right2.facade.Ese118:0.img.PLACEHOLDER
).mn.RegiTrio
```
**Total encounter HP:** 12 + 12 + 12 = 36 (matches F12 budget of ~35-36, same as Xerneas 25 + Florges 10)

---

### Floor 16: Regigigas + Regi Guardians

**Fight structure:** `ch.om16.fight.(MINIONS + BOSS)` — modeled after Zygarde, with weakened Regi trio as guardians to avoid a solo stat-check boss. Regigigas is the creator/master of the Regi trio; Regirock, Regice, and Registeel serve as his guardians in this fight. These are weakened versions (HP 6-8 each) compared to their full-power F12 boss versions (HP 12 each).

#### Regigigas (Main Boss, HP 20)

- **Template base:** Dragon (6 faces)
- **Concept:** "Slow Start" mechanic — first phase has weakened/stasis faces. After powering up: massive Heavy damage + Rampage. Modeled after Zygarde phase pattern.
- **Phase 1 (Slow Start) sd:** `170-2:170-2:0:0:0:0`
- **Phase 2 (Full Power) sd:** `170-8:170-6:171-4:171-4:170-3:170-2`
- **Items/keywords (Phase 1 — Slow Start):**
  - `.i.k.stasis` (Slow Start — limited action)
  - `.i.left2.facade.bas170:0:0:0#sidesc.[pips] damage[red] [n]slow start (weakened)[nokeyword]`
- **Items/keywords (Phase 2 — Full Power):**
  - `.i.k.cruel` (Crush Grip — cruel punishment)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword]` (Giga Impact — heavy)
  - `.i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] rampage[nokeyword]` (Earthquake — AoE rampage)
- **Doc:** `.doc.The Colossal Titan Awakens`
- **Notes:** Phase transition triggered by HP threshold (like Zygarde). Phase 1 = HP 20-11 (weak). Phase 2 = HP 10-0 (devastating). The "Slow Start" gives players time to deal with the Regi guardians before the onslaught. HP reduced from 25 to 20 to accommodate the stronger Regi guardians within budget.

#### Regirock Guardian (Minion, HP 7)

- **Template base:** Wolf (6 faces)
- **Concept:** Weakened version of the F12 Regirock boss. Heavy damage + basic shields. Rock-type guardian.
- **sd:** `170-3:170-3:170-2:170-2:118-2:118-2`
- **Items/keywords:**
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword]` (Stone Edge — heavy)
- **Doc:** `.doc.The Sealed Stone Serves Its Master`

**Full definition string:**
```
Wolf.n.Regirock.hp.7.doc.The Sealed Stone Serves Its Master.sd.170-3:170-3:170-2:170-2:118-2:118-2.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].img.PLACEHOLDER
```

#### Regice Guardian (Minion, HP 6)

- **Template base:** Wolf (6 faces)
- **Concept:** Weakened version of the F12 Regice boss. Damage + weaken. Ice-type guardian.
- **sd:** `170-3:170-3:170-2:170-2:170-1:170-1`
- **Items/keywords:**
  - `.i.k.exert` (Ice Beam — forces exertion)
  - `.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]` (top faces weaken)
- **Doc:** `.doc.The Sealed Ice Serves Its Master`

**Full definition string:**
```
Wolf.n.Regice.hp.6.doc.The Sealed Ice Serves Its Master.sd.170-3:170-3:170-2:170-2:170-1:170-1.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].img.PLACEHOLDER
```

#### Registeel Guardian (Minion, HP 8)

- **Template base:** Wolf (6 faces)
- **Concept:** Weakened version of the F12 Registeel boss. Shields + repel. Steel-type guardian wall.
- **sd:** `170-2:170-2:118-3:118-3:118-2:118-2`
- **Items/keywords:**
  - `.i.k.stasis` (Iron Defense — resists control effects)
  - `.i.topbot.facade.Ese118:0` (mid shield faces repel-like)
- **Doc:** `.doc.The Sealed Steel Serves Its Master`

**Full definition string:**
```
Wolf.n.Registeel.hp.8.doc.The Sealed Steel Serves Its Master.sd.170-2:170-2:118-3:118-3:118-2:118-2.i.k.stasis.i.topbot.facade.Ese118:0.img.PLACEHOLDER
```

**Phase 1 definition string:**
```
Dragon.n.Regigigas.hp.20.doc.The Colossal Titan Awakens.sd.170-2:170-2:0:0:0:0.i.k.stasis.i.left2.facade.bas170:0:0:0#sidesc.[pips] damage[red] [n]slow start (weakened)[nokeyword].i.triggerhpdata.(lost.sd.170-8:170-6:171-4:171-4:170-3:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] rampage[nokeyword]).img.PLACEHOLDER
```

#### Complete Fight String (Floor 16 — Gen 3):
```
ch.om16.fight.(
  Wolf.n.Regirock.hp.7.doc.The Sealed Stone Serves Its Master.sd.170-3:170-3:170-2:170-2:118-2:118-2.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].img.PLACEHOLDER
  +Wolf.n.Regice.hp.6.doc.The Sealed Ice Serves Its Master.sd.170-3:170-3:170-2:170-2:170-1:170-1.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].img.PLACEHOLDER
  +Wolf.n.Registeel.hp.8.doc.The Sealed Steel Serves Its Master.sd.170-2:170-2:118-3:118-3:118-2:118-2.i.k.stasis.i.topbot.facade.Ese118:0.img.PLACEHOLDER
  +Dragon.n.Regigigas.hp.20.doc.The Colossal Titan Awakens.sd.170-2:170-2:0:0:0:0.i.k.stasis.i.left2.facade.bas170:0:0:0#sidesc.[pips] damage[red] [n]slow start (weakened)[nokeyword].i.triggerhpdata.(lost.sd.170-8:170-6:171-4:171-4:170-3:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] rampage[nokeyword]).img.PLACEHOLDER
).mn.Regigigas
```
**Total encounter HP:** 7 + 6 + 8 + 20 = 41 (within F16 budget of 25-35+, comparable to Zygarde ~25-35; the higher total is offset by the Regi guardians being individually weaker than dedicated minions, providing tactical variety over raw HP pressure)

---

### Floor 20: Deoxys (4 Forms)

**Fight structure:** Multi-phase like Necrozma. Each form is a separate phase triggered by HP loss.

#### Deoxys Normal (Phase 1, HP 10)

- **Template base:** Basalt (6 faces)
- **Concept:** Balanced — damage + shields + moderate threat.
- **sd:** `170-4:170-4:118-3:118-3:170-3:170-3`
- **Items/keywords:**
  - `.i.k.rite` (Cosmic Power — rite for scaling)
- **Doc:** `.doc.The Extraterrestrial Mutant`

**Full definition string:**
```
Basalt.n.Deoxys.hp.10.doc.The Extraterrestrial Mutant.sd.170-4:170-4:118-3:118-3:170-3:170-3.i.k.rite.img.PLACEHOLDER
```

#### Deoxys Attack (Phase 2, HP 8)

- **Template base:** Basalt (6 faces)
- **Concept:** Nearly all high damage faces. Glass cannon form.
- **sd:** `170-8:170-8:170-6:170-6:171-4:171-4`
- **Items/keywords:**
  - `.i.k.cruel` (Psycho Boost — cruel for extra punishment)
  - `.i.k.first` (Extreme Speed — acts first)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] critical[nokeyword]`
- **Doc:** `.doc.Attack Form — Maximum Offense`

**Full definition string:**
```
Basalt.n.Deoxys.hp.8.doc.Attack Form — Maximum Offense.sd.170-8:170-8:170-6:170-6:171-4:171-4.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] critical[nokeyword].img.PLACEHOLDER
```

#### Deoxys Defense (Phase 3, HP 12)

- **Template base:** Basalt (6 faces)
- **Concept:** Nearly all shield + repel faces. Stalling form.
- **sd:** `118-6:118-6:118-5:118-5:170-2:170-2`
- **Items/keywords:**
  - `.i.k.stasis` (Cosmic Power — stasis for control resistance)
  - `.i.left2.facade.Ese118:0` (shields with repel)
  - `.i.topbot.facade.Ese118:0` (more shields with repel)
- **Doc:** `.doc.Defense Form — Impenetrable`

**Full definition string:**
```
Basalt.n.Deoxys.hp.12.doc.Defense Form — Impenetrable.sd.118-6:118-6:118-5:118-5:170-2:170-2.i.k.stasis.i.left2.facade.Ese118:0.i.topbot.facade.Ese118:0.img.PLACEHOLDER
```

#### Deoxys Speed (Phase 4, HP 8)

- **Template base:** Basalt (6 faces)
- **Concept:** Dodge + high damage. Fast and evasive.
- **sd:** `170-5:170-5:170-4:170-4:0:0`
- **Items/keywords:**
  - `.i.k.first` (Extreme Speed — acts first)
  - `.i.k.exert` (Agility — forces hero exertion)
  - `.i.right2.facade.bas170:0:0:0#sidesc.dodge[green] [n]evade next attack[nokeyword]` (bottom blank faces = dodge via facade)
- **Doc:** `.doc.Speed Form — Untouchable`

**Full definition string:**
```
Basalt.n.Deoxys.hp.8.doc.Speed Form — Untouchable.sd.170-5:170-5:170-4:170-4:0:0.i.k.first.i.k.exert.i.right2.facade.bas170:0:0:0#sidesc.dodge[green] [n]evade next attack[nokeyword].img.PLACEHOLDER
```

#### Complete Fight String (Floor 20 — Gen 3):
Uses Necrozma-style phase transitions with `.triggerhpdata.` for form changes.
```
ch.om20.fight.(
  Basalt.n.Deoxys.hp.10.doc.The Extraterrestrial Mutant.sd.170-4:170-4:118-3:118-3:170-3:170-3.i.k.rite
  .i.triggerhpdata.(lost.sd.170-8:170-8:170-6:170-6:171-4:171-4.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] critical[nokeyword].doc.Attack Form — Maximum Offense.hp.8.img.PLACEHOLDER
    .i.triggerhpdata.(lost.sd.118-6:118-6:118-5:118-5:170-2:170-2.i.k.stasis.i.left2.facade.Ese118:0.i.topbot.facade.Ese118:0.doc.Defense Form — Impenetrable.hp.12.img.PLACEHOLDER
      .i.triggerhpdata.(lost.sd.170-5:170-5:170-4:170-4:0:0.i.k.first.i.k.exert.i.right2.facade.bas170:0:0:0#sidesc.dodge[green] [n]evade next attack[nokeyword].doc.Speed Form — Untouchable.hp.8.img.PLACEHOLDER
      )
    )
  )
  .img.PLACEHOLDER
).mn.Deoxys
```
**Total encounter HP:** 10 + 8 + 12 + 8 = 38 (within F20 budget of 30-40+)

---

## PART 3: Boss Fights — Gen 4

---

### Floor 12: Palkia + Bronzong Support

#### Palkia (Main Boss, HP 25)

- **Template base:** Alpha (6 faces)
- **Concept:** Ranged high damage (Spatial Rend). Space-warping legendary.
- **sd:** `170-5:170-5:171-3:171-3:170-4:170-4`
- **Items/keywords:**
  - `.i.k.cruel` (Spatial Rend — cruel for critical-like behavior)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Spatial Rend[nokeyword]` (top faces = Spatial Rend)
  - `.i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]distort space[nokeyword]` (mid faces = AoE space distortion)
- **Doc:** `.doc.Master of Space`

**Full definition string:**
```
Alpha.n.Palkia.hp.25.doc.Master of Space.sd.170-5:170-5:171-3:171-3:170-4:170-4.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Spatial Rend[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]distort space[nokeyword].img.PLACEHOLDER
```

#### Bronzong (Support Minion x1, HP 8)

- **Template base:** Wolf (6 faces)
- **Concept:** Shields Palkia + inflicts stasis on heroes.
- **sd:** `118-3:118-3:118-2:118-2:170-1:170-1`
- **Items/keywords:**
  - `.i.k.stasis` (Hypnosis — stasis on heroes)
  - `.i.left2.facade.Ese118:0` (shield with ally-shield behavior)
- **Doc:** `.doc.Warps Time and Space`

**Full definition string:**
```
Wolf.n.Bronzong.hp.8.doc.Warps Time and Space.sd.118-3:118-3:118-2:118-2:170-1:170-1.i.k.stasis.i.left2.facade.Ese118:0.img.PLACEHOLDER
```

#### Spatial Rift (Hazard Minion x1, HP 5)

- **Template base:** Slimelet (4 faces)
- **Concept:** Environmental hazard — passive AoE damage each turn.
- **sd:** `171-1:171-1:171-1:171-1`
- **Items/keywords:**
  - `.i.k.pain` (spatial distortion — pain for DoT)
- **Doc:** `.doc.A Tear in Reality`

**Full definition string:**
```
Slimelet.n.Spatial Rift.hp.5.doc.A Tear in Reality.sd.171-1:171-1:171-1:171-1.i.k.pain.img.PLACEHOLDER
```

#### Complete Fight String (Floor 12 — Gen 4):
```
ch.om12.fight.(
  Slimelet.n.Spatial Rift.hp.5.doc.A Tear in Reality.sd.171-1:171-1:171-1:171-1.i.k.pain.img.PLACEHOLDER
  +Wolf.n.Bronzong.hp.8.doc.Warps Time and Space.sd.118-3:118-3:118-2:118-2:170-1:170-1.i.k.stasis.i.left2.facade.Ese118:0.img.PLACEHOLDER
  +Alpha.n.Palkia.hp.25.doc.Master of Space.sd.170-5:170-5:171-3:171-3:170-4:170-4.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Spatial Rend[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]distort space[nokeyword].img.PLACEHOLDER
).mn.Palkia
```
**Total encounter HP:** 5 + 8 + 25 = 38 (within F12 budget — Xerneas+Florges = ~35; slightly over but F12 Gen 4 should be challenging)

---

### Floor 16: Dialga + Temporal Minions

#### Dialga (Main Boss, HP 25)

- **Template base:** Dragon (6 faces)
- **Concept:** Massive damage + weaken all (Roar of Time). Time-warping legendary.
- **sd:** `170-6:170-6:171-4:171-4:170-3:170-3`
- **Items/keywords:**
  - `.i.k.exert` (Roar of Time — forces hero exertion, simulating time distortion)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Roar of Time[nokeyword]` (top = Roar of Time)
  - `.i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword]` (mid = AoE weaken)
- **Doc:** `.doc.Master of Time`

**Full definition string:**
```
Dragon.n.Dialga.hp.25.doc.Master of Time.sd.170-6:170-6:171-4:171-4:170-3:170-3.i.k.exert.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Roar of Time[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword].img.PLACEHOLDER
```

#### Bronzong (Support Minion x1, HP 8)

- **Template base:** Wolf (6 faces)
- **Concept:** Shields Dialga + stasis.
- **sd:** `118-3:118-3:118-2:118-2:170-2:170-2`
- **Items/keywords:**
  - `.i.k.stasis` (Hypnosis)
  - `.i.left2.facade.Ese118:0`
- **Doc:** `.doc.Keeper of Temporal Order`

**Full definition string:**
```
Wolf.n.Bronzong.hp.8.doc.Keeper of Temporal Order.sd.118-3:118-3:118-2:118-2:170-2:170-2.i.k.stasis.i.left2.facade.Ese118:0.img.PLACEHOLDER
```

#### Temporal Anomaly (Hazard Minion x1, HP 5)

- **Template base:** Slimelet (4 faces)
- **Concept:** Buffs Dialga each turn. Passive stasis threat.
- **sd:** `171-2:171-2:170-1:170-1`
- **Items/keywords:**
  - `.i.k.stasis` (Time Warp — stasis on heroes)
  - `.i.k.exert` (temporal drain)
- **Doc:** `.doc.A Fracture in Time`

**Full definition string:**
```
Slimelet.n.Temporal Anomaly.hp.5.doc.A Fracture in Time.sd.171-2:171-2:170-1:170-1.i.k.stasis.i.k.exert.img.PLACEHOLDER
```

#### Complete Fight String (Floor 16 — Gen 4):
```
ch.om16.fight.(
  Slimelet.n.Temporal Anomaly.hp.5.doc.A Fracture in Time.sd.171-2:171-2:170-1:170-1.i.k.stasis.i.k.exert.img.PLACEHOLDER
  +Wolf.n.Bronzong.hp.8.doc.Keeper of Temporal Order.sd.118-3:118-3:118-2:118-2:170-2:170-2.i.k.stasis.i.left2.facade.Ese118:0.img.PLACEHOLDER
  +Dragon.n.Dialga.hp.25.doc.Master of Time.sd.170-6:170-6:171-4:171-4:170-3:170-3.i.k.exert.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Roar of Time[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword].img.PLACEHOLDER
).mn.Dialga
```
**Total encounter HP:** 5 + 8 + 25 = 38 (within F16 budget — Zygarde is HP 25 solo + cells ~8-10)

---

### Floor 20: Arceus (Multi-Phase Final Boss)

**Fight structure:** Multi-phase like Necrozma. Type-shifting via `.triggerhpdata.` with different facades per phase.

#### Arceus Phase 1 — Normal Type (HP 12)

- **Template base:** Basalt (6 faces)
- **Concept:** Balanced Judgment. Moderate damage + moderate shields.
- **sd:** `170-5:170-5:171-3:171-3:118-3:118-3`
- **Items/keywords:**
  - `.i.k.rite` (Cosmic Power — rite for scaling)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Judgment[nokeyword]`
- **Doc:** `.doc.The Alpha Pokemon — Origin of All`

#### Arceus Phase 2 — Fire Type (HP 10)

- **Concept:** Fire Judgment = massive AoE damage. Burns everything.
- **sd:** `171-5:171-5:171-4:171-4:170-3:170-3`
- **Items/keywords:**
  - `.i.k.cruel` (Sacred Fire)
  - `.i.k.pain` (burn DoT)
  - `.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]fire Judgment[nokeyword]`

#### Arceus Phase 3 — Steel Type (HP 12)

- **Concept:** Steel Judgment = heavy shields + repel. Defensive shift.
- **sd:** `118-6:118-6:118-5:118-5:170-4:170-4`
- **Items/keywords:**
  - `.i.k.stasis` (Iron Defense)
  - `.i.left2.facade.Ese118:0` (repel shields)
  - `.i.topbot.facade.Ese118:0`

#### Arceus Phase 4 — Dragon Type (HP 8)

- **Concept:** Dragon Judgment = devastating single-target. Final burst.
- **sd:** `170-8:170-8:170-6:170-6:171-4:171-4`
- **Items/keywords:**
  - `.i.k.cruel` (Draco Meteor)
  - `.i.k.first` (Extreme Speed)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Dragon Judgment[nokeyword]`

#### Complete Fight String (Floor 20 — Gen 4):
```
ch.om20.fight.(
  Basalt.n.Arceus.hp.12.doc.The Alpha Pokemon — Origin of All.sd.170-5:170-5:171-3:171-3:118-3:118-3.i.k.rite.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Judgment[nokeyword]
  .i.triggerhpdata.(lost.sd.171-5:171-5:171-4:171-4:170-3:170-3.i.k.cruel.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]fire Judgment[nokeyword].doc.Fire Type — Everything Burns.hp.10.img.PLACEHOLDER
    .i.triggerhpdata.(lost.sd.118-6:118-6:118-5:118-5:170-4:170-4.i.k.stasis.i.left2.facade.Ese118:0.i.topbot.facade.Ese118:0.doc.Steel Type — Impervious.hp.12.img.PLACEHOLDER
      .i.triggerhpdata.(lost.sd.170-8:170-8:170-6:170-6:171-4:171-4.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Dragon Judgment[nokeyword].doc.Dragon Type — Final Judgment.hp.8.img.PLACEHOLDER
      )
    )
  )
  .img.PLACEHOLDER
).mn.Arceus
```
**Total encounter HP:** 12 + 10 + 12 + 8 = 42 (within F20 budget of 30-40+, appropriate for final boss)

---

## PART 4: Boss Fights — Gen 5

---

### Floor 8: Serperior + Emboar + Samurott (Unova Starters)

#### Serperior (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Grass + shields. Defensive starter.
- **sd:** `170-3:170-3:118-3:118-3:118-2:118-2`
- **Items/keywords:**
  - `.i.k.rite` (Coil — rite for scaling)
  - `.i.topbot.facade.Ese118:0` (Leaf Blade shield)
- **Doc:** `.doc.The Regal Serpent`

**Full definition string:**
```
Alpha.n.Serperior.hp.12.doc.The Regal Serpent.sd.170-3:170-3:118-3:118-3:118-2:118-2.i.k.rite.i.topbot.facade.Ese118:0.img.PLACEHOLDER
```

#### Emboar (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Fire damage + rampage. Aggressive starter.
- **sd:** `170-4:170-4:171-3:171-3:170-2:170-2`
- **Items/keywords:**
  - `.i.k.cruel` (Flare Blitz — cruel)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Flare Blitz[nokeyword]`
  - `.i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] rampage[nokeyword]`
- **Doc:** `.doc.The Blazing Boar`

**Full definition string:**
```
Alpha.n.Emboar.hp.12.doc.The Blazing Boar.sd.170-4:170-4:171-3:171-3:170-2:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Flare Blitz[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] rampage[nokeyword].img.PLACEHOLDER
```

#### Samurott (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Water damage + cleave. Balanced starter.
- **sd:** `170-3:170-3:171-2:171-2:170-2:170-2`
- **Items/keywords:**
  - `.i.k.first` (Aqua Jet — acts first)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Razor Shell[nokeyword]`
- **Doc:** `.doc.The Seamitar Warrior`

**Full definition string:**
```
Alpha.n.Samurott.hp.12.doc.The Seamitar Warrior.sd.170-3:170-3:171-2:171-2:170-2:170-2.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Razor Shell[nokeyword].img.PLACEHOLDER
```

#### Complete Fight String (Floor 8 — Gen 5):
```
ch.om8.fight.(
  Alpha.n.Serperior.hp.12.doc.The Regal Serpent.sd.170-3:170-3:118-3:118-3:118-2:118-2.i.k.rite.i.topbot.facade.Ese118:0.img.PLACEHOLDER
  +Alpha.n.Emboar.hp.12.doc.The Blazing Boar.sd.170-4:170-4:171-3:171-3:170-2:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Flare Blitz[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] rampage[nokeyword].img.PLACEHOLDER
  +Alpha.n.Samurott.hp.12.doc.The Seamitar Warrior.sd.170-3:170-3:171-2:171-2:170-2:170-2.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Razor Shell[nokeyword].img.PLACEHOLDER
).mn.UnovaStarters
```
**Total encounter HP:** 12 + 12 + 12 = 36 (within F8 budget — Exeggutor is ~30-35)

---

### Floor 12 (Path A): Swords of Justice

#### Cobalion (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Steel/Fighting. Shields + moderate damage.
- **sd:** `170-3:170-3:118-4:118-4:170-2:170-2`
- **Items/keywords:**
  - `.i.k.stasis` (Iron Defense)
  - `.i.topbot.facade.Ese118:0`
- **Doc:** `.doc.The Iron Will`

**Full definition string:**
```
Alpha.n.Cobalion.hp.12.doc.The Iron Will.sd.170-3:170-3:118-4:118-4:170-2:170-2.i.k.stasis.i.topbot.facade.Ese118:0.img.PLACEHOLDER
```

#### Terrakion (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Rock/Fighting. Heavy damage bruiser.
- **sd:** `170-5:170-5:170-4:170-4:170-2:170-2`
- **Items/keywords:**
  - `.i.k.cruel` (Close Combat — cruel)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword]`
- **Doc:** `.doc.The Cavern Champion`

**Full definition string:**
```
Alpha.n.Terrakion.hp.12.doc.The Cavern Champion.sd.170-5:170-5:170-4:170-4:170-2:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].img.PLACEHOLDER
```

#### Virizion (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Grass/Fighting. Fast damage + weaken.
- **sd:** `170-4:170-4:170-3:170-3:158-2:158-2`
- **Items/keywords:**
  - `.i.k.first` (Quick Attack)
  - `.i.k.exert` (Leaf Blade — forces exertion)
- **Doc:** `.doc.The Forest Guardian`

**Full definition string:**
```
Alpha.n.Virizion.hp.12.doc.The Forest Guardian.sd.170-4:170-4:170-3:170-3:158-2:158-2.i.k.first.i.k.exert.img.PLACEHOLDER
```

#### Complete Fight String (Floor 12, Path A — Gen 5):
```
ch.om12.fight.(
  Alpha.n.Cobalion.hp.12.doc.The Iron Will.sd.170-3:170-3:118-4:118-4:170-2:170-2.i.k.stasis.i.topbot.facade.Ese118:0.img.PLACEHOLDER
  +Alpha.n.Terrakion.hp.12.doc.The Cavern Champion.sd.170-5:170-5:170-4:170-4:170-2:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].img.PLACEHOLDER
  +Alpha.n.Virizion.hp.12.doc.The Forest Guardian.sd.170-4:170-4:170-3:170-3:158-2:158-2.i.k.first.i.k.exert.img.PLACEHOLDER
).mn.SwordsOfJustice
```
**Total encounter HP:** 12 + 12 + 12 = 36

---

### Floor 12 (Path B): Forces of Nature

#### Tornadus (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** AoE + dodge. Wind-based.
- **sd:** `171-3:171-3:170-3:170-3:0:0`
- **Items/keywords:**
  - `.i.k.exert` (Hurricane — forces exertion)
  - `.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]hurricane[nokeyword]`
  - `.i.right2.facade.bas170:0:0:0#sidesc.dodge[green] [n]tailwind[nokeyword]` (blank faces = dodge)
- **Doc:** `.doc.The Cyclone`

**Full definition string:**
```
Alpha.n.Tornadus.hp.12.doc.The Cyclone.sd.171-3:171-3:170-3:170-3:0:0.i.k.exert.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]hurricane[nokeyword].i.right2.facade.bas170:0:0:0#sidesc.dodge[green] [n]tailwind[nokeyword].img.PLACEHOLDER
```

#### Thundurus (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Charged + high single-target damage.
- **sd:** `170-5:170-5:170-4:170-4:170-2:170-2`
- **Items/keywords:**
  - `.i.k.cruel` (Thunder — cruel)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Thunder[nokeyword]`
- **Doc:** `.doc.The Bolt Strike`

**Full definition string:**
```
Alpha.n.Thundurus.hp.12.doc.The Bolt Strike.sd.170-5:170-5:170-4:170-4:170-2:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Thunder[nokeyword].img.PLACEHOLDER
```

#### Landorus (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Heavy + ground AoE.
- **sd:** `170-4:170-4:171-3:171-3:118-2:118-2`
- **Items/keywords:**
  - `.i.k.pain` (Earth Power — pain DoT)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword]`
  - `.i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Earthquake[nokeyword]`
- **Doc:** `.doc.The Abundant Land`

**Full definition string:**
```
Alpha.n.Landorus.hp.12.doc.The Abundant Land.sd.170-4:170-4:171-3:171-3:118-2:118-2.i.k.pain.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Earthquake[nokeyword].img.PLACEHOLDER
```

#### Complete Fight String (Floor 12, Path B — Gen 5):
```
ch.om12.fight.(
  Alpha.n.Tornadus.hp.12.doc.The Cyclone.sd.171-3:171-3:170-3:170-3:0:0.i.k.exert.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]hurricane[nokeyword].i.right2.facade.bas170:0:0:0#sidesc.dodge[green] [n]tailwind[nokeyword].img.PLACEHOLDER
  +Alpha.n.Thundurus.hp.12.doc.The Bolt Strike.sd.170-5:170-5:170-4:170-4:170-2:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Thunder[nokeyword].img.PLACEHOLDER
  +Alpha.n.Landorus.hp.12.doc.The Abundant Land.sd.170-4:170-4:171-3:171-3:118-2:118-2.i.k.pain.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Earthquake[nokeyword].img.PLACEHOLDER
).mn.ForcesOfNature
```
**Total encounter HP:** 12 + 12 + 12 = 36

---

### Floor 16: Reshiram/Zekrom

**Fight structure:** Zygarde-style variant pattern (two versions, one randomly selected per run).

#### Version A: Reshiram (Main Boss, HP 25)

- **Template base:** Dragon (6 faces)
- **Concept:** Fire AoE + weaken. The White Dragon.
- **sd:** `171-5:171-5:170-4:170-4:171-3:171-3`
- **Items/keywords:**
  - `.i.k.cruel` (Blue Flare — cruel)
  - `.i.k.pain` (Fire AoE — burn DoT)
  - `.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Blue Flare[nokeyword]`
  - `.i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword]`
- **Doc:** `.doc.The Vast White Dragon of Truth`

#### Reshiram Environmental Hazard — Zekrom Echo (Passive, HP 8)

- **Template base:** Slimelet (4 faces)
- **Concept:** Passive AoE damage each turn (Zekrom's influence from afar).
- **sd:** `171-1:171-1:171-1:171-1`
- **Items/keywords:**
  - `.i.k.pain` (electric field DoT)
- **Doc:** `.doc.Zekrom's Thunder Echoes`

#### Complete Fight String (Floor 16A — Reshiram):
```
ch.om16.fight.(
  Slimelet.n.Zekrom Echo.hp.8.doc.Zekrom's Thunder Echoes.sd.171-1:171-1:171-1:171-1.i.k.pain.img.PLACEHOLDER
  +Dragon.n.Reshiram.hp.25.doc.The Vast White Dragon of Truth.sd.171-5:171-5:170-4:170-4:171-3:171-3.i.k.cruel.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Blue Flare[nokeyword].i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword].img.PLACEHOLDER
).mn.Reshiram
```

#### Version B: Zekrom (Main Boss, HP 25)

- **Template base:** Dragon (6 faces)
- **Concept:** Electric burst + charged damage. The Black Dragon.
- **sd:** `170-6:170-6:170-5:170-5:171-3:171-3`
- **Items/keywords:**
  - `.i.k.cruel` (Bolt Strike — cruel)
  - `.i.k.first` (Thunder — fast)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Bolt Strike[nokeyword]`
  - `.i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Fusion Bolt[nokeyword]`
- **Doc:** `.doc.The Deep Black Dragon of Ideals`

#### Zekrom Environmental Hazard — Reshiram Echo (Passive, HP 8)

- **Template base:** Slimelet (4 faces)
- **Concept:** Passive fire AoE (Reshiram's influence from afar).
- **sd:** `171-1:171-1:171-1:171-1`
- **Items/keywords:**
  - `.i.k.pain` (fire field DoT)
- **Doc:** `.doc.Reshiram's Flames Linger`

#### Complete Fight String (Floor 16B — Zekrom):
```
ch.om16.fight.(
  Slimelet.n.Reshiram Echo.hp.8.doc.Reshiram's Flames Linger.sd.171-1:171-1:171-1:171-1.i.k.pain.img.PLACEHOLDER
  +Dragon.n.Zekrom.hp.25.doc.The Deep Black Dragon of Ideals.sd.170-6:170-6:170-5:170-5:171-3:171-3.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Bolt Strike[nokeyword].i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Fusion Bolt[nokeyword].img.PLACEHOLDER
).mn.Zekrom
```
**Total encounter HP (either version):** 8 + 25 = 33 (within F16 budget of 25-30+)

---

### Floor 20: Kyurem (Multi-Phase with Fusion)

**Fight structure:** Multi-phase. Base Kyurem → absorbs one dragon → Black or White Kyurem.

#### Kyurem Base (Phase 1, HP 15)

- **Template base:** Basalt (6 faces)
- **Concept:** Ice damage + weaken. Cold and slow.
- **sd:** `170-4:170-4:171-3:171-3:170-2:170-2`
- **Items/keywords:**
  - `.i.k.exert` (Glaciate — forces exertion, representing ice slowing)
  - `.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]`
- **Doc:** `.doc.The Boundary Pokemon — Incomplete`

#### Black Kyurem (Phase 2A — if absorbs Zekrom, HP 20)

- **Concept:** Fire+Ice hybrid. Massive physical damage.
- **sd:** `170-7:170-7:170-5:170-5:171-4:171-4`
- **Items/keywords:**
  - `.i.k.cruel` (Freeze Shock — cruel)
  - `.i.k.first` (Bolt Strike — fast)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Freeze Shock[nokeyword]`
- **Doc:** `.doc.Black Kyurem — Absolute Power`

#### White Kyurem (Phase 2B — if absorbs Reshiram, HP 20)

- **Concept:** Electric+Ice hybrid. Massive AoE.
- **sd:** `171-5:171-5:171-4:171-4:170-4:170-4`
- **Items/keywords:**
  - `.i.k.cruel` (Ice Burn — cruel)
  - `.i.k.pain` (burn DoT)
  - `.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Ice Burn[nokeyword]`
- **Doc:** `.doc.White Kyurem — Absolute Zero`

#### Complete Fight String (Floor 20 — Gen 5, Black Kyurem variant):
```
ch.om20.fight.(
  Basalt.n.Kyurem.hp.15.doc.The Boundary Pokemon — Incomplete.sd.170-4:170-4:171-3:171-3:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]
  .i.triggerhpdata.(lost.sd.170-7:170-7:170-5:170-5:171-4:171-4.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Freeze Shock[nokeyword].doc.Black Kyurem — Absolute Power.hp.20.img.PLACEHOLDER
  )
  .img.PLACEHOLDER
).mn.Black Kyurem
```

#### Complete Fight String (Floor 20 — Gen 5, White Kyurem variant):
```
ch.om20.fight.(
  Basalt.n.Kyurem.hp.15.doc.The Boundary Pokemon — Incomplete.sd.170-4:170-4:171-3:171-3:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]
  .i.triggerhpdata.(lost.sd.171-5:171-5:171-4:171-4:170-4:170-4.i.k.cruel.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Ice Burn[nokeyword].doc.White Kyurem — Absolute Zero.hp.20.img.PLACEHOLDER
  )
  .img.PLACEHOLDER
).mn.White Kyurem
```
**Total encounter HP:** 15 + 20 = 35 (within F20 budget of 30-40+)

---

## PART 5: HP Budget Summary

| Floor | Gen 3 | Gen 4 | Gen 5 | Budget Reference |
|-------|-------|-------|-------|-----------------|
| **4** | 24 (Golem 12 + Geodude 3x2 + Graveler 6) | — (uses existing Quagsire) | — (uses existing Quagsire) | Quagsire fight ~22-27 |
| **8** | 32 (A.Steelix 18 + Onix 7x2) | — (uses existing Exeggutor) | 36 (Serperior 12 + Emboar 12 + Samurott 12) | Exeggutor fight ~30-35 |
| **12** | 36 (Regi Trio 12x3 OR Legendary Birds 12x3) | 38 (Palkia 25 + Bronzong 8 + Rift 5) | 36 (SoJ or FoN 12x3) | Xerneas+Florges ~35 |
| **16** | 41 (Regigigas 20 + Regirock 7 + Regice 6 + Registeel 8) | 38 (Dialga 25 + Bronzong 8 + Anomaly 5) | 33 (Reshiram/Zekrom 25 + Echo 8) | Zygarde ~25-35 |
| **20** | 38 (Deoxys 4 phases) | 42 (Arceus 4 phases) | 35 (Kyurem 15 + fusion 20) | Necrozma ~35-40+ |

---

## PART 6: Design Notes

### Monster Design Philosophy
1. **Floor 1-3:** Simple mechanics. 1-2 keywords max. Teach players what monsters do.
2. **Floor 9-11:** Multi-keyword combinations. Monsters start synergizing (e.g., Lanturn shields other monsters).
3. **Floor 17-19:** Full-power enemies. Multiple keywords + high pips. Every monster is a threat.

### Boss Design Philosophy
1. **Floor 4:** Tutorial boss. Minions teach focus-fire (Geodude self-destruct teaches kill order).
2. **Floor 8:** Multi-body awareness. Shield walls (Steelix/Onix) or trio fights (Unova starters).
3. **Floor 12:** Trio puzzles. Three distinct threats require prioritization (Regi Trio, Swords, Forces).
4. **Floor 16:** Phase/variant bosses. Regigigas "Slow Start" teaches prep time. Reshiram/Zekrom environmental hazards.
5. **Floor 20:** Multi-phase final bosses. Deoxys form-shifting, Arceus type-shifting, Kyurem fusion. Demand adaptation.

### Status Effect Delivery on Monsters
- **Poison:** `.i.k.pain` (pain keyword simulates DoT)
- **Weaken:** `.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]` via facade on specific faces
- **Exert/Confuse:** `.i.k.exert` (forces hero exertion)
- **Stasis/Freeze:** `.i.k.stasis` (resists control, simulates frozen/iron defense)
- **Cruel:** `.i.k.cruel` (extra punishment on damage)
- **First/Speed:** `.i.k.first` (acts before heroes)
- **Single-use/Self-destruct:** `.i.k.singleuse` (one-time effect then locked)

### Image Placeholders
All `.img.PLACEHOLDER` values need to be replaced with actual pixel art data generated separately. Each monster/boss needs a unique sprite in the mod's compressed image format.

---

## PART 7: Legendary Birds Boss Fight

---

### Floor 12 (Gen 3 Alternative): Legendary Birds — Articuno, Zapdos, Moltres

**Fight structure:** `ch.om12.fight.(BIRD1 + BIRD2 + BIRD3)` — trio boss fight following the same pattern as the Regi Trio (Gen 3 F12), Swords of Justice (Gen 5 F12 Path A), and Forces of Nature (Gen 5 F12 Path B). Randomly selected as an alternative to the Regi Trio at Gen 3 Floor 12.

#### Articuno (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Ice-themed defensive bird. Shield + Weaken + damage. The wall of the trio — absorbs hits and debuffs heroes.
- **sd:** `170-3:170-3:118-4:118-4:170-2:170-2`
- **Items/keywords:**
  - `.i.k.exert` (Blizzard — forces hero exertion, simulating freezing cold)
  - `.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]` (Ice Beam — top damage faces inflict weaken)
  - `.i.topbot.facade.Ese118:0` (Ice Shield — mid shield faces with repel-like behavior)
- **Doc:** `.doc.The Frozen Wings of Winter`
- **Notes:** Defensive bird. 2 damage+weaken faces (3 pips), 2 shield faces (4 pips), 2 small damage faces (2 pips). Exert keyword + weaken makes Articuno the priority debuff target, but the shields make it hard to burst down. Total pips: 18.

**Full definition string:**
```
Alpha.n.Articuno.hp.12.doc.The Frozen Wings of Winter.sd.170-3:170-3:118-4:118-4:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].i.topbot.facade.Ese118:0.img.PLACEHOLDER
```

#### Zapdos (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Electric-themed offensive bird. High damage + Charged. The primary damage threat — hits hard and fast.
- **sd:** `170-5:170-5:170-4:170-4:170-2:170-2`
- **Items/keywords:**
  - `.i.k.cruel` (Thunder — cruel for extra punishment)
  - `.i.k.first` (Agility — acts before heroes)
  - `.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Thunder[nokeyword]` (top faces = Thunder, heavy electric damage)
- **Doc:** `.doc.The Thunder That Splits the Sky`
- **Notes:** Pure offense bird. Every face deals damage (5/5/4/4/2/2). Cruel + first means Zapdos hits first and hits hard. No shields, no heals — glass cannon among the trio. Must be focus-fired or it rips through the party. Total pips: 22. (Reduced from 24 to 22 to match Terrakion/Thundurus pip ceiling — 24 pips with cruel+first was overbudget for an F12 trio member.)

**Full definition string:**
```
Alpha.n.Zapdos.hp.12.doc.The Thunder That Splits the Sky.sd.170-5:170-5:170-4:170-4:170-2:170-2.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Thunder[nokeyword].img.PLACEHOLDER
```

#### Moltres (HP 12)

- **Template base:** Alpha (6 faces)
- **Concept:** Fire-themed AoE bird. Damage to All + Pain. Spreads fire damage across the entire party.
- **sd:** `171-4:171-4:170-3:170-3:171-2:171-2`
- **Items/keywords:**
  - `.i.k.pain` (Fire Spin — pain for burn damage over time)
  - `.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Heat Wave[nokeyword]` (top cleave faces = Heat Wave AoE)
  - `.i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict burn[nokeyword]` (bottom cleave faces = burn AoE)
- **Doc:** `.doc.The Inferno That Engulfs All`
- **Notes:** AoE bird. 4 cleave faces (4/4/2/2 pips) hit all heroes. 2 single-target mid faces (3 pips). Pain keyword means every hit stacks burn. The trio's area pressure — while Zapdos focuses one target, Moltres softens the entire party. Total pips: 18.

**Full definition string:**
```
Alpha.n.Moltres.hp.12.doc.The Inferno That Engulfs All.sd.171-4:171-4:170-3:170-3:171-2:171-2.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Heat Wave[nokeyword].i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict burn[nokeyword].img.PLACEHOLDER
```

#### Complete Fight String (Floor 12, Gen 3 Alternative — Legendary Birds):
```
ch.om12.fight.(
  Alpha.n.Articuno.hp.12.doc.The Frozen Wings of Winter.sd.170-3:170-3:118-4:118-4:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].i.topbot.facade.Ese118:0.img.PLACEHOLDER
  +Alpha.n.Zapdos.hp.12.doc.The Thunder That Splits the Sky.sd.170-5:170-5:170-4:170-4:170-2:170-2.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Thunder[nokeyword].img.PLACEHOLDER
  +Alpha.n.Moltres.hp.12.doc.The Inferno That Engulfs All.sd.171-4:171-4:170-3:170-3:171-2:171-2.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Heat Wave[nokeyword].i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict burn[nokeyword].img.PLACEHOLDER
).mn.LegendaryBirds
```
**Total encounter HP:** 12 + 12 + 12 = 36 (matches F12 budget of ~35-36, identical to Regi Trio, Swords of Justice, and Forces of Nature)

#### Trio Comparison (all F12 trio fights):

| Trio | Defensive Member | Offensive Member | AoE/Control Member | Total HP |
|------|-----------------|-----------------|-------------------|----------|
| Regi Trio | Registeel (shields+repel) | Regirock (heavy damage) | Regice (weaken+exert) | 36 |
| Legendary Birds | Articuno (shields+weaken) | Zapdos (cruel+first damage) | Moltres (AoE+pain burn) | 36 |
| Swords of Justice | Cobalion (steel shields) | Terrakion (heavy cruel) | Virizion (fast+exert) | 36 |
| Forces of Nature | Landorus (heavy+pain) | Thundurus (cruel thunder) | Tornadus (AoE+dodge) | 36 |

**Design note:** The Legendary Birds fight is selected randomly alongside the Regi Trio at Gen 3 Floor 12. Each run randomly picks either Birds or Regis, providing variety on repeated playthroughs. The Birds are slightly more offense-oriented (Zapdos is a harder-hitting DPS than Regirock, Moltres applies more AoE pressure than Regice) while the Regis are more defense-oriented (Registeel is a harder wall than Articuno). This gives the two fights distinct tactical feels despite the same total HP.

---

## PART 8: Legendary Dogs — Capture Items

---

### Legendary Dogs — Suicune, Entei, Raikou

**System:** Legendary summon items, using the same system as Ho-Oh (Rainbow Wing), Lugia (Silver Wing), Kyogre (Blue Orb), Groudon (Red Orb), Rayquaza (Jade Orb), Latias (Soul Dew), and Latios (Eon Flute). These are HERO summons — they fight on your side as temporary allies. They use hero-style Face IDs (15, 56, 103, etc.), NOT enemy-style (170/171).

**Flee mechanic:** Like all legendary summons, each dog flees on turn 7 (replaces all sides with "I flee"). This is the standard legendary summon behavior — powerful but temporary.

**Format reference:** Follows the `itempool.((hat.(replica.Thief.i.(all.(cast.sthief.abilitydata.(thief.sd.182-25:0:0:0:76-0:0.i.(mid.hat.egg.dragon.n.[NAME].doc.[DESC].doc.On turn 7[comma] replace all sides with "I flee".img.[DATA]` pattern from L115/L117 of textmod.txt.

---

#### Suicune — Item: Clear Bell

- **Item name:** Clear Bell
- **Item tier:** 7 (same tier as Latias Soul Dew / Latios Eon Flute)
- **Concept:** Water-themed legendary summon. Defensive/cleanse support. The Aurora Pokemon purifies and protects the party.
- **HP:** 10
- **sd:** `56-3:56-3:103-3:103-3:111-2:111-2`
  - Face 56 (Shield) x2 — Aurora Veil protective barrier
  - Face 103 (Heal) x2 — Purifying water heal
  - Face 111 (Heal Cleanse) x2 — Cleanse removes debuffs (signature ability: purification)
- **Items/keywords:** (none beyond base legendary summon structure)
- **Doc:** `.doc.The Aurora Pokemon. Purifies water and protects allies with crystalline shields.` `.doc.On turn 7[comma] replace all sides with "I flee".`
- **Notes:** Defensive/support legendary. Shield + Heal + Heal Cleanse provides party-wide sustain and debuff removal. Weaker offensively than Entei/Raikou but keeps the team alive. The cleanse is particularly valuable against poison/weaken-heavy bosses. Total pips: 16.

**Full definition string:**
```
itempool.((hat.(replica.Thief.i.(all.(cast.sthief.abilitydata.(thief.sd.182-25:0:0:0:76-0:0.i.(mid.hat.egg.dragon.n.Suicune.doc.The Aurora Pokemon. Purifies water and protects allies with crystalline shields.doc.On turn 7[comma] replace all sides with "I flee".sd.56-3:56-3:103-3:103-3:111-2:111-2.hp.10.img.PLACEHOLDER
```

---

#### Entei — Item: Flame Plate

- **Item name:** Flame Plate
- **Item tier:** 7
- **Concept:** Fire-themed legendary summon. Damage + AoE fire. The Volcano Pokemon erupts with devastating power.
- **HP:** 9
- **sd:** `15-4:15-4:34-3:34-3:15-3:15-3`
  - Face 15 (Damage) x2 — Sacred Fire single-target burst (4 pips)
  - Face 34 (Damage to All) x2 — Eruption AoE fire (3 pips)
  - Face 15 (Damage) x2 — Fire Fang follow-up (3 pips)
- **Items/keywords:**
  - `.i.k.heavy` (Sacred Fire — heavy hits)
- **Doc:** `.doc.The Volcano Pokemon. Erupts with sacred fire that scorches all enemies.` `.doc.On turn 7[comma] replace all sides with "I flee".`
- **Notes:** Offensive legendary. 4 single-target damage faces + 2 AoE faces. Heavy keyword adds extra impact. Hits hard both single-target and AoE. Lower HP (9) than Suicune (10) reflects glass cannon role. Total pips: 20.

**Full definition string:**
```
itempool.((hat.(replica.Thief.i.(all.(cast.sthief.abilitydata.(thief.sd.182-25:0:0:0:76-0:0.i.(mid.hat.egg.dragon.n.Entei.doc.The Volcano Pokemon. Erupts with sacred fire that scorches all enemies.doc.On turn 7[comma] replace all sides with "I flee".sd.15-4:15-4:34-3:34-3:15-3:15-3.hp.9.i.k.heavy.img.PLACEHOLDER
```

---

#### Raikou — Item: Zap Plate

- **Item name:** Zap Plate
- **Item tier:** 7
- **Concept:** Electric-themed legendary summon. Charged damage + speed. The Thunder Pokemon strikes with lightning speed and precision.
- **HP:** 9
- **sd:** `42-4:42-4:42-3:42-3:17-3:17-3`
  - Face 42 (Damage Charged) x2 — Thunder single-target charged burst (4 pips)
  - Face 42 (Damage Charged) x2 — Thunderbolt charged follow-up (3 pips)
  - Face 17 (Damage Engage) x2 — Extreme Speed engage strike (3 pips)
- **Items/keywords:** (none beyond base legendary summon structure — Charged and Engage are inherent to the face IDs)
- **Doc:** `.doc.The Thunder Pokemon. Strikes with lightning speed and devastating charged bolts.` `.doc.On turn 7[comma] replace all sides with "I flee".`
- **Notes:** Speed/burst legendary. 4 Charged damage faces build up for devastating hits. 2 Engage faces let Raikou act early and aggressively. Charged keyword on face 42 means damage ramps up over successive rolls. Lower HP (9) reflects aggressive playstyle. Total pips: 20.

**Full definition string:**
```
itempool.((hat.(replica.Thief.i.(all.(cast.sthief.abilitydata.(thief.sd.182-25:0:0:0:76-0:0.i.(mid.hat.egg.dragon.n.Raikou.doc.The Thunder Pokemon. Strikes with lightning speed and devastating charged bolts.doc.On turn 7[comma] replace all sides with "I flee".sd.42-4:42-4:42-3:42-3:17-3:17-3.hp.9.img.PLACEHOLDER
```

---

#### Legendary Dogs — Summary Table

| Dog | Item | Type | Role | HP | Key Faces | Total Pips | Flee |
|-----|------|------|------|----|-----------|------------|------|
| Suicune | Clear Bell | Water | Defensive/Cleanse Support | 10 | Shield (56) + Heal (103) + Heal Cleanse (111) | 16 | Turn 7 |
| Entei | Flame Plate | Fire | Damage + AoE | 9 | Damage (15) + Damage to All (34) + Heavy | 20 | Turn 7 |
| Raikou | Zap Plate | Electric | Charged Burst + Speed | 9 | Damage Charged (42) + Damage Engage (17) | 20 | Turn 7 |

#### Legendary Summon Roster (Complete):

| Legendary | Item | Tier | Role | Source |
|-----------|------|------|------|--------|
| Ho-Oh | Rainbow Wing | 8 | Revive support | Existing (L115) |
| Lugia | Silver Wing | 8 | Defensive support | Existing (L115) |
| Kyogre | Blue Orb | 8 | Water AoE | Existing (L117) |
| Groudon | Red Orb | 8 | Ground/Fire damage | Existing (L117) |
| Rayquaza | Jade Orb | 8 | Dragon AoE + buff removal | New (expansion) |
| Latias | Soul Dew | 7 | Shield/Heal/Dodge | New (expansion) |
| Latios | Eon Flute | 7 | Ranged/Cleave damage | New (expansion) |
| Suicune | Clear Bell | 7 | Shield/Heal/Cleanse | New (expansion) |
| Entei | Flame Plate | 7 | Damage/AoE + Heavy | New (expansion) |
| Raikou | Zap Plate | 7 | Charged/Engage burst | New (expansion) |

**Total legendary summons:** 10 (4 existing + 6 new)
