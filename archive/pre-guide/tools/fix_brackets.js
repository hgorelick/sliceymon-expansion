#!/usr/bin/env node
/**
 * Fix unbalanced parentheses in textmod_expanded.txt
 * For each line, counts ( and ) and appends missing closing parens if needed.
 */
const fs = require('fs');
const path = '/Users/hgorelick/Documents/slice-and-dice/textmod_expanded.txt';
const lines = fs.readFileSync(path, 'utf-8').split('\n');

let fixes = 0;
lines.forEach((line, idx) => {
  if (!line.trim()) return;
  let depth = 0;
  for (let i = 0; i < line.length; i++) {
    if (line[i] === '(') depth++;
    if (line[i] === ')') depth--;
  }
  if (depth > 0) {
    // Missing closing parens — need to insert them before the .mn. suffix
    // The suffix pattern is: .mn.Name@2!m(skip&hidden&temporary),
    // We need to add closing parens BEFORE .part.1&hidden.mn. or .mn.
    const mnMatch = line.match(/(.+?)(\.part\.1&hidden\.mn\..+|\.mn\..+)$/);
    if (mnMatch) {
      const body = mnMatch[1];
      const suffix = mnMatch[2];
      const closingParens = ')'.repeat(depth);
      lines[idx] = body + closingParens + suffix;
      console.log(`Line ${idx + 1}: Added ${depth} closing paren(s) before suffix`);
      fixes++;
    } else {
      // No .mn. suffix — just append at end
      const closingParens = ')'.repeat(depth);
      // Check if line ends with comma
      if (line.endsWith(',')) {
        lines[idx] = line.slice(0, -1) + closingParens + ',';
      } else {
        lines[idx] = line + closingParens;
      }
      console.log(`Line ${idx + 1}: Added ${depth} closing paren(s) at end`);
      fixes++;
    }
  }
});

fs.writeFileSync(path, lines.join('\n'));
console.log(`\nFixed ${fixes} lines. Verifying...`);

// Verify
const verifyLines = fs.readFileSync(path, 'utf-8').split('\n');
let remaining = 0;
verifyLines.forEach((line, idx) => {
  if (!line.trim()) return;
  let depth = 0;
  for (let i = 0; i < line.length; i++) {
    if (line[i] === '(') depth++;
    if (line[i] === ')') depth--;
  }
  if (depth !== 0) {
    console.log(`  STILL UNBALANCED - Line ${idx + 1}: depth=${depth}`);
    remaining++;
  }
});
if (remaining === 0) {
  console.log('All lines now have balanced parentheses!');
} else {
  console.log(`${remaining} lines still have issues.`);
}
