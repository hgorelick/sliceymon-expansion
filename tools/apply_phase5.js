#!/usr/bin/env node
/**
 * apply_phase5.js — Phase 5: Tyranitar Redesign + Nidoqueen Poison Inheritance
 *
 * Modifies textmod_expanded.txt:
 *   Line 21: Redesign Larvitar->Tyranitar from Poison to Rock/Dark theme
 *   Line 31: Enhance Nidoran-F->Nidoqueen with inherited poison mechanics from old Tyranitar
 *
 * Usage: node apply_phase5.js
 */

const fs = require('fs');
const path = require('path');

const TEXTMOD_PATH = path.join(__dirname, '..', 'textmod_expanded.txt');

// ============================================================
// HELPERS
// ============================================================

function readLines() {
  const raw = fs.readFileSync(TEXTMOD_PATH, 'utf8');
  // Split preserving blank lines; file uses \n line endings
  return raw.split('\n');
}

function writeLines(lines) {
  fs.writeFileSync(TEXTMOD_PATH, lines.join('\n'), 'utf8');
}

// ============================================================
// SPRITE DATA (unchanged — reuse existing sprites)
// ============================================================
// Larvitar T1 sprite (from current line 21)
const LARVITAR_SPRITE = '342r8g2v483yrvT5Cw8J8d2mb000FTH9a9uMBzQEqHziokjxqZhh===KVKMy1y1y08kgvT08kg58Socg79QrW8OrUF58MocWE58Lod9vUr8K8chA8vUaJ8chA9U9CoAw978ACg9XvI8ldr6go5g8I8sndi5go83808selffc82k808hcnkfd82l86gfi4gk8c81kr5hdhUc8l80s8i9ch6p4gk8k80tr8eh5oxo4g8s9';
// Pupitar sprite (from current line 21)
const PUPITAR_SPRITE = '382m4k2n4c2rc42xch2zo42Dg42Fk42Gso2pc0008fhBJXovKuCQfpCdlt333===JU=EQME3f14gcgm4251o8o04O5khcm048m4Ac0zPnaD0o8DADDPkcgag594A8c84Qngbgkg8A9cs2U04dYm8Bgog2hdkzhbg8ck9z8o1i=84omf8r1x=gtF8n94o1mxbh8DGhk85go1mxa5c9Gca4gko3ng9ktw4o9cbcs434iFsEymbrs42mirsEwIwFb714ei5swE5kbcl414eiczEsFg8gc9sr04fdhc5ociGxr0fFskjicsk5cok1dlGkojhrl5cs41z0mnscickokck5o341ocmsfkmdk6Nokd4semxctPmxnksxckcR4lcgnG5q4R4ocnibrQ';
// Tyranitar sprite (from current line 21)
const TYRANITAR_SPRITE = '322ycX2D8k2se000PVJLRF9djBJtHPzX=TvBpVZJrvtpvlhjllppobdX0fXXXcpwdXXVc4p4sdXXUcgwpwEy2fd3cxqAy0c48gtEcIeg9wpEy0cg4kiditMsqdXcDikse4qwp4Aydsn8ldAgroEcVfl95DgfqwA3dEtk9=4sAdspc28j9=6AQd8oEc1ki9=7kukw8c1hsa=4zz5sc0tc49%xDCk84w8gc0kcsa%gdAiCcAc1cIka%tnmc0c2gkbZ8iugscVgkbYDhvscWgb8YDhuMyE4bYDjgy0Mb8Y9ki8Eykb8%9l5y4b8%b5y49ka=6o4cW9kMAk8=5wowcW9kIcI8=4wqcWDIfs8ZrocW';

// ============================================================
// TASK 1: Redesign Line 21 — Larvitar->Tyranitar (Poison -> Rock/Dark)
// ============================================================

function buildNewLine21() {
  // The line structure is:
  // hidden&temporary&ph.blarvitar;1;!mheropool.(<T1 block>).<T1 triggers>.<T1 speech>.<T1 doc>.<T1 name>
  //   +(<T2a block>).<T2a extras>.<T2a name>
  //   +(<T2b block>).<T2b extras>.<T2b name>
  //   +(<T3a block>).<T3a extras>.<T3a name>
  //   +(<T3b block>).<T3b extras>.<T3b name>
  //   .part.1&hidden.mn.Larvitar@2!m(skip&hidden&temporary),

  const prefix = 'hidden&temporary&ph.blarvitar;1;!mheropool.';
  const suffix = '.part.1&hidden.mn.Larvitar@2!m(skip&hidden&temporary),';

  // --- T1 Larvitar: HP 8 ---
  // REMOVED: k.poison, k.acidic, all poison facades, i.self.Bottom Poison^1/1, doc, self-damage cantrip face
  // NEW: Clean Rock/Dark. sd: 39-1:39-1:63-1:63-1:0:0 (Heavy 1, Heavy 1, Steel Shield 1, Steel Shield 1, Blank, Blank)
  // No keywords, no facades at T1
  const t1 = `(replica.statue.tier.1.col.h.hp.8` +
    `.sd.39-1:39-1:63-1:63-1:0:0` +
    `.img.${LARVITAR_SPRITE})` +
    `.speech.Lar!:Tar!:baba!` +
    `.n.Larvitar`;

  // --- T2a Pupitar: HP 12 ---
  // REMOVED: old sd (45-1:45-1:56-4:56-4), antivenom, immune-to-poison doc
  // NEW: sd 39-2:39-2:63-3:63-3:30-1:0, k.minusflesh, facade.the32:0, doc "Reduce damage taken by 1"
  const t2a = `+(replica.Statue.col.h.hp.12` +
    `.k.minusflesh#facade.the32:0` +
    `.sd.39-2:39-2:63-3:63-3:30-1:0` +
    `.img.${PUPITAR_SPRITE})` +
    `.doc.[plus] Reduce damage taken by 1` +
    `.speech.[dot][dot][dot]` +
    `.n.Pupitar`;

  // --- T2b Pupitar: HP 12 ---
  // NEW: sd 39-3:39-3:63-2:63-2:0:0, k.minusflesh, facade.the32:0
  const t2b = `+(replica.Statue.col.h.hp.12` +
    `.k.minusflesh#facade.the32:0` +
    `.sd.39-3:39-3:63-2:63-2:0:0` +
    `.img.${PUPITAR_SPRITE})` +
    `.doc.[plus] Reduce damage taken by 1` +
    `.speech.[dot][dot][dot]` +
    `.n.Pupitar`;

  // --- T3a Tyranitar: HP 13 ---
  // REMOVED: k.regen, k.poison, k.acidic, all poison facades, i.t.Carrier, self-damage cantrip, poison doc
  // NEW: sd 39-4:39-3:63-3:63-3:30-2:30-1, k.minusflesh, facade.the32:0, i.t.gnoll (keep), Sandstorm doc
  const t3a = `+(replica.Statue.col.h.tier.3.hp.13` +
    `.k.minusflesh#facade.the32:0` +
    `.sd.39-4:39-3:63-3:63-3:30-2:30-1` +
    `.img.${TYRANITAR_SPRITE})` +
    `.doc.[plus] Reduce damage taken by 1[n][plus] At the end of each turn[comma] 1 damage to all heroes and monsters` +
    `.speech.Graa!:Tar!` +
    `.i.t.gnoll` +
    `.n.Tyranitr`;

  // --- T3b Tyranitar: HP 9 ---
  // KEEP: duel/ego keywords and facades (they're Rock/Dark appropriate)
  // NEW: sd 30-4:30-4:39-4:39-4:63-3:0 (Cruel 4, Cruel 4, Heavy 4, Heavy 4, Steel Shield 3, Blank)
  // No sandstorm doc on this variant
  const t3b = `+(replica.Statue.col.h.tier.3.hp.9` +
    `.i.t.gnoll` +
    `.i.left2.k.duel#facade.dan6:55:0:10` +
    `.i.topbot.k.ego#facade.Che19:0` +
    `.sd.30-4:30-4:39-4:39-4:63-3:0` +
    `.img.${TYRANITAR_SPRITE})` +
    `.speech.GRA!:TAR!:[i][sin]rumble` +
    `.n.Tyranitr`;

  return prefix + t1 + t2a + t2b + t3a + t3b + suffix;
}

// ============================================================
// TASK 2: Enhance Line 31 — Nidoran-F->Nidoqueen (Poison Inheritance)
// ============================================================

// Nidoqueen sprites (from current line 31)
const NIDORANF_SPRITE = '3c2s8w2yw82z082A9w2Cwa2Io02J1o2K802Lni2O1a2Q0y2Sws2E7vNTdfjNTHRTZvxB9lBDd3=Za69O=59z7CQ=39odw4aQ=19Is0pzO19Q=zp4A1cwJy%KIa0s3eJzZ8IbS3fw0ozXyowcy3Ff0ozWsocEy3HepsV8IbFy3Gs1x90osUyQx8FS2Fs4SIfa1AzFsKg0F869Is1q1wA91yEs8g0gF8693voC28Es8iF85S2vp0aw18E8ziF8593vIs09Ozia4y4vz0os909z0i0A3s5t0qzO09j1wAQUr0w1sp909ks1bw4Cwr0w0w0p0al0AUyo8Msvpang0bw48p81sv0ank8x38o8MpMKu8oMegwf828o8NoMsusMA0m0Cx2aPsuwMAnjs5wd0u0scwns9Vvq0An0CwYv18mwf9Zr28LA=7snhC=78Lbw5SXyLsa7wA382s8Ly0xWCw1KwznjKp4w7xewLso5AJ6yp8zL8IA1CJw68p8osnhOw1bJy5wAoznh0bxcJz5y2o8nhcw0c1p85AJKng';
const NIDORINA_SPRITE = '3f2k8g2lg82mgw2ywg2A802B4g2Ckw2H0g2I8h2J0w2K8w2Qlq2Vg72W813XtgP2E6LTRzFDffhvPRnrtxd59vvtl5k0Q8iw8u8vok7AQl1I8rk8vlNlrl3h8pmguyOQk183hClXQgJj82hAgXQ0C9wh2kHXhwp0mcwm1w0kslOgkpHewh3gskOwHpkUy2gtlOlpmbwiw8y18nh86kqmbgah8y1Vwm5lo0omaycmlHMk83lo0o0magemgHNk2k4m9yfhkOm0k5y9yUmlNky6C9gf1lPI6mm9gdT0Ak6mhwkbTB7hwmI9T3kNjxl4wRy584Kiw3iWwMh5y6l3K0hWw7h7hBBJFlJ68EgNh2g3kHGgMwEg583yJl2kHF1VC8B5hk4whF0l5WkB6Al68xlNw4K7kT5w4hMgT4k5l3K1k29Sg5I2y4Aj86WAhwB9287wJIAI1wJmC3wRkW8ixm0C0Kl495g4869wiwB3wk4k5wAwjw9wl4m2wlB7gMK6g385Vl7m6gOg18jAVwl5kMk0y3mVC6h';
const NIDOQUEEN_SPRITE = '3d2mg82u082G8g2H382I9g2Ko82LoE2O182PE82Qga2R092S1m2VHu2E7vNTbfjdvFBldXZJNzn===ug9==H2I=U9683bg=3a696a=c691i3m4G68hb581egO28mg4GRpG3mRsaOOO3mRKp83Gusc0mmOHh8p8p8HRr8r90IS28Rq8PVr8tGI290I9q8PVr8t9m4eq8PVpPvaUQp8KVoF8vaXaoGVoF8vIY9K4G9FKtI%mg4muLKs9=Qg480mLKqG%agO1wb0g9r9=580w2Qhc4QSUax3Qg1g58wMw90m6b0wg4Q78Nwau5mmKwi1gI7mN8oM8h848h8M8wl8h7mwM9EKu7aw9i9hUfm7g9xIaiZhX8ybj=X8zak4h=HAak2bi%myejRLf9hSumxI3bi8oJrdmh9x87Qg9LDyEodyWeLDxEbAWmEoIbtbguAX8owEKhfL9SzX8oyE90l8owP29yWmEzEG0k8Ex928yh6h9EAEG3myoG19xnboCPHoyEaguxgf9LDGSAewch28LDKOozPhc';

function buildNewLine31() {
  const prefix = 'hidden&temporary&ph.bnidoranf;1;!mheropool.';
  const suffix = '.part.1&hidden.mn.NidoranF@2!m(skip&hidden&temporary),';

  // --- T1 Nidoran-F: HP 6 ---
  // Inherits Tyranitar's T1 poison pattern exactly:
  //   k.poison, k.acidic, i.rightmost, facades (bas12, Eme53, pos121)
  //   i.self.Bottom Poison^1/1, doc: Start Poisoned 1
  //   sd: 39-1:39-1:56-1:56-1:0:12-0 (same as old Larvitar T1)
  const t1 = `(replica.Statue.tier.1.hp.6.col.n` +
    `.i.rightmost.k.poison.i.k.acidic` +
    `.i.rightmost.facade.bas12:60:-40:-20` +
    `.i.topbot.facade.Eme53:0` +
    `.i.left2.facade.pos121:48:-30:-10` +
    `.sd.39-1:39-1:56-1:56-1:0:12-0` +
    `.img.${NIDORANF_SPRITE})` +
    `.i.self.Bottom Poison^1/1` +
    `.doc.[plus] Start [green]Poisoned[cu] 1` +
    `.speech.Nido!:[i]squeak` +
    `.n.NidoranF`;

  // --- T2a Nidorina: HP 8 ---
  // Inherits Pupitar's antivenom + immune doc
  // Keeps Nidorina's own dice and facades (minusera, Che20:0)
  const t2a = `+(replica.Statue.hp.8.col.n.tier.2` +
    `.i.antivenom` +
    `.k.minusera#facade.Che20:0` +
    `.sd.39-2:39-2:119-2:119-2:56-2:0` +
    `.img.${NIDORINA_SPRITE})` +
    `.doc.[plus] Immune to [green]Poison` +
    `.speech.Rina!:[i]growl` +
    `.n.Nidorina`;

  // --- T2b Nidorina: HP 7 ---
  // Also inherits antivenom + immune doc
  const t2b = `+(replica.Statue.hp.7.col.n.tier.2` +
    `.i.antivenom` +
    `.sd.39-2:39-2:53-1:53-1:119-1:0` +
    `.img.${NIDORINA_SPRITE})` +
    `.doc.[plus] Immune to [green]Poison` +
    `.speech.Rina!:[i]growl` +
    `.n.Nidorina`;

  // --- T3a Nidoqueen: HP 11 ---
  // Inherits Tyranitar T3a poison pattern:
  //   k.regen, k.poison, k.acidic (from old Tyranitar T3a)
  //   i.t.Carrier (from old Tyranitar T3a)
  //   Poison facades: Eme53:90:20:0, bas12:60:-40:-20, pos121:48:-30:-10
  //   Doc: Start Poisoned two
  // Keeps Nidoqueen's own dice, HP, sprite
  // Earth Power spell (abilitydata) stays
  const t3a = `+(replica.Statue.tier.3.hp.11.col.n` +
    `.k.regen.k.poison.k.acidic` +
    `.i.t.Carrier` +
    `.facade.Eme53:90:20:0` +
    `.facade.bas12:60:-40:-20` +
    `.facade.pos121:48:-30:-10` +
    `.sd.39-3:39-2:119-3:119-3:56-2:12-2` +
    `.img.${NIDOQUEEN_SPRITE}` +
    `.abilitydata.(Fey.sd.39-4:0-0:0-0:0-0:76-4.img.spark.n.Earth Power))` +
    `.doc.[plus] Start [green]Poisoned[cu] two` +
    `.speech.Queen!:[i]EARTH POWER` +
    `.n.Nidoqueen`;

  // --- T3b Nidoqueen: HP 8 ---
  // Inherits Tyranitar T3b duel/ego pattern:
  //   k.duel, k.ego, facades dan6:55:0:10 and Che19:0
  // Keeps Nidoqueen's own dice, HP, sprite
  const t3b = `+(replica.Statue.tier.3.hp.8.col.n` +
    `.k.duel.k.ego` +
    `.facade.dan6:55:0:10` +
    `.facade.Che19:0` +
    `.sd.39-4:39-4:119-4:119-4:53-2:0` +
    `.img.${NIDOQUEEN_SPRITE})` +
    `.speech.Queen!:[i]EARTH POWER` +
    `.n.Nidoqueen`;

  return prefix + t1 + t2a + t2b + t3a + t3b + suffix;
}

// ============================================================
// MAIN
// ============================================================

function main() {
  console.log('Phase 5: Tyranitar Redesign + Nidoqueen Poison Inheritance');
  console.log('===========================================================\n');

  const lines = readLines();
  console.log(`Read ${lines.length} lines from textmod_expanded.txt\n`);

  // Validate current lines
  const line21 = lines[20]; // 0-indexed
  const line31 = lines[30]; // 0-indexed

  if (!line21 || !line21.includes('ph.blarvitar')) {
    console.error('ERROR: Line 21 does not contain expected Larvitar hero. Found:', line21 ? line21.substring(0, 80) : '(empty)');
    process.exit(1);
  }
  if (!line31 || !line31.includes('ph.bnidoranf')) {
    console.error('ERROR: Line 31 does not contain expected Nidoran-F hero. Found:', line31 ? line31.substring(0, 80) : '(empty)');
    process.exit(1);
  }

  // Store originals for diff summary
  const origLine21 = line21;
  const origLine31 = line31;

  // Build new lines
  const newLine21 = buildNewLine21();
  const newLine31 = buildNewLine31();

  // Apply changes
  lines[20] = newLine21;
  lines[30] = newLine31;

  // Write back
  writeLines(lines);
  console.log('Changes written to textmod_expanded.txt\n');

  // ============================================================
  // SUMMARY
  // ============================================================

  console.log('=== LINE 21: Tyranitar Redesign (Poison -> Rock/Dark) ===\n');

  // T1 changes
  console.log('T1 Larvitar (HP 8):');
  console.log('  REMOVED: k.poison, k.acidic, i.rightmost keywords');
  console.log('  REMOVED: facade.bas12:60:-40:-20, facade.Eme53:0, facade.pos121:48:-30:-10');
  console.log('  REMOVED: i.self.Bottom Poison^1/1 trigger');
  console.log('  REMOVED: doc "[plus] Start [green]Poisoned[cu] 1"');
  console.log('  REMOVED: Self-damage Cantrip face (12-0)');
  console.log('  NEW sd: 39-1:39-1:63-1:63-1:0:0 (Heavy 1, Heavy 1, Steel Shield 1, Steel Shield 1, Blank, Blank)');
  console.log('  OLD sd: 39-1:39-1:56-1:56-1:0:12-0');
  console.log('');

  // T2 changes
  console.log('T2a Pupitar (HP 12):');
  console.log('  REMOVED: i.antivenom, doc "[plus] Immune to [green]Poison"');
  console.log('  REMOVED: old sd 45-1:45-1:56-4:56-4 (Damage Era + Shields)');
  console.log('  REMOVED: k.minusera keyword (replaced)');
  console.log('  NEW: k.minusflesh (sand armor), facade.the32:0 (Sandstorm)');
  console.log('  NEW sd: 39-2:39-2:63-3:63-3:30-1:0 (Heavy 2, Heavy 2, Steel Shield 3, Steel Shield 3, Cruel 1, Blank)');
  console.log('  NEW doc: "[plus] Reduce damage taken by 1"');
  console.log('');

  console.log('T2b Pupitar (HP 12) [NEW TIER]:');
  console.log('  Previously: only one T2 Pupitar existed (copy of T2a)');
  console.log('  NEW: Offensive variant with k.minusflesh, facade.the32:0');
  console.log('  NEW sd: 39-3:39-3:63-2:63-2:0:0 (Heavy 3, Heavy 3, Steel Shield 2, Steel Shield 2, Blank, Blank)');
  console.log('');

  // T3 changes
  console.log('T3a Tyranitar (HP 13):');
  console.log('  REMOVED: k.regen, k.poison, k.acidic keywords');
  console.log('  REMOVED: facade.Eme53:90:20:0, facade.bas12:60:-40:-20, facade.pos121:48:-30:-10');
  console.log('  REMOVED: i.t.Carrier item');
  console.log('  REMOVED: Self-damage Cantrip face (12-2)');
  console.log('  REMOVED: doc "[plus] Start [green]Poisoned[cu] two"');
  console.log('  NEW: k.minusflesh, facade.the32:0 (Sandstorm)');
  console.log('  NEW: i.t.gnoll (kept from T3b, generic item)');
  console.log('  NEW sd: 39-4:39-3:63-3:63-3:30-2:30-1 (Heavy 4, Heavy 3, Steel Shield 3, Steel Shield 3, Cruel 2, Cruel 1)');
  console.log('  NEW doc: "[plus] Reduce damage taken by 1[n][plus] At the end of each turn[comma] 1 damage to all heroes and monsters"');
  console.log('');

  console.log('T3b Tyranitar (HP 9):');
  console.log('  KEPT: k.duel, k.ego, facade.dan6:55:0:10, facade.Che19:0');
  console.log('  REMOVED: old sd 39-5:39-5:56-5:56-5:43:0');
  console.log('  NEW sd: 30-4:30-4:39-4:39-4:63-3:0 (Cruel 4, Cruel 4, Heavy 4, Heavy 4, Steel Shield 3, Blank)');
  console.log('  No Sandstorm on this variant — pure duel/ego aggression');
  console.log('');

  console.log('\n=== LINE 31: Nidoqueen Poison Inheritance ===\n');

  console.log('T1 Nidoran-F (HP 6):');
  console.log('  ADDED: i.rightmost.k.poison.i.k.acidic (from Tyranitar T1)');
  console.log('  ADDED: facade.bas12:60:-40:-20, facade.Eme53:0, facade.pos121:48:-30:-10 (from Tyranitar T1)');
  console.log('  ADDED: i.self.Bottom Poison^1/1 trigger (from Tyranitar T1)');
  console.log('  KEPT: doc "[plus] Start [green]Poisoned[cu] 1" (already present)');
  console.log('  KEPT: sd 39-1:39-1:56-1:56-1:0:12-0, HP 6, sprite');
  console.log('');

  console.log('T2a Nidorina (HP 8):');
  console.log('  KEPT: i.antivenom (already present)');
  console.log('  KEPT: k.minusera#facade.Che20:0 (already present)');
  console.log('  KEPT: doc "[plus] Immune to [green]Poison" (already present)');
  console.log('  KEPT: sd 39-2:39-2:119-2:119-2:56-2:0');
  console.log('');

  console.log('T2b Nidorina (HP 7):');
  console.log('  KEPT: i.antivenom (already present)');
  console.log('  KEPT: doc "[plus] Immune to [green]Poison" (already present)');
  console.log('  KEPT: sd 39-2:39-2:53-1:53-1:119-1:0');
  console.log('');

  console.log('T3a Nidoqueen (HP 11):');
  console.log('  ADDED: k.regen, k.poison, k.acidic keywords (from Tyranitar T3a)');
  console.log('  ADDED: i.t.Carrier item (from Tyranitar T3a)');
  console.log('  ADDED: facade.Eme53:90:20:0, facade.bas12:60:-40:-20, facade.pos121:48:-30:-10 (from Tyranitar T3a)');
  console.log('  KEPT: doc "[plus] Start [green]Poisoned[cu] two" (already present)');
  console.log('  KEPT: sd 39-3:39-2:119-3:119-3:56-2:12-2, HP 11, sprite');
  console.log('  KEPT: Earth Power spell (abilitydata)');
  console.log('');

  console.log('T3b Nidoqueen (HP 8):');
  console.log('  ADDED: k.duel, k.ego keywords (from Tyranitar T3b)');
  console.log('  ADDED: facade.dan6:55:0:10, facade.Che19:0 (from Tyranitar T3b)');
  console.log('  KEPT: sd 39-4:39-4:119-4:119-4:53-2:0, HP 8, sprite');
  console.log('');

  // Verify line count unchanged
  const verifyLines = readLines();
  console.log(`\nVerification: File now has ${verifyLines.length} lines (was ${lines.length}).`);

  if (verifyLines.length !== lines.length) {
    console.error('WARNING: Line count changed!');
  } else {
    console.log('Line count preserved. Phase 5 complete.');
  }
}

main();
