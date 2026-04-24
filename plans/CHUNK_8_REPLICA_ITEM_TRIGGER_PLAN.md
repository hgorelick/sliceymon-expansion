# Chunk 8: Typed itempool + trigger-based ReplicaItem redesign + SPEC amendment (retire Capture/Legendary kinds) + 3-bucket X003/V020 unification

<!-- (M1 fix) Title rewritten to foreground the SPEC amendment and the X003/V020 bucket-label
     unification — previously these were buried under "typed itempool". The title now states
     every chunk-scope axis so a reader cannot miss the SPEC retirement or the xref unification
     that the plan executes alongside the IR work. -->


> **Scope**: the Rust compiler's IR for every `itempool.((...))` entry (both the dominant non-summon shape — TMs, accessories, consumables, ritemx refs, base-game refs, keyword/template compositions — and the sliceymon-specific Pokemon-summon shape), the extractor/classifier/emitter paths that feed them, V020/X003 cross-reference rules that bucket the summon side, and SPEC.md prose that describes both.
>
> **Plan chain**: four sub-chunks `8A → 8A.5 → 8B → 8C`. 8A is the upstream gate; 8A.5 closes the SPEC §3.2 violation opened by 8A's transitional `NonSummon { content: String }`; 8B ships the real summon classifier + xref widening on top of the typed NonSummon sum; 8C lands SPEC/foundations prose. 8A.5 and 8B are serial (8B consumes 8A.5's typed IR); 8A.5 and 8C are serial (8C's SPEC prose references the typed IR). See §9 for the dependency graph.
>
> **Corpus framing (load-bearing — do not reorder)**. Non-summon itempool entries are the **dominant shape** in the corpus: 3 of 4 working mods (pansaer 11 pools, punpuns 4 pools, community 24 pools, summon-free in all three) contain zero `SummonTrigger`-classifiable entries; only sliceymon's 16 itempool pools carry the 23 summon envelopes. `SummonTrigger::{SideUse, Cast}` is therefore a **sliceymon-specific specialization** layered on top of the base `NonSummonEntry` typed schema — not the organizing principle of Chunk 8. The 8A.5 typed sum (15 evidenced variants + `Unclassified` permissive hatch per SPEC §3.3) closes the base-case correctness gap before 8B's summon work ships.
>
> **SPEC.md §6.3 line 246**: the parenthetical `(captures / legendaries)` is dropped outright; no replacement. §3.6 specifies the exact edit.

## 1. Authority and evidence

### 1.1 What the corpus actually contains

**Corpus-wide itempool shape (4 working mods).** Non-summon entries (TMs, accessories, consumables, ritemx refs, base-game refs, keyword/template compositions) are the dominant shape by every count that matters: sliceymon carries 16 `itempool.` pools + 23 summon envelopes + many non-summon entries; pansaer has 11 pools and **0 summons**; punpuns has 4 pools and **0 summons**; community has 24 pools and **0 summons**. 3 of 4 mods are entirely non-summon. The `SummonTrigger` IR shipped in §3.2 is a sliceymon-specific specialization; the `NonSummonEntry` typed sum shipped in 8A.5 (`plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md` §3.1, 15 evidenced variants + `Unclassified` permissive hatch) covers the base case. §1.1's remainder walks the sliceymon summon bytes in detail because those are the bytes the new `SummonTrigger` / `DiceLocation` discriminators must classify; the per-mod non-summon enumeration is in 8A.5 §1.3.

**Sliceymon summon audit (load-bearing for 8A trigger IR and 8B classifier).** `working-mods/sliceymon.txt` contains three structural shapes across every `itempool.((…))` — two clean action-verb shapes plus one wrapped-only outlier (`Master Ball?`). Re-run these commands at implementation start; any count discrepancy against the numbers below blocks the chunk and triggers a §1.1 rewrite:

```
$ rg -o 'hat\.replica\.Thief\.n\.\w+' working-mods/sliceymon.txt | wc -l              # 18
$ rg -o 'hat\.replica\.Thief\.n\.\w+' working-mods/sliceymon.txt | sort -u | wc -l    # 18 distinct Pokemon
$ rg -c 'cast\.sthief\.abilitydata\.' working-mods/sliceymon.txt                      # 2 lines (both in Summons pools)
$ rg -o 'cast\.sthief\.abilitydata' working-mods/sliceymon.txt | wc -l                # 4 occurrences (Rainbow Wing, Silver Wing, Blue Orb, Red Orb)
$ rg -o 'hat\.egg\.' working-mods/sliceymon.txt | wc -l                               # 95 occurrences total across the whole file — NOT all inside itempool summon envelopes. Most are inside boss/phase/event bytes (ph.b…egg, mheropool.(replica…egg, etc.); only ~23 appear inside `itempool.((…))` bodies, and §3.3 rule 1 itempool-scoping filters the rest out.
$ rg -o 'vase\.\(add\.\(\(replica\.' working-mods/sliceymon.txt | wc -l               # 25 occurrences total — 23 inside itempool summon envelopes + 2 inside `ph.b` boss hero blocks (filtered out by §3.3 rule 1's itempool scope)
$ rg -o 'vase\.\(add\.\(replica\.'   working-mods/sliceymon.txt | wc -l               # 0 — single-inner form does NOT exist in any working mod
$ rg -o 'vase\.\(\(add\.'            working-mods/sliceymon.txt | wc -l               # 0 — doubled-outer form does NOT exist in any working mod
$ rg -o '\.n\.[A-Z][A-Za-z ]*? Ball\??\.tier\.' working-mods/sliceymon.txt | wc -l    # 21 Ball modifiers — 19 inside itempool (18 distinct SideUse balls including `Master Ball` for Mewtwo + 1 OnWrapped `Master Ball?` for Arceus) + 2 boss-block hits filtered by §3.3 rule 1 (one `Master Ball?` at byte ~106078 inside `ph.blillipup` boss hero block)

Per-pool entry counts (walked paren-depth, not string-split):
  Pokeballs Part 1: 11 entries | vase.(add.((replica. = 11
  Pokeballs Part 2:  8 entries | vase.(add.((replica. =  8
  Summons   Part 1:  2 entries | vase.(add.((replica. =  2
  Summons   Part 2:  2 entries | vase.(add.((replica. =  2
  Total summon-shaped envelopes: 23.
```

The "21 Ball modifiers" total vs "18 `hat.replica.Thief.n.<Pokemon>` occurrences" gap is `21 − 18 = 3`, explained by: (a) the OnWrapped `Master Ball?` (trailing `?` load-bearing, summons Arceus, in-scope itempool); (b) the boss-block `Master Ball?` at byte ~106078 (out of scope per §3.3 rule 1); (c) `Master Ball` (no `?`, tier 9, in-scope itempool, summons Mewtwo) — `Master Ball` IS one of the 18 SideUse entries because it has the outer flat `hat.replica.Thief.n.Mewtwo.sd.<faces>` preface and is otherwise an ordinary SideUse ball that happens to share a surface name with the OnWrapped `Master Ball?`. Surface-name overlap is a classification trap: a string match on `Master Ball` would conflate two different items. The detector is structural (preface presence + dice-location), not lexical.

The OnWrapped `Master Ball?` itempool entry has NO outer flat `hat.replica.Thief.n.Arceus.sd.<faces>` preface and NO `cast.sthief.abilitydata.`. The thief's side-definition lives inside the wrapper as `.i.(hat.Thief.sd.34-10:34-10:34-8:34-8:34-5:34-5)` (capital `Thief`). This is the InnerWrapper dice-location case; see §3.2's `SummonTrigger::SideUse { dice_location: InnerWrapper }` and §3.3 rule 2(d) for the classifier's resolution.

`hat.egg.` also appears in bosses and event modifiers, and `vase.(add.(…` appears with at least three paren-depth variants (`vase.(add.((replica.`, `vase.(add.(replica.`, `vase.((add.(…`). Across summon-item envelopes in all four working mods this round, the `vase.(add.((replica.<team>.n.<Pokemon>…)).mn.<Pokemon>))` form is the only one observed. If implementation surfaces additional variants, §3.3's detector must catalog and accept them, never narrow.

| Trigger / dice-location shape | Expected count | Container names |
|---|---|---|
| **`SideUse { dice_location: OuterPreface }`**: outer flat `hat.replica.Thief.n.<Pokemon>.sd.<faces>…` preface + wrapped `hat.(replica.Thief.i.(all.(left.hat.egg.(…))))` payload. Engine reads dice from the outer preface. | 18 | Pokeballs Part 1 (10): Poke (Pikachu), Great (Ivysaur), Ultra (Charizard), Fast (Sneasel), Friend (Furret), Heavy (Metagross), Level (Rattata), Lure (Poliwag), Moon (Delcatty), and `Master Ball` (no `?`, tier 9, summons Mewtwo). Pokeballs Part 2 (8): Dive, Luxury, Nest, Premier, Timer, Dusk, Quick, Dream Ball. Total 18 — one per distinct `hat.replica.Thief.n.<Pokemon>` occurrence. |
| **`SideUse { dice_location: InnerWrapper }`** (`Master Ball?` outlier): no outer flat preface, no `cast.sthief.`. Thief's side-definition lives inside the wrapper as `.i.(hat.Thief.sd.<faces>)` within `hat.(replica.Thief.i.(all.(left.hat.egg.(Dragon.n.Arceus…)…)))`. **Engine behavior IS THE SAME as `OuterPreface`** — the engine reads `hat.Thief.sd.<faces>` identically whether on the outer preface or inside the wrapper; the variant exists ONLY to preserve source-byte fidelity on round-trip. The discriminator is `DiceLocation`, not a separate game mechanic; that is why this is a sub-axis of `SideUse`, not a third top-level trigger. | 1 inside itempool (2 total `rg` hits on `\.n\.Master Ball\?`: byte ~106078 inside `ph.blillipup` boss hero block (out of scope per §3.3 rule 1), byte ~274161 inside Pokeballs Part 1 itempool (in scope)). | `Master Ball?` in Pokeballs Part 1 (trailing `?`; target = `Arceus`; **`enemy_template = Dragon` (capital), `team_template = Housecat` (capital)** — verified via `rg 'Dragon\.n\.Arceus' working-mods/sliceymon.txt`; the boss-block hit uses lowercase `dragon`/`housecat` and is out of scope). |
| **`Cast { dice }`**: `cast.sthief.abilitydata.(thief.sd.<faces>…)` inside a wrapper. Equipping grants a spell; the summon fires when the player **casts** it. Genuinely different player action from `SideUse`. | 4 | Rainbow Wing, Silver Wing (Summons Part 1); Blue Orb, Red Orb (Summons Part 2). |

**Total**: 23 summon-shaped envelopes across the four itempool pools. **Baselines (§4 and T22) must reflect 23, not 21.**

No ball uses `cast.sthief`; no wing/orb uses the `hat.replica.Thief.n.<Pokemon>` outer preface. Re-verify at impl time: `rg '\.n\.\w+ Ball.*cast\.sthief|cast\.sthief.*\.n\.\w+ Ball' working-mods/sliceymon.txt` must return zero.

**Shared payload — structure confirmed at three distinct paren depths per trigger:**

- **`SideUse { dice_location: OuterPreface }`** (18): the `hat.egg.<enemy-template>.n.<Pokemon>…` subtree sits inside `hat.(replica.Thief.i.(all.(left.hat.egg.(…))))` at wrapper depth 3; dice live on the outer preface `hat.replica.Thief.n.<Pokemon>.sd.<faces>`.
- **`SideUse { dice_location: InnerWrapper }`** (1, Master Ball?): the `hat.egg.<enemy>.n.<Pokemon>…` subtree sits inside `hat.(replica.Thief.i.(all.(left.hat.egg.(…))))` at wrapper depth 3; `hat.Thief.sd.<faces>` lives INSIDE the egg body as `.i.(hat.Thief.sd.<faces>)`, not on an outer preface.
- **`Cast { dice }`** (4): the `hat.egg.<enemy>.n.<Pokemon>…` subtree sits inside `cast.sthief.abilitydata.(thief.sd.<faces>.i.(mid.hat.egg.(…)))` at wrapper depth 5.

Canonical defeat/join emission (shared across all three shapes, present per entry):
```
…i.t.vase.(add.((replica.<team-template>.n.<Pokemon>.hp.<hp>.col.<col>.tier.<tier>.sd.<faces>
  …speech.<speech>).mn.<Pokemon>))
```
The `.mn.<Pokemon>` appears OUTSIDE the innermost `replica.(…)` paren group but INSIDE the `vase.(add.((…)))` outer parens. Emitter must reproduce this exact nesting.

**Enemy templates** observed in sliceymon (source-byte-preserving — case-exact, no normalization at extract):
```
rg -o 'hat\.egg\.\(?([A-Za-z]+)\.n\.' working-mods/sliceymon.txt | sort -u
  → inside itempool summon envelopes (after §3.3 rule 1 scoping): Wolf, wolf (lowercase wolf appears in 1 entry), Dragon, dragon (lowercase dragon appears only in the boss-block Master Ball? at byte ~106078, OUT OF SCOPE).
  → outside itempool (boss/event hero blocks; out of scope here): Agnes, Alpha, Bones, Caw, Chomp, Demon, Gytha, Magrat, Saber, Slimelet, Spiker — total 16 distinct strings across the file. The IR field is a `String`; the extractor preserves source bytes verbatim; do NOT normalize case.
```

**Team templates** observed in sliceymon vase-add target (source-byte-preserving):
```
rg 'vase\.\(add\.\(\(replica\.([A-Za-z]+)\.' -o working-mods/sliceymon.txt | sort -u
  → housecat, Housecat, prodigy, Statue
```
`Statue` is the team-add template for Groudon (Red Orb). `housecat`/`Housecat` and `Wolf`/`wolf` are case-variants that the extractor MUST preserve verbatim; any normalization is an authoring-layer concern and out of scope here (SPEC §3.3 permissive extract). Plan field comments in §3.2 name real-corpus values only, not a hypothetical superset.

**Guide authority.** `reference/textmod_guide.md` table entry for `replica.` reads verbatim: *"Not explicitly referenced in guide text examined — no guide entry."* Line 764 documents only `Summon | sd.172, any hat.egg.entity`. The guide does NOT name `hat.replica.Thief.n.<Pokemon>`, `cast.sthief.abilitydata.(thief.sd.<faces>…)`, or the "equipping replaces sides / equipping grants a spell" narrative. This §1.1's mechanical narrative is an inference from sliceymon bytes, not guide text. The IR therefore models source-byte-observed shape per SPEC §3.3 permissive extract. Per CLAUDE.md "when the parser, emitter, and the guide disagree, the guide wins": if a guide revision later contradicts these observations, the IR must be revised in the same PR as the guide change.

**Trigger-name vocabulary.** `SideUse`, `Cast`, and the `DiceLocation::{OuterPreface, InnerWrapper}` sub-axis are IR-internal labels coined by this plan. None correspond to a guide term. The guide uses `cast.<ability>` (line 1008 etc.) as a construct; there is no guide term for the "use-a-side" action. The `DiceLocation` sub-axis names a structural source-byte fact (where the dice live in the wrapper), not a player-action — that is why it is encoded as a sub-discriminator on `SideUse`, not a third top-level variant. Do not rename these names to match imagined game vocabulary without new evidence.

Also note for the detector (§3.3): "Shadow Ball" in sliceymon is an **ability name** (inside `.abilitydata.(…).n.Shadow Ball`) on the Duskull hero, not a summon container. A string-match on `Ball` would false-positive here. Detection must be structural (`hat.egg.<enemy>` sub-block + `vase.(add.((replica.<team>.n.<same-Pokemon>…))` pair at correct nesting), not lexical on container-name suffix.

### 1.2 What the mod does NOT do

Top-level `item.<Template>…` modifiers exist in sliceymon (`item.t.jinx.Summon`, `item.hat.barbarian.sd`, `item.k.growth.t`, `item.determination.t.jinx`, `item.handcuffs.part.1`, etc.) — all of them are **accessory/template definitions**, none are Pokemon-summon items.

Corpus-wide count of comma-split top-level modifiers starting with `item.` (run at implementation start; expect all four zeros):
```
$ python3 -c "
import sys
for name in ['sliceymon','pansaer','punpuns','community']:
    with open(f'working-mods/{name}.txt') as f:
        mods = f.read().split(',')
    n = sum(1 for m in mods if m.strip().lower().startswith('item.'))
    print(f'{name}: {n}')"
  sliceymon: 0    pansaer: 0    punpuns: 0    community: 0
```

No mod in the working-mods corpus uses the shape the current `parse_legendary` gate expects.

**Community-mod summon-shape audit (in-scope, not deferred).** Community has 21 `itempool.` modifiers, 9 `hat.egg.` occurrences, 1 `cast.sthief` occurrence, and zero `vase.(add.((replica.` occurrences. The conjunctive detector (§3.3 rule 2) will therefore not reclassify any community entry as a summon. Evidence:
```
$ rg -c 'itempool\.' working-mods/community.txt                      # 21
$ rg -c 'hat\.egg\.' working-mods/community.txt                      # 9
$ rg -c 'cast\.sthief' working-mods/community.txt                    # 1  (inside standalone `.n.Mental Defense` item, NOT inside itempool — confirmed by `rg -c 'itempool.*cast\.sthief|cast\.sthief.*itempool' working-mods/community.txt` → 0)
$ rg -c 'vase\.\(add\.\(\(replica\.' working-mods/community.txt      # 0
```
The `Mental Defense` item is a top-level `(mid.mid.hat.thief.i.(cast.sthief.abilitydata.(mage.sd…))…).n.Mental Defense.tier.0…` modifier — not an itempool member, no `hat.egg.` pair, and no `vase.(add.((replica.` pair. §3.3 rule 1 excludes it (itempool-scoped extractor only). It is classified by the existing non-itempool route (Structural or generic item modifier, depending on top-level classifier). T27 asserts community produces zero `ReplicaItem`s; the classifier route for `Mental Defense` is not `ReplicaItem` under any §3.1 post-retirement gate.

This must be re-verified at impl time; if any of these counts change, the community-summon test (T27 below) will fail loudly.

**Pansaer summon-shape audit (in-scope, not deferred).** Pansaer has 0 `hat.replica.Thief`, 0 `cast.sthief`. It has 1 line with `hat.egg.` (7 occurrences on that single monoline) and 1 `abilitydata.`. None of the `hat.egg.` occurrences sit in an itempool summon envelope (no matching `vase.(add.((replica.`). Line-count vs occurrence-count only differ here because pansaer is written on a single line; the semantically meaningful count is the occurrence count. Evidence:
```
$ rg -c 'hat\.replica\.Thief' working-mods/pansaer.txt               # 0 lines, 0 occurrences
$ rg -c 'cast\.sthief'        working-mods/pansaer.txt               # 0 lines, 0 occurrences
$ rg -c 'hat\.egg\.'          working-mods/pansaer.txt               # 1 line; but `rg -o ... | wc -l` = 7 occurrences (all on the single monoline file — see "Note on pansaer byte size" below)
$ rg -c 'abilitydata\.'       working-mods/pansaer.txt               # 1 line, 1 occurrence
$ rg -c 'vase\.\(add\.\(\(replica\.' working-mods/pansaer.txt        # 0
```
Test T27 asserts pansaer produces zero `ReplicaItem`s post-change, and classifies the single `hat.egg.` as Structural/boss bytes.

**Note on pansaer byte size.** `wc -l working-mods/pansaer.txt` returns 0 because the file contains no newlines; the file is 350 779 bytes (not empty). Any audit that uses `wc -l` alone will miss pansaer — use `wc -c` or grep-then-count.

**Punpuns summon-shape audit.** Punpuns has 0 `hat.replica.Thief` and 3 `cast.sthief` (inside BooleanPhase boss abilities — Reflection and Surtr; see sliceymon-audit table). No `vase.(add.((replica.` occurrences; punpuns has no itempool summons. Evidence (note: `rg -c` counts LINES, not OCCURRENCES — for multi-line corpus files these can differ):
```
$ rg -c 'hat\.replica\.Thief' working-mods/punpuns.txt               # 0 lines, 0 occurrences
$ rg -c 'cast\.sthief'        working-mods/punpuns.txt               # 3 lines, 3 occurrences (all inside ph.b boss abilitydata, not itempool)
$ rg -c 'hat\.egg\.'          working-mods/punpuns.txt               # 13 lines; `rg -o ... | wc -l` = 19 occurrences (all inside ph.b boss abilitydata)
$ rg -c 'vase\.\(add\.\(\(replica\.' working-mods/punpuns.txt        # 0
```
Punpuns' `cast.sthief` and `hat.egg.` occurrences live inside `ph.b…abilitydata.(…)` boss-encounter modifiers, not itempools — §3.3 rule 1 (walk only itempool body) excludes them, so they stay Structural. Test T27 covers this assertion as well.

### 1.3 Where the IR currently sits relative to that evidence

| IR / parser artifact | Corpus instances | Notes |
|---|---|---|
| `ModifierType::Legendary` (`classifier.rs`) + `starts_with_ci(modifier, "item.")` gate | 0 | Models a top-level `item.<Pokemon>…` shape that appears in no mod. |
| `parse_legendary` (`extractor/replica_item_parser.rs`) | 0 | Dispatch target of the dead gate; untested against real-mod input. |
| `ReplicaItemContainer::Legendary` variant (`ir/mod.rs`) | 0 | Carries a mechanical claim ("persistent ally with spell") that contradicts the mod — in-mod, the Wing/Orb items summon an enemy that must be defeated before joining, same as balls. |
| `ReplicaItemContainer::Capture { name }` variant | 0 via the current `ReplicaItem` route | Every Pokemon-summon entry in the corpus is **inside** `itempool.((…))`, which the extractor currently keeps as opaque structural text. `roundtrip_diag` reports `Replicas ir1=0` for all four mods. |
| `ReplicaItem` struct | 0 data instances across all four working mods | Consequence of the above: the entire IR type is exercised only by synthetic test helpers. |

### 1.4 Authority conflicts to resolve in this chunk

1. **SPEC.md line 342** says verbatim: `**ReplicaItem** — IR type for items that summon a Pokemon as a unit. Two kinds: **Capture** (one-shot, mid-fight, via ball-style item) and **Legendary** (persistent ally with spell). Both share the same IR struct with a kind discriminant; "capturable" and "legendary" are *kinds*, not separate IR types.` The corpus shows neither "one-shot mid-fight" nor "persistent ally" matches the actual mechanic. Both kinds summon an enemy that on defeat is vase-added to the team via `replica.<team-template>`; both require a kill before the Pokemon joins. The action-verb differs (`use-side` vs `cast-spell`), not the downstream pipeline. The "kind discriminant" framing survives intact in the new IR — only the discriminator axis moves from `Capture/Legendary` to `SummonTrigger::{SideUse, Cast}`.
2. **SPEC.md §6.3 line 246** says `A Pokemon may exist in at most one of: heroes, replica items (captures / legendaries), monsters.` The bucket `replica items` is singular — the parenthetical `(captures / legendaries)` is prose gloss. Chunk 6 reified the parenthetical as an IR enum (`ReplicaItemContainer`). That was an extrapolation from prose, not from mod bytes.
3. **`PLATFORM_FOUNDATIONS_PLAN.md` §F9** already records in its own Lessons block that restating a canonical set inside a §F / chunk must be identical or strictly narrower, never contradictory. This chunk records a sibling: **variants must be evidenced; zero corpus instances = zero variants.**

## 2. Dependencies

**Pre-condition: Chunks 5, 7, and 9 must land on `main` before Chunk 8 starts.** As of this plan revision (Round 2 audit, 2026-04-23), verified against `main` HEAD `bc0ff44`:
- `rg -c 'slice_before_chain_and_cast' compiler/src/util.rs` → **0** (Chunk 9 NOT yet on main)
- `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` → **0** (Chunk 5 NOT yet on main)
- `main` top commits show Chunk 6 (`bc0ff44`) is the latest merge; no PRs for Chunks 5/7/9 are merged.

The pre-condition is **not currently satisfied**. Chunk 8 MUST NOT start until all three merge. Implementer re-runs the §9.0 checks; if any fail, halt and resolve upstream before any 8A code begins. No fold-in branch exists in this plan — it was explicitly removed in favor of the hard pre-condition.

**Intra-chunk dependencies (8A → 8A.5 → 8B → 8C)**. Each downstream sub-chunk requires its upstream sibling to have landed on `main`:
- 8A.5 requires 8A's IR types (`ItempoolItem::NonSummon` with transitional `content: String`) and stub extractor to exist on `main`.
- 8B requires 8A.5's typed `NonSummonEntry` sum on `main` — without it, 8B's real `extract_from_itempool` has no typed non-summon output surface and would re-open the SPEC §3.2 raw-passthrough violation per 8A.5 §1.2.
- 8C requires 8A.5's typed IR on `main` — SPEC prose describes the shipped typed shape, not a transitional raw-string debt.
See §9.4 for the serial execution map.

- **Chunk 6** landed `ReplicaItemContainer::{Capture, Legendary}` on `main`. Verify at implementation start: `rg -nc 'pub enum ReplicaItemContainer' compiler/src/ir/mod.rs` must return ≥1. This chunk deletes that enum; straightforward delete-and-replace, no merge-ordering.
- **(M3 fix) Chunk 5 status — DETERMINISTIC, not conditional.** Chunk 5 is merged (`git log`: commit `975da96`, "Merge pull request #12 from hgorelick/feat/chunk-5-merge-derived-strip"). `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` returns **0** (verified 2026-04-24 this session). PR #12 shipped other Chunk 5 work (provenance-gated strip, `merge_with_overlay`) but did NOT ship the function. Chunk 8 therefore **authors `generate_hero_item_pool` from scratch** against the trigger-based IR shape — no migration step exists because there is no prior-shape implementation to migrate. See 8A fix A9 for the concrete scope. The §9.0 pre-condition grep is now a status check: `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` → expect 0 at Chunk 8 start, ≥1 at Chunk 8 completion. The original "Chunk 5 MUST land first" gate and the Round-2 "branch-state caveat" paragraphs below are retracted — superseded by this deterministic state. The paragraphs are preserved below only for reader continuity; treat them as historical context, not authoritative.

- **Chunk 5 (`generate_hero_item_pool`) — MUST land on `main` before Chunk 8 starts; NOT yet in-tree as of Round 2 audit.** _(HISTORICAL — see M3 fix above for the current deterministic state.)_ Chunk 5 ships first (per user direction) against the pre-Chunk-8 IR shape: `compiler/src/builder/derived.rs::generate_hero_item_pool` keyed on `match item.container { Capture { name } => ... }`. Chunk 8 therefore **must include a migration patch to `builder/derived.rs`** that rekeys the function on `matches!(item.trigger, SummonTrigger::SideUse { .. })` + `item.target_pokemon`, and renames Chunk 5's test `derived::hero_item_pool_matches_sliceymon_via_container_enum` to `derived::hero_item_pool_matches_sliceymon_via_trigger`. Verify Chunk 5 is in-tree before starting: `rg -nc 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` must return ≥1. The amendment to the foundations plan's Chunk 5 block (§3.7) reflects this final-state spec but does not change Chunk 5's shipped code — only Chunk 8 does that. **Branch-state caveat (Round 2 audit):** as of plan-write time the `feat/chunk-5-merge-derived-strip` branch (PR #12) does NOT contain `generate_hero_item_pool` — its diff covers `ir/merge.rs`, `builder/mod.rs`, the provenance-gated derived-structural strip, and the new `merge_with_overlay` signature, not a new `derived::generate_hero_item_pool`. If Chunk 5's merged state continues to lack the function, **Chunk 8 becomes the author** (not the migrator) of `generate_hero_item_pool` and the §4 `derived.rs` row expands to cover the full implementation + byte-equality test against sliceymon's hero-bound ItemPool, not just the rekey + rename. Re-verify at §9.0: if `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` returns 0 post-Chunk-5-merge, Chunk 8 authors the function against the final trigger-based IR shape (no migration step needed; the function is born correct). Either way, the pre-condition grep at §9.0 gates the work.
- **Chunk 7 (lib-code unwrap/expect/panic elimination) — MUST land on `main` before Chunk 8 starts; NOT yet in-tree as of Round 2 audit.** Touches `extractor/*` panic sites and `builder/hero_emitter.rs`. **File overlap is real but harmless.** Chunk 7's branch diff (`git diff main..feat/chunk-7-panic-unwrap-elimination --stat`, verified at Round 2) touches `extractor/replica_item_parser.rs` (57 lines), `extractor/mod.rs` (2 lines), and `builder/replica_item_emitter.rs` (4 lines) — all files Chunk 8 rewrites. The Chunk 7 edits are **subsumed** by Chunk 8's wholesale rewrites (see §9.1 — Chunk 8 replaces those file bodies entirely), so no merge conflict survives the rewrite. The only practical consequence for Chunk 8 is **pattern inheritance**: if Chunk 7 introduced new `CompilerError` constructors (e.g. `CompilerError::classify`, `CompilerError::invalid_field`), Chunk 8's new error sites in `replica_item_parser.rs` and `classifier.rs` use the same constructors — grep `compiler/src/error.rs` for the post-Chunk-7 public surface before authoring §3.1's `classify(...)` error path. Verify at implementation start: `rg -c '\.unwrap\(\)|\.expect\(' compiler/src/extractor/ compiler/src/builder/` should be ≤ Chunk 7's post-merge baseline.
- **Chunk 9 (§F10, replica-parser depth-aware scalar extraction) — MUST land on `main` before Chunk 8 starts; NOT yet in-tree as of Round 2 audit.** Lives as §F10 inside `plans/PLATFORM_FOUNDATIONS_PLAN.md` (no standalone plan file). The `util::slice_before_chain_and_cast` helper + `depth_aware` parameter on `extract_color` are in-tree. Verify at implementation start: `rg -c 'slice_before_chain_and_cast' compiler/src/util.rs` must return ≥1. **There is no fold-in path** — the §3.0 / §3.8 / §9.0 fold-in branches in earlier drafts of this plan are removed.
- **PR #10 (V020 capture/legendary narrowing)**. Verify state at implementation start: `gh pr view 10 --json state,mergedAt,mergeCommit`. If `state != "CLOSED"` or `mergedAt` is non-null, §3.5 must be reconciled with its shipped state before any work here lands — do not assume. The boss-bucket V020 invariant in `no_double_fire_on_working_mods` is covered by §3.5's structured-field predicate regardless of PR #10's state.
- **Chunk 10 (BooleanPhase phase routing)** — **verify existence before relying on this dependency**. As of plan-write time the file `plans/CHUNK_10_*.md` is not present and no `chunk-10` remote branch exists. Implementation must re-check (`ls plans/CHUNK_10_*.md 2>/dev/null && git branch --remotes | grep chunk-10`); if either returns a hit, diff the file list against §4 and reconcile. If both return zero, note on the PR that no Chunk 10 is in flight; the overlap risk is moot.

## 3. What ships

### 3.1 Retirements (zero-corpus-user code)

- `compiler/src/extractor/classifier.rs`
  - Delete `ModifierType::Legendary` variant (declared near line 17).
  - Delete the gate `if starts_with_ci(modifier, "item.") { return Ok(ModifierType::Legendary); }` (line 184).
  - **Also delete** `ModifierType::ReplicaItem` and `ModifierType::ReplicaItemWithAbility` variants (declared near lines 12-13). Rationale: these dispatch in `compiler/src/extractor/mod.rs:59-63` to the now-deleted `parse_simple` / `parse_with_ability` (both retired in §3.1's `replica_item_parser.rs` bullet). Leaving the variants alive but the dispatch targets gone is a compile error; keeping the dispatch alive is a broken route. With this chunk the only remaining replica-item entry point is `ModifierType::ItemPool → extract_from_itempool`. Verify with `rg "ModifierType::ReplicaItem(WithAbility)?\b" compiler/` returning zero hits post-chunk (extend T12–T14 or add T12a below).
  - Route remains: `itempool.` → `ModifierType::ItemPool`, which is now the sole entry point to replica-item extraction.
- `compiler/src/extractor/replica_item_parser.rs`
  - Delete `parse_legendary` + its helper functions + all its tests.
  - Delete `parse_simple` and `parse_with_ability` in their current form — both model the "Capture inside itempool" shape assuming a single entry per modifier, which is not how itempools actually work (multiple `+`-joined entries per pool).
  - Module's new public surface is a single fn: `extract_from_itempool(body: &str, modifier_index: usize) -> Result<Vec<ExtractedSummon>, CompilerError>` described in §3.3 (non-summon bytes stay in the structural channel, not a parallel enum).
- `compiler/src/ir/mod.rs`
  - Delete `ReplicaItemContainer` enum.
  - Delete any `#[cfg(test)]` helpers that build `ReplicaItemContainer::{Capture, Legendary}` values — replace with builders for the new shape.
  - **Delete the existing `pub struct ItemPoolEntry { name, tier, content: String }` (line 934) and replace `StructuralContent::ItemPool { body, items: Vec<ItemPoolEntry> }` (line 801-803) with `StructuralContent::ItemPool { items: Vec<ItempoolItem> }` — there is no surviving `body: String` field, because pure-IR rebuild (per §3.4) regenerates the body at build time from the typed item list.** `ItempoolItem` is the new typed sum: `Summon(usize)` (index into `ModIR.replica_items`) | `NonSummon { name: String, tier: Option<i8>, content: String }`. The `NonSummon.content: String` is the existing pre-Chunk-8 raw-passthrough debt for non-summon entries (TMs, accessories, consumables); this chunk explicitly preserves but does not retire that debt — the responsibility moves from `ItemPoolEntry.content` to `NonSummon.content` with no shape change. Plan-wide consequence: there is no second representation surface — the legacy `Vec<ItemPoolEntry>` is gone.
  - Retire the existing `ReplicaItem.template: String`, `ReplicaItem.sticker: Option<String>`, and `ReplicaItem.name: String` fields. The new struct shape is in §3.2; the migration of every callsite that touches these fields is in the §4 file-touch table. Exact line numbers (verified Round 2 audit against `main` HEAD `bc0ff44`):
    - `item.template` emission sites: `replica_item_emitter.rs:33, 94, 157`
    - `item.sticker` emission site: `replica_item_emitter.rs:72`
    - `item.name` emission sites: `replica_item_emitter.rs:59, 83, 120, 146, 182`
    - `parsed.template` / `item.template` test asserts in the parser: `replica_item_parser.rs:243, 269` (and `classifies_legendary_into_enum` / `legendary_name_is_last_depth0_n_before_cast` tests surrounding them)
    - `parsed.template` / `item.template` test asserts in the emitter: `replica_item_emitter.rs:350, 402` (plus adjacent `parsed.container`/`parsed.name` asserts on 348-349, 400-401)
    - `item.name`/`item.container`/`item.sd`/`item.template` read sites in xref: `xref.rs:229-231` (container routing), `234, 236` (owner-map keys inside X003), `528-529` (owner-map keys inside `check_cross_category_names`), `337` (`format!("replica_items[{}].sd", item.name)` in `iter_dice_faces`), `338` (`&item.sd` — after §3.2 the `sd` field is gone; becomes `item.trigger.dice_faces()`), `339` (`item.template.as_str()` passed to `iter_dice_faces`)
    - `item.name` read site in ops: `ir/ops.rs:90` (`find_name_category(&item.name)`)
  - `template` is unconditionally replaced by the literal `"Thief"` (capital — matches source bytes) at the emitter sites (`replica_item_emitter.rs:33, 94, 157`) and by the literal `"thief"` (lowercase — face-template-compat key) at `xref.rs:339`'s `iter_dice_faces` call (verify the face-compat table key shape before changing; the lowercase/capital distinction is load-bearing per the Round 1 face-compat lookup evidence).

### 3.2 New IR shape (`compiler/src/ir/mod.rs`)

**Two axes land across 8A → 8A.5:**
- **Summon axis (sliceymon-specific, shipped by 8A)**: `ReplicaItem` + `SummonTrigger` + `DiceLocation` per the Rust sketch below. Covers the 23 sliceymon summon envelopes enumerated in §1.1.
- **Non-summon axis (dominant shape, transitional in 8A → typed in 8A.5)**: 8A ships `ItempoolItem::NonSummon { name, tier, content: String }` as an **intentional, tracked SPEC §3.2 violation** (raw-passthrough `content`); 8A.5 closes that violation by retyping it as `ItempoolItem::NonSummon(NonSummonEntry)` with 15 evidenced variants (V1–V15) plus a single `NonSummonEntry::Unclassified { source_bytes }` permissive hatch per SPEC §3.3. The full typed schema — variant prose, `NonSummonTrailer` / `ImgPayload` / `AbilityBody` helper records, recursive `.splice.<body>` modeling — lives in `plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md` §3.1 and is NOT reproduced here. The canonical-set rule applies: name the typed sum once (`NonSummonEntry`), reference it by name; the variant list is authoritative in 8A.5.

The Rust sketch below covers the summon-axis types only. 8A.5 landing retypes `ItempoolItem::NonSummon` without touching the summon-axis shape.

```rust
/// A Pokemon-summon item extracted from an entry inside `itempool.((…))`.
///
/// An entry is classified as a `ReplicaItem` iff its inner
/// `hat.(replica.Thief.i.(all.(…)))` wrapper contains BOTH:
///
///   1. a `hat.egg.<enemy_template>.n.<Pokemon>…` sub-block (the summoned
///      enemy that must be defeated), AND
///   2. a `vase.(add.((replica.<team_template>.n.<Pokemon>…)))` sub-block
///      (the team-join replica emitted on defeat) whose `<Pokemon>` matches
///      the egg's.
///
/// **No raw-passthrough escape hatch.** Every field this chunk ships must be
/// derivable from verified corpus bytes; if implementation finds a sub-block
/// not covered by the field list below, the struct must be widened in the
/// same commit. `extras: Vec<RawSubBlock>` is explicitly rejected — that
/// pattern is raw passthrough with extra steps (SPEC §3.3; architecture.md
/// "No raw passthrough"). Non-summon itempool entries are covered by §3.3's
/// classification-level decision, not by a per-field escape hatch.
pub struct ReplicaItem {
    pub container_name: String,         // "Great Ball", "Silver Wing", "Master Ball?", "Master Ball" (no `?`), ...; never None; preserve trailing `?` and trailing whitespace from source bytes
    pub target_pokemon: String,         // "Ivysaur", "Lugia", "Ho Oh" (with space), "Arceus", ...; preserved as source bytes, no case normalization
    pub trigger: SummonTrigger,
    pub enemy_template: String,         // in-scope sliceymon corpus values: "Wolf", "wolf", "Dragon" (capital — used by the in-scope OnWrapped Master Ball? entry); the lowercase `dragon` only appears in the boss-block hit that §3.3 rule 1 filters out. Field is the raw source-byte string; do not normalize case.
    pub team_template: String,          // in-scope sliceymon corpus values: "housecat", "Housecat" (capital — used by the in-scope OnWrapped Master Ball? entry), "Statue" (Red Orb / Groudon). Field is the raw source-byte string; do not normalize case.
    pub tier: Option<u8>,
    pub hp: Option<u16>,                // matches existing ReplicaItem.hp type
    pub color: Option<char>,            // matches existing ReplicaItem.color type; `Color` is not a repo type
    pub sprite: SpriteId,
    pub sticker_stack: Option<ModifierChain>, // RENAME + TYPE CHANGE from existing `sticker: Option<String>`. The existing field carries a raw `.sticker.<bytes>` payload; the new `sticker_stack` parses that payload through the existing `ModifierChain` (`.i.` and `.sticker.` are both valid chain segment introducers per `ir/mod.rs:103-198`). Verify against the corpus: `Master Ball?` boss-block (out of scope but corpus-visible) carries `.sticker.k.possessed#togfri#togres`; the `Caterpie` SideUse entry carries `.i.mid.sticker.(right.hat.statue)#togfri`. The chain parser already handles both; the rename moves a raw String to a typed chain.
    pub speech: Option<String>,
    pub doc: Option<String>,
    pub toggle_flags: Option<String>,   // matches existing ReplicaItem.toggle_flags; `ToggleFlags` is not a repo type and no shape was proposed
    pub item_modifiers: Option<ModifierChain>, // `ItemModifiers` is not a repo type — `ModifierChain` is the correct existing type
    // NOTE: No top-level `abilitydata: Option<AbilityData>` field. Corpus
    // walk of all 4 cast.sthief.abilitydata bodies (see §1.1) shows zero
    // depth-0 `.n.<spell_name>` — AbilityData (which requires a non-Optional
    // `name: String`) does not fit the cast.sthief shape. Cast-specific
    // source-byte universals live as emitter constants (§3.4); per-item cast
    // dice live in `SummonTrigger::Cast { dice }`. If a future corpus entry
    // surfaces abilitydata-shaped bytes on a ReplicaItem — either on SideUse
    // or on Cast — widen the relevant variant (or add a field on
    // SummonTrigger) in the same PR; do NOT re-introduce `Option<AbilityData>`
    // next to the trigger.
    pub source: Source,
}

/// The player action that triggers the summon. Two variants — `SideUse` (use
/// a thief side) and `Cast` (cast a thief spell) — capture the only two
/// distinct game mechanics observed in the corpus. The historical "OnWrapped"
/// case (Master Ball?) is NOT a third variant: the engine reads
/// `hat.Thief.sd.<faces>` identically whether the dice live on the outer
/// preface or inside the wrapper, so it is the same player-action with a
/// different source-byte encoding. That source-byte encoding is captured by
/// the `dice_location` sub-discriminator on `SideUse`.
///
/// Dice payload uses the same field name across both variants to forbid
/// variant-branching for dice access at callsites — §3.4, xref, and
/// authoring all route through `trigger.dice_faces()`. Branching on the
/// variant is legitimate where the action axis genuinely matters (emitter
/// must know whether to emit `cast.sthief.abilitydata.(...)` or a wrapper-
/// only payload; `ability` field exists only on `Cast`); the rule is
/// "don't `match` for `dice` access," not "never `match` at all."
pub enum SummonTrigger {
    /// Player uses a thief-side during a fight. Dice live somewhere in the
    /// source bytes — `dice_location` says where. Engine behavior is
    /// identical for both locations; the discriminator exists for
    /// round-trip fidelity. Corpus counts (verified per §1.1):
    ///   `OuterPreface` = 18 (every Ball entry except `Master Ball?`),
    ///   `InnerWrapper` = 1 (`Master Ball?` only).
    SideUse {
        dice: DiceFaces,
        dice_location: DiceLocation,
    },

    /// Player casts a thief-spell. The summon fires on cast. Wrapper carries
    /// `cast.sthief.abilitydata.(thief.sd.<UNIVERSAL>.i.(…))` at depth 0 of
    /// the `hat.(replica.Thief.i.(all.(...)))` body. Corpus count per §1.1: 4
    /// (Rainbow Wing + Silver Wing in Summons Part 1; Blue Orb + Red Orb in
    /// Summons Part 2).
    ///
    /// **Payload rationale.** `dice: DiceFaces` — the item's displayed dice,
    /// parsed from the PER-ITEM inner `.i.hat.(replica.thief.sd.<faces>)`
    /// chain segment inside the abilitydata body, NOT from the outer
    /// `thief.sd.<UNIVERSAL>`. Corpus walk (see §1.1 evidence block) confirms
    /// all 4 entries share the outer `thief.sd.182-25:0:0:0:76-0:0` and the
    /// outer template `thief` — these are cast-spell universals. The outer
    /// cast-template and outer cast-dice are therefore EMITTER LITERALS (see
    /// §3.4 constants `CAST_SPELL_TEMPLATE = "thief"`,
    /// `CAST_SPELL_DICE = "182-25:0:0:0:76-0:0"`), not IR fields.
    ///
    /// **No parallel-representation risk.** There is no `AbilityData` reuse
    /// here: all 4 corpus abilitydata bodies have **zero depth-0 `.n.`**
    /// (verified via paren-walk — see §1.1). AbilityData requires a
    /// non-Optional `name: String` and would extract empty or spurious inner
    /// egg `.n.<Pokemon>`. The Pokemon identity lives on `target_pokemon`
    /// and the item identity lives on `container_name`; the cast body has no
    /// separate identity of its own.
    ///
    /// **Widening contract.** If a future corpus entry surfaces a Cast body
    /// with a different outer template, different outer dice, or a depth-0
    /// `.n.<spell_name>`, the IR MUST widen in the same PR (per SPEC §3.3
    /// no-raw-passthrough and CLAUDE.md no-deferred-correctness): either
    /// lift `CAST_SPELL_TEMPLATE` / `CAST_SPELL_DICE` from constants into
    /// variant fields, or introduce a `cast_spell_name: String` field.
    /// Until corpus shows variation, the constants are the minimal correct
    /// model.
    Cast { dice: DiceFaces },
}

/// Where the dice live in the source bytes for a `SideUse` summon.
/// **Source-shape sub-axis, not a player-action.** The two locations
/// produce identical engine behavior; the discriminator exists only to make
/// extract → build round-trip byte-equal.
pub enum DiceLocation {
    /// Outer flat preface: `hat.replica.Thief.n.<Pokemon>.sd.<faces>` sits
    /// BEFORE the wrapper's opening `hat.(replica.Thief.i.(all.(...`. 18
    /// corpus entries.
    OuterPreface,
    /// Inner wrapper: no outer preface; dice live inside the wrapper's egg
    /// body as `.i.(hat.Thief.sd.<faces>)` (capital `Thief` — case-preserving).
    /// 1 corpus entry (`Master Ball?` summoning Arceus). If a future corpus
    /// adds another, this variant's count widens; if it stays at 1, the
    /// variant still ships because the source-shape invariant is preserved
    /// (round-trip would otherwise lose 1 entry).
    InnerWrapper,
}

impl SummonTrigger {
    /// Shared accessor to forbid variant-branching at callsites for dice
    /// access. Every consumer (emitter, xref, authoring) routes dice
    /// through this method — mirrors the hook rule against duplicated
    /// N-line incantations. Variant-branching for `ability` (Cast-only) and
    /// for the wrapper-shape decision (SideUse `OuterPreface` vs
    /// `InnerWrapper` vs `Cast`) is legitimate at the emitter; that
    /// branching captures real differences (different emit code paths),
    /// while dice access does not.
    pub fn dice_faces(&self) -> &DiceFaces {
        match self {
            SummonTrigger::SideUse { dice, .. } => dice,
            SummonTrigger::Cast    { dice }     => dice,
        }
    }
}
```

Field-level notes:

- `container_name` replaces the old `Capture { name }` variant's `name` field as a plain `String`. It is never `None` — every corpus summon item carries a container name (the outermost `.n.<ItemName>.tier.<n>`). Trailing `?` and trailing whitespace are part of the source bytes; preserve verbatim. Note `Master Ball` (no `?`) and `Master Ball?` are TWO distinct entries with overlapping surface-name root in sliceymon Pokeballs Part 1 — the field stores them distinctly.
- `target_pokemon`, `enemy_template`, `team_template` are extracted by name from the inner `hat.egg.<enemy>.n.<Pokemon>` and `vase.(add.((replica.<team>.n.<Pokemon>…)))` sub-blocks — not guessed, not looked up, not registry-gated. These are source-preserving byte fields: preserve the **exact** source string (including trailing whitespace if the corpus emits it — see `hat.egg.((replica.Prodigy.…).n.Anorith ` with trailing space in sliceymon); do NOT apply `.trim()` or case normalization to these fields. **Case matters**: the in-scope `Master Ball?` entry uses `Dragon`/`Housecat` (capital) per §1.1; an extractor that lowercased these would silently fail T2a's byte-equality round-trip.
- **Identity contract — clarification, not a behavior change.** The retired `ReplicaItem.name: String` field already carries the inner `.mn.<Pokemon>` value (set by `parse_simple` line 25 / `parse_with_ability` line 68) — it is the Pokemon-identity, not the container-identity. The current X003 owner-map keying at `xref.rs:234` (`owners.entry(item.name.to_lowercase())`) ALREADY buckets by Pokemon name (Poke Ball ≠ Great Ball today, because their `name` fields hold different Pokemon strings). The Chunk 8 rename `item.name → item.target_pokemon` is a **clarity rename**, not a contract change: the existing semantics are preserved, just under a less-misleading field name. Renames apply at every callsite: `compiler/src/xref.rs:234, 236, 528-529` (owner-map keys), `compiler/src/xref.rs:337` (`format!("replica_items[{}].sd", item.target_pokemon)`), `compiler/src/ir/ops.rs:90-92` (`find_name_category(&item.target_pokemon)`). Container-name collisions are covered by the separate duplicate-item check (verify present at `compiler/src/ir/ops.rs:298-310` `remove_replica_item_by_name` neighborhood); X003/V020 stay Pokemon-axis.
- **`template` field deletion + replacement.** Current `ReplicaItem.template: String` (`ir/mod.rs:619`) is deleted. The dice carried by `SummonTrigger.dice_faces()` are emitted on the `hat.Thief` / `cast.sthief` block; the literal template string varies by callsite: emitter (`builder/replica_item_emitter.rs:33, 94, 157`) emits the literal `"Thief"` (capital, matches sliceymon source bytes); face-template-compat lookup (`xref.rs:339` `iter_dice_faces`) passes the literal `"thief"` (lowercase, matches the face-compat table key — verify by reading the existing face-compat module before changing). Test fixture sites (`replica_item_parser.rs:243, 269`, `replica_item_emitter.rs:350, 402`) currently assert `parsed.template, item.template` — rewrite to assert against the literal `"Thief"` source bytes.
- **`sticker` → `sticker_stack` rename + `Option<String>` → `Option<ModifierChain>` type change.** Current `ReplicaItem.sticker: Option<String>` (`ir/mod.rs:636`), emitted at `replica_item_emitter.rs:72-75` as raw `.sticker.<bytes>` passthrough. The new field carries a parsed `ModifierChain` (the existing chain parser already accepts `.sticker.` segments per `ir/mod.rs:124, 143, 163, 198`). Migration: extractor parses the `.sticker.<bytes>` payload through `ModifierChain::parse`; emitter writes via `chain.emit()`. Round-trip test required: at least one corpus entry containing `.sticker.` must extract to a `ModifierChain` and re-emit byte-equal. Add this assertion to T1 / T3 (Pokeballs Part 1 round-trip — the `Master Ball?` boss-block sticker chain is out-of-scope, but the Caterpie SideUse entry inside Pokeballs Part 1 carries `.i.mid.sticker.(right.hat.statue)#togfri` which IS in scope). If the parser cannot round-trip a corpus sticker chain byte-equal, the type change must be deferred to a follow-up chunk and `sticker: Option<String>` stays.
- `sprite: SpriteId` uses Chunk 3b's unified sprite shape. Extraction is `SpriteId::owned(source_n_verbatim, img_data_bytes)` where `source_n_verbatim` is the exact bytes after `.n.` — **not** `target_pokemon.to_lowercase()`. Per Chunk 3b lesson 1, do not reach for the registry. The sprite-name field is for identity/UX only; emission uses `.img.<bytes>` directly.
- `sticker_stack`, `item_modifiers` both reuse the existing `ModifierChain` type (no new `StickerRef` / `ItemModifiers` types). `toggle_flags: Option<String>` matches the existing `ReplicaItem.toggle_flags` shape (no new `ToggleFlags` type).
- **Field list is exhaustive, not provisional.** If implementation surfaces a corpus entry whose payload includes a sub-block not listed above, the struct must be widened **in the same commit** with a typed field. There is no `extras` / `raw_remainder` / `RawSubBlock` escape. The tests in §5 must fail loudly on any sub-block the struct does not model.
- **No `source_byte_range` / extraction-time-only fields on the struct.** The build pipeline reconstructs the itempool body purely from typed IR (`StructuralContent::ItemPool { items: Vec<ItempoolItem> }`); see §3.4 for the pure-IR rebuild and §3.1 for `ItemPoolEntry` retirement.

### 3.3 Itempool extractor extension (`compiler/src/extractor/replica_item_parser.rs`)

**Preamble — pure-IR rebuild, single representation.** Itempool extraction returns a typed list (`Vec<ItempoolItem>`); build regenerates the whole itempool body from that list (no source-byte ranges on the IR, no extract-time substring storage on `ReplicaItem`). The legacy `ItemPoolEntry` is retired in §3.1 — there is no parallel representation; `ItempoolItem` is the sole surface. Non-summon itempool entries (TMs, accessories, consumables) are modeled by `ItempoolItem::NonSummon { name, tier, content: String }` whose `content` field is the existing pre-Chunk-8 raw-passthrough debt for un-modeled inner content (SPEC §3.2 acknowledges this debt for non-summon entries; this chunk does not pretend to retire it but also does not multiply it). Summon entries are typed via `ItempoolItem::Summon(usize)` carrying an index into `ModIR.replica_items`. The build pipeline emits `+`-joined entries by walking `items`; for each `Summon(i)` it calls `emit_replica_item(&ir.replica_items[i])`, for each `NonSummon { content }` it emits `content` verbatim with its `name`/`tier` prefix.

**Index-stability invariant for `ItempoolItem::Summon(usize)`.** The index into `ModIR.replica_items` is a position key, not a stable identifier. Any CRUD op that reorders or shrinks `replica_items` MUST rewrite every `Summon(i)` in every `StructuralContent::ItemPool` simultaneously, within the same IR-mutation call. Concretely:
- `ir::ops::remove_replica_item_by_name` (today at `compiler/src/ir/ops.rs:298-310`, which today just does a `Vec::retain` on `replica_items`) becomes a two-step: (i) find the removed index `j`; (ii) for every `StructuralContent::ItemPool { items }` in `ir.structurals`, traverse `items` and for each `ItempoolItem::Summon(i)`: if `i == j` remove the entry from `items`, if `i > j` decrement it by 1.
- `ir::ops::add_replica_item` (appends to `replica_items`) is index-stable for existing `Summon(i)` entries (the new index is `replica_items.len() - 1`, which does not collide with any existing `i`). Insertions at a non-tail position are not supported by the current authoring layer; if a future op adds that, it must shift all `Summon(i)` where `i ≥ insertion_point` up by 1 in the same call.
- No third party writes to `replica_items` directly; authoring routes through `ops.rs`. The invariant is enforced at the ops boundary, not as a runtime check.
- Test `ops::remove_replica_item_reindexes_itempool_summons` (T28 in §5) constructs an IR with 3 replica_items and a `StructuralContent::ItemPool { items: vec![Summon(0), NonSummon{..}, Summon(2)] }`, removes `replica_items[0]`, asserts the pool becomes `vec![NonSummon{..}, Summon(1)]` (the `Summon(0)` entry removed; the `Summon(2)` decremented to `Summon(1)`). Without this test the invariant rots silently: a CRUD op that desyncs indices would pass IR-equality round-trip but break build.

Replace the module contents with a single public function and its helpers:

```rust
/// Extract the typed itempool item list from an itempool modifier body.
///
/// Returns the FULL typed item list (both Summon and NonSummon entries) in
/// source order. The summons are added to `ModIR.replica_items` by the
/// caller (`extractor/mod.rs`), and the indices captured here in
/// `ItempoolItem::Summon(usize)`. Build (§3.4) rebuilds the entire
/// itempool body from this list — no source-byte ranges, no opaque body
/// string, no parallel representation.
pub fn extract_from_itempool(
    body: &str,                     // the full `itempool.((…)).n.…` modifier source
    modifier_index: usize,          // caller-provided for Finding.position wiring
    next_replica_index: usize,      // first index this call will assign for Summon entries
) -> Result<ItempoolExtraction, CompilerError>;

pub struct ItempoolExtraction {
    /// New `ReplicaItem`s to append to `ModIR.replica_items` in source order.
    pub new_replica_items: Vec<ReplicaItem>,
    /// The full ordered item list for `StructuralContent::ItemPool { items }`.
    /// `Summon(i)` indices reference positions in `new_replica_items` offset
    /// by `next_replica_index`.
    pub items: Vec<ItempoolItem>,
}
```

Parser rules:

1. **Entry splitting** — walk the body one paren-depth at a time; split the inner content of the outermost `itempool.((…))` on `+` **only** at paren-depth 0. **`#` is NOT an itempool entry separator** — it appears intra-entry at deeper depths (`.i.k.stasis#(Handcuffs.part.1)`, `(Eye of Horus#Chainmail#…)`, etc.) inside every sliceymon summon entry. Splitting on `#` at depth 0 fragments a single entry into pieces and breaks the detector. Preserve the `+` joiner as part of the following entry's prefix for the structural channel's re-emission.
2. **Summon detection** — for each entry byte-range:
   a. Parse the outer `hat.(replica.Thief.i.(all.(…)))` wrapper; on failure, skip (entry stays in the structural bytes).
   b. Within `all.(…)`, scan depth-0 sub-blocks for a `hat.egg.<enemy_template>.n.<Pokemon>` match. If absent → skip.
   c. Within the same wrapper, scan for a vase-add pair. **Only the corpus-evidenced form is accepted**; per §3.7's "Lessons" entry, zero-corpus-hit variants are hypotheses, not models, and do not ship. Today only ONE form appears across all four working mods (verified §1.1: 25 hits in sliceymon, 0 in pansaer/punpuns/community):
      - `vase.(add.((replica.<team_template>.n.<Pokemon>…)))` — outer `(` + inner `((`.
      If a future mod surfaces another depth variant, expand the detector AND add a corpus-proof test in the same commit (no speculative widening). The `<Pokemon>` in the vase-add must equal the egg's `<Pokemon>` byte-for-byte (case-sensitive; trailing whitespace preserved on both sides — a trim applied to only one side is a classification bug). Mismatch or absence → skip (entry classified as `ItempoolItem::NonSummon`).
   d. Classify trigger (three cases, evaluated in this order — the order is required so `Cast` wins over `SideUse{InnerWrapper}` when both patterns happen to co-occur in future entries):
      - **`Cast`** — wrapper contains `cast.sthief.abilitydata.(thief.sd.<UNIVERSAL>.i.(…))` at depth 0 relative to the wrapper body. Extraction: (i) assert the outer template = `thief` and outer dice = `"182-25:0:0:0:76-0:0"` against the emitter-constant values (bytes mismatch = Finding + demote to NonSummon per SPEC §3.3 permissive extract; widening-contract guidance is in §3.2's Cast doc); (ii) walk the abilitydata chain to locate the PER-ITEM `.i.hat.(replica.thief.sd.<faces>)` segment at depth ≥ 1 inside the abilitydata body (case-preserving: lowercase `replica.thief`, distinct from SideUse's capital `hat.Thief`) and parse `<faces>` through `DiceFaces::parse`; (iii) emit `SummonTrigger::Cast { dice }`. The universal outer template + outer dice are NOT stored on the IR (they reconstruct at emit time from §3.4 constants). A Cast body missing the inner `replica.thief.sd.<faces>` segment is a bug — fail loudly with `CompilerError::classify(modifier_index, preview, "Cast trigger detected but no inner replica.thief.sd.<faces> segment found in abilitydata body")`.
      - **`SideUse { dice_location: OuterPreface }`** — entry is prefixed by an outer flat `hat.replica.Thief.n.<Pokemon>.sd.<faces>` preface ending at (and not crossing into) the wrapper's opening `hat.(replica.Thief.i.(all.…` — dice = `<faces>` from the preface → `SummonTrigger::SideUse { dice, dice_location: DiceLocation::OuterPreface }`.
      - **`SideUse { dice_location: InnerWrapper }`** — no outer flat preface AND no `cast.sthief.` at wrapper depth 0. Dice live INSIDE the wrapper as `.i.(hat.Thief.sd.<faces>)` (capital `Thief`, case-preserving) within the egg body → `SummonTrigger::SideUse { dice: <faces from the inner .i.(hat.Thief.sd.<faces>)>, dice_location: DiceLocation::InnerWrapper }`. Required for the `Master Ball?` entry in sliceymon Pokeballs Part 1; must not silently fall through to `OuterPreface` with a zero-faces dice.
      - If none of the three match, the (a)+(b)+(c) pre-checks succeeded (egg + vase-add pair present, Pokemon names match byte-for-byte) but no wrapper-shape was identifiable — this is a **new corpus shape** the IR does not yet model. Per SPEC §3.3 permissive extract, do NOT fail the run: (i) emit a `Finding` with code `W-REPLICA-TRIGGER-UNCLASSIFIED` at Severity::Warning carrying `modifier_index` + 120-byte preview, (ii) demote the entry to `ItempoolItem::NonSummon { name, tier, content }` so it survives the structural channel with byte-equal round-trip, (iii) do NOT construct a `ReplicaItem` (no default variant, no placeholder trigger). The `CompilerError::classify` path is reserved for the Cast-inner-dice-missing case above where (a)+(b)+(c)+outer-template-match succeeded but the documented inner shape was absent — a genuine contract violation, not a new shape. Same test required in §5 as T9c below.
   e. Extract every other field per §3.2. Push the constructed `ReplicaItem` onto `new_replica_items` and append `ItempoolItem::Summon(next_replica_index + new_replica_items.len() - 1)` to `items`.
3. **Non-summon preservation** — any entry not matched as a summon becomes `ItempoolItem::NonSummon { name, tier, content }` where `name` and `tier` come from the entry's outer `.n.<name>.tier.<n>` and `content` is the source bytes between the entry's structural fences (the existing pre-Chunk-8 raw-passthrough surface for non-summon content; SPEC §3.2 debt preserved without multiplication). No second representation surface — `ItempoolItem` is the sole shape; the legacy `ItemPoolEntry` is retired in §3.1.
4. **Paren/depth correctness — REQUIRES §F10 helper in-tree.** This rule is enforceable only if `util::slice_before_chain_and_cast` + `extract_color(_, depth_aware: true)` are available at implementation time. **Per §2 Dependencies, Chunk 9 is a hard prerequisite**: either land it first, or land its helper as the first commit of this chunk (explicit §3.0 below). Without the helper, scalar extractors leak chain-interior `.hp.` / `.col.` / `.sd.` / `.img.` substrings into top-level `ReplicaItem` fields, breaking SPEC §3.3 self-containment. Do not ship §3.3 without the helper in-tree.

### 3.8 Chunk 9 helper precondition

**Removed in favor of §2's pre-condition statement.** Chunk 9 lands on `main` before Chunk 8 starts (per user direction); the helper is in-tree at start. No fold-in branch. §9.0 verifies the precondition with a single `rg` check.

### 3.4 Builder / emitter (`compiler/src/builder/replica_item_emitter.rs`)

- New fn: `emit_replica_item(item: &ReplicaItem) -> String` — emits a single summon entry (trigger-shape + shared payload) without any itempool wrapping.
- New fn: `emit_itempool(items: &[ItempoolItem], replica_items: &[ReplicaItem], pool_name: &str) -> String` — pure-IR rebuild of an entire `itempool.((…)).n.<pool_name>` modifier. Walks `items` in source order, joins with `+` at paren-depth 0. For each `ItempoolItem::Summon(i)`, emits `emit_replica_item(&replica_items[i])`. For each `ItempoolItem::NonSummon { name, tier, content }`, emits the entry's `.n.<name>.tier.<n>` outer + verbatim `content` bytes. Build is a function of IR fields only — no source-byte ranges, no extraction-time substring storage on `ReplicaItem`.
- Trigger shape (three emit paths, matching the §3.2 enum):
  - `SideUse { dice, dice_location: OuterPreface }` → outer `hat.replica.Thief.n.<target_pokemon>.sd.<dice>` preface + shared wrapped payload `hat.(replica.Thief.i.(all.(left.hat.egg.(<enemy_template>.n.<target_pokemon>…))))`.
  - `SideUse { dice, dice_location: InnerWrapper }` → no outer preface; shared wrapped payload with dice emitted as `.i.(hat.Thief.sd.<dice>)` (capital `Thief`) inside the egg body (matching `Master Ball?`'s source shape).
  - `Cast { dice }` → wrapped payload `hat.(replica.Thief.i.(all.(cast.sthief.abilitydata.(thief.sd.{CAST_SPELL_DICE}.i.(mid.hat.egg.(<enemy_template>.n.<target_pokemon>…).hp.<hp>.i.hat.(replica.thief.sd.<dice>.i.<item_modifiers>)…)))))`. The outer `cast.sthief.abilitydata.(thief.sd.{CAST_SPELL_DICE}.i.(...))` preamble is emitted from two emitter constants declared at the top of `replica_item_emitter.rs`: `const CAST_SPELL_TEMPLATE: &str = "thief"` and `const CAST_SPELL_DICE: &str = "182-25:0:0:0:76-0:0"`. These constants capture source-byte universals observed across all 4 corpus Cast entries (§1.1); widening them to IR fields requires new corpus evidence. The inner `.i.hat.(replica.thief.sd.<dice>.i.<item_modifiers>)` segment uses the PER-ITEM `Cast.dice` and the ReplicaItem's `item_modifiers`. Capital `Thief` on the outer `hat.(replica.Thief` wrapper; lowercase `thief` on the inner `replica.thief.sd.<dice>` segment — both case-preserving per corpus.
- Shared payload emitter handles `hat.egg.<enemy_template>.n.<target_pokemon>…`, `vase.(add.((replica.<team_template>.n.<target_pokemon>…)).mn.<target_pokemon>))` (with `.mn.` emitted OUTSIDE the innermost `replica.(…)` but INSIDE the `vase.(add.((…)))` outer parens — see §1.1 canonical shape), `sticker_stack` (emit via `ModifierChain::emit`), speech, doc, img. Single private helper, not duplicated per trigger.
- Dice access at all callsites goes through `item.trigger.dice_faces()` — no variant-branching for dice; variant-branching for the wrapper-shape decision (which emit path) and for `ability` (Cast-only) is legitimate and required.
- The literal template string is `"Thief"` (capital) at every emission site — replaces the retired `item.template` field per §3.1.
- No source-byte storage anywhere in this module. The structural-channel emitter calls `emit_itempool` for `StructuralContent::ItemPool`; the function takes only typed IR.

### 3.5 Xref (`compiler/src/xref.rs`) — 3-bucket shape

SPEC §6.3's canonical Pokemon-uniqueness bucket set is `{hero, replica_item, monster}`. Both X003 and V020 must reference it by name; neither re-lists or subdivides its members.

- **`Finding` shape widening — first commit of 8b.** The current `Finding` struct (in `compiler/src/finding.rs` post-main-merge — NOT `xref.rs`; Chunk 9 split the struct out) has no typed bucket fields. Add two fields with `#[serde(default, skip_serializing_if = "Vec::is_empty")]` / `#[serde(default)]` so the schema delta is non-breaking:
  - `pub buckets: Vec<&'static str>` — populated by X003 (and any other rule that bucket-checks); empty by default.
  - `pub includes_boss: bool` — populated by V020 when the collision crosses a boss bucket; `false` by default.

  **(M5 fix) Finding-shape widening audit — a DISTINCT commit before the widening.** Enumerate every `Finding { .. }` construction site in the tree and confirm each uses `..Default::default()` tail syntax. Verified 2026-04-24 this session via `rg -n 'Finding \{' compiler/src/ compiler/tests/`:
    - **13 sites in compiler/src/xref.rs** (lines 237, 388, 419, 456, 519, 559, 597, 620, 639, 658, 688, 707, 726)
    - **1 site in compiler/src/lib.rs** (line 84, `report.errors.push(Finding {`)
    - **1 site in compiler/src/ir/merge.rs** (line 79, `warnings.push(Finding {`)
    - **1 site in compiler/tests/build_options_tests.rs** (line 277, `let f = Finding {`)
    - **Total: 16 construction sites.** The struct DEFINITION at compiler/src/finding.rs:24 is NOT a construction site.

  Audit step (new commit BEFORE the widening): for each of the 16 sites, read surrounding lines, confirm `..Default::default()` is present; if any site is exhaustive (no default tail), append `..Default::default()` in the audit commit. Then widen `Finding` in the next commit — `cargo build` clean proves no exhaustive-literal breakage.

  Serde/schemars audit: add a test asserting `serde_json::to_string(&Finding { buckets: vec!["hero", "replica_item"], includes_boss: true, ..Default::default() })` round-trips cleanly via `serde_json::from_str`, and that `schemars::schema_for!(Finding)` emits the new fields with `default` + `skip_serializing_if` annotations (so downstream consumers don't get a schema breakage).

  T20/T21 below assert against these typed fields, NOT against message-prose substrings.
- **X003** (`check_duplicate_pokemon_buckets`): three buckets `{hero, replica_item, monster}`. No `match &item.container` routing — `ReplicaItemContainer` is retired. Every `ReplicaItem` (whether `SideUse{OuterPreface}`, `SideUse{InnerWrapper}`, or `Cast`) contributes to the single `replica_item` bucket. **Concrete edits to `compiler/src/xref.rs`**:
  - Lines 229-231 (the `match &item.container { Capture => "capture", Legendary => "legendary" }` routing): delete the match entirely; bucket is unconditionally the string literal `"replica_item"`.
  - Lines 234, 236, 528-529 (owner-map keys): switch from `item.name.to_lowercase()` / `item.name.clone()` to `item.target_pokemon.to_lowercase()` / `item.target_pokemon.clone()` per the §3.2 identity contract (clarity rename — see §3.2's "Identity contract — clarification, not a behavior change" note).
  - Line 337 field-path format string: change `format!("replica_items[{}].sd", item.name)` to `format!("replica_items[{}].sd", item.target_pokemon)`.
  - Line 338 (`&item.sd`): change to `item.trigger.dice_faces()` — after §3.2's field move, dice live on the trigger, not on the top-level struct. The accessor returns `&DiceFaces` matching the expected type. This edit is REQUIRED; omitting it is a compile error because `ReplicaItem.sd` no longer exists.
  - Line 339 (`iter_dice_faces` call passing `item.template.as_str()`): replace with the literal `"thief"` (lowercase — face-template-compat key; verify by reading the face-compat table key shape before changing).
  - Suggestion string at xref.rs:279 currently reads `Rename '{}' so it exists in exactly one of: hero, capture, legendary, monster`. Change to `... exactly one of: hero, replica_item, monster`.
  - X003 message format at xref.rs:271-275 currently embeds the bucket list (so `"capture"` and `"legendary"` appear in error messages). The format itself is fine post-rename (buckets become `["hero", "replica_item"]`); but **delete or rewrite any test in `xref.rs:#[cfg(test)] mod tests` that asserts on `"capture"` or `"legendary"` substrings in X003 messages**. Concrete grep: `rg -n '"capture"|"legendary"' compiler/src/xref.rs` — every hit in the test module is in scope; specifically `xref::x003_distinguishes_capture_from_legendary_buckets` (line ~1015 today) becomes meaningless and must be deleted.
- **V020** (`check_cross_category_names`): narrowing predicate is `pokemon_only = distinct_buckets is subset of {hero, replica_item, monster} with cardinality ≥ 2`. V020 scope: boss-involving collisions + intra-bucket duplicates. Any collision whose bucket set is entirely inside the Pokemon-uniqueness set is silenced here (X003 owns it); any collision that includes a boss bucket still fires V020 with `includes_boss = true` set on the Finding.
- **Tests to keep / write**:
  - `v020_silent_on_cross_bucket_pokemon_{hero_replica, hero_monster, replica_monster, case_insensitive}` — one Replica is enough per test; the trigger variant is irrelevant and should be tested as such (see next bullet).
  - `v020_still_fires_on_boss_{hero, replica, monster}_collision` — assert `finding.includes_boss == true`.
  - `v020_still_fires_on_intra_bucket_duplicate_{heroes, replicas, monsters, bosses}`.
  - `v020_and_x003_coexist_when_collision_spans_boss_and_pokemon_buckets` — asserts via the typed Finding fields above. Predicate: `v020_finding.includes_boss == true` AND `x003_finding.buckets.iter().all(|b| ["hero", "replica_item", "monster"].contains(b))` (i.e. X003's buckets stay inside the Pokemon-uniqueness set; V020 carries the boss flag). No substring scan on message prose.
  - `no_double_fire_on_working_mods` — uses the same typed-field predicate. If any pre-existing test asserts on message strings for the double-fire condition, rewrite in the same commit.
- **Tests to delete** (already-shipped tests that depend on retired distinctions):
  - `xref::x003_distinguishes_capture_from_legendary_buckets` (line ~1015) — the discriminator is gone.
  - Any sibling test asserting on `"capture"` / `"legendary"` substrings in X003 messages (run `rg -n '"capture"|"legendary"' compiler/src/xref.rs` to enumerate before landing).
- **New source-vs-IR tests** (trigger-variance coverage for Option B):
  - `xref::x003_one_replica_bucket_across_triggers` — IR with hero "Pikachu" + `SideUse{OuterPreface}` replica "Pikachu" + `SideUse{InnerWrapper}` replica "Pikachu" + `Cast` replica "Pikachu" fires X003 exactly **once** with `buckets = ["hero", "replica_item"]`. Proof that trigger / dice-location granularity did not leak into Pokemon uniqueness.
  - `xref::x003_treats_all_trigger_variants_as_one_bucket` — IR with one of each `SideUse{OuterPreface}` / `SideUse{InnerWrapper}` / `Cast` replica all named "Pikachu" (no hero) fires X003 once with bucket set `{"replica_item"}` (intra-bucket duplicate), and V020 stays silent. All three combinations exercised; a test that covers only two leaves the third silently broken.

### 3.6 SPEC amendments (`SPEC.md`)

- **§6.3 line 246** — drop the parenthetical per user decision (D1 resolved 2026-04-22). Preserve markdown bold and the trailing CRUD-enforcement sentence verbatim:
  - Before: `A Pokemon may exist in **at most one** of: heroes, replica items (captures / legendaries), monsters. CRUD operations enforce this; the author cannot accidentally bypass it.`
  - After:  `A Pokemon may exist in **at most one** of: heroes, replica items, monsters. CRUD operations enforce this; the author cannot accidentally bypass it.`
  - Rationale: §6.3 is about uniqueness buckets; bucket internal structure and flavor are IR-level detail covered in SPEC line 342. Replacing `(captures / legendaries)` with `(summon items)` would preserve exactly the pattern this chunk retracts (IR flavor leaking into the uniqueness-invariant statement) and create two sources of truth for what a ReplicaItem is.
- **Line 342** — full rewrite (preserves SPEC's "kind discriminant" framing — only the discriminator axis moves):
  - Before: `**ReplicaItem** — IR type for items that summon a Pokemon as a unit. Two kinds: **Capture** (one-shot, mid-fight, via ball-style item) and **Legendary** (persistent ally with spell). Both share the same IR struct with a kind discriminant; "capturable" and "legendary" are *kinds*, not separate IR types.`
  - After: `**ReplicaItem** — IR type for items that summon a Pokemon as a monster; on defeat, the Pokemon joins the team for the rest of the run. Every ReplicaItem comes from an entry inside an itempool.((...)) modifier and shares the same summon → defeat → team-join pipeline. Discriminated by `SummonTrigger`: **SideUse** (player uses a thief-side; dice live either on an outer flat preface — e.g. Poke Ball — or inside the wrapper — e.g. Master Ball? — captured by the `dice_location` sub-discriminator) and **Cast** (equipping grants a spell via cast.sthief.abilitydata; e.g. Silver Wing). All trigger variants carry identical summon/defeat/team-join payload; the discriminator sits on SummonTrigger, not on a container-position enum.`
- **Line 343** — delete the dangling Capturable/Legendary glossary entry:
  - Before: `**Capturable / Legendary** — Kinds of `ReplicaItem` (see above). User-facing vocabulary only.`
  - After:  *(line removed entirely; "Capturable" and "Legendary" survive only as game-flavor copy-paste UX text — covered by the "Derived/downstream mentions" bullet)*
- **Derived/downstream mentions** — grep `SPEC.md` for "Capture" / "Legendary" / "capturable" (singular and plural, upper- and lower-case) referring to replica kinds and update; keep the words where they refer to game-mechanic flavor (copy-paste UX text), not IR variants. Run `rg -ni 'capture|legendary|legendaries|capturable' SPEC.md` (case-insensitive, covers singular+plural) and adjudicate each hit explicitly. Round 2 audit enumerates the current hits (verify at impl time — line numbers drift):
  - **line 78** (`classifier → type parsers (hero/capture/monster/`) — IR pipeline diagram. Rewrite: `classifier → type parsers (hero/replica_item/monster/...`).
  - **line 104** (`heroes → items → replica items (captures, legendaries)`) — IR pipeline list. Rewrite: drop the parenthetical (same rationale as §6.3 line 246).
  - **line 168** (`**Lossless across all types**: heroes, replica items (captures, legendaries), monsters, ...`) — roundtrip invariant. Rewrite: drop the parenthetical.
  - **line 188** (API surface comment `// ... same for Monster, Boss, ReplicaItem (captures and legendaries are both ReplicaItem kinds)`) — rewrite to: `// ... same for Monster, Boss, ReplicaItem (SummonTrigger::{SideUse, Cast} are both ReplicaItem trigger variants)`.
  - **line 246** — §6.3 edit already specified above.
  - **line 335** (`**Modifier** — One comma-separated entry in a textmod (one hero, capture, monster, structural, etc.)`) — rewrite to: `(one hero, replica_item, monster, structural, etc.)`.
  - **line 342-343** — full rewrite already specified above.
  IR-variant references rewrite; game-flavor references stay (none currently identified — all 7 hits above are IR/pipeline/invariant prose). Implementation must list each hit with the verdict in the PR description. A narrower grep that omits plural/lowercase forms (e.g. `rg '\bCapture\b|\bLegendary\b'`) silently leaves lines 78/104/168/188/335 stale — DO NOT use it.

### 3.7 Plan-layer updates (`plans/PLATFORM_FOUNDATIONS_PLAN.md`)

- **§F7 rewrite (mandatory).** `rg -n 'ReplicaItemContainer\|ReplicaItemKind\|ReplicaItem.container' plans/PLATFORM_FOUNDATIONS_PLAN.md` surfaces every site that normatively describes the retired enum; each must be updated to the trigger-based IR shape in §3.2. The chunk does not land with stale §F language — the foundations plan is a persona/architecture authority, not a changelog.
- **Dependency-graph + Parallel Execution Map edits — concrete line numbers.** At implementation start, re-grep for the exact strings below because line numbers in `PLATFORM_FOUNDATIONS_PLAN.md` drift with unrelated edits; the replacement contract is by string, not by line. All five sites must land in the same PR as this chunk:
  - **Dependency graph ASCII tree** (currently near lines 480–492). Two edits, not one:
    1. Move Chunk 8 from a sibling of Chunk 9 under Chunk 6 to a **child of Chunk 9** (since Chunks 5/7/9 are upstream of Chunk 8 per §2). Post-edit shape under Chunk 6 reads:
       ```
       └── Chunk 6 (…) ✅ COMPLETE
             ├── Chunk 5 (…) ✅ MERGED-FIRST
             └── Chunk 9 (replica-parser chain-and-depth-aware scalar extraction) ✅ MERGED-FIRST
                   └── Chunk 8 (ReplicaItem trigger redesign + itempool extraction + derived.rs migration patch + V020/X003 3-bucket collapse) [needs 4, 5, 6, 7, 9 — ships as 8a → {8b ∥ 8c}]
       ```
    2. Replace the Chunk 8 label text itself: `Chunk 8 (V020 restructure — remove cross-bucket Pokemon overlap) [needs both 4 and 6]` → `Chunk 8 (ReplicaItem trigger redesign + itempool extraction + derived.rs migration patch + V020/X003 3-bucket collapse) [needs 4, 5, 6, 7, 9 — ships as 8a → {8b ∥ 8c}]`.
    The new Chunk 5/7/9 → Chunk 8 edges reflect the merge-order pre-condition in §2.
  - **Narrative parallel bullet** (currently near line 498). Replace `Chunk 8 only touches xref.rs; Chunk 5 writes ir/merge.rs / builder/derived.rs / builder/mod.rs; Chunk 7 writes extractor lib-code + post-3c hero_emitter.rs. No overlap.` with a bullet that enumerates Chunk 8's full file list verbatim from §4 of this plan: `ir/mod.rs, extractor/{classifier,replica_item_parser,structural_parser,mod}.rs, builder/{replica_item_emitter,structural_emitter,mod}.rs, util.rs, authoring/{replica_item,mod}.rs, xref.rs, ir/ops.rs, tests/{build_options,roundtrip_baseline,correctness,integration,retirements}_tests.rs, SPEC.md, PLATFORM_FOUNDATIONS_PLAN.md, and .claude/settings.json.` Check for overlap against Chunk 5's and Chunk 7's file lists; if the grep finds any shared file, reconcile before landing.
  - **Wall-clock rounds** (currently near line 502). Any `{5, 7, 8, 9 parallel — no shared files once 3c/4/6 land}` clause is false — Chunks 5/7/9 land before Chunk 8 (per §2). Replace with `{5, 7, 9 parallel after 4/6 land; 8 after 5, 7, 9}`.
  - **Chunk 8 block header** (currently near line 811). Replace `### Chunk 8: V020 restructure — remove cross-bucket Pokemon overlap with X003 [serial after Chunks 4 and 6; parallel with 5 and 7]` with `### Chunk 8: ReplicaItem trigger redesign — trigger-based IR, itempool extraction, derived.rs migration patch, retire unevidenced variants [serial after Chunks 4, 5, 6, 7, 9 — all upstream]`. Body is rewritten per §3 of this plan.
  - **§F9 success criterion**. The `{hero, replica_item, monster}` canonical-set wording must read bucket-by-name with no subdivision. The verification grep `rg -n '\{hero,\s*replica_item,\s*monster\}' plans/PLATFORM_FOUNDATIONS_PLAN.md` must return at least one hit (the line 901 `(`{hero, replica_item, monster}` slice is X003's sole territory)` already satisfies it; do not add a variant-subdivided sibling).
  - **Chunk 9 block's "Parallel with" claim** (currently near line 853 — line numbers drift, anchor by string). Replace `**Parallel with**: Chunks 5, 7, 8 (no shared files).` with `**Parallel with**: Chunks 5 and 7 (no shared files). **Chunk 8 is NOT parallel** — it depends on Chunk 9's helper; see Chunk 8's §2 and §9.0.` This keeps the Chunk 9 block's internal claim consistent with the dep-graph rewrite in the Parallel-Execution-Map section above.
  - **Chunk 9 block's merge-ordering line** (immediately below the "Parallel with" line). Replace `**Merge ordering**: merges after 6. No constraint against 5, 7, 8.` with `**Merge ordering**: merges after 6, **before 8**. No constraint against 5 or 7.` Chunk 9 must land before Chunk 8 (or as Chunk 8's §3.0 first commit) so the `util::slice_before_chain_and_cast` + `depth_aware` helper is in-tree when Chunk 8 §3.3 ships.
  - **Chunk 5 SHAPE dependency edge** (NEW — must land in the dep-graph in the same PR). Chunk 5's `generate_hero_item_pool` references `ReplicaItem` shape; if Chunk 5 ships first against `main` (where Chunk 6's `ReplicaItemContainer` is live), it MUST be implemented against the post-Chunk-8 IR shape (otherwise it breaks when Chunk 8 lands and deletes the enum). Add to the dep-graph narrative: `Chunk 5 cannot merge before Chunk 8 unless Chunk 5's `generate_hero_item_pool` is implemented against the post-Chunk-8 IR shape (SummonTrigger + target_pokemon — not yet on main). Either: (a) hold Chunk 5 until Chunk 8 merges, or (b) Chunk 5 ships against the current container enum and Chunk 8 includes a derived.rs migration patch. Pick one; "either way works" is not an ordering rule.`
- **Chunk 5 block — routing-key substitution + actual migration patch** (currently near lines 738 and 748). Chunk 5 ships first against `main` keyed on `match item.container { Capture { name } => ... }` (the pre-Chunk-8 IR shape); Chunk 8 retires that enum and **owns the migration patch to `compiler/src/builder/derived.rs::generate_hero_item_pool`** + the rename of Chunk 5's test (see §2 dependencies and §4's `derived.rs` row). The function's purpose (regenerate hero-bound ItemPool modifiers from IR) is unchanged — only the routing key moves to the new IR shape. Two edits in the Chunk 5 block of the foundations plan to reflect the post-Chunk-8 spec:
  1. **Requirements bullet** (line 738). Replace `` `generate_hero_item_pool` matches on each `ReplicaItem.container` — `Capture { name }` routes the item into the hero's pool keyed by `name`; `Legendary` is skipped for hero-bound pools (legendaries have their own emission path). `` with `` `generate_hero_item_pool` matches on each `ReplicaItem.trigger` — both `SummonTrigger::SideUse` variants (`OuterPreface` and `InnerWrapper` dice-locations) route the item into the hero's pool keyed by `ReplicaItem.target_pokemon`; `Cast` is skipped for hero-bound pools (the cast trigger has its own emission path per §3.4 of the Chunk 8 plan). The `dice_location` sub-axis is irrelevant for routing — both source-shapes share the SideUse player-action and therefore both belong in the hero's pool. ``
  2. **Verification bullet** (line 748). Rename the test from `derived::hero_item_pool_matches_sliceymon_via_container_enum` to `derived::hero_item_pool_matches_sliceymon_via_trigger` and rewrite its gloss: `` `generate_hero_item_pool` uses `matches!(item.trigger, SummonTrigger::SideUse { .. })` (both dice-locations) + `target_pokemon` to bucket items; byte-matches hero-bound ItemPool in sliceymon. ``
  Chunk 5 lands first against `main` (per user-confirmed merge order); Chunk 8 owns the actual code migration in `builder/derived.rs` (see §2 and §4). The foundations-plan amendment above is the final-state spec; Chunk 5's already-shipped code keys on `container`, and Chunk 8's `derived.rs` patch is the rekey. No further coordination required.
- **Chunk 6 block** — Chunk 6 demonstrably shipped `ReplicaItemContainer` (foundations plan line 754: `✅ COMPLETE (2026-04-21)`); rewriting the entry to claim it shipped the new shape would be false. Allowed wording: keep the historical Chunk 6 entry truthful about what landed (the `Capture`/`Legendary` enum), and append a one-line forward note: `**Superseded by Chunk 8**: ReplicaItemContainer is replaced by SummonTrigger (SideUse with DiceLocation sub-axis + Cast); container_name moves to a plain String.` Do not delete Chunk 6's history; do not pretend it shipped the new shape; do use "Superseded by Chunk 8" as the forward pointer.
- **Chunk 8 entry** — rewrite in full to match §3 of this plan (trigger-based IR, itempool extraction, V020/X003 3-bucket collapse). Overwrite in place; do not leave "superseded" / "was" / "historical" wording around the text being replaced.
- **§F9 success-criteria** — ensure bucket-set wording reads `{hero, replica_item, monster}`, named once, referenced by name downstream.
- **New "Lessons" entry (sibling to Chunk 3b's entry)**:
  > **Chunk 8 — IR variants must be corpus-evidenced.** `ReplicaItemContainer::{Capture, Legendary}` and `ModifierType::Legendary` have zero corpus instances: all four working mods contain zero top-level `item.*` modifiers (the shape `parse_legendary` gates on), and every Pokemon-summon item lives inside `itempool.((…))` where the extractor currently keeps them opaque (`roundtrip_diag` reports `Replicas ir1=0` for all four). Chunk 8 retires those variants and roots the discriminator on `SummonTrigger` (`SideUse { dice_location: OuterPreface | InnerWrapper }` / `Cast`) — the axis every corpus summon item actually exhibits.
  >
  > **Takeaway:** before an IR variant discriminator ships, grep the corpus for an instance of each variant. Zero instances for a variant means the variant is a hypothesis, not a model — do not land it. A rule authored against an unevidenced variant (like the abandoned V020 split) compounds the defect. Prose in SPEC/plan is not evidence; `rg` output is.

**Hook update (the current PreToolUse hook does NOT yet encode this lesson — Step 0 audit confirmed three rules: authority diff, source-vs-IR divergence, structural smells).** Add a fourth bullet to `.claude/settings.json`'s `additionalContext` (escape quotes/newlines as the existing hook does):

> - Before an IR variant discriminator ships, grep the corpus for an instance of each variant. Zero `rg` hits for a variant means the variant is a hypothesis, not a model — do not land it. Rules authored against unevidenced variants compound the defect.

Add the hook update to this chunk's §4 file-touch table (`.claude/settings.json`) so it ships with the code change it codifies.

## 4. Files touched

| File | Change |
|---|---|
| `compiler/src/ir/mod.rs` | Delete `ReplicaItemContainer` enum. Delete `pub struct ItemPoolEntry` (line 934). Replace `ReplicaItem` struct per §3.2 (delete `name`/`template`/`sticker` fields; add `container_name`, `target_pokemon`, `trigger`, `enemy_template`, `team_template`, `sticker_stack: Option<ModifierChain>`). Add `SummonTrigger` enum (Option B: `SideUse { dice, dice_location }` + `Cast { dice }`) and `DiceLocation` enum. Replace `StructuralContent::ItemPool { body, items: Vec<ItemPoolEntry> }` (line 801-803) with `StructuralContent::ItemPool { items: Vec<ItempoolItem> }` (no body field; pure-IR rebuild). Add `ItempoolItem` enum: `Summon(usize)` \| `NonSummon { name, tier, content }`. Update `#[cfg(test)]` helpers. |
| `compiler/src/extractor/classifier.rs` | Delete `ModifierType::Legendary` variant + gate. Delete `ModifierType::ReplicaItem` and `ModifierType::ReplicaItemWithAbility` variants (per §3.1). `itempool.` routing unchanged. Add a typed `Err(CompilerError::classify(...))` path for any remaining top-level `item.` modifier, with a suggestion pointing at `itempool.((item…))` — do not silently fall through to later gates. |
| `compiler/src/extractor/replica_item_parser.rs` | Full rewrite per §3.3. New `extract_from_itempool` entry point returning `Result<ItempoolExtraction, CompilerError>` carrying `new_replica_items: Vec<ReplicaItem>` + `items: Vec<ItempoolItem>`. No source-byte ranges, no opaque `body` string — pure-IR rebuild. `parse_legendary` / `parse_simple` / `parse_with_ability` deleted. |
| `compiler/src/extractor/mod.rs` | Route `ItemPool` modifiers through `replica_item_parser::extract_from_itempool`, append the returned `new_replica_items` to `ModIR.replica_items`, store the typed `items: Vec<ItempoolItem>` on the `StructuralContent::ItemPool` entry. **Also delete the dispatch arms for `ModifierType::{ReplicaItem, ReplicaItemWithAbility, Legendary}` (lines 59-67 today)** — the variants are gone; non-exhaustive match is a compile error. The only remaining replica-item entry point is `ModifierType::ItemPool`. |
| `compiler/src/extractor/structural_parser.rs` | Delete the existing `ItemPoolEntry` construction loop (line 96 today, `use crate::ir::{ItemPoolEntry, ...}` at line 1). The new `ItempoolItem` list is populated by `extract_from_itempool` (called from `extractor/mod.rs`); `structural_parser` no longer owns itempool entry parsing. |
| `compiler/src/builder/replica_item_emitter.rs` | Full rewrite per §3.4. `emit_replica_item` emits a single summon entry (trigger-shape + shared payload). NEW `emit_itempool(items, replica_items, pool_name)` reconstructs the entire itempool body from typed IR — no source-byte splice. The literal template string at emission sites is `"Thief"` (capital). **Delete both Legendary roundtrip tests**: `legendary_emit_parse_roundtrip_with_all_fields` (line 315 today) and `legendary_emit_parse_roundtrip_with_item_modifiers` (line 369 today). Both import `crate::extractor::replica_item_parser::parse_legendary` (lines 323 and 375 today); the function is deleted in §3.1, so the tests cannot compile. They pin a retired concept (Legendary-shape emit/parse parity on a `ReplicaItemContainer::Legendary` discriminator) and have no replacement — the new tests at T1–T5a exercise the trigger-based shape via `extract_from_itempool` → `emit_itempool` round-trips. |
| `compiler/src/builder/structural_emitter.rs` | For `StructuralContent::ItemPool { items }`, call `emit_itempool(items, &ir.replica_items, pool_name)` from `replica_item_emitter` to regenerate the body. No byte-range bookkeeping; the previous body-clone passthrough for ItemPool is replaced by the typed call. |
| `compiler/src/builder/derived.rs` | **Migration patch for Chunk 5's `generate_hero_item_pool`** (Chunk 5 lands first against `main` keyed on `match item.container { Capture { name } => ... }`; Chunk 8 retires that enum). Rewrite the routing match to `matches!(item.trigger, SummonTrigger::SideUse { .. })` (both dice-locations route into the hero pool — see §3.7's Chunk 5 amendment). Replace every `item.name` reference with `item.target_pokemon`. Rename test `derived::hero_item_pool_matches_sliceymon_via_container_enum` → `derived::hero_item_pool_matches_sliceymon_via_trigger`; rewrite its assertion against the new IR shape but keep the byte-equality target unchanged (sliceymon hero-bound ItemPool source bytes). |
| `compiler/src/util.rs` | Add `slice_before_chain_and_cast` + `depth_aware` on `extract_color` per §3.0 if Chunk 9 has not landed. (No-op if Chunk 9 is already merged.) |
| `compiler/src/authoring/replica_item.rs` (new) | Typed builder for `ReplicaItem` with separate `sideuse_builder` / `cast_builder` entry points per §5 T24–T26. Compile-error guards on missing trigger data. |
| `compiler/src/authoring/mod.rs` | `pub mod replica_item;` + `pub use replica_item::{…};` re-exports. |
| `.claude/settings.json` | Add fourth bullet to `PreToolUse` hook encoding the "zero-corpus-hits → hypothesis, not model" rule per §3.7. |
| `compiler/src/builder/mod.rs` | Builder dispatch updated for the new emitter shape. |
| `compiler/src/xref.rs` | Per §3.5. **First commit: widen `Finding` struct** — add `pub buckets: Vec<&'static str>` and `pub includes_boss: bool` with `#[serde(default)]`. Audit every Finding constructor (V016/V019/V020/X003/X016/X017 etc.) for default-init compatibility. Then: X003 + V020 predicates on the 3-bucket set; line 229-231 routing match deleted; lines 234/236/337/528-529 switch from `item.name` to `item.target_pokemon`; line 339 `iter_dice_faces` template arg switches from `item.template.as_str()` to literal `"thief"`; line 279 suggestion string updates; **delete `xref::x003_distinguishes_capture_from_legendary_buckets` (line ~1015) and any sibling test asserting on `"capture"` / `"legendary"` substrings (run `rg -n '"capture"\|"legendary"' compiler/src/xref.rs` to enumerate)**; full test suite per §3.5 test list. |
| `compiler/src/ir/ops.rs` | `add_replica_item` re-tested against new struct. Container-kind CRUD assertions deleted. **Rewrite `remove_replica_item_by_name` to re-index `ItempoolItem::Summon(i)` entries across all `StructuralContent::ItemPool` items** per the §3.4 index-stability invariant: after locating the removed index `j`, traverse every `StructuralContent::ItemPool { items }`; for each `Summon(i)`, drop the entry when `i == j`, decrement when `i > j`. New test `ops::remove_replica_item_reindexes_itempool_summons` (T28) pins the invariant. **Replace the `make_replica_item` test helper** (line 229 today, currently builds `ReplicaItemContainer::Capture { name }`) to construct the new shape: `SummonTrigger::SideUse { dice, dice_location: OuterPreface }` + `target_pokemon = name` + `container_name = "Test Ball"` + the rest of the new fields with sane defaults — keeps the 5+ ops.rs tests compiling. Switch `find_name_category(&item.name)` (line 90) to `find_name_category(&item.target_pokemon)`. Also update `add_replica_item`'s error-constructor sites at lines 91-95 (`item.name.clone()` in the `duplicate_name` error) to `item.target_pokemon.clone()`. **Remove `ReplicaItemContainer` from the `use crate::ir::{DiceFaces, HeroBlock, HeroFormat, ReplicaItemContainer};` import at `ops.rs:167`** — the enum is deleted in §3.1; leaving the import is a compile error. Replace with the new trigger imports (`SummonTrigger`, `DiceLocation`, `ItempoolItem`, `StructuralContent`) as needed by `make_replica_item` and the re-index routine. |
| `compiler/tests/build_options_tests.rs` | `v020_cross_category_source_is_global` stays on hero+boss; any construction-site assertions referencing `ReplicaItemContainer` move to trigger-based shape (`SummonTrigger::SideUse { dice, dice_location: OuterPreface }`). The current `use textmod_compiler::ir::{ReplicaItem, ReplicaItemContainer};` (line ~186) loses `ReplicaItemContainer`; replace with `SummonTrigger`. |
| `compiler/tests/roundtrip_baseline.rs` | Baselines for all four working mods re-generated. Baseline file format is `replica_items.count: N -> M` (written to `tests/baselines/roundtrip/<mod>.baseline`) — NOT `Replicas ir1=N` (that string is `roundtrip_diag` example output, distinct from this test). Sliceymon's baseline gains `replica_items.count: 23 -> 23` (**23** summon items: 18 SideUse{OuterPreface} + 1 SideUse{InnerWrapper} (Master Ball?) + 4 Cast, across Pokeballs Parts 1+2 and Summons Parts 1+2 — verify by paren-depth walk at implementation, not `rg` count). pansaer/punpuns/community expected `replica_items.count: 0 -> 0`. |
| `compiler/tests/correctness_tests.rs` | If any test asserts `parse_legendary` or `ReplicaItemContainer::Legendary` reachability, delete. |
| `compiler/tests/integration_tests.rs` | `:412` has the comment `` `ModifierType::ReplicaItem` isn't produced by the top-level classifier yet `` — stale after §3.1's variant retirement (`ModifierType::ReplicaItem` + `ReplicaItemWithAbility` deleted). Remove the comment and any associated gated test body; if the test asserted that a specific classification produces `ReplicaItem`, rewrite it to assert the new `ItemPool → extract_from_itempool` route. Audit the rest of the file with `rg 'Legendary\|ReplicaItemContainer\|ReplicaItem\b' compiler/tests/integration_tests.rs` before landing. |
| `compiler/tests/retirements.rs` (new) | T12–T14 + T12a retirement-greps per §5. `std::fs` + static search; no `build.rs` coupling. |
| `compiler/examples/roundtrip_diag.rs` | No code change expected, but the reported `Replicas ir1=XX` stdout line will now be non-zero for sliceymon. Optionally add a per-entry trigger breakdown (`SideUse{OuterPreface}=18 SideUse{InnerWrapper}=1 Cast=4` for sliceymon). |
| `compiler/examples/drift_audit.rs` | Verify drift-audit still passes; add Pokemon-summon drift class if relevant. |
| `SPEC.md` | Per §3.6. |
| `plans/PLATFORM_FOUNDATIONS_PLAN.md` | Per §3.7. |

## 5. Verification — shipped tests

Numbered for reference; each must land with the chunk.

**Test-file placement contract.** Rust integration tests in `compiler/tests/*.rs` are **flat** — each file is its own crate, and a name like `extractor::foo` is a module path only valid for `#[cfg(test)]` blocks inside `compiler/src/**`, not for integration-test files. This plan's test names use `extractor::…`, `xref::…`, `authoring::…` as **semantic prefixes**; they map to actual Rust test functions as follows:

| Semantic prefix | File | Rust function name |
|---|---|---|
| `extractor::`  | `compiler/src/extractor/replica_item_parser.rs` under `#[cfg(test)] mod tests`                | use the bare name, e.g. `itempool_summon_entry_sideuse_roundtrips_ivysaur` |
| `xref::`       | `compiler/src/xref.rs` under `#[cfg(test)] mod tests`                                          | bare name, e.g. `x003_one_replica_bucket_across_triggers` |
| `authoring::`  | `compiler/src/authoring/replica_item.rs` under `#[cfg(test)] mod tests`                        | bare name, e.g. `replica_item_builder_sideuse` |
| `retirements::`| `compiler/tests/retirements.rs` (new integration-test file, flat functions, no module prefix)  | bare name, e.g. `grep_crate_for_replica_item_container_enum` |
| `roundtrip_baseline::` | `compiler/tests/roundtrip_baseline.rs` (existing flat file)                             | bare name |

Tests that exercise corpus round-trip (T1–T5a, T22–T23a) live inline in-module where extract/build are called; retirement greps (T12–T14 + T12a) live in the new flat integration file; authoring builder tests (T24–T26) live in-module next to the builder. No test needs to be renamed — the `::` prefix is only a reading aid.

**Source-vs-IR roundtrip**
- [ ] T1. `extractor::itempool_summon_entry_sideuse_outer_roundtrips_ivysaur` — feed the Ivysaur entry slice from sliceymon Pokeballs Part 1 (`SideUse{OuterPreface}`, Great Ball) through extract → build → assert byte-equal input vs output. Also assert `item.target_pokemon == "Ivysaur"` and `item.enemy_template == "Wolf"` (capital — case-preserving).
- [ ] T2. `extractor::itempool_summon_entry_cast_roundtrips_silver_wing` — same for the Silver Wing entry in sliceymon Summons Part 1. Name the test by container, not by target Pokemon (Silver Wing summons Lugia — a `_lugia` test name would conflate target with container).
- [ ] T2a. `extractor::itempool_summon_entry_sideuse_inner_roundtrips_master_ball_question` — same for the `Master Ball?` entry (container name contains trailing `?`; target `Arceus`; `enemy_template = "Dragon"` (capital); `team_template = "Housecat"` (capital); dice = `34-10:34-10:34-8:34-8:34-5:34-5`). Ensures the `SideUse{InnerWrapper}` classifier path is exercised end-to-end with byte-equality on the `?`. Assertion includes `matches!(item.trigger, SummonTrigger::SideUse { dice_location: DiceLocation::InnerWrapper, .. })`.
- [ ] T2b. `extractor::sticker_chain_roundtrips_within_summon_entry` — at least one sliceymon summon entry containing a `.sticker.` segment must extract through `ModifierChain::parse` and emit byte-equal via `chain.emit()`. The Caterpie SideUse entry inside Pokeballs Part 1 carries `.i.mid.sticker.(right.hat.statue)#togfri` — use it. Validates the §3.1 `sticker → sticker_stack` rename + `Option<String> → Option<ModifierChain>` type change. If the chain parser cannot round-trip a corpus sticker chain byte-equal, the type change must be deferred and `sticker: Option<String>` stays.
- [ ] T3. `extractor::itempool_full_pool_roundtrips_pokeballs_part_1` — full Pokeballs Part 1 modifier (**11 entries: 10 `SideUse{OuterPreface}` balls (including `Master Ball` no-`?` summoning Mewtwo) + 1 `SideUse{InnerWrapper}` `Master Ball?`**, in source order). Byte-equal. Also asserts the `Master Ball` (no `?`) and `Master Ball?` (with `?`) both extract as DISTINCT entries with different `container_name` strings, different `target_pokemon` (Mewtwo vs Arceus), and different `dice_location` values.
- [ ] T4. `extractor::itempool_full_pool_roundtrips_pokeballs_part_2` — full Pokeballs Part 2 modifier (**8 entries**, all `SideUse{OuterPreface}`). Byte-equal.
- [ ] T5. `extractor::itempool_full_pool_roundtrips_summons_part_1` — **2 entries**: Rainbow Wing + Silver Wing (both `Cast`). Byte-equal.
- [ ] T5a. `extractor::itempool_full_pool_roundtrips_summons_part_2` — **2 entries**: Blue Orb + Red Orb (both `Cast`; Red Orb uses `team_template = "Statue"` for Groudon). Byte-equal. Summons Part 1 and Part 2 are separate modifiers; both pools must have their own round-trip test so a drift in either is caught directly.
- [ ] T6. `extractor::non_summon_entry_stays_structural` — an itempool entry that is a TM / accessory / other orb (no `hat.egg` + `vase.(add.((replica.…)))` pair) becomes `ItempoolItem::NonSummon { name, tier, content }` and round-trips byte-equal via `emit_itempool`. Exercise with sliceymon's accessory pool (the entry count is to be verified at implementation time via paren-depth-0 `+` walk — do NOT assert a hardcoded count; expected 0 `ReplicaItem` output regardless of count).
- [ ] T7. `extractor::half_summon_entry_stays_structural` — synthesize an entry with `hat.egg` but no matching `vase.(add.((replica.<same-pokemon>…)))` (or with mismatched Pokemon names). Becomes `ItempoolItem::NonSummon`, not `ItempoolItem::Summon`. Proves the detector is conjunctive, not disjunctive.

**Trigger classification (corpus-complete)**
- [ ] T8. `extractor::all_sideuse_outer_entries_classify_as_outer_preface` — iterate every summon entry in sliceymon whose outer wrapper is `hat.replica.Thief.n.<Pokemon>.sd.<faces>` (**exact count verified at implementation time via §1.1 regex**; expected 18 — one per distinct `hat.replica.Thief.n.<Pokemon>` occurrence; `Master Ball?` is NOT one of them and is T9a's target; `Master Ball` no-`?` IS one of the 18). Each → `SummonTrigger::SideUse { dice_location: DiceLocation::OuterPreface, .. }`.
- [ ] T9. `extractor::all_cast_entries_classify_as_cast` — iterate every summon entry whose outer wrapper contains `cast.sthief.abilitydata.(thief.sd.<faces>…)` (expected 4: Rainbow Wing, Silver Wing, Blue Orb, Red Orb; **must not** match `Evasion Orb` / `Itemizer Orb` / `Foe Seal Orb` / `Two Edge Orb` — those are non-summon items per §1.2). Each → `SummonTrigger::Cast`.
- [ ] T9a. `extractor::master_ball_question_classifies_as_inner_wrapper` — **exact target**: the single in-scope `Master Ball?` entry in sliceymon Pokeballs Part 1 (byte ~274161, NOT the boss-block hit at byte ~106078). Classifier → `SummonTrigger::SideUse { dice, dice_location: DiceLocation::InnerWrapper }` with `dice = 34-10:34-10:34-8:34-8:34-5:34-5` extracted from the `.i.(hat.Thief.sd.<faces>)` inside the wrapper (NOT from any outer preface, which is absent). Fails if classifier silently routes to `OuterPreface` with zero dice.
- [ ] T9b. `extractor::inner_wrapper_count_is_corpus_exact` — across all four working mods, `ir.replica_items.iter().filter(|r| matches!(r.trigger, SummonTrigger::SideUse { dice_location: DiceLocation::InnerWrapper, .. })).count() == 1`. The single in-scope hit is the sliceymon Pokeballs Part 1 `Master Ball?`; the boss-block `Master Ball?` at byte ~106078 must NOT surface (proves §3.3 rule 1 itempool-scoping is enforced).
- [ ] T10. `extractor::trigger_classification_reads_inner_payload_not_name` — **source-vs-IR** test. Synthetic input: itempool entry whose container name is `Cast-Iron Ball` (lexically resembles both "cast" and "Ball") but whose inner trigger shape is `SideUse{OuterPreface}`. Classification yields `SummonTrigger::SideUse { dice_location: OuterPreface, .. }`. Second synthetic: container named `Silver Wing Deluxe Ball` but inner shape is Cast → classifier yields `SummonTrigger::Cast`. Proves classification reads payload bytes, not surface strings — this is the test the Step 0 hook demands (an IR-equality-only roundtrip would pass even if the classifier reached for container-name heuristics).
- [ ] T10a. `extractor::target_pokemon_preserves_source_bytes_exactly` — **source-vs-IR** test. Use the actual `Ho Oh` entry from sliceymon (capital H, embedded space — verified via `rg 'Ho Oh' working-mods/sliceymon.txt`); assert `replica_item.target_pokemon == "Ho Oh"` exactly — **not** `"Ho-Oh"`, `"hooh"`, `"HoOh"`, or any normalized form. Second synthetic with trailing whitespace (`n.Anorith `, note trailing space — also corpus-evidenced) — assert extraction preserves the space and emission round-trips it. Fails loudly if any implementer reaches for a Pokemon registry, case-title normalizer, or hyphen canonicalizer. Covers the same class of drift as T10 but targets `target_pokemon` specifically, which is the xref/ops identity key per §3.2's identity contract.
- [ ] T11. `extractor::trigger_classification_is_total_per_entry` — not a tautology over the Rust exhaustive match; asserts concrete per-entry classification: (a) all 18 balls in Pokeballs Parts 1+2 (Poke, Great, Ultra, Fast, Friend, Heavy, Level, Lure, Moon, Master-no-`?`, Dive, Luxury, Nest, Premier, Timer, Dusk, Quick, Dream) classify as `SummonTrigger::SideUse { dice_location: DiceLocation::OuterPreface, .. }`; (b) `Master Ball?` (trailing `?`) classifies as `SummonTrigger::SideUse { dice_location: DiceLocation::InnerWrapper, .. }`; (c) all 4 Summons (Rainbow Wing, Silver Wing, Blue Orb, Red Orb) classify as `SummonTrigger::Cast { .. }`. Assertion is exact — a test that covers "at least one of each" is insufficient because it leaves silent mis-classification of specific corpus entries undetected.
- [ ] T9c. `extractor::unclassifiable_wrapper_shape_demotes_to_nonsummon_with_finding` — synthetic itempool entry with valid `hat.egg.<enemy>.n.Zubat` + matching `vase.(add.((replica.<team>.n.Zubat)))` pair (rules (a)+(b)+(c) pass) but wrapper contains NEITHER outer preface, NOR `cast.sthief.abilitydata.`, NOR inner `.i.(hat.Thief.sd.<faces>)` — an unmodeled wrapper shape. Extraction: zero `ReplicaItem` produced (no default variant), a single `Finding { code: "W-REPLICA-TRIGGER-UNCLASSIFIED", severity: Severity::Warning, .. }` surfaces, and the entry is demoted to `ItempoolItem::NonSummon` that round-trips byte-equal. Proves the fallback is permissive (SPEC §3.3), not a hard `CompilerError::classify`.

**Retirements verified**
- [ ] T12. `grep_crate_for_replica_item_container_enum` — `rg "ReplicaItemContainer"` across `compiler/src/` and `compiler/tests/` yields zero hits.
- [ ] T13. `grep_crate_for_parse_legendary` — `rg "parse_legendary"` yields zero hits.
- [ ] T14. `grep_crate_for_modifier_type_legendary` — `rg "ModifierType::Legendary"` yields zero hits.

(Tests T12–T14 **must** live in `compiler/tests/retirements.rs` as integration tests using `std::fs` + a small static search. **`build.rs` tree-grep is forbidden** — `compiler/build.rs` is part of the library build graph and would couple `cargo build` success to retirement-absence; that drifts the WASM build surface (the persona's Phase 4 rule against `std::fs` in library code) and turns a test assertion into a build-time failure mode. Integration tests are the right place: `std::fs` is allowed there, and the failure surfaces as a test failure, not a build failure.)

- [ ] T12a. `grep_crate_for_modifier_type_replica_item_variants` — `rg "ModifierType::ReplicaItem(?:WithAbility)?\b" compiler/src/ compiler/tests/` yields zero hits. Covers the §3.1 variant retirements that T14 does not (T14 only checks `ModifierType::Legendary`). Lives alongside T12–T14 in `compiler/tests/retirements.rs`.

**Xref 3-bucket shape**
- [ ] T15. `xref::x003_one_replica_bucket_across_triggers` — per §3.5. Exercises all three combinations (`SideUse{OuterPreface}`, `SideUse{InnerWrapper}`, `Cast`) sharing one Pokemon name. Asserts `x003_finding.buckets == vec!["hero", "replica_item"]`.
- [ ] T16. `xref::x003_treats_all_trigger_variants_as_one_bucket` — per §3.5. Must include all three combinations in the test body (`SideUse{OuterPreface}`, `SideUse{InnerWrapper}`, `Cast`); omitting any leaves a combination's bucket-collapse silently unverified.
- [ ] T17. `xref::v020_silent_on_cross_bucket_pokemon_hero_replica` — `SideUse{OuterPreface}` replica is enough; the dice_location is irrelevant to bucketing.
- [ ] T18. Same for `hero_monster` / `replica_monster` / `case_insensitive` pairs.
- [ ] T19. `xref::v020_still_fires_on_boss_replica_collision` — one test (not split by trigger) covers the boss-vs-replica case. Asserts `v020_finding.includes_boss == true`.
- [ ] T20. `xref::v020_and_x003_coexist_when_collision_spans_boss_and_pokemon_buckets` — asserts the invariant via the typed Finding fields per §3.5 (V020's `includes_boss == true`; X003's `buckets.iter().all(|b| ["hero", "replica_item", "monster"].contains(b))`). Fails if either rule stops populating its typed field — do not test message-string substrings.
- [ ] T21. `xref::no_double_fire_on_working_mods` — re-run with the new Replica counts populated (sliceymon expected 23 per §1.1). Uses the same typed-field predicate from T20 as the sole permitted co-fire test; no message-string scan.

**Baseline regeneration**
- [ ] T22. `roundtrip_baseline` tests pass for all four mods with regenerated baselines. `Replicas` count is **23** for sliceymon (18 `SideUse{OuterPreface}` + 1 `SideUse{InnerWrapper}` + 4 `Cast` across Pokeballs Part 1 [11] + Pokeballs Part 2 [8] + Summons Part 1 [2] + Summons Part 2 [2]; verify by paren-depth walk at impl time) and **0** for pansaer / punpuns / community.
- [ ] T23. `cargo run --example roundtrip_diag` reports `Status: ROUNDTRIP OK` for all four mods and prints a per-trigger breakdown (`SideUse{OuterPreface}=18 SideUse{InnerWrapper}=1 Cast=4` for sliceymon; `0/0/0` for the others). Variant names must match the §3.2 Option-B enum exactly — the retired `OnWrapped` / `OnCast` labels are not in the IR.
- [ ] T23a. `no_false_positive_on_boss_hat_egg` — punpuns (13 lines / 19 occurrences of `hat.egg.`) and community (9 lines / 9 occurrences) contain `hat.egg.<template>` usages **inside boss/event hero blocks**, not itempool summon envelopes. The extractor must produce zero `ReplicaItem` entries for these two mods. Test fails if any match surfaces — proves the classification gate is `itempool. → extract_from_itempool` scoped, not a global `hat.egg.` scan.
- [ ] T27. `extractor::non_sliceymon_mods_produce_zero_replica_items` — run the full extractor on pansaer, punpuns, community and assert `ir.replica_items.len() == 0` for each. Covers pansaer's 7-occurrence `hat.egg.` (on its single monoline), punpuns' 19 `hat.egg.` + 3 `cast.sthief` occurrences, and community's 9 `hat.egg.` + 21 `itempool.` lines. The 3 `cast.sthief` occurrences in punpuns (inside `ph.b` boss abilitydata) must specifically classify as non-summon because they are not inside an `itempool.` modifier — failing this test means the classifier's route gate is too broad. (Plan §1.2 `rg -c` numbers are line counts; occurrence counts are the semantically meaningful value when the file is a single monoline, as pansaer is.)
- [ ] T28. `ops::remove_replica_item_reindexes_itempool_summons` — construct an IR with 3 replica_items (`A`, `B`, `C`) and a single `StructuralContent::ItemPool { items: vec![Summon(0), NonSummon { name: "TM1".into(), tier: Some(3), content: "…".into() }, Summon(2)] }`. Call `ir::ops::remove_replica_item_by_name("A")`. Assert: (i) `replica_items == [B, C]`; (ii) the pool's items is `vec![NonSummon { name: "TM1", .. }, Summon(1)]` — the `Summon(0)` entry is removed, and the `Summon(2)` is decremented to `Summon(1)` (B's new index). Second assertion: `remove_replica_item_by_name("B")` on the post-remove IR drops the last `Summon(1)`, leaving `items == [NonSummon{..}]` and `replica_items == [C]`. Guards the §3.4 index-stability invariant; without this test a regression that forgot to re-index silently corrupts build output (IR-equality would pass; build would emit wrong replicas or panic on out-of-bounds).

**Authoring-path sanity**
- [ ] T24. `authoring::replica_item_builder_sideuse` — typed constructor for a SideUse replica accepts `container_name`, `target_pokemon`, `enemy_template`, `team_template`, `sd`, etc. Compile error if `SummonTrigger` variant is omitted. Mirrors §6.1 strict/typed authoring path.
- [ ] T25. `authoring::replica_item_builder_cast` — typed constructor for a Cast replica requires `dice: DiceFaces` and rejects zero-faces at compile time where possible (builder carries `PhantomData<HasDice>` type-state flag; see existing SideUse builder at `authoring/replica_item.rs`). The builder does NOT accept an `abilitydata` field: Cast carries no AbilityData (corpus bodies have zero depth-0 `.n.<spell_name>` — see §3.2). Authoring must emit the outer `cast.sthief.abilitydata.(thief.sd.<CAST_SPELL_DICE>.i.(...))` preamble using the §3.4 emitter constants — builder exposes no knob for those universals.
- [ ] T26. `authoring::replica_item_emits_inside_itempool` — calling the builder then emitting via `emit_replica_item` (§3.4) produces a syntactically valid single summon entry (byte-equal to a hand-written reference). The test drives one entry through the structural-channel emitter to confirm it splices correctly into a synthetic `itempool.((…))` wrapper.

**(M4 fix) Source-vs-IR divergence tests — REQUIRED cohort for 8B (not just round-trip).** Existing T1–T26 prove `extract → emit` idempotence on raw bytes — they pass even if the emitter silently reads cached source bytes instead of typed IR fields. Add these source-vs-IR divergence tests:

- [ ] **T30b.** `emitter::non_summon_reads_typed_tier_field` — extract a non-summon entry whose source bytes contain `.tier.3`; assert the IR carries `ItempoolItem::NonSummon(NonSummonEntry::<variant> { trailer: NonSummonTrailer { tier: Some(3), .. } })`. Mutate the IR: set `tier = Some(7)`. Rebuild via `emit_itempool`. Assert the output contains `.tier.7` and does NOT contain `.tier.3`. Proves the emitter reads the typed field, not a cached source blob. Fails loudly if any emitter branch reaches for stored source bytes.
- [ ] **T30c.** `emitter::cast_summon_reads_typed_target_pokemon` — extract a Cast summon entry (e.g. Silver Wing → Lugia); assert the IR carries `ReplicaItem { target_pokemon: "Lugia", trigger: SummonTrigger::Cast { .. }, .. }`. Mutate the IR: set `target_pokemon = "Synthetic"`. Rebuild via `emit_replica_item`. Assert the output contains `.n.Synthetic.` at the `hat.egg.<enemy>.n.<target>` and `vase.(add.((replica.<team>.n.<target>` sites AND does NOT contain `Lugia` anywhere. Proves the Cast emission path reads the typed `target_pokemon` field everywhere it emits the Pokemon name — no cached-bytes copy-paste, no registry reach. Complements T2a's capture-preservation assertion with a mutation-based witness.

Both tests are 8B scope (they need 8B's real extractor to produce the IR they mutate). T30a is 8A.5's zero-Unclassified ratchet (per its §5.4). The lettered cohort (T30a / T30b / T30c) prevents the "generic T30 round-trip" oracle from masking source-vs-IR drift.

## 6. Structural check (per hook rules)

- **Collapses two paths with different invariants?** Yes, and this chunk **is** a SPEC amendment — §3.6 rewrites SPEC.md line 342 and §6.3 line 246 in the same PR. The hook rule ("A chunk that collapses two paths with different invariants … is a spec amendment masquerading as an implementation detail — stop and raise it") is honored by hoisting the SPEC edits into §3.6 explicitly, not hiding the mechanical relabel inside §3.2/§3.3.
- **N-line incantation duplicated across callsites?** No. Itempool extraction and emission each consolidate into one helper; shared summon-payload emission sits in one private fn used by all three trigger arms. Dice-face access is routed through `SummonTrigger::dice_faces()` (§3.2) — xref/emitter/authoring never `match` on the variant for that field.
- **Canonical set restatement?** `{hero, replica_item, monster}` is named once in SPEC §6.3 and referenced by name from X003 and V020. Per the hook rule, members are not re-listed inside the chunk.
- **Evidence for every variant?** Required ≥ 1 corpus instance per retained variant; re-verify at implementation start via the §1.1 regex set. `SideUse { dice_location: OuterPreface }`: 18 (ball entries via `hat.replica.Thief.n.<Pokemon>` including `Master Ball` no-`?` for Mewtwo). `SideUse { dice_location: InnerWrapper }`: 1 (in-scope `Master Ball?`). `Cast`: 4 (Rainbow Wing + Silver Wing in Summons Part 1; Blue Orb + Red Orb in Summons Part 2). Retired variants: 0 instances. `DiceLocation::InnerWrapper` carries a 1-instance corpus floor — defensible because the source-shape invariant (dice inside wrapper vs on outer preface) is genuinely distinct and round-trip would lose the 1 entry without it; if a future corpus ever has 0 InnerWrapper hits AND no proven need for the shape, the variant retires.
- **Raw passthrough?** No. Non-summon itempool content rides in the existing structural channel's verbatim byte storage, which is the pre-existing exception for permissive extraction, not a new one. `ReplicaItem`'s field list is exhaustive (§3.2) — no `extras` / `raw_remainder` / `RawSubBlock` field, and no `ItempoolEntry::Structural(String)` wrapper. Non-summon entries are an extraction-scope decision, not a per-field escape hatch.
- **Source-vs-IR test present?** T10 (trigger classification reads inner payload not name), T7 (half-summon stays structural), and T23a (no false-positive boss `hat.egg.` extraction) all exercise cases where an IR-equality-only test would pass on silently wrong extraction.
- **Integration assertions encode reason?** T20 (`v020_and_x003_coexist_when_collision_spans_boss_and_pokemon_buckets`) asserts via **structured Finding fields** — V020's `field_path` contains `"bosses["` and X003's `buckets` does NOT contain `"boss"`. No substring scan on human-readable message text. T21 (`no_double_fire_on_working_mods`) uses the same structured predicate.

## 7. Risks

- **Itempool extraction is new parser territory.** The extractor currently keeps pools opaque; teaching it to walk `+`-joined entries at paren-depth 0 (NOT `#`-joined — `#` is intra-entry per §3.3 rule 1) and sub-detect summon shape requires careful paren-depth tracking. Failure mode: roundtrip drift on non-summon entries inside the same pool. Mitigation: non-summon entries become `ItempoolItem::NonSummon { name, tier, content: String }` whose `content` field carries the source bytes verbatim; pure-IR rebuild walks the typed list and re-emits each entry. Test with the full pools (T3: 11 entries with `Master Ball` and `Master Ball?` distinct, T4: 8 entries, T5: 2 entries, T5a: 2 entries) before shipping. The detector accepts only the **one** corpus-evidenced vase-add depth shape (`vase.(add.((replica.…)))`); two speculative variants enumerated in earlier drafts are removed per §3.7's "Lessons" rule.
- **community.txt / pansaer.txt / punpuns.txt false-positive hat.egg.** Step 0 audit recorded (line counts / occurrence counts — `rg -c` vs `rg -o | wc -l`, which differ on single-line mods):
  - punpuns: **13 lines / 19 occurrences** of `hat.egg.`
  - community: **9 lines / 9 occurrences**
  - pansaer: **1 line / 7 occurrences** (pansaer is a single monoline; occurrence count is the semantically meaningful number)

  These are all inside boss/event hero blocks (e.g. `ph.blarvesta;1;!mheropool.(…egg.(…))`), not itempool summon envelopes. The extractor must NOT produce `ReplicaItem` entries for them. T23a is the guardrail. If any of these surface as `ReplicaItem`, the classification gate is wrong — block landing.
- **Community unknown-summon audit.** Beyond the false-positive guardrail, community has 9 `hat.egg.` uses — none should be itempool-scoped, but the detector must run a sweep at implementation: `rg 'itempool\.\(\(.*hat\.egg' working-mods/community.txt`. Any match is a summon shape the IR does not model and must be covered by a new test before landing.
- **Chunk 5/7/9 landing order — RESOLVED.** Per §2, Chunks 5, 7, and 9 land on `main` before Chunk 8 starts. The §3.0 / §3.8 / §9.0 fold-in branches in earlier drafts of this plan are removed. Chunk 8 includes the `builder/derived.rs` migration patch for Chunk 5's `generate_hero_item_pool` (see §3.7 Chunk 5 amendment + §4 derived.rs row).
- **Chunk 10 file overlap.** Chunk 10 (BooleanPhase routing) is referenced as a parallel sibling but has no plan file in `plans/`. Before landing: run `ls plans/CHUNK_10_*.md 2>/dev/null` and `git branch --remotes | grep chunk-10`. If either returns a hit, diff the Chunk 10 file list against this plan's §4 and reconcile. If both return zero, note on the PR that no Chunk 10 is in flight; the overlap risk is moot.

### 7.1 Out of scope (enumerated, not "future work")

The following are explicitly NOT addressed by any sub-chunk in this plan chain; they are named here so a later reader does not assume the omission is an oversight.

- **Typed textmod-escape enum for `doc` fields** (surfaced by 8A.5 §7.2 risk #3). `NonSummonTrailer.doc` stays `Option<String>` across 8A.5 because escape tokens (`[n]`, `[grey]`, `[plusfive]`, etc.) are textmod presentation artefacts, not IR invariants — no xref rule, no CRUD op, no emit-time structural logic reads into `doc`. Closing them would require a guide-authoritative escape-token enum. If a future xref rule DOES read `doc` bytes, the `String` field becomes a latent SPEC §3.2 violation and must be retyped at that time; this chunk does not pre-emptively do that work.
- **Authoring-layer case normalization for `enemy_template` / `team_template`** (§1.1). Source-byte case variants (`Wolf`/`wolf`, `housecat`/`Housecat`) are preserved verbatim by the extractor per SPEC §3.3 permissive extract. Case-normalization is an authoring-layer concern (CRUD helpers, compile-time guards against Pokemon typos) and ships in a later authoring chunk, not here.
- **`item_modifiers` deep-typing inside `ModifierChain`** (§3.2). The `Option<ModifierChain>` field types the chain envelope but does not further type each chain segment's inner `.k.`/`.part.`/`.m.`/etc. body. That's the existing pre-Chunk-8 chain-parser surface; Chunk 8 reuses it as-is and does not retype it.
- **Widening `SummonTrigger::Cast` beyond `dice: DiceFaces`** (§3.2). The outer cast template (`thief`) and outer cast dice (`182-25:0:0:0:76-0:0`) land as emitter constants because the 4 corpus Cast bodies share them identically. Widening to IR fields requires new corpus evidence; the widening contract is documented on the enum doc-comment (§3.2). No speculative widening.
- **`DiceLocation` retirement or expansion**. 1 corpus instance for `InnerWrapper` is defensible — the source-shape invariant is genuinely distinct, and round-trip would lose the entry without it. This chunk does not retire it and does not add a third dice-location. If a future corpus entry surfaces a third source-shape, the enum widens in the same PR that lands that corpus.

## 8. Open questions

None. Q1 (Round-2 open) is resolved — see below.

- **Q1. `SummonTrigger::Cast` payload shape — RESOLVED.** Corpus paren-walk of all 4 `cast.sthief.abilitydata.(...)` bodies (Rainbow Wing, Silver Wing, Blue Orb, Red Orb in sliceymon) shows: (i) outer template = `thief` (literal, identical across all 4); (ii) outer dice = `182-25:0:0:0:76-0:0` (literal, identical across all 4); (iii) **zero** depth-0 `.n.<spell_name>` inside the abilitydata body. AbilityData (which requires a non-Optional `name: String`) does not fit. Options (a) / (b) / (c) are all rejected: there is no spell-name to capture, so no `AbilityData` shape survives the evidence. Decision: `Cast { dice: DiceFaces }` — a single field, symmetric with `SideUse`. Per-item dice live inside the abilitydata body at `.i.hat.(replica.thief.sd.<faces>)` (lowercase `replica.thief`, distinct from SideUse's capital `hat.Thief`). The outer universals move to emitter constants (`CAST_SPELL_TEMPLATE = "thief"`, `CAST_SPELL_DICE = "182-25:0:0:0:76-0:0"` in §3.4). The §3.2 Cast doc comment records the widening contract if a future corpus entry exhibits variation.

All scope, variant count, field list, SPEC wording, and execution split are pinned — see §3.2, §3.6, and §9. Implementation-time re-verifications are documented in §7 and in each sub-chunk's verification gate in §9.

## 9. Sub-chunk execution map

> **This chunk ships as four sub-chunks** (8A → 8A.5 → 8B → 8C; 8C cannot begin until 8A.5 has landed because 8C's SPEC/foundations prose references the typed NonSummon IR). §1–§8 describe the complete change; §9 partitions it into sub-chunks with their own files, tests, and verification gates. Each sub-chunk is a one-shot unit per `personas/ai-development.md` — checkpoint at sub-chunk boundaries, not inside them.
>
> **Per-sub-chunk scope (one-line summary per plan chain)**:
> - **8A** — atomic IR rewrite (`plans/CHUNK_8A_REPLICA_IR_ATOMIC_REWRITE.md`): delete retired types (`ItemPoolEntry`, `ModifierType::Legendary`, `ReplicaItemContainer` remnants, old `ReplicaItem.{name,template,sticker,sd}` flat fields); add `SummonTrigger` / `DiceLocation` / `ItempoolItem` enums; ship **transitional** `ItempoolItem::NonSummon { name, tier, content: String }` (an intentional, tracked SPEC §3.2 violation closed in 8A.5); stub `extract_from_itempool` returns one `NonSummon { content: <entire pool body> }` per pool, keeping all four working mods byte-equal via pass-through.
> - **8A.5** — typed NonSummon schema (`plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`): close the SPEC §3.2 violation by retyping `ItempoolItem::NonSummon(NonSummonEntry)` with 15 evidenced variants + `NonSummonEntry::Unclassified { source_bytes }` permissive hatch (SPEC §3.3). Every byte of every non-summon entry lands in a typed field; recursive `.splice.` body + typed `NonSummonTrailer` / `ImgPayload` / `AbilityBody` records (full schema authoritative in 8A.5 §3.1). 8A.5 must land before 8B because 8B's real `extract_from_itempool` must produce only typed values on both branches.
> - **8B** — real summon classifier + xref widening (`plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md`): real `extract_from_itempool` per §3.3 classifying into `SummonTrigger` + 15 non-summon variants; `Finding` struct widened (`buckets: Vec<&'static str>`, `includes_boss: bool`) per §3.5; X003 + V020 rewrites against the typed fields; bucket-label unification to `replica_item`.
> - **8C** — prose: SPEC §3.6 edits + `PLATFORM_FOUNDATIONS_PLAN.md` rewrites per §3.7 + optional `roundtrip_diag.rs` per-trigger breakdown line.
>
> **Sub-chunk size deviation.** AI-dev's ≤5-file rule is honored at the *spirit* level — 8A's ~11-file IR-rewrite is the smallest atomic unit that compiles (deleting `ReplicaItemContainer` and the old `ReplicaItem` flat fields requires updating every callsite in the same commit per "no parallel representations"; the IR struct rewrite, the parser/dispatch deletions, the emitter rewrite, the ops.rs callsite migration, the structural_parser.rs cleanup, derived.rs's Chunk-5 migration patch, and the new authoring builder must all land together or `cargo build` fails). 8B's larger file list is the smallest unit that round-trips to non-zero summon counts (an extractor change without the emitter+structural-emitter+derived.rs change leaves sliceymon broken). Any further split produces non-compiling intermediate states. Implementer must still checkpoint at the 8A/8A.5/8B/8C boundaries; **inside** a sub-chunk, no checkpoints.

### 9.0 Pre-conditions

Before any 8a/8b/8c code begins, the following must all be true on `main`:
- `rg -c 'slice_before_chain_and_cast' compiler/src/util.rs` returns ≥1 (Chunk 9 in-tree).
- `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` returns **0** (Chunk 5 merged at commit `975da96` but did not ship the function — Chunk 8 authors it from scratch; see M3 fix in §2). At 8A completion this grep returns ≥1.
- Chunk 7's panic-elimination is in-tree (verify post-merge baseline of `rg -c '\.unwrap\(\)|\.expect\(' compiler/src/extractor/ compiler/src/builder/`).

**(M2 fix) 23-vs-25 corpus-audit pre-flight.** §1.1 declares "23 summon envelopes in sliceymon's itempools" while also noting 25 global `vase.(add.((replica.` hits (2 live in boss blocks and are out of scope). Pin the invariant concretely so it is checkable at implementation start, not carried in prose:

```bash
# Total global hits — ALL `vase.(add.((replica.` occurrences across sliceymon.
rg -oF 'vase.(add.((replica.' working-mods/sliceymon.txt | wc -l             # expect 25 (verified 2026-04-24)

# In-scope hits — those inside `itempool.((…))` modifiers. The parent plan's §1.1 count is 23.
# A concrete walker (ships in 8A.5's Phase-A examples/itempool_entry_shapes.rs) can emit the
# filter, OR the count is confirmed by subtraction from the boss-block exclusion grep:
rg -oF 'ph.b' working-mods/sliceymon.txt | wc -l                              # boss-block lines
# The 2 out-of-scope hits live in the Solgaleo / Lunala boss-block `hat.replica.<team>` emit
# bodies visible via `rg -nF 'vase.(add.((replica.' working-mods/sliceymon.txt`; both are inside
# `ph.b<template>` phase bodies, not `itempool.((`).
#
# Gate: if (total != 25) OR (after-boss-exclusion != 23), halt and re-audit §1.1.
```

If the Chunk-9 or Chunk-7 check fails, stop — they must merge first per §2. There is no fold-in branch. The 23-vs-25 audit halts Chunk 8 start if the corpus shape diverges from §1.1's count.

### 9.1 Sub-chunk 8a — atomic IR rewrite (must compile in one commit)

**Scope**: §3.1 classifier/parser/IR retirements + §3.2 IR rewrite + emitter rewrite + dispatch update + authoring builder. **All in one commit** because deleting `ReplicaItemContainer` and `ReplicaItem.{name,template,sticker}` breaks every callsite simultaneously — the only way `cargo build` clean is to update every callsite in the same commit (no parallel representations per CLAUDE.md).

**Files (11, justified by atomic-rewrite — see §9 sub-chunk size deviation)**:
- `compiler/src/extractor/classifier.rs` — delete `ModifierType::{Legendary, ReplicaItem, ReplicaItemWithAbility}` variants + `starts_with_ci(modifier, "item.")` gate. Add the typed classify-error path per §3.1's note.
- `compiler/src/extractor/replica_item_parser.rs` — delete `parse_legendary` / `parse_simple` / `parse_with_ability` and their tests. Leave a stub `extract_from_itempool(body: &str, …) -> Result<ItempoolExtraction, CompilerError>` that returns `Ok(ItempoolExtraction { new_replica_items: vec![], items: vec![ItempoolItem::NonSummon { name: "".into(), tier: None, content: body.to_string() }] })` — a single `NonSummon` entry carrying the ENTIRE itempool body verbatim. This preserves build-side byte-equality with zero `ReplicaItem` extraction, exactly matching the current pre-Chunk-8 behavior where itempool is opaque. Full per-entry implementation lands in 8b.
- `compiler/src/extractor/mod.rs` — delete dispatch arms for the retired ModifierType variants; route ItemPool through the stub `extract_from_itempool` (which returns one-big-NonSummon in 8a — itempool stays structurally opaque but byte-preserving until 8b ships the real parser).
- `compiler/src/extractor/structural_parser.rs` — remove ItemPoolEntry construction loop. For each itempool modifier, delegate to `extract_from_itempool` (called from `extractor/mod.rs`) and store the returned `items` on `StructuralContent::ItemPool { items }`. In 8a that yields one `NonSummon` per pool (body verbatim); in 8b it yields the real per-entry typed list.
- `compiler/src/ir/mod.rs` — delete `ReplicaItemContainer` enum; delete `ItemPoolEntry` struct; rewrite `ReplicaItem` struct per §3.2; add `SummonTrigger` + `DiceLocation` + `ItempoolItem` enums + `dice_faces()` accessor; update `StructuralContent::ItemPool` to `{ items: Vec<ItempoolItem> }` (no body field); update `#[cfg(test)]` helpers.
- `compiler/src/ir/ops.rs` — switch `find_name_category(&item.name)` → `find_name_category(&item.target_pokemon)`; rewrite `make_replica_item` test helper for the new struct; update container-kind CRUD assertions.
- `compiler/src/builder/replica_item_emitter.rs` — full rewrite per §3.4. Emit literal `"Thief"` template; trigger-shape branches for SideUse{OuterPreface}/{InnerWrapper}/Cast.
- `compiler/src/builder/structural_emitter.rs` — call `emit_itempool` from `replica_item_emitter` for ItemPool entries.
- `compiler/src/builder/derived.rs` — Chunk 5 migration patch: rewrite `generate_hero_item_pool` routing match from `match item.container` to `matches!(item.trigger, SummonTrigger::SideUse { .. })`; replace `item.name` with `item.target_pokemon`; rename test `derived::hero_item_pool_matches_sliceymon_via_container_enum` → `derived::hero_item_pool_matches_sliceymon_via_trigger`.
- `compiler/src/authoring/replica_item.rs` (new) — typed builder per T24–T26.
- `compiler/src/authoring/mod.rs` — `pub mod replica_item;` + re-exports.

**Tests in 8a**: T12 (`ReplicaItemContainer` retired), T12a (`ModifierType::ReplicaItem{,WithAbility}` retired), T13 (`parse_legendary` retired — the function is deleted in 8a, so T13 fires here, not in 8b), T14 (`ModifierType::Legendary` retired), T24–T26 (authoring builder compile-guards), plus `#[cfg(test)]` round-trip of the struct via serde. Retirement greps live in `compiler/tests/retirements.rs`.

**Verification gate** (before 8b begins):
- `cargo build` + `cargo clippy` clean.
- `cargo test` passes.
- All four `working-mods/*.txt` still roundtrip byte-equal (`cargo run --example roundtrip_diag` reports `Status: ROUNDTRIP OK`) with `Replicas ir1=0` for all four mods — the 8a stub `extract_from_itempool` returns one `ItempoolItem::NonSummon { content: <entire pool body> }` per pool, which `emit_itempool` writes verbatim (byte-preserving opaque pass-through, matching current pre-Chunk-8 behavior). The real per-entry parser ships in 8b with `replica_items.count: 23 -> 23` for sliceymon.
- `rg -c 'ReplicaItemContainer' compiler/` returns 0.
- `rg -c 'ModifierType::(Legendary|ReplicaItem(WithAbility)?)\b' compiler/src/` returns 0.
- `rg -c 'ItemPoolEntry' compiler/src/` returns 0 (legacy struct retired).
- `rg -c 'item\.template\b|item\.name\b|item\.sticker\b' compiler/src/` returns 0 (all callsites migrated).

### 9.2 Sub-chunk 8b — real extractor + xref Finding widening + tests (after 8a)

**Scope**: §3.3 extractor real implementation (replace 8a's stub) + §3.5 xref rewrite (Finding widening + bucket routing) + all round-trip and trigger-classification tests.

**Files (6)**:
- `compiler/src/extractor/replica_item_parser.rs` — replace 8a's stub `extract_from_itempool` with the real implementation per §3.3 (returns `Result<ItempoolExtraction, CompilerError>` with populated `new_replica_items` and `items: Vec<ItempoolItem>`).
- `compiler/src/extractor/mod.rs` — wire `extract_from_itempool`'s output: append `new_replica_items` to `ModIR.replica_items`, store `items: Vec<ItempoolItem>` on the corresponding `StructuralContent::ItemPool` entry.
- `compiler/src/builder/mod.rs` — confirm dispatch for the new emitter shape (8a wired the call sites; 8b validates against real data).
- `compiler/src/xref.rs` — §3.5 edits: **first commit widens `Finding` struct** (add `pub buckets: Vec<&'static str>` and `pub includes_boss: bool` with `#[serde(default)]`); then bucket routing delete, `target_pokemon` keying, `iter_dice_faces` template arg switches to literal `"thief"`, suggestion-string rewrite, **delete `xref::x003_distinguishes_capture_from_legendary_buckets` and message-prose-asserting siblings**.
- `compiler/tests/build_options_tests.rs` — migrate construction-site assertions to trigger-based shape; replace `use ... ReplicaItemContainer` with `use ... SummonTrigger`.
- `compiler/tests/roundtrip_baseline.rs` — regenerate baselines; sliceymon's `replica_items.count: 23 -> 23`.
- `compiler/tests/integration_tests.rs` — remove `:412` stale comment and any gated body per §4.

**Tests in 8b**: T1–T11 (source-vs-IR + trigger classification + Master Ball? + `target_pokemon` byte preservation + sticker-chain), T15–T21 (xref 3-bucket shape + typed-Finding-field predicates), T22 / T23 / T23a (baseline regeneration + roundtrip_diag + no-false-positive boss `hat.egg.`), T27 (non-sliceymon mods produce zero `ReplicaItem`). T13 already shipped in 8a.

**Verification gate**:
- `cargo build` + `cargo clippy` clean; `cargo test` passes.
- All four mods roundtrip; sliceymon reports `replica_items.count: 23 -> 23` (baseline file format) and `roundtrip_diag` stdout shows `Replicas ir1=23` with breakdown `SideUse{OuterPreface}=18 SideUse{InnerWrapper}=1 Cast=4`; the other three report `0 -> 0` / `Replicas ir1=0`.
- `rg -c 'ItemPoolEntry|source_byte_range' compiler/src/` returns 0 (legacy struct + abandoned byte-range design both gone).
- X003's typed `buckets` field is a subset of `["hero", "replica_item", "monster"]`; V020's `includes_boss: bool` is true only for boss-involving collisions. **No bucket-string substring banlist** (T10's `Cast-Iron Ball` synthetic would falsely trip a coarse banlist; use the typed field).

### 9.3 Sub-chunk 8c — SPEC + plan-layer + hook (parallel with 8b)

**Scope**: §3.6 SPEC edits + §3.7 foundations-plan rewrites + hook update + example helpers.

**Files (5)**:
- `SPEC.md` — §3.6: line 246 parenthetical dropped (D1); line 342 full rewrite to three-trigger narrative; grep-and-update downstream `Capture` / `Legendary` IR-kind references.
- `plans/PLATFORM_FOUNDATIONS_PLAN.md` — §3.7: §F7 rewrite; Chunk 6 block in-place rewrite; Chunk 8 entry full rewrite; dependency-graph and Parallel-Execution-Map edits per §3.7's bulleted string-replacement contract; new "Lessons" entry.
- `.claude/settings.json` — fourth PreToolUse-hook bullet per §3.7 (corpus-grounding "zero `rg` hits → hypothesis, not model").
- `compiler/examples/roundtrip_diag.rs` — optional per-trigger breakdown line. No behavior change required, but confirm post-8b breakdown numbers.
- `compiler/examples/drift_audit.rs` — verify pass; add Pokemon-summon drift class if relevant.

**Tests in 8c**: none beyond the existing SPEC/plan CI checks (which in this repo are prose, not automated) plus `cargo run --example roundtrip_diag` smoke check. 8c does not ship any Rust test — its verification is that 8b's T22/T23 still pass after 8c's example edits and that the hook diff is valid JSON.

**Verification gate**:
- `cargo run --example roundtrip_diag` still reports `Status: ROUNDTRIP OK` for all four mods with the same numbers as 8b's gate.
- `.claude/settings.json` parses (`python3 -c 'import json; json.load(open(".claude/settings.json"))'`).
- `rg -n 'Chunk 8.*V020 restructure\|only touches xref\.rs' plans/PLATFORM_FOUNDATIONS_PLAN.md` returns zero hits.
- `rg -ni 'capture|legendary|legendaries|capturable' SPEC.md` only matches intentionally preserved game-flavor uses (if any — Round 2 audit found zero), not IR-variant descriptions. Case-insensitive + plural variants required; the narrower pattern `'Capture\|Legendary'` silently misses lines 78/104/168/188/335 per §3.6.

### 9.4 Parallel execution map

```
[Chunks 5, 7, 9 must be on `main` per §2 pre-condition — verified by §9.0]
    │
    ▼
Sub-chunk 8A (atomic IR rewrite — ~11 files in one commit; ships transitional NonSummon.content: String)
    │
    ▼
Sub-chunk 8A.5 (typed NonSummonEntry sum — closes SPEC §3.2 raw-passthrough violation)
    │
    ▼
Sub-chunk 8B (real summon classifier + Finding widening + X003/V020 rewrites + roundtrip tests)
    │
    ▼
Sub-chunk 8C (SPEC + foundations-plan prose + optional roundtrip_diag helper)
    │
    ▼
integrate + final verification
```

- **Sequential (strict)**: 8A → 8A.5 → 8B → 8C. No pair in this chain is parallel:
  - **8A.5 blocks 8B**: 8B's real `extract_from_itempool` classifies each `+`-joined entry into either a typed `SummonTrigger`-shaped `ItempoolItem::Summon(i)` OR a typed `NonSummonEntry`. If 8A.5 has not landed, 8B's only non-summon output surface is 8A's transitional raw-string `content: String`, which re-introduces the SPEC §3.2 violation at every summon-failed entry (per 8A.5 §1.2).
  - **8A.5 blocks 8C**: 8C's SPEC/foundations prose describes the shipped IR surface, which includes `NonSummonEntry`'s typed schema. Authoring SPEC prose against 8A's transitional raw-string `content: String` would either (a) document a known SPEC §3.2 violation as if it were shipped, or (b) require a second SPEC rewrite when 8A.5 lands. Both are worse than serializing 8C after 8A.5.
- **Entry gate**: 8A (gated by §9.0's verification that Chunks 5/7/9 are on `main`; as of plan-write time they are NOT).
- **File overlap**: 8A's file set (IR + extractor stub + emitter + ops + authoring + derived.rs migration) disjoint from 8A.5's (IR `NonSummonEntry` sum + extractor NonSummon classifier + emitter NonSummon reconstructor). 8B's file set (real summon classifier + xref + tests) disjoint from 8C's (SPEC + foundations-plan + optional example). No intra-chunk overlap.
- **Wall-clock**: 4 serial rounds (8A → 8A.5 → 8B → 8C) — Chunks 5/7/9 are upstream, not part of Chunk 8's wall clock.

### 9.5 Checkpoint discipline (per `personas/ai-development.md`)

| Boundary | Checkpoint report? | Wait for user? |
|---|---|---|
| §9.0 pre-conditions verified (Chunks 5/7/9 in-tree on `main`) | Yes (rg outputs proving each precondition) | Yes — confirm before 8A starts |
| 8A complete | Yes (8A's verification gate — transitional NonSummon byte-equal pass-through; all four mods `Replicas ir1=0`) | Yes — critical; IR shape change ripples through ~11 files; transitional `NonSummon.content: String` is a **tracked SPEC §3.2 violation** that 8A.5 closes |
| 8A.5 complete | Yes (typed `NonSummonEntry` + all four mods round-trip byte-equal through typed reconstruction; zero `Unclassified` hits on corpus or a Finding for each) | Yes — dominant-shape correctness; 8B cannot start until this lands |
| 8B complete | Yes (sliceymon `replica_items.count: 23 -> 23`; no double-fire; X003 buckets typed) | Yes — biggest risk sub-chunk per §7 |
| 8C complete | Yes (SPEC + plan diff summary) | Yes — end-state PR prep |

No sub-chunk is permitted to silently continue past its verification gate. If a gate fails, fix in-place before the next sub-chunk starts; do NOT defer to a follow-up.
