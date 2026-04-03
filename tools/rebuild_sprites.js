#!/usr/bin/env node
/**
 * Re-encode all sprites at 24x24 with proper transparency.
 * PMDCollab portraits are 40x40 — game expects 24x24.
 *
 * Usage: node tools/rebuild_sprites.js [compression]
 * Output: tools/sprite_encodings.json (overwritten)
 */

const { createCanvas, loadImage } = require('canvas');
const path = require('path');
const fs = require('fs');

const FORMAT = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ%=';
const TARGET_SIZE = 24; // Match original Sliceymon sprite size

// --- Encoder functions (from encode_sprite.js) ---

function closeEnough(a, b, CF) {
  if (a === b) return true;
  if (a === '000' || b === '000') return false;
  let diff = 0;
  for (let i = 0; i < 3; i++) {
    diff += Math.pow(Math.abs(FORMAT.indexOf(a[i]) - FORMAT.indexOf(b[i])), 2);
  }
  return Math.sqrt(diff) <= CF * 4;
}

function getColStr(pixels, i, palette, CF) {
  const r = pixels[i] / 4;
  const g = pixels[i + 1] / 4;
  const b = pixels[i + 2] / 4;
  const a = pixels[i + 3];
  if (a < 12) return '000';
  const colStr = FORMAT[Math.floor(r)] + FORMAT[Math.floor(g)] + FORMAT[Math.floor(b)];
  for (let j = 0; j < palette.length; j++) {
    if (closeEnough(palette[j], colStr, CF)) return palette[j];
  }
  return colStr;
}

function getString(sameCol, paletteIndex, paletteSize, posBits, paletteBits, extraVals) {
  const baseMax = Math.pow(2, posBits);
  const ev = paletteIndex >= extraVals.length ? 0 : extraVals[paletteIndex];
  const extraBitValue = Math.pow(2, posBits);
  const extraBitsMax = extraBitValue * ev;
  const actualMax = baseMax + extraBitsMax;
  let result = "!";
  const advance = Math.min(sameCol, actualMax);
  if (advance <= baseMax) {
    const charIndex = (paletteIndex << posBits) + (advance - 1);
    sameCol -= advance;
    result = FORMAT[charIndex];
  } else {
    const actualAdvance = Math.min(advance, actualMax);
    sameCol -= actualAdvance;
    const ebv = Math.floor(actualAdvance / extraBitValue);
    const ebvn = ebv * extraBitValue;
    const rightBits = actualAdvance - ebvn - 1;
    let leftBits = paletteIndex << posBits;
    if (ebvn > 0) {
      let extraOffset = 0;
      for (let i = 0; i < paletteIndex; i++) extraOffset += extraVals[i];
      leftBits = ((paletteSize - 1 + ebv + extraOffset) << posBits);
    }
    const charIndex = leftBits + rightBits;
    result = FORMAT[charIndex];
  }
  if (sameCol > 0) {
    return result + getString(sameCol, paletteIndex, paletteSize, posBits, paletteBits, extraVals);
  }
  return result;
}

function findUnusedChar(data) {
  for (let i = 0; i < FORMAT.length; i++) {
    if (!data.includes(FORMAT[i])) return FORMAT[i];
  }
  return null;
}

function compressV3(data) {
  const encodingParts = [];
  for (let repLen = 2; repLen <= 6; repLen++) {
    for (let attempt = 0; attempt < 64; attempt++) {
      const map = new Map();
      for (let si = 0; si < data.length - repLen; si++) {
        const part = data.substring(si, si + repLen);
        map.set(part, (map.get(part) || 0) + 1);
      }
      if (map.size === 0) continue;
      const val = [...map.entries()].reduce((a, e) => e[1] > a[1] ? e : a);
      const charsRemoved = val[1] * (repLen - 1) - 2 - repLen;
      if (charsRemoved <= 0) break;
      const unusedChar = findUnusedChar(data);
      if (unusedChar === null) break;
      data = data.replaceAll(val[0], unusedChar);
      encodingParts.push(unusedChar + val[0]);
    }
  }
  let result = "3" + FORMAT[encodingParts.length];
  for (let i = 0; i < encodingParts.length; i++) {
    result += (encodingParts[i].length - 1) + encodingParts[i];
  }
  result += data;
  return result;
}

async function encodeSprite(imagePath, compression = 4) {
  const img = await loadImage(imagePath);

  // Resize to TARGET_SIZE x TARGET_SIZE
  const canvas = createCanvas(TARGET_SIZE, TARGET_SIZE);
  const ctx = canvas.getContext('2d');

  // Use nearest-neighbor for pixel art (disable smoothing)
  ctx.imageSmoothingEnabled = false;
  ctx.drawImage(img, 0, 0, TARGET_SIZE, TARGET_SIZE);

  const imageData = ctx.getImageData(0, 0, TARGET_SIZE, TARGET_SIZE);
  const pixels = imageData.data;

  // PMDCollab sprites have solid backgrounds instead of transparency.
  // Detect the background color from corners and make it transparent.
  const corners = [
    0,                                        // top-left
    (TARGET_SIZE - 1) * 4,                    // top-right
    (TARGET_SIZE * (TARGET_SIZE - 1)) * 4,    // bottom-left
    (TARGET_SIZE * TARGET_SIZE - 1) * 4,      // bottom-right
  ];
  // Find the most common corner color
  const cornerColors = corners.map(i => `${pixels[i]},${pixels[i+1]},${pixels[i+2]}`);
  const freq = {};
  cornerColors.forEach(c => freq[c] = (freq[c] || 0) + 1);
  const bgColor = Object.entries(freq).sort((a,b) => b[1] - a[1])[0][0].split(',').map(Number);

  // Make all pixels matching the background color (within tolerance) transparent
  const BG_TOLERANCE = 30;
  for (let i = 0; i < pixels.length; i += 4) {
    const dr = Math.abs(pixels[i] - bgColor[0]);
    const dg = Math.abs(pixels[i+1] - bgColor[1]);
    const db = Math.abs(pixels[i+2] - bgColor[2]);
    if (dr + dg + db < BG_TOLERANCE) {
      pixels[i+3] = 0; // Make transparent
    }
  }

  // Extract palette
  const palette = [];
  const colCnt = {};
  for (let i = 0; i < pixels.length; i += 4) {
    const colStr = getColStr(pixels, i, palette, compression);
    if (!palette.includes(colStr)) {
      palette.push(colStr);
      colCnt[colStr] = 1;
      if (palette.length > 60) {
        throw new Error(`Palette too big (${palette.length}). Use higher compression.`);
      }
    } else {
      colCnt[colStr]++;
    }
  }

  // Sort palette: transparent first, then by frequency
  palette.sort((a, b) => {
    if (a === '000') return -1;
    if (b === '000') return 1;
    return colCnt[b] - colCnt[a];
  });

  // Encode pixel data
  const paletteBits = Math.ceil(Math.log(palette.length) / Math.log(2));
  const extraPaletteSlots = Math.pow(2, paletteBits) - palette.length;
  const extraBits = [];
  let epsc = extraPaletteSlots;
  while (epsc > 0) {
    const half = Math.round(epsc / 2);
    extraBits.push(half);
    epsc -= half;
  }

  const posBits = 6 - paletteBits;
  let dataString = '';
  let currentCol = -1;
  let sameCol = 0;

  for (let i = 0; i < pixels.length; i += 4) {
    const colStr = getColStr(pixels, i, palette, compression);
    if (currentCol === -1) {
      currentCol = colStr;
    } else if (currentCol !== colStr || i === pixels.length - 4) {
      if (i === pixels.length - 4) sameCol++;
      dataString += getString(sameCol, palette.indexOf(currentCol), palette.length, posBits, paletteBits, extraBits);
      sameCol = 0;
    }
    currentCol = colStr;
    sameCol++;
  }

  // Build raw string
  let finalData = "2" + FORMAT[TARGET_SIZE] + FORMAT[palette.length];
  for (let i = 0; i < palette.length; i++) {
    finalData += palette[i];
  }
  finalData += dataString;

  // Try V3 compression
  const compressed = compressV3(finalData);
  return compressed.length < finalData.length ? compressed : finalData;
}

// --- Main ---

async function main() {
  const compression = process.argv[2] || 'moderate';
  const CF = { none: 0, mild: 1, some: 2, moderate: 4, extreme: 6, hyper: 8, giga: 10 }[compression] || 4;

  const spritesDir = path.join(__dirname, 'sprites');
  const outFile = path.join(__dirname, 'sprite_encodings.json');

  const files = fs.readdirSync(spritesDir).filter(f => f.endsWith('.png'));
  console.error(`Re-encoding ${files.length} sprites at ${TARGET_SIZE}x${TARGET_SIZE} (compression: ${compression})...`);

  const results = {};
  let errors = [];

  for (let i = 0; i < files.length; i++) {
    const name = files[i].replace('.png', '');
    const pngPath = path.join(spritesDir, files[i]);
    try {
      const encoded = await encodeSprite(pngPath, CF);
      results[name] = encoded;
      process.stderr.write(`[${i + 1}/${files.length}] ${name} (${encoded.length} chars)\n`);
    } catch (e) {
      errors.push({ name, error: e.message });
      process.stderr.write(`[${i + 1}/${files.length}] ${name} FAILED: ${e.message}\n`);
    }
  }

  fs.writeFileSync(outFile, JSON.stringify(results, null, 2));
  console.error(`\nDone! ${Object.keys(results).length} sprites encoded, ${errors.length} errors.`);
  console.error(`Saved to: ${outFile}`);

  if (errors.length > 0) {
    console.error('\nFailed:');
    errors.forEach(e => console.error(`  ${e.name}: ${e.error}`));
  }
}

main().catch(e => { console.error(e); process.exit(1); });
