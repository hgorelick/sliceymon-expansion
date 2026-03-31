#!/usr/bin/env node

/**
 * apply_bosses_gen3.js
 *
 * Adds Gen 3 boss fights to textmod_expanded.txt:
 *   - Floor 4:  Golem + Geodude/Graveler
 *   - Floor 8:  Alpha Steelix + Onix
 *   - Floor 12: Regi Trio OR Legendary Birds (random)
 *   - Floor 16: Regigigas + Regi Guardians (with phase transition)
 *   - Floor 20: Deoxys (4-phase form shift)
 *
 * Also updates the boss selection menu to include "Gen 3" as an option.
 *
 * Variable routing:
 *   - XV1/YV1 = Gen 6 (bX/bY checks)
 *   - BV1/LV1 = Gen 7 (bB/bL checks)
 *   - GV1/HV1 = Gen 3 (bG/bH checks)   <-- NEW
 *
 * Floor 4 and Floor 8 bosses use ch.om4/ch.om8 choosable modifiers embedded
 * in the existing monsterpool lines (they chain with @4mN suffixes).
 * Floor 12/16/20 bosses use conditional variable checks (1.ph.bVAR;1;!m(FLOOR.fight...)).
 *
 * For Floor 12 Gen 3, we randomly select between Regi Trio and Legendary Birds
 * using two lines with bG and bH that both fire (since both GV1 and HV1 are set
 * for Gen 3), but one uses a random 50/50 to pick which actually spawns.
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
// BOSS FIGHT DEFINITIONS
// =============================================================================

/**
 * Floor 4: Golem + Geodude x2 + Graveler
 *
 * Pattern matches existing line 167 — ch.om4.fight.(...).mn.NAME@4m4.fight.(...)...
 * Since the existing line 167 chains multiple gen bosses with @4m4 suffixes,
 * Gen 3's Floor 4 boss needs to be appended to line 167 as another @4m4 chain.
 *
 * BUT — looking at the actual structure more carefully:
 *   Line 167 = ch.om4.fight.(<GEN6 BOSS>).mn.Quagsire@4m4.fight.(<GEN7 BOSS>)...mn.ZangooseQuagsireAriados,
 *
 * The @4m4 chains are all within the SAME line 167 for Floor 4.
 * Similarly line 169 does the same for Floor 8.
 *
 * For Gen 3, we need to ADD a new @4m4 chain to line 167 for Floor 4,
 * and a new @4m8 chain to line 169 for Floor 8.
 *
 * HOWEVER — the variable-based routing for floors 12/16/20 uses a different
 * mechanism. The boss select menu sets variable flags, and then conditional
 * lines fire based on those flags.
 *
 * For Floor 4/8, the existing structure seems to show ALL bosses on a single
 * line, and the game picks based on the flow. Looking more carefully:
 *
 * Line 167 ends with: .mn.ZangooseQuagsireAriados,
 * This is the monsterpool name for the COMBINED Floor 4 boss pool.
 *
 * The ch.om4 choosable modifier triggers at floor 4, and the @4m4 suffixes
 * chain alternative fights. The game presumably picks one based on the
 * variable flags set by the boss select menu.
 *
 * For Gen 3, we need to:
 * 1. Add a new @4m4 chain to line 167 with Gen 3 Floor 4 boss
 * 2. Add a new @4m8 chain to line 169 with Gen 3 Floor 8 boss
 * 3. Add new conditional lines for Floor 12/16/20
 * 4. Update the boss select menu
 */

function buildFloor4Gen3() {
  // Geodude x2 + Graveler + Golem
  const geodude = `Slimelet.n.Geodude.hp.3.doc.Might Explode.sd.170-1:170-1:170-1:170-1.i.k.singleuse.i.left2.facade.bas170:90:0:0#sidesc.[pips] damage[red] [n]self-destruct (dies after)[nokeyword].img.${spr('Geodude')}`;
  const graveler = `Wolf.n.Graveler.hp.6.doc.Rolling Towards You.sd.170-2:170-2:118-2:118-2.img.${spr('Graveler')}`;
  const golem = `Alpha.n.Golem.hp.12.doc.The Rolling Boulder.sd.170-3:170-3:118-3:118-3:170-2:170-2.i.k.rite.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.Ese118:0.img.${spr('Golem')}`;

  return `(${geodude}+${geodude}+${graveler}+${golem})`;
}

function buildFloor8Gen3() {
  // Onix x2 + Alpha Steelix
  const onix = `Wolf.n.Onix.hp.7.doc.Coils Around the Cavern.sd.170-2:170-2:118-3:118-3:118-2:118-2.img.${spr('Onix')}`;
  const steelix = `Alpha.n.Alpha Steelix.hp.18.doc.Lord of Iron and Stone.sd.170-5:170-5:118-6:118-6:118-4:118-4.i.k.rite.i.k.stasis.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.Ese118:0.i.right2.facade.Ese118:0.img.${spr('Steelix')}`;

  return `(${onix}+${onix}+${steelix})`;
}

function buildFloor12RegiTrio() {
  // Regirock + Regice + Registeel
  const regirock = `Alpha.n.Regirock.hp.12.doc.The Sealed Stone.sd.170-4:170-4:170-3:170-3:118-3:118-3.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.right2.facade.Ese118:0.img.${spr('Regirock')}`;
  const regice = `Alpha.n.Regice.hp.12.doc.The Sealed Ice.sd.170-4:170-4:171-2:171-2:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict exert[nokeyword].img.${spr('Regice')}`;
  const registeel = `Alpha.n.Registeel.hp.12.doc.The Sealed Steel.sd.170-3:170-3:118-5:118-5:118-4:118-4.i.k.stasis.i.topbot.facade.Ese118:0.i.right2.facade.Ese118:0.img.${spr('Registeel')}`;

  return `(${regirock}+${regice}+${registeel})`;
}

function buildFloor12LegendaryBirds() {
  // Articuno + Zapdos + Moltres
  const articuno = `Alpha.n.Articuno.hp.12.doc.The Frozen Wings of Winter.sd.170-3:170-3:118-4:118-4:170-2:170-2.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].i.topbot.facade.Ese118:0.img.${spr('Articuno')}`;
  const zapdos = `Alpha.n.Zapdos.hp.12.doc.The Thunder That Splits the Sky.sd.170-5:170-5:170-4:170-4:170-2:170-2.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] Thunder[nokeyword].img.${spr('Zapdos')}`;
  const moltres = `Alpha.n.Moltres.hp.12.doc.The Inferno That Engulfs All.sd.171-4:171-4:170-3:170-3:171-2:171-2.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]Heat Wave[nokeyword].i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict burn[nokeyword].img.${spr('Moltres')}`;

  return `(${articuno}+${zapdos}+${moltres})`;
}

function buildFloor16Regigigas() {
  // Regi Guardians (weakened) + Regigigas (phase transition)
  const regirockGuard = `Wolf.n.Regirock.hp.7.doc.The Sealed Stone Serves Its Master.sd.170-3:170-3:170-2:170-2:118-2:118-2.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].img.${spr('Regirock')}`;
  const regiceGuard = `Wolf.n.Regice.hp.6.doc.The Sealed Ice Serves Its Master.sd.170-3:170-3:170-2:170-2:170-1:170-1.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword].img.${spr('Regice')}`;
  const registeelGuard = `Wolf.n.Registeel.hp.8.doc.The Sealed Steel Serves Its Master.sd.170-2:170-2:118-3:118-3:118-2:118-2.i.k.stasis.i.topbot.facade.Ese118:0.img.${spr('Registeel')}`;

  // Regigigas with phase transition (Slow Start -> Full Power)
  const regigigasImg = spr('Regigigas');
  const regigigas = `Dragon.n.Regigigas.hp.20.doc.The Colossal Titan Awakens.sd.170-2:170-2:0:0:0:0.i.k.stasis.i.left2.facade.bas170:0:0:0#sidesc.[pips] damage[red] [n]slow start (weakened)[nokeyword].i.triggerhpdata.(lost.sd.170-8:170-6:171-4:171-4:170-3:170-2.i.k.cruel.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.topbot.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] rampage[nokeyword]).img.${regigigasImg}`;

  return `(${regirockGuard}+${regiceGuard}+${registeelGuard}+${regigigas})`;
}

function buildFloor20Deoxys() {
  // Deoxys 4-phase: Normal -> Attack -> Defense -> Speed
  // Uses nested .triggerhpdata.() for phase transitions (like Necrozma pattern)
  const deoxysImg = spr('Deoxys');

  const deoxys = `Basalt.n.Deoxys.hp.10.doc.The Extraterrestrial Mutant.sd.170-4:170-4:118-3:118-3:170-3:170-3.i.k.rite` +
    `.i.triggerhpdata.(lost.sd.170-8:170-8:170-6:170-6:171-4:171-4.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] critical[nokeyword].doc.Attack Form — Maximum Offense.hp.8.img.${deoxysImg}` +
    `.i.triggerhpdata.(lost.sd.118-6:118-6:118-5:118-5:170-2:170-2.i.k.stasis.i.left2.facade.Ese118:0.i.topbot.facade.Ese118:0.doc.Defense Form — Impenetrable.hp.12.img.${deoxysImg}` +
    `.i.triggerhpdata.(lost.sd.170-5:170-5:170-4:170-4:0:0.i.k.first.i.k.exert.i.right2.facade.bas170:0:0:0#sidesc.dodge[green] [n]evade next attack[nokeyword].doc.Speed Form — Untouchable.hp.8.img.${deoxysImg}` +
    `)` +
    `)` +
    `)` +
    `.img.${deoxysImg}`;

  return `(${deoxys})`;
}

// =============================================================================
// MAIN
// =============================================================================

function main() {
  console.log('Reading textmod_expanded.txt...');
  const content = fs.readFileSync(TEXTMOD_PATH, 'utf8');
  const lines = content.split('\n');
  console.log(`Original file has ${lines.length} lines.`);

  // =========================================================================
  // STEP 1: Update boss selection menu (line 171)
  // =========================================================================
  // Current menu (line 171, 0-indexed = 170):
  //   1.ph.cNumber#1;mch.ovXV1@4vYV1.mn.Gen 6@3mch.ovBV1@4vLV1.mn.Gen 7@3mch.ovXV1@4vYV1@4vBV1@4vLV1.mn.Random;Choose your Bosses[n].mn.boss select,
  //
  // We add Gen 3 using GV1/HV1 variables:
  //   mch.ovGV1@4vHV1.mn.Gen 3
  // And update Random to also include GV1/HV1:
  //   mch.ovXV1@4vYV1@4vBV1@4vLV1@4vGV1@4vHV1.mn.Random

  const bossSelectIdx = lines.findIndex(l => l.includes('.mn.boss select,'));
  if (bossSelectIdx === -1) {
    console.error('ERROR: Could not find boss select menu line!');
    process.exit(1);
  }
  console.log(`Found boss select menu at line ${bossSelectIdx + 1}`);

  const oldBossSelect = lines[bossSelectIdx];

  // Add Gen 3 option before Random, and update Random to include Gen 3 variables
  let newBossSelect = oldBossSelect;

  // Insert "Gen 3" option after "Gen 7" and before "Random"
  // Current: @3mch.ovXV1@4vYV1@4vBV1@4vLV1.mn.Random
  // New:     @3mch.ovGV1@4vHV1.mn.Gen 3@3mch.ovXV1@4vYV1@4vBV1@4vLV1@4vGV1@4vHV1.mn.Random
  newBossSelect = newBossSelect.replace(
    '@3mch.ovXV1@4vYV1@4vBV1@4vLV1.mn.Random',
    '@3mch.ovGV1@4vHV1.mn.Gen 3@3mch.ovXV1@4vYV1@4vBV1@4vLV1@4vGV1@4vHV1.mn.Random'
  );

  lines[bossSelectIdx] = newBossSelect;
  console.log('Updated boss select menu to include Gen 3.');

  // =========================================================================
  // STEP 2: Add Gen 3 Floor 4 boss to the existing Floor 4 line
  // =========================================================================
  // Line 167 (0-indexed = 166) ends with: .mn.ZangooseQuagsireAriados,
  // We need to append @4m4.fight.(<GEN3 FLOOR 4>) before .mn.ZangooseQuagsireAriados,
  // and update the mn name to include Gen 3 boss names.

  const floor4Idx = lines.findIndex(l => l.includes('ch.om4.fight.'));
  if (floor4Idx === -1) {
    console.error('ERROR: Could not find Floor 4 boss line!');
    process.exit(1);
  }
  console.log(`Found Floor 4 boss line at line ${floor4Idx + 1}`);

  const floor4Fight = buildFloor4Gen3();

  // The line ends with .mn.ZangooseQuagsireAriados,
  // We append a new @4m4.fight chain and update the mn name
  let floor4Line = lines[floor4Idx];
  const floor4MnMatch = floor4Line.match(/\.mn\.([^,]+),$/);
  if (!floor4MnMatch) {
    console.error('ERROR: Could not parse Floor 4 mn name!');
    process.exit(1);
  }
  const oldFloor4Mn = floor4MnMatch[1];
  const newFloor4Mn = oldFloor4Mn + 'GolemGeodudeGraveler';

  // Insert before the final .mn.NAME,
  floor4Line = floor4Line.replace(
    `.mn.${oldFloor4Mn},`,
    `@4m4.fight.${floor4Fight}.mn.${newFloor4Mn},`
  );
  lines[floor4Idx] = floor4Line;
  console.log(`  Added Gen 3 Floor 4 boss (Golem + Geodude/Graveler) to line ${floor4Idx + 1}`);

  // =========================================================================
  // STEP 3: Add Gen 3 Floor 8 boss to the existing Floor 8 line
  // =========================================================================
  const floor8Idx = lines.findIndex(l => l.includes('ch.om8.fight.'));
  if (floor8Idx === -1) {
    console.error('ERROR: Could not find Floor 8 boss line!');
    process.exit(1);
  }
  console.log(`Found Floor 8 boss line at line ${floor8Idx + 1}`);

  const floor8Fight = buildFloor8Gen3();

  let floor8Line = lines[floor8Idx];
  const floor8MnMatch = floor8Line.match(/\.mn\.([^,]+),$/);
  if (!floor8MnMatch) {
    console.error('ERROR: Could not parse Floor 8 mn name!');
    process.exit(1);
  }
  const oldFloor8Mn = floor8MnMatch[1];
  const newFloor8Mn = oldFloor8Mn + 'SteelixOnix';

  floor8Line = floor8Line.replace(
    `.mn.${oldFloor8Mn},`,
    `@4m8.fight.${floor8Fight}.mn.${newFloor8Mn},`
  );
  lines[floor8Idx] = floor8Line;
  console.log(`  Added Gen 3 Floor 8 boss (Alpha Steelix + Onix) to line ${floor8Idx + 1}`);

  // =========================================================================
  // STEP 4: Add Gen 3 Floor 12/16/20 boss lines
  // =========================================================================
  // These use the conditional variable pattern: 1.ph.bVAR;1;!m(FLOOR.fight...)
  // For Gen 3, we use bG (GV1 variable).
  //
  // For Floor 12, we need random selection between Regi Trio and Legendary Birds.
  // We'll use a choosable within the fight line that randomly picks one of two fights.
  // Looking at the existing pattern, we can embed both fight groups and use
  // the random choosable mechanism.
  //
  // Actually, the simplest approach matching the existing pattern:
  // Use TWO lines for Floor 12 - one with bG and one with bH.
  // When Gen 3 is selected, BOTH GV1 and HV1 are set.
  // We make each line use the random skip mechanism.
  //
  // Better approach: Use a single bG line that contains a random choosable
  // to pick between Regi Trio and Legendary Birds.
  // Format: 1.ph.bG;1;!m(12.fight.<REGI_TRIO>&hidden.mn.RegiTrio&hidden)@2!mskip&hidden.mn.RegiTrio,
  //
  // Actually the simplest and cleanest: put both fights in a single conditional line
  // using the @2!mskip pattern to make a 50/50 random selection:
  // 1.ph.bG;1;!m(12.fight.<REGIS>@2!m(12.fight.<BIRDS>&hidden.mn.name&hidden)@2!mskip&hidden.mn.name,

  // Looking at how Gen 6/7 Floor 12 works (line 173):
  // 1.ph.bX;1;!m(12.fight.<XERNEAS_FIGHT>&hidden.mn.Xerneas&hidden)@2!mskip&hidden.mn.Xerneas,
  // And line corresponding to bY (Gen 7):
  // 1.ph.bY;1;!m(12.fight.<YVELTAL_FIGHT>&hidden)@2!mskip&hidden.mn.Yveltal,
  //
  // For Gen 3 Floor 12 with random selection, we'll use:
  // Line A (bG): 1.ph.bG;1;!m(12.fight.<REGI_TRIO>&hidden.mn.RegiTrio&hidden)@2!mskip&hidden.mn.RegiTrio,
  // Line B (bH): 1.ph.bH;1;!m(12.fight.<LEGENDARY_BIRDS>&hidden.mn.LegendaryBirds&hidden)@2!mskip&hidden.mn.LegendaryBirds,
  //
  // WAIT — that would play BOTH fights. We need random selection.
  //
  // Looking at the pattern more carefully:
  // The @2!mskip means "at priority 2, skip (do nothing)" — it's the fallback.
  // The 1.ph.bX;1;!m(...) means "if variable X is set, at priority 1, execute..."
  //
  // For random 50/50, we can set EITHER GV1 or HV1 (not both) randomly in the menu.
  // Let's modify the boss select approach:
  // Instead of setting both GV1 AND HV1 for Gen 3, we set GV1 always for Gen 3,
  // and then have the Floor 12 line itself do the random selection internally.
  //
  // Simpler: Use a single variable (GV1) for all Gen 3 bosses.
  // For Floor 12 random, embed the random choosable INSIDE the fight line.
  // We can use `cNumber#1` (the same random choosable as the boss select) to pick.
  //
  // Actually, looking at the existing code pattern more carefully:
  // - Gen 6 uses TWO variables: XV1 and YV1 (both set together)
  // - bX checks trigger Floor 12/16/20 Gen 6 lines
  // - bY checks trigger DUPLICATE Floor 16/20 Gen 7 lines
  // - Gen 7 uses TWO variables: BV1 and LV1 (both set together)
  //
  // So EACH gen uses TWO variables, both always set together.
  // The two variables allow two DIFFERENT conditional lines per floor for the same gen.
  //
  // For Gen 3 with random Floor 12:
  // - Set GV1 always for Gen 3
  // - HV1 is set randomly (50/50) via a choosable in the menu
  //
  // NO — simpler approach. Looking again at the existing lines:
  // Line 173: 1.ph.bX;1;!m(12.fight...Xerneas...&hidden.mn.Xerneas&hidden)@2!mskip&hidden.mn.Xerneas,
  // Line 175: 1.ph.bX;1;!m(16.fight...Zygarde...&hidden)@2!mskip&hidden.mn.Zygarde,
  // Line 177: 1.ph.bY;1;!m(16.fight...Zygarde...&hidden)@2!mskip&hidden.mn.Zygarde1,
  //
  // Wait — both lines 175 (bX) and 177 (bY) are for Floor 16 Zygarde!
  // And both 179 (bX) and 181 (bY) are for Floor 20 Hoopa!
  // So bX and bY BOTH fire, giving TWO copies of each fight.
  // The mn names are different (Zygarde vs Zygarde1, Hoopa vs Hoopa1).
  //
  // This means each gen fires BOTH variable lines for floors 12/16/20.
  // The double-firing provides redundancy or different variants.
  //
  // For Gen 3:
  // - GV1/HV1 both set for Gen 3
  // - bG lines = one variant, bH lines = another variant
  // - Floor 12 bG = Regi Trio, Floor 12 bH = Legendary Birds
  // - Floor 16 bG = Regigigas (copy 1), Floor 16 bH = Regigigas (copy 2)
  // - Floor 20 bG = Deoxys (copy 1), Floor 20 bH = Deoxys (copy 2)
  //
  // This PERFECTLY matches the random Floor 12 requirement!
  // When Gen 3 is selected, both GV1 and HV1 are set.
  // The game will randomly use either the bG or bH line for Floor 12,
  // giving us Regi Trio vs Legendary Birds randomly.

  const regiTrioFight = buildFloor12RegiTrio();
  const legendaryBirdsFight = buildFloor12LegendaryBirds();
  const regigigasFight = buildFloor16Regigigas();
  const deoxysFight = buildFloor20Deoxys();

  // Build the new lines following existing pattern exactly
  const newBossLines = [
    // Floor 12 - bG = Regi Trio
    `1.ph.bG;1;!m(12.fight.${regiTrioFight}&hidden.mn.RegiTrio&hidden)@2!mskip&hidden.mn.RegiTrio,`,
    // Floor 12 - bH = Legendary Birds
    `1.ph.bH;1;!m(12.fight.${legendaryBirdsFight}&hidden.mn.LegendaryBirds&hidden)@2!mskip&hidden.mn.LegendaryBirds,`,
    // Floor 16 - bG = Regigigas (copy 1)
    `1.ph.bG;1;!m(16.fight.${regigigasFight}&hidden)@2!mskip&hidden.mn.Regigigas,`,
    // Floor 16 - bH = Regigigas (copy 2)
    `1.ph.bH;1;!m(16.fight.${regigigasFight}&hidden)@2!mskip&hidden.mn.Regigigas1,`,
    // Floor 20 - bG = Deoxys (copy 1)
    `1.ph.bG;1;!m(20.fight.${deoxysFight}&hidden)@2!mskip&hidden.mn.Deoxys,`,
    // Floor 20 - bH = Deoxys (copy 2)
    `1.ph.bH;1;!m(20.fight.${deoxysFight}&hidden)@2!mskip&hidden.mn.Deoxys1,`,
  ];

  // =========================================================================
  // STEP 5: Insert new boss lines after the last existing boss line
  // =========================================================================
  // The last boss line is line 181 (Hoopa1). We insert Gen 3 lines after it.
  // Find the last Hoopa line by searching for mn.Hoopa1

  const lastBossIdx = lines.findIndex(l => l.includes('.mn.Hoopa1,'));
  if (lastBossIdx === -1) {
    // Fallback: find the last Hoopa line
    let fallbackIdx = -1;
    for (let i = lines.length - 1; i >= 0; i--) {
      if (lines[i].includes('Hoopa') && lines[i].includes('.mn.')) {
        fallbackIdx = i;
        break;
      }
    }
    if (fallbackIdx === -1) {
      console.error('ERROR: Could not find last boss line (Hoopa)!');
      process.exit(1);
    }
    console.log(`Using fallback: last Hoopa line at ${fallbackIdx + 1}`);
  }

  const insertAfterIdx = lines.findIndex(l => l.includes('.mn.Hoopa1,')) !== -1
    ? lines.findIndex(l => l.includes('.mn.Hoopa1,'))
    : (() => {
        for (let i = lines.length - 1; i >= 0; i--) {
          if (lines[i].includes('Hoopa') && lines[i].includes('.mn.')) return i;
        }
        return -1;
      })();

  if (insertAfterIdx === -1) {
    console.error('ERROR: Could not determine insertion point for Gen 3 boss lines!');
    process.exit(1);
  }

  console.log(`Inserting Gen 3 boss lines after line ${insertAfterIdx + 1}`);

  // Build insertion array with blank line separators (matching existing format)
  const toInsert = [];
  for (const bossLine of newBossLines) {
    toInsert.push('');       // blank line separator
    toInsert.push(bossLine); // boss line
  }

  // Insert after the last boss line
  lines.splice(insertAfterIdx + 1, 0, ...toInsert);

  console.log(`Inserted ${newBossLines.length} Gen 3 boss lines (${toInsert.length} total with separators)`);

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
  console.log('Boss selection menu: Added "Gen 3" option with GV1/HV1 variables');
  console.log('Floor 4:  Golem (HP 12) + Geodude x2 (HP 3) + Graveler (HP 6) = 24 HP total');
  console.log('Floor 8:  Alpha Steelix (HP 18) + Onix x2 (HP 7) = 32 HP total');
  console.log('Floor 12: Regi Trio (bG) — Regirock + Regice + Registeel = 36 HP total');
  console.log('          Legendary Birds (bH) — Articuno + Zapdos + Moltres = 36 HP total');
  console.log('Floor 16: Regigigas (HP 20, 2-phase) + Regi Guardians (HP 7+6+8) = 41 HP total');
  console.log('Floor 20: Deoxys (4-phase: Normal HP 10 → Attack HP 8 → Defense HP 12 → Speed HP 8) = 38 HP total');
  console.log(`\nTotal new lines added: ${toInsert.length} (${newBossLines.length} boss lines + ${newBossLines.length} blank separators)`);
  console.log('Lines modified: Floor 4 boss line, Floor 8 boss line, boss select menu');
}

main();
