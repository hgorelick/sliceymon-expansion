# Chunk 8A.5 — Typed `NonSummonEntry` schema (close SPEC §3.2 raw-passthrough violation)

> **Parent plan**: `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md`.
> **Upstream dependency**: `plans/CHUNK_8A_REPLICA_IR_ATOMIC_REWRITE.md` (ships `ItempoolItem::NonSummon { name: String, tier: Option<i8>, content: String }` as a **transitional** raw-passthrough form; this chunk retires the `content: String` field).
> **Downstream dependency**: `plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md` (wires the real summon extractor; cannot start until 8A.5 lands because 8B's `extract_from_itempool` must classify every `+`-separated entry into either `ItempoolItem::Summon(i)` OR a typed `ItempoolItem::NonSummon(NonSummonEntry)` — no raw-string escape).
>
> **Ships as a single one-shot sub-chunk** per `personas/ai-development.md`: enumerate every non-summon shape from corpus, build a recursive typed IR covering every byte of every entry, wire extractor+emitter+retest, commit. One PR, one `cargo test` clean gate.
>
> **Authority rule**: every corpus reference in this plan is anchored by **verbatim quoted substring** from `working-mods/*.txt`, never by line number. Every variant named below has ≥1 corpus instance quoted inline in §1.3. Variants with zero corpus instances are hypotheses and do not ship (hook rule #3, `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` §3.7 Lessons entry).

---

## 1. Overview

### 1.1 What this chunk does

Replaces `ItempoolItem::NonSummon { name, tier, content: String }` with a fully typed recursive sum `ItempoolItem::NonSummon(NonSummonEntry)` where `NonSummonEntry` models every observed shape of a non-summon itempool entry in corpus. Every byte of every entry lands in a typed field; there is no `content: String`, no `body: String`, no `residue: Option<String>`, no `extras: Vec<RawSubBlock>`, no escape hatch of any kind. The shared outer `.n.<name>.tier.<n>.img.<…>.doc.<…>…` tail is itself a first-class typed record (`NonSummonTrailer`), recursively typed.

This chunk also closes the corresponding emitter path — `emit_itempool` (shipped in 8A) rebuilds every non-summon entry's source bytes from the typed IR, not from a stored string. Every non-sliceymon working mod that currently round-trips via the 8A stub's one-blob-per-pool `NonSummon { content: <entire body> }` must continue to round-trip byte-equal after 8A.5 through the typed reconstruction path.

### 1.2 Why this chunk exists — SPEC §3.2 violation closure

SPEC §3.2 ("No raw passthrough") is authoritative: every byte in a textmod must either parse into typed IR or fail the extract. 8A landed a **transitional** `NonSummon.content: String` — an admitted, tracked debt — because closing it in the same commit as the IR atomic rewrite would have pushed 8A past any reasonable file-touch count. Leaving it is not an option: per `CLAUDE.md` "No deferred correctness", a parallel-representation hatch (typed summons + stringly-typed non-summons) is the exact pattern the repo's design rules forbid.

8A.5 is the dedicated, evidence-first chunk that types the non-summon half of the `ItempoolItem` sum. 8B's real extractor (§3.3 parent plan) depends on 8A.5 landing first because 8B's classifier must produce **only typed values** on both branches; if 8A.5 hasn't landed, 8B's only non-summon output surface is the transitional raw string, which re-introduces the SPEC §3.2 violation at every summon-failed entry.

### 1.3 Corpus audit — variant list with per-mod counts

This section is the **evidentiary authority for every variant in §3.1's IR schema**. Implementation must re-run the corpus enumeration (§3.2.1 Phase-A exhaustive walk) at impl start; if the per-variant counts below disagree with the Phase-A output, halt and rewrite this section before writing any IR code. The numbers here are a plan-time snapshot of the classifier's target; they are not a fixed oracle.

**Non-summon pool counts (plan-time snapshot — `+`-joined entries at paren-depth 0 of each itempool body, excluding the `Summon` entries extracted by 8B). Re-verify at impl start.**

| Mod | Pools | Pool-shape note | Summon entries (8B path) | Non-summon entries (this chunk's scope) |
|---|---|---|---|---|
| sliceymon | 4 itempool pools (Pokeballs Part 1, Pokeballs Part 2, Summons Part 1, Summons Part 2) + 4 `!mitempool.` monster-item-injection modifiers (Porygon/Ditto item overrides) + 2 misc `itempool.` modifiers (Scary Face TM list; `Clear Itempool`) | Pokeballs pools use `itempool.((<entries>)).n.<PoolName>.tier.<n>`; Summons pools use `itempool.((<entries>)).mn.<PoolName>`; non-Pokeball `itempool.` modifiers vary (TM list uses `itempool.(<entry>)+(<entry>)+…`, `Clear Itempool` uses `itempool.Void.part.0.mn.Clear Itempool` with NO paren body); `!mitempool.` uses `!mitempool.((<entries>)).n.<Name>.tier.<n>`. | 23 summon envelopes (parent §1.1) | ~13 (10 Pokeball accessory-composed entries, ≤3 misc; **phase-A walk required**) |
| pansaer | 11 Itempool modifiers (capital `I`, single-paren) | `Itempool.(<entry1>)+(<entry2>)+…+BareName1+BareName2+…&Hidden.mn.<PoolName>` | 0 | ~145 distinct entries (hand-count by `+`-split on pool 1 = 60 entries; plan-time rough estimate across 11 pools = 120–170; **phase-A walk required**) |
| punpuns | 4 itempool modifiers | Shape varies; walk required | 0 | **phase-A walk required** |
| community | 19 itempool modifiers | Multi-line bodies, `itempool.\n\n<entries>`; `+`-joined at paren-depth 0 | 0 | ~80–100 per main pool plus ~40–60 across the other 18 (**phase-A walk required**) |

**Plan-time claim (to be verified at impl start)**: non-summon entries vastly dominate the corpus. The 23 summon envelopes in sliceymon are a minority shape; 8A.5's typed schema covers the base case. The specific counts above are ROUGH; the Phase-A walk in §3.2.1 is the authoritative source.

**Variant classes observed in the corpus (each anchored by a verbatim quoted substring from a specific `working-mods/*.txt` file; implementation must re-confirm every quote reads exactly as written here at impl start — any mismatch is plan defect, fix before coding).** Variant numbering is IR-internal; §3.1 names the enum variants.

**Component types (A5-9 fix — prose catalog of every `CompositionComponent` variant named below; each variant must appear in ≥1 V-N entry listed after this block).** The top-level `NonSummonEntry` variants decompose into `CompositionComponent` sub-components:

- `BaseGameRef { name }` — used in V1, V2, V3 (bare-item component, `Amnesia` / `Compulsion`).
- `Keyword { keyword, suffixes }` — used in V8, V14 (bare keyword `k.death`, `k.first`, `k.singleuse`).
- `Learn { spell }` — used in V15 (bare learn-spell `learn.Poke`, `learn.Bandage`).
- `Ritemx(RitemxRef)` — used in V4, V5, V7 (ritemx reference head).
- `SideDef { template, dice, i_chain, sidesc, facade }` — used in V14 (side-definition with sidesc/facade, e.g. pansaer Whistle).
- `Scoped { scope, body }` — used in V3 and others where a scope prefix (`self`, `rightmost`, `top`, etc.) wraps a sub-component.
- `Nested(NonSummonEntry)` — used whenever a paren-wrapped sub-composition appears inside a `#`-chain (recursive).
- `Jinx { body }` — used in V9 (template-jinx body).
- `Ability { template, body }` — used in V10 (inline ability definition).
- `Vase(VaseOp)` — used in V9 and allitem bodies that embed `t.vase.(…)` operations.
- `Sentinel { token }` — used in V12 when the sentinel is COMPOSED with other content (entry-level bare sentinel with outer-name-only goes through `StructuralContent::ItemPool { outer_name, items: vec![] }`, not a top-level NonSummonEntry variant; see V12 note above).

**Every V-N entry below must map every component it uses to one of the above.** If the Phase-A walker surfaces a component not enumerated here, widen `CompositionComponent` in the SAME commit — no string fallback (see D-E).

**V1 — BareBaseGameRef.** A bare base-game item name, no wrapper, no renames, no modifiers. Pool-membership only. Instance: `Amnesia` in pansaer pool 1's tail segment `…tier.0)+Amnesia+Broken Spirit+Compulsion+Parasite+Pharaoh Curse+Soul Link+Wretched Crown+Affliction+Brittle+Broken Heart+Cursed Bolt+D4+Handcuffs+Martyr+Mould+Tracked+Big Fish+Brick+…&Hidden.mn.Tier 0 and Lower Items`. Corpus counts (plan-time): dozens per pansaer pool tail, 0 in sliceymon, **walk required** for community/punpuns.

**V2 — BaseGameRefWithAccessoryChain.** A base-game item name composed with `#`-joined accessory/keyword modifiers, no outer paren wrap. Instance: `self.j2k#leather vest.m.2.n.Cat Ears.doc.mini.img.wolf ears.hsv.0:30:20.tier.0` in community line 13. The leading `self.j2k#leather vest.m.2` is the accessory-composition; the `.n.Cat Ears.…` tail is the trailer. Distinct from V3 (which has paren-wrapped composition) only by outer-paren presence — the IR can collapse them; see §3.1 Design decision D-A.

**V3 — ParenWrappedComposition.** A paren-wrapped `#`-joined composition + trailer. Instance: `(self.unpack.Boss Smash^99#k.unusable#Camomile#k.stasis).n.Consolation Prize.doc.bord.tier.0.img.Bond Certificate.hsv.0:0:50` in community line 16. The `(X#Y#Z)` form is structurally identical to V2's head, merely paren-wrapped. Repeats with varied inner shapes: `(Ocular Amulet.m.3#(left.k.rampage)#(left.k.hyperboned))` (community line 76; note inner `(left.k.rampage)` is itself a paren-wrapped sub-ref), `((Gauntlet)#Peaked Cap#Dragonhide Gloves#(right2.Square Wheel))` (sliceymon's Choice Band entry — NOTE: this is sliceymon's FIRST sliceymon `itempool.((` pool, named `Choice Band.tier.8`).

**V4 — RitemxRef (with optional `.part.<n>`, `.splice.<body>`, `.m.<mult>`, composition).** A `ritemx.<hex>` reference, possibly with part/splice/multiplier suffixes and/or `#`-composition. Instances:
- `ritemx.fb71.n.Blue World.doc.kas333.tier.0.img.coin.hsv.40:50:0` (community line 70) — plain ritemx + trailer, no composition.
- `ritemx.11b99.part.0.n.Purple Heart.tier.0.doc.mini.img.big heart.hsv.80:-20:0` (community line 100) — ritemx + part, no composition.
- `ritemx.10181.part.0.n.Curse Of Light.tier.0.doc.…` (community line 79) — same shape.
- `unpack.ritemx.bf25.n.Ash Of War.doc.mini.tier.0.img.Powdered Mana.hsv.40:-90:0` (community line 67) — ritemx with `unpack.` prefix.
- `ritemx.a482.part.1#bot.k.unusable.n.Disabled.tier.0.img.bone charm.hsv.0:-20:-50.doc.[g] mere` (community line 82) — ritemx + `#`-composition with a keyword accessory.
- `(ritemx.a348).tier.0.n.Seeing Red.doc.Sefcear.img.sapphire skull.hue.42` (community line 43) — paren-wrapped ritemx only (trailer OUTSIDE paren — see §3.1 Design decision D-B).
- `(ritemx.5770.part.0)#(ritemx.431.part.0)#(ritemx.7419.part.0)#(ritemx.aed7.part.1)#(ritemx.b6ea.part.0)#(ritemx.152a9.part.0)#(ritemx.14c96.part.0)#k.rainbow.n.Rainbow hat.doc.punpun.img.2ed000…tier.0` (community line 37) — seven `#`-joined paren-wrapped ritemx refs + one bare-keyword accessory + trailer. Trailer is at paren-depth 0 outside every inner paren.

**V5 — RitemxWithSpliceAndRename.** `ritemx.<hex>.splice.<target>.tier.<n>.n.<newname>.img.<bytes>`. Instance (from the ask-context sample): `ritemx.158948.splice.Blindfold.tier.0.n.Power Herb.img.<bytes>`; `ritemx.1768a.splice.t.Boar.tier.1.n.Focus Sash.img.<bytes>` (both quoted in the agent-instructions ask; grep these at impl start to locate them in the exact mod — they appear to be inside sliceymon's non-summon itempool entries or community; Phase-A walk confirms). This is V4 with a recursive `.splice.<body>` payload where `<body>` itself is a `NonSummonEntry` (typed recursion — see §3.1).

**V6 — ChainCompositionWithRename.** A composition like `(A#B#C).n.<newname>.tier.<n>.img.<bytes>` where the pre-paren content already has a name and the `.n.<newname>` is an outer rename. Instance from the ask-context: `(Gauntlet.splice.k.selfheal).n.Shell Bell.tier.4.img.<bytes>`. Distinguished from V3 by the presence of a body-internal `.n.` and/or `.splice.` that the outer trailer overrides.

**V7 — HashJoinedRitemxWithRename.** A `#`-joined set of ritemx refs at the top of the entry, then a trailer. Instance: `(ritemx.384923#ritemx.259131#ritemx.1615972).n.Twisted Spoon.tier.4.img.<bytes>` (quoted in ask-context). In corpus, community line 88 is a realized form: `((ritemx.b45a)#(ritemx.bcfd)#(ritemx.11b99.part.0)#(unpack.ritemx.bf25)#(ritemx.e338.part.0)#(ritemx.10181.part.0)#(ritemx.fb71)#(ritemx.6349.part.2)#(ritemx.a348)).n.Keyword Hell 2.doc.mini l sef.tier.0.img.taxes.hsv.40:60:0`.

**V8 — KeywordCompositionWithRename.** Bare keyword/accessory composition with a rename, no ritemx. Instance from ask-context: `k.first.n.Carbos.tier.7.img.<bytes>`, `k.treble.tier.6.n.Expert Belt` (these bare-keyword renames appear in the ask-context sample; §3.2.1 Phase-A walk will locate the exact corpus site). In corpus: `k.pristeel.n.Titanium.img.2c50009abgjlq…` in pansaer. Also: `(k.death.n.Destiny.tier.0.img.Sapphire Skull.doc.kas333 l mini l ajfish)` (community line 130) — paren-wrapped form.

**V9 — TemplateJinxWithComposition.** `t.jinx.<body>` / `t.<name>.<body>` compositions, often with nested parens. Instances:
- `t.jinx.unpack.Wurst/4.tier.0.n. .doc.<bytes>.img.<bytes>` (community line 94; note the name `.n. .` — literal space). Unpack-summon shape.
- `t.jinx.unpack.et2.Summon.Slimelet).n.The Flu.tier.0.doc.rorbee.img.slimed.hue.-10` → the actual entry is `(t.jinx.unpack.et2.Summon.Slimelet).n.The Flu.…` (community line 55). Paren-wrapped template jinx.
- `t.jinx.allitem.(k.enduring#(all.twin daggers.n.mini circus)#k.pain#k.unusable).n.Mini Circus.doc.roboxeno.tier.0.img.clumsy shoes` (community line 196). The `t.jinx.allitem.(…)` body contains recursive composition.

**V10 — InlineAbilityDefinition.** `t.<Template>.abilitydata.(<body>).n.<name>…` — a non-cast inline ability definition (distinct from 8B's `SummonTrigger::Cast` which is summon-routed). Instance: `(t.zm.abilitydata.(statue.sd.176-1:0-0:0-0:0-0:76-0:0-0.n.Spamming.img.<bytes>)).doc.kas333.n.Spam.tier.0` (community line 46). The outer entry's trailer is AFTER the outer `)`. The abilitydata body is itself recursive content that may include keyword chains, sides, sidescs, faces, images.

**V11 — DeepNestedAllitemWithTrailer.** `self.<scope>.allitem.<body>.n.<outer>.…` with recursive body. Instance: `self.(t1.allitem.t.jinx.(Summon.Wolf.i.Twisted Bar.t.jinx.monster.fluctuate.img.dice.p.fff:111:90.n.Chaos Dice).n.Summon Chaos Dice.img.dice).n.Pandoras Cube.doc.rorbee.tier.0.img.golden d6.hue.30` (community line 160). Note TWO depth-0 `.n.<…>` sites — the inner `.n.Chaos Dice` and `.n.Summon Chaos Dice` are INSIDE the outer paren; the outer `.n.Pandoras Cube` is at entry top-level. IR must preserve both.

**V12 — BareVoidOrSpecialRoot (A5-3 fix — clarified: NOT an ItempoolItem variant at the outer-modifier case).** Entry-level V12 covers bare anchor tokens (`Void.part.0`, `uy.`, `Void#…`) that appear INSIDE an itempool body as `+`-split entries. The sliceymon `itempool.Void.part.0.mn.Clear Itempool` case is **NOT an itempool entry** — it is the OUTER `StructuralContent::ItemPool { items: vec![], outer_name: Some("Clear Itempool") }` record with no entries at all. When a sentinel (`Void`, `uy`) appears *composed* with other content inside an actual itempool entry, it classifies as `Composition` with a `CompositionComponent::Sentinel { token }` component (see A5-9 for the prose callout of this component). Instances:
- `(Void#rightmost.k.potion.n.Empty Bottle.img.ite272.hsv.0:-70:0.tier.0)` (pansaer pool 1 entry). The `Void` token is a pool anchor (presence-sentinel); `#`-composed with the rest.
- `uy.n.Glyph.img.<bytes>.doc.mini` (community line 136). Bare `uy` with trailer only.
- `uy#hat.(Ace.sd.1-5:2-5:3-5:7-5:9-5:6-5.n.5 damage X).n.Treasure Map.doc.<bytes>.tier.0.img.paper.hsv.50:0:0` (community line 115). `uy` + `#`-composition.
- `itempool.Void.part.0.mn.Clear Itempool` (sliceymon line 103) — this is an ENTIRE itempool modifier consisting of a bare `Void.part.0` with `.mn.Clear Itempool`, no paren body. This is the OUTER itempool modifier, not a nested entry; it still must round-trip through the typed IR (SPEC §3.2 applies to the outer modifier too). See §3.1 on how `StructuralContent::ItemPool { items }` models the "no entries, outer-name-only" case.

**V13 — NestedDoubleParenOuter.** `((X).n.<...>)` or `((X#Y).n.<…>))` — entry is itself wrapped in an extra paren pair. Instances:
- `((hat.(zm.sd.181-0:…).n.The Contraption.i.Pharaoh Curse.part.1.m.4#k.groooooowth#k.singleuse.i.left.k.inflictdeath)#(handcuffs.part.1).n.Rube Goldberg Machine.img.uy.draw.wand grips:-2:2.draw.lead weight:4:0.tier.0.doc.punpun))` (community line 103). Note the outer `((` + `))` and the body-internal `#`-composition with paren sub-groups.
- `((hat.gambler)#(demon claw)#(self.left.hyperboned)).n.Therapy.tier.0.doc.posalla.img.Bone Charm` (community line 112) — outer double-paren, three paren-wrapped inner sub-items.

**V14 — SideDefWithSidescAndFacade.** A chain of `hat.<Template>.sd.<faces>.i.<keyword>#sidesc.<text>#facade.<bytes>` producing a "side-definition with descriptive text". Instance: `(Void#top.hat.(Thief.sd.0-0:0-0:157-0.i.Blindfold.i.k.descend.i.top.facade.bas157:0:-20:0).n.Whistle.img.<bytes>.tier.2)` (pansaer pool 3). Also many in pansaer: `(Void#rightmost.hat.(Thief.sd.0-0:0-0:0-0:0-0:0-0:103-1.i.k.vitality.i.k.bloodlust.i.rightmost.facade.eba16:0).n.Condensed Milk.img.<bytes>.tier.2)`. Shape is a Template-Thief with dice faces + a chain of `.i.` modifiers + a sidesc and/or facade.

**V15 — LearnSpellInline.** Entry is a `learn.<SpellName>.n.<ItemName>.img.<bytes>.tier.<n>` shape. Instance: `(learn.Poke.n.Learn Poke.img.Poke.tier.2)` (pansaer pool 3). Also: `(learn.Bandage.n.Learn Bandage.img.Bandage.tier.2)` (pansaer pool 3).

**V16 — Remaining / Unclassified.** Any entry whose bytes the Phase-A walker cannot fit into V1–V15 even after exhaustive inspection. Per SPEC §3.3 permissive extract, the extractor must NOT fail on an unrecognized shape: it emits a `Finding { code: "W-REPLICA-NONSUMMON-UNCLASSIFIED", severity: Severity::Warning, modifier_index, preview: <120 bytes of the entry> }` and demotes the entry to a **new** variant `NonSummonEntry::Unclassified { source_bytes: String }` that preserves bytes for round-trip. **CRITICAL**: `Unclassified` is NOT an "escape hatch alongside other variants"; it is the sole remaining raw-byte surface and exists so that first-ship of 8A.5 can classify every shape it has evidence for without blocking on edge cases. Every `Unclassified` occurrence at roundtrip time is a **tracked defect** that must be retired in a follow-up (§3.5 enumerates the tracking protocol and the retirement gate). At plan-time, if any test in §5 produces `Unclassified` for any shape named in V1–V15 above, the classifier is buggy — do not widen the tolerance; fix the classifier. An alternative IR framing that genuinely has no string field (a separate `StructuralContent::UnclassifiedItempoolEntry` variant) is rejected in §3.1 Design decision D-C below — it moves the byte-string one container up the tree without retiring it.

**Numbering note.** V1–V15 are the typed variants; V16 is the permissive-extract pressure-valve. The §3.1 enum prose must name the typed variants once (by name, not by re-listing), and treat `Unclassified` as its own documented exception. Hook rule #2 (canonical-set re-listing): name the typed set once, reference by name thereafter.

---

## 2. Pre-conditions (verify before any 8A.5 code lands)

All must be true on `main` at 8A.5 start.

1. **8A shipped.** Parent §9.1 verification gate passes. Specifically:
   ```
   rg -n 'pub enum SummonTrigger\|pub enum DiceLocation\|pub enum ItempoolItem' compiler/src/ir/mod.rs  # 3 hits
   rg -n 'pub struct ItempoolExtraction' compiler/src/extractor/replica_item_parser.rs                   # 1 hit
   rg -c 'ItemPoolEntry\b' compiler/                                                                    # 0
   rg -c 'ReplicaItemContainer\b' compiler/                                                             # 0
   rg -n 'NonSummon\s*\{\s*name' compiler/src/ir/mod.rs                                                 # 1 hit (transitional shape 8A.5 retires)
   cargo build                                                                                          # clean
   cargo test                                                                                           # clean
   cargo run --example roundtrip_diag                                                                   # all four mods Status: ROUNDTRIP OK; Replicas ir1=0
   ```
   If any grep disagrees with its expectation, 8A is incomplete — halt, finish 8A, do not start 8A.5.

2. **Chunks 5, 7, 9 on `main`** (parent §2 hard gate; 8A's §2 restates it; 8A.5 inherits).
   ```
   rg -c 'slice_before_chain_and_cast' compiler/src/util.rs               # ≥1
   rg -c 'fn generate_hero_item_pool'  compiler/src/builder/derived.rs    # ≥1 (Chunk 5 OR 8A authored)
   ```

3. **8B NOT started.** 8B depends on 8A.5; if 8B already shipped raw-string NonSummon entries to `main`, the SPEC §3.2 violation is already loose in the tree and the work order is wrong. Halt and reconcile with user before continuing. Concrete grep:
   ```
   rg -c 'NonSummonEntry\b' compiler/src/ir/mod.rs     # 0 — 8A.5 authors this type
   ```

4. **(A5-5 fix) `Unclassified` ship gate (ratchet).** The `NonSummonEntry::Unclassified { source_bytes }` variant is retained as a SPEC §3.3 permissive-extract
   pressure-valve, but must never fire on corpus bytes. If the Phase-A walk produces ANY
   `Unclassified` entry across the four working mods, the variant set MUST be widened IN
   THE SAME PR — not deferred to a follow-up. The ratchet is enforced by the `T30.0`
   budget test (§5.4; predicate `unclassified_count == 0`, no tolerance) that runs in
   `cargo test`, AND by the explicit CI gate grep:
   ```
   cargo test --test roundtrip_baseline working_mods_produce_zero_unclassified_entries  # must pass
   ```
   The variant is allowed to EXIST so the extractor cannot panic on a novel shape;
   it is forbidden to FIRE on the current corpus. If a future corpus shape surfaces
   `Unclassified`, that is the ratchet signal to widen §1.3 + §3.1 in the same PR.

Halt condition: if ANY pre-condition fails, stop. Do not weaken 8A's or Chunk 5/7/9's landed state to unblock 8A.5.

---

## 3. What ships

### 3.1 IR schema (`compiler/src/ir/mod.rs`)

Replace 8A's `ItempoolItem::NonSummon { name: String, tier: Option<i8>, content: String }` with:

```
pub enum ItempoolItem {
    /// Index into `ModIR.replica_items` (summon path, shipped in 8A; 8B populates).
    Summon(usize),
    /// Non-summon itempool entry. Typed recursive schema — see `NonSummonEntry`.
    NonSummon(NonSummonEntry),
}

/// Typed non-summon itempool entry. Every byte of every entry in every
/// corpus itempool body maps into one of the variants below; there is no
/// `content: String` escape. The `trailer: NonSummonTrailer` field is
/// shared across all variants (per §1.3 "every non-summon entry carries
/// a `.n.<name>.tier.<n>.…` trailer at depth 0, outside any inner parens").
pub enum NonSummonEntry {
    /// V1 §1.3. Bare base-game item name. Pool-membership only. No trailer
    /// (base-game refs do not rename themselves). `name` is the verbatim
    /// base-game item identifier bytes (case-preserving; no registry
    /// lookup — see §3.2.3 "no registry reach").
    BareBaseGameRef { name: String },

    /// V2+V3 §1.3. Composition of `#`-joined components (the "body") plus
    /// the outer trailer. Presence or absence of an outer `(…)` wrap is
    /// captured by `outer_wrap: Option<OuterWrap>` (see below). The typed
    /// body is a non-empty Vec; a single-component body is still a typed
    /// vector of length 1 so V2 and V3 share the same shape at this level.
    Composition {
        outer_wrap: OuterWrap,
        components: Vec<CompositionComponent>,
        trailer: NonSummonTrailer,
    },

    /// V4+V5+V7 §1.3. A `ritemx.<hex>` reference, optionally with `.part.<n>`,
    /// `.splice.<body>`, `.m.<mult>`, and an accessory chain. Distinct from
    /// `Composition` because the typed `ritemx_ref: RitemxRef` head carries
    /// structure (hex, part, splice-target) that `CompositionComponent`
    /// alone does not; keeping it as a dedicated variant avoids a
    /// `CompositionComponent::Ritemx { … }` + parallel-ish wiring on every
    /// emitter branch. V7 (multi-ritemx with rename) lives in `Composition`
    /// because every component is itself a `CompositionComponent::Ritemx`
    /// — see `CompositionComponent` below.
    Ritemx {
        outer_wrap: OuterWrap,
        head: RitemxRef,
        tail_chain: Vec<CompositionComponent>,  // `#`-joined trailing accessories; empty for plain ritemx
        trailer: NonSummonTrailer,
    },

    // V8+V15 §1.3 (A5-1 fix): collapsed at the `NonSummonEntry` TOP LEVEL only.
    // There is NO top-level `NonSummonEntry::Keyword` or `NonSummonEntry::Learn`
    // variant here. Inside a `Composition`, `CompositionComponent::Keyword` and
    // `CompositionComponent::Learn` ARE declared below — they are the sub-
    // component types that represent the bare-head case when
    // `components.len() == 1`. Design decision D-D formalizes the entry-level
    // collapse, not a component-level deletion. This comment stands in place of
    // the (deleted) top-level variant; the component variants are authoritative.

    /// V9 §1.3. A `t.jinx.<body>` / `t.<name>.<body>` template-jinx composition.
    /// Distinct from Composition because the leading `t.<Template>.` is a typed
    /// template reference, not a generic `#`-component.
    TemplateJinx {
        outer_wrap: OuterWrap,
        template: TemplateRef,
        body: Box<NonSummonEntry>,   // recursive: jinx body is itself a typed entry
        trailer: NonSummonTrailer,
    },

    /// V10 §1.3. Inline `abilitydata.(…)` definition (non-summon; the Cast
    /// summon shape is 8B's `SummonTrigger::Cast`).
    InlineAbility {
        outer_wrap: OuterWrap,
        template: TemplateRef,            // `zm`, `thief`, `mage`, etc.
        ability: AbilityBody,             // dice + sides + sidescs + img, typed
        tail_chain: Vec<CompositionComponent>,  // `#`-joined post-ability accessories
        trailer: NonSummonTrailer,
    },

    /// V11 §1.3. `self.<scope>.allitem.<body>.…` recursive composition. The
    /// inner body may itself contain nested entries with their own trailers;
    /// IR preserves every depth-0 `.n.<name>` of the source bytes.
    AllitemComposition {
        outer_wrap: OuterWrap,
        scope: AllitemScope,              // typed: `self`, `self.t1`, `self.et2`, etc.
        body: Box<NonSummonEntry>,        // recursive
        tail_chain: Vec<CompositionComponent>,
        trailer: NonSummonTrailer,
    },

    /// V12 §1.3. Entry rooted at a bare sentinel/anchor token (`Void`, `uy`,
    /// etc.). Covered by `Composition` when `#`-composed with other content;
    /// split only when the entry is ONLY the sentinel (e.g. sliceymon's
    /// outer `itempool.Void.part.0.mn.Clear Itempool` which has no paren
    /// body). `Void.part.0` alone with `.mn.<name>` trailer is NOT an
    /// itempool ENTRY — it is an itempool MODIFIER whose `items: Vec`
    /// is empty; see §3.3's `StructuralContent::ItemPool { items,
    /// outer_name: Option<String> }` note. A bare `Void` or `uy` as ENTRY
    /// classifies under `Composition` (single-component), so no variant
    /// needed here either.

    /// V14 §1.3. Template-thief side-definition composition with sidesc/facade.
    /// Single-purpose variant because the structure (template + dice faces
    /// + i-chain + sidesc + facade) is rich enough that folding it into
    /// `Composition` would lose type structure at the emitter.
    SideDefWithSidesc {
        outer_wrap: OuterWrap,
        template: TemplateRef,
        dice: DiceFaces,
        i_chain: Vec<CompositionComponent>,
        sidesc: Option<String>,      // `#sidesc.<text>#` payload
        facade: Option<FacadeRef>,   // `#facade.<bytes>#` payload
        trailer: NonSummonTrailer,
    },

    /// V16 §1.3. Permissive-extract pressure-valve. Represents an entry whose
    /// bytes the classifier did not recognize. Paired with a `Finding`
    /// (`W-REPLICA-NONSUMMON-UNCLASSIFIED`). **Every `Unclassified` occurrence
    /// at round-trip time is a tracked defect** — see §3.5. The `source_bytes`
    /// field is the DEFECT HATCH, not a general escape.
    Unclassified { source_bytes: String },
}

/// Whether the entry is paren-wrapped at depth 0. Captures source-byte
/// fidelity: `(X#Y).n.Z.tier.n` round-trips differently from `X#Y.n.Z.tier.n`.
pub enum OuterWrap {
    Unwrapped,
    SingleParen,          // `(X)…`
    DoubleParen,          // `((X)).n.<…>` — trailer OUTSIDE inner paren but INSIDE outer paren.
                          // Verified 2026-04-24 via `rg '\(\([^)]*\)\)\.n\.' working-mods/*.txt | wc -l` → 8 hits.
    // A5-4 fix: DoubleParenTrailerInside is CONDITIONAL on Phase-A walker finding
    // a clear itempool-entry instance of `((X.n.<…>))` (trailer inside INNERMOST paren,
    // NOT reachable by depth-climbing past `))`). If the walker returns zero such
    // entries at impl start, DELETE the variant in the same commit (hypothesis
    // masquerading as model, per chunk-impl rule 3). The preliminary grep
    // `rg '\(\([^)]*\.n\.[^)]*\)\)' working-mods/*.txt` returned hits, but most
    // resolve to deeper-nested `((...(...n....)))` shapes, not true
    // "double-paren with depth-0 trailer inside". Phase-A walk is the arbiter.
    // DoubleParenTrailerInside,
}

/// A single `#`-joined component inside a Composition body or tail_chain.
pub enum CompositionComponent {
    /// Bare base-game item reference (no modifier chain).
    BaseGameRef { name: String },
    /// Bare keyword (`k.first`, `k.treble`, `k.rainbow`, `k.pristeel`, etc.)
    /// with optional `.<modifier>` suffixes.
    Keyword { keyword: String, suffixes: Vec<String> },
    /// `learn.<SpellName>` — keyword that learns a spell.
    Learn { spell: String },
    /// `ritemx.<hex>.[part.<n>][.splice.<…>][.m.<mult>]` component.
    Ritemx(RitemxRef),
    /// `hat.<Template>.sd.<faces>.i.<chain>#facade.<…>` side-definition chain.
    SideDef {
        template: TemplateRef,
        dice: Option<DiceFaces>,
        i_chain: Vec<CompositionComponent>,
        sidesc: Option<String>,
        facade: Option<FacadeRef>,
    },
    /// Scope-prefixed composition — `self.<scope>.<body>` / `right2.<body>` /
    /// `top.<body>` / etc. Typed scope head, recursive body.
    Scoped {
        scope: ScopeSpec,
        body: Box<CompositionComponent>,
    },
    /// Paren-wrapped sub-composition — recursive.
    Nested(Box<NonSummonEntry>),
    /// Template-jinx component: `t.jinx.<body>`.
    Jinx { body: Box<NonSummonEntry> },
    /// `t.<Template>.abilitydata.(…)` inline ability body.
    Ability { template: TemplateRef, body: AbilityBody },
    /// `t.vase.<…>` template-vase insertion (add/replace).
    Vase(VaseOp),
    /// Bare sentinel — `Void`, `uy`, etc.
    Sentinel { token: String },
    /// Numeric multiplier / HSV / hue / hsl / tier / doc / img / draw /
    /// part suffix attached to a preceding token. Represented as a typed
    /// suffix on the preceding component via `Keyword.suffixes` or
    /// `Ritemx.part` — this variant is NOT a top-level component.
    /// (Kept as a prose note; no enum case.)
}

pub struct RitemxRef {
    pub hex: String,                             // "ritemx.1768a" → "1768a"
    pub prefix: Option<RitemxPrefix>,            // "unpack.", etc.
    pub part: Option<u8>,                        // ".part.<n>"
    pub splice: Option<Box<NonSummonEntry>>,     // ".splice.<body>" — recursive typed splice target
    pub multiplier: Option<i8>,                  // ".m.<n>"
}

pub enum RitemxPrefix { Unpack, /* enumerate only observed prefixes in §3.2.1 */ }

pub struct TemplateRef { pub name: String }   // source-byte: "jinx", "Lost", "zm", "Boar", …

pub enum AllitemScope {
    Self_,                // `self.allitem`
    SelfT1,               // `self.t1.allitem`
    SelfEt2,              // `self.et2.allitem`
    // A5-7 fix: `Other(String)` DELETED. An exhaustive Phase-A walk is a
    // prerequisite for ship — if the walker surfaces a scope token not enumerated
    // here, halt and widen this enum in the same PR before any classifier code
    // is authored. A raw-string escape here contradicts the atomic-rewrite
    // principle (either the walk is exhaustive → variants are dead code, or the
    // variant is a raw-string escape → SPEC §3.2 violation). Per D-E below,
    // corpus surprise widens the enum; it never silently absorbs as string.
}

pub enum ScopeSpec {
    Self_, SelfT1, SelfT2, SelfEt2, Left, Right, Right2, Right3, Right5,
    Top, Bot, Mid, Mid2, Mid4, Left2, Topbot, Row, Col, Rightmost,
    // A5-7 fix: `Other(String)` DELETED. Same "widen on corpus surprise" contract
    // as AllitemScope above — Phase-A walk is authoritative; no string-escape.
}

pub struct AbilityBody {
    pub template: TemplateRef,                  // inner `(thief.sd.…` / `(zm.sd.…` / `(statue.sd.…` template
    pub dice: DiceFaces,
    pub i_chain: Vec<CompositionComponent>,     // `.i.<…>` chain
    pub ability_name: Option<String>,           // `.n.<AbilityName>` inside the abilitydata body
    pub img: Option<ImgPayload>,
    pub sidescs: Vec<String>,                   // `#sidesc.<text>#` sub-entries
    pub facade: Option<FacadeRef>,
}

pub enum VaseOp {
    Add    { target: Box<NonSummonEntry> },    // `t.vase.(add.(…))`
    Ch     { target: String },                 // `t.vase.ch.<target>.mn.<name>`
    Other  { op: String, body: String },       // widen on surprise
}

pub struct FacadeRef { pub name: String, pub params: Vec<i16> }  // "Bal35:0" → name "Bal35", params [0]; "Che22:11" → name "Che22", params [11]

/// Shared outer trailer: `.n.<name>.tier.<n>.img.<…>.doc.<…>.hue.<n>` etc.
/// `name` is the ONLY always-present field; the trailer is the ".n." that
/// names the non-summon entry inside its pool.
pub struct NonSummonTrailer {
    pub name: String,                   // `.n.<bytes>` (case-preserving; space-preserving — see community `.n. .` entry for Wurst/4)
    pub tier: Option<i8>,               // `.tier.<n>` — negative tiers observed (pansaer pool 1 has `-3`, `-2`)
    pub mod_tier: Option<i8>,           // `.modtier.<n>` (community line 2 `itempool.uy.part.0.mn.NOItems…modtier.-1`)
    pub img: Option<ImgPayload>,        // `.img.<…>` — typed (§ImgPayload below)
    pub doc: Option<String>,            // `.doc.<raw bytes>` — doc payload is raw text with textmod escape sequences (`[n]`, `[grey]`, etc.); `String` here is source-preserving, NOT a typed breakdown of the escapes (that is out of scope and not a SPEC §3.2 violation — doc is human-readable flavor, not IR invariant-bearing structure)
    pub hue: Option<i16>,
    pub hsv: Option<(i16, i16, i16)>,
    pub hsl: Option<(i16, i16, i16)>,
    pub b: Vec<String>,                 // `.b.<hex>` × N (multiple `.b.` suffixes observed: community line 52 `(all.k.growth#k.decay.img.Fly.b.FFF.b.0F0.b.00F.b.F00)`)
    pub p: Option<PPayload>,            // `.p.<fg>:<bg>:<alpha>` (community line 22 `.p.fff:fd3:60`)
    pub draw: Vec<DrawOp>,              // `.draw.<source>:<x>:<y>` × N (community line 88 has two `.draw.` sites)
    pub rect: Option<String>,           // `.rect.<bytes>`
    pub speech: Option<String>,         // `.speech.<text>`
    pub speech_alt: Option<String>,     // rare dual-speech — extend if corpus demands
    pub mn: Option<String>,             // `.mn.<name>` — pool-outer "monster name" / "pool name" marker
    pub suffixes_other: Vec<TrailerSuffix>,  // NOT a byte-string: typed enum of every suffix key we observe; widen on surprise
}

pub enum TrailerSuffix {
    Part(u8),
    Hidden,        // `&Hidden` (pansaer pool 1 tail `…&Hidden.mn.Tier 0 and Lower Items`)
    Temporary,
    ModTier(i8),
    /* widen as Phase-A walk surfaces new keys */
}

pub enum ImgPayload {
    Raw { bytes: String },                 // `.img.332648…` — base64-like raw pixel bytes
    NamedRef { name: String, transforms: Vec<ImgTransform> },  // `.img.Relic.hue.12` / `.img.Stream.hsv.20:0:-12`
}

pub enum ImgTransform {
    Hue(i16),
    Hsv(i16, i16, i16),
    Hsl(i16, i16, i16),
    DrawOverlay(DrawOp),
    /* widen on corpus */
}

pub struct DrawOp { pub source: ImgRefOrRaw, pub x: i16, pub y: i16 }
pub enum ImgRefOrRaw { Ref(String), Raw(String) }

pub struct PPayload { pub fg: String, pub bg: String, pub alpha: i16 }
```

**Design decisions (load-bearing — record on the PR body for review).**

- **D-A.** V2 (unwrapped composition) and V3 (paren-wrapped) collapse into a single `Composition` variant discriminated by `OuterWrap`, because the body shape is byte-identical; only the source's paren framing differs. Keeping them as two enum variants would force emitter branching at every site without gaining any invariant.
- **D-B.** `(ritemx.a348).tier.0.n.Seeing Red.…` — paren wraps the ritemx but trailer is outside. Modeled by `Ritemx { outer_wrap: SingleParen, head: RitemxRef { hex: "a348", … }, tail_chain: [], trailer: NonSummonTrailer { name: "Seeing Red", tier: Some(0), … } }`. The `outer_wrap` field disambiguates the paren framing; `tail_chain` being empty means no `#`-composed accessories after the ritemx; `trailer` is parsed from the post-paren bytes.
- **D-C.** `NonSummonEntry::Unclassified { source_bytes: String }` is the SOLE raw-byte field in the whole 8A.5 schema. It is NOT an "alongside escape hatch" — it is a paired variant with a **mandatory** `Finding` and a **tracked-defect retirement protocol** (§3.5). The alternative framing (`StructuralContent::UnclassifiedItempoolEntry`) is rejected because it moves the byte-string one container up and destroys the `+`-order invariant on the typed `items: Vec<ItempoolItem>`.
- **D-D.** Bare-keyword entries (V8) and bare-learn entries (V15) collapse into `Composition` with a single-component body (`Keyword` or `Learn`). A dedicated variant per head-token yields no new invariants.
- **D-E.** `TrailerSuffix::Other(String)` is **explicitly rejected**. If Phase-A walk surfaces a suffix key not in the enum, **widen the enum in the same commit** — do not silently absorb as a string (SPEC §3.2). The widening-on-surprise rule is the whole-plan invariant; every `Option<X>` / `Vec<Y>` / enum variant is widened on corpus surprise, never the `Unclassified` hatch.

### 3.2 Extractor classifier (`compiler/src/extractor/replica_item_parser.rs`)

#### 3.2.1 Phase-A exhaustive corpus walk (MUST run at impl start)

Before authoring any classifier code, run a script (shipped as `compiler/examples/itempool_entry_shapes.rs` — a new file that **is** expected for this chunk's §4 file list) that:

1. Reads every `working-mods/*.txt`.
2. Parses every `itempool.` / `Itempool.` / `!mitempool.` modifier via the classifier's current `ItemPool`-route detection.
3. For each modifier, locates the body opening (first `(` after `itempool.`), then splits on `+` at paren-depth 0.
4. For each `+`-split entry, emits a **shape-fingerprint**: a list of salient structural tokens seen at depth 0 or depth 1 (leading `(`, leading `ritemx.`, leading `t.jinx.`, leading `self.`, leading `k.`, leading `hat.`, leading `learn.`, presence of `.splice.`, presence of `.abilitydata.`, presence of `.n.` at depth 0, presence of `.mn.` at depth 0, trailing `.tier.<n>`, trailing `.img.<bytes>`/`.img.<name>`, etc.).
5. Groups entries by fingerprint; writes a per-fingerprint count + ≥3 verbatim corpus samples to `target/itempool-shape-audit.txt`.

**(A5-8 fix) Concrete output format** — `target/itempool-shape-audit.txt`. One block per distinct fingerprint, blocks separated by a line of three hyphens `---`:

```text
FINGERPRINT: <salient-token-list, order-preserved; e.g. "leading-paren, ritemx., .part.<n>, .n., .img.<name>, .tier.<n>, trailing-close-paren">
COUNT: <N>
VARIANT_MAPPING: <V-N label, e.g. V4 RitemxRef>  |  UNMAPPED
SAMPLES:
  1. <up to 120-char verbatim entry preview with ellipsis if truncated>
  2. <up to 120-char verbatim entry preview>
  3. <up to 120-char verbatim entry preview>
---
```

**Halt rule** (encodes ratchet, not tolerance): if ANY block's `VARIANT_MAPPING:` reads `UNMAPPED`, halt. Extend §1.3 with a new V-N entry (with ≥1 verbatim corpus quote) and extend §3.1 with the new enum variant, in the SAME PR. Re-run the walker. Loop until zero `UNMAPPED`. The walker's exit code is non-zero while any fingerprint is `UNMAPPED` so `cargo run --example itempool_entry_shapes` integrates cleanly into CI gating.

6. Implementation reads the audit output and cross-checks against §1.3's V1–V15. Any fingerprint with `UNMAPPED` is a plan defect — halt, fix §1.3 per the rule above, then return to implementation.

The Phase-A walker is **not a throwaway**. It is retained as `compiler/examples/itempool_entry_shapes.rs` so §F-layer corpus audits (`plans/PLATFORM_FOUNDATIONS_PLAN.md` §F9 "evidenced variants" rule) can re-run it whenever a new working-mod lands.

#### 3.2.2 Classifier wire-up

In `extract_from_itempool` (which in 8A returned `Ok(ItempoolExtraction { new_replica_items: vec![], items: vec![NonSummon { content: body }] })` as a stub), replace the stub with a **two-stage dispatch**: summon prefilter first, non-summon classifier on rejection. Routing lives in the return-variant (`Option<…>`), NOT the error channel.

1. Split the body on `+` at paren-depth 0 (new helper: `util::split_itempool_entries(body: &str) -> Vec<&str>`; see §3.2.3 for the helper's contract). 8B reuses this helper unchanged.

2. **Summon prefilter contract (shipped by 8A.5, swapped by 8B).** 8A.5 ships the symbol + signature + a trivial stub body; 8B replaces the body only.

   ```rust
   pub struct SummonClassification {
       pub replica_item: ReplicaItem,
       // 8B widens this struct as the prefilter populates it.
   }

   pub fn classify_summon_entry(
       entry: &str,
       modifier_index: usize,
   ) -> Result<Option<SummonClassification>, CompilerError> {
       // 8A.5 stub: prefilter not yet wired. 8B replaces the body.
       Ok(None)
   }
   ```

   Pinning this contract in 8A.5 removes the impl-time negotiation 8B would otherwise carry. 8B swaps the body; no signature change, no call-site change.

3. **Per-entry dispatch loop.** For each `+`-split entry:
   - Call `classify_summon_entry(entry, modifier_index)`:
     - `Ok(Some(SummonClassification { replica_item, .. }))` → push `replica_item` onto `new_replica_items` and push `ItempoolItem::Summon(i)` onto `items` where `i = next_replica_index + new_replica_items.len() - 1`.
     - `Ok(None)` → delegate to `classify_non_summon_entry` (step 4).
     - `Err(_)` → propagate. This is the strict `CompilerError::classify` path for genuine prefilter failures (e.g. a summon-shaped pair whose wrapper fails every known template — parent §3.3 rule 3's `W-REPLICA-TRIGGER-UNCLASSIFIED` path, owned by 8B). Not a soft demote.

4. **Non-summon classifier** (`classify_non_summon_entry`):

   ```rust
   pub fn classify_non_summon_entry(
       entry: &str,
       modifier_index: usize,
   ) -> Result<NonSummonEntry, CompilerError>;
   ```

   - Typed-classify into V1–V15 per §1.3. Every classification step is a single structural check (e.g. "starts with `ritemx.` at depth 0"); NO surface-name lookup, NO registry reach, NO case normalization (§3.2.3 details).
   - **This function does NOT inspect summon shape.** The prefilter already handled that. Summon-shaped bytes that reach this function (only possible when the prefilter returned `Ok(None)` — i.e. never in 8A.5, and only on real misclassification after 8B ships) get classified as whatever V1–V15 fingerprint matches, or fall through to `Unclassified`. A summon-pair that fools the prefilter is not an 8A.5 concern; it's a 8B prefilter-regression (see 8B §7 risks).
   - If no V1–V15 variant fits, emit `Finding { code: "W-REPLICA-NONSUMMON-UNCLASSIFIED", severity: Warning, modifier_index, preview: <120 bytes> }` (Finding emission threads through the `Report` in scope at the caller; impl-time detail, not a plan boundary) and return `Ok(NonSummonEntry::Unclassified { source_bytes: entry.to_string() })`.

5. Populate the returned `ItempoolExtraction.items` as `Vec<ItempoolItem>` in source order.

6. `ItempoolExtraction.new_replica_items` stays `vec![]` in 8A.5 because the stub always returns `Ok(None)`. 8B populates when the prefilter body becomes real.

**Why two-stage dispatch, not error-as-routing.** The prefilter and the non-summon classifier answer different questions: *"is this a summon?"* vs. *"what typed non-summon shape is this?"*. Using `Err(CompilerError::summon_reroute)` from the non-summon classifier to signal "call the other classifier" collapses routing into the error channel and trips hook rule #6 (two paths with different invariants — permissive typing vs. strict routing — sharing one return surface). The two-stage dispatch keeps `Err` for real failures and `Option` for routing.

#### 3.2.3 No registry reach (hook rule #2 source-vs-IR guard)

The classifier MUST NOT:
- Look up an item name in any base-game item registry / Pokemon registry / keyword registry to decide V1 (BareBaseGameRef). V1 membership is "the `+`-split entry is a single token at depth 0 (no `.`, no `#`, no `(`) with no trailer"; the `name` field is the verbatim bytes.
- Apply case normalization anywhere (per parent §3.2 "source-byte preserving" contract).
- Reach for `SpriteId` / face-compat / any derived lookup table.
- Split on `.` to detect `.tier.`/`.img.`/`.n.` — these splits must be paren-depth-aware (`util::slice_before_chain_and_cast` from Chunk 9, OR a local helper if the 8A.5 classifier needs a different discipline; document at impl time which helper each classifier-stage uses).

Every test in §5 includes at least one source-vs-IR divergence assertion: synthetic bytes that, if the classifier reached for a derived/registry source instead of source bytes, would be classified wrong and a round-trip would silently preserve the wrong bytes. Hook rule #2 is enforced per-variant, not once at the end.

### 3.3 Emitter (`compiler/src/builder/replica_item_emitter.rs` + `compiler/src/builder/structural_emitter.rs`)

`emit_itempool` (shipped in 8A against `StructuralContent::ItemPool { items }`) already iterates `items` in source order; 8A.5's change is the inner `emit_non_summon_entry(entry: &NonSummonEntry) -> String` function. One private helper per variant, NO duplicated incantations across variants (hook rule #3).

Shared helpers:
- `emit_trailer(t: &NonSummonTrailer) -> String` — `.n.<name>[.tier.<n>][.img.<…>][.doc.<…>][.hue.<n>][.hsv.<…>]…&<suffix_keys>.mn.<mn>` in source order. Source-order is captured at extraction time by preserving suffix order in `suffixes_other: Vec<TrailerSuffix>`; non-order-sensitive fields emit in a canonical order that matches the corpus majority order (document at impl time which corpus entry fixed the canonical order).
- `emit_composition_component(c: &CompositionComponent) -> String` — one match arm per variant.
- `emit_img(i: &ImgPayload) -> String` — `.img.<raw>` vs `.img.<name>.<transforms>`.
- `emit_ritemx(r: &RitemxRef) -> String` — `[<prefix>.]ritemx.<hex>[.part.<n>][.splice.<emit_non_summon_entry(target)>][.m.<mult>]`.

Every `NonSummonEntry` variant has a single emit arm. The `Unclassified { source_bytes }` variant emits `source_bytes` verbatim — the only string-pass-through in the emitter, paired with the matching `Finding` at extract time so the defect is visible.

`structural_emitter.rs`'s `StructuralContent::ItemPool { items, .. }` arm calls `emit_itempool`; that wiring is from 8A and unchanged in 8A.5.

### 3.4 Callsite migration — every site that reads `NonSummon.content`

At 8A ship time, `ItempoolItem::NonSummon { content: String }` is read at:
- `compiler/src/builder/replica_item_emitter.rs` — `emit_itempool`'s NonSummon branch.
- `compiler/src/xref.rs` — **verify at impl start**: the 8A.5 agent greps `rg -n 'NonSummon' compiler/src/` before editing. Any xref check that currently reads `content: String` for duplicate-name detection, name-collision bucketing, or tier comparison MUST be rewritten to read the typed `NonSummonTrailer.name` / `NonSummonTrailer.tier`. If any xref rule reaches into the `content` bytes for a downstream decision, note the rule in the PR body — that is ALSO a SPEC §3.2 violation that 8A.5 closes.
- `compiler/src/ir/ops.rs` — any CRUD op that searches by name. Rewrite to read `NonSummonTrailer.name`.
- `compiler/src/authoring/*.rs` — any builder API that constructs an ItempoolItem. Rewrite to construct typed `NonSummonEntry` variants via a builder module (§3.4.1 below).

#### 3.4.1 Authoring builders (`compiler/src/authoring/replica_item.rs` or a new `compiler/src/authoring/non_summon_entry.rs`)

The authoring layer must expose typed builders — `bare_base_game_ref(name)`, `composition(outer_wrap, components, trailer)`, `ritemx(head, tail_chain, trailer)`, `template_jinx(template, body, trailer)`, etc. — so web/mobile frontends can construct non-summon entries without touching source bytes. Per `MEMORY.md` "Compiler is a mod-building backend": every IR shape has a typed authoring door.

`Unclassified` has NO authoring builder — it is not constructible via the authoring API; only the extractor emits it (and only when paired with a Finding). The authoring gate is the SPEC §3.3 "strict construction" enforcement point; extract is permissive, author is strict.

### 3.5 `Unclassified`-retirement protocol (tracked-defect gate)

`NonSummonEntry::Unclassified` is the only raw-byte field in 8A.5's IR. It exists because first-ship cannot guarantee zero corpus shapes are missed at Phase-A walk time. The retirement gate:

1. **Round-trip test (T30 in §5)** runs `cargo run --example itempool_entry_shapes` against all four working mods at `cargo test` time. Any entry classified as `Unclassified` in the working mods CAUSES THE TEST TO FAIL. The test failure message includes the 120-byte preview so the maintainer can add the shape to §1.3 as a new V-N variant, extend the classifier, and the failure disappears.
2. **No user-facing path ever accepts an `Unclassified` entry as "fine"**: the variant is `#[serde(serialize_with = "ser_unclassified_warn")]` — serialization emits a `W-REPLICA-NONSUMMON-UNCLASSIFIED` log-line to stderr at every encounter. This makes the defect hot, not silent.
3. **T30's budget is ZERO**. It does not accept "up to N unclassified entries" — the budget is a ratchet: if 8A.5 ships with T30 at zero and later a new working mod adds a shape, the ratchet forces a same-PR widening. Hook rule #4 (rules must encode the reason a residual is permitted, not just the identity): T30's pass predicate is `unclassified_count == 0`, not `unclassified_count <= N`.
4. **The `Unclassified` variant does NOT receive an authoring builder** (§3.4.1). New content cannot be authored into it; only extract may produce it.

---

## 4. Files touched (defense if >5)

| # | File | Change |
|---|---|---|
| 1 | `compiler/src/ir/mod.rs` | Replace `ItempoolItem::NonSummon { name, tier, content }` with `NonSummon(NonSummonEntry)`. Add all typed structs/enums per §3.1. |
| 2 | `compiler/src/extractor/replica_item_parser.rs` | Replace 8A stub NonSummon with `classify_non_summon_entry` + V1–V15 classifiers. |
| 3 | `compiler/src/extractor/mod.rs` | Behavioral change in the `ModifierType::ItemPool` arm (already wired in 8A); no signature change. |
| 4 | `compiler/src/builder/replica_item_emitter.rs` | `emit_non_summon_entry` + `emit_trailer` + `emit_composition_component` + `emit_img` + `emit_ritemx`. |
| 5 | `compiler/src/builder/structural_emitter.rs` | Already dispatches to `emit_itempool`; verify no ambient `body.clone()` survives. Delete any leftover 8A transitional code. |
| 6 | `compiler/src/xref.rs` | Callsite migration (§3.4) — rewrite any `NonSummon.content` read to typed `NonSummonTrailer` access. |
| 7 | `compiler/src/ir/ops.rs` | CRUD ops rewrite (§3.4). |
| 8 | `compiler/src/authoring/non_summon_entry.rs` (NEW) | Typed authoring builders per §3.4.1. |
| 9 | `compiler/src/util.rs` | Add `split_itempool_entries` helper (paren-depth-0 `+`-split) if it does not already exist post-8A. Verify by `rg -n 'fn split_itempool_entries\|fn split_at_depth0' compiler/src/util.rs` at impl start; use the existing `split_at_depth0` if its contract matches. |
| 10 | `compiler/examples/itempool_entry_shapes.rs` (NEW) | Phase-A corpus walker (§3.2.1). Committed as tracked tooling, not temporary script. |
| 11 | `compiler/tests/retirements_tests.rs` | New grep retirements: `rg -c 'NonSummon\s*\{\s*content\s*:\s*String' compiler/` must be 0 post-ship. |
| 12 | `compiler/tests/integration_tests.rs` | New tests T30.1–T30.15 (per-variant corpus round-trip, one per shipped V-N variant). |
| 13 | `compiler/tests/roundtrip_baseline.rs` | Regenerate baselines — NonSummon entry counts per mod become non-zero (previously bundled into a single `content: String`). |

**Size deviation — 13 files, over the 5-file soft cap.** Defense: 8A.5 is a **representation change**; every reader of `NonSummon` must migrate in the same commit because the field is retired, not renamed. Parent plan §9.1 applies the same spirit-not-letter rule for 8A; 8A.5 inherits the reasoning. A partial landing (retype IR only, callers still read `content: String`) does not compile. The 5-file cap is a heuristic, not an invariant; the invariant is "one atomic compiling commit" (per `CLAUDE.md` "No deferred correctness"), which is satisfied. The alternative (ship `content: String` + new typed field in parallel, migrate callers over time) is exactly the parallel-representation pattern the repo forbids.

The two NEW files (the authoring builder + the Phase-A walker) are defensible additions, not accidental scope creep:
- `authoring/non_summon_entry.rs` is the web/mobile frontend's door to the typed IR (per `MEMORY.md` "Compiler is a mod-building backend"). Without it the authoring API cannot construct non-summon entries at all post-retirement.
- `examples/itempool_entry_shapes.rs` is the retained corpus audit tool — it runs at `cargo test` for T30 (retirement-budget) and is the evidence-gathering tool for future mod additions. Deleting it after first-ship would shift the evidentiary burden to ad-hoc greps on every future chunk.

---

## 5. Enumerated tests

TDD — write each test before authoring the code it exercises. Every test has a source-byte corpus anchor (hook rule #2).

### 5.1 Per-variant round-trip (T30.1–T30.15)

Format: `#[test] fn non_summon_<variant>_roundtrips_byte_equal()`. Input = exact corpus bytes of a known entry of that variant (quoted in §1.3); assertion = `emit_non_summon_entry(classify_non_summon_entry(input)) == input`. Byte-equal, no normalization.

**(A5-10 fix) Every T30.N must pair the corpus-bytes input with a SYNTHETIC variant that would classify differently if the classifier reached for a registry / derived / canonical source.** Example for T30.1:

> **T30.1** `bare_base_game_ref_amnesia` — corpus: pansaer's `Amnesia` (grep anchor: `rg -noF 'Amnesia' working-mods/pansaer.txt`). **Synthetic pair**: `NotABaseGameItem` between two `+` delimiters, no wrapper, no suffixes. Both must parse to `NonSummonEntry::BareBaseGameRef { name }` and emit byte-equal on round-trip. Proves the classifier does not reach for a base-game item registry; V1 membership is purely tokenization-shape.

Every T30.N below specifies its synthetic pair in the same form (rewrite the per-test description at implementation time if the plan-time sketch proves too narrow). A test whose synthetic pair only differs by whitespace or trivial bytes is an accepting oracle per checklist rule #4 — reject and widen.

**(A5-11 fix) pansaer / community line-number citations**: every reference of the form `pansaer line N`, `pansaer pool N`, or `community line N` below and throughout §1.3 must be re-verified via `rg -nF '<verbatim_substring>' working-mods/<mod>.txt` at implementation start. pansaer.txt is a ~350KB monoline file (verified 2026-04-24 via `wc -l working-mods/pansaer.txt` → 1 line); EVERY `pansaer line N` citation for N≥2 is stale and must be replaced with a grep anchor (`rg -nF '<verbatim>' working-mods/pansaer.txt`) or deferred to Phase-A walker output. Community line numbers (V2, V3, V4-subset, V7, V9, V10, V11, V13) were spot-verified by Read this session (V2=13, V4=70, V8=130, V13=103 confirmed); the remaining lines (V3=16, V4=79/82/43/37/67, V7=88, V9=55/94/196, V10=46, V11=160) are RE-VERIFY-ON-IMPL-START. Any citation that fails verification is replaced with a grep anchor in the same PR as the classifier landing.

- **T30.1** `bare_base_game_ref_amnesia` — pansaer pool 1's `Amnesia` (between two `+` fences). Classifier must not reach for an item registry; the bytes `Amnesia` are classified as V1 purely by their tokenization.
- **T30.2** `composition_consolation_prize` — community line 16 `(self.unpack.Boss Smash^99#k.unusable#Camomile#k.stasis).n.Consolation Prize.doc.bord.tier.0.img.Bond Certificate.hsv.0:0:50`.
- **T30.3** `ritemx_blue_world` — community line 70.
- **T30.4** `ritemx_with_unpack_ash_of_war` — community line 67.
- **T30.5** `ritemx_paren_wrapped_seeing_red` — community line 43. Proves `OuterWrap::SingleParen` + trailer-outside shape round-trips.
- **T30.6** `ritemx_splice_rename_power_herb` — sliceymon (or community, locate via Phase-A walk) entry quoted in agent-instructions. Proves recursive `.splice.<body>` typing.
- **T30.7** `chain_composition_shell_bell` — quoted in agent-instructions, corpus-locate via Phase-A walk.
- **T30.8** `hash_joined_ritemx_keyword_hell_2` — community line 88.
- **T30.9a (A5-2 fix — renamed from T30.9)** `composition_single_keyword_component_destiny` — community line 130 `(k.death.n.Destiny.tier.0.img.Sapphire Skull.doc.kas333 l mini l ajfish)`. Exercises a `NonSummonEntry::Composition` with a single `CompositionComponent::Keyword` component (not a dedicated top-level Keyword variant; per A5-1 there is none).
- **T30.10** `template_jinx_the_flu` — community line 55 `(t.jinx.unpack.et2.Summon.Slimelet).n.The Flu.tier.0.…`.
- **T30.11** `inline_ability_spam` — community line 46.
- **T30.12** `allitem_composition_pandoras_cube` — community line 160.
- **T30.13a (A5-2 fix — renamed from T30.13)** `outer_wrap_double_paren_rube_goldberg` — community line 103. Exercises the `OuterWrap::DoubleParen` variant (trailer outside inner paren). A5-4 resolves whether a second `DoubleParenTrailerInside` variant ships at all; if it does, T30.13b is added alongside; if not, this test fully covers the double-paren surface.
- **T30.14** `side_def_with_sidesc_whistle` — pansaer pool 3 `(Void#top.hat.(Thief.sd.0-0:0-0:157-0.i.Blindfold.i.k.descend.i.top.facade.bas157:0:-20:0).n.Whistle.img.<bytes>.tier.2)`.
- **T30.15a (A5-2 fix — renamed from T30.15)** `composition_single_learn_component_poke` — pansaer `(learn.Poke.n.Learn Poke.img.Poke.tier.2)` (grep anchor: `rg -oF 'learn.Poke' working-mods/pansaer.txt` — verified 2026-04-24 returns 1 hit). V15 evidence is overwhelming: `rg -o 'learn\.[a-zA-Z]+' working-mods/*.txt | sort -u` returned 51 distinct `learn.<name>` tokens across all four mods (A5-6 fix: the earlier "evidence gap" concern turned out to be an artefact of pansaer's monoline shape — V15 is corpus-evidenced and ships). Exercises a `NonSummonEntry::Composition` with a single `CompositionComponent::Learn` component.

Every test fails if the classifier reaches for a registry / normalizer / canonical-source (source-vs-IR divergence). Each test's synthetic-variant pair (`<same shape, different-from-corpus but-round-trippable synthetic>`) is authored inline so the test also fails if the classifier hardcodes a corpus-specific string instead of reading structure.

### 5.2 Cross-variant distinctness (T31)

`#[test] fn variants_do_not_collapse()` — construct one instance of every V-N variant, assert all pairwise `!=`. Catches the "two variants silently emit to the same typed shape" regression.

### 5.3 Serde round-trip per variant (T32)

`#[test] fn non_summon_entry_serde_roundtrips_all_variants()` — serialize every variant through JSON + YAML, deserialize, assert `==`. Catches dropped `Deserialize` wiring on any variant added later (hook rule: every enum variant has a round-trip witness).

### 5.4 Unclassified-budget ratchet (T30.0)

`#[test] fn working_mods_produce_zero_unclassified_entries()` — run `extract_from_itempool` against every itempool body in every working mod; `NonSummonEntry::Unclassified` count must be 0 across the entire corpus. If any entry classifies as Unclassified, the Finding's 120-byte preview is printed in the failure message so the implementer knows what shape to type. Budget is ZERO — no tolerance (§3.5.3).

### 5.5 Four-mod round-trip (T33)

`#[test] fn all_four_mods_roundtrip_byte_equal_with_typed_non_summon()` — same as the existing four-mod round-trip guard, but asserts `ir.structurals.iter().any(|s| matches!(s, StructuralContent::ItemPool { items, .. } if items.iter().any(|i| matches!(i, ItempoolItem::NonSummon(NonSummonEntry::Unclassified { .. })))))` returns false. This is a second-line ratchet on top of T30.0.

### 5.6 `NonSummonEntry::Unclassified` has no authoring builder (T34)

`#[test] fn non_summon_entry_authoring_does_not_expose_unclassified()` — pure compile-test: `authoring::non_summon_entry::*` module does not expose any fn that returns `NonSummonEntry::Unclassified`. Use `cargo expand` or a proc-macro-level check; simpler is a grep assertion inside a build-script-style test (`rg -n 'NonSummonEntry::Unclassified' compiler/src/authoring/` returns 0).

### 5.7 Inherit from parent (T6, T7, T9c, T28 per agent-instructions)

- **T6** (shape fallback) — TMs / accessories / non-summon shapes that hit 8B's summon-shape prefilter first but fail the conjunctive detector → classify as NonSummon (some V1–V15 variant; specifically NOT `Unclassified`).
- **T7** (half-summon) — entry with `hat.egg.` but no matching vase-add pair → demotes to NonSummon with its V1–V15 variant.
- **T9c** (unclassifiable-wrapper Finding) — synthetic entry that passes the summon prefilter but no trigger shape matches → demotes to the 8B-owned `W-REPLICA-TRIGGER-UNCLASSIFIED` Finding path. Parallel to 8A.5's `W-REPLICA-NONSUMMON-UNCLASSIFIED` but NOT the same — the two Findings encode different classifier failure modes (trigger classification vs non-summon classification). Pair is preserved verbatim from parent §3.3.
- **T28** (index-stability for `ItempoolItem::Summon(i)`) — unchanged from 8A; 8A.5 adds a NonSummon-axis assertion: removing a `NonSummon` entry from a pool does NOT renumber `Summon(i)` indices (NonSummon entries are not in the `replica_items` vec).

---

## 6. Structural checks

### 6.1 Authority diff (hook rule #1)

Against parent plan `CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md`:
- **§3.1** — parent says `ItempoolItem { Summon(usize), NonSummon { name, tier, content: String } }` is the interim target. 8A ships that. 8A.5 replaces the `NonSummon { … content: String }` variant with `NonSummon(NonSummonEntry)`. This is **not** a new invariant; it is the closure of the SPEC §3.2 violation parent §3.1's prose already flagged as open debt. No parent amendment required.
- **§3.3** — parent's "Non-summon preservation" clause names `content` as the "existing pre-Chunk-8 raw-passthrough surface for un-modeled inner content". 8A.5 closes it. The parent's clause becomes stale on 8A.5 merge; add a one-line pointer in parent §3.3 to "SPEC §3.2 violation closed by 8A.5; see plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md" in the same PR.

Against SPEC:
- **SPEC §3.2** — "no raw passthrough". 8A.5 is the closure PR for itempool non-summon entries. No SPEC amendment; the §3.2 gate simply starts applying to this code path.
- **SPEC §3.3** — "permissive extract". `Unclassified` is a permissive path (warning + demoted typed variant); the `Finding` pairing makes the pressure-valve auditable (§3.5).

### 6.2 Canonical-set re-listing (hook rule #2)

V1–V15 is named once in §1.3 with one verbatim corpus instance per variant. §3.1's enum names the variants in code; prose references refer to the variant names via `NonSummonEntry::Foo`, not via re-listing the set. If a future plan section repeats the variant list, delete the repeat and reference by name.

### 6.3 Source-vs-IR coverage (hook rule #2)

Every test in §5.1 is a source-vs-IR divergence test — it uses the verbatim corpus bytes as input and asserts round-trip equality. The synthetic-variant pair in each test catches classifiers that hardcode the corpus string.

### 6.4 Structural smells (hook rule #3)

- **Permissive vs strict path split**: `NonSummonEntry::Unclassified` (permissive extract) vs authoring API rejecting the variant (strict construction) is explicitly preserved. They are NOT the same path with a policy knob — they cover distinct failure modes (extract can't classify yet vs author tries to construct malformed). Hook rule #3 preservation confirmed.
- **Repeated incantations**: emitter helpers are factored (§3.3); each variant has one emit arm; no variant-branching for shared operations like "emit trailer". Hook rule #3 guard.
- **Collapse of unlike invariants**: V2 and V3 collapse into `Composition` because their invariants are identical (hook rule #3 says the SAME invariant should be one type; different invariants should be different types). `OuterWrap` captures the sole source-byte difference. Variants like `TemplateJinx` and `InlineAbility` DO NOT collapse into `Composition` because their invariants differ (typed head vs generic head; abilitydata's typed body is not a generic composition component).

---

## 7. Risks and SPEC §3.2 closure proof

### 7.1 SPEC §3.2 closure proof

Claim: after 8A.5 merge, `rg -c '\bcontent\s*:\s*String\b\|\bbody\s*:\s*String\b' compiler/src/ir/mod.rs` returns 0 for every field on an `ItempoolItem` / `NonSummonEntry` / nested struct — i.e. no itempool-path IR field is a raw byte blob, EXCEPT `NonSummonEntry::Unclassified { source_bytes: String }` (SOLE exception; §3.5 gates it to zero corpus occurrences via T30.0).

Verification greps at §8 verification gate:
- `rg -n 'String' compiler/src/ir/mod.rs` — every hit audited inline in the PR body; each hit is either a typed name/identifier field (e.g. `NonSummonTrailer.name`, `TemplateRef.name`, `RitemxRef.hex`) or the single documented `Unclassified.source_bytes`. No "content" / "body" / "residue" field.
- `rg -c 'Unclassified' compiler/src/ir/mod.rs` — 1 hit (the variant declaration); implementation body count = 1 (the `#[serde(serialize_with = …)]` stderr-warning wiring).

### 7.2 Risks

1. **Phase-A walk misses a corpus shape.** Mitigation: T30.0's zero budget. Any missed shape fails T30.0 at `cargo test`; the Finding preview tells the maintainer what to type. This is the **primary** risk-mitigation strategy; the ratchet is the enforcement.
2. **Emitter canonical order drift.** If a trailer field's emit order diverges from source-byte order on any corpus entry, T30.1–T30.15 fails byte-equal. Mitigation: `NonSummonTrailer.suffixes_other` captures source-order suffix keys at extract time; emitter re-emits in that order. Non-order-sensitive fields (name, tier, img, doc, hue, etc.) emit in a fixed canonical order documented at impl time from the majority corpus order.
3. **`doc` field drift with textmod-escape sequences.** `doc` is stored as `String` because the escape sequences (`[n]`, `[grey]`, `[plusfive]`, etc.) are textmod presentation tokens, not IR invariants. Closing them would require a guide-authoritative escape-token enum (out of this chunk's scope; not a SPEC §3.2 violation because the doc field is not invariant-bearing — no xref rule, no CRUD op, no emit-time structural logic reads into `doc`). Risk: if a future xref rule DOES read `doc` bytes, the `String` field becomes a latent structural violation; document this as an open-surface in SPEC §3.2's commentary at that time.
4. **`Unclassified` occurrences on follow-on working-mods.** When a new mod lands, T30.0 will fail at first PR that includes it. Mitigation is the ratchet design — the failure IS the closure signal. No risk accrues silently.
5. **Classifier reaches for item registry for V1 disambiguation.** Mitigation: T30.1's synthetic assertion — "a token that is NOT a known base-game item but matches V1's tokenization shape" classifies as V1. Catches registry reach.
6. **Parent plan drift.** Parent §3.1 retains the transitional-shape language for 8A; 8A.5 merge updates parent §3.1's "open debt" prose to "closed by 8A.5". If parent prose is not updated in the 8A.5 PR, a future reader may author a new chunk against stale parent guidance. Mitigation: 8A.5 PR description includes the parent amendment as a checklist item.

---

## 8. Self-verification checklist (AI executes before completion)

- [ ] §3.2.1 Phase-A walker ran, output written to `target/itempool-shape-audit.txt`, every fingerprint maps to a §1.3 V-N variant. Per-variant counts recorded in the PR body; any deviation from §1.3's plan-time snapshot is explained.
- [ ] Every V-N variant in §1.3 has a verbatim corpus quote anchored to a specific `working-mods/*.txt` file. Re-verified at impl start.
- [ ] `rg -n 'content\s*:\s*String' compiler/src/ir/mod.rs` returns 0 in the `NonSummon` / `ItempoolItem` / `NonSummonEntry` scope.
- [ ] `rg -n 'body\s*:\s*String' compiler/src/ir/mod.rs` returns 0 for any itempool-path struct.
- [ ] Every V-N variant has a T30.N test (§5.1) with a corpus-verbatim input.
- [ ] T30.0 (Unclassified budget) passes with 0 unclassified entries across all four mods.
- [ ] T30.0's predicate is `== 0`, not `<= N` — ratchet, not tolerance.
- [ ] `cargo build` + `cargo clippy` + `cargo test` all clean.
- [ ] `cargo run --example roundtrip_diag` reports `Status: ROUNDTRIP OK` for all four mods; ItempoolItem counts match expected per-mod shape distribution.
- [ ] `NonSummonEntry::Unclassified` is not exposed in any `authoring/*.rs` file (T34 grep).
- [ ] Every emit arm in `replica_item_emitter.rs` / `structural_emitter.rs` reaches the shared `emit_trailer` / `emit_composition_component` / `emit_img` helpers — no duplicated incantations across variants.
- [ ] Parent plan §3.1 prose updated in same PR: the "open debt" clause on `ItempoolItem::NonSummon { content: String }` rewrites to "closed by 8A.5; see plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md".
- [ ] PR body enumerates every design decision D-A through D-E with the corpus evidence each one is justified on.

---

## 9. Verification gate (8A.5 → 8B handoff)

Ship only if all pass:

- `cargo build` + `cargo clippy` clean.
- `cargo test` passes (including T30.0–T30.15, T31, T32, T33, T34, inherited T6, T7, T9c, T28).
- All four mods roundtrip byte-equal.
- `rg -c 'NonSummon\s*\{\s*(name|content|body)\s*:' compiler/src/` returns 0 (the transitional 8A shape is fully retired).
- `rg -c 'NonSummonEntry\b' compiler/src/` returns ≥2 (type declaration + at least one authoring / extractor / emitter consumer).
- `rg -n 'W-REPLICA-NONSUMMON-UNCLASSIFIED' compiler/src/` returns ≥1 (the Finding code is wired).
- `rg -n 'Unclassified.*source_bytes' compiler/src/ir/mod.rs` returns 1 (the sole documented raw-byte field, bounded by T30.0).
- No `NonSummonEntry::Unclassified` in authoring API surface.
- Parent plan prose amendment landed in same PR.

If any item fails, fix in-place before the PR merges. Do not defer to 8B. 8B cannot start until 8A.5 has landed.
