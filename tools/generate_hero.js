#!/usr/bin/env node
/**
 * Generate a hero line following proven working patterns from the original textmod.
 *
 * Rules learned from testing:
 * 1. .n.NAME must be LAST before + or line end (always OUTSIDE replica parens)
 * 2. .speech. and .doc. also go OUTSIDE replica parens
 * 3. .abilitydata.() goes INSIDE replica parens
 * 4. Spell face IDs must be valid for the spell template (Fey/lost/Statue each have restrictions)
 * 5. Spells should have only 1 active face (except Statue which tolerates 2)
 * 6. .k.KEYWORD must have .i.POSITION prefix (e.g., .i.topbot.k.poison)
 * 7. Only add abilitydata if the ORIGINAL hero at that line position had abilitydata
 * 8. .n. for spell names goes INSIDE abilitydata parens
 *
 * Usage: node tools/generate_hero.js <config.json>
 */
const fs = require('fs');
const path = require('path');
const SPRITES = require(path.join(__dirname, 'sprite_encodings.json'));

function buildHeroLine(config) {
  const { internalName, template, color, tiers, mnName } = config;

  let line = `hidden&temporary&ph.b${internalName};1;!mheropool.`;

  for (let i = 0; i < tiers.length; i++) {
    const t = tiers[i];
    if (i > 0) line += '+';

    // === INSIDE the replica paren ===
    line += `(replica.${template}`;
    if (t.tier) line += `.tier.${t.tier}`;
    line += `.col.${color}`;
    line += `.hp.${t.hp}`;

    // Keywords/items INSIDE paren (must have .i.POSITION prefix)
    if (t.keywordsBefore) line += t.keywordsBefore;

    line += `.sd.${t.sd}`;

    // Sprite
    const sprite = SPRITES[t.spriteName];
    if (!sprite) throw new Error(`No sprite for "${t.spriteName}"`);
    line += `.img.${sprite}`;

    // Keywords/items after img (some heroes put items here)
    if (t.keywordsAfter) line += t.keywordsAfter;

    // Abilitydata INSIDE paren (spell .n. is inside abilitydata parens)
    if (t.abilitydata) line += `.abilitydata.${t.abilitydata}`;

    // Close the replica paren
    line += ')';

    // === OUTSIDE the replica paren ===
    // Items that go outside (e.g., .i.self.Bottom Poison^1/1)
    if (t.itemsOutside) line += t.itemsOutside;

    // Doc (description) outside
    if (t.doc) line += `.doc.${t.doc}`;

    // Speech outside
    line += `.speech.${t.speech}`;

    // .n.NAME must be LAST before + or end
    line += `.n.${t.name}`;
  }

  line += `.part.1&hidden.mn.${mnName}@2!m(skip&hidden&temporary),`;
  return line;
}

// Read config
const configFile = process.argv[2];
if (!configFile) { console.error('Usage: node generate_hero.js <config.json>'); process.exit(1); }
const config = JSON.parse(fs.readFileSync(configFile, 'utf8'));
const line = buildHeroLine(config);

// Validate
let bal = 0;
for (const c of line) { if (c === '(') bal++; if (c === ')') bal--; }
if (bal !== 0) console.error(`WARNING: paren imbalance ${bal}`);

// Check for bare .k. without .i.POSITION prefix (must have .i.SOMETHING.k. or .i.k.)
const bareK = line.match(/(?<!\.[a-z0-9]+)\.k\.[a-z]/g);
if (bareK) console.error(`WARNING: ${bareK.length} bare .k. without .i. prefix — keywords need .i.POSITION prefix`);
console.error(`Generated: ${config.mnName} (${line.length} chars, paren balance: ${bal})`);

process.stdout.write(line);
