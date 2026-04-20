# Pipeline Fidelity Plan

> **Target:** compiler rebuild produces **structurally correct** textmods for all 4 known-working mods (sliceymon, pansaer, punpuns, community). Byte-identity is only required where Thunder's guide implies a form carries meaning; otherwise normalization is acceptable. Covers the full extract→build pipeline: heroes, bosses, monsters, items, selectors, phases, entity-refs, and structural modifiers.

## Context & trigger

After the fight-unit fixes landed (chain-segment interleaving, head-paren Case B with single-paren nested, multi-variant last-fight name extraction), all 3 `ch.om` bosses in sliceymon paste byte-identically and the 2 previously-failing bosses (Quagsire, Floor8) are resolved.

On in-game paste-test, sliceymon now paste-succeeds but exhibits **gameplay-affecting drift** post-paste:
- **BUG curse** is applied to the run (symptom of corrupted content)
- **Axew → "AA", Larvitar → "HS"** during team selection (heroes loading with wrong names)

Audit across all 4 mods confirms widespread hero-pipeline drift (not limited to sliceymon). Each mod shows data-loss drifts beyond mere property reorder.

## Correctness bar

Thunder's guide is the source of truth.

- **Must preserve** any shape the guide describes as semantically meaningful.
- **May normalize** shapes the guide shows in multiple equivalent forms without claiming one is required.

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
| O | Boss `.mn.NAME&hidden)` vs `.mn.NAME)&hidden` suffix placement | Structural — suffix placement must be preserved | Phase 1.8 |
| P | Hero body content replaced by inheritance-target fabrication (parser substitutes a template's body for the source's authored body) | Structural — data corruption | Phase 1.10 |
| Q | `v`-prefixed boss-ref list in selectors (`Number#1;vX@3vY@3...`) replaced with a different set of boss refs | Structural — data corruption | Phase 1.11 |
| R | Inner-block `.n.DisplayName` (`.n.Punch`, `.n.Flux`, `.n.Wilding`, etc. *inside* a `(replica.Template...)` block) dropped | Structural — data loss | Phase 1.9 |
| S | Selector inline modifier-kind rewrite: `!m(add.(dabble...))` → `!m(party.(dabble...))` (sliceymon Warning selector; the `add`/`party`/`skip` inline verb inside a selector's `!m(...)` clause is being replaced) | Structural — data corruption | Phase 1.12 |
| T | Fight-unit auto-injection of `.n.TemplateName` into bare tier entries: source `fight.alpha+wolf` → rebuild `fight.alpha.n.alpha+wolf.n.wolf`; source `fight.Slimelet+Rat.n.Venus` → rebuild `fight.Slimelet.n.Slimelet+Rat.n.Venus`. Parser normalizes every fight-unit entry to carry an explicit `.n.`, synthesizing one when absent | Structural — content fabrication (sometimes semantically equivalent, sometimes not; the game treats bare `alpha` and `alpha.n.alpha` differently in some cases — verify per entry) | Phase 1.13 |
| U | Boss `.ph.` prefix + outer wrapper rewrite: source `((0.ph.bAlpha;1;!m(...)&Hidden)@2!mskip&Hidden)&Hidden).mn.Alpha` → rebuild `1.ph.bA;1;!m(...&hidden).mn.Alpha@2!m(skip&hidden&temporary)` — (a) `.ph.bAlpha` truncated to `.ph.bA`, (b) leading phase index `0` changed to `1`, (c) doubly-wrapped `(( ... )&Hidden)&Hidden)` collapsed to a single flat form, (d) `@N!m` suffix re-parenthesized | Structural — data corruption | Phase 1.14 |
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

### Phase 1 — IR changes

1.1 **ChainEntry sub-entry parser overlap check** — when `.col.`, `.tier.`, `.hp.`, `.t.` appears in content preceded by `.i.` or `.k.` (3-char overlap check on the preceding bytes, same pattern used for `.i.t.X` in fight_parser), do NOT treat as property boundary. It's a chain sub-entry. (Fixes class B.)

1.2 **ChainSegment/sub-entry scanner termination** — scan `#`-joined groups until the whole group terminates at a real chain boundary (next `.i.`/`.sticker.` at depth 0, or non-chain property preceded by appropriate context). (Fixes class F.)

1.3 **DiceFaces parse robustness** — accept mixed bare-id / id-pip per face within one `.sd.`. Bare id = pip count 0 (guide lines 1010/1012/1016 confirm equivalence); emitter may canonicalize. (Fixes class H.)

1.4 **HeroBlock format flag for bare head-paren hero** — when block shape is `(Template.n.Name).rest` or bare `Template.rest` (no `replica.` prefix), parse and emit preserving that shape. Add a `BlockWrapper` enum or flags (`bare`, `head_paren_with_name`). (Fixes class J.)

1.5 **Heropool mixed-list entry** — new `HeropoolEntry` enum with `BareName(String)`, `ReplicaBlock(HeroBlock)`, separator-preserved list. Handles punpuns `heropool.Thief+Scoundrel+...+(replica.Reflection...).n.Reflection+...` (bare + replica mix) AND community's multi-tier `+`-joined bodies with embedded newlines (each tier is its own `ReplicaBlock`). Emitter re-joins with `+`; internal whitespace is cosmetic class M. (Fixes class K, both scopes.)

1.6 **Itempool ritemx/rmod references** — `ItemPoolEntry::EntityRef { kind: "ritemx"|"rmod"|"rmon", id, part }` variant. Parser must match these before falling through to replica-block parsing. Guide line 1211 confirms `rmon.` as an entity prefix; `ritemx`/`rmod` follow the same pattern. (Fixes class L.)

1.7 **Block-closure paren accounting around abilitydata** — class I is the outer block's closing `)` being dropped when `abilitydata` is the final property of a `(replica...)` (or bare-template) block. The emitter must emit the block's own closing `)` regardless of what the last property is; `))` in source is just inner-close + outer-close (guide line 1012 `(Lazy.abilitydata.(Statue...n.Snooze)).n.Lazy` confirms). No new IR type — emitter correctness bug. (Fixes class I.)

1.8 **Boss `.mn.` suffix-ordering preservation** — source bosses use `.mn.NAME&hidden)` (suffix inside close) or `.mn.NAME)&hidden` (suffix outside). Add `hidden_placement: BeforeClose | AfterClose` on the boss IR node and preserve through emit. (Fixes class O.)

1.9 **Inner-block `.n.DisplayName`** — `HeroBlock` (and any `(replica.Template...)` sub-block) must carry `inner_display_name: Option<String>` distinct from the outer-block `n`. Today the IR collapses both into one slot; inner `.n.Punch`/`.n.Flux`/`.n.Wilding` are silently dropped. Fix at IR + parser + emitter simultaneously. (Fixes class R.)

1.10 **No body-inheritance replacement during parse** — the hero/replica parser must not substitute a template's body for the source body. The source block IS the authoritative body; the template name is a reference for game runtime, not a parser lookup. Remove whatever template-table substitution is producing the fabricated `.i.left.hat.(Alpha.sd.8-2)...` content in pansaer. (Fixes class P.)

1.11 **Vpool / boss-ref list** — same shape as class K but for `v`-prefixed boss references in selectors like `Number#1;vX@3vY@3...`. Parser must treat `v`-separated boss-id entries as a verbatim reference list, not re-encode them from some other IR list. (Fixes class Q.)

1.12 **Selector inline modifier-kind preservation** — `Selector` IR must carry the inline verb (`add`/`party`/`skip`/etc.) inside `!m(...)` as an explicit field, not re-derive it from some default. Parser reads the verb verbatim; emitter emits it verbatim. Source `!m(add.(dabble...))` must not become `!m(party.(dabble...))`. (Fixes class S.)

1.13 **Fight-unit `.n.TemplateName` sparsity preservation** — `FightUnit` tier entries (the `+`-joined template list inside `fight.X+Y+Z.n.Name`) must remember which entries had an explicit `.n.` in source and which did not. Emitter must not auto-inject `.n.TemplateName` after bare entries. Source `fight.alpha+wolf` must round-trip verbatim; source `fight.Slimelet+Rat.n.Venus` must not gain a synthesized `.n.Slimelet`. (Fixes class T.)

1.14 **Boss `.ph.b` + outer-wrapper preservation** — the boss IR must capture (a) the full `.ph.b<NAME>` head token verbatim (not truncate to a single letter), (b) the phase index prefix (`0.` vs `1.`), (c) the outer paren-nesting depth and `&Hidden` placement of the boss entry (`((0.ph.b<NAME>;...&Hidden)&Hidden)`), and (d) the suffix-clause re-parenthesization (`@2!mskip&Hidden` vs `@2!m(skip&hidden&temporary)`). Add explicit fields: `ph_head: String`, `phase_index: u8`, `outer_wrap: Vec<Wrapper>`, `suffix_clauses: Vec<SuffixClause>`. (Fixes class U.)

1.15 **Monsterpool template shape (class V)** — monsterpool entries mirror heropool's variety: `(rmon.ID...)` entity-ref form, bare `(Saber.bal.rat)` head-paren form, and `(replica.Template...)` form all occur. Add a `MonsterPoolEntry` enum (`EntityRef { kind, id, body }`, `BareTemplate { template, body }`, `ReplicaBlock { ... }`) and have the parser dispatch on the first token inside the paren. Do NOT coerce bare or entity-ref forms into `(replica.X.n.Name)` with an injected outer-`.mn.` name. (Fixes class V.)

1.16 **Suffix-group preservation around `@N` transitions (class W)** — the fight-unit / boss segmenter must preserve every `&Hidden`/`&hidden&temporary` suffix group *in place* when it immediately precedes an `@N!m` transition. Today the segmenter treats the `@N` as a hard boundary and drops the preceding suffix. Fix: capture suffix groups as terminal tokens on the preceding segment before the `@N` split. (Fixes class W.)

1.17 **Multiline phase-modifier content (class X)** — `ch.omN.ph.t<type>.n.Title.doc.<multi-line body>.img.X` phase-prompt modifiers use embedded newlines inside `.doc.` bodies. The modifier splitter and phase parser must retain newlines as part of the `.doc.` value, not treat them as modifier delimiters. All 6 community phase modifiers currently unmatched because the splitter treats `\n` inside `.doc.` as a separator. (Fixes class X.)

1.18 **`add.` modifier with entity-ref / custom body (class Y)** — `add.` modifiers can carry an arbitrary body: entity-refs (`rmon.8.hp.6.n.Fallen`), vanilla templates, item references with `.i.hat.X`/`.i.self.Y`/`.doc.Z` chains. The parser currently discards the body and emits only `fight.` + `.mn.`. The IR needs an `AddModifier { body: RawContent | StructuredBody }` representation that preserves the full body and an emitter that round-trips it. Canonical test case: community "fallen". (Fixes class Y.)

### Phase 2 — Parser fixes

Apply IR changes from Phase 1 to `src/extractor/hero_parser.rs`, `src/extractor/chain_parser.rs`, `src/extractor/structural_parser.rs`, `src/extractor/boss_parser.rs`, and any other affected extractor modules.

### Phase 3 — Emitter fixes

3.1 Gate `.speech.` on `!block.speech.is_empty()` across all emitters. (Fixes class G.)

3.2 Emit bare-format and head-paren-name blocks per new format flags. (Applies class J fix.)

3.3 Emit heropool mixed-list entries verbatim, preserving bare names + replica blocks in order. (Applies class K fix, both scopes.)

3.4 Emit itempool entity-refs per fixed IR. (Applies class L fix.)

3.5 Emit boss `.mn.NAME&hidden)` / `.mn.NAME)&hidden` per preserved placement flag. (Applies class O fix.)

3.6 Emit inner-block `.n.DisplayName` when present. (Applies class R fix.)

3.7 Emit vpool/boss-ref lists verbatim. (Applies class Q fix.)

3.8 Emit block closure `)` unconditionally at block end, independent of last property. (Applies class I fix.)

3.9 Emit selector inline verb verbatim (`add`/`party`/`skip`) from IR field. (Applies class S fix.)

3.10 Emit fight-unit tier entries respecting per-entry `has_explicit_n` flag; never synthesize `.n.TemplateName`. (Applies class T fix.)

3.11 Emit boss `.ph.b<FULL_NAME>` with preserved phase index, outer wrapper, and suffix-clause structure. (Applies class U fix.)

3.12 Emit monsterpool entries per `MonsterPoolEntry` variant (`rmon.`/`ritemx.` entity-ref, bare head-paren, replica). (Applies class V fix.)

3.13 Emit `&Hidden` suffix before `@N` transition without dropping. (Applies class W fix.)

3.14 Emit phase-modifier `.doc.` bodies with embedded newlines preserved. (Applies class X fix.)

3.15 Emit `add.` modifier bodies verbatim. (Applies class Y fix.)

### Phase 4 — Verify

The current `roundtrip_diag` only compares IR node *counts* between source and rebuild — it is a shape-preservation smoke test, not a fidelity guarantee. Every structural class in this plan (B/F/P/Q/R/I/K/L) passes count-equivalence while silently corrupting content. The parser itself cannot catch this either: corrupted rebuilds are syntactically valid, the parser is intentionally lenient (mods must be authorable in non-canonical forms), and there is no source↔rebuild equivalence check anywhere in the pipeline.

Phase 4 closes both gaps.

4.1 **Replace `roundtrip_diag` with an IR-equivalence check.** Rename to `roundtrip_verify` (or similar) and make it assert `extract(source) == extract(build(extract(source)))` as structural equality across the full `ModIR` (heroes, replica_items, monsters, bosses, structural — all fields, recursively). Derive `PartialEq` where needed on IR types. Any mismatch prints the path (`heroes[3].body.chain[2].sub_entries[0]`) and the two differing subtrees. Node-count reporting stays as a first-pass summary but no longer gates `OK`.

4.2 **Wire `drift_audit` into the test harness.** Move it from `examples/` to `tests/` (or add a test that invokes it programmatically), parametrise the classifier (cosmetic vs structural per class A–Y), and fail the test on any drift in a structural class. This is the byte-level counterpart to 4.1's IR-level check — they must both pass.

4.3 **CI gate.** `cargo test` must run both 4.1 and 4.2 against all 4 working mods. No `ROUNDTRIP OK` print without both checks green.

4.4 Target: zero drift in structural classes (B, F, G, H, I, J, K, L, O, P, Q, R, S, T, U, V, W, X, Y). Cosmetic classes (A, C, D, E, M) may remain — each is guide-backed equivalent.

4.5 In-game paste test (user task): sliceymon rebuild pastes without BUG curse; Axew/Larvitar show correct names during team selection.

## Correctness guarantees (non-negotiable)

- No raw-content bypass. Every field in IR is properly typed.
- No parallel / compat fields ("old_field alongside new_field"). Replace the current form.
- No phased scope. Every mod, every drift class, single pass.
- Fixes land across parser AND emitter AND tests AND schema simultaneously — no half-shipped changes.

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
