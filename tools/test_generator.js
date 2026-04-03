#!/usr/bin/env node
/**
 * Test the hero generator by parsing working original heroes
 * and verifying the generator can reproduce them exactly.
 *
 * Usage: node tools/test_generator.js
 */
const fs = require('fs');
const path = require('path');

const orig = fs.readFileSync(path.join(__dirname, '..', 'textmod.txt'), 'utf8').split('\n');

// Parse a hero line into its structural components
function parseHeroLine(line) {
  const phMatch = line.match(/ph\.b([a-z]+)/);
  const mnMatch = line.match(/\.mn\.([^@]+)@/);
  if (!phMatch || !mnMatch) return null;

  const internalName = phMatch[1];
  const mnName = mnMatch[1];

  // Find heropool content
  const poolMatch = line.match(/!m[hH]eropool\./);
  if (!poolMatch) return null;
  const poolStart = poolMatch.index + poolMatch[0].length;

  // Find .part.1
  const partIdx = line.indexOf('.part.1&hidden');
  const content = line.substring(poolStart, partIdx);

  // Split into tier segments at depth-0 + boundaries
  const tiers = [];
  let depth = 0, start = 0;
  for (let i = 0; i < content.length; i++) {
    if (content[i] === '(') depth++;
    if (content[i] === ')') depth--;
    if (depth === 0 && content[i] === '+' && content[i + 1] === '(') {
      tiers.push(content.substring(start, i));
      start = i + 1;
    }
  }
  tiers.push(content.substring(start));

  // Parse each tier
  const parsedTiers = tiers.map(tier => {
    // Find the main closing paren
    let d = 0, closeIdx = -1;
    for (let i = 0; i < tier.length; i++) {
      if (tier[i] === '(') d++;
      if (tier[i] === ')') { d--; if (d === 0) { closeIdx = i; break; } }
    }

    const inside = tier.substring(1, closeIdx);  // inside (...)
    const outside = tier.substring(closeIdx + 1); // after )

    return { inside, outside, full: tier };
  });

  return {
    internalName,
    mnName,
    tierCount: parsedTiers.length,
    tiers: parsedTiers,
    raw: line,
  };
}

// Test: parse a working hero, then reconstruct and compare
function testHero(lineNum) {
  const line = orig[lineNum - 1];
  const parsed = parseHeroLine(line);
  if (!parsed) {
    console.log(`  Line ${lineNum}: Could not parse`);
    return false;
  }

  // Reconstruct
  const poolMatch = line.match(/!m[hH]eropool\./);
  const prefix = line.substring(0, poolMatch.index + poolMatch[0].length);
  const partIdx = line.indexOf('.part.1&hidden');
  const suffix = line.substring(partIdx);

  let reconstructed = prefix;
  for (let i = 0; i < parsed.tiers.length; i++) {
    if (i > 0) reconstructed += '+';
    const t = parsed.tiers[i];
    reconstructed += '(' + t.inside + ')' + t.outside;
  }
  reconstructed += suffix;

  const match = reconstructed === line;
  console.log(`  Line ${lineNum} ${parsed.mnName}: ${match ? '✓ MATCH' : '✗ MISMATCH'} (${parsed.tierCount} tiers)`);

  if (!match) {
    // Find first difference
    for (let i = 0; i < Math.max(reconstructed.length, line.length); i++) {
      if (reconstructed[i] !== line[i]) {
        console.log(`    First diff at pos ${i}:`);
        console.log(`    Original:      ...${line.substring(Math.max(0,i-20), i+20)}...`);
        console.log(`    Reconstructed: ...${reconstructed.substring(Math.max(0,i-20), i+20)}...`);
        break;
      }
    }
  }

  // Also show the structure
  parsed.tiers.forEach((t, i) => {
    const hasAbility = t.inside.includes('.abilitydata.');
    const insideN = t.inside.match(/\.n\.([A-Z][A-Za-z ]*)/);
    const outsideN = t.outside.match(/\.n\.([A-Z][A-Za-z ]*)/);
    const insideSpeech = t.inside.includes('.speech.');
    const outsideSpeech = t.outside.includes('.speech.');

    console.log(`    T${i}: .n.=${insideN ? 'IN(' + insideN[1] + ')' : ''}${outsideN ? 'OUT(' + outsideN[1] + ')' : ''} .speech.=${insideSpeech ? 'IN' : ''}${outsideSpeech ? 'OUT' : ''} abilitydata=${hasAbility ? 'YES' : 'no'}`);
  });

  return match;
}

console.log('=== Testing parser against original working heroes ===\n');

// Test all hero lines in the original textmod
let passed = 0, failed = 0;
orig.forEach((line, i) => {
  if (!line.match(/heropool/i) || !line.includes('.mn.')) return;
  if (testHero(i + 1)) passed++;
  else failed++;
  console.log('');
});

console.log(`\n=== Results: ${passed} passed, ${failed} failed ===`);
