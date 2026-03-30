#!/usr/bin/env node
/**
 * Slice & Dice Sprite Encoder
 * Ported from tann.fun/things/dice-img
 *
 * Usage: node encode_sprite.js <input.png> [compression]
 *   compression: none|mild|some|moderate|extreme|hyper|giga (default: none)
 *
 * Outputs the .img. encoded string to stdout.
 */

const { createCanvas, loadImage } = require('canvas');
const path = require('path');
const fs = require('fs');

// --- Encoder functions (ported from tann.fun/things/dice-img) ---

function getFormat() {
  return '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ%=';
}

function closeEnough(a, b, COMPRESSION_FACTOR, FORMAT) {
  if (a === b) return true;
  if (a === '000' || b === '000') return false;
  let diff = 0;
  for (let i = 0; i < 3; i++) {
    diff += Math.pow(Math.abs(FORMAT.indexOf(a[i]) - FORMAT.indexOf(b[i])), 2);
  }
  return Math.sqrt(diff) <= COMPRESSION_FACTOR * 4;
}

function getColStr(pixels, i, palette, COMPRESSION_FACTOR, FORMAT) {
  const ratio = 4;
  const r = pixels[i] / ratio;
  const g = pixels[i + 1] / ratio;
  const b = pixels[i + 2] / ratio;
  const a = pixels[i + 3];
  if (a < 12) {
    return '000';
  }
  const colStr = FORMAT[Math.floor(r)] + FORMAT[Math.floor(g)] + FORMAT[Math.floor(b)];
  for (let j = 0; j < palette.length; j++) {
    if (closeEnough(palette[j], colStr, COMPRESSION_FACTOR, FORMAT)) {
      return palette[j];
    }
  }
  return colStr;
}

function getString(sameCol, paletteIndex, paletteSize, posBits, paletteBits, extraVals, FORMAT) {
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
      for (let i = 0; i < paletteIndex; i++) {
        extraOffset += extraVals[i];
      }
      leftBits = ((paletteSize - 1 + ebv + extraOffset) << posBits);
    }
    const charIndex = leftBits + rightBits;
    result = FORMAT[charIndex];
  }
  if (sameCol > 0) {
    return result + getString(sameCol, paletteIndex, paletteSize, posBits, paletteBits, extraVals, FORMAT);
  } else {
    return result;
  }
}

function findUnusedChar(data) {
  const FORMAT = getFormat();
  for (let i = 0; i < FORMAT.length; i++) {
    const ch = FORMAT[i];
    if (!data.includes(ch)) {
      return ch;
    }
  }
  return null;
}

function compressV3(data) {
  const FORMAT = getFormat();
  const encodingParts = [];
  for (let repLen = 2; repLen <= 6; repLen++) {
    for (let attempt = 0; attempt < 64; attempt++) {
      const map = new Map();
      for (let stringIndex = 0; stringIndex < data.length - repLen; stringIndex++) {
        const part = data.substring(stringIndex, stringIndex + repLen);
        if (map.get(part) === undefined) {
          map.set(part, 1);
        } else {
          map.set(part, map.get(part) + 1);
        }
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

const MAX_PALETTE = 60;

const COMPRESSION_LEVELS = {
  'none': 0,
  'mild': 1,
  'some': 2,
  'moderate': 4,
  'extreme': 6,
  'hyper': 8,
  'giga': 10
};

async function encodeImage(imagePath, compressionLevel = 'none') {
  const FORMAT = getFormat();
  const COMPRESSION_FACTOR = COMPRESSION_LEVELS[compressionLevel] || 0;

  const img = await loadImage(imagePath);

  // Canvas at original image size
  const canvas = createCanvas(img.width, img.height);
  const ctx = canvas.getContext('2d');
  ctx.drawImage(img, 0, 0);

  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  const pixels = imageData.data;

  // First pass: extract palette
  const palette = [];
  const colCnt = {};
  for (let i = 0; i < pixels.length; i += 4) {
    const colStr = getColStr(pixels, i, palette, COMPRESSION_FACTOR, FORMAT);
    if (!palette.includes(colStr)) {
      palette.push(colStr);
      colCnt[colStr] = 1;
      if (palette.length > MAX_PALETTE) {
        throw new Error(`Palette too big (${palette.length} colors, max ${MAX_PALETTE}). Use higher compression or reduce image colors.`);
      }
    } else {
      colCnt[colStr]++;
    }
  }

  // Sort palette by frequency (most used first), keeping '000' (transparent) at front if present
  palette.sort((a, b) => {
    if (a === '000') return -1;
    if (b === '000') return 1;
    return colCnt[b] - colCnt[a];
  });

  // Second pass: encode pixel data
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
    const colStr = getColStr(pixels, i, palette, COMPRESSION_FACTOR, FORMAT);
    if (currentCol === -1) {
      currentCol = colStr;
    } else if (currentCol !== colStr || i === pixels.length - 4) {
      if (i === pixels.length - 4) sameCol++;
      dataString += getString(sameCol, palette.indexOf(currentCol), palette.length, posBits, paletteBits, extraBits, FORMAT);
      sameCol = 0;
    }
    currentCol = colStr;
    sameCol++;
  }

  // Build final string
  let finalData = "2"
    + FORMAT[canvas.width]
    + FORMAT[palette.length];
  for (let i = 0; i < palette.length; i++) {
    finalData += palette[i];
  }
  finalData += dataString;

  // Try V3 compression
  const compressed = compressV3(finalData);
  if (compressed.length < finalData.length) {
    finalData = compressed;
  }

  return finalData;
}

// --- CLI ---

async function main() {
  const args = process.argv.slice(2);
  if (args.length === 0) {
    console.error('Usage: node encode_sprite.js <input.png> [compression]');
    console.error('  compression: none|mild|some|moderate|extreme|hyper|giga (default: none)');
    process.exit(1);
  }

  const imagePath = args[0];
  const compression = args[1] || 'none';

  if (!fs.existsSync(imagePath)) {
    console.error(`File not found: ${imagePath}`);
    process.exit(1);
  }

  try {
    const encoded = await encodeImage(imagePath, compression);
    console.log(encoded);
  } catch (err) {
    console.error(`Error: ${err.message}`);
    process.exit(1);
  }
}

main();
