const fs = require('fs');

function fixHeroLine(line) {
  const poolMarker = line.includes('!mheropool.') ? '!mheropool.' : '!mHeropool.';
  const poolIdx = line.indexOf(poolMarker);
  if (poolIdx === -1) return line;
  
  const prefix = line.substring(0, poolIdx + poolMarker.length);
  const rest = line.substring(poolIdx + poolMarker.length);
  
  const partIdx = rest.indexOf('.part.1&hidden');
  if (partIdx === -1) return line;
  
  const content = rest.substring(0, partIdx);
  const suffix = rest.substring(partIdx);
  
  // Find all .speech. positions — each marks a tier boundary
  const tiers = [];
  const speechRegex = /\.speech\./g;
  let m;
  const speechPositions = [];
  while ((m = speechRegex.exec(content)) !== null) {
    speechPositions.push(m.index);
  }
  
  // For each speech position, find the .n.Name after it, then the tier boundary
  let segments = [];
  for (let i = 0; i < speechPositions.length; i++) {
    const speechPos = speechPositions[i];
    
    // The tier data is everything from the start (or after previous name+separator) to .speech.
    let dataStart;
    if (i === 0) {
      dataStart = 0;
    } else {
      dataStart = segments[i-1].afterName;
    }
    
    let tierData = content.substring(dataStart, speechPos);
    
    // Find .n.Name after .speech.
    const afterSpeech = content.substring(speechPos);
    const nameMatch = afterSpeech.match(/\.n\.([A-Z][A-Za-z ]*)/);
    const nameEndOffset = afterSpeech.indexOf(nameMatch[0]) + nameMatch[0].length;
    const nameEnd = speechPos + nameEndOffset;
    
    const speechAndName = content.substring(speechPos, nameEnd);
    
    // Determine what comes after the name
    let afterName = nameEnd;
    if (nameEnd < content.length && content[nameEnd] === '+') {
      afterName = nameEnd + 1;
    }
    
    segments.push({ dataStart, tierData, speechAndName, nameEnd, afterName });
  }
  
  // Reconstruct each tier with proper parens
  let result = '';
  for (let i = 0; i < segments.length; i++) {
    const seg = segments[i];
    let data = seg.tierData;
    
    if (i > 0) result += '+';
    
    // Strip leading + or +( or (
    data = data.replace(/^\+?\(?/, '');
    // Strip trailing ) chars 
    data = data.replace(/\)+$/, '');
    
    // Now count internal paren balance
    let balance = 0;
    for (const c of data) {
      if (c === '(') balance++;
      if (c === ')') balance--;
    }
    
    // If balance > 0, there are unclosed parens inside (like abilitydata) 
    // We need to add that many ) to close them
    const closingParens = ')'.repeat(balance + 1); // +1 for the replica paren itself
    
    result += '(';
    // Ensure data starts with replica. or a valid template name
    if (!data.startsWith('replica.') && !data.match(/^[A-Z][a-z]+\./)) {
      console.error('  WARNING: tier data does not start with replica.: ' + data.substring(0, 50));
    }
    result += data + closingParens + seg.speechAndName;
  }
  
  return prefix + result + suffix;
}

// Process files
const files = [
  'generated/line_69_torchic.txt',
  'generated/line_73_wailmer.txt',
  'generated/line_81_cyndaquil.txt',
  'generated/line_99_bagon.txt',
];

// First, restore originals from the textmod (in case previous fix corrupted them)
const textmod = fs.readFileSync('textmod_heroes_only.txt', 'utf8').split('\n');
const heroNames = { 69: 'Torchic', 73: 'Wailmer', 81: 'Cyndaquil', 99: 'Bagon' };

// Re-read from textmod to get uncorrupted versions
for (const lineNum of [69, 73, 81, 99]) {
  const line = textmod[lineNum - 1];
  const file = files[Object.keys(heroNames).indexOf(String(lineNum))];
  // Overwrite generated file with textmod version (which is the original generated)
  fs.writeFileSync(file, line + '\n');
}

// Now fix each file
files.forEach(file => {
  const orig = fs.readFileSync(file, 'utf8').trim();
  const fixed = fixHeroLine(orig);
  
  // Verify
  let balance = 0;
  for (const c of fixed) {
    if (c === '(') balance++;
    if (c === ')') balance--;
  }
  
  let d0 = 0;
  let depth = 0;
  for (let i = 0; i < fixed.length; i++) {
    if (fixed[i] === '(') depth++;
    if (fixed[i] === ')') depth--;
    if (fixed[i] === '+' && fixed[i+1] === '(' && depth === 0) d0++;
  }
  
  const mn = fixed.match(/\.mn\.([^@]+)@/);
  console.log((mn ? mn[1] : 'unknown') + ':');
  console.log('  Paren balance: ' + balance + (balance === 0 ? ' ✓' : ' ✗'));
  console.log('  Depth-0 +( count: ' + d0 + (d0 === 4 ? ' ✓' : ' ✗'));
  console.log('  Ends with comma: ' + fixed.endsWith(','));
  
  fs.writeFileSync(file, fixed + '\n');
  console.log('  Saved.');
  console.log();
});
