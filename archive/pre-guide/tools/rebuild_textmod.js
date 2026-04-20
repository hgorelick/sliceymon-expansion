#!/usr/bin/env node
/**
 * Deterministic textmod rebuilder.
 *
 * Reads the original textmod.txt, applies all hero replacements from generated/ files,
 * updates sprites in all generated files from sprite_encodings.json,
 * fixes known structural issues (paren balancing), and validates.
 *
 * Usage: node tools/rebuild_textmod.js
 * Output: textmod_heroes_only.txt (overwritten)
 */

const fs = require('fs');
const path = require('path');

const BASE = path.join(__dirname, '..');
const TEXTMOD = path.join(BASE, 'textmod.txt');
const GENERATED = path.join(BASE, 'generated');
const SPRITES = require(path.join(__dirname, 'sprite_encodings.json'));
const OUTPUT = path.join(BASE, 'textmod_heroes_only.txt');

// ============================================================
// STEP 1: Update sprites in all generated hero files
// ============================================================

function updateSprites(line) {
  // Replace .img.OLDDATA with .img.NEWDATA for each Pokemon name found
  // Strategy: find all .img. occurrences and the .n.Name that follows them
  // to determine which sprite to use.

  // Build a map of all tier names in the line
  const tierNames = [];
  const nameRegex = /\.n\.([A-Z][A-Za-z ]+)/g;
  let m;
  while ((m = nameRegex.exec(line)) !== null) {
    tierNames.push({ name: m[1], pos: m.index });
  }

  // For each .img. in the line, find the NEXT .n.Name to determine the Pokemon
  const imgRegex = /\.img\.([A-Za-z0-9%=]+)/g;
  const replacements = [];

  while ((m = imgRegex.exec(line)) !== null) {
    const imgStart = m.index;
    const imgData = m[1];

    // Check if this .img. is inside abilitydata (spell icon — don't replace)
    // Spell icons are like .img.spark or .img.beam or .img.light (short strings, <30 chars)
    // Hero sprites are long encoded strings (>50 chars typically)
    if (imgData.length < 50) continue;

    // Find the next .n.Name after this img
    const nextName = tierNames.find(t => t.pos > imgStart);
    if (!nextName) continue;

    // Look up the sprite
    const spriteName = nextName.name;
    const newSprite = SPRITES[spriteName];
    if (!newSprite) {
      console.error(`  WARNING: No sprite for "${spriteName}"`);
      continue;
    }

    replacements.push({
      start: imgStart + 5, // after ".img."
      end: imgStart + 5 + imgData.length,
      oldData: imgData,
      newData: newSprite,
      name: spriteName,
    });
  }

  // Apply replacements in reverse order to maintain positions
  let result = line;
  for (let i = replacements.length - 1; i >= 0; i--) {
    const r = replacements[i];
    result = result.substring(0, r.start) + r.newData + result.substring(r.end);
  }

  return { line: result, replacedCount: replacements.length };
}

// ============================================================
// STEP 2: Fix parenthesization (ensure tier separators at depth 0)
// ============================================================

function fixHeroParens(line) {
  const poolMarker = line.includes('!mheropool.') ? '!mheropool.' : '!mHeropool.';
  const poolIdx = line.indexOf(poolMarker);
  if (poolIdx === -1) return line;

  const prefix = line.substring(0, poolIdx + poolMarker.length);
  const rest = line.substring(poolIdx + poolMarker.length);
  const partIdx = rest.indexOf('.part.1&hidden');
  if (partIdx === -1) return line;

  const content = rest.substring(0, partIdx);
  const suffix = rest.substring(partIdx);

  // Find all .speech. positions (tier boundaries)
  const speechRegex = /\.speech\./g;
  let m;
  const speechPositions = [];
  while ((m = speechRegex.exec(content)) !== null) {
    speechPositions.push(m.index);
  }

  const segments = [];
  for (let i = 0; i < speechPositions.length; i++) {
    const speechPos = speechPositions[i];
    let dataStart = i === 0 ? 0 : segments[i - 1].afterName;

    const afterSpeech = content.substring(speechPos);
    const nameMatch = afterSpeech.match(/\.n\.([A-Z][A-Za-z ]*)/);
    if (!nameMatch) return line; // Can't parse, return unchanged
    const nameEnd = speechPos + afterSpeech.indexOf(nameMatch[0]) + nameMatch[0].length;

    let afterName = nameEnd;
    if (nameEnd < content.length && content[nameEnd] === '+') afterName = nameEnd + 1;

    let tierData = content.substring(dataStart, speechPos);
    const speechAndName = content.substring(speechPos, nameEnd);
    segments.push({ tierData, speechAndName, afterName });
  }

  // Reconstruct with proper parens
  let result = '';
  for (let i = 0; i < segments.length; i++) {
    const seg = segments[i];
    let data = seg.tierData;
    if (i > 0) result += '+';
    data = data.replace(/^\+?\(?/, '');
    data = data.replace(/\)+$/, '');

    // Count internal paren balance (for abilitydata etc)
    let balance = 0;
    for (const c of data) {
      if (c === '(') balance++;
      if (c === ')') balance--;
    }
    const closingParens = ')'.repeat(balance + 1);
    if (!data.startsWith('replica.') && !data.match(/^[A-Z][a-z]+\./)) {
      // Handle special templates like Primrose
    }
    result += '(' + data + closingParens + seg.speechAndName;
  }

  return prefix + result + suffix;
}

// ============================================================
// STEP 3: Validate a hero line
// ============================================================

function validateHeroLine(line, lineNum) {
  const errors = [];

  // Check paren balance
  let balance = 0;
  for (const c of line) {
    if (c === '(') balance++;
    if (c === ')') balance--;
  }
  if (balance !== 0) errors.push(`Paren imbalance: ${balance}`);

  // Check depth-0 +( count
  let depth = 0, d0 = 0, d1 = 0;
  const poolIdx = line.indexOf('heropool.');
  if (poolIdx === -1) return errors;
  const content = line.substring(poolIdx + 9);
  for (let i = 0; i < content.length; i++) {
    if (content[i] === '(') depth++;
    if (content[i] === ')') depth--;
    if (content[i] === '+' && content[i + 1] === '(' && depth === 0) d0++;
    else if (content[i] === '+' && content[i + 1] === '(' && depth > 0) d1++;
  }
  if (d1 > 0) errors.push(`${d1} nested +(  at depth>0 (should all be depth 0)`);

  // Check required fields
  if (!line.includes('.mn.')) errors.push('Missing .mn.');
  if (!line.includes('.part.1')) errors.push('Missing .part.1');
  if (!line.endsWith(',')) errors.push('Missing trailing comma');

  return errors;
}

// ============================================================
// MAIN
// ============================================================

console.log('=== Sliceymon+ Textmod Rebuilder ===\n');

// Read original textmod
const origLines = fs.readFileSync(TEXTMOD, 'utf-8').split('\n');
console.log(`Original textmod: ${origLines.length} lines`);

// Copy original lines
const lines = [...origLines];

// Hero replacements: lineNumber -> generatedFile
const REPLACEMENTS = {
  21: null,  // Larvitar — modified in-place (color change only)
  27: 'line_27_chikorita.txt',
  29: null,  // Turtwig — moved from line 75, col.n→col.m
  31: 'line_31_nidoranf.txt',
  39: 'line_39_togepi.txt',
  53: 'line_53_charmander.txt',
  61: 'line_61_mudkip.txt',
  69: 'line_69_torchic.txt',
  71: 'line_71_treecko.txt',
  73: 'line_73_wailmer.txt',
  75: 'line_75_nidoranm.txt',
  79: 'line_79_bulbasaur.txt',
  81: 'line_81_cyndaquil.txt',
  87: 'line_87_weedle.txt',
  89: 'line_89_cleffa.txt',
  93: 'line_93_pikachu.txt',
  97: 'line_97_beldum.txt',
  99: 'line_99_bagon.txt',
};

const NEW_LINES = [
  'line_new_dratini.txt',
  'line_new_machop.txt',
  'line_new_riolu.txt',
  'line_new_totodile.txt',
  'line_new_poliwag.txt',
];

// Step 1: Process Larvitar (line 21) — Rock/Dark redesign
console.log('\n--- Larvitar (line 21): Rock/Dark Tyranitar redesign ---');
const currentHeroes = fs.readFileSync(OUTPUT, 'utf-8').split('\n');
const larvitarFile = path.join(GENERATED, 'line_21_larvitar.txt');
if (fs.existsSync(larvitarFile)) {
  lines[20] = fs.readFileSync(larvitarFile, 'utf-8').trim();
  console.log('  Applied Larvitar redesign from generated/line_21_larvitar.txt');
} else {
  console.log('  WARNING: generated/line_21_larvitar.txt not found, keeping original');
}

// Step 2: Move Turtwig from line 75 to line 29 (col.n → col.m)
console.log('\n--- Turtwig: Move line 75 → 29, col.n → col.m ---');
const turtwigOrig = origLines[74]; // Original Turtwig at line 75
const turtwigMoved = turtwigOrig.replace(/\.col\.n/g, '.col.m');
lines[28] = turtwigMoved;
console.log('  Moved and changed color: ' + (turtwigMoved.match(/\.col\.m/g) || []).length + ' col.m replacements');

// Step 3: Apply hero replacements from generated files
console.log('\n--- Applying hero replacements ---');
let totalSpriteUpdates = 0;
let fixedParens = 0;

for (const [lineNumStr, file] of Object.entries(REPLACEMENTS)) {
  const lineNum = parseInt(lineNumStr);
  if (!file) continue; // Skip Larvitar/Turtwig (handled above)

  const filePath = path.join(GENERATED, file);
  if (!fs.existsSync(filePath)) {
    console.error(`  WARNING: Missing ${file}`);
    continue;
  }

  let content = fs.readFileSync(filePath, 'utf-8').trim();

  // Update sprites
  const spriteResult = updateSprites(content);
  content = spriteResult.line;
  totalSpriteUpdates += spriteResult.replacedCount;

  // Only fix paren structure if there's actually an issue (nested depth-1+ separators)
  const preErrors = validateHeroLine(content, lineNum);
  if (preErrors.some(e => e.includes('nested'))) {
    const fixed = fixHeroParens(content);
    if (fixed !== content) {
      content = fixed;
      fixedParens++;
    }
  }

  // Validate
  const errors = validateHeroLine(content, lineNum);
  if (errors.length > 0) {
    console.error(`  L${lineNum} ${file}: ERRORS — ${errors.join(', ')}`);
  } else {
    console.log(`  L${lineNum} ${file}: OK (${spriteResult.replacedCount} sprites)`);
  }

  // Write back to generated file (with updated sprites)
  fs.writeFileSync(filePath, content + '\n');
  lines[lineNum - 1] = content;
}

// Step 4: Add new hero lines after line 99
console.log('\n--- Adding new hero lines ---');
const newHeroContent = [];
for (const file of NEW_LINES) {
  const filePath = path.join(GENERATED, file);
  if (!fs.existsSync(filePath)) {
    console.error(`  WARNING: Missing ${file}`);
    continue;
  }

  let content = fs.readFileSync(filePath, 'utf-8').trim();

  // Update sprites
  const spriteResult = updateSprites(content);
  content = spriteResult.line;
  totalSpriteUpdates += spriteResult.replacedCount;

  // Only fix parens if there's an issue
  const preErrors = validateHeroLine(content, 'new');
  if (preErrors.some(e => e.includes('nested'))) {
    const fixed = fixHeroParens(content);
    if (fixed !== content) {
      content = fixed;
      fixedParens++;
    }
  }

  // Validate
  const errors = validateHeroLine(content, 'new');
  if (errors.length > 0) {
    console.error(`  ${file}: ERRORS — ${errors.join(', ')}`);
  } else {
    console.log(`  ${file}: OK (${spriteResult.replacedCount} sprites)`);
  }

  fs.writeFileSync(filePath, content + '\n');
  newHeroContent.push(content);
  newHeroContent.push(''); // blank spacer
}

// Insert new lines after line 100 (blank spacer after Bagon)
// Original line 101 starts the items section
const insertIdx = 100; // After line 100
lines.splice(insertIdx, 0, ...newHeroContent);

// Step 5: Apply display line changes (lines 3, 5, 7, 11)
console.log('\n--- Applying display line changes ---');
// Use the current heroes_only version for these since they were manually crafted
for (const lineNum of [3, 5, 7, 11]) {
  lines[lineNum - 1] = currentHeroes[lineNum - 1];
  console.log(`  L${lineNum}: Using existing display content`);
}

// Step 6: Apply the hidden trigger line (line 111 in current file)
// This is the level-up trigger that was added
const hiddenTriggerIdx = currentHeroes.findIndex(l => l.includes('hidden&temporary&ph.!m(hidden&temporary&level up)'));
if (hiddenTriggerIdx >= 0) {
  // Find where to put it in the new assembly
  const origTriggerIdx = lines.findIndex(l => l.includes('hidden&temporary&ph.!m(hidden&temporary&level up)'));
  if (origTriggerIdx >= 0) {
    // Already exists, update it
    lines[origTriggerIdx] = currentHeroes[hiddenTriggerIdx];
  }
}

// Step 7: Write output
fs.writeFileSync(OUTPUT, lines.join('\n'));
console.log(`\n=== Output written to: ${OUTPUT} ===`);
console.log(`Total lines: ${lines.length}`);
console.log(`Sprites updated: ${totalSpriteUpdates}`);
console.log(`Paren structures fixed: ${fixedParens}`);

// Step 8: Run validation
console.log('\n--- Final Validation ---');
const outLines = fs.readFileSync(OUTPUT, 'utf-8').split('\n');
const nonBlank = outLines.filter(l => l.trim().length > 0).length;
const blank = outLines.filter(l => l.trim().length === 0).length;
console.log(`Non-blank lines: ${nonBlank}`);
console.log(`Blank spacers: ${blank}`);

// Check all hero lines
let heroErrors = 0;
outLines.forEach((line, i) => {
  if (!line.includes('heropool') || !line.includes('.mn.')) return;
  const mn = line.match(/\.mn\.([^@]+)@/);
  const errors = validateHeroLine(line, i + 1);
  if (errors.length > 0) {
    console.error(`  L${i + 1} ${mn ? mn[1] : '?'}: ${errors.join(', ')}`);
    heroErrors++;
  }
});

if (heroErrors === 0) {
  console.log('✓ All hero lines valid');
} else {
  console.error(`✗ ${heroErrors} hero lines with errors`);
}

console.log('\n=== Rebuild complete ===');
