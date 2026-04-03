# Template Properties Reference

> Research from `working-mods/pansaer.txt` (total conversion mod) and existing Sliceymon usage.
> Created: 2026-04-03
> Purpose: Document defaults for 7 new templates before deploying them in Sliceymon+

---

## Executive Summary

**GOOD NEWS**: None of the 7 templates have built-in abilitydata (spells), triggerhpdata, or problematic default keywords. This was confirmed by the pansaer mod using each template both WITH and WITHOUT abilitydata across different hero variants -- if the template had built-in spells, they would appear in ALL usages.

**The replica system works like CSS**: template provides defaults, and any property you set explicitly overrides the default. Since Sliceymon always overrides `.hp.`, `.sd.`, `.col.`, `.img.`, `.speech.`, `.n.`, the template mainly affects the **die frame visual** and potentially any un-overridden properties.

### Risk Assessment

| Template | Risk Level | Notes |
|----------|-----------|-------|
| `replica.Fighter` | **LOW** | Already used in Sliceymon (line 53/99). Proven safe. |
| `replica.Eccentric` | **LOW** | No built-in spells/keywords. Default color yellow (may need `.col.` override). |
| `replica.Stalwart` | **LOW** | No built-in spells/keywords. Default color grey. |
| `replica.Dancer` | **LOW** | No built-in spells/keywords. Default color orange. |
| `replica.Fencer` | **LOW** | No built-in spells/keywords. Default color orange. |
| `replica.Alloy` | **LOW** | No built-in spells/keywords. Default color grey. |
| `replica.Guardian` | **LOW** | No built-in spells/keywords. Default color grey. Most heavily used in pansaer. |

---

## Template-by-Template Analysis

---

### 1. replica.Fighter

**Default Color**: Yellow
**Default HP**: 5 (from SLICEYMON_AUDIT)
**Default Tier**: Not 0 (Tier 0 entries explicitly set `.tier.0`)
**Built-in abilitydata**: NO (confirmed -- Hammer, Hound, Partner all lack abilitydata)
**Built-in doc**: NO
**Built-in facades**: NO
**Built-in triggerhpdata**: NO
**Built-in keywords**: NONE confirmed (all keywords in pansaer are explicitly added)

**Already proven in Sliceymon**: YES (line 53 MGreymon T3, line 99 MGreymon variant)
- Sliceymon overrides: `.col.`, `.tier.`, `.hp.`, `.sd.`, `.img.`
- Works without `.abilitydata.`, `.doc.`, `.facade.`, `.triggerhpdata.`

**Pansaer usages** (6 instances):

| Hero Name | HP Override | Col Override | Has abilitydata | Keywords Added |
|-----------|-----------|-------------|-----------------|----------------|
| Hammer | NOT SET (=5) | NOT SET (=yellow) | No | exert, mandatory |
| Hound | NOT SET (=5) | NOT SET (=yellow) | No | overdog, dog, underdog, singleuse |
| Greybeard | 4 | NOT SET | No | exert |
| Tender | 3 | facade | No | cantrip, sticky, unusable |
| Partner | NOT SET (=5) | NOT SET (=yellow) | No | armoured, zeroed |

**Notes**: Hammer and Hound don't override HP or color, confirming the defaults. Fighter is the safest template since it's already in our mod.

**Target Sliceymon heroes**: Machop (F P1), Totodile (J P1), Poliwag (J P2)

---

### 2. replica.Eccentric

**Default Color**: Yellow (inferred -- Noble in Yellow segment doesn't override `.col.`)
**Default HP**: Unknown (ALL pansaer usages override HP)
**Default Tier**: Standard (not 0)
**Built-in abilitydata**: NO (Noble has no abilitydata; others add it explicitly)
**Built-in doc**: NO
**Built-in facades**: NO
**Built-in triggerhpdata**: NO
**Built-in keywords**: NONE confirmed

**Pansaer usages** (5 instances):

| Hero Name | HP | Col | Has abilitydata | Keywords Added |
|-----------|-----|-----|-----------------|----------------|
| Noble | 8 | NOT SET (=yellow?) | No | heavy, eliminate, singleuse |
| Wailer | 8 | blue | Yes (Screech spell) | fizz, cantrip, pain, ranged, vulnerable |
| Mesmer | 8 | blue | Yes (Strain spell) | steel, hypnotise, singleuse, boost, inflictsingleuse |
| Magus | 8 | blue | Yes (Magic spell) | cantrip, heavy, eliminate |
| Anarchy | 8 | blue | Yes (Chaos spell) | lucky, fluctuate, weaken, vulnerable, cleave, engage |

**Abilitydata details** (added by pansaer, NOT inherited from template):
- Screech: `(Fey.sd.181-1:0-0:0-0:0-0:76-2.i.left.k.ranged#k.vulnerable#sidesc.[orange]Vulnerable[cu] [pips] [light]ranged[cu][nokeyword].img.284000HPEFFEzzt0j7g2y1g4w1g0N1w0g2M0w0g0M0M0w0g7.n.Screech)`
- Strain: `(Fey.sd.177-2:0-0:0-0:0-0:76-2.i.Ritemx.161bf.i.left.k.boost#k.inflictsingleuse#sidesc.[blue]Boost[cu] [pips]...`
- Magic: `(Fey.sd.0-0:0-0:0-0:0-0:76-1.i.left.hat.(Alpha.sd.5-2.i.left.Blindfold#k.heavy).i.Ritemx.161bf.img.284000j==jt=0f=8M6g6gw6gw6gw6hM8.n.Magic)`
- Chaos: `(Fey.sd.15-1:0-0:0-0:0-0:76-3.i.Ritemx.132fb.part.1.i.left.k.weaken#k.vulnerable#k.cleave#k.engage.img.289000n0rbjH=q4t89Ih0x4=SF0YVJ9504bo7k0gkdkcg04gktc14gcswc048dxk0o8g0og14C.n.Chaos)`

**Notes**: Eccentric seems to be a "quirky/chaotic" template. Noble (no abilitydata) proves the template itself has no spells. All spells are explicitly added by the pansaer mod.

**IMPORTANT**: Since default color is likely yellow, Sliceymon heroes using this template MUST set `.col.` explicitly if they need a different color (e.g., Snorunt is color B).

**Target Sliceymon hero**: Snorunt/Glalie/Froslass (B P1)

---

### 3. replica.Stalwart

**Default Color**: Grey (inferred -- Rose and Paragon in Grey segment don't override `.col.`)
**Default HP**: Unknown (ALL pansaer usages override HP)
**Default Tier**: Standard (not 0)
**Built-in abilitydata**: NO (Rose and Paragon have no abilitydata)
**Built-in doc**: NO (Rose has a doc, but Paragon doesn't -- doc is added, not inherited)
**Built-in facades**: NO
**Built-in triggerhpdata**: NO
**Built-in keywords**: NONE confirmed

**Pansaer usages** (3 instances):

| Hero Name | HP | Col | Has abilitydata | Keywords Added |
|-----------|-----|-----|-----------------|----------------|
| Rose | 12 | NOT SET (=grey) | No | (none -- keywords come from elsewhere) |
| Paragon | 11 | NOT SET (=grey) | No | lead |
| Florist | 11 | red | Yes (Wolfsbane spell) | damage, singleuse, groooooowth, cantrip, poison |

**Notes**: Stalwart = heavy armored fighter. Default color grey is convenient for Aron (color H). Wailmer (color M) will need `.col.` override.

**Target Sliceymon heroes**: Aron/Aggron (H P2), Wailmer/Wailord (M P2)

---

### 4. replica.Dancer

**Default Color**: Orange (inferred -- Firebug and Magitek in Orange segment don't override `.col.`)
**Default HP**: Unknown (ALL usages override HP)
**Default Tier**: Standard (not 0)
**Built-in abilitydata**: NO (Firebug, Magitek, Moray have no abilitydata)
**Built-in doc**: NO
**Built-in facades**: NO
**Built-in triggerhpdata**: NO
**Built-in keywords**: NONE confirmed

**Special syntax note**: In segment 26 (T3 Red), Dancer appears as `replica.(Dancer)` with parentheses around the name. This may be a pansaer convention for emphasis or may indicate a variant form. The parenthesized form appears to work identically.

**Pansaer usages** (6 instances):

| Hero Name | HP | Col | Has abilitydata | Keywords Added |
|-----------|-----|-----|-----------------|----------------|
| Firebug | 8 | NOT SET (=orange) | No | cantrip, rampage |
| Magitek | 7 | NOT SET (=orange) | No | charged, picky, engine, pain, onesie |
| Necro | 8 | red | Yes (spell not extracted) | boost, inflictdeath, possessed |
| Moray | 8 | red | No | poison, plague |
| Nymph | 8 | red | Yes (Cherish spell) | groupgrowth |
| Amazon | 9 | red | Yes (Drain spell) | selfheal, selfshield |

**Notes**: Dancer = graceful/agile class. Default color orange means Togepi (color R) MUST set `.col.` explicitly.

**Target Sliceymon hero**: Togepi/Togekiss (R P1)

---

### 5. replica.Fencer

**Default Color**: Orange (inferred -- Drunk in Orange segment doesn't override `.col.`)
**Default HP**: Unknown (ALL usages override HP)
**Default Tier**: Standard (not 0)
**Built-in abilitydata**: NO (Drunk has no abilitydata)
**Built-in doc**: NO
**Built-in facades**: NO
**Built-in triggerhpdata**: NO
**Built-in keywords**: NONE confirmed

**Pansaer usages** (5 instances):

| Hero Name | HP | Col | Has abilitydata | Keywords Added |
|-----------|-----|-----|-----------------|----------------|
| Drunk | 8 | NOT SET (=orange) | No | fumble |
| Nullmage | 9 | red | Yes (Erase spell) | selfheal, doubleuse, annul |
| Sawbones | 9 | red | Yes (Excise spell) | rescue, managain, cleanse, wither |
| Elder | 10 | red | Yes (Wisdom spell) | doublegrowth, cantrip, generous |
| Wilder | 7 | blue | Yes (Pure spell) | pain, manacost, heal, shield, regen, zeroed |

**Notes**: Fencer = parry/riposte style. Default color orange means Cleffa (color U) MUST set `.col.` explicitly.

**Target Sliceymon hero**: Cleffa/Clefable (U P2)

---

### 6. replica.Alloy

**Default Color**: Grey (inferred -- Jailbird, Freebird, Lamb in Grey segments don't override `.col.`)
**Default HP**: Unknown but notable -- Singer and Teacher do NOT override HP, meaning they use the template default. Since Singer and Teacher are in the Red segment (T1 Red) alongside other heroes with various HPs, the Alloy default HP is whatever the game assigns to the Alloy class.
**Default Tier**: Standard (not 0; Tier 0 entries explicitly set `.tier.0`)
**Built-in abilitydata**: NO (Jailbird, Freebird, Lamb have no abilitydata)
**Built-in doc**: NO
**Built-in facades**: NO
**Built-in triggerhpdata**: NO
**Built-in keywords**: NONE confirmed

**IMPORTANT**: Singer and Teacher do NOT set `.hp.`, meaning they use Alloy's default HP. This is the ONLY template where we see HP not overridden at non-T0 tiers. Sliceymon ALWAYS sets `.hp.` so this doesn't affect us, but it's notable.

**Pansaer usages** (5 instances):

| Hero Name | HP | Col | Tier | Has abilitydata | Keywords Added |
|-----------|-----|-----|------|-----------------|----------------|
| Jailbird | 4 | NOT SET (=grey) | 0 | No | cleave, exert, generous, singleuse |
| Freebird | 4 | NOT SET (=grey) | 0 | No | (none) |
| Lamb | 4 | NOT SET (=grey) | -- | No | death, generous |
| Singer | NOT SET (=default) | red | -- | Yes (Lullaby spell) | singleuse, charged, inflictexert |
| Teacher | NOT SET (=default) | red | -- | Yes (Nurture spell) | boost |

**Notes**: Alloy = steel/metal class. Default color grey works well for Beldum (color Z) but we'll still want to set `.col.` explicitly.

**Target Sliceymon hero**: Beldum/Metagross (Z P2)

---

### 7. replica.Guardian

**Default Color**: Grey (inferred -- all Grey-segment usages don't override `.col.`)
**Default HP**: Unknown (ALL usages override HP)
**Default Tier**: Standard (not 0)
**Built-in abilitydata**: NO (Matador, Piper, Lantern, Lapidary have no abilitydata)
**Built-in doc**: NO (Lapidary has a doc, but Matador/Piper/Lantern don't -- added not inherited)
**Built-in facades**: NO
**Built-in triggerhpdata**: NO (Font has triggerhpdata, but others don't -- added not inherited)
**Built-in keywords**: NONE confirmed

**Most heavily used template in pansaer** (14 instances across 3 color segments).

**Pansaer usages**:

| Hero Name | HP | Col | Has abilitydata | Has triggerhpdata | Keywords Added |
|-----------|-----|-----|-----------------|-------------------|----------------|
| Matador | 10 | NOT SET (=grey) | No | No | selfshield |
| Piper | 8 | NOT SET (=grey) | No | No | singleuse |
| Sexton | 8 | NOT SET (=grey) | Yes (Repose) | No | managain |
| Partisan | 8 | NOT SET (=grey) | Yes (Slash) | No | exert |
| Lantern | 8 | NOT SET (=grey) | No | No | vitality, cleanse |
| Lapidary | 10 | NOT SET (=grey) | No | No | (none) |
| Sedate | 6 | red | Yes (Anesthesia) | No | cleave, weaken |
| Priest | 6 | blue | Yes (Bless) | No | rite, serrated |
| Haruspex | 8 | blue | Yes (Omen) | No | flurry |
| Font | 7 | blue | Yes (Ache) | **YES** | selfheal, selfshield, inflictpain |
| Pyro | 6 | blue | Yes (Flame) | No | era |
| Cryo | 6 | blue | Yes (Frost) | No | era, weaken |

**Font's triggerhpdata**: `(Mage.hp.2.sd.76-1)` -- this is explicitly added by pansaer, not inherited from Guardian. Other Guardian usages (Matador, Piper, Lantern, etc.) have no triggerhpdata, confirming it's not a template default.

**Notes**: Guardian = party protector. Default color grey. Mudkip (color G) and Bulbasaur (color P) will both need `.col.` overrides.

**Target Sliceymon heroes**: Mudkip/Swampert (G P2), Bulbasaur/Venusaur (P P2)

---

## Critical Findings Summary

### 1. NO BUILT-IN ABILITYDATA ON ANY TEMPLATE

This is the most important finding. All 7 templates are **spell-free** by default. The abilitydata seen in pansaer is explicitly added by the mod for specific hero variants.

**Proof method**: For each template, at least one pansaer hero variant uses the template WITHOUT abilitydata. If the template had a built-in spell, it would appear even when the mod doesn't add one.

| Template | Variant WITHOUT abilitydata | Proves no built-in spell |
|----------|---------------------------|-------------------------|
| Eccentric | Noble | YES |
| Stalwart | Rose, Paragon | YES |
| Dancer | Firebug, Magitek, Moray | YES |
| Fencer | Drunk | YES |
| Alloy | Jailbird, Freebird, Lamb | YES |
| Fighter | Hammer, Hound, Partner | YES |
| Guardian | Matador, Piper, Lantern, Lapidary | YES |

### 2. NO BUILT-IN KEYWORDS

All keywords in pansaer usages are explicitly added with `.k.` or `#k.` syntax. No template has invisible default keywords that would leak into our heroes.

### 3. DEFAULT COLORS

| Template | Default Color | Sliceymon Heroes Need `.col.` Override? |
|----------|--------------|----------------------------------------|
| Fighter | Yellow | YES (Machop=F, Totodile=J, Poliwag=J) |
| Eccentric | Yellow | YES (Snorunt=B) |
| Stalwart | Grey | YES (Aron=H, Wailmer=M) |
| Dancer | Orange | YES (Togepi=R) |
| Fencer | Orange | YES (Cleffa=U) |
| Alloy | Grey | YES (Beldum=Z) |
| Guardian | Grey | YES (Mudkip=G, Bulbasaur=P) |

**ALL Sliceymon heroes need explicit `.col.` overrides** since none of our Pokemon use the default template colors.

### 4. NO BUILT-IN FACADES, DOC, OR TRIGGERHPDATA

All of these properties seen in pansaer are explicitly added per-hero, not inherited from templates. Safe to use these templates without worrying about inherited visual effects.

---

## Recommended Override Checklist

When creating a hero line using any of these 7 templates, ALWAYS set:

- [x] `.hp.` -- override the template default HP
- [x] `.sd.` -- override the template default dice faces
- [x] `.col.` -- override the template default color (CRITICAL -- all our heroes need different colors)
- [x] `.img.` -- override the template default sprite
- [x] `.speech.` -- override the template default battle cries
- [x] `.n.` -- override the template default display name

Optional (only if needed by the Pokemon design):
- [ ] `.abilitydata.` -- add spells (template has none by default)
- [ ] `.doc.` -- add description text
- [ ] `.facade.` -- add visual overlays
- [ ] `.triggerhpdata.` -- add HP-triggered effects
- [ ] `.tier.` -- override if needed (default is standard progression)
- [ ] `.k.` -- add keywords (template has none by default)

---

## Comparison with Already-Proven Templates

| Property | Lost (24 heroes) | Statue (12 heroes) | Fighter (2 heroes) | New Templates |
|----------|-----------------|--------------------|--------------------|---------------|
| Default HP | 3 | 20 | 5 | Varies |
| Default Color | Orange | Grey | Yellow | Varies |
| Built-in abilitydata | No | No | No | No |
| Built-in keywords | No | No | No | No |
| Risk level | None (proven) | None (proven) | None (proven) | Low |

The new templates behave identically to Lost/Statue/Fighter in terms of the override pattern. The only difference is the die frame visual, which is a cosmetic choice matching the Pokemon's character archetype.
