#!/usr/bin/env node
/**
 * Assembles the expanded Sliceymon textmod by applying all hero replacements,
 * Ditto deletion, and new color additions to the original textmod.txt.
 *
 * Usage: node assemble_textmod.js
 * Output: textmod_expanded.txt
 */

const fs = require('fs');
const path = require('path');

const BASE = path.join(__dirname, '..');
const TEXTMOD = path.join(BASE, 'textmod.txt');
const GENERATED = path.join(BASE, 'generated');
const OUTPUT = path.join(BASE, 'textmod_expanded.txt');

// Read the original textmod
const lines = fs.readFileSync(TEXTMOD, 'utf-8').split('\n');
console.log(`Original textmod: ${lines.length} lines`);

// Hero line replacements: { lineNumber: generatedFile }
const REPLACEMENTS = {
  27: 'line_27_chikorita.txt',    // Fomantis → Chikorita (l, P1)
  31: 'line_31_nidoranf.txt',     // Applin → Nidoran♀ (n, P1)
  39: 'line_39_togepi.txt',       // Darumaka → Togepi (r, P1)
  53: 'line_53_charmander.txt',   // Agumon → Charmander (z, P1)
  61: 'line_61_mudkip.txt',       // Varoom → Mudkip (g, P2)
  69: 'line_69_torchic.txt',      // Espurr → Torchic (k, P2)
  71: 'line_71_treecko.txt',      // Sunkern → Treecko (l, P2)
  73: 'line_73_wailmer.txt',      // Roggenrola → Wailmer (m, P2)
  75: 'line_75_nidoranm.txt',     // Turtwig(slot) → Nidoran♂ (n, P2)
  81: 'line_81_cyndaquil.txt',    // Slugma → Cyndaquil (q, P2)
  87: 'line_87_weedle.txt',       // Burmy → Weedle (t, P2)
  89: 'line_89_cleffa.txt',       // Tinkatink → Cleffa (u, P2)
  93: 'line_93_pikachu.txt',      // Joltik → Pikachu (x, P2)
  97: 'line_97_beldum.txt',       // Tentomon → Beldum (z, P2)
  79: 'line_79_bulbasaur.txt',    // Trubbish → Bulbasaur (p, P2)
  99: 'line_99_bagon.txt',        // Ditto → Bagon (w, P1)
};

// New lines to ADD (these don't replace existing lines)
const NEW_LINES = [
  'line_new_dratini.txt',    // Dratini (w, P2) — paired with Bagon
  'line_new_machop.txt',     // Machop (e, P1) — new color
  'line_new_riolu.txt',      // Riolu (e, P2) — new color
  'line_new_totodile.txt',   // Totodile (j, P1) — new color
  'line_new_poliwag.txt',    // Poliwag (j, P2) — new color
];

// Turtwig needs to MOVE from Line 75 (n, P2) to Line 29 (m, P1, replacing Rockruff)
// But we need the existing Turtwig line. Let's read it from the original.
const TURTWIG_MOVE = {
  fromLine: 75,  // Original Turtwig position (now replaced by Nidoran♂)
  toLine: 29,    // Rockruff position (being replaced)
};

let changeLog = [];

// Step 1: Apply hero replacements
for (const [lineNum, file] of Object.entries(REPLACEMENTS)) {
  const ln = parseInt(lineNum);
  const filePath = path.join(GENERATED, file);
  if (!fs.existsSync(filePath)) {
    console.error(`WARNING: Missing ${file}`);
    continue;
  }
  const newContent = fs.readFileSync(filePath, 'utf-8').trim();
  const oldPreview = lines[ln - 1].substring(0, 60);
  lines[ln - 1] = newContent;
  changeLog.push(`Line ${ln}: Replaced (${oldPreview}...) with ${file}`);
}

// Step 2: Move Turtwig from Line 75 to Line 29
// Line 75 is now Nidoran♂ (replaced above). We need the ORIGINAL Turtwig line.
// Read it from the original textmod before our changes.
const originalLines = fs.readFileSync(TEXTMOD, 'utf-8').split('\n');
const turtwigLine = originalLines[TURTWIG_MOVE.fromLine - 1];
// Modify Turtwig's color from n to m (since it's moving to Rockruff's color slot)
// The Turtwig line has .col.n — we need to change to .col.m
const turtwigModified = turtwigLine.replace(/\.col\.n/g, '.col.m');
lines[TURTWIG_MOVE.toLine - 1] = turtwigModified;
changeLog.push(`Line ${TURTWIG_MOVE.toLine}: Moved Turtwig here (from Line ${TURTWIG_MOVE.fromLine}), changed col.n→col.m`);

// Step 3: Add new color lines
// New heroes need to be inserted. In the textmod, hero lines are on odd numbers 9-99.
// We'll append new lines after line 99 (before the item/monster sections at line 101).
// Actually, let's insert them right before line 101 to keep them in the hero section.
// We need to add blank spacer lines between them (even lines are blank).
const newHeroLines = [];
for (const file of NEW_LINES) {
  const filePath = path.join(GENERATED, file);
  if (!fs.existsSync(filePath)) {
    console.error(`WARNING: Missing ${file}`);
    continue;
  }
  const content = fs.readFileSync(filePath, 'utf-8').trim();
  newHeroLines.push(content);
  newHeroLines.push(''); // blank spacer line
  changeLog.push(`Added new line: ${file}`);
}

// Insert new hero lines after line 99 (index 98) and before line 100 (index 99)
// Line 100 is a blank spacer, line 101 starts the item section
const insertIndex = 100; // After line 100 (blank spacer after Ditto/Bagon)
lines.splice(insertIndex, 0, ...newHeroLines);

// Step 4: Delete the erroneous line_87_machop.txt reference (Machop goes to new color e, not line 87)
// Line 87 is already correctly set to Weedle above. No action needed.

// Write output
fs.writeFileSync(OUTPUT, lines.join('\n'));
console.log(`\nExpanded textmod written to: ${OUTPUT}`);
console.log(`Total lines: ${lines.length} (was ${originalLines.length})`);
console.log(`\n=== Change Log ===`);
changeLog.forEach(c => console.log(`  ${c}`));
console.log(`\nHero replacements: ${Object.keys(REPLACEMENTS).length}`);
console.log(`New hero lines added: ${NEW_LINES.length}`);
console.log(`Turtwig moved: Line ${TURTWIG_MOVE.fromLine} → Line ${TURTWIG_MOVE.toLine}`);
