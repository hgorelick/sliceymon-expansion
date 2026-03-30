# Hero Designs Batch 3 — Exact Dice Stats

7 new hero lines + 1 redesign. All face IDs verified against SLICEYMON_AUDIT.md Key Face IDs table.

**Format key**: `sd=FACE_ID-PIPS:FACE_ID-PIPS:...` (6 faces for heroes, 5 faces for spells in abilitydata)
- Spells use 5 faces (abilitydata format): `sd.FACE-PIPS:0-0:0-0:0-0:76-N`
- Face 0 = Blank, Face 6 = Stasis/Blank

---

## 1. Pichu -> Pikachu -> Raichu (color x, P2, replaces Joltik L93)

**Template**: replica.Lost
**Speech**: Pichu!:Pi!:Piii | Pika!:Pikachu!:Pika pika! | Rai!:Raichu!:Rai rai!
**Doc**: (none)

### T1 Pichu: HP 4
- sd: `42-1:88-1:76:76:0:0`
- Faces: Charged 1, SU Charged 1, Mana, Mana, Blank, Blank
- Explanation: Baby electric mouse. 2 blanks (T1 standard). Charged faces build up damage potential. SU Charged is a strong burst but single-use at this tier. Two Mana faces fuel future spells.

### T2a Pikachu: HP 6
- sd: `42-2:42-2:88-2:76-1:76-1:0`
- Faces: Charged 2, Charged 2, SU Charged 2, Mana 1, Mana 1, Blank
- Explanation: Growing stronger. Two solid Charged faces, one big SU Charged burst. Mana with pips to fuel Thunderbolt at T3. Single blank keeps T2 honest.

### T2b Pikachu: HP 7
- sd: `42-1:42-1:76-2:76-2:88-1:0`
- Faces: Charged 1, Charged 1, Mana 2, Mana 2, SU Charged 1, Blank
- Explanation: Mana-focused variant. Trades raw damage for faster spell charging. Higher HP compensates for lower pip values.

### T3a Raichu: HP 9
- sd: `42-4:42-3:88-3:76-2:76-2:0`
- Faces: Charged 4, Charged 3, SU Charged 3, Mana 2, Mana 2, Blank
- Explanation: Electric burst DPS. Two big Charged hits, one devastating SU Charged (use it wisely). Mana generation for Thunderbolt spell. One blank at T3 is a minor weakness for a strong payoff kit.
- Spell — **Thunderbolt**: `abilitydata.(Fey.sd.42-4:0-0:0-0:0-0:76-4.img.IMG.n.Thunderbolt)`
  - 5 faces: Charged 4, targeting, targeting, targeting, Mana 4
  - Big charged damage spell that also generates mana for next cast

### T3b Raichu: HP 8
- sd: `42-4:42-4:88-4:88-4:76-2:76-2`
- Faces: Charged 4, Charged 4, SU Charged 4, SU Charged 4, Mana 2, Mana 2
- Explanation: Glass cannon variant. Low HP (8, T3 minimum) but strong burst damage. Two SU Charged at 4 pips = devastating alpha strike. All faces are active (no blanks). High risk, high reward. Pip values reduced from 5 to 4 to compensate for the 0-blank, all-active die — total 20 pips is at T3 ceiling but justified by the SU drawback (single-use faces are gone after first roll).
- Spell — **Thunder**: `abilitydata.(Fey.sd.88-5:0-0:0-0:0-0:76-4.img.IMG.n.Thunder)`
  - 5 faces: SU Charged 5, targeting x3, Mana 4
  - Massive single-use burst. Fits the "all-in" glass cannon theme. Pip reduced from 6 to 5 to match rebalanced die.

---

## 2. Weedle -> Kakuna -> Beedrill (color t, P2, replaces Burmy L87)

**Template**: replica.Lost
**Speech**: Wee!:Weedle!:dle? | [dot][dot][dot]:[i]Hardens | Bzzzz!:Beedrill!:Drill!
**Doc**: (none — pure physical, no spells)

### T1 Weedle: HP 4
- sd: `53-1:53-1:17-1:0:0:0`
- Faces: Poison 1, Poison 1, Engage 1, Blank, Blank, Blank
- Explanation: Weak little bug. 3 blanks (caterpillar phase). Two Poison Damage faces apply poison on hit. One Engage for speed. Low pips, low HP — classic weak T1.

### T2a Kakuna: HP 8
- sd: `56-3:56-3:56-2:56-2:65-1:0`
- Faces: Shield 3, Shield 3, Shield 2, Shield 2, Pristine Shield 1, Blank
- Explanation: COCOON PHASE — almost entirely defensive. Four Shield faces + one Pristine Shield. Like Shelgon's cocoon concept. High HP (8) for T2. Kakuna hardens and waits. One blank. No offense at all.

### T2b Kakuna: HP 7
- sd: `56-2:56-2:65-2:53-1:17-1:0`
- Faces: Shield 2, Shield 2, Pristine Shield 2, Poison 1, Engage 1, Blank
- Explanation: Slightly less defensive variant that retains minimal offensive capability. Still mostly shields but can poke with Poison and Engage. Choosing this path means slightly less safe cocoon but smoother transition to Beedrill.

### T3a Beedrill: HP 8
- sd: `53-3:53-3:17-3:17-3:46-2:46-2`
- Faces: Poison 3, Poison 3, Engage 3, Engage 3, Ranged 2, Ranged 2
- Explanation: Fast poison striker breaks out of cocoon. NO blanks — Beedrill is all offense. Poison stacks from two faces. Engage for priority hits. Ranged for safe back-row sniping. Pure physical DPS, no spell needed. The payoff for surviving the cocoon.

### T3b Beedrill: HP 9
- sd: `55-2:55-2:17-4:17-4:46-1:0`
- Faces: Poison Plague 2, Poison Plague 2, Engage 4, Engage 4, Ranged 1, Blank
- Explanation: Poison Plague variant. Plague spreads poison to adjacent enemies — AoE poison pressure. Higher Engage pips (4) for devastating priority damage. One blank, but higher HP (9) to compensate. Trades the clean 6-active-face kit for plague spreading + bigger Engage hits.

---

## 3. Riolu -> Lucario (color e, P2, NEW line)

**Template**: replica.Lost
**Speech**: Ri!:Riolu!:Olu? | Ri!:Riolu!:[i]trains hard | Luca!:Lucario!:[i]Aura is with me
**Doc**: (none at T1-T2; T3 gets spell)

**NOTE**: 2-stage Pokemon. Riolu at T1 and T2 (growing stronger), Lucario at T3.

### T1 Riolu: HP 5
- sd: `17-1:17-1:42-1:56-1:0:0`
- Faces: Engage 1, Engage 1, Charged 1, Shield 1, Blank, Blank
- Explanation: Young fighting pup. Two Engage faces for quick strikes. One Charged face stores up for bigger hits. One Shield (Steel typing defense). Two blanks standard for T1.

### T2a Riolu: HP 7
- sd: `17-2:17-2:42-2:42-2:63-1:0`
- Faces: Engage 2, Engage 2, Charged 2, Charged 2, Steel Shield 1, Blank
- Explanation: Training Riolu. Engage + Charged combo grows. Steel Shield (63) represents the Steel typing emerging. One blank at T2.

### T2b Riolu: HP 8
- sd: `17-1:17-1:42-1:63-2:63-2:0`
- Faces: Engage 1, Engage 1, Charged 1, Steel Shield 2, Steel Shield 2, Blank
- Explanation: Defensive training path. More Steel Shields, fewer damage pips. Higher HP (8). Riolu focusing on defense before evolution.

### T3a Lucario: HP 9
- sd: `17-3:42-3:42-3:41-2:63-2:0`
- Faces: Engage 3, Charged 3, Charged 3, Steel Damage 2, Steel Shield 2, Blank
- Explanation: Balanced Aura Warrior. Strong Charged hits (Aura-infused), Steel Damage for type coverage, Steel Shield for defense. Engage for priority. One blank keeps budget honest.
- Spell — **Aura Sphere**: `abilitydata.(Fey.sd.46-4:0-0:0-0:0-0:76-4.i.left.k.focus.img.IMG.n.Aura Sphere)`
  - 5 faces: Ranged 4 (with focus keyword = never misses), targeting x3, Mana 4
  - Ranged damage that "never misses" via the focus keyword. Thematic Lucario signature move.

### T3b Lucario: HP 8
- sd: `17-4:17-4:42-4:42-4:41-3:41-3`
- Faces: Engage 4, Engage 4, Charged 4, Charged 4, Steel Damage 3, Steel Damage 3
- Explanation: All-offense Mega Lucario fantasy. No blanks, no shields. Six active damage faces. Lower HP (8). Pure glass cannon aura warrior. Trades survival for devastating output.
- Spell — **Close Combat**: `abilitydata.(Fey.sd.174-5:0-0:0-0:0-0:76-3.img.IMG.n.Close Combat)`
  - 5 faces: Defy 5, targeting x3, Mana 3
  - Defy = damage that ignores shields. Close Combat's signature "hit through defense" feel.

---

## 4. Togepi -> Togetic -> Togekiss (color r, P1, replaces Darumaka L39)

**Template**: replica.Statue (Darumaka used replica.Statue; healer focus works with Statue since Ralts uses Statue for spell support)
**Speech**: Toge!:Piii!:Priii! | Togetic!:Tic! | Kiss!:Togekiss!:[i]serene
**Doc**: (none)

### T1 Togepi: HP 5
- sd: `106-1:103-1:61-1:0:0:0`
- Faces: Heal Rescue 1, Heal 1, Shield ManaGain 1, Blank, Blank, Blank
- Explanation: Baby fairy. Three blanks (Togepi is an egg baby — even weaker than standard T1). One Heal Rescue (saves low-HP allies), one basic Heal, one Shield ManaGain. Tiny support contribution, grows into a powerhouse healer.

### T2a Togetic: HP 7
- sd: `106-2:109-1:109-1:61-2:103-1:0`
- Faces: Heal Rescue 2, Heal Cleave 1, Heal Cleave 1, Shield ManaGain 2, Heal 1, Blank
- Explanation: Growing fairy healer. Heal Rescue grows. Two Heal Cleave faces = healing spreads to neighbors. Shield ManaGain fuels spells. One blank.

### T2b Togetic: HP 6
- sd: `106-2:106-2:61-1:61-1:109-1:0`
- Faces: Heal Rescue 2, Heal Rescue 2, Shield ManaGain 1, Shield ManaGain 1, Heal Cleave 1, Blank
- Explanation: Double Rescue variant. Two Rescue heals for clutch saves. Double ManaGain shields fuel spells faster. Lower HP (6) — trades bulk for focused rescue healing.

### T3a Togekiss: HP 9
- sd: `106-3:106-3:109-3:109-3:61-2:0`
- Faces: Heal Rescue 3, Heal Rescue 3, Heal Cleave 3, Heal Cleave 3, Shield ManaGain 2, Blank
- Explanation: Premier targeted healer. Two Rescue faces at 3 pips = strong clutch healing for lowest-HP ally. Two Heal Cleave at 3 = sustained party healing. Shield ManaGain provides mana + protection. One blank at T3 acceptable for a pure support.
- Spell — **Wish**: `abilitydata.(Fey.sd.107-4:0-0:0-0:0-0:76-5.img.IMG.n.Wish)`
  - 5 faces: Heal All 4, targeting x3, Mana 5
  - Heals entire party. The signature Togekiss support move.

### T3b Togekiss: HP 8
- sd: `106-4:109-4:109-4:61-3:61-3:123`
- Faces: Heal Rescue 4, Heal Cleave 4, Heal Cleave 4, Shield ManaGain 3, Shield ManaGain 3, Dodge
- Explanation: Serene Grace variant. No blanks — every face does something. Dodge (123) represents Serene Grace flinch avoidance. Bigger heal pips (4) across the board. Double Shield ManaGain 3 = excellent mana generation. Lower HP than T3a but more consistent.
- Spell — **Dazzling Gleam**: `abilitydata.(Fey.sd.34-3:0-0:0-0:0-0:76-4.img.IMG.n.Dazzling Gleam)`
  - 5 faces: Damage to All 3, targeting x3, Mana 4
  - Offensive spell on a healer! Dazzling Gleam hits all enemies. Gives Togekiss a damage option when healing isn't needed. Low damage (3 pips) keeps it support-focused.

---

## 5. Cleffa -> Clefairy -> Clefable (color u, P2, replaces Tinkatink L89)

**Template**: replica.Lost
**Speech**: Cleff!:Fa! | Fairy!:Clefairy!:Clef clef! | Clefable!:[i]Moonlight:[i]wiggles fingers
**Doc**: (none)

### T1 Cleffa: HP 4
- sd: `27-1:110-1:56-1:0:0:0`
- Faces: Copycat 1, Heal Regen 1, Shield 1, Blank, Blank, Blank
- Explanation: Baby fairy star. Three blanks. One Copycat face (Metronome theme — copies the keyword of the face above it on the die). One Heal Regen (sustained healing). One Shield. Minimal but thematic.

### T2a Clefairy: HP 7
- sd: `27-2:27-1:110-2:56-2:56-1:0`
- Faces: Copycat 2, Copycat 1, Heal Regen 2, Shield 2, Shield 1, Blank
- Explanation: Metronome grows. Two Copycat faces make Clefairy unpredictable (copying different keywords each roll). Regen provides sustained healing. Shields for survivability. One blank.

### T2b Clefairy: HP 8
- sd: `27-1:110-2:110-1:56-2:56-2:0`
- Faces: Copycat 1, Heal Regen 2, Heal Regen 1, Shield 2, Shield 2, Blank
- Explanation: Defensive support variant. Less Copycat randomness, more consistent healing (double Regen) and shielding. Higher HP (8). The "reliable Clefairy" path.

### T3a Clefable: HP 9
- sd: `27-3:27-3:110-3:110-3:56-2:0`
- Faces: Copycat 3, Copycat 3, Heal Regen 3, Heal Regen 3, Shield 2, Blank
- Explanation: Metronome master. Two big Copycat faces at 3 pips — wild, adaptive damage that copies the best keyword on the die. Double Regen 3 for sustained team healing. Shield and one blank. The "wildcard support" fantasy.
- Spell — **Metronome**: `abilitydata.(Fey.sd.27-4:0-0:0-0:0-0:76-4.img.IMG.n.Metronome)`
  - 5 faces: Copycat 4, targeting x3, Mana 4
  - A spell that copies — thematically perfect for Metronome. High Copycat pips for big adaptive damage.

### T3b Clefable: HP 10
- sd: `27-2:27-2:110-4:110-4:56-3:56-3`
- Faces: Copycat 2, Copycat 2, Heal Regen 4, Heal Regen 4, Shield 3, Shield 3
- Explanation: Moonlight tank variant. Higher HP (10), big Regen (4 pips = strong sustain), solid shields. Still has Copycat for flavor but leans into durable support. No blanks, all active faces.
- Spell — **Moonlight**: `abilitydata.(Fey.sd.107-3:0-0:0-0:0-0:76-5.img.IMG.n.Moonlight)`
  - 5 faces: Heal All 3, targeting x3, Mana 5
  - Party-wide healing under moonlight. Lower damage than Metronome spell but heals the whole team.

---

## 6. Nidoran-F -> Nidorina -> Nidoqueen (color n, P1, replaces Applin L31)

**Template**: replica.Statue (same as Applin)
**Speech**: Nido!:Ran! | Rina!:Nidorina! | Queen!:Nidoqueen!:[i][sin]rumble
**Doc**: `[plus] Start [green]Poisoned[cu] 1` (T1), `[plus] Immune to [green]Poison` (T2), `[plus] Start [green]Poisoned[cu] two` (T3a)

**IMPORTANT**: This hero INHERITS Tyranitar's current poison design philosophy:
- T1: Start Poisoned 1 (facade + keyword: `k.poison`, `k.acidic`, facade `bas12:60:-40:-20`)
- T2: Immune to Poison (`i.antivenom`)
- T3a: Start Poisoned 2, Carrier item, poison synergy (regen to offset, facades for poison aura)
- T3b: Offensive variant without poison-start (duel/ego)

### T1 Nidoran-F: HP 6
- sd: `39-1:39-1:56-1:56-1:0:12-0`
- Faces: Heavy 1, Heavy 1, Shield 1, Shield 1, Blank, Self-damage Cantrip 0
- Keywords: `k.poison`, `k.acidic`, `i.rightmost`
- Facades: `facade.bas12:60:-40:-20` (self-poison aura), `facade.Eme53:0` (poison on engage), `facade.pos121:48:-30:-10` (poison position)
- Triggers: `i.self.Bottom Poison^1/1`
- Explanation: Direct copy of Larvitar's T1 poison design. Heavy damage + shields + self-damage cantrip (poison ticks). Starts Poisoned 1. Lower HP than Larvitar (6 vs 8) because Nidoran is smaller — but same poison mechanics. The acidic keyword means poison damage is beneficial (synergy).

### T2a Nidorina: HP 8
- sd: `39-2:39-2:119-2:119-2:56-2:0`
- Faces: Heavy 2, Heavy 2, Shield Repel 2, Shield Repel 2, Shield 2, Blank
- Keywords: `i.antivenom` (immune to poison), `k.minusera`
- Facades: `facade.Che20:0`
- Doc: `[plus] Immune to [green]Poison`
- Explanation: Cocoon/growth phase. Immune to Poison = the poison from T1 no longer hurts her (she's adapted). Shield Repel (119) represents Poison Point — enemies that attack her get repelled. Heavy damage grows. Defensive core.

### T2b Nidorina: HP 7
- sd: `39-2:39-2:53-1:53-1:119-1:0`
- Faces: Heavy 2, Heavy 2, Poison 1, Poison 1, Shield Repel 1, Blank
- Keywords: `i.antivenom`
- Doc: `[plus] Immune to [green]Poison`
- Explanation: Offensive variant. Keeps Poison damage faces (actively poisoning enemies). Less shielding. Still immune to poison. Lower HP.

### T3a Nidoqueen: HP 11
- sd: `39-3:39-2:119-3:119-3:56-2:12-2`
- Faces: Heavy 3, Heavy 2, Shield Repel 3, Shield Repel 3, Shield 2, Self-damage Cantrip 2
- Keywords: `k.regen`, `k.poison`, `k.acidic`
- Facades: `facade.Eme53:90:20:0` (poison aura), `facade.bas12:60:-40:-20` (poison self), `facade.pos121:48:-30:-10`
- Item: `i.t.Carrier`
- Doc: `[plus] Start [green]Poisoned[cu] two`
- Explanation: Full poison tank. Inherits Tyranitar T3a's exact poison design: Start Poisoned 2, Regen to offset, Carrier item for team utility. Shield Repel at 3 pips = strong Poison Point defense. Heavy damage for offense. Self-damage Cantrip 2 ticks poison. The tanky poison queen.
- Spell — **Earth Power**: `abilitydata.(Fey.sd.39-4:0-0:0-0:0-0:76-4.img.IMG.n.Earth Power)`
  - 5 faces: Heavy 4, targeting x3, Mana 4
  - Big ground-type heavy damage spell.

### T3b Nidoqueen: HP 8
- sd: `39-4:39-4:119-4:119-4:53-2:0`
- Faces: Heavy 4, Heavy 4, Shield Repel 4, Shield Repel 4, Poison 2, Blank
- Keywords: `k.duel`, `k.ego`
- Facades: `facade.dan6:55:0:10` (duel), `facade.Che19:0` (ego)
- Explanation: Aggressive Nidoqueen. Mirrors Tyranitar T3b's duel/ego keywords. No poison-start, no Carrier. Instead: huge Heavy hits (4 pips), strong Repel shields, and active Poison damage. Duel = 1v1 focus, Ego = stat boost when winning. Lower HP, more kill pressure.

---

## 7. Nidoran-M -> Nidorino -> Nidoking (color n, P2, replaces Turtwig-slot L75)

**Template**: replica.Lost
**Speech**: Nido!:Ran! | Rino!:Nidorino!:[i]charges | KING!:Nidoking!:[i][sin]rumble
**Doc**: (none — pure physical, no spells)

### T1 Nidoran-M: HP 5
- sd: `55-1:17-1:30-1:0:0:0`
- Faces: Poison Plague 1, Engage 1, Cruel 1, Blank, Blank, Blank
- Explanation: Aggressive little poison horn. Three blanks (T1 standard). Poison Plague spreads poison to neighbors. Engage for speed. Cruel for bonus damage. Low pips but the face TYPES are strong — Sheer Force identity from the start.

### T2a Nidorino: HP 7
- sd: `55-2:55-1:17-2:17-2:30-1:0`
- Faces: Poison Plague 2, Poison Plague 1, Engage 2, Engage 2, Cruel 1, Blank
- Explanation: Growing aggression. Double Plague for poison spread. Double Engage for speed priority. Cruel for Sheer Force damage bonus. One blank. Pure offense.

### T2b Nidorino: HP 8
- sd: `55-1:17-2:17-1:30-2:30-2:0`
- Faces: Poison Plague 1, Engage 2, Engage 1, Cruel 2, Cruel 2, Blank
- Explanation: Sheer Force variant. Less Plague, more raw Cruel damage (2 pips each = strong bonus hits). Higher HP (8). Trades poison spread for pure physical brutality.

### T3a Nidoking: HP 10
- sd: `55-3:55-3:17-3:17-3:30-3:0`
- Faces: Poison Plague 3, Poison Plague 3, Engage 3, Engage 3, Cruel 3, Blank
- Explanation: Poison King. Double Plague at 3 pips = heavy poison spreading across enemy team. Double Engage 3 = fast priority strikes. Cruel 3 = Sheer Force bonus damage. One blank at T3. Balanced offensive powerhouse. No spell — pure physical Nidoking.

### T3b Nidoking: HP 8
- sd: `55-3:55-3:30-3:30-3:17-4:17-4`
- Faces: Poison Plague 3, Poison Plague 3, Cruel 3, Cruel 3, Engage 4, Engage 4
- Explanation: Maximum Sheer Force. No blanks, all six faces active. Lower HP (8) — glass cannon. Plague 3 = strong poison spread. Cruel 3 = significant bonus damage. Engage 4 = priority strikes that hit hard. The "delete everything" variant. Total 20 pips is at the T3 ceiling but justified by no shields, no heals, and low HP.

---

## 8. REDESIGN: Larvitar -> Tyranitar (color h, EXISTING L21)

**Template**: replica.Statue (keep same)
**Speech**: KEEP EXISTING — `Lar!:Tar!:baba!` (Larvitar), `[dot][dot][dot]` (Pupitar), `Graa!:Tar!` (T3a), `GRA!:TAR!:[i][sin]rumble` (T3b)

### What's Being REMOVED
- All poison mechanics: `k.poison`, `k.acidic` keywords
- Poison facades: `facade.Eme53:0`, `facade.pos121:48:-30:-10`
- Self-poison trigger: `i.self.Bottom Poison^1/1`
- Doc `[plus] Start [green]Poisoned[cu] 1` and `[plus] Start [green]Poisoned[cu] two`
- Doc `[plus] Immune to [green]Poison` from Pupitar
- `i.antivenom` from Pupitar
- `k.regen` from T3a (was offsetting poison)
- `i.t.Carrier` item from T3a (was poison synergy)
- Self-damage Cantrip face (12) from T1 and T3a
- Face 45 (Damage Era) from T2a Pupitar

### What's Being ADDED
- Rock/Dark identity: Heavy (39) + Cruel (30) + Steel Shield (63)
- Sandstorm facade (modeled on Gigalith L73): `facade.the32:0` on hat + relevant keywords
- T3 doc: `[plus] At the end of each turn[comma] 1 damage to all heroes and monsters` (Sandstorm — identical to Gigalith's wording pattern)
- T2+ gets `k.minusflesh` (Gigalith pattern — "reduce damage taken by 1" equivalent for Sandstorm's sand armor)

### T1 Larvitar: HP 8 (unchanged)
- sd: `39-1:39-1:63-1:63-1:0:0`
- Faces: Heavy 1, Heavy 1, Steel Shield 1, Steel Shield 1, Blank, Blank
- Doc: (removed — no more poison start)
- Keywords: REMOVED `k.poison`, `k.acidic`. No special keywords at T1.
- Facades: REMOVED all poison facades. No facades at T1.
- Explanation: Clean Rock/Dark baby. Two Heavy damage faces (rock), two Steel Shields (dark-type toughness). Two blanks (standard T1). Simple and honest. HP 8 stays — Larvitar is bulky.

### T2a Pupitar: HP 12 (unchanged)
- sd: `39-2:39-2:63-3:63-3:30-1:0`
- Faces: Heavy 2, Heavy 2, Steel Shield 3, Steel Shield 3, Cruel 1, Blank
- Keywords: `k.minusflesh` (sand armor — reduce damage taken by 1)
- Facades: `facade.the32:0` (Sandstorm visual, like Gigalith's hat.slate)
- Doc: `[plus] Reduce damage taken by 1`
- Explanation: Pupitar hardens into a rock cocoon. High HP (12), big Steel Shields (3 pips), and the new minusflesh keyword = extremely tanky. Cruel (30) face introduces the Dark-type damage. One blank. Sandstorm facade starts here.

### T2b Pupitar: HP 12 (keep same HP as T2a — Pupitar is a cocoon, always tanky)
- sd: `39-3:39-3:63-2:63-2:0:0`
- Faces: Heavy 3, Heavy 3, Steel Shield 2, Steel Shield 2, Blank, Blank
- Keywords: `k.minusflesh`
- Facades: `facade.the32:0`
- Doc: `[plus] Reduce damage taken by 1`
- Explanation: Offensive Pupitar variant. Trades one shield pip for more Heavy damage (3 pips). Two blanks but minusflesh + high HP makes it survivable. No Cruel yet — saving Dark-type burst for Tyranitar.

### T3a Tyranitar: HP 13 (unchanged)
- sd: `39-4:39-3:63-3:63-3:30-2:30-1`
- Faces: Heavy 4, Heavy 3, Steel Shield 3, Steel Shield 3, Cruel 2, Cruel 1
- Keywords: `k.minusflesh`
- Facades: `facade.the32:0` (Sandstorm hat)
- Item: `i.t.gnoll` (keep — gnoll is generic, not poison-specific)
- Doc: `[plus] Reduce damage taken by 1[n][plus] At the end of each turn[comma] 1 damage to all heroes and monsters`
- Explanation: Mountain Crusher. No blanks — all six faces active at T3. Two big Heavy hits (Rock), two big Steel Shields (armor), two Cruel hits (Dark). Sandstorm facade does 1 passive damage to all enemies AND heroes each turn (same as Gigalith). HP 13 = tankiest in the game alongside Gyarados. A bruiser that passively grinds enemies down while smashing them with Heavy/Cruel.

### T3b Tyranitar: HP 9 (unchanged)
- sd: `30-4:30-4:39-4:39-4:63-3:0`
- Faces: Cruel 4, Cruel 4, Heavy 4, Heavy 4, Steel Shield 3, Blank
- Keywords: `k.duel`, `k.ego`
- Facades: `facade.dan6:55:0:10` (duel), `facade.Che19:0` (ego)
- Doc: (none — no Sandstorm on this variant)
- Explanation: Dark Tyranitar duel variant. KEEPS the existing duel/ego keywords from the current T3b — those are Rock/Dark appropriate (a Tyranitar that challenges foes 1v1). Big Cruel (4) and Heavy (4) pips = devastating single-target damage. Steel Shield 3 for defense. One blank. Lower HP (9) but high damage output. No Sandstorm — this variant is about raw power, not environmental damage. Total 19 pips is within T3 budget (14-22), with duel/ego providing conditional bonus damage on top.

---

## Cross-Reference Validation

### Balance Check: Pips per Tier

| Hero | T1 Total Pips | T1 Blanks | T1 HP | T3a Total Pips | T3a Blanks | T3a HP |
|------|--------------|-----------|-------|----------------|------------|--------|
| Pichu/Raichu | 3 | 2 | 4 | 16 | 1 | 9 |
| Weedle/Beedrill | 3 | 3 | 4 | 16 | 0 | 8 |
| Riolu/Lucario | 4 | 2 | 5 | 13 | 1 | 9 |
| Togepi/Togekiss | 3 | 3 | 5 | 14 | 1 | 9 |
| Cleffa/Clefable | 3 | 3 | 4 | 15 | 1 | 9 |
| Nidoran-F/Nidoqueen | 4 | 1(+cantrip) | 6 | 15 | 0(+cantrip) | 11 |
| Nidoran-M/Nidoking | 3 | 3 | 5 | 15 | 1 | 10 |
| Larvitar/Tyranitar (new) | 4 | 2 | 8 | 16 | 0 | 13 |

### Comparison to Existing Heroes

| Reference | T1 Pips | T1 Blanks | T1 HP | T3a Pips | T3a HP |
|-----------|---------|-----------|-------|----------|--------|
| Tinkatink (being replaced) | 6 | 1(face 0) | 5 | 17 | 10 |
| Joltik (being replaced) | 5 | 0 | 4 | 22 | 9 |
| Darumaka (being replaced) | 9 | 0 | 6 | 18 | 10 |
| Scyther (existing DPS) | 5 | 2 | 5 | 10+ | 8 |
| Ralts (existing spell) | 3 | 2 | 3 | 12 | 9 |
| Larvitar current | 4 | 1(+cantrip) | 8 | 12 | 13 |
| Happiny (existing healer) | 11 | 0 | 9 | 18 | 12 |

### Premium Keyword Budget

| Hero | Keywords Used | Notes |
|------|-------------|-------|
| Pichu line | Charged, SU Charged | SU is premium but offset by blanks |
| Weedle line | Engage, Poison, Plague | Plague only at T3b |
| Riolu line | Engage, Charged, Steel, Focus (spell only) | Focus restricted to spell |
| Togepi line | Rescue, Cleave (heal), ManaGain, Dodge (T3b only) | Support keywords, not DPS |
| Cleffa line | Copycat, Regen | Copycat is unique, not traditionally premium |
| Nidoran-F line | Poison, Acidic, Repel, Carrier (T3a), Duel/Ego (T3b) | Inherited from Tyranitar |
| Nidoran-M line | Plague, Engage, Cruel, Duel/Ego (T3b) | Offensive budget |
| Tyranitar redesign | Cruel, Heavy, Steel, MinusFlesh, Duel/Ego (T3b) | Clean Rock/Dark kit |

No hero uses Rampage or Revive (reserved as premium). QuadUse is absent (reserved for Machamp in batch 1). **Note:** Nidoran-F (T1) and Nidoqueen (T3a) use face 12 (Self-damage Cantrip) — this is inherited directly from the existing Larvitar design and is not a new premium keyword addition. The Self-damage Cantrip on these faces is a poison-tick mechanic (0-2 pips) that damages the hero, not a traditional Cantrip power spike.
