#!/usr/bin/env node
/**
 * Download Pokemon Showdown gen5 sprites (96x96 transparent PNGs)
 * and re-encode them at 24x24 for Slice & Dice.
 *
 * Usage: node tools/download_showdown_sprites.js [compression]
 * Output: tools/sprite_encodings.json (overwritten)
 */

const https = require('https');
const fs = require('fs');
const path = require('path');
const { createCanvas, loadImage } = require('canvas');

// --- Sprite encoder (same as rebuild_sprites.js) ---
const FORMAT = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ%=';
const TARGET_SIZE = 24;

function closeEnough(a, b, CF) {
  if (a === b) return true;
  if (a === '000' || b === '000') return false;
  let diff = 0;
  for (let i = 0; i < 3; i++) diff += Math.pow(Math.abs(FORMAT.indexOf(a[i]) - FORMAT.indexOf(b[i])), 2);
  return Math.sqrt(diff) <= CF * 4;
}

function getColStr(pixels, i, palette, CF) {
  const r = pixels[i] / 4, g = pixels[i+1] / 4, b = pixels[i+2] / 4, a = pixels[i+3];
  if (a < 12) return '000';
  const colStr = FORMAT[Math.floor(r)] + FORMAT[Math.floor(g)] + FORMAT[Math.floor(b)];
  for (let j = 0; j < palette.length; j++) { if (closeEnough(palette[j], colStr, CF)) return palette[j]; }
  return colStr;
}

function getString(sameCol, pi, ps, posBits, pb, extraVals) {
  const baseMax = Math.pow(2, posBits);
  const ev = pi >= extraVals.length ? 0 : extraVals[pi];
  const ebv = Math.pow(2, posBits);
  const actualMax = baseMax + ebv * ev;
  let result = "!";
  const advance = Math.min(sameCol, actualMax);
  if (advance <= baseMax) {
    result = FORMAT[(pi << posBits) + (advance - 1)];
    sameCol -= advance;
  } else {
    const aa = Math.min(advance, actualMax);
    sameCol -= aa;
    const ebvN = Math.floor(aa / ebv);
    const ebvnV = ebvN * ebv;
    let leftBits = pi << posBits;
    if (ebvnV > 0) {
      let eo = 0; for (let i = 0; i < pi; i++) eo += (i < extraVals.length ? extraVals[i] : 0);
      leftBits = ((ps - 1 + ebvN + eo) << posBits);
    }
    result = FORMAT[leftBits + (aa - ebvnV - 1)];
  }
  return sameCol > 0 ? result + getString(sameCol, pi, ps, posBits, pb, extraVals) : result;
}

function findUnusedChar(data) {
  for (let i = 0; i < FORMAT.length; i++) { if (!data.includes(FORMAT[i])) return FORMAT[i]; }
  return null;
}

function compressV3(data) {
  const ep = [];
  for (let rl = 2; rl <= 6; rl++) {
    for (let att = 0; att < 64; att++) {
      const map = new Map();
      for (let si = 0; si < data.length - rl; si++) {
        const part = data.substring(si, si + rl);
        map.set(part, (map.get(part) || 0) + 1);
      }
      if (map.size === 0) continue;
      const val = [...map.entries()].reduce((a, e) => e[1] > a[1] ? e : a);
      if (val[1] * (rl - 1) - 2 - rl <= 0) break;
      const uc = findUnusedChar(data);
      if (!uc) break;
      data = data.replaceAll(val[0], uc);
      ep.push(uc + val[0]);
    }
  }
  let result = "3" + FORMAT[ep.length];
  for (let i = 0; i < ep.length; i++) result += (ep[i].length - 1) + ep[i];
  return result + data;
}

async function encodeSprite(imageBuffer, compression) {
  const img = await loadImage(imageBuffer);
  const canvas = createCanvas(TARGET_SIZE, TARGET_SIZE);
  const ctx = canvas.getContext('2d');
  ctx.imageSmoothingEnabled = false;
  ctx.drawImage(img, 0, 0, TARGET_SIZE, TARGET_SIZE);
  const pixels = ctx.getImageData(0, 0, TARGET_SIZE, TARGET_SIZE).data;

  const palette = [], colCnt = {};
  for (let i = 0; i < pixels.length; i += 4) {
    const cs = getColStr(pixels, i, palette, compression);
    if (!palette.includes(cs)) { palette.push(cs); colCnt[cs] = 1; if (palette.length > 60) throw new Error('Palette too big'); }
    else colCnt[cs]++;
  }
  palette.sort((a, b) => { if (a === '000') return -1; if (b === '000') return 1; return colCnt[b] - colCnt[a]; });

  const paletteBits = Math.ceil(Math.log(palette.length) / Math.log(2));
  const eps = Math.pow(2, paletteBits) - palette.length;
  const extraBits = []; let epsc = eps;
  while (epsc > 0) { const h = Math.round(epsc / 2); extraBits.push(h); epsc -= h; }
  const posBits = 6 - paletteBits;

  let ds = '', cc = -1, sc = 0;
  for (let i = 0; i < pixels.length; i += 4) {
    const cs = getColStr(pixels, i, palette, compression);
    if (cc === -1) cc = cs;
    else if (cc !== cs || i === pixels.length - 4) {
      if (i === pixels.length - 4) sc++;
      ds += getString(sc, palette.indexOf(cc), palette.length, posBits, paletteBits, extraBits);
      sc = 0;
    }
    cc = cs; sc++;
  }

  let fd = "2" + FORMAT[TARGET_SIZE] + FORMAT[palette.length];
  for (let i = 0; i < palette.length; i++) fd += palette[i];
  fd += ds;
  const compressed = compressV3(fd);
  return compressed.length < fd.length ? compressed : fd;
}

// --- Download helper ---
function downloadBuffer(url) {
  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
        return downloadBuffer(res.headers.location).then(resolve).catch(reject);
      }
      if (res.statusCode !== 200) return reject(new Error(`HTTP ${res.statusCode} for ${url}`));
      const chunks = [];
      res.on('data', c => chunks.push(c));
      res.on('end', () => resolve(Buffer.concat(chunks)));
      res.on('error', reject);
    }).on('error', reject);
  });
}

// --- Pokemon list ---
const POKEMON = {
  'Charmander': 'charmander', 'Charmeleon': 'charmeleon', 'Charizard': 'charizard',
  'Bulbasaur': 'bulbasaur', 'Ivysaur': 'ivysaur', 'Venusaur': 'venusaur',
  'Treecko': 'treecko', 'Grovyle': 'grovyle', 'Sceptile': 'sceptile',
  'Chikorita': 'chikorita', 'Bayleef': 'bayleef', 'Meganium': 'meganium',
  'Torchic': 'torchic', 'Combusken': 'combusken', 'Blaziken': 'blaziken',
  'Mudkip': 'mudkip', 'Marshtomp': 'marshtomp', 'Swampert': 'swampert',
  'Cyndaquil': 'cyndaquil', 'Quilava': 'quilava', 'Typhlosion': 'typhlosion',
  'Totodile': 'totodile', 'Croconaw': 'croconaw', 'Feraligatr': 'feraligatr',
  'Pichu': 'pichu', 'Pikachu': 'pikachu', 'Raichu': 'raichu',
  'Machop': 'machop', 'Machoke': 'machoke', 'Machamp': 'machamp',
  'Riolu': 'riolu', 'Lucario': 'lucario',
  'Beldum': 'beldum', 'Metang': 'metang', 'Metagross': 'metagross',
  'Bagon': 'bagon', 'Shelgon': 'shelgon', 'Salamence': 'salamence',
  'Dratini': 'dratini', 'Dragonair': 'dragonair', 'Dragonite': 'dragonite',
  'Togepi': 'togepi', 'Togetic': 'togetic', 'Togekiss': 'togekiss',
  'Cleffa': 'cleffa', 'Clefairy': 'clefairy', 'Clefable': 'clefable',
  'Weedle': 'weedle', 'Kakuna': 'kakuna', 'Beedrill': 'beedrill',
  'Poliwag': 'poliwag', 'Poliwhirl': 'poliwhirl', 'Poliwrath': 'poliwrath', 'Politoed': 'politoed',
  'Wailmer': 'wailmer', 'Wailord': 'wailord',
  'NidoranF': 'nidoran-f', 'Nidorina': 'nidorina', 'Nidoqueen': 'nidoqueen',
  'NidoranM': 'nidoran-m', 'Nidorino': 'nidorino', 'Nidoking': 'nidoking',
  // Legendaries and extras
  'Raikou': 'raikou', 'Entei': 'entei', 'Suicune': 'suicune',
  'Articuno': 'articuno', 'Zapdos': 'zapdos', 'Moltres': 'moltres',
  'Regirock': 'regirock', 'Regice': 'regice', 'Registeel': 'registeel', 'Regigigas': 'regigigas',
  'Dialga': 'dialga', 'Palkia': 'palkia', 'Arceus': 'arceus',
  'Zekrom': 'zekrom', 'Kyurem': 'kyurem', 'Deoxys': 'deoxys',
  'Virizion': 'virizion', 'Absol': 'absol', 'Arcanine': 'arcanine',
  'Weavile': 'weavile', 'Manectric': 'manectric', 'Whiscash': 'whiscash',
  'Zubat': 'zubat', 'Serperior': 'serperior',
};

async function main() {
  const compression = { none: 0, mild: 1, some: 2, moderate: 4, extreme: 6, hyper: 8 }[process.argv[2]] || 4;
  const cacheDir = path.join(__dirname, 'sprites_showdown');
  if (!fs.existsSync(cacheDir)) fs.mkdirSync(cacheDir);

  const names = Object.entries(POKEMON);
  console.error(`Downloading ${names.length} sprites from Pokemon Showdown gen5...`);

  const results = {};
  let errors = [];

  for (let i = 0; i < names.length; i++) {
    const [key, urlName] = names[i];
    const cachePath = path.join(cacheDir, key + '.png');

    try {
      let buffer;
      if (fs.existsSync(cachePath)) {
        buffer = fs.readFileSync(cachePath);
        process.stderr.write(`[${i+1}/${names.length}] ${key} (cached)... `);
      } else {
        const url = `https://play.pokemonshowdown.com/sprites/gen5/${urlName}.png`;
        buffer = await downloadBuffer(url);
        fs.writeFileSync(cachePath, buffer);
        process.stderr.write(`[${i+1}/${names.length}] ${key} (downloaded)... `);
      }

      const encoded = await encodeSprite(buffer, compression);
      results[key] = encoded;
      process.stderr.write(`${encoded.length} chars\n`);
    } catch (e) {
      errors.push({ name: key, error: e.message });
      process.stderr.write(`FAILED: ${e.message}\n`);
    }
  }

  const outFile = path.join(__dirname, 'sprite_encodings.json');
  fs.writeFileSync(outFile, JSON.stringify(results, null, 2));
  console.error(`\nDone! ${Object.keys(results).length} sprites, ${errors.length} errors.`);
  if (errors.length > 0) {
    console.error('Failed:');
    errors.forEach(e => console.error(`  ${e.name}: ${e.error}`));
  }
}

main().catch(e => { console.error(e); process.exit(1); });
