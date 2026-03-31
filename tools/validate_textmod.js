#!/usr/bin/env node
/**
 * Validates a Slice & Dice textmod file for common issues.
 * Run after every change to catch problems before pasting into the game.
 *
 * Usage: node validate_textmod.js [path]
 *   Default path: ../textmod_expanded.txt
 *
 * Exit code 0 = all clean, 1 = issues found
 */

const fs = require('fs');
const path = require('path');

const filePath = process.argv[2] || path.join(__dirname, '..', 'textmod_expanded.txt');
const text = fs.readFileSync(filePath, 'utf-8');
const lines = text.split('\n');

let errors = 0;
let warnings = 0;

function error(msg) { console.log(`  ❌ ERROR: ${msg}`); errors++; }
function warn(msg) { console.log(`  ⚠  WARN:  ${msg}`); warnings++; }
function ok(msg) { console.log(`  ✓  ${msg}`); }

console.log(`Validating: ${filePath}`);
console.log(`Lines: ${lines.length}`);
console.log('');

// ============================================================
// 1. Non-ASCII characters
// ============================================================
console.log('--- 1. Non-ASCII Characters ---');
let nonAsciiCount = 0;
const nonAsciiLocations = [];
lines.forEach((line, idx) => {
  for (let i = 0; i < line.length; i++) {
    if (line.charCodeAt(i) > 127) {
      nonAsciiCount++;
      if (nonAsciiLocations.length < 5) {
        nonAsciiLocations.push({ line: idx + 1, pos: i, char: line[i], code: line.charCodeAt(i) });
      }
    }
  }
});
if (nonAsciiCount === 0) {
  ok('No non-ASCII characters');
} else {
  error(`${nonAsciiCount} non-ASCII characters found`);
  nonAsciiLocations.forEach(l => console.log(`       Line ${l.line}, pos ${l.pos}: '${l.char}' (U+${l.code.toString(16).padStart(4, '0')})`));
}

// ============================================================
// 2. Parenthesis Balance
// ============================================================
console.log('\n--- 2. Parenthesis Balance ---');
let parenIssues = 0;
lines.forEach((line, idx) => {
  if (!line.trim()) return;
  let depth = 0, minDepth = 0;
  for (let i = 0; i < line.length; i++) {
    if (line[i] === '(') depth++;
    if (line[i] === ')') depth--;
    if (depth < minDepth) minDepth = depth;
  }
  if (depth !== 0) {
    error(`Line ${idx + 1}: Unbalanced parens (depth=${depth})`);
    parenIssues++;
  }
  if (minDepth < 0) {
    error(`Line ${idx + 1}: Negative paren depth (min=${minDepth}) — structural nesting error`);
    parenIssues++;
  }
});
if (parenIssues === 0) ok('All parentheses balanced');

// ============================================================
// 3. Line Endings (modifiers should end with comma)
// ============================================================
console.log('\n--- 3. Line Endings ---');
let commaIssues = 0;
lines.forEach((line, idx) => {
  if (!line.trim()) return;
  // Lines that legitimately don't end with comma:
  // - The very first line (=party...) starts with =
  // - The last line (end screen) may not
  // - Some lines are known exceptions
  if (!line.endsWith(',')) {
    // Check if it's a known exception
    const isFirst = line.startsWith('=party');
    const isLast = idx === lines.length - 1 || (idx === lines.length - 2 && !lines[lines.length - 1].trim());
    const isItemPool = line.startsWith('itempool.') && line.includes('.n.Choice');
    if (!isFirst && !isLast && !isItemPool) {
      warn(`Line ${idx + 1}: Does not end with comma`);
      commaIssues++;
    }
  }
});
if (commaIssues === 0) ok('All modifier lines end with comma');

// ============================================================
// 4. Duplicate .mn. Names
// ============================================================
console.log('\n--- 4. Duplicate Modifier Names ---');
const mnNames = {};
lines.forEach((line, idx) => {
  if (!line.trim()) return;
  const m = line.match(/\.mn\.([^,]+),?\s*$/);
  if (m) {
    const name = m[1];
    if (mnNames[name]) {
      error(`Duplicate .mn. "${name}" on lines ${mnNames[name]} and ${idx + 1}`);
    } else {
      mnNames[name] = idx + 1;
    }
  }
});
if (errors === parenIssues + nonAsciiCount + commaIssues) ok('No duplicate modifier names');

// ============================================================
// 5. Hero Line Validation
// ============================================================
console.log('\n--- 5. Hero Lines ---');
let heroIssues = 0;
lines.forEach((line, idx) => {
  if (!line.includes('heropool.')) return;
  const num = idx + 1;

  if (!line.includes('.sd.')) { warn(`Line ${num}: Hero missing .sd. (dice definition)`); heroIssues++; }
  if (!line.includes('.n.')) { error(`Line ${num}: Hero missing .n. (name)`); heroIssues++; }
  if (!line.includes('.img.')) { warn(`Line ${num}: Hero missing .img. (sprite)`); heroIssues++; }
  if (!line.includes('.mn.') && !line.includes('Missingno')) { error(`Line ${num}: Hero missing .mn. (modifier name)`); heroIssues++; }

  // Check for truncation (hero lines should be >1000 chars usually)
  if (line.length < 800 && !line.includes('Missingno')) {
    warn(`Line ${num}: Suspiciously short hero line (${line.length} chars)`);
    heroIssues++;
  }
});
if (heroIssues === 0) ok('All hero lines have required fields');

// ============================================================
// 6. Monster Line Validation
// ============================================================
console.log('\n--- 6. Monster Lines ---');
let monsterIssues = 0;
lines.forEach((line, idx) => {
  if (!line.includes('monsterpool.')) return;
  const num = idx + 1;

  // Monsters should NOT have cantrip
  if (line.match(/\b126-\d/)) {
    error(`Line ${num}: Monster has Cantrip face (126)! Monsters don't reroll.`);
    monsterIssues++;
  }

  // Monsters should use 170/171 for damage, not 15/36
  // But only check within sd. values (not in .img. sprite data)
  const sdMatches = line.match(/\.sd\.[^.]+/g) || [];
  sdMatches.forEach(sd => {
    if (sd.match(/\b15-\d/) && !line.includes('heropool') && !line.includes('Mareep')) {
      warn(`Line ${num}: Monster may have hero-style Damage face (15) instead of Enemy Damage (170)`);
      monsterIssues++;
    }
  });
});
if (monsterIssues === 0) ok('Monster lines look correct');

// ============================================================
// 7. Capture/Item Line Validation
// ============================================================
console.log('\n--- 7. Capture/Item Lines ---');
let captureIssues = 0;
lines.forEach((line, idx) => {
  if (!line.startsWith('itempool.')) return;
  const num = idx + 1;
  if (!line.includes('.mn.') && !line.includes('.n.')) {
    warn(`Line ${num}: Item pool line missing name`);
    captureIssues++;
  }
});
if (captureIssues === 0) ok('Capture/item lines look correct');

// ============================================================
// 8. File Structure
// ============================================================
console.log('\n--- 8. File Structure ---');
const nonBlankCount = lines.filter(l => l.trim()).length;
console.log(`  Non-blank lines (modifiers): ${nonBlankCount}`);
console.log(`  Blank lines (spacers): ${lines.length - nonBlankCount}`);

// Check that line 1 starts with =party
if (!lines[0].startsWith('=party')) {
  error('Line 1 does not start with =party');
}

// Check character selection exists
if (!lines.some(l => l.includes('ph.sWarning'))) {
  warn('Character selection line not found');
}

// ============================================================
// Summary
// ============================================================
console.log('\n========================================');
if (errors === 0 && warnings === 0) {
  console.log('✓ ALL CLEAN — ready for game');
  process.exit(0);
} else {
  console.log(`${errors} errors, ${warnings} warnings`);
  if (errors > 0) {
    console.log('✗ FIX ERRORS before pasting into game');
    process.exit(1);
  } else {
    console.log('⚠ Warnings only — may still work');
    process.exit(0);
  }
}
