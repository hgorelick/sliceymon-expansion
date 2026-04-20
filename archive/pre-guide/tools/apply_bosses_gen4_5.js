#!/usr/bin/env node

/**
 * apply_bosses_gen4_5.js
 *
 * Adds Gen 4 and Gen 5 boss fights to textmod_expanded.txt.
 * Designed to run AFTER apply_bosses_gen3.js (both modify textmod_expanded.txt
 * sequentially; this script uses findIndex rather than hard-coded line numbers).
 *
 * === Gen 4 Bosses ===
 *   - Floor 4:  Reuses existing Quagsire (no change needed)
 *   - Floor 8:  Reuses existing Exeggutor (no change needed)
 *   - Floor 12: Palkia (HP 25) + Bronzong (HP 8) + Spatial Rift (HP 5)
 *   - Floor 16: Dialga (HP 25) + Bronzong (HP 8) + Temporal Anomaly (HP 5)
 *   - Floor 20: Arceus (4-phase type shift: Normal→Fire→Steel→Dragon)
 *
 * === Gen 5 Bosses ===
 *   - Floor 4:  Reuses existing Quagsire (no change needed)
 *   - Floor 8:  Serperior + Emboar + Samurott (Unova Starters)
 *   - Floor 12: Random: Swords of Justice OR Forces of Nature
 *   - Floor 16: Random: Reshiram OR Zekrom
 *   - Floor 20: Kyurem (2-phase: Base → Black Kyurem or White Kyurem)
 *
 * === Variable Routing ===
 *   Existing:
 *     XV1/YV1 = Gen 6 (bX/bY checks)
 *     BV1/LV1 = Gen 7 (bB/bL checks)
 *     GV1/HV1 = Gen 3 (bG/bH checks)  — added by apply_bosses_gen3.js
 *   New:
 *     PV1/TV1 = Gen 4 (bP/bT checks)
 *     UV1/WV1 = Gen 5 (bU/bW checks)
 *
 *   When a gen is selected, BOTH variables are set. Each gen gets two
 *   conditional line slots (bP/bT, bU/bW) per floor. For floors with random
 *   selection, the two slots hold different fights (the engine picks one).
 *   For floors without random selection, both slots hold the same fight.
 *
 * === Floor 4/8 Shared Bosses ===
 *   Gen 4 Floor 4 and 8 reuse existing bosses (Quagsire, Exeggutor) — no additions.
 *   Gen 5 Floor 4 reuses Quagsire — no addition needed.
 *   Gen 5 Floor 8 adds a new Unova Starters fight to the ch.om8 line.
 */

const fs = require('fs');
const path = require('path');

// --- Paths ---
const TEXTMOD_PATH = path.join(__dirname, '..', 'textmod_expanded.txt');
const SPRITES_PATH = path.join(__dirname, 'sprite_encodings.json');

// --- Load sprite encodings ---
const sprites = JSON.parse(fs.readFileSync(SPRITES_PATH, 'utf8'));

/**
 * Helper: get sprite or PLACEHOLDER
 */
function spr(name) {
  const s = sprites[name];
  if (!s) {
    console.warn(`WARNING: No sprite found for "${name}" — using PLACEHOLDER`);
    return 'PLACEHOLDER';
  }
  return s;
}

// =============================================================================
// GEN 4 BOSS FIGHT DEFINITIONS
// =============================================================================

/**
 * Floor 12 Gen 4: Palkia + Bronzong + Spatial Rift
 * Total HP: 25 + 8 + 5 = 38
 */
function buildFloor12Palkia() {
  const spatialRift = `Slimelet.n.Spatial Rift.hp.5.doc.A Tear in Reality.sd.171-1:171-1:171-1:171-1.i.k.pain.img.${spr('Palkia')}`;
  const bronzong = `Wolf.n.Bronzong.hp.8.doc.Warps Time and Space.sd.118-3:118-3:118-2:118-2:170-1:170-1.i.k.stasis.i.left2.facade.Ese118:0.img.${spr('Dialga')}`;
  const palkia = `Alpha.n.Palkia.hp.25.doc.Master of Space.sd.170-5:170-5:171-3:171-3:170-4:170-4.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Spatial Rend[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]distort space[nokeyword].img.${spr('Palkia')}`;

  return `(${spatialRift}+${bronzong}+${palkia})`;
}

/**
 * Floor 16 Gen 4: Dialga + Bronzong + Temporal Anomaly
 * Total HP: 25 + 8 + 5 = 38
 */
function buildFloor16Dialga() {
  const temporalAnomaly = `Slimelet.n.Temporal Anomaly.hp.5.doc.A Fracture in Time.sd.171-2:171-2:170-1:170-1.i.k.stasis.i.k.exert.img.${spr('Dialga')}`;
  const bronzong = `Wolf.n.Bronzong.hp.8.doc.Keeper of Temporal Order.sd.118-3:118-3:118-2:118-2:170-2:170-2.i.k.stasis.i.left2.facade.Ese118:0.img.${spr('Dialga')}`;
  const dialga = `Dragon.n.Dialga.hp.25.doc.Master of Time.sd.170-6:170-6:171-4:171-4:170-3:170-3.i.k.exert.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Roar of Time[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword].img.${spr('Dialga')}`;

  return `(${temporalAnomaly}+${bronzong}+${dialga})`;
}

/**
 * Floor 20 Gen 4: Arceus (4-phase type shift)
 * Phase 1: Normal (HP 12) → Phase 2: Fire (HP 10) → Phase 3: Steel (HP 12) → Phase 4: Dragon (HP 8)
 * Total HP: 42
 *
 * Uses nested .triggerhpdata.() for phase transitions (Necrozma/Deoxys pattern).
 */
function buildFloor20Arceus() {
  const arceusImg = spr('Arceus');

  const arceus = `Basalt.n.Arceus.hp.12.doc.The Alpha Pokemon — Origin of All.sd.170-5:170-5:171-3:171-3:118-3:118-3.i.k.rite.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Judgment[nokeyword]` +
    `.i.triggerhpdata.(lost.sd.171-5:171-5:171-4:171-4:170-3:170-3.i.k.cruel.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]fire Judgment[nokeyword].doc.Fire Type — Everything Burns.hp.10.img.${arceusImg}` +
    `.i.triggerhpdata.(lost.sd.118-6:118-6:118-5:118-5:170-4:170-4.i.k.stasis.i.left2.facade.Ese118:0.i.topbot.facade.Ese118:0.doc.Steel Type — Impervious.hp.12.img.${arceusImg}` +
    `.i.triggerhpdata.(lost.sd.170-8:170-8:170-6:170-6:171-4:171-4.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Dragon Judgment[nokeyword].doc.Dragon Type — Final Judgment.hp.8.img.${arceusImg}` +
    `)` +
    `)` +
    `)` +
    `.img.${arceusImg}`;

  return `(${arceus})`;
}

// =============================================================================
// GEN 5 BOSS FIGHT DEFINITIONS
// =============================================================================

/**
 * Floor 8 Gen 5: Serperior + Emboar + Samurott (Unova Starters)
 * Total HP: 12 + 12 + 12 = 36
 */
function buildFloor8UnovaStarters() {
  const serperior = `Alpha.n.Serperior.hp.12.doc.The Regal Serpent.sd.170-3:170-3:118-3:118-3:118-2:118-2.i.k.rite.i.topbot.facade.Ese118:0.img.${spr('Serperior')}`;
  const emboar = `Alpha.n.Emboar.hp.12.doc.The Blazing Boar.sd.170-4:170-4:171-3:171-3:170-2:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Flare Blitz[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] rampage[nokeyword].img.${spr('Emboar')}`;
  const samurott = `Alpha.n.Samurott.hp.12.doc.The Seamitar Warrior.sd.170-3:170-3:171-2:171-2:170-2:170-2.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Razor Shell[nokeyword].img.${spr('Samurott')}`;

  return `(${serperior}+${emboar}+${samurott})`;
}

/**
 * Floor 12 Gen 5 Path A: Swords of Justice
 * Cobalion + Terrakion + Virizion (HP 12 each = 36 total)
 */
function buildFloor12SwordsOfJustice() {
  const cobalion = `Alpha.n.Cobalion.hp.12.doc.The Iron Will.sd.170-3:170-3:118-4:118-4:170-2:170-2.i.k.stasis.i.topbot.facade.Ese118:0.img.${spr('Cobalion')}`;
  const terrakion = `Alpha.n.Terrakion.hp.12.doc.The Cavern Champion.sd.170-5:170-5:170-4:170-4:170-2:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].img.${spr('Terrakion')}`;
  const virizion = `Alpha.n.Virizion.hp.12.doc.The Forest Guardian.sd.170-4:170-4:170-3:170-3:158-2:158-2.i.k.first.i.k.exert.img.${spr('Virizion')}`;

  return `(${cobalion}+${terrakion}+${virizion})`;
}

/**
 * Floor 12 Gen 5 Path B: Forces of Nature
 * Tornadus + Thundurus + Landorus (HP 12 each = 36 total)
 */
function buildFloor12ForcesOfNature() {
  const tornadus = `Alpha.n.Tornadus.hp.12.doc.The Cyclone.sd.171-3:171-3:170-3:170-3:0:0.i.k.exert.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]hurricane[nokeyword].i.right2.facade.bas170:0:0:0#sidesc.dodge[green] [n]tailwind[nokeyword].img.${spr('Tornadus')}`;
  const thundurus = `Alpha.n.Thundurus.hp.12.doc.The Bolt Strike.sd.170-5:170-5:170-4:170-4:170-2:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Thunder[nokeyword].img.${spr('Thundurus')}`;
  const landorus = `Alpha.n.Landorus.hp.12.doc.The Abundant Land.sd.170-4:170-4:171-3:171-3:118-2:118-2.i.k.pain.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Earthquake[nokeyword].img.${spr('Landorus')}`;

  return `(${tornadus}+${thundurus}+${landorus})`;
}

/**
 * Floor 16 Gen 5 Version A: Reshiram + Zekrom Echo
 * Total HP: 25 + 8 = 33
 */
function buildFloor16Reshiram() {
  const zekromEcho = `Slimelet.n.Zekrom Echo.hp.8.doc.Zekrom's Thunder Echoes.sd.171-1:171-1:171-1:171-1.i.k.pain.img.${spr('Zekrom')}`;
  const reshiram = `Dragon.n.Reshiram.hp.25.doc.The Vast White Dragon of Truth.sd.171-5:171-5:170-4:170-4:171-3:171-3.i.k.cruel.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Blue Flare[nokeyword].i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword].img.${spr('Reshiram')}`;

  return `(${zekromEcho}+${reshiram})`;
}

/**
 * Floor 16 Gen 5 Version B: Zekrom + Reshiram Echo
 * Total HP: 25 + 8 = 33
 */
function buildFloor16Zekrom() {
  const reshiramEcho = `Slimelet.n.Reshiram Echo.hp.8.doc.Reshiram's Flames Linger.sd.171-1:171-1:171-1:171-1.i.k.pain.img.${spr('Reshiram')}`;
  const zekrom = `Dragon.n.Zekrom.hp.25.doc.The Deep Black Dragon of Ideals.sd.170-6:170-6:170-5:170-5:171-3:171-3.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Bolt Strike[nokeyword].i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Fusion Bolt[nokeyword].img.${spr('Zekrom')}`;

  return `(${reshiramEcho}+${zekrom})`;
}

/**
 * Floor 20 Gen 5 Black Kyurem variant: Base Kyurem → Black Kyurem
 * Total HP: 15 + 20 = 35
 *
 * Uses .triggerhpdata.() for phase transition.
 */
function buildFloor20BlackKyurem() {
  const kyuremImg = spr('Kyurem');

  const kyurem = `Basalt.n.Kyurem.hp.15.doc.The Boundary Pokemon — Incomplete.sd.170-4:170-4:171-3:171-3:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]` +
    `.i.triggerhpdata.(lost.sd.170-7:170-7:170-5:170-5:171-4:171-4.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Freeze Shock[nokeyword].doc.Black Kyurem — Absolute Power.hp.20.img.${kyuremImg}` +
    `)` +
    `.img.${kyuremImg}`;

  return `(${kyurem})`;
}

/**
 * Floor 20 Gen 5 White Kyurem variant: Base Kyurem → White Kyurem
 * Total HP: 15 + 20 = 35
 *
 * Uses .triggerhpdata.() for phase transition.
 */
function buildFloor20WhiteKyurem() {
  const kyuremImg = spr('Kyurem');

  const kyurem = `Basalt.n.Kyurem.hp.15.doc.The Boundary Pokemon — Incomplete.sd.170-4:170-4:171-3:171-3:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]` +
    `.i.triggerhpdata.(lost.sd.171-5:171-5:171-4:171-4:170-4:170-4.i.k.cruel.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Ice Burn[nokeyword].doc.White Kyurem — Absolute Zero.hp.20.img.${kyuremImg}` +
    `)` +
    `.img.${kyuremImg}`;

  return `(${kyurem})`;
}

// =============================================================================
// MAIN
// =============================================================================

function main() {
  console.log('=== apply_bosses_gen4_5.js ===');
  console.log('Reading textmod_expanded.txt...');
  const content = fs.readFileSync(TEXTMOD_PATH, 'utf8');
  const lines = content.split('\n');
  console.log(`Original file has ${lines.length} lines.`);

  // =========================================================================
  // STEP 1: Update boss selection menu
  // =========================================================================
  // After Gen 3 script runs, the menu looks like:
  //   1.ph.cNumber#1;mch.ovXV1@4vYV1.mn.Gen 6@3mch.ovBV1@4vLV1.mn.Gen 7
  //   @3mch.ovGV1@4vHV1.mn.Gen 3
  //   @3mch.ovXV1@4vYV1@4vBV1@4vLV1@4vGV1@4vHV1.mn.Random;Choose your Bosses[n].mn.boss select,
  //
  // We add:
  //   Gen 4 using PV1/TV1 variables (bP/bT checks)
  //   Gen 5 using UV1/WV1 variables (bU/bW checks)
  // And update Random to include all new variables.

  const bossSelectIdx = lines.findIndex(l => l.includes('.mn.boss select,'));
  if (bossSelectIdx === -1) {
    console.error('ERROR: Could not find boss select menu line!');
    process.exit(1);
  }
  console.log(`Found boss select menu at line ${bossSelectIdx + 1}`);

  let bossSelectLine = lines[bossSelectIdx];

  // Find the current Random option pattern (it includes all existing variables)
  // We need to match the Random part dynamically since Gen 3 may have already modified it
  const randomMatch = bossSelectLine.match(/@3mch\.ov([^.]+)\.mn\.Random/);
  if (!randomMatch) {
    console.error('ERROR: Could not find Random option in boss select menu!');
    console.error('Line content:', bossSelectLine);
    process.exit(1);
  }
  const currentRandomVars = randomMatch[1]; // e.g. "XV1@4vYV1@4vBV1@4vLV1@4vGV1@4vHV1"

  // Build the new Random variables string (add PV1, TV1, UV1, WV1)
  const newRandomVars = currentRandomVars + '@4vPV1@4vTV1@4vUV1@4vWV1';

  // Insert Gen 4 and Gen 5 options before Random
  const oldRandomSection = `@3mch.ov${currentRandomVars}.mn.Random`;
  const newRandomSection =
    `@3mch.ovPV1@4vTV1.mn.Gen 4` +
    `@3mch.ovUV1@4vWV1.mn.Gen 5` +
    `@3mch.ov${newRandomVars}.mn.Random`;

  bossSelectLine = bossSelectLine.replace(oldRandomSection, newRandomSection);
  lines[bossSelectIdx] = bossSelectLine;
  console.log('Updated boss select menu to include Gen 4 and Gen 5.');

  // =========================================================================
  // STEP 2: Add Gen 5 Floor 8 boss to the existing Floor 8 line
  // =========================================================================
  // Gen 4 Floor 4/8 reuse existing Quagsire/Exeggutor — no changes needed.
  // Gen 5 Floor 4 reuses existing Quagsire — no changes needed.
  // Gen 5 Floor 8 adds Unova Starters to the ch.om8 choosable line.
  //
  // The ch.om8 line ends with .mn.COMBINED_NAME,
  // We append a new @4m8.fight.(STARTERS) before the final .mn.NAME,
  // and update the combined name.

  const floor8Idx = lines.findIndex(l => l.includes('ch.om8.fight.'));
  if (floor8Idx === -1) {
    console.error('ERROR: Could not find Floor 8 boss line!');
    process.exit(1);
  }
  console.log(`Found Floor 8 boss line at line ${floor8Idx + 1}`);

  const floor8Fight = buildFloor8UnovaStarters();

  let floor8Line = lines[floor8Idx];
  const floor8MnMatch = floor8Line.match(/\.mn\.([^,]+),$/);
  if (!floor8MnMatch) {
    console.error('ERROR: Could not parse Floor 8 mn name!');
    process.exit(1);
  }
  const oldFloor8Mn = floor8MnMatch[1];
  const newFloor8Mn = oldFloor8Mn + 'UnovaStarters';

  floor8Line = floor8Line.replace(
    `.mn.${oldFloor8Mn},`,
    `@4m8.fight.${floor8Fight}.mn.${newFloor8Mn},`
  );
  lines[floor8Idx] = floor8Line;
  console.log(`  Added Gen 5 Floor 8 boss (Unova Starters) to line ${floor8Idx + 1}`);

  // =========================================================================
  // STEP 3: Build Gen 4 conditional boss lines (Floors 12, 16, 20)
  // =========================================================================
  // Gen 4 uses PV1/TV1 variables → bP/bT checks.
  // Floor 12: Both bP and bT = Palkia fight (same fight, two slots)
  // Floor 16: Both bP and bT = Dialga fight (same fight, two slots)
  // Floor 20: Both bP and bT = Arceus fight (same fight, two slots)

  const palkiaFight = buildFloor12Palkia();
  const dialgaFight = buildFloor16Dialga();
  const arceusFight = buildFloor20Arceus();

  const gen4BossLines = [
    // Floor 12 - bP = Palkia
    `1.ph.bP;1;!m(12.fight.${palkiaFight}&hidden.mn.Palkia&hidden)@2!mskip&hidden.mn.Palkia,`,
    // Floor 12 - bT = Palkia (duplicate for variable routing)
    `1.ph.bT;1;!m(12.fight.${palkiaFight}&hidden.mn.Palkia1&hidden)@2!mskip&hidden.mn.Palkia1,`,
    // Floor 16 - bP = Dialga
    `1.ph.bP;1;!m(16.fight.${dialgaFight}&hidden)@2!mskip&hidden.mn.Dialga,`,
    // Floor 16 - bT = Dialga (duplicate)
    `1.ph.bT;1;!m(16.fight.${dialgaFight}&hidden)@2!mskip&hidden.mn.Dialga1,`,
    // Floor 20 - bP = Arceus
    `1.ph.bP;1;!m(20.fight.${arceusFight}&hidden)@2!mskip&hidden.mn.Arceus,`,
    // Floor 20 - bT = Arceus (duplicate)
    `1.ph.bT;1;!m(20.fight.${arceusFight}&hidden)@2!mskip&hidden.mn.Arceus1,`,
  ];

  // =========================================================================
  // STEP 4: Build Gen 5 conditional boss lines (Floors 12, 16, 20)
  // =========================================================================
  // Gen 5 uses UV1/WV1 variables → bU/bW checks.
  //
  // Floor 12: Random between Swords of Justice (bU) and Forces of Nature (bW)
  //   When Gen 5 is selected, both UV1 and WV1 are set.
  //   The engine randomly selects between the bU and bW line for that floor.
  //
  // Floor 16: Random between Reshiram (bU) and Zekrom (bW)
  //   Same random selection mechanism.
  //
  // Floor 20: Random between Black Kyurem (bU) and White Kyurem (bW)
  //   Same random selection mechanism — each variant per slot.

  const swordsOfJusticeFight = buildFloor12SwordsOfJustice();
  const forcesOfNatureFight = buildFloor12ForcesOfNature();
  const reshiramFight = buildFloor16Reshiram();
  const zekromFight = buildFloor16Zekrom();
  const blackKyuremFight = buildFloor20BlackKyurem();
  const whiteKyuremFight = buildFloor20WhiteKyurem();

  const gen5BossLines = [
    // Floor 12 - bU = Swords of Justice
    `1.ph.bU;1;!m(12.fight.${swordsOfJusticeFight}&hidden.mn.SwordsOfJustice&hidden)@2!mskip&hidden.mn.SwordsOfJustice,`,
    // Floor 12 - bW = Forces of Nature
    `1.ph.bW;1;!m(12.fight.${forcesOfNatureFight}&hidden.mn.ForcesOfNature&hidden)@2!mskip&hidden.mn.ForcesOfNature,`,
    // Floor 16 - bU = Reshiram
    `1.ph.bU;1;!m(16.fight.${reshiramFight}&hidden)@2!mskip&hidden.mn.Reshiram,`,
    // Floor 16 - bW = Zekrom
    `1.ph.bW;1;!m(16.fight.${zekromFight}&hidden)@2!mskip&hidden.mn.Zekrom,`,
    // Floor 20 - bU = Black Kyurem
    `1.ph.bU;1;!m(20.fight.${blackKyuremFight}&hidden)@2!mskip&hidden.mn.BlackKyurem,`,
    // Floor 20 - bW = White Kyurem
    `1.ph.bW;1;!m(20.fight.${whiteKyuremFight}&hidden)@2!mskip&hidden.mn.WhiteKyurem,`,
  ];

  // =========================================================================
  // STEP 5: Find insertion point and insert all new boss lines
  // =========================================================================
  // Insert after the last existing boss conditional line (before "Boss Mods" line).
  // The Boss Mods line contains "no flee" or "horde" and ".mn.Boss Mods,".
  // We insert our new lines BEFORE that line (maintaining the pattern where
  // all conditional boss fight lines come before the Boss Mods line).

  const bossModsIdx = lines.findIndex(l => l.includes('.mn.Boss Mods,'));
  if (bossModsIdx === -1) {
    console.error('ERROR: Could not find Boss Mods line!');
    process.exit(1);
  }
  console.log(`Found Boss Mods line at line ${bossModsIdx + 1}`);

  // We insert just before the Boss Mods line.
  // Build insertion array with blank line separators (matching existing format).
  const allNewBossLines = [...gen4BossLines, ...gen5BossLines];
  const toInsert = [];
  for (const bossLine of allNewBossLines) {
    toInsert.push('');       // blank line separator
    toInsert.push(bossLine); // boss line
  }

  // Insert before Boss Mods line
  lines.splice(bossModsIdx, 0, ...toInsert);

  console.log(`Inserted ${allNewBossLines.length} boss lines (${toInsert.length} total with separators) before Boss Mods.`);

  // =========================================================================
  // STEP 6: Write back
  // =========================================================================
  fs.writeFileSync(TEXTMOD_PATH, lines.join('\n'));
  console.log(`\nNew file has ${lines.length} lines.`);
  console.log('Written to textmod_expanded.txt successfully.');

  // =========================================================================
  // SUMMARY
  // =========================================================================
  console.log('\n=== SUMMARY ===');
  console.log('Boss selection menu: Added "Gen 4" (PV1/TV1) and "Gen 5" (UV1/WV1) options');
  console.log('');
  console.log('--- Gen 4 ---');
  console.log('Floor 4:  Reuses existing Quagsire (no change)');
  console.log('Floor 8:  Reuses existing Exeggutor (no change)');
  console.log('Floor 12: Palkia (HP 25) + Bronzong (HP 8) + Spatial Rift (HP 5) = 38 HP');
  console.log('Floor 16: Dialga (HP 25) + Bronzong (HP 8) + Temporal Anomaly (HP 5) = 38 HP');
  console.log('Floor 20: Arceus 4-phase (Normal HP 12 → Fire HP 10 → Steel HP 12 → Dragon HP 8) = 42 HP');
  console.log('');
  console.log('--- Gen 5 ---');
  console.log('Floor 4:  Reuses existing Quagsire (no change)');
  console.log('Floor 8:  Serperior + Emboar + Samurott (HP 12 each) = 36 HP');
  console.log('Floor 12: Swords of Justice (bU) — Cobalion + Terrakion + Virizion = 36 HP');
  console.log('          Forces of Nature (bW) — Tornadus + Thundurus + Landorus = 36 HP');
  console.log('Floor 16: Reshiram (bU) — Reshiram (HP 25) + Zekrom Echo (HP 8) = 33 HP');
  console.log('          Zekrom (bW) — Zekrom (HP 25) + Reshiram Echo (HP 8) = 33 HP');
  console.log('Floor 20: Black Kyurem (bU) — Kyurem (HP 15) → Black Kyurem (HP 20) = 35 HP');
  console.log('          White Kyurem (bW) — Kyurem (HP 15) → White Kyurem (HP 20) = 35 HP');
  console.log('');
  console.log(`Total new lines added: ${toInsert.length} (${allNewBossLines.length} boss lines + ${allNewBossLines.length} blank separators)`);
  console.log('Lines modified: Floor 8 boss line (added Unova Starters), boss select menu');
}

main();
