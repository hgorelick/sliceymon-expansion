#!/usr/bin/env node
/**
 * apply_phase6.js — Phase 6: Update character selection & remove Togepi/Riolu eggs
 *
 * Modifies textmod_expanded.txt to:
 * 1. Update Line 11 (character selection draft picker) with new hero names
 *    - Phase 1 and Phase 3 (repeats of Phase 1) name swaps
 *    - Phase 2 and Phase 4 (repeats of Phase 2) name swaps
 *    - Add new colors E and J
 *    - Replace Ditto with Bagon (P1) / Dratini (P2) for color W
 * 2. Remove Togepi and Riolu egg items from the item pool (lines 119, 105)
 *    while preserving their hero definitions (line 39)
 *
 * Usage: node apply_phase6.js
 */

const fs = require('fs');
const path = require('path');

const TEXTMOD_PATH = path.join(__dirname, '..', 'textmod_expanded.txt');

// ============================================================
// Read the file
// ============================================================
let content = fs.readFileSync(TEXTMOD_PATH, 'utf8');
let lines = content.split('\n');

console.log(`Read ${lines.length} lines from textmod_expanded.txt`);

// ============================================================
// TASK 1: Update Line 11 (Character Selection)
// Line 11 is at index 10 (0-indexed)
// ============================================================

let line11 = lines[10];
console.log(`\nLine 11 length: ${line11.length} characters`);

// The line has multiple phases separated by "&hidden,"
// Phase 1: party.(dabble...) - first block
// Phase 2: add.(dabble...) - second block
// Phase 3: add.(dabble...) - third block (repeat of Phase 1 names)
// Phase 4: add.(dabble...) - fourth block (repeat of Phase 2 names)
// etc.

// Split into the phase blocks
const phaseBlocks = line11.split('&hidden,');
console.log(`Found ${phaseBlocks.length} phase blocks`);

// Identify which blocks are Phase 1 type (P1/P3/P5) vs Phase 2 type (P2/P4/P6)
// Phase 1 type contains: gible, vanillite, magikarp, slakoth, larvitar, eevee, litwick, fomantis, rockruff, applin, etc.
// Phase 2 type contains: axew, spheal, feebas, varoom, lillipup, porygon, espurr, sunkern, roggenrola, turtwig, etc.

// Phase 1/3/5 replacements (display text and variable references):
const phase1Replacements = [
  // [color, oldName, newName]
  ['L', 'fomantis', 'chikorita'],
  ['R', 'darumaka', 'togepi'],
  ['Z', 'agumon', 'charmander'],
  ['M', 'rockruff', 'turtwig'],
  ['N', 'applin', 'nidoranf'],
  // W: ditto -> bagon
  ['W', 'ditto', 'bagon'],
];

// Phase 2/4/6 replacements:
const phase2Replacements = [
  // [color, oldName, newName]
  ['G', 'varoom', 'mudkip'],
  ['K', 'espurr', 'torchic'],
  ['L', 'sunkern', 'treecko'],
  ['M', 'roggenrola', 'wailmer'],
  ['N', 'turtwig', 'nidoranf'],
  // O: trapinch stays — no change
  ['P', 'trubbish', 'bulbasaur'],
  ['Q', 'slugma', 'cyndaquil'],
  ['T', 'burmy', 'weedle'],
  ['U', 'tinkatink', 'cleffa'],
  ['X', 'joltik', 'pikachu'],
  ['Z', 'tentomon', 'beldum'],
  // W: ditto -> dratini
  ['W', 'ditto', 'dratini'],
];

// New colors to add:
// E: machop (Phase1) / riolu (Phase2)
// J: totodile (Phase1) / poliwag (Phase2)

// Helper: determine if a phase block is Phase 1 type or Phase 2 type
function isPhase1Type(block) {
  // Phase 1 type blocks contain "gible" and use "party." for the first one or "add." for repeats
  return block.includes('gible') && block.includes('vanillite');
}

function isPhase2Type(block) {
  // Phase 2 type blocks contain "axew" or will contain "axew" and "spheal"
  return block.includes('axew') || block.includes('spheal');
}

// Helper: apply a single name replacement in a block
// Pattern: [color]LETTER - oldname[cu]...oldnameV1
// Need to replace in display text and variable reference
function applyReplacement(block, color, oldName, newName) {
  let modified = block;

  // Replace display text: "LETTER - oldname[cu]" -> "LETTER - newname[cu]"
  const displayPattern = `${color} - ${oldName}[cu]`;
  const displayReplace = `${color} - ${newName}[cu]`;
  modified = modified.split(displayPattern).join(displayReplace);

  // Replace variable reference: "!voldnameV1" -> "!vnewnameV1"
  const varPattern = `!v${oldName}V1`;
  const varReplace = `!v${newName}V1`;
  modified = modified.split(varPattern).join(varReplace);

  return modified;
}

// Helper: generate a new entry for a color
// Phase 1 format (first block uses "party.", repeats use "add."):
// @1[COLOR_NAME]LETTER - pokemonname[cu]@2!m(party.(dabble.tier.0.n.LETTER.col.LETTER.img.THUMBNAIL))@2!vpokemonnameV1
// Phase 2+ use "add." instead of "party."
function generateEntry(colorName, colorLetter, pokemonName, useParty) {
  const action = useParty ? 'party' : 'add';
  // Use a placeholder thumbnail - same format as others (2k1 + 6 chars + f)
  // We'll use a generic one since we can't generate real thumbnails here
  const thumbnail = '2k1000======f';
  return `@1[${colorName}]${colorLetter} - ${pokemonName}[cu]@2!m(${action}.(dabble.tier.0.n.${colorLetter}.col.${colorLetter}.img.${thumbnail}))@2!v${pokemonName}V1`;
}

// Process each phase block
let changeCount = 0;

for (let i = 0; i < phaseBlocks.length; i++) {
  let block = phaseBlocks[i];

  if (isPhase1Type(block)) {
    console.log(`  Block ${i}: Phase 1 type (contains gible/vanillite)`);

    // Apply Phase 1 replacements
    for (const [color, oldName, newName] of phase1Replacements) {
      const before = block;
      block = applyReplacement(block, color, oldName, newName);
      if (block !== before) {
        console.log(`    Replaced ${oldName} -> ${newName} (color ${color})`);
        changeCount++;
      }
    }

    // Add new colors E and J (Phase 1: machop, totodile)
    // Insert before the [dark]D - Skip entry
    const isFirstBlock = block.includes('party.(dabble');
    const useParty = isFirstBlock;

    const eEntry = generateEntry('euish', 'E', 'machop', useParty);
    const jEntry = generateEntry('juish', 'J', 'totodile', useParty);

    // Insert E after the D (Skip) entry... actually, let's insert before Skip
    // Find the Skip entry pattern
    const skipPattern = `@1[dark]D - Skip`;
    if (block.includes(skipPattern) && !block.includes('machop')) {
      // Insert E and J entries before Skip
      block = block.replace(skipPattern, `${eEntry}${jEntry}${skipPattern}`);
      console.log(`    Added machop (E) and totodile (J) entries`);
      changeCount += 2;
    }

    phaseBlocks[i] = block;

  } else if (isPhase2Type(block)) {
    console.log(`  Block ${i}: Phase 2 type (contains axew/spheal)`);

    // Apply Phase 2 replacements
    for (const [color, oldName, newName] of phase2Replacements) {
      const before = block;
      block = applyReplacement(block, color, oldName, newName);
      if (block !== before) {
        console.log(`    Replaced ${oldName} -> ${newName} (color ${color})`);
        changeCount++;
      }
    }

    // Add new colors E and J (Phase 2: riolu, poliwag)
    const eEntry = generateEntry('euish', 'E', 'riolu', false);
    const jEntry = generateEntry('juish', 'J', 'poliwag', false);

    const skipPattern = `@1[dark]D - Skip`;
    if (block.includes(skipPattern) && !block.includes('riolu')) {
      block = block.replace(skipPattern, `${eEntry}${jEntry}${skipPattern}`);
      console.log(`    Added riolu (E) and poliwag (J) entries`);
      changeCount += 2;
    }

    phaseBlocks[i] = block;

  } else {
    console.log(`  Block ${i}: Unknown/terminal block (length ${block.length})`);
  }
}

// Rejoin
lines[10] = phaseBlocks.join('&hidden,');
console.log(`\nTotal character selection changes: ${changeCount}`);

// ============================================================
// TASK 2: Remove Togepi and Riolu Eggs from Item Pool
// ============================================================
// Eggs are on lines 119 and 105 (1-indexed)
// They appear as item entries within those lines, separated by "+"
// We need to find and remove entries containing ".n.Togepi" and ".n.Riolu"
// that are egg items, NOT the hero definitions on line 39

console.log('\n--- Task 2: Remove Togepi/Riolu Eggs ---');

// Process lines that contain egg entries (119 and 105, 1-indexed)
const eggLines = [118, 104]; // 0-indexed: line 119 and line 105

for (const lineIdx of eggLines) {
  if (lineIdx >= lines.length) {
    console.log(`  Line ${lineIdx + 1} out of range, skipping`);
    continue;
  }

  let line = lines[lineIdx];
  const lineNum = lineIdx + 1;

  // Check if this line contains Togepi or Riolu
  const hasTogepi = line.includes('.n.Togepi') || line.includes('mn.Togepi');
  const hasRiolu = line.includes('.n.Riolu') || line.includes('mn.Riolu');

  if (!hasTogepi && !hasRiolu) {
    console.log(`  Line ${lineNum}: No Togepi/Riolu entries found`);
    continue;
  }

  console.log(`  Line ${lineNum}: Found Togepi=${hasTogepi}, Riolu=${hasRiolu}`);

  // Items in the pool are separated by "+"
  // Each egg item will contain ".n.Togepi" or ".n.Riolu" in its definition
  // and will have "mn.Togepi" or "mn.Riolu" at the end
  //
  // We need to split by "+" and remove segments that are Togepi/Riolu eggs
  // But be careful: the "+" delimiter is also used within item definitions
  // A better approach: find and remove the complete egg entries

  // The egg entries follow a pattern like:
  // +(ITEM_DEFINITION.n.Togepi...mn.Togepi Egg variant)
  // or they could be separated by "+" at the top level

  // Strategy: Use regex to find and remove Togepi/Riolu egg entries
  // Egg entries contain ".n.Togepi" and end with something like "mn.Togepi..."
  // They are preceded by "+" in the item pool

  let removedCount = 0;

  // Remove Togepi egg entries
  // Pattern: a "+" followed by content containing Togepi item definition up to the next "+" or end
  // The entries look like: +(...n.Togepi...mn.Togepi SOMETHING)
  // More precisely, entries in the item pool are delimited by )+( or start/end patterns

  // Let's try a different approach: split the line into individual item entries
  // Item entries in the advanced pool are typically enclosed in parentheses and
  // separated. Let's look for "+(" patterns that start new items.

  // Actually, the simplest reliable approach: find each occurrence of a Togepi/Riolu
  // egg definition and remove the complete entry (from the preceding "+" to the next
  // entry boundary)

  // Approach: Find "mn.Togepi" or "mn.Riolu" markers, then work backwards to find
  // the start of that entry (the preceding "+") and forward to find the end.

  // The items in these lines are separated by "+" at the top level.
  // Each item ends with .mn.NAME,  or .mn.NAME VARIANT,
  // So we can split on the pattern that separates items.

  // Better approach: use regex to remove entries
  // An egg entry will match: \+\([^)]*\.n\.Togepi[^)]*\)  but items can have nested parens

  // Most reliable: find "mn.Togepi" occurrences and trace back to the "+" that starts
  // that item entry, then forward to find where it ends.

  // Let's use a regex that matches from "+" to "mn.Togepi..." up to the next comma or "+"
  // Pattern for removing: \+\(.*?\.n\.Togepi.*?\.mn\.Togepi[^,]*,?

  // Actually let me think about the structure more carefully.
  // The line has items like: (item1.mn.Name1)+(item2.mn.Name2)+...
  // OR: item1.mn.Name1,item2.mn.Name2,...
  // The "+" joins sub-pools and items within them.

  // Since items contain nested parens, let's use a paren-counting approach.

  if (hasTogepi) {
    // Find all positions of ".n.Togepi" in the line
    let searchStr = '.n.Togepi';
    let pos = 0;
    while ((pos = line.indexOf(searchStr, pos)) !== -1) {
      // This could be a hero line or an egg line. On lines 105/119, these are eggs.
      // Find the "mn.Togepi" that ends this entry
      const mnPos = line.indexOf('mn.Togepi', pos);
      if (mnPos === -1) {
        pos++;
        continue;
      }
      // Find the end of this entry name (up to comma or end)
      let entryEnd = mnPos;
      while (entryEnd < line.length && line[entryEnd] !== ',') {
        entryEnd++;
      }
      if (entryEnd < line.length && line[entryEnd] === ',') {
        entryEnd++; // Include the comma
      }

      // Find the start of this entry - search backwards for "+" that starts this item
      let entryStart = pos;
      // Go back to find the "+(" that starts this entry
      let parenDepth = 0;
      for (let j = pos - 1; j >= 0; j--) {
        if (line[j] === ')') parenDepth++;
        if (line[j] === '(') parenDepth--;
        if (line[j] === '+' && parenDepth <= 0) {
          entryStart = j;
          break;
        }
      }

      const removed = line.substring(entryStart, entryEnd);
      console.log(`    Removing Togepi egg entry (${removed.length} chars): ...${removed.substring(0, 80)}...`);
      line = line.substring(0, entryStart) + line.substring(entryEnd);
      removedCount++;
      // Don't advance pos since string shifted
    }
  }

  if (hasRiolu) {
    let searchStr = '.n.Riolu';
    let pos = 0;
    while ((pos = line.indexOf(searchStr, pos)) !== -1) {
      const mnPos = line.indexOf('mn.Riolu', pos);
      if (mnPos === -1) {
        pos++;
        continue;
      }
      let entryEnd = mnPos;
      while (entryEnd < line.length && line[entryEnd] !== ',') {
        entryEnd++;
      }
      if (entryEnd < line.length && line[entryEnd] === ',') {
        entryEnd++;
      }

      let entryStart = pos;
      let parenDepth = 0;
      for (let j = pos - 1; j >= 0; j--) {
        if (line[j] === ')') parenDepth++;
        if (line[j] === '(') parenDepth--;
        if (line[j] === '+' && parenDepth <= 0) {
          entryStart = j;
          break;
        }
      }

      const removed = line.substring(entryStart, entryEnd);
      console.log(`    Removing Riolu egg entry (${removed.length} chars): ...${removed.substring(0, 80)}...`);
      line = line.substring(0, entryStart) + line.substring(entryEnd);
      removedCount++;
    }
  }

  lines[lineIdx] = line;
  console.log(`  Line ${lineNum}: Removed ${removedCount} egg entries`);
}

// ============================================================
// Verify hero lines are untouched
// ============================================================
const line39 = lines[38]; // 0-indexed
if (line39.includes('.n.Togepi') || line39.includes('mn.Togepi')) {
  console.log('\nVerification: Line 39 (Togepi hero) still intact - OK');
} else {
  console.log('\nWARNING: Line 39 (Togepi hero) may have been modified!');
}

// Also check that Riolu hero definition wasn't touched
// (Riolu hero might be on a different line - let's check)
let rioluHeroFound = false;
for (let i = 0; i < lines.length; i++) {
  if (i === 104 || i === 118) continue; // Skip egg lines
  if (lines[i].includes('.n.Riolu')) {
    rioluHeroFound = true;
    console.log(`Verification: Riolu hero on line ${i + 1} still intact - OK`);
  }
}

// ============================================================
// Write the file
// ============================================================
const output = lines.join('\n');
fs.writeFileSync(TEXTMOD_PATH, output, 'utf8');

const newLines = output.split('\n');
console.log(`\nWrote ${newLines.length} lines to textmod_expanded.txt`);

// ============================================================
// Final verification
// ============================================================
console.log('\n--- Final Verification ---');

// Check Line 11 for expected new names
const newLine11 = newLines[10];

const phase1Expected = ['chikorita', 'togepi', 'charmander', 'turtwig', 'nidoranf', 'bagon', 'machop', 'totodile'];
const phase2Expected = ['mudkip', 'torchic', 'treecko', 'wailmer', 'nidoranf', 'bulbasaur', 'cyndaquil', 'weedle', 'cleffa', 'pikachu', 'beldum', 'dratini', 'riolu', 'poliwag'];

console.log('Phase 1/3 names in Line 11:');
for (const name of phase1Expected) {
  const count = (newLine11.match(new RegExp(name, 'g')) || []).length;
  console.log(`  ${name}: ${count} occurrences ${count > 0 ? 'OK' : 'MISSING!'}`);
}

console.log('Phase 2/4 names in Line 11:');
for (const name of phase2Expected) {
  const count = (newLine11.match(new RegExp(name, 'g')) || []).length;
  console.log(`  ${name}: ${count} occurrences ${count > 0 ? 'OK' : 'MISSING!'}`);
}

// Check removed names are gone (except those that stay)
const removedPhase1 = ['fomantis', 'darumaka', 'agumon', 'rockruff', 'applin'];
const removedPhase2 = ['varoom', 'espurr', 'sunkern', 'roggenrola', 'trubbish', 'slugma', 'burmy', 'tinkatink', 'joltik', 'tentomon'];

console.log('Removed Phase 1 names (should be 0):');
for (const name of removedPhase1) {
  const count = (newLine11.match(new RegExp(name, 'g')) || []).length;
  console.log(`  ${name}: ${count} occurrences ${count === 0 ? 'OK' : 'STILL PRESENT!'}`);
}

console.log('Removed Phase 2 names (should be 0):');
for (const name of removedPhase2) {
  const count = (newLine11.match(new RegExp(name, 'g')) || []).length;
  console.log(`  ${name}: ${count} occurrences ${count === 0 ? 'OK' : 'STILL PRESENT!'}`);
}

// Check ditto is fully removed from line 11
const dittoCount = (newLine11.match(/ditto/g) || []).length;
console.log(`\nditto in Line 11: ${dittoCount} occurrences ${dittoCount === 0 ? 'OK' : 'STILL PRESENT!'}`);

// Check Togepi/Riolu eggs removed from item lines
const line105 = newLines[104];
const line119 = newLines[118];
const togepiInItems = (line119.match(/\.n\.Togepi/g) || []).length;
const rioluInItems = ((line105.match(/\.n\.Riolu/g) || []).length) + ((line119.match(/\.n\.Riolu/g) || []).length);
console.log(`\nTogepi eggs in line 119: ${togepiInItems} ${togepiInItems === 0 ? 'OK' : 'STILL PRESENT!'}`);
console.log(`Riolu eggs in lines 105/119: ${rioluInItems} ${rioluInItems === 0 ? 'OK' : 'STILL PRESENT!'}`);

// Verify noibat and trapinch are unchanged
const noibatCount = (newLine11.match(/noibat/g) || []).length;
const trapinchCount = (newLine11.match(/trapinch/g) || []).length;
console.log(`\nnoibat (should stay): ${noibatCount} occurrences ${noibatCount > 0 ? 'OK' : 'MISSING!'}`);
console.log(`trapinch (should stay): ${trapinchCount} occurrences ${trapinchCount > 0 ? 'OK' : 'MISSING!'}`);

console.log('\nPhase 6 complete!');
