#!/usr/bin/env node

/**
 * apply_monsters.js
 *
 * Adds new regular monsters to the Sliceymon mod's textmod_expanded.txt.
 * Each monster is added as a new .part.1 line AFTER the existing monsterpool
 * line for its floor range.
 *
 * Format: (FLOOR.monsterpool.(TEMPLATE.n.NAME.hp.N.doc.DESC.sd.FACES.ITEMS.img.SPRITE).part.1&hidden).mn.NAME,
 */

const fs = require('fs');
const path = require('path');

// --- Paths ---
const TEXTMOD_PATH = path.join(__dirname, '..', 'textmod_expanded.txt');
const SPRITES_PATH = path.join(__dirname, 'sprite_encodings.json');

// --- Load sprite encodings ---
const sprites = JSON.parse(fs.readFileSync(SPRITES_PATH, 'utf8'));

// --- Monster Definitions ---
// Organized by the line number (1-indexed) they should be inserted AFTER.
// Line 129 = Floor 1-3 monsterpool (mn.Enemiepool1)
// Line 131 = Floor 9-11 monsterpool (mn.Enemiepool2)
// Line 133 = Floor 17-19 monsterpool (mn.Enemiepool3)
// Line 135 = Elite/custom monsterpool (mn.Custompool)

const MONSTERS = {
  // Floor 1-3 monsters — insert after line 129
  129: [
    {
      floorRange: '1-3',
      template: 'Wolf',
      name: 'Zubat',
      hp: 3,
      doc: 'Screeches in the Dark',
      sd: '170-1:170-1:0:0',
      items: '.i.k.exert.i.topbot.facade.bas170:0:0:0#sidesc.[pips] damage[red] [n]inflict weaken[nokeyword]',
      sprite: sprites['Zubat'],
    },
    {
      floorRange: '1-3',
      template: 'Wolf',
      name: 'Tentacool',
      hp: 4,
      doc: 'Stings on Contact',
      sd: '170-2:170-2:170-1:170-1',
      items: '.i.k.pain.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict poison[nokeyword]',
      sprite: sprites['Tentacool'],
    },
    {
      floorRange: '1-3',
      template: 'Wolf',
      name: 'Carvanha',
      hp: 3,
      doc: "Bites First, Thinks Never",
      sd: '170-2:170-2:170-2:170-2',
      items: '.i.k.cruel',
      sprite: sprites['Carvanha'],
    },
    {
      floorRange: '1-3',
      template: 'Wolf',
      name: 'Chinchou',
      hp: 4,
      doc: 'Glows Ominously',
      sd: '170-1:170-1:158-1:158-1',
      items: '',
      sprite: sprites['Chinchou'],
    },
  ],

  // Floor 9-11 monsters — insert after line 131
  131: [
    {
      floorRange: '9-11',
      template: 'Wolf',
      name: 'Golbat',
      hp: 6,
      doc: 'Drains the Will to Fight',
      sd: '170-2:170-2:170-1:170-1:158-1:158-1',
      items: '.i.k.exert.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict poison[nokeyword]',
      sprite: sprites['Golbat'],
    },
    {
      floorRange: '9-11',
      template: 'Wolf',
      name: 'Tentacruel',
      hp: 8,
      doc: 'Tentacles Lash All',
      sd: '171-3:171-3:170-2:170-2:170-1:170-1',
      items: '.i.k.pain.i.left2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict poison[nokeyword]',
      sprite: sprites['Tentacruel'],
    },
    {
      floorRange: '9-11',
      template: 'Wolf',
      name: 'Sharpedo',
      hp: 7,
      doc: 'The Bully of the Sea',
      sd: '170-4:170-4:170-3:170-3:170-2:170-2',
      items: '.i.k.cruel.i.k.first',
      sprite: sprites['Sharpedo'],
    },
    {
      floorRange: '9-11',
      template: 'Wolf',
      name: 'Lanturn',
      hp: 7,
      doc: 'Lights the Abyss',
      sd: '170-2:170-2:118-2:118-2:158-1:158-1',
      items: '.i.topbot.facade.Ese118:0',
      sprite: sprites['Lanturn'],
    },
    {
      floorRange: '9-11',
      template: 'Sarcophagus',
      name: 'Wild Steelix',
      hp: 10,
      doc: 'An Iron Serpent Blocks the Path',
      sd: '170-3:170-3:118-4:118-4:118-3:118-3',
      items: '.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.right2.facade.Ese118:0',
      sprite: sprites['Steelix'],
    },
  ],

  // Floor 17-19 monsters — insert after line 133
  133: [
    {
      floorRange: '17-19',
      template: 'Wolf',
      name: 'Crobat',
      hp: 9,
      doc: 'Silent Wings, Venomous Fangs',
      sd: '170-4:170-4:170-3:170-3:171-2:171-2',
      items: '.i.k.exert.i.k.pain.i.left2.facade.bas170:40:0:0#sidesc.[pips] damage[red] [n]inflict poison[nokeyword].i.right2.facade.bas171:0:0:0#sidesc.[pips] damage to all[red] [n]inflict weaken[nokeyword]',
      sprite: sprites['Crobat'],
    },
    {
      floorRange: '17-19',
      template: 'Sarcophagus',
      name: 'Elite Steelix',
      hp: 12,
      doc: 'The Mountain That Moves',
      sd: '170-5:170-5:118-5:118-5:118-4:118-4',
      items: '.i.k.stasis.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] heavy[nokeyword].i.right2.facade.Ese118:0',
      sprite: sprites['Steelix'],
    },
    {
      floorRange: '17-19',
      template: 'Wolf',
      name: 'Absol',
      hp: 8,
      doc: 'Disaster Follows in Its Wake',
      sd: '170-5:170-5:170-4:170-4:170-3:170-3',
      items: '.i.k.cruel.i.k.first.i.left2.facade.bas170:90:40:20#sidesc.[pips] damage[red] critical[nokeyword]',
      sprite: sprites['Absol'],
    },
  ],
};

/**
 * Build a .part.1 monster line in the textmod format.
 *
 * Format matches existing entries like line 137 (Diglett):
 *   (FLOOR.monsterpool.(TEMPLATE.n.NAME.hp.N.doc.DESC.sd.FACES.ITEMS.img.SPRITE).part.1&hidden).mn.NAME,
 */
function buildMonsterLine(monster) {
  const { floorRange, template, name, hp, doc, sd, items, sprite } = monster;

  if (!sprite) {
    console.error(`WARNING: No sprite found for "${name}" — using PLACEHOLDER`);
  }

  // Build the inner definition
  let definition = `${template}`;
  definition += `.n.${name}`;
  definition += `.hp.${hp}`;
  if (doc) {
    definition += `.doc.${doc}`;
  }
  definition += `.sd.${sd}`;
  if (items) {
    definition += items;  // items string already starts with '.'
  }
  definition += `.img.${sprite || 'PLACEHOLDER'}`;

  // Build the full line
  // Format: (FLOOR.monsterpool.(DEFINITION).part.1&hidden).mn.NAME,
  const line = `(${floorRange}.monsterpool.(${definition}).part.1&hidden).mn.${name},`;

  return line;
}

// --- Main ---
function main() {
  console.log('Reading textmod_expanded.txt...');
  const content = fs.readFileSync(TEXTMOD_PATH, 'utf8');
  const lines = content.split('\n');

  console.log(`Original file has ${lines.length} lines.`);

  // We need to insert lines AFTER specific line numbers (1-indexed).
  // Process in reverse order of line number so insertions don't shift earlier targets.
  const insertionPoints = Object.keys(MONSTERS)
    .map(Number)
    .sort((a, b) => b - a);  // descending order

  for (const lineNum of insertionPoints) {
    const monsters = MONSTERS[lineNum];
    const insertIndex = lineNum; // Insert AFTER lineNum (1-indexed), so at 0-indexed position lineNum

    console.log(`\nInserting ${monsters.length} monster(s) after line ${lineNum}:`);

    // Build all monster lines for this insertion point
    const newLines = [];
    for (const monster of monsters) {
      const line = buildMonsterLine(monster);
      console.log(`  - ${monster.name} (Floor ${monster.floorRange}, HP ${monster.hp}, template: ${monster.template})`);
      newLines.push(line);
    }

    // Insert: add a blank line before each monster block, then the monster lines
    // Looking at existing format (lines 136-137), there's a blank line between entries
    const toInsert = [];
    for (const newLine of newLines) {
      toInsert.push('');  // blank line separator
      toInsert.push(newLine);
    }

    // Splice into the lines array at position insertIndex (after the target line)
    lines.splice(insertIndex, 0, ...toInsert);

    console.log(`  Inserted ${toInsert.length} lines (${newLines.length} monsters + ${newLines.length} blank separators)`);
  }

  console.log(`\nNew file has ${lines.length} lines.`);

  // Write back
  fs.writeFileSync(TEXTMOD_PATH, lines.join('\n'));
  console.log('Written to textmod_expanded.txt successfully.');

  // Summary
  console.log('\n=== SUMMARY ===');
  let totalMonsters = 0;
  for (const lineNum of Object.keys(MONSTERS).map(Number).sort((a, b) => a - b)) {
    const monsters = MONSTERS[lineNum];
    totalMonsters += monsters.length;
    console.log(`Line ${lineNum} (Floor ${monsters[0].floorRange}): ${monsters.map(m => m.name).join(', ')}`);
  }
  console.log(`Total new monsters added: ${totalMonsters}`);
}

main();
