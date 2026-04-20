# Pipeline Fidelity Plan

> **Target:** `extract(build(extract(mod))) == extract(mod)` as **semantic IR equality** (SPEC §3.1) for all 4 known-working mods (sliceymon, pansaer, punpuns, community). Byte-level comparison (`drift_audit`) is a supporting canary, not a co-equal gate — it catches byte regressions that also happen to be semantic, but the contract is IR equality. Covers the full extract→build pipeline: heroes, bosses, monsters, items, selectors, phases, entity-refs, and structural modifiers.

## Context & trigger

After the fight-unit fixes landed (chain-segment interleaving, head-paren Case B with single-paren nested, multi-variant last-fight name extraction), all 3 `ch.om` bosses in sliceymon paste byte-identically and the 2 previously-failing bosses (Quagsire, Floor8) are resolved.

On in-game paste-test, sliceymon now paste-succeeds but exhibits **gameplay-affecting drift** post-paste:
- **BUG curse** is applied to the run (symptom of corrupted content)
- **Axew → "AA", Larvitar → "HS"** during team selection (heroes loading with wrong names)

Audit across all 4 mods confirms widespread hero-pipeline drift (not limited to sliceymon). Each mod shows data-loss drifts beyond mere property reorder.

## Correctness bar

Per SPEC §3.1, the contract is **semantic IR equality**: `extract(build(extract(mod))) == extract(mod)`. Per SPEC §2, Thunder's guide is the authoritative format spec — when parser, emitter, and guide disagree, the guide wins.

- **Must preserve** any shape the guide describes as semantically meaningful → represent as a typed IR field with a semantic (not presentation) name.
- **May normalize** shapes the guide shows in multiple equivalent forms → IR collapses to one canonical shape; emitter picks it deterministically.
- **Must not** encode source text presentation as IR state (no `has_explicit_n`, `hidden_placement: BeforeClose|AfterClose`, `outer_wrap_depth`, etc.) unless a guide lookup confirms the distinction is semantic. "Preserve verbatim" is not a substitute for understanding what the shape means.

Specific guide evidence already gathered:

| Shape | Guide says | Our stance |
|---|---|---|
| `memory` ↔ `Memory` | Lines 1199, 1233: explicitly interchangeable | Normalize acceptable |
| No general case-sensitivity note | Nothing found | Normalize acceptable (but confirm for each token before assuming) |
| `.i.left.k.pain` vs `.i.col.k.pain` | Different slot targets in examples | **Must preserve** — different game effect |
| `#` as sub-entry separator within `.i.` | Guide lines 737, 875, 941 show `#` chaining multi-entry groups | **Must preserve** all sub-entries |
| `.speech.` with empty value | Not documented; `.speech.` only ever shown with content | **Must not emit** when value is empty |
| `abilitydata.(X)` vs `abilitydata.((X))` | Guide line 1012 `(Lazy.abilitydata.(Statue...n.Snooze)).n.Lazy` shows `))` is natural nested-paren closure: inner `)` closes the abilitydata value, outer `)` closes the containing replica/template block. There is no distinct `abilitydata.((X))` form. | Structure is fixed by balanced-paren rules. Rebuild must close the outer block; dropping it is data loss, not a format choice. |
| `.sd.` bare-id vs `X-0` vs `X-Y` pip | Guide examples mix all three: line 1010 `sd.15-2:0-0:177-1`, line 1012 `sd.56-2:0:5:5`, line 1016 `sd.181-1:105-1:1-0:0-0:0-0:0-0` | Bare id ≡ `X-0`. Parser must accept all three; emitter picks a canonical form. Normalize acceptable. |
| Property order inside replica blocks (e.g. `col.hp.sd` vs `hp.col.sd`) | Guide line 857 `col.r.hp.7.sd.` vs line 1041 `hp.24.col.w.sd.` — guide uses both orderings freely. No order invariant stated. | Normalize acceptable — class A drift is cosmetic. |

## Audit findings — drift classes

Drift classes are enumerated A–R. Each class appears only once in the legend below; per-mod tables cite the class letter and include only mod-specific evidence.

### Class legend (A–R)

| Class | Description | Severity | Fix reference |
|---|---|---|---|
| A | Property reorder inside a `(replica.Template...)` block (e.g. `hp.tier.col` ↔ `col.tier.hp`) | Cosmetic — guide lines 857/1041 show both orderings | Normalize; no fix needed |
| B | Chain sub-entry lost when `.col.`/`.tier.`/`.hp.`/`.t.` appears after `.i.`/`.k.` and is misread as a property boundary | Structural — data loss | Phase 1.1 |
| C | `memory` ↔ `Memory` case | Cosmetic — guide lines 1199/1233 call them interchangeable | Normalize |
| D | `Replica.`/`Heropool.` → `replica.`/`heropool.` case | Cosmetic — guide uses lowercase consistently; no case-sensitivity invariant found | Normalize |
| E | `.sd.` faces: bare id `X`, `X-0`, `X-Y` all mix in source; rebuild normalizes | Cosmetic — guide lines 1010/1012/1016 mix all three forms | Parser must accept all three |
| F | Sub-entries after the first `#` dropped when the chain scanner terminates early | Structural — data loss | Phase 1.2 |
| G | `.speech..n.X` emitted with empty speech value | Structural — format error; guide never shows empty `.speech.` | Phase 3.1 |
| H | Mixed bare-id + id-pip face list in one `.sd.` causes the whole `.sd.` to be dropped | Structural — data loss | Phase 1.3 |
| I | `abilitydata.(X))` `))` is inner-close + outer-block-close; rebuild drops the outer block's `)`, orphaning subsequent properties | Structural — data loss | Phase 1.7 |
| J | Bare / head-paren hero shape `(Template.n.Name).rest` or bare `Template.rest` converted to `(replica.Template...)` with data corruption | Structural — data corruption | Phase 1.4 |
| K | `heropool` with `+`-separated mixed list (bare vanilla names, `(replica...)` blocks, and multi-tier `+`-joined bodies, some with embedded newlines) collapsed to a single replica block | Structural — data corruption | Phase 1.5 |
| L | `itempool` `(ritemx.ID.part.N)#(rmod.ID.part.N)#...` entity-ref groups replaced with fabricated replica content | Structural — data corruption | Phase 1.6 |
| M | Leading whitespace / newlines between `+` tier separators | Cosmetic | Normalize |
| O | Boss `.mn.NAME&hidden)` vs `.mn.NAME)&hidden` suffix placement | **Unresolved** — presentation vs semantic is not yet confirmed against the guide | Phase 1.8 (guide lookup first) |
| P | Hero body content replaced by inheritance-target fabrication (parser substitutes a template's body for the source's authored body) | Structural — data corruption | Phase 1.10 |
| Q | `v`-prefixed boss-ref list in selectors (`Number#1;vX@3vY@3...`) replaced with a different set of boss refs | Structural — data corruption | Phase 1.11 |
| R | Inner-block `.n.DisplayName` (`.n.Punch`, `.n.Flux`, `.n.Wilding`, etc. *inside* a `(replica.Template...)` block) dropped | Structural — data loss | Phase 1.9 |
| S | Selector inline modifier-kind rewrite: `!m(add.(dabble...))` → `!m(party.(dabble...))` (sliceymon Warning selector; the `add`/`party`/`skip` inline verb inside a selector's `!m(...)` clause is being replaced) | Structural — data corruption | Phase 1.12 |
| T | Fight-unit auto-injection of `.n.TemplateName` into bare tier entries: source `fight.alpha+wolf` → rebuild `fight.alpha.n.alpha+wolf.n.wolf`; source `fight.Slimelet+Rat.n.Venus` → rebuild `fight.Slimelet.n.Slimelet+Rat.n.Venus`. Parser normalizes every fight-unit entry to carry an explicit `.n.`, synthesizing one when absent | **Unresolved** — semantic status unclear: bare vs `.n.TemplateName` may be equivalent, may override a template inner name, may trigger a template lookup. Guide lookup required before IR shape | Phase 1.13 (guide lookup first) |
| U | Boss `.ph.` prefix + outer wrapper rewrite: source `((0.ph.bAlpha;1;!m(...)&Hidden)@2!mskip&Hidden)&Hidden).mn.Alpha` → rebuild `1.ph.bA;1;!m(...&hidden).mn.Alpha@2!m(skip&hidden&temporary)` — (a) `.ph.bAlpha` truncated to `.ph.bA` (**unambiguous data loss**), (b) leading phase index `0` changed to `1` (**unambiguous data corruption**), (c) doubly-wrapped `(( ... )&Hidden)&Hidden)` collapsed (**presentation vs semantic — guide lookup required**), (d) `@N!m` suffix re-parenthesized (**presentation vs semantic — guide lookup required**) | Mixed: (a)(b) data corruption; (c)(d) unresolved | Phase 1.14 (split: name/index fix, wrapper/suffix guide lookup first) |
| V | Monsterpool template form: source `monsterpool.(rmon.ded.bal.boar).hp.4.n.Soldier.sd....` and `monsterpool.(Saber.bal.rat).hp.2.sd....` → rebuild `monsterpool.(replica.rmon.n.Forest Soldier)` / `(replica.Saber.n.Forest Worker Ant)` with body content stripped or reordered. Bare-template / entity-ref monsterpool entries forced into replica form, inheriting the outer `.mn.` as the inner `.n.` | Structural — data corruption (monster-scope variant of classes J, K, P) | Phase 1.15 |
| W | Suffix-group loss around `@N` transitions in boss fight-unit chains: source `...facade.bas242:20&Hidden@4m4.fight.Dragon...` → rebuild `...facade.Eme66:0.i@4m4.fight.Dragon...` — the `&Hidden` suffix before the `@4` transition is dropped (pansaer "Boss Fight 01") | Structural — data loss | Phase 1.16 |
| X | Multi-line phase content dropped: source `ch.om4.ph.tivoid.n.Diary Page.doc.\nYour Group Notice...\n.img.pape` → rebuild omits the modifier entirely (6/6 community phase modifiers unmatched) | Structural — data loss | Phase 1.17 |
| Y | `add.` modifier with entity-ref body gutted: source `ch.om2.add.(rmon.8.hp.6.n.Fallen).i.hat.Thief.i.Wand of Wand.i.self.Wolf.doc.Gain the effect of...` → rebuild `ch.om2.fight..mn.fallen` — the entire modifier body is discarded, replaced with an empty `fight.` head and the `.mn.` suffix (community "fallen") | Structural — catastrophic data loss | Phase 1.18 |

Each per-mod table below lists every structural class observed in that mod. Kind column indicates which pipeline kind surfaced the drift (hero / boss / monster / item / selector / phase / entity-ref / heropool / itempool).

### Sliceymon (92 modifiers: 48 heroes + 12 bosses + 10 itempools + 8 selectors + others)

From `drift_audit`: 48/92 drifted or unmatched; breakdown by kind: heropool 44 drifted + 2 unmatched, boss 4 drifted + 8 unmatched, selector 4 drifted.

| Class | Kind | Example | Concrete drift |
|---|---|---|---|
| A | hero | darumaka, larvitar, Vanillite, Slakoth | `hp.6.tier.1.col.r` ↔ `col.r.tier.1.hp.6` |
| B | hero | lillipup, T3 Haxorus, Gible | `.i.col.facade.Che7:0` / `.i.col.k.pain#facade.eba3:0:20:0` sub-entry dropped |
| C | hero | rockruff | `.i.memory` → `.i.Memory` |
| D | hero | honedge, sunkern | `Replica.Thief` → `replica.Thief`; `Heropool.` → `heropool.` |
| E | hero | applin, Gyarados | `.sd.119-1:0-0:0-0:0-0:76-2` ↔ `.sd.119-1:0:0:0:76-2`; bare-id ↔ `X-0` |
| F | hero | rockruff | `.part.1.m.4#facade.the32:0.i.memory.i.left2.k.run` → sub-entries after first `#` dropped |
| G | hero | tentomon | `.speech..n.Tentomon` emitted with empty speech value |
| H | hero | duskull | `.sd.43:30-2:30-1:30-1:123` (mixed bare/pip) → `.sd.` entirely missing |
| I | hero | applin + 25 sliceymon abilitydata blocks | `.abilitydata.(...n.X))` — rebuild emits single `)`, orphaning `.speech.`/`.doc.`/`.hp.`/`.i.self.` |
| J | hero | sunkern | `(Primrose.n.Sunkern).speech.X.col.l.img.Y.n.Sunkern` → `(replica.Primrose.sd..img.Y)` |
| O | boss | Xerneas, Yveltal, Necrozma, Dawn, Ultra, Dusk (8 unmatched) | `.mn.NAME&hidden)` vs `.mn.NAME)&hidden` suffix misplacement |
| S | selector | "Warning - Do not pick multiple of the same color" (4 drifts) | `!m(add.(dabble.tier.0.n.A.col.A.img.X))` → `!m(party.(dabble.tier.0.n.A.col.A.img.X))` — `add` verb rewritten to `party` |

### Pansaer (76 modifiers: 16 heropools + ~30 bosses + ~30 structural — per `drift_audit` kind breakdown: heropool 16/16, entity-ref 8/8, other 12/24 drifted)

Pansaer surfaces one `Heropool.(replica.Template...)` modifier per tier-slot.

| Class | Kind | Example | Concrete drift |
|---|---|---|---|
| D | heropool | All 16 lines | `Heropool.` → `heropool.` |
| P | heropool / hero | All 16: Thief/Punch, Lost/Scar, Ninja/Gorgon, Roulette/Penitent, Fighter/Hammer, Sinew/Gourmet, Leader/Crusade, Buckle/Guard, etc. | Source `.n.Punch.hp.3.img.X` → rebuild `.hp.3.i.left.hat.(Alpha.sd.8-2)#facade.OkN2:0.i.sd....img.Y` (body fabricated from template inheritance) |
| R | heropool / hero | All 16 (same lines) | Inner `.n.Punch`, `.n.Scar`, `.n.Gorgon`, `.n.Penitent`, `.n.Hammer`, `.n.Gourmet`, `.n.Crusade`, `.n.Guard` dropped |
| T | boss / fight-unit | "Fight 01", "Fight 02", "Fight 07" (12 "other"-kind drifts) | Source `fight.Slimelet+Rat.n.Venus` → rebuild `fight.Slimelet.n.Slimelet+Rat.n.Venus`; source `fight.Spider+Wolf.n.Witch` → `fight.Spider.n.Spider+Wolf.n.Witch`; source `fight.Archer+Archer+Alpha.n.Flesh` → `fight.Archer.n.Archer+Archer.n.Archer+Alpha.n.Flesh` — explicit `.n.TemplateName` auto-injected into every bare tier entry |
| W | boss / entity-ref | "Boss Fight 01" (8 entity-ref drifts) | Source `#facade.bas242:20&Hidden@4m4.fight.Dragon...` → rebuild drops the `&Hidden` suffix before the `@4` transition: `#facade.Eme66:0.i@4m4.fight.Dragon...` |

Root cause P + R: template-inheritance lookup substitutes target's body; `HeroBlock` IR has one `n` slot so inner `.n.` collides with outer. Root cause T: fight-unit emitter unconditionally emits `.n.TemplateName` after every bare entry, not preserving the source's sparse pattern.

### Punpuns (75 modifiers: 8 heropool + 28 bosses + 9 entity-refs + 7 selectors + 15 "other" (monsterpool/fight-unit) + 4 itempool + 1 difficulty + 2 phase — 52 drifted)

Kind breakdown: boss 18 drifted + 10 unmatched, entity-ref 9/9, heropool 7/8, selector 4/7, other 14/15.

| Class | Kind | Example | Concrete drift |
|---|---|---|---|
| A | heropool | T1 Yellows, T1 Blues, T2s, T3s | `.n.Flux.hp.4.tier.1.col.b` → `.col.b.tier.1.hp.4`; `.sd.` also moves to after sub-entries |
| K | heropool | Main heropool | `heropool.Thief+Scoundrel+...+Tinder+(replica.Reflection...).n.Reflection+...` — `+`-mixed list of bare names + replica blocks collapsed to a fabricated single replica block |
| Q | selector | "Random" modifier | `vSlimeQueenV1@3vSarcophagusV1@3vWitchesV1@3vBellV1@3vMatronV1@3vReflectionV1@3` → `vAlphaV1@3vTrollV1@3vBrambleV1@3vTeddyV1@3vMotherV1@3` (boss-ref list replaced with a different set) |
| R | heropool / hero | T1 Oranges, T1 Yellows (`.n.Wilding`), T1 Blues (`.n.Flux`) | Inner-block `.n.DisplayName` dropped |
| T | boss / fight-unit | Alpha, Bramble, Teddy, Mother, etc. (18 boss drifts) | Source `fight.alpha+wolf` → rebuild `fight.alpha.n.alpha+wolf.n.wolf`; `.n.TemplateName` auto-injected into every bare tier entry |
| U | boss | Alpha, Bramble, Troll, Teddy, Mother (10 bosses unmatched) | Source `((0.ph.bAlpha;1;!m(4.fight.alpha+wolf&Hidden)@2!mskip&Hidden)&Hidden).mn.Alpha` → rebuild `1.ph.bA;1;!m(4.fight.alpha.n.alpha+wolf.n.wolf&hidden).mn.Alpha@2!m(skip&hidden&temporary)`: `.ph.bAlpha` truncated to `.ph.bA`; leading `0` → `1`; outer `(( ... )&Hidden)&Hidden)` wrapper collapsed; `@N!m` suffix re-parenthesized |
| V | monsterpool | Forest Soldier, Forest Crone, Forest Worker Ant, Forest Hobgoblin (9 entity-ref + 14 "other" drifts) | Source `monsterpool.(rmon.ded.bal.boar).hp.4.n.Soldier.sd...` → rebuild `monsterpool.(replica.rmon.n.Forest Soldier).hp.4...` (body stripped, outer `.mn.` forced into inner `.n.`); source `monsterpool.(Saber.bal.rat).hp.2.sd...` → rebuild `monsterpool.(replica.Saber.n.Forest Worker Ant)...`; source `monsterpool.(rmon.ded.bal.boar).sd.X.hp.6.i.triggerhpdata.(...)` → rebuild completely restructured |

### Community (127 modifiers: 54 heropool + 18 bosses + 16 itempool + 13 entity-ref + 11 selector + 7 other + 6 phase + 2 ph-header — dominant drift)

Kind breakdown: heropool 50/54 unmatched, boss 3 drifted + 6 unmatched, entity-ref 1 drifted + 7 unmatched, phase 6/6 unmatched, other 4/7 unmatched.

| Class | Kind | Example | Concrete drift |
|---|---|---|---|
| K | heropool | Every `.modtier.X` heropool (50/54 unmatched) | Multi-tier `+`-joined bodies with embedded newlines and per-tier `.doc.`/`.img.`/`.n.` collapsed or matched incorrectly |
| L | itempool | All itempool lines | `(ritemx.ID.part.N)#(rmod.ID.part.N)#...` entity-ref groups replaced with fabricated replica content |
| M | heropool | All heropool lines | Source has `\n` between `+` tier separators; rebuild collapses to one line (cosmetic) |
| U | boss | "THE DEVs", "The Other DEVs" (6 bosses unmatched) | Bosses with embedded newlines in body (`20.ph.bBOSS;1;!m(add.\nWolf.n.JVB.hp.12...)`) not matched — body content likely lost |
| X | phase | "Fight 5?", "Change Fight?", 4 others (6/6 phase modifiers unmatched) | `ch.om4.ph.tivoid.n.Diary Page.doc.\nYour Group Notice...\n.img.pape` — multiline phase content dropped entirely |
| Y | entity-ref | "fallen" | Source `ch.om2.add.(rmon.8.hp.6.n.Fallen).i.hat.Thief.i.Wand of Wand.i.self.Wolf.doc.Gain the effect of: Wand of Wa...` → rebuild `ch.om2.fight..mn.fallen` — entire modifier body gutted to empty `fight.` + `.mn.` |

---

## Fix plan

### Phase 0 — SPEC-invariant gaps surfaced by the 2026-04-20 audit

These are violations of SPEC §3–§5/§8 that exist independently of the A–Y drift classes. They are prerequisites: every Phase 1 change lands on top of a pipeline that already satisfies them. None may be deferred to a follow-up.

0.1 **Self-contained IR — remove external sprite map (SPEC §3.3, BLOCKING).** Today `build(ir: &ModIR, sprites: &HashMap<String, String>)` (`compiler/src/lib.rs:23`, `compiler/src/builder/mod.rs:25`) requires an external sprite table. SPEC §3.3 forbids this: every type that references a sprite must own its `img_data`. Path B (author-from-scratch) is structurally impossible while the builder needs an ambient map. Fix:

- Drop the `sprites` parameter from `build`, `build_with`, `build_complete`, and every internal emitter signature.
- `img_data` moves from `Option<String>` to `String` on every type that can carry a sprite (`Hero`/`HeroBlock`, `ReplicaItem`, `Monster`, `Boss` fight units, `AbilityData`, `TriggerHpDef`). Absent = empty string is not acceptable; the extractor must populate it from the source, and the authoring layer must populate it via the typed sprite registry (see 0.6 and the authoring plan).
- Merge and CRUD must carry `img_data` through; no callsite may "fill it in later."

0.2 **`build_with` + `BuildOptions { include: SourceFilter }` (SPEC §5 #4, HIGH).** `Source::{Base, Custom, Overlay}` is stamped by the extractor and `merge` but consumed nowhere — SPEC §5 #4 says an unused provenance field is a schema bug. Fix:

- Introduce `pub struct BuildOptions { include: SourceFilter }` and `pub enum SourceFilter { All, Only(BitFlags<Source>), Exclude(BitFlags<Source>) }` (exact shape per the implementation; the contract is filter-by-provenance).
- `build_with(ir, opts)` filters every content emission by `opts.include`. `build(ir)` is `build_with(ir, &BuildOptions::default())` where the default is `All`.
- `xref` scopes findings by source: a rule violation on `Source::Base` is a warning about the source mod; `Source::Custom` / `Source::Overlay` is an author error. Add `source: Source` to `Finding` if not already present, populate it during xref, and use it in severity promotion.

0.3 **Path C merge: strip derived structurals, regenerate on build (SPEC §4, HIGH).** `compiler/src/ir/merge.rs` (lines 1–78) copies all structural modifiers including derived ones (Character Selection, HeroPoolBase, PoolReplacement, hero-bound ItemPool) — see SPEC §4 derived-structural table. This causes silent semantic drift when overlays touch heroes. Fix:

- `merge` must reject (or strip, with a warning) any derived structural it encounters on either base or overlay. SPEC says `build`/`xref` "reject it with a `CompilerError`" for user-authored derived structurals; the merge path must not smuggle them through.
- `build` regenerates derived structurals from content after merge, unconditionally — independent of what either input carried.
- Add a Path C integration test: `base + overlay` where overlay adds a hero; the rebuilt character selection / hero pools reflect the merged content, not base's snapshot.

0.4 **`ReplicaItem` kind discriminator (SPEC §3.6, §5 glossary, MEDIUM).** `ReplicaItem` at `ir/mod.rs:566–590` is one struct with no Capture/Legendary discriminator, conflating two semantically distinct kinds (SPEC §5 glossary explicitly names them as kinds of the same type). Fix:

- Add `pub kind: ReplicaItemKind` where `ReplicaItemKind::{Capture, Legendary}` is a typed enum (not a boolean, not a string).
- Extractor classifies from source shape; builder dispatches emission on `kind`.
- xref uses `kind` for the "no duplicate Pokemon across heroes / captures / legendaries / monsters" rule (SPEC §6.3).

0.5 **`panic!` in library code (SPEC §8, MEDIUM).** `compiler/src/ir/mod.rs:284` — `panic!("Expected Item segment")` inside `ModifierChain::split_at_segment()`. SPEC §8 forbids `unwrap()`/`expect()`/`panic!` in library code. Fix: return `Result<_, CompilerError>` with a populated `field_path` and `suggestion`; propagate through callers.

0.6 **`FaceId` / `SpriteId` newtypes (SPEC §3.6, BLOCKING).** `DiceFace::Active { face_id: u16, pips: i16 }` and `sprite_name: String` make invalid states representable. SPEC §3.6 requires typed newtypes with whitelist constructors: "`FaceId` is a newtype backed by the guide's whitelist — constructing an invalid `FaceId` is a compile error, not a runtime validation failure."

Design and implementation of the newtypes live in the **authoring plan** (`AUTHORING_ERGONOMICS_PLAN.md` §S5/§S6, revised to introduce newtypes, not bare `pub const u16`). The fidelity plan's obligation is:

- The IR **uses** `FaceId` and `SpriteId` in `DiceFace::Active`, `HeroBlock.sprite_name`, etc. — not `u16` / `String`.
- The extractor produces them via the fallible constructor; unknown IDs / sprites are `CompilerError`, not silently admitted.
- Round-trip and Path B tests assert equality on the newtype values.

Lands simultaneously with the authoring plan's Chunk 3/4 — neither ships alone.

0.7 **IR-equivalence gate replaces byte-count canary (SPEC §3.1, HIGH).** Clarification, not a new task: the existing `roundtrip_diag` example compares only node *counts* (`compiler/examples/roundtrip_diag.rs:79–100`). Phase 4.1 already specifies replacement with `roundtrip_verify` asserting `PartialEq` on the full `ModIR`. Note here so 0.7 is tracked with the other audit findings.

Each Phase 0 item fits the non-negotiables: no parallel fields, no deferred replacement, all callsites updated in one pass (SPEC §3.7).

### Phase 1 — IR changes

1.1 **ChainEntry sub-entry parser overlap check** — when `.col.`, `.tier.`, `.hp.`, `.t.` appears in content preceded by `.i.` or `.k.` (3-char overlap check on the preceding bytes, same pattern used for `.i.t.X` in fight_parser), do NOT treat as property boundary. It's a chain sub-entry. (Fixes class B.)

1.2 **ChainSegment/sub-entry scanner termination** — scan `#`-joined groups until the whole group terminates at a real chain boundary (next `.i.`/`.sticker.` at depth 0, or non-chain property preceded by appropriate context). (Fixes class F.)

1.3 **DiceFaces parse robustness** — accept mixed bare-id / id-pip per face within one `.sd.`. Bare id = pip count 0 (guide lines 1010/1012/1016 confirm equivalence); emitter may canonicalize. (Fixes class H.)

1.4 **HeroBlock format flag for bare head-paren hero** — when block shape is `(Template.n.Name).rest` or bare `Template.rest` (no `replica.` prefix), parse and emit preserving that shape. Add a `BlockWrapper` enum or flags (`bare`, `head_paren_with_name`). (Fixes class J.)

1.5 **Heropool mixed-list entry — distinguish user-authored from derived first.** SPEC §4 lists `HeroPoolBase` and `PoolReplacement` as derived structurals — the builder regenerates them from hero content, they are **never** round-tripped from the source text. Before touching parser/emitter:

- Inventory which heropool-adjacent modifier forms are *user-authored* (round-tripped) vs *derived* (regenerated). The community mod's 50/54 unmatched heropools may be partially explained by derived-structural regeneration, not by class K alone.
- For derived heropools: the IR-equivalence check (Phase 4.1) must either compare *before* regeneration strips them, or explicitly exclude them with a justified exception. Byte-level `drift_audit` will naturally show drift for derived structurals; that drift is expected and must be classified, not fixed.
- For user-authored heropool modifiers: new `HeropoolEntry` enum with `BareName(String)`, `ReplicaBlock(HeroBlock)`, separator-preserved list. Handles punpuns `heropool.Thief+Scoundrel+...+(replica.Reflection...).n.Reflection+...` (bare + replica mix) AND community's multi-tier `+`-joined bodies with embedded newlines (each tier is its own `ReplicaBlock`). Emitter re-joins with `+`; internal whitespace is cosmetic class M.

(Fixes user-authored scope of class K; derived scope is out-of-scope for this plan and handled by the builder's derived-structural regeneration path.)

1.6 **Itempool ritemx/rmod references** — `ItemPoolEntry::EntityRef { kind: "ritemx"|"rmod"|"rmon", id, part }` variant. Parser must match these before falling through to replica-block parsing. Guide line 1211 confirms `rmon.` as an entity prefix; `ritemx`/`rmod` follow the same pattern. (Fixes class L.)

1.7 **Block-closure paren accounting around abilitydata** — class I is the outer block's closing `)` being dropped when `abilitydata` is the final property of a `(replica...)` (or bare-template) block. The emitter must emit the block's own closing `)` regardless of what the last property is; `))` in source is just inner-close + outer-close (guide line 1012 `(Lazy.abilitydata.(Statue...n.Snooze)).n.Lazy` confirms). No new IR type — emitter correctness bug. (Fixes class I.)

1.8 **Boss `.mn.` suffix-ordering — guide lookup first (class O).** Source bosses use `.mn.NAME&hidden)` vs `.mn.NAME)&hidden`. Before any IR change, search `reference/textmod_guide.md` for `.mn.` and `&hidden`/`&Hidden` examples. Two possible outcomes only:

- **Guide treats the two orderings as equivalent** → mark class O cosmetic, normalize in the emitter, no IR field. Done.
- **Guide shows the ordering carries meaning** → represent the semantic difference with a named field (e.g., `hidden_scope: Modifier | Replica` or whatever the guide distinction actually is). Do **not** add a positional `BeforeClose|AfterClose` flag that encodes text shape.

"Preserve source verbatim because we saw both forms" is not a valid conclusion — pick one based on guide evidence.

1.9 **Inner-block `.n.DisplayName`** — `HeroBlock` (and any `(replica.Template...)` sub-block) must carry `inner_display_name: Option<String>` distinct from the outer-block `n`. Today the IR collapses both into one slot; inner `.n.Punch`/`.n.Flux`/`.n.Wilding` are silently dropped. Fix at IR + parser + emitter simultaneously. (Fixes class R.)

1.10 **No body-inheritance replacement during parse** — the hero/replica parser must not substitute a template's body for the source body. The source block IS the authoritative body; the template name is a reference for game runtime, not a parser lookup. Remove whatever template-table substitution is producing the fabricated `.i.left.hat.(Alpha.sd.8-2)...` content in pansaer. (Fixes class P.)

1.11 **Vpool / boss-ref list** — same shape as class K but for `v`-prefixed boss references in selectors like `Number#1;vX@3vY@3...`. Parser must treat `v`-separated boss-id entries as a verbatim reference list, not re-encode them from some other IR list. (Fixes class Q.)

1.12 **Selector inline modifier-kind preservation** — `Selector` IR must carry the inline verb (`add`/`party`/`skip`/etc.) inside `!m(...)` as an explicit field, not re-derive it from some default. Parser reads the verb verbatim; emitter emits it verbatim. Source `!m(add.(dabble...))` must not become `!m(party.(dabble...))`. (Fixes class S.)

1.13 **Fight-unit `.n.TemplateName` — guide lookup first (class T).** Before any IR change, search `reference/textmod_guide.md` for `fight.` examples with and without per-entry `.n.`. The design hinges on what the game does:

- **Bare `alpha` and `alpha.n.alpha` are equivalent per guide** → normalize; no per-entry flag; emitter picks one form deterministically. Class T becomes cosmetic.
- **Bare vs explicit `.n.` differ semantically** (e.g., `.n.` renames the unit, bare uses template's inner name) → the IR field is the display name itself (`display_name: Option<String>`, where `None` means "use template's intrinsic name"), not a presentation flag `has_explicit_n`.
- **Explicit `.n.Slimelet` where name matches template is a no-op** → canonicalize at parse by dropping it.

In no case should the IR carry a boolean that records "did the author type the redundant form." That is text-shape leakage (SPEC §3.6).

1.14 **Boss `.ph.b` head + phase index (class U, sub-parts a/b).** These are unambiguous data corruption:

- `.ph.bAlpha` truncated to `.ph.bA` — the parser is taking the first character of the boss name instead of the whole token. Fix: `ph_head: String` captures the full `.ph.b<NAME>` content.
- Leading phase index `0.` rewritten to `1.` — the parser is defaulting a field it failed to read. Fix: `phase_index: u8` read verbatim from source.

1.14b **Boss outer wrapper + suffix clauses — guide lookup first (class U, sub-parts c/d).** Source `((0.ph.b<NAME>;...&Hidden)&Hidden)` doubly-wrapped vs rebuild's single wrapper; source `@2!mskip&Hidden` vs rebuild's `@2!m(skip&hidden&temporary)`. Before adding `outer_wrap: Vec<Wrapper>` or `suffix_clauses: Vec<SuffixClause>`:

- Search `reference/textmod_guide.md` for phase wrapper depth rules and `!m` clause grammar.
- If wrapping/parenthesization is semantically free per guide → normalize in emitter; no IR change.
- If it is semantic (e.g., each `&Hidden` layer scopes to a different level) → encode the scoping, not the paren depth. The IR field names reflect what the game sees, not how the text is bracketed.

Do not ship `outer_wrap` as a `Vec<Wrapper>` of presentation tokens.

1.15 **Monsterpool template shape (class V)** — monsterpool entries mirror heropool's variety: `(rmon.ID...)` entity-ref form, bare `(Saber.bal.rat)` head-paren form, and `(replica.Template...)` form all occur. Add a `MonsterPoolEntry` enum (`EntityRef { kind, id, body }`, `BareTemplate { template, body }`, `ReplicaBlock { ... }`) and have the parser dispatch on the first token inside the paren. Do NOT coerce bare or entity-ref forms into `(replica.X.n.Name)` with an injected outer-`.mn.` name. (Fixes class V.)

1.16 **Suffix-group preservation around `@N` transitions (class W)** — the fight-unit / boss segmenter must preserve every `&Hidden`/`&hidden&temporary` suffix group *in place* when it immediately precedes an `@N!m` transition. Today the segmenter treats the `@N` as a hard boundary and drops the preceding suffix. Fix: capture suffix groups as terminal tokens on the preceding segment before the `@N` split. (Fixes class W.)

1.17 **Multiline phase-modifier content (class X)** — `ch.omN.ph.t<type>.n.Title.doc.<multi-line body>.img.X` phase-prompt modifiers use embedded newlines inside `.doc.` bodies. The modifier splitter and phase parser must retain newlines as part of the `.doc.` value, not treat them as modifier delimiters. All 6 community phase modifiers currently unmatched because the splitter treats `\n` inside `.doc.` as a separator. (Fixes class X.)

1.18 **`add.` modifier with entity-ref / custom body (class Y)** — `add.` modifiers can carry an arbitrary body: entity-refs (`rmon.8.hp.6.n.Fallen`), vanilla templates, item references with `.i.hat.X`/`.i.self.Y`/`.doc.Z` chains. The parser currently discards the body and emits only `fight.` + `.mn.`. The IR needs an `AddModifier { body: StructuredBody }` representation that preserves the full body as typed fields (no `raw: String` — SPEC §3.2 prohibits raw passthrough). If extraction cannot represent a construct with fields, extend the IR schema. Canonical test case: community "fallen". (Fixes class Y.)

### Phase 1b — IR-adjacent obligations (SPEC §3.4, §3.7, §5, §8)

Each new IR variant introduced in 1.1–1.18 must ship simultaneously with the items below. No variant lands half-done.

1b.1 **Authoring-layer constructors (SPEC §3.4, §6.1).** For every new IR variant (`HeropoolEntry::{BareName, ReplicaBlock}`, `ItemPoolEntry::EntityRef`, `MonsterPoolEntry::{EntityRef, BareTemplate, ReplicaBlock}`, `AddModifier`, `HeroBlock::BlockWrapper` variants, `FightUnitEntry` shape, and the `ReplicaItemKind` discriminator from 0.4), add a typed constructor in `compiler/src/authoring/` so Path B (author-from-scratch) can produce the variant without going through `extract`. The authoring-layer scope, newtypes, and constructor catalog are owned by `AUTHORING_ERGONOMICS_PLAN.md`; this plan's obligation is that **every new IR variant lands with its authoring-layer constructor in the same PR** — no IR variant merges ahead of its authoring counterpart. Direct struct-literal construction is unsupported per SPEC §6.1.

1b.2 **Provenance (SPEC §3, §4 Path C).** Every new variant that represents a content item must carry `Source::{Base, Custom, Overlay}` and propagate through `merge`. Variants that are sub-fields of an already-provenance-tracked parent inherit from the parent; confirm this explicitly rather than leaving it implicit.

1b.3 **JSON Schema (SPEC §8).** Every new IR variant, field, and enum extends the `schemars`-derived schema. A variant without a schema entry is incomplete.

1b.4 **Structured errors (SPEC §5).** Every new parser branch that can fail emits `CompilerError` / `Finding` with populated `field_path` and an actionable `suggestion`. No flat-string errors. No `unwrap()`/`expect()`/`panic!` in library code (SPEC §8).

1b.5 **No `std::fs` / `std::process` leakage (SPEC §3.4, §8).** New parser and IR code stays WASM-clean. Test fixtures may read files in `tests/`; library code must not.

### Phase 2 — Parser fixes

Apply IR changes from Phase 1 to `src/extractor/hero_parser.rs`, `src/extractor/chain_parser.rs`, `src/extractor/structural_parser.rs`, `src/extractor/boss_parser.rs`, and any other affected extractor modules.

### Phase 3 — Emitter fixes

3.1 Gate `.speech.` on `!block.speech.is_empty()` across all emitters. (Fixes class G.)

3.2 Emit bare-format and head-paren-name blocks per new format flags. (Applies class J fix.)

3.3 Emit heropool mixed-list entries verbatim, preserving bare names + replica blocks in order. (Applies class K fix, both scopes.)

3.4 Emit itempool entity-refs per fixed IR. (Applies class L fix.)

3.5 Emit boss `.mn.` suffix per the resolution of 1.8: either canonical normalized form (if guide shows equivalence) or semantic-field-driven form. No positional `BeforeClose|AfterClose` flag. (Applies class O fix.)

3.6 Emit inner-block `.n.DisplayName` when present. (Applies class R fix.)

3.7 Emit vpool/boss-ref lists verbatim. (Applies class Q fix.)

3.8 Emit block closure `)` unconditionally at block end, independent of last property. (Applies class I fix.)

3.9 Emit selector inline verb verbatim (`add`/`party`/`skip`) from IR field. (Applies class S fix.)

3.10 Emit fight-unit tier entries per the resolution of 1.13: either canonical normalized form (bare ≡ `.n.TemplateName` per guide) or semantic `display_name: Option<String>`. No `has_explicit_n` presentation flag. (Applies class T fix.)

3.11 Emit boss `.ph.b<FULL_NAME>` with full name and preserved `phase_index` (class U data-corruption fixes from 1.14). Outer wrapper and suffix-clause emission per the resolution of 1.14b: either normalized or driven by semantic scoping fields — never by a recorded paren-depth vector. (Applies class U fix.)

3.12 Emit monsterpool entries per `MonsterPoolEntry` variant (`rmon.`/`ritemx.` entity-ref, bare head-paren, replica). (Applies class V fix.)

3.13 Emit `&Hidden` suffix before `@N` transition without dropping. (Applies class W fix.)

3.14 Emit phase-modifier `.doc.` bodies with embedded newlines preserved. (Applies class X fix.)

3.15 Emit `add.` modifier bodies verbatim. (Applies class Y fix.)

### Phase 4 — Verify

Per SPEC §3.1, the primary correctness bar is **IR equality**, not byte diff. The current `roundtrip_diag` only compares IR node *counts* between source and rebuild — a shape smoke test, not a fidelity guarantee. Corrupted rebuilds are syntactically valid and the parser is intentionally lenient, so shape count is meaningless. The parser catches nothing on its own.

Phase 4 makes IR equality the contract and adds byte-level drift as a supporting canary.

4.1 **Primary gate: IR-equivalence check.** Replace `roundtrip_diag` with `roundtrip_verify` that asserts `extract(source) == extract(build(extract(source)))` as structural equality across the full `ModIR` (heroes, replica_items, monsters, bosses, structural — all fields, recursively). Derive `PartialEq` where needed on IR types. Any mismatch prints the path (`heroes[3].body.chain[2].sub_entries[0]`) and the two differing subtrees.

**Derived-structural handling:** per SPEC §4, derived structurals (Character Selection, HeroPoolBase, PoolReplacement, Hero-bound ItemPool) are regenerated from content during `build`, not round-tripped from source text. The IR-equivalence check must compare `ModIR` *excluding* derived structurals (since the source may contain author-written derived structurals that the builder regenerates), and separately assert that `regenerate_derived(extracted) == regenerate_derived(rebuilt)` — i.e., derived structurals are stable under rebuild even if they don't match the author's source.

4.2 **Path B round-trip fixtures (SPEC §4 Path B).** For each new IR variant introduced in Phase 1 (heropool mixed list, itempool entity-ref, monsterpool variants, `AddModifier`, fight-unit tier entry, boss phase fields, etc.), add a fixture under `compiler/tests/path_b/` that constructs the IR directly via the authoring layer (no `extract` call) and asserts `extract(build(ir)) == ir`. This is the regression test that the builder does not secretly depend on extractor metadata.

4.3 **Secondary canary: byte-level drift audit.** Wire `drift_audit` into the test harness (move from `examples/` to `tests/` or invoke programmatically), parametrise the classifier per class A–Y, and fail on any drift in a **structural** class **that is not also explained by derived-structural regeneration**. This is a supporting canary, not a co-equal gate — if 4.1 passes and 4.3 reports drift, the drift is evidence of a bug in the cosmetic/structural classification or of derived-structural regeneration; investigate and reclassify. Cosmetic classes (A, C, D, E, M) may freely drift.

4.4 **CI gate.** `cargo test` runs 4.1, 4.2, and 4.3 against all 4 working mods. 4.1 and 4.2 are hard gates; 4.3 is a hard gate for classes marked structural post-guide-lookup (final list depends on 1.8, 1.13, 1.14b resolutions).

4.5 **Target state:** zero IR-equivalence failures; zero Path B round-trip failures; zero byte drift in confirmed-structural classes. Cosmetic classes (A, C, D, E, M, and any of O/T/U sub-parts that guide-lookup confirms as normalize-acceptable) may remain.

4.6 **In-game paste test (user task):** sliceymon rebuild pastes without BUG curse; Axew/Larvitar show correct names during team selection.

## Correctness guarantees (non-negotiable)

- **IR equality is the contract** (SPEC §3.1). Byte-level drift is a canary, not the bar.
- **No raw-content bypass** (SPEC §3.2). Every field in IR is properly typed; no `raw: String`.
- **No text-shape leakage into IR** (SPEC §3.6). Fields encode game-observable semantics, not source presentation. No `has_explicit_n`, `hidden_placement: BeforeClose|AfterClose`, `outer_wrap: Vec<Wrapper>`, etc.
- **No parallel / compat fields** (SPEC §3.7). No `new_field` alongside `old_field`. Replace the current form across every callsite.
- **No phased scope.** Every mod, every drift class, single pass.
- **Fixes land simultaneously** across parser AND emitter AND tests AND JSON Schema AND authoring-layer constructors — no half-shipped changes (SPEC §8).
- **Every new variant carries provenance** (`Source::{Base, Custom, Overlay}`) and propagates through `merge` (SPEC §3 #4, §4 Path C).
- **Every new parser branch emits structured errors** with `field_path` and `suggestion` (SPEC §5). No `unwrap`/`expect`/`panic` in library code.
- **Guide is the tiebreaker** (SPEC §2). When parser, emitter, and `reference/textmod_guide.md` disagree, the guide wins. Classes O, T, and U sub-parts require a guide lookup before the IR shape is decided.

## Non-goals

- Byte-identity for drifts that Thunder's guide describes as equivalent forms.
- Preserving leading/trailing whitespace or formatting that doesn't affect game behavior.
- Optimizing parser perf.

## Resolved questions (answered from guide + source audit)

1. **`abilitydata.((X))` vs `abilitydata.(X)`** — not a distinct format. Guide line 1012 (`(Lazy.abilitydata.(Statue...n.Snooze)).n.Lazy`) and sliceymon's 25 in-source occurrences confirm `))` is `inner-close + outer-block-close`. Class I is a block-closure bug, not a format choice. Resolved.
2. **Bare-id DiceFace `43` vs `43-0`** — semantically equivalent. Guide lines 1010 (`0-0`), 1012 (`0`, `5`), 1016 (`0-0`) all coexist without distinction. Parser must accept all three shapes; emitter may canonicalize. Resolved.
3. **Property-order invariant in replica blocks** — none. Guide line 857 uses `col.hp.sd`, line 1041 uses `hp.col.sd`. Class A is cosmetic. Resolved.

## Audit provenance

Drift tables above were produced by `cargo run --release --example drift_audit` against `working-mods/{sliceymon,punpuns,pansaer,community}.txt`. Per Phase 4, this tool becomes a structural-class-gated test that runs under `cargo test`, alongside the new IR-equivalence check that replaces `roundtrip_diag`.

---

## Tasks

- Fix hero parser/emitter for source fidelity → Phases 1–3
- Verify full-mod structural correctness (no byte-identity required for cosmetic classes) → Phase 4
