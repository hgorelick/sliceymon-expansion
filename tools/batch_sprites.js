#!/usr/bin/env node
/**
 * Batch Pokemon Sprite Downloader + Encoder
 * Downloads portraits from PMDCollab and encodes them for Slice & Dice textmod.
 *
 * Usage: node batch_sprites.js [compression]
 *   compression: none|mild|some|moderate|extreme|hyper|giga (default: moderate)
 *
 * Outputs JSON mapping: { "Pokemon Name": "encoded_string", ... }
 */

const https = require('https');
const fs = require('fs');
const path = require('path');
const { createCanvas, loadImage } = require('canvas');

// Import encoder from encode_sprite.js by extracting the core function
// (We inline the encoder here to keep it self-contained)

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
  if (a < 12) return '000';
  const colStr = FORMAT[Math.floor(r)] + FORMAT[Math.floor(g)] + FORMAT[Math.floor(b)];
  for (let j = 0; j < palette.length; j++) {
    if (closeEnough(palette[j], colStr, COMPRESSION_FACTOR, FORMAT)) return palette[j];
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
      for (let i = 0; i < paletteIndex; i++) extraOffset += extraVals[i];
      leftBits = ((paletteSize - 1 + ebv + extraOffset) << posBits);
    }
    const charIndex = leftBits + rightBits;
    result = FORMAT[charIndex];
  }
  if (sameCol > 0) {
    return result + getString(sameCol, paletteIndex, paletteSize, posBits, paletteBits, extraVals, FORMAT);
  }
  return result;
}

function findUnusedChar(data) {
  const FORMAT = getFormat();
  for (let i = 0; i < FORMAT.length; i++) {
    if (!data.includes(FORMAT[i])) return FORMAT[i];
  }
  return null;
}

function compressV3(data) {
  const FORMAT = getFormat();
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
      if (val[1] * (repLen - 1) - 2 - repLen <= 0) break;
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
const COMPRESSION_LEVELS = { none: 0, mild: 1, some: 2, moderate: 4, extreme: 6, hyper: 8, giga: 10 };

async function encodeImageBuffer(buffer, compressionLevel = 'moderate') {
  const FORMAT = getFormat();
  const COMPRESSION_FACTOR = COMPRESSION_LEVELS[compressionLevel] || 4;
  const img = await loadImage(buffer);
  const canvas = createCanvas(img.width, img.height);
  const ctx = canvas.getContext('2d');
  ctx.drawImage(img, 0, 0);
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  const pixels = imageData.data;

  const palette = [];
  const colCnt = {};
  for (let i = 0; i < pixels.length; i += 4) {
    const colStr = getColStr(pixels, i, palette, COMPRESSION_FACTOR, FORMAT);
    if (!palette.includes(colStr)) {
      palette.push(colStr);
      colCnt[colStr] = 1;
      if (palette.length > MAX_PALETTE) throw new Error(`Palette too big (${palette.length})`);
    } else {
      colCnt[colStr]++;
    }
  }
  palette.sort((a, b) => {
    if (a === '000') return -1;
    if (b === '000') return 1;
    return colCnt[b] - colCnt[a];
  });

  const paletteBits = Math.ceil(Math.log(palette.length) / Math.log(2));
  const extraPaletteSlots = Math.pow(2, paletteBits) - palette.length;
  const extraBits = [];
  let epsc = extraPaletteSlots;
  while (epsc > 0) { const half = Math.round(epsc / 2); extraBits.push(half); epsc -= half; }

  const posBits = 6 - paletteBits;
  let dataString = '';
  let currentCol = -1;
  let sameCol = 0;
  for (let i = 0; i < pixels.length; i += 4) {
    const colStr = getColStr(pixels, i, palette, COMPRESSION_FACTOR, FORMAT);
    if (currentCol === -1) { currentCol = colStr; }
    else if (currentCol !== colStr || i === pixels.length - 4) {
      if (i === pixels.length - 4) sameCol++;
      dataString += getString(sameCol, palette.indexOf(currentCol), palette.length, posBits, paletteBits, extraBits, FORMAT);
      sameCol = 0;
    }
    currentCol = colStr;
    sameCol++;
  }

  let finalData = "2" + FORMAT[canvas.width] + FORMAT[palette.length];
  for (let i = 0; i < palette.length; i++) finalData += palette[i];
  finalData += dataString;

  const compressed = compressV3(finalData);
  if (compressed.length < finalData.length) finalData = compressed;
  return finalData;
}

function downloadImage(url) {
  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      if (res.statusCode === 301 || res.statusCode === 302) {
        return downloadImage(res.headers.location).then(resolve).catch(reject);
      }
      if (res.statusCode !== 200) {
        reject(new Error(`HTTP ${res.statusCode} for ${url}`));
        return;
      }
      const chunks = [];
      res.on('data', (chunk) => chunks.push(chunk));
      res.on('end', () => resolve(Buffer.concat(chunks)));
      res.on('error', reject);
    }).on('error', reject);
  });
}

// Pokemon we need sprites for — name: pokedex number
const POKEMON_LIST = {
  // New heroes
  'Charmander': 4, 'Charmeleon': 5, 'Charizard': 6,
  'Bulbasaur': 1, 'Ivysaur': 2, 'Venusaur': 3,
  'Treecko': 252, 'Grovyle': 253, 'Sceptile': 254,
  'Chikorita': 152, 'Bayleef': 153, 'Meganium': 154,
  'Cyndaquil': 155, 'Quilava': 156, 'Typhlosion': 157,
  'Torchic': 255, 'Combusken': 256, 'Blaziken': 257,
  'Mudkip': 258, 'Marshtomp': 259, 'Swampert': 260,
  'Totodile': 158, 'Croconaw': 159, 'Feraligatr': 160,
  'Poliwag': 60, 'Poliwhirl': 61, 'Poliwrath': 62, 'Politoed': 186,
  'Wailmer': 320, 'Wailord': 321,
  'Machop': 66, 'Machoke': 67, 'Machamp': 68,
  'Riolu': 447, 'Lucario': 448,
  'Bagon': 371, 'Shelgon': 372, 'Salamence': 373,
  'Dratini': 147, 'Dragonair': 148, 'Dragonite': 149,
  'Beldum': 374, 'Metang': 375, 'Metagross': 376,
  'Pichu': 172, 'Pikachu': 25, 'Raichu': 26,
  'Weedle': 13, 'Kakuna': 14, 'Beedrill': 15,
  'Togepi': 175, 'Togetic': 176, 'Togekiss': 468,
  'Cleffa': 173, 'Clefairy': 35, 'Clefable': 36,
  'NidoranF': 29, 'Nidorina': 30, 'Nidoqueen': 31,
  'NidoranM': 32, 'Nidorino': 33, 'Nidoking': 34,
  // Monsters
  'Zubat': 41, 'Golbat': 42, 'Crobat': 169,
  'Tentacool': 72, 'Tentacruel': 73,
  'Carvanha': 318, 'Sharpedo': 319,
  'Chinchou': 170, 'Lanturn': 171,
  'Steelix': 208, 'Absol': 359,
  // Bosses
  'Geodude': 74, 'Graveler': 75, 'Golem': 76,
  'Onix': 95,
  'Regirock': 377, 'Regice': 378, 'Registeel': 379, 'Regigigas': 486,
  'Articuno': 144, 'Zapdos': 145, 'Moltres': 146,
  'Palkia': 484, 'Dialga': 483, 'Arceus': 493,
  'Serperior': 497, 'Emboar': 500, 'Samurott': 503,
  'Cobalion': 638, 'Terrakion': 639, 'Virizion': 640,
  'Tornadus': 641, 'Thundurus': 642, 'Landorus': 645,
  'Reshiram': 643, 'Zekrom': 644, 'Kyurem': 646,
  'Deoxys': 386,
  // Captures
  'Kangaskhan': 115, 'Heracross': 214, 'Greninja': 658,
  'Electivire': 466, 'Magmortar': 467, 'Rhyperior': 464,
  'Rayquaza': 384, 'Jirachi': 385, 'Mew': 151,
  'Latias': 380, 'Latios': 381,
  'Suicune': 245, 'Entei': 244, 'Raikou': 243,
  'Skarmory': 227, 'Lapras': 131, 'Arcanine': 59,
  // Capture upgrades
  'Butterfree': 12, 'Weavile': 461, 'Whiscash': 340, 'Manectric': 310,
};

async function main() {
  const compression = process.argv[2] || 'moderate';
  const outDir = path.join(__dirname, 'sprites');
  const resultsFile = path.join(__dirname, 'sprite_encodings.json');

  if (!fs.existsSync(outDir)) fs.mkdirSync(outDir, { recursive: true });

  const results = {};
  const errors = [];
  const names = Object.keys(POKEMON_LIST);

  console.error(`Downloading and encoding ${names.length} Pokemon sprites (compression: ${compression})...`);

  for (let idx = 0; idx < names.length; idx++) {
    const name = names[idx];
    const dexNum = POKEMON_LIST[name];
    const paddedDex = String(dexNum).padStart(4, '0');
    const url = `https://raw.githubusercontent.com/PMDCollab/SpriteCollab/master/portrait/${paddedDex}/Normal.png`;
    const pngPath = path.join(outDir, `${name}.png`);

    try {
      // Download if not cached
      let buffer;
      if (fs.existsSync(pngPath)) {
        buffer = fs.readFileSync(pngPath);
        process.stderr.write(`[${idx + 1}/${names.length}] ${name} (cached)... `);
      } else {
        buffer = await downloadImage(url);
        fs.writeFileSync(pngPath, buffer);
        process.stderr.write(`[${idx + 1}/${names.length}] ${name} (downloaded)... `);
      }

      // Encode
      const encoded = await encodeImageBuffer(buffer, compression);
      results[name] = encoded;
      console.error(`OK (${encoded.length} chars)`);
    } catch (err) {
      console.error(`FAILED: ${err.message}`);
      errors.push({ name, dexNum, error: err.message });
    }
  }

  // Write results
  fs.writeFileSync(resultsFile, JSON.stringify(results, null, 2));
  console.error(`\nDone! ${Object.keys(results).length} sprites encoded, ${errors.length} errors.`);
  console.error(`Results saved to: ${resultsFile}`);

  if (errors.length > 0) {
    console.error('\nFailed sprites:');
    errors.forEach(e => console.error(`  ${e.name} (#${e.dexNum}): ${e.error}`));
  }

  // Also output to stdout as JSON
  console.log(JSON.stringify(results, null, 2));
}

main().catch(err => { console.error(err); process.exit(1); });
