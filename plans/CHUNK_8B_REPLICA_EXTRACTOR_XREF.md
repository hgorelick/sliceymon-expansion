# Chunk 8b: Real summon prefilter + xref Finding widening + roundtrip tests

> **Parent plans**: §3.3, §3.5, §4, §9.2 of `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md`. `plans/CHUNK_8A_REPLICA_IR_ATOMIC_REWRITE.md` §9.1 AND `plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md` §9 are the upstream gates — 8A.5 lands BEFORE 8B.
>
> **Authority contract**: this plan is **strictly narrower** than the parent's §3.3/§3.5/§9.2 — every sentence here either (a) restates a parent claim verbatim, (b) narrows it to a single-commit scope, or (c) references the parent by §. It introduces **no new field, enum variant, or invariant** not already in the parent or in 8A.5. A contradiction = fix the parent first, in the same PR (hook rule #1).
>
> **Ships as a single one-shot sub-chunk** per `personas/ai-development.md`: wire the real summon prefilter against the 8A/8A.5 IR shape, widen `Finding` for typed bucket assertions, and land every source-vs-IR test that proves the summon detector reads inner payload bytes, not surface names. No internal checkpoints — one PR, one cargo-test-clean gate.

---

## 1. Overview

### 1.1 What this chunk does

8A shipped the trigger-based IR (`SummonTrigger::{SideUse{dice, dice_location}, Cast{dice}}` + `DiceLocation::{OuterPreface, InnerWrapper}` + transitional `ItempoolItem::{Summon(usize), NonSummon { name, tier, content: String }}`) plus a stub `extract_from_itempool` that returned one raw-passthrough `NonSummon { content: <entire pool body> }` per itempool modifier. **8A.5 then replaced the transitional `NonSummon` variant with `NonSummon(NonSummonEntry)` — a typed 15-variant recursive sum** (per `plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md` §3.1), shipped the typed emitter, and wired a `classify_non_summon_entry` function that types every `+`-split entry in every corpus itempool body. 8A.5's classifier ran under the stub assumption that **every** entry is non-summon — i.e. no summon prefilter was wired; instead a placeholder `classify_summon_entry` fn always returned `None`, so zero `ReplicaItem`s were produced.

8B replaces the stub summon prefilter with the real detector (parent §3.3 rule 2, sub-rules (a)-(d)): a conjunctive `hat.egg.<enemy>` + `vase.(add.((replica.<team>.n.<same-Pokemon>…)))` pair plus one of three wrapper shapes (SideUse{OuterPreface}, SideUse{InnerWrapper}, Cast). For each entry the prefilter matches, 8B produces `ItempoolItem::Summon(i)` and populates a `ReplicaItem` in `ir.replica_items`. For entries the prefilter rejects, 8B delegates to 8A.5's landed `classify_non_summon_entry` — no rewrite of the typed classifier here. 8B also widens `Finding` for the typed `buckets` / `includes_boss` fields (parent §3.5), rewrites X003 + V020 against those fields, unifies X003's bucket labels with V020's (rename `"legendary"` → `"replica_item"`), and lands every source-vs-IR test that proves summon classification reads inner payload bytes — never container surface names.

### 1.2 What this chunk does NOT do

- **No IR shape change.** `ReplicaItem`, `SummonTrigger`, `DiceLocation`, `ItempoolItem`, `NonSummonEntry`, and `StructuralContent::ItemPool { items: Vec<ItempoolItem> }` are frozen as 8A + 8A.5 shipped them. 8B widening of any of these types = plan defect, fix upstream first.
- **No NonSummon typing work.** 8A.5 closed the SPEC §3.2 raw-passthrough violation; 8B does NOT re-shape `NonSummonEntry`, does NOT add per-variant NonSummon emit helpers, does NOT re-ship T30.1–T30.15 / T31 / T32 / T34. Those shipped in 8A.5.
- **No `split_itempool_entries` helper re-authoring.** Per 8A.5 §4 bullet 9, the `+`-at-paren-depth-0 splitter ships as `util::split_itempool_entries` in 8A.5. 8B's prefilter CALLS the existing helper — verify by grep at impl start; if absent, 8A.5 is incomplete and 8B halts.
- **No `W-REPLICA-NONSUMMON-UNCLASSIFIED` Finding code.** 8A.5 ships it (per 8A.5 §3.2.2 bullet 3). 8B's parallel Finding is `W-REPLICA-TRIGGER-UNCLASSIFIED` (parent §3.3 rule 2(d) final bullet) — a distinct failure mode, not the same code.
- **No SPEC / plan-layer / hook edits** — those ship in 8c.
- **No builder/emitter signature change.** 8A + 8A.5 shipped `emit_replica_item` + `emit_itempool` + `emit_non_summon_entry` with final signatures; 8B calls them, does not rewrite them.
- **No `derived.rs` edit** — Chunk 5 + 8a own that file's final state for `generate_hero_item_pool`.
- **No `SummonTrigger::Cast` widening.** The parent §3.2 pins `Cast { dice: DiceFaces }` with emitter constants `CAST_SPELL_TEMPLATE = "thief"` and `CAST_SPELL_DICE = "182-25:0:0:0:76-0:0"` — 8b implements detection against these constants; widening requires new corpus evidence + a parent plan amendment.

### 1.3 Checkpoint + parallel map

- **Dependency chain**: 8A → 8A.5 → 8B → 8c. 8A and 8A.5 are both upstream of 8B; 8c is prose-only and runs concurrently with 8B after 8A.5 lands.
- **Parallel with**: 8c (prose-only — SPEC + plan-layer + hook). 8b owns Rust library code; 8c owns prose. Zero file overlap (verified in §6.2).
- **Upstream (must be on `main`)**: both 8A (parent §9.1) AND 8A.5 (its §9). 8B's §2 pre-conditions gate both.
- **Wall-clock**: one round. 8B and 8c run concurrently after 8A.5 lands.

---

## 2. Pre-conditions (verify before any 8b code lands)

All must be true on `main` at 8b start. If any fails, **halt**; fix upstream.

```bash
# 1. Parent §9.0 pre-conditions still hold (Chunks 5, 7, 9 on main).
rg -c 'slice_before_chain_and_cast' compiler/src/util.rs          # expect ≥1 — Chunk 9 in-tree
rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs # expect ≥1 — Chunk 5 in-tree (or 8a authored it)
rg -c '\.unwrap\(\)|\.expect\(' compiler/src/extractor/ compiler/src/builder/ # expect ≤ Chunk 7's post-merge baseline
# Chunk 9 (§F10) must have added `depth_aware: bool` to every scalar extractor §3.1 uses.
# `extract_img_data` is depth-aware by construction (no parameter — uses `find_last_at_depth0`).
# `extract_sd` and `extract_hp` accept `depth_aware: bool` pre-Chunk-9 (already in-tree).
# `extract_color` must gain the parameter in Chunk 9. If Chunk 9 landed but omitted it, §3.1's
# chain-interior color-leak guard is unreachable at callsite — halt and reopen Chunk 9.
rg -n 'pub fn extract_color\(content: &str, depth_aware: bool\)' compiler/src/util.rs # expect 1 hit
rg -n 'pub fn extract_sd\(content: &str, depth_aware: bool\)'    compiler/src/util.rs # expect 1 hit
rg -n 'pub fn extract_hp\(content: &str, depth_aware: bool\)'    compiler/src/util.rs # expect 1 hit

# 2. 8A shipped (parent §9.1 verification gate).
cargo build 2>&1 | grep -q 'Finished' && echo 'build OK'
cargo test --lib 2>&1 | tail -5
rg -c 'ReplicaItemContainer' compiler/                                     # expect 0
rg -c 'ModifierType::(Legendary|ReplicaItem(WithAbility)?)\b' compiler/src/ # expect 0
rg -c 'ItemPoolEntry' compiler/src/                                        # expect 0
rg -c 'item\.template\b|item\.name\b|item\.sticker\b' compiler/src/        # expect 0 (all callsites migrated to target_pokemon/container_name/sticker_stack)
rg -n 'pub enum SummonTrigger|pub enum DiceLocation|pub enum ItempoolItem' compiler/src/ir/mod.rs # expect 3 hits
rg -n 'pub fn dice_faces\b'                                        compiler/src/ir/mod.rs         # expect 1 hit
rg -n 'pub struct ItempoolExtraction'                              compiler/src/extractor/replica_item_parser.rs # expect 1 hit

# 2a. (B4 fix) Nested `hat.egg.(` gate — inherits from 8A fix A2. Red Orb's body contains a
# nested `hat.egg.(wolf.n.Geyser...` inside the outer `hat.egg.dragon.n.Groudon` (verified
# 2026-04-24: sliceymon.txt line 117 via `rg -nF 'hat.egg.(wolf' working-mods/sliceymon.txt`).
# 8A's `ReplicaItem.enemy_template: String` captures only the outer template; nested sub-eggs
# are not modeled. If 8B starts with this shape in corpus AND 8A's IR does not carry a
# nested-egg field, 8B must widen the ReplicaItem schema IN THE SAME COMMIT before producing
# any `Summon(i)` for a nested-egg body — emitting a partial ReplicaItem with silently-lost
# nested bytes is raw-passthrough by omission (SPEC §3.2 violation).
rg -nF 'hat.egg.(wolf' working-mods/sliceymon.txt | wc -l                   # expect ≥1 (Red Orb latent case)
rg -n 'pub enemy_template\b' compiler/src/ir/mod.rs                         # confirms 8A's shape post-8A landing
# Gate: if the first grep ≥1 AND the second does not surface a nested-egg field on ReplicaItem,
# 8B must widen the schema in the same commit. Do NOT paper over via `source_bytes` stringification.

# 3. 8A.5 shipped (per 8A.5 §9 verification gate). This is the new gate 8B inherits.
rg -n 'pub enum NonSummonEntry\b' compiler/src/ir/mod.rs                    # expect ≥1 — 8A.5 authored the typed sum
rg -n 'Unclassified\s*\{\s*source_bytes' compiler/src/ir/mod.rs             # expect 1 — 8A.5's SOLE raw-byte pressure-valve
rg -c 'NonSummon\s*\{\s*(name|content|body)\s*:' compiler/src/              # expect 0 — the transitional 8A shape is fully retired
rg -c 'NonSummon\s*\(\s*NonSummonEntry' compiler/src/ir/mod.rs              # expect ≥1 — the typed ItempoolItem::NonSummon variant
rg -n 'fn split_itempool_entries\b' compiler/src/util.rs                    # expect ≥1 — the shared `+`-at-depth-0 helper 8A.5 shipped; 8B calls it
rg -n 'fn classify_non_summon_entry\b' compiler/src/extractor/replica_item_parser.rs # expect 1 — 8A.5's typed classifier 8B delegates non-summon entries to
rg -n 'W-REPLICA-NONSUMMON-UNCLASSIFIED' compiler/src/                      # expect ≥1 — 8A.5's Finding code wired
cargo run --example roundtrip_diag 2>&1 | grep -c 'Status: ROUNDTRIP OK'   # expect 4 (all four mods byte-equal through 8A.5's typed emit)
cargo run --example roundtrip_diag 2>&1 | grep 'Replicas ir1='             # expect `ir1=0` for all four — 8A.5 stubs every entry as NonSummon; 8B turns ir1=23 on for sliceymon
# 8A.5 ships its Phase-A walker and T30.0 budget ratchet; both must be green before 8B.
cargo run --example itempool_entry_shapes 2>&1 | tail -5                    # expect zero Unclassified across the four mods
```

If any expected-≥1 count returns 0 or any expected-0 count returns ≥1, either 8A or 8A.5 did not complete its verification gate. **Do NOT start 8B.** Reopen the upstream chunk, land the missing state, re-run the gate.

---

## 3. Scope — what ships (scoped narrowings of parent §3.3 + §3.5)

### 3.1 Summon prefilter + dispatch order (the load-bearing 8B contribution)

File: `compiler/src/extractor/replica_item_parser.rs`.

Parent §3.3 is authoritative — do not restate rules. 8B's narrowing:

- **Signature unchanged from 8A stub**: `pub fn extract_from_itempool(body: &str, modifier_index: usize, next_replica_index: usize) -> Result<ItempoolExtraction, CompilerError>`. The `ItempoolExtraction { new_replica_items: Vec<ReplicaItem>, items: Vec<ItempoolItem> }` struct ships in 8A; 8A.5 wired the per-entry loop using `util::split_itempool_entries` + `classify_non_summon_entry`; 8B replaces 8A.5's placeholder `classify_summon_entry` (always `None`) with the real prefilter.
- **Entry splitting — inherited from 8A.5.** The helper `util::split_itempool_entries(body: &str) -> Vec<&str>` is authored by 8A.5 (its §4 bullet 9) against parent §3.3 rule 1: walk paren-depth; split on `+` at depth 0 only; `#` is intra-entry, never split; preserve the leading `+` joiner as part of the following entry's source slice. 8B does not re-author the helper; 8B calls it. If §2's `rg -n 'fn split_itempool_entries\b'` pre-condition returns 0, 8A.5 is incomplete — halt.
- **Dispatch order inside `extract_from_itempool`** (this is 8B's new routing logic):
  1. For each `+`-split entry returned by `util::split_itempool_entries`, 8B first calls `classify_summon_entry(entry: &str, modifier_index: usize) -> Result<Option<SummonClassification>, CompilerError>` (8B's real implementation, replacing 8A.5's `None`-stub placeholder).
  2. If `classify_summon_entry` returns `Ok(Some(SummonClassification { replica_item, /* … */ }))`, push the `ReplicaItem` onto `new_replica_items` and push `ItempoolItem::Summon(i)` onto `items` (where `i = next_replica_index + new_replica_items.len() - 1`).
  3. If `classify_summon_entry` returns `Ok(None)` (the summon prefilter REJECTED the entry — either (a)+(b) conjunctive pair missing, or the entry has no summon shape at all), 8B delegates to 8A.5's landed `classify_non_summon_entry(entry: &str, modifier_index: usize) -> Result<NonSummonEntry, CompilerError>` and pushes `ItempoolItem::NonSummon(entry)` onto `items`. The `modifier_index` parameter is required so `classify_non_summon_entry` can carry it into `Finding { code: "W-REPLICA-NONSUMMON-UNCLASSIFIED", modifier_index, … }` emission on the `Unclassified` fallback (8A.5 §3.2.2 pins this signature).
  4. If `classify_summon_entry` returns `Err(_)`, propagate — this is the strict `CompilerError::classify` path (see below), not a soft demote.
- **Summon detection (conjunctive pair)** — parent §3.3 rule 2 (a)-(c). The detection gate is a **conjunction**: (a) `hat.egg.<enemy_template>.n.<Pokemon>` sub-block at the correct nesting + (b) `vase.(add.((replica.<team_template>.n.<Pokemon>…)))` sub-block with the **same** `<Pokemon>` bytes + (c) the outer wrapper shape is one of the three trigger templates. Mismatch or missing either half → `classify_summon_entry` returns `Ok(None)`; the outer loop demotes to 8A.5's typed NonSummon path (parent §3.3 rule 3) — no partial `ReplicaItem` with default fields, no `ItempoolItem::NonSummon` with raw bytes (the raw-byte hatch died with 8A.5).
- **Trigger classification order** — parent §3.3 rule 2(d) ships Cast → SideUse{OuterPreface} → SideUse{InnerWrapper} → unclassifiable-with-Finding. The order is load-bearing: checking SideUse{OuterPreface} first would misclassify the `Master Ball?` inner-wrapper case if a future corpus entry ever co-emitted an outer preface and an inner-wrapper shape; Cast first ensures `cast.sthief.abilitydata.` detection wins when both are present. Preserve this order verbatim from parent §3.3.
- **Paren-depth correctness** — parent §3.3 rule 4. Scalar extraction (`extract_sd`, `extract_hp`, `extract_color`, `extract_img_data`) passes `depth_aware = true` via the Chunk 9 `util::slice_before_chain_and_cast` helper. **Verification against current post-main-merge worktree** (`grep -n 'pub fn extract_\w*\|pub fn slice_before_chain_and_cast' compiler/src/util.rs`): the signatures `pub fn extract_sd(content: &str, depth_aware: bool) -> Option<String>`, `pub fn extract_hp(content: &str, depth_aware: bool) -> Option<u16>`, and `pub fn extract_color(content: &str, depth_aware: bool) -> Option<char>` all accept the `depth_aware: bool` parameter (Chunk 9 landed the parameter pre-merge); `pub fn extract_img_data(content: &str) -> Option<String>` uses `find_last_at_depth0` internally and is depth-aware by construction (no parameter). §2's pre-condition greps restated here are therefore expected to **pass in current state** — if any regresses, Chunk 9 was reverted and 8b halts. `depth_aware = false` at any callsite would let chain-interior `.hp.` / `.col.` substrings leak into top-level `ReplicaItem` fields and silently pass an IR-equality-only round-trip (exactly the source-vs-IR divergence hook rule #2 flags).
- **`Finding` on unclassifiable wrapper shape (B3 fix: BACKLOG, not 8B scope)** — parent §3.3 rule 2(d) final bullet sketches `W-REPLICA-TRIGGER-UNCLASSIFIED` for the "conjunctive pair matches, but wrapper shape is none of Cast / SideUse{OuterPreface} / SideUse{InnerWrapper}" case. **Corpus grep returns zero instances of this shape** (verified this session: the 23 sliceymon summon envelopes all classify as Cast(4) + SideUse{OuterPreface}(18) + SideUse{InnerWrapper}(1); pansaer/punpuns/community have zero summon envelopes per T23a/T27). Per checklist rule #3 "every IR variant discriminator must have at least one corpus instance ... zero instances for a variant means the variant is a hypothesis masquerading as a model", **this Finding code is NOT wired in 8B.** Remove it from 8B's failure-mode matrix (§3.1). The strict `CompilerError::classify` path below handles the same signal via the error channel during 8B development; if a genuine unclassifiable-wrapper instance ever surfaces (future corpus addition), a new chunk authors the Finding with the corpus entry as evidence. 8A.5's `W-REPLICA-NONSUMMON-UNCLASSIFIED` remains distinct — it fires for a different failure mode (typed NonSummon classifier cannot find a V1–V15 variant) that DOES have corpus coverage via the Phase-A walker. The split is preserved; only the unevidenced half is deferred.
- **`CompilerError::classify` on Cast inner-dice missing** — parent §3.3 rule 2(d) Cast-branch final sentence. This is a **strict** path: a wrapper with `cast.sthief.abilitydata.(thief.sd.<UNIVERSAL>.i.(…))` that passed (a)+(b)+(c)+outer-template-match but lacks the inner `.i.hat.(replica.thief.sd.<faces>)` is a contract violation (the corpus and the emitter constants agree on the shape; divergence is a bug, not a new shape).

**Failure-mode matrix** (dispatch order summary — state once, reference by name everywhere else):
| Entry shape | Summon prefilter result | NonSummon classifier result | Emitted `ItempoolItem` | Finding |
|---|---|---|---|---|
| Conjunctive pair present + recognized wrapper | `Some(SummonClassification)` | (not called) | `Summon(i)` + `ReplicaItem` in `new_replica_items` | none |
| Conjunctive pair present + wrapper shape unknown | `Ok(None)` (after Finding emitted) | `Ok(V1–V15)` OR `Ok(Unclassified)` | `NonSummon(entry)` | `W-REPLICA-TRIGGER-UNCLASSIFIED` (permissive) |
| No conjunctive pair | `Ok(None)` | `Ok(V1–V15)` | `NonSummon(entry)` | none |
| No conjunctive pair, 8A.5 classifier can't type | `Ok(None)` | `Ok(Unclassified{source_bytes})` | `NonSummon(Unclassified)` | `W-REPLICA-NONSUMMON-UNCLASSIFIED` (8A.5's code) |
| Conjunctive pair + Cast wrapper + outer template match, but no inner `replica.thief.sd.<faces>` | `Err(CompilerError::classify)` | (not called) | (fails extract) | (hard error) |

The matrix's "none" / "V1–V15" rows correspond to the working-mods outcome once T30.0 (8A.5's zero-Unclassified ratchet) is green; `Unclassified` rows exist for synthetic T9c and for future corpus shapes that the maintainer widens the typed IR to cover.

**Structural smell check** (hook rule #3): the three paths (summon-matched, prefilter-rejected → 8A.5 NonSummon, prefilter-accepted-but-wrapper-unknown → NonSummon + Finding) are NOT the same path with a policy knob — they cover genuinely different failure modes. The split is load-bearing and explicit in parent §3.3 rule 2(d); preserve it verbatim.

**N-line-incantation check** (hook rule #3): the three trigger-classify branches share the egg + vase-add pair detector and the `target_pokemon / enemy_template / team_template` extraction. Factor the shared work into **one** private helper (`detect_summon_pair(wrapper_body: &str) -> Option<SummonPair>`), and let the three branches differ only in dice extraction and variant construction. The parent §3.4's `emit_replica_item` already structures itself this way (one shared payload emitter + three wrapper-shape branches); mirror the structure on the extract side so extract and emit are symmetric.

### 3.2 Extractor mod wiring

File: `compiler/src/extractor/mod.rs`.

8A.5 already wired the `ModifierType::ItemPool` arm against `extract_from_itempool`; 8B inherits the wiring with zero shape change. The behavioral difference post-8B is that `extraction.new_replica_items` is non-empty for sliceymon (23 entries) and `extraction.items` now contains a mixture of `Summon(i)` and `NonSummon(typed-entry)` instead of all-`NonSummon`. Every working mod's per-pool `+`-order of entries must match source order byte-for-byte — this is the T28 index-stability invariant (see §5.4).

If 8A.5 did NOT wire the arm (i.e. §2's pre-condition greps fail), halt; this is 8A.5's scope, not 8B's.

`structural_parser.rs` already delegates to this path (8a retired the local `ItemPoolEntry` loop; 8A.5's typed-emit path reads the same surface). No edit to `structural_parser.rs` in 8B — if one is needed, either 8A or 8A.5 did not complete.

### 3.3 `Finding` widening (first commit of 8b)

File: `compiler/src/finding.rs` (post-main-merge: `Finding` moved out of `xref.rs` — see the re-export `pub use crate::finding::{Finding, Severity};` in `xref.rs`).

The current `Finding` struct in `compiler/src/finding.rs` (verified this session by Read) derives `#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]` and uses `#[serde(default)]` on `severity`, `field_path`, `suggestion`, and `source` — four fields. The remaining four optional fields (`modifier_index`, `modifier_name`, `position`, `context`) carry only `#[serde(skip_serializing_if = "Option::is_none")]` without a `#[serde(default)]` attribute. This is behaviorally benign for default-init (Rust's `Default` derive fills them with `None`), but the new 8b fields must mirror the `serde(default)` attribute explicitly so deserialization without the field produces the default. Add two new fields in this style:

```rust
#[serde(default, skip_serializing_if = "Vec::is_empty")]
pub buckets: Vec<&'static str>,
#[serde(default)]
pub includes_boss: bool,
```

**Backward-compat**: every existing `Finding { .. }` construction site continues to compile IF AND ONLY IF it either ends with `..Default::default()` or omits `buckets`/`includes_boss` (allowed because both have `#[serde(default)]` *and* type-level defaults `Vec::new()` / `false` — but **only** when the struct literal is not exhaustive). The **13 raw `Finding {` struct-literal sites** in `compiler/src/xref.rs` (verified this session: `grep -c 'Finding {' compiler/src/xref.rs` = 13, each a `push_finding(..., Finding {` call, one beginning `let f = Finding {` inside `check_duplicate_pokemon_buckets` / V020 / V016 / V019 / single-item context checks) must each be audited. **Plus 3 additional sites outside `xref.rs`** (verified this session via `grep -n 'Finding {' compiler/src/lib.rs compiler/src/ir/merge.rs compiler/tests/build_options_tests.rs`): one `report.errors.push(Finding {` in `compiler/src/lib.rs`, one `warnings.push(Finding {` in `compiler/src/ir/merge.rs`, and one `let f = Finding {` literal in `compiler/tests/build_options_tests.rs` inside `fn finding_json_omits_absent_source`. Rust's struct-update sugar (`..Default::default()`) is required on any exhaustive literal; without it, adding `buckets` + `includes_boss` is a compile error at that site. **All 16 construction sites must be audited**, not just the 13 inside `xref.rs`.

**Audit requirement (load-bearing, per parent §3.5)**: before proceeding past the `Finding` widening commit, run:

```bash
# (B10 fix) Verified 2026-04-24 this conversation via `rg -n 'Finding \{' compiler/src/ compiler/tests/`:
#   - 13 sites in compiler/src/xref.rs (lines 237, 388, 419, 456, 519, 559, 597, 620, 639, 658, 688, 707, 726)
#   - 1 site in compiler/src/lib.rs (line 84, `report.errors.push(Finding {`)
#   - 1 site in compiler/src/ir/merge.rs (line 79, `warnings.push(Finding {`)
#   - 1 site in compiler/tests/build_options_tests.rs (line 277, `let f = Finding {`)
#   Total = 16 construction sites. The struct DEFINITION (`pub struct Finding {`) at
#   compiler/src/finding.rs:24 is NOT a construction site — exclude it from the 16 count.
grep -c 'Finding {' compiler/src/xref.rs                   # expect 13
grep -rn 'Finding {' compiler/src/ compiler/tests/ | grep -v 'pub struct Finding' | wc -l  # expect 16
grep -nB1 -A20 'Finding {' compiler/src/xref.rs            # visual inspection for `..Default::default()` tails
cargo build 2>&1 | tee /tmp/8b-finding-widen-build.log     # must be clean after ONLY adding the two fields
```

If `cargo build` errors at an exhaustive-literal site, fix that site in the same commit by appending `..Default::default()` (do NOT populate the new fields with defaults — that defeats the purpose of typed-field assertions elsewhere). The only rules that populate the new fields affirmatively: X003 (`buckets`) and V020 (`includes_boss`).

### 3.4 X003 rewrite against typed buckets (includes bucket-label rename `"legendary"` → `"replica_item"`)

File: `compiler/src/xref.rs`.

**User decision (this PR)**: X003's bucket set is renamed from `{hero, legendary, monster}` to `{hero, replica_item, monster}` — unifying labels across X003 and V020. The prior "one-to-one but different spelling" equivalence is replaced by a single canonical label. This is a real code change 8B ships, not just a plan rewrite.

**Current state (post-main-merge, verified this session by Read of `compiler/src/xref.rs` and `compiler/src/ir/mod.rs`):**

- `ReplicaItemContainer` was **already retired pre-Chunk-8** (`grep -rn 'ReplicaItemContainer' compiler/` → zero hits in non-comment source). The prior plan draft's "Capture / Legendary match" does not exist; the code collapsed to unconditional `"legendary"` when `ReplicaItemContainer` was deleted.
- Current `ReplicaItem` IR shape in `compiler/src/ir/mod.rs` (verified by Read: `pub struct ReplicaItem { ... }` with fields `name`, `template`, `hp`, `sd`, `sprite`, `color`, `tier`, `doc`, `speech`, `abilitydata`, `item_modifiers`, `sticker`, `toggle_flags`, `source`). No `container`, no `trigger`, no `target_pokemon`. These are 8a-intended renames; the real code still uses `name` / `template` / `sd` everywhere.
- X003's bucket set (verified this session by Read: the doc-comment `// X003 therefore collects buckets as {hero, legendary, monster} — narrower than V020's {hero, replica_item, monster, boss}` directly above `fn check_duplicate_pokemon_buckets`, plus the three `.push((..., "hero"))`, `.push((..., "legendary"))`, `.push((..., "monster"))` lines inside that function) is **currently `{hero, legendary, monster}`**. 8B renames the middle label to `"replica_item"` so X003 and V020 share a single canonical vocabulary.

**Edit table — locate each target by the verbatim quoted grep pattern at impl time (line numbers drift after 8a's `ReplicaItem` field renames land).**

| Grep pattern (verbatim) | Current state | Edit |
|---|---|---|
| `.push((item.name.clone(), "legendary"));` | Inside `fn check_duplicate_pokemon_buckets`: the replica_items loop pushes the tuple with bucket literal `"legendary"` | **Rename label `"legendary"` → `"replica_item"` AND access field via `item.target_pokemon.clone()`.** (B1 fix: this is unconditional — 8B cannot start until 8A ships per §2's `rg -n 'pub enum SummonTrigger'` pre-condition, so at 8B start 8A's `name→target_pokemon` rename has already landed. There is no "if 8A landed" conditional branch.) |
| `push_finding(report, Finding {` (the call inside `fn check_duplicate_pokemon_buckets`, body containing `rule_id: X003.to_string(),` and the `message: format!("Pokemon '{}' appears in multiple buckets: [{}]` template) | X003 `Finding { .. }` construction (rule_id X003, `buckets` not yet populated) | Add `buckets: <set as Vec<&'static str>>` to the literal. Set is a subset of `["hero", "replica_item", "monster"]` (post-rename labels — identical to what the message prose will print, since message `buckets.join(", ")` reads the same in-scope `Vec<&str>`). |
| `suggestion: Some(format!(` (inside the same X003 `push_finding(report, Finding { ... })`, body containing `"Rename one of the colliding '{}' entries so the name appears in at \`) | The `suggestion` clause — bucket list is computed from the actual colliding buckets, not hardcoded | No structural edit. The suggestion reads the same in-scope `Vec<&str>` the message reads; after the literal rename it enumerates `{hero, replica_item, monster}` automatically. The existing test `fn x003_suggestion_only_enumerates_live_buckets` (verbatim quote: `"suggestion must not reference the deleted \`capture\` bucket"`) continues to pass — it asserts absence of `"capture"` and `"boss"`, not presence of `"legendary"`. |
| `format!("replica_items[{}].sd", item.name)` (in `iter_dice_faces`-adjacent code) + `&item.sd` + `item.template.as_str()` | Template-compat iteration over replica dice | After 8a: `item.target_pokemon`, `item.trigger.dice_faces()`, literal `"thief"` (Chunk 9 chose this constant). |
| `.push((item.name.clone(), "replica_item"));` | Inside `fn check_cross_category_names` / V020: replica_items loop pushes the tuple with bucket literal `"replica_item"` | **Swap field access to `item.target_pokemon.clone()` only; label already canonical.** (B1 fix: unconditional per the same §2 8A-landing gate as the X003 row above.) |
| `// The former \`capture\` bucket was removed along with \`ReplicaItemContainer\`` | Doc comment block above `fn x003_duplicate_pokemon_across_kinds` | No edit. The comment remains accurate. |
| `assert!(x003[0].message.contains("legendary"));` (inside `fn x003_duplicate_pokemon_across_kinds`, xref.rs:965 verified 2026-04-24) | Prose-substring scan of a bucket label in the Finding message | **B2 fix: DELETE this prose-scan and replace with a typed-buckets assertion.** Substring scans of bucket labels in `Finding.message` prose are accepting oracles — any rename (`"legendary"` → `"replica_item"`, future `"capturable"` → `"replica_item"`) silently breaks or silently passes on regression. After §3.3's `Finding` widens with `buckets: Vec<&'static str>`, the canonical assertion is `assert_eq!(x003[0].buckets.as_slice(), &["hero", "replica_item"]);` — typed, order-controlled, stable under future prose edits. The sibling assertion at xref.rs:964 (`assert!(x003[0].message.contains("hero"));`) is the same defect and must be deleted in the same commit. The entity-name assertion at xref.rs:963 (`assert!(x003[0].message.contains("Pikachu"));`) STAYS — it asserts the payload content, not a set-label. |
| `fn x003_suggestion_only_enumerates_live_buckets` body containing `suggestion.contains("hero") && suggestion.contains("legendary")` (xref.rs:1028 verified 2026-04-24) | Positive assertion that the suggestion mirrors the colliding buckets via label-substring scan | **B2 fix: retarget to typed `buckets` — either delete the positive `suggestion.contains(...)` scan and assert `x003[0].buckets.as_slice() == ["hero", "replica_item"]`, OR keep the suggestion scan AS A SECONDARY assertion after the typed-buckets check (the suggestion prose is computed from the `buckets` field, so it changes atomically with the rename). Negative assertions in the same test (`!suggestion.contains("capture")`, `!suggestion.contains("boss")`) STAY — they assert the absence of deleted buckets and are invariant under the relabel.** |
| Every other `message.contains(<bucket-label>)` assertion in `xref.rs` — audit via `rg -n 'message\.contains' compiler/src/xref.rs`, which returned 10 hits on `main` (verified 2026-04-24: xref.rs lines 848, 897, 963, 964, 965, 1070, 1111, 1112, 1231, 1232, 1280 per the ground-truth table in `plans/CHUNK_8_TRIBUNAL_FIXES.md`). | Mix of bucket-label scans + entity-name scans | **B2 fix: audit ALL 10 sites in the same commit.** Bucket-label substring scans (`"hero"`, `"replica_item"`, `"monster"`, `"boss"`, `"legendary"`, `"capture"`) migrate to typed `buckets` / `includes_boss` predicates. Entity-name scans (`"Pikachu"`, `"Goblin"`, numeric literals) STAY — they are payload, not set-label. |
| `fn x003_silent_on_intra_bucket_duplicate` body containing `` "X003 must not fire for intra-bucket duplicates (two legendaries of the same name)" `` | Test affirming intra-bucket duplicates don't fire X003 | Preserve behavior (V020 owns intra-bucket collisions). Optional: rename the panic-message string `"(two legendaries of the same name)"` to `"(two replica_items of the same name)"` for prose consistency — not load-bearing. |

**Populate `buckets` on X003 Findings**: inside `fn check_duplicate_pokemon_buckets`, the in-scope `let mut buckets: Vec<&str> = entries.iter().map(|(_, b)| *b).collect();` line (verified this session by Read) already holds the canonical set. Emit it onto the constructed `Finding`. The three compile-time-constant literals pushed earlier (`"hero"`, `"replica_item"` post-rename, `"monster"`) are all `&'static str`, so the collected `Vec<&str>` coerces to `Vec<&'static str>` via the same iteration. Typed field replaces every downstream message-string substring scan (hook rule #3: one correct line, not N incantations).

**Update the doc-comment block above `fn check_duplicate_pokemon_buckets`** (verified this session by Read, currently reads `// X003 therefore collects buckets as {hero, legendary, monster} — narrower than V020's {hero, replica_item, monster, boss}`): rewrite to name a single canonical three-bucket set once, e.g. `// X003 therefore collects buckets as {hero, replica_item, monster} — identical to V020's labels, narrower than V020's {hero, replica_item, monster, boss} by excluding boss per SPEC §6.3`. No re-listing of the set members elsewhere in the doc.

**Deleted from previous plan draft (retracted)**:
- "Delete `contains("capture")` assert" — no such assert exists; `fn x003_suggestion_only_enumerates_live_buckets` already affirms the absence of `"capture"`.
- "Delete `fn x003_distinguishes_capture_from_legendary_buckets`" — function does not exist in post-main-merge source; `ReplicaItemContainer` was retired pre-Chunk-8, and this test died with it.
- "Test helper `container: ReplicaItemContainer::Legendary`" — already retired pre-merge.

### 3.5 V020 rewrite against typed `includes_boss`

File: `compiler/src/xref.rs`.

Parent §3.5 is authoritative. 8b narrowing:

- V020's `check_cross_category_names` collects the bucket-set per collision using label set `{hero, replica_item, monster, boss}` (verified this session by Read of `fn check_cross_category_names` in `compiler/src/xref.rs`: four `.push((..., "hero"))`, `.push((..., "replica_item"))`, `.push((..., "monster"))`, `.push((..., "boss"))` tuples). After the §3.4 rename, the Pokemon-only sub-set `{hero, replica_item, monster}` is byte-identical to X003's set: if a collision spans ≥2 of these three labels, X003 owns it (per parent §3.5's Pokemon-only narrowing); if the set includes `"boss"`, V020 emits a Finding with `includes_boss: true`.
- **Intra-bucket duplicates** (e.g. two replica items named "Pikachu") still fire V020 with `includes_boss: false` — this is existing behavior, preserved.
- **No bucket-string substring banlist on messages.** T10's `Cast-Iron Ball` synthetic intentionally contains the substring `cast` — a coarse banlist on `"cast" / "capture" / "legendary"` in message-prose scanning would falsely trip; use `finding.buckets` + `finding.includes_boss` predicates instead.

### 3.6 Test migration + new tests

**Inline tests** (`#[cfg(test)] mod tests` in-source):
- `compiler/src/extractor/replica_item_parser.rs` — inline T1, T2, T2a, T2b, T3, T4, T5, T5a, T6, T7, T8, T9, T9a, T9b, T9c, T10, T10a, T11 (parent §5 source-vs-IR + trigger-classification suite). These all target the summon prefilter — NOT the typed NonSummon classifier. 8A.5's T30.1–T30.15 cover per-variant NonSummon round-trip and stay in 8A.5's test surface; 8B does not duplicate them.
- `compiler/src/xref.rs` — inline T15-T21 (parent §3.5 "Tests to keep / write" + "New source-vs-IR tests").

**Integration tests**:
- `compiler/tests/build_options_tests.rs` — the `fn v020_cross_category_source_is_global` test (verified this session by Read) currently constructs a `ReplicaItem` using the **post-main-merge shape**, a 14-field exhaustive literal beginning with `ir.replica_items.push(ReplicaItem {` and fields (quoted verbatim from the Read) `name: "Pikachu".to_string(),`, `template: "Slime".to_string(),`, `hp: Some(4),`, `sd: DiceFaces { faces: vec![DiceFace::Blank] },`, `sprite: SpriteId::owned("pikachu", ""),`, `color: None,`, `tier: None,`, `doc: None,`, `speech: None,`, `abilitydata: None,`, `item_modifiers: None,`, `sticker: None,`, `toggle_flags: None,`, `source: Source::Base,`. **No `container: ReplicaItemContainer::…` field exists** — `ReplicaItemContainer` was retired pre-merge. The import line reads `use textmod_compiler::ir::ReplicaItem;` (no `ReplicaItemContainer`). 8A renamed `name→target_pokemon`, introduced `container_name`, replaced `template`+`sd`+`abilitydata`+`sticker` with `trigger: SummonTrigger` + `enemy_template` + `team_template` + `sticker_stack`. If 8A landed the struct rewrite in this test file already, 8B inherits the post-8A state and edits nothing; if 8A left the literal stale (because the test passed trivially under the stub), 8B rewrites per parent §3.2. Concrete migration (apply only if the current-state Read shows the pre-8A literal):
    - The import `use textmod_compiler::ir::ReplicaItem;` → `use textmod_compiler::ir::{ReplicaItem, SummonTrigger, DiceLocation};` (plus `DiceFaces` / `DiceFace` if not already imported in this test file — grep the file at impl time).
    - The `ir.replica_items.push(ReplicaItem { ... })` literal: replace every retired field. Final shape per parent §3.2:
      - `container_name: "Pikachu".to_string()` (new)
      - `target_pokemon: "Pikachu".to_string()` (renamed from `name`)
      - `trigger: SummonTrigger::SideUse { dice: DiceFaces { faces: vec![DiceFace::Blank] }, dice_location: DiceLocation::OuterPreface }` (replaces `template`, `sd`, `container`, `abilitydata`)
      - `enemy_template: "Wolf".to_string()` (new, any in-scope corpus value)
      - `team_template: "housecat".to_string()` (new)
      - `sticker_stack: None` (renamed + retyped from `sticker: Option<String>`)
      - keep `hp`, `color`, `tier`, `doc`, `speech`, `toggle_flags`, `item_modifiers`, `source`, `sprite` as-is (parent §3.2 preserves these).
    - V020's `check_cross_category_names` behavior under the rewrite: the test still fires V020 because `hero.mn_name == "Pikachu"` and `replica.target_pokemon == "Pikachu"` collide on the Pokemon-uniqueness axis. The assertion (`finding.source.is_none()`, `finding.severity == Severity::Error`) is orthogonal to trigger shape.
- `compiler/tests/roundtrip_baseline.rs` — regenerate baselines. Sliceymon's baseline goes from `replica_items.count: 0 -> 0` (8A.5 stub-summon-prefilter state) to `replica_items.count: 23 -> 23` (real parser: 18 SideUse{OuterPreface} + 1 SideUse{InnerWrapper} + 4 Cast). Format verified this session by Read: the format literal is `"replica_items.count: {} -> {}\n"`. pansaer/punpuns/community stay `0 -> 0`. NonSummon entry counts are 8A.5's surface to regenerate — 8B only regenerates the Summon-axis portion.
- `compiler/tests/integration_tests.rs` — Verification this session (`grep -n "ReplicaItem\|extract_preserves_replica_item_img_data" compiler/tests/integration_tests.rs`) returned only one hit: `assert_eq!(ir1.replica_items, ir2.replica_items, "replica items must roundtrip IR-equal");`. **8B owes no edit to this file unless the real-extractor changes invalidate the equality assertion.** At impl time, re-grep; if no stale comment / test surfaces, drop this file from §4's list.

**Why not duplicate tests from 8A.5**: the per-NonSummon-variant round-trip / distinctness / serde / authoring-exclusion tests (T30.0–T30.15, T31, T32, T34) and Phase-A walker (T33) ship in 8A.5. 8B's tests all target the real-summon-prefilter behavior + the xref Finding widening. No overlap with 8A.5.

---

## 4. Files touched (narrowed vs prior tribunal-approved scope — 8A.5 now owns the emitter + IR authoring paths)

Parent §9.2 header `**Files (6)**:` enumerates 7 bullets underneath (`replica_item_parser.rs`, `extractor/mod.rs`, `builder/mod.rs`, `xref.rs`, `build_options_tests.rs`, `roundtrip_baseline.rs`, `integration_tests.rs` — verified this session by Read of the parent plan). **Post-main-merge, the `Finding` struct lives in `compiler/src/finding.rs` (new file, Chunk 9), not in `xref.rs`** — that file remains 8B's to touch. **But post-8A.5, the emitter / IR / authoring files (`builder/replica_item_emitter.rs`, `ir/mod.rs`, `authoring/non_summon_entry.rs`) are 8A.5's surface and NOT 8B's.** The post-8A.5 list:

| # | File | Change |
|---|---|---|
| 1 | `compiler/src/extractor/replica_item_parser.rs` | Replace 8A.5's placeholder `classify_summon_entry` (always `None`) with the real prefilter per §3.1. Factor `detect_summon_pair` helper. Add inline T1–T11 tests (summon-axis only; 8A.5's T30.N cover NonSummon). |
| 2 | `compiler/src/extractor/mod.rs` | **No edit expected.** 8A.5 wired the `ModifierType::ItemPool` arm already; behavioral difference post-8B is `new_replica_items.len() == 23` for sliceymon vs 0. If a shape edit is needed here, 8A.5 is incomplete — halt. |
| 3 | `compiler/src/builder/mod.rs` | **Validation only.** Confirm dispatch to `emit_itempool` (wired in 8A, typed-NonSummon-aware since 8A.5) produces correct output with real `ir.replica_items`. No expected edit; if one is needed, 8A's emitter rewrite or 8A.5's typed-emit wiring was incomplete. |
| 4 | `compiler/src/finding.rs` | `Finding` widen (§3.3): add `buckets: Vec<&'static str>` + `includes_boss: bool` fields. Struct's real home post-main-merge (not `xref.rs`). |
| 5 | `compiler/src/xref.rs` | X003 + V020 rewrites (§3.4, §3.5): rename X003's middle bucket literal `"legendary"` → `"replica_item"`; populate `buckets` on X003 (label set `{hero, replica_item, monster}`) and V020 (label set `{hero, replica_item, monster, boss}`); populate `includes_boss` on V020 when set contains `"boss"`. Retarget existing tests: `assert!(x003[0].message.contains("legendary"))` → `...contains("replica_item")`; `suggestion.contains("legendary")` → `suggestion.contains("replica_item")`. Add inline T15–T21. |
| 6 | `compiler/tests/build_options_tests.rs` | Audit `fn v020_cross_category_source_is_global`'s `ReplicaItem` literal against the 8A post-merge shape. Edit only if 8A did not already migrate it. |
| 7 | `compiler/tests/roundtrip_baseline.rs` | Regenerate baselines: sliceymon `replica_items.count: 23 -> 23`; pansaer/punpuns/community stay `0 -> 0`. NonSummon-axis baseline fields (if any) are 8A.5's surface to regenerate. |
| 8 | `compiler/tests/integration_tests.rs` | **Contingent edit.** Verification this session via `grep -n 'replica_items\|ReplicaItem' compiler/tests/integration_tests.rs` returned a single hit: `assert_eq!(ir1.replica_items, ir2.replica_items, "replica items must roundtrip IR-equal");`. Only the `replica_items` equality assertion remains, which is expected to survive. If `cargo test` fails on this file, re-grep at impl time. If not, drop this file from the list. |

**Ownership boundary (B9 fix — collapsed)**: 8A ships the IR schema + authoring
builders + stub extractor/emitter; 8A.5 ships the typed `NonSummonEntry` schema,
emitter dispatch, Phase-A walker, `split_itempool_entries`, `classify_non_summon_entry`,
CRUD-op migration, and `NonSummon(NonSummonEntry)` retype. 8B does not edit any
file owned by those chunks; if a 8B commit touches IR/authoring/NonSummon-emitter
surface, stop — either 8A / 8A.5 shipped incomplete or 8B's scope is miscalibrated.

**Size**: 5 implementation/test files (1–5) + 3 test-migration files (6–8). Inside the AI-dev persona's 5-file soft cap on the implementation side; test migrations do not multiply the cap. Narrower than the prior tribunal-approved 8-file list (dropped `ir/mod.rs` + `builder/replica_item_emitter.rs` + `util.rs` + `authoring/non_summon_entry.rs` because 8A.5 now owns them).

---

## 5. Enumerated tests (lifted from parent §5; no renumbering)

Each test name below is the exact Rust `fn` name. Parent §5 test-file-placement contract: semantic prefix `extractor::` / `xref::` → inline `#[cfg(test)] mod tests`, bare fn name in Rust.

### 5.1 Source-vs-IR roundtrip (inline in `replica_item_parser.rs`)
- [ ] **T1**. `itempool_summon_entry_sideuse_outer_roundtrips_ivysaur` — Ivysaur entry from Pokeballs Part 1 (Great Ball / SideUse{OuterPreface}). Byte-equal extract→build; assert `target_pokemon == "Ivysaur"`, `enemy_template == "Wolf"`.
- [ ] **T2**. `itempool_summon_entry_cast_roundtrips_silver_wing` — Silver Wing entry (Cast). Byte-equal. Named by container, not target (Lugia).
- [ ] **T2a**. `itempool_summon_entry_sideuse_inner_roundtrips_master_ball_question` — `Master Ball?` (trailing `?`). Assert `container_name == "Master Ball?"`, `target_pokemon == "Arceus"`, `enemy_template == "Dragon"` (capital), `team_template == "Housecat"` (capital), dice = `34-10:34-10:34-8:34-8:34-5:34-5`, `matches!(item.trigger, SummonTrigger::SideUse { dice_location: DiceLocation::InnerWrapper, .. })`.
- [ ] **T2b**. `sticker_chain_roundtrips_within_summon_entry` — the Caterpie SideUse entry inside Pokeballs Part 1 carries `.i.mid.sticker.(right.hat.statue)#togfri`; extract through `ModifierChain::parse`, emit byte-equal. If chain parser cannot round-trip, the `sticker: Option<String> → sticker_stack: Option<ModifierChain>` type change from 8a defers — flag 8a as incomplete.
- [ ] **T3**. `itempool_full_pool_roundtrips_pokeballs_part_1` — full pool (11 entries: 10 SideUse{OuterPreface} balls including `Master Ball` (Mewtwo) + 1 SideUse{InnerWrapper} `Master Ball?` (Arceus)). Byte-equal. Asserts `Master Ball` and `Master Ball?` are distinct entries with different `container_name`, `target_pokemon`, and `dice_location`.
- [ ] **T4**. `itempool_full_pool_roundtrips_pokeballs_part_2` — 8 entries, all SideUse{OuterPreface}. Byte-equal.
- [ ] **T5**. `itempool_full_pool_roundtrips_summons_part_1` — 2 entries: Rainbow Wing + Silver Wing (both Cast). Byte-equal.
- [ ] **T5a**. `itempool_full_pool_roundtrips_summons_part_2` — 2 entries: Blue Orb + Red Orb (both Cast; Red Orb's `team_template == "Statue"`). Byte-equal.
- [ ] **T6**. `non_summon_entry_stays_structural` — entry without `hat.egg` + `vase.(add.((replica.…)))` pair demotes to `ItempoolItem::NonSummon { name, tier, content }`. Round-trip byte-equal via `emit_itempool`.
- [ ] **T7**. `half_summon_entry_stays_structural` — synthesize entry with `hat.egg` but no matching vase-add (or mismatched Pokemon). Demotes to `NonSummon`. Proves conjunctive detection.

### 5.2 Trigger classification (inline in `replica_item_parser.rs`)
- [ ] **T8**. `all_sideuse_outer_entries_classify_as_outer_preface` — 18 per §1.1 count; each → `SummonTrigger::SideUse { dice_location: OuterPreface, .. }`.
- [ ] **T9**. `all_cast_entries_classify_as_cast` — 4 per §1.1 count; each → `SummonTrigger::Cast { .. }`. Must NOT match Evasion Orb / Itemizer Orb / Foe Seal Orb / Two Edge Orb (non-summon).
- [ ] **T9a**. `master_ball_question_classifies_as_inner_wrapper` — in-scope `Master Ball?` (byte ~274161, NOT the boss-block hit at ~106078) → `SummonTrigger::SideUse { dice, dice_location: InnerWrapper }` with dice extracted from inner `.i.(hat.Thief.sd.<faces>)`. Fails if classifier silently routes to OuterPreface with zero dice.
- [ ] **T9b**. `inner_wrapper_count_is_corpus_exact` — across all four working mods, `ir.replica_items.iter().filter(|r| matches!(r.trigger, SideUse { dice_location: InnerWrapper, .. })).count() == 1`. Proves parent §3.3 rule 1 itempool-scoping — the boss-block `Master Ball?` must NOT surface.
- [ ] **T9c**. `unclassifiable_wrapper_shape_demotes_to_nonsummon_with_finding` — synthetic entry with valid egg + matching vase-add but no outer preface AND no `cast.sthief` AND no inner `.i.(hat.Thief.sd.<faces>)`. Extraction: zero `ReplicaItem`, one `Finding { code: "W-REPLICA-TRIGGER-UNCLASSIFIED", severity: Severity::Warning, .. }`, entry demotes to `NonSummon` round-tripping byte-equal. Proves permissive (SPEC §3.3), not `CompilerError::classify`.
- [ ] **T10 (B7 fix — strengthen)**. `trigger_classification_reads_inner_payload_not_name` — **source-vs-IR** (hook rule #2). The prior synthetics (`Cast-Iron Ball`, `Silver Wing Deluxe Ball`) are still weak against a "container contains Ball / contains Cast / contains Wing — use that" hybrid heuristic. Use these adversarial synthetics instead:
      - **Synthetic A**: container `"Thief Ball"` (contains `Ball` — would tempt a "has-Ball-substring → SideUse" heuristic) **WITH a Cast-shape payload** (`cast.sthief.abilitydata.(thief.sd.182-25:0:0:0:76-0:0...)`) — classifier MUST return `SummonTrigger::Cast { .. }`.
      - **Synthetic B**: container `"Cast Item"` (contains `Cast` — would tempt a "has-Cast-substring → Cast" heuristic) **WITH a SideUse{OuterPreface}-shape payload** (`hat.replica.Thief.n.<Pokemon>.sd.<faces>.i.(hat.(replica.Thief.i.(all.(left.hat.egg.wolf.n.<Pokemon>...`) — classifier MUST return `SummonTrigger::SideUse { dice_location: DiceLocation::OuterPreface, .. }`.
      Each synthetic falsifies BOTH "name-substring wins" and "name-substring is a soft hint" heuristics — the only way to pass both synthetics is to read the payload bytes. A classifier whose dispatch peeks at `container_name` for routing hints will fail one of the two.
- [ ] **T10a**. `target_pokemon_preserves_source_bytes_exactly` — **source-vs-IR** (hook rule #2). Use actual `Ho Oh` entry from sliceymon (capital H, embedded space); assert `target_pokemon == "Ho Oh"` exactly — NOT `"Ho-Oh"`, `"hooh"`, `"HoOh"`. Second synthetic with `n.Anorith ` (trailing space, corpus-evidenced); preserve space through round-trip. Fails if any implementer reaches for a Pokemon registry / case-title normalizer / hyphen canonicalizer.
- [ ] **T11**. `trigger_classification_is_total_per_entry` — exact per-entry classification across all 23 sliceymon summon entries. Not "at least one of each" — that would leave silent mis-classification undetected.

### 5.3 Xref 3-bucket shape (inline in `xref.rs`)

**Canonical bucket labels (post-§3.4 rename — unified across both rules):**
- X003 uses `{hero, replica_item, monster}`.
- V020 uses `{hero, replica_item, monster, boss}`.

X003's labels are byte-identical to the Pokemon-only subset of V020's. Tests below assert against these labels verbatim.

- [ ] **T15**. `x003_one_replica_item_bucket_across_triggers` — IR with hero "Pikachu" + SideUse{OuterPreface}/Pikachu + SideUse{InnerWrapper}/Pikachu + Cast/Pikachu fires X003 **exactly once** with `buckets == vec!["hero", "replica_item"]` and no X003 duplicates. V020 **also fires** (intra-replica-duplicate), but the invariant here is that X003 deduplicates the three-trigger replica collision to a single bucket — not that V020 is silent. Assert BOTH: X003 count == 1 AND X003's `buckets` is the 2-element set, AND V020's count reflects the intra-bucket duplicate semantics parent §3.5 preserves (the exact V020 count is parent-§3.5-authoritative; T15 asserts `v020.count >= 1`, not `== 1`, to avoid pinning V020's dedup policy here). Proves trigger/dice-location granularity does not leak into Pokemon uniqueness.
- [ ] **T16**. `x003_silent_on_intra_replica_item_trigger_variants` — one of each SideUse{OuterPreface}/SideUse{InnerWrapper}/Cast all named "Pikachu" (no hero) — intra-`replica_item` duplicates. X003 must be **silent** (per the existing test `fn x003_silent_on_intra_bucket_duplicate`, which establishes that intra-bucket duplicates are V-rule territory, not X003's). V020 fires with `includes_boss: false`. All three trigger variants exercised — a 2-variant test leaves the third silently broken. Assert `x003.is_empty() && v020.len() >= 1 && v020[0].includes_boss == false`.
- [ ] **T17-T18**. `v020_silent_on_cross_bucket_pokemon_{hero_replica, hero_monster, replica_monster, case_insensitive}` — one replica is enough; `dice_location` irrelevant to bucketing.
- [ ] **T19**. `v020_still_fires_on_boss_replica_collision` — asserts `finding.includes_boss == true`.
- [ ] **T20 (B6 fix)**. `v020_and_x003_coexist_when_collision_spans_boss_and_pokemon_buckets` — asserts the **co-fire INVARIANT** (why the pair is permitted), not just "both fired". Rule-id equality alone is an accepting oracle per checklist rule #5. Concrete predicate:
      ```rust
      // X003 fires iff collision includes ≥2 of {hero, replica_item, monster}.
      // V020 fires iff collision includes ≥2 of {hero, replica_item, monster, boss}.
      // Both fire iff collision spans ≥1 Pokemon bucket AND boss bucket.
      if !x003.is_empty() && !v020.is_empty() {
          for f in &x003 {
              assert!(f.buckets.iter().all(|b|
                  ["hero", "replica_item", "monster"].contains(b)));
              assert!(f.buckets.len() >= 2);
          }
          for f in &v020 {
              if f.includes_boss {
                  let has_pokemon = f.buckets.iter()
                      .any(|b| ["hero", "replica_item", "monster"].contains(b));
                  assert!(has_pokemon,
                      "V020 with boss must also have ≥1 Pokemon bucket to co-fire with X003");
              }
          }
      }
      ```
      No substring scan on message prose anywhere in the assertion.
- [ ] **T21 (B6 fix)**. `no_double_fire_on_working_mods` — same typed-field INVARIANT predicate as T20, applied across all four mods' cross-check reports. Permissive co-fire (both fire, both legal) is allowed; the invariant it encodes is WHY: the collision spans both axes. A bare "both fired, ship it" oracle is rejected.
- [ ] **T20a / T21a (B8 fix — exhaustive failure-mode matrix test)**. The §3.1 failure-mode matrix enumerates 4 rows (5 pre-B3 fix — the `W-REPLICA-TRIGGER-UNCLASSIFIED` row is deleted per B3). No single existing T-number walks every row; without exhaustive coverage, a regression on any row passes "because that row has no test". New test:
      ```rust
      #[test]
      fn failure_mode_matrix_exhaustive() {
          // Rows from §3.1 (post-B3):
          //   1. conjunctive pair + recognized wrapper → Summon(i) + ReplicaItem, no Finding
          //   2. no conjunctive pair → NonSummon(typed V1–V15), no Finding
          //   3. no conjunctive pair + 8A.5 classifier can't type → NonSummon(Unclassified) + W-REPLICA-NONSUMMON-UNCLASSIFIED (8A.5's code)
          //   4. Cast with missing inner dice → Err(CompilerError::classify)  // hard error, strict path
          let cases = matrix_cases();  // synthetic: (entry_bytes, expected_item, expected_findings, expected_err)
          for (i, (entry, expected_item, expected_findings, expected_err)) in cases.iter().enumerate() {
              let got = extract_from_itempool(entry, 0, 0);
              // Match on the tagged 4-tuple; no regex / substring scan.
              assert_matrix_row(i + 1, &got, expected_item, expected_findings, expected_err);
          }
      }
      ```
      The matrix pins the dispatch contract beyond the individual T-tests (T1–T11 touch rows 1 and 4; T6/T7 touch row 2; 8A.5's T30 touches row 3).

### 5.4 Baseline + audit
- [ ] **T22**. `roundtrip_baseline` tests pass with regenerated baselines. Sliceymon: `replica_items.count: 23 -> 23`. pansaer / punpuns / community: `replica_items.count: 0 -> 0`.
- [ ] **T23**. `cargo run --example roundtrip_diag` reports `Status: ROUNDTRIP OK` for all four mods; sliceymon stdout `Replicas ir1=23`. Per-trigger breakdown (`SideUse{OuterPreface}=18 SideUse{InnerWrapper}=1 Cast=4` for sliceymon) is 8c's optional edit.
- [ ] **T23a**. `no_false_positive_on_boss_hat_egg` — punpuns (13 lines / 19 occurrences of `hat.egg.`) and community (9 lines / 9 occurrences) produce zero `ReplicaItem`. Fails if any match surfaces — proves parent §3.3 rule 1 itempool-scoping.
- [ ] **T27**. `non_sliceymon_mods_produce_zero_replica_items` — full extractor on pansaer/punpuns/community yields `ir.replica_items.len() == 0` for each. Covers (verified this session via `grep -c 'cast\.sthief' working-mods/*.txt`): punpuns 3 `cast.sthief` hits (all inside `ph.b…` boss abilitydata — grep context `ph.bAlpha`, `ph.bBramble`, `ph.bTeddy` etc. confirmed this session), community 1 `cast.sthief` hit (inside a top-level `.n.Mental Defense` modifier — grep context this session: `(mid.mid.hat.thief.i.(cast.sthief.abilitydata.(mage.sd.128-2.i.mid.left...).n.Mental Defense.tier.0...` — NOT an itempool member, classified by existing non-itempool route per parent §1.1), pansaer 0 hits. T27 fails if any of these surfaces as a `ReplicaItem`, proving parent §3.3 rule 1 itempool-scoping holds across all four mods.

Parent §5 also includes T24/T25/T26 (authoring builder — shipped in 8A), T28 (ops re-index — shipped in 8A; NonSummon-axis extension shipped in 8A.5 per its §5.7), T30.0–T30.15 (NonSummon per-variant round-trip — shipped in 8A.5), T31/T32/T34 (NonSummon distinctness/serde/authoring-exclusion — shipped in 8A.5), T33 (Phase-A walker — shipped in 8A.5). **8B ships T1–T11, T15–T21, T22, T23, T23a, T27 — summon-axis and xref-widening tests only.** No overlap with 8A or 8A.5. (Parent §5's T12/T12a/T13/T14 retirement greps were deleted — Rust's type system enforces those retirements at compile time; the standalone grep tests duplicated that enforcement.)

---

## 6. Structural check (hook rules)

### 6.1 Authority diff (hook rule #1)

Against parent `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md`:

- **§3.3 rules 1–4**: restated here as `§3.1 bullet list`; claims are strictly narrower (same rules, scoped to "8B ships the real summon prefilter (NonSummon classifier is 8A.5's)", no new rule).
- **§3.5 Finding widening**: restated here as `§3.3`; `buckets: Vec<&'static str>` + `includes_boss: bool` field names and serde attrs are byte-identical to parent.
- **§3.5 X003/V020 edits**: restated as `§3.4`/`§3.5`; each edit is located by a verbatim quoted grep pattern against `compiler/src/xref.rs` verified this session, not line numbers.
- **§9.2 file count "6"**: parent §9.2's header contradicts its enumerated 7-file list AND predates the 8A → 8A.5 → 8B split. The 8A.5 PR already rewrites parent §3.1's "transitional `content: String`" clause (per 8A.5 §6.1); 8B's own PR amends parent §9.2 to reflect the narrowed 8B file list (§4 of this plan) and to insert the `compiler/src/finding.rs` bullet (Chunk-9 post-merge struct split that the parent also missed).
- **§3.5 X003 bucket set**: parent §3.5 prose names the set `{hero, replica_item, monster}`. Post-Chunk-9 `compiler/src/xref.rs` currently names X003's set as `{hero, legendary, monster}`. Resolved by user decision this PR: 8B renames X003's middle label from `"legendary"` to `"replica_item"` (see §3.4), so X003 and V020 share a byte-identical Pokemon-only labelset. No further amendment required.

Against sibling `plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`:

- 8A.5 §9 handoff greps are mirrored verbatim in 8B §2 pre-condition block 3 (NonSummonEntry declared, `NonSummon.content:String` retired, `split_itempool_entries` shipped, `classify_non_summon_entry` shipped, `W-REPLICA-NONSUMMON-UNCLASSIFIED` wired, T30.0 green). If 8A.5 tightens its gate, 8B inherits the tightening automatically.
- 8A.5 §3.2.2 authors the placeholder `classify_summon_entry: always None` that 8B replaces. The contract surface (fn name, signature) is 8A.5's to ship; 8B swaps the body only.

One parent correction required (§9.2 file count + `finding.rs` bullet) in 8B's PR.

### 6.2 Source-vs-IR test present? (hook rule #2)

Yes:
- **T10** — classification reads payload not container name.
- **T10a** — `target_pokemon` preserves source bytes exactly (catches registry/normalizer drift).
- **T7** — half-summon entry stays structural (catches disjunctive-detection regression).
- **T9a** — Master Ball? routes to InnerWrapper, not silent fallback to OuterPreface with zero dice.
- **T23a** — boss/event `hat.egg.` outside itempool does NOT surface as `ReplicaItem` (proves the gate is itempool-scoped, not global).

All five test cases fail under an IR-equality-only round-trip if the extractor silently reaches for derived / registry / canonical data instead of source bytes. The hook rule is satisfied.

### 6.3 Structural smells (hook rule #3)

- **Collapsing different-invariant paths**: the `W-REPLICA-TRIGGER-UNCLASSIFIED` (permissive, Warning Finding) and `CompilerError::classify` (strict, hard error) paths are **preserved as distinct** per parent §3.3. 8b does not collapse them; the split covers genuinely different failure modes.
- **Duplicated N-line incantations**: the three trigger-classify branches share the egg + vase-add detector via a single `detect_summon_pair` helper (§3.1); the three emit paths share a single shared-payload emitter (8a's `emit_replica_item`). No pasted incantation across branches.

### 6.4 Parallel file check against 8c

8c files: `SPEC.md`, `plans/PLATFORM_FOUNDATIONS_PLAN.md`, `.claude/settings.json`, `compiler/examples/roundtrip_diag.rs`, `compiler/examples/drift_audit.rs`.

8b files: all under `compiler/src/` and `compiler/tests/`; zero overlap with 8c's list. 8b and 8c run concurrently after 8a lands, merge independently.

---

## 7. Risks

- **Summon prefilter regression (new in this scope)** — replacing 8A.5's `classify_summon_entry: always None` with the real prefilter is the single load-bearing behavioral change in 8B. A bug in the prefilter that mis-classifies a non-summon entry as a summon (false positive) leaks a garbage `ReplicaItem` into `ir.replica_items`; a bug that rejects a real summon entry (false negative) leaves 8A.5's typed classifier to try to fit the wrapper shape into V1–V15, either succeeding with an *incorrect typing* (silent IR corruption) or producing `Unclassified { source_bytes }` and tripping 8A.5's zero-budget ratchet (T30.0 — visible failure). Mitigation: (a) T11's "exact per-entry classification across all 23 sliceymon summon entries" pins the positive count; (b) T23a/T27 pin zero false positives on pansaer/punpuns/community; (c) T10 source-vs-IR proves the prefilter reads inner payload bytes, not container surface names.
- **Chain-interior scalar leaks** — parent §3.3 rule 4. Without `depth_aware = true` on every scalar extractor, chain-interior `.hp.` / `.col.` / `.sd.` substrings leak into top-level `ReplicaItem` fields. Caught by T1-T5a byte-equality but only loudly if the scalar field values differ source vs output — a silent leak where the chain-interior value equals the top-level value would pass. Mitigation: T10a's `Ho Oh` / `Anorith ` variants deliberately diverge surface bytes from any registry-derived value; any scalar extractor reaching for a registry would fail T10a.
- **Finding-widening blast radius** — 16 `Finding { .. }` construction sites (verified this session via `grep -n 'Finding {' ...`: 13 in `compiler/src/xref.rs` + 1 `report.errors.push(Finding {` in `compiler/src/lib.rs` + 1 `warnings.push(Finding {` in `compiler/src/ir/merge.rs` + 1 `let f = Finding {` in `compiler/tests/build_options_tests.rs` inside `fn finding_json_omits_absent_source`). Rust's struct-update syntax with `#[derive(Default)]` means default-init is backward-compatible, but any site using `Finding { rule_id, severity, message, field_path, ... }` with an exhaustive field list (no `..Default::default()`) would break with two new fields. Mitigation: `cargo build` with the two fields added and no other change is the verification step before any further edit; if build fails, every listed-but-incomplete site is flagged in one pass. Prior plan draft claimed "34 sites" — retracted as fictional.
- **X003 bucket-label rename blast radius** — the §3.4 rename touches (verified this session by Read): one string literal `.push((item.name.clone(), "legendary"))` inside `fn check_duplicate_pokemon_buckets`; one doc-comment block above that function (`// X003 therefore collects buckets as {hero, legendary, monster} — narrower than V020's {hero, replica_item, monster, boss}`); one `assert!(x003[0].message.contains("legendary"));` inside `fn x003_duplicate_pokemon_across_kinds`; one `suggestion.contains("hero") && suggestion.contains("legendary")` inside `fn x003_suggestion_only_enumerates_live_buckets`; optional prose in `fn x003_silent_on_intra_bucket_duplicate`'s panic message. All other X003 tests key off absence of `"capture"` / `"boss"` and are invariant under the rename.
- **X003 test deletions — retracted.** Previous plan draft said `fn x003_distinguishes_capture_from_legendary_buckets` and assorted `"capture"` asserts must be deleted. None exist in current `compiler/src/xref.rs`: the `capture` bucket and its distinguishing test were already retired pre-main-merge (Chunk 9 cleanup, see the xref.rs comment `// The former \`capture\` bucket was removed along with \`ReplicaItemContainer\``). The surviving tests (`fn x003_duplicate_pokemon_across_kinds`, `fn x003_suggestion_only_enumerates_live_buckets`, `fn x003_silent_on_intra_bucket_duplicate`) all survive post-8b with the per-site asserts retargeted per the §3.4 edit table.
- **Baseline regeneration drift** — sliceymon's new `replica_items.count: 23 -> 23` is correct per parent §1.1 count. If the first build of 8b yields a different number, the parser has classified wrong: either (a) a boss-block `Master Ball?` or `hat.egg.` leaked in (T23a/T27 catch), or (b) a SideUse{InnerWrapper} / Cast miscount (T9b/T11 catch). Do NOT regenerate the baseline to silence the diff — investigate per the named test.
- **SPEC §3.2 deferral — CLOSED by 8A.5.** Prior 8B plan drafts (pre-8A.5) carried risk entries for the transitional `NonSummon.content: String` raw-passthrough hatch surviving into 8B's extractor. 8A.5 retired the transitional variant and replaced it with typed `NonSummonEntry` + zero-Unclassified ratchet; 8B inherits a closed-SPEC-§3.2 state. These risk entries are dropped.
- **Upstream gate (8A OR 8A.5) quietly incomplete** — §2 pre-conditions are the gate. If any `rg` expected-0 check returns ≥1 or expected-≥1 returns 0, halt. Shipping 8B against incomplete upstream state means some 8B tests pass "by accident" against stale code, leaving a latent regression. The specific failure surface widens vs the prior tribunal-approved plan (8A only) because 8A.5 adds its own set of greps — `NonSummonEntry` declared, `NonSummon.content: String` retired, `split_itempool_entries` shipped, `classify_non_summon_entry` shipped, T30.0 ratchet green.

---

## 8. Self-verification checklist (AI executes before completion)

- [ ] Every §3.4 grep pattern relocates a unique target in current `compiler/src/xref.rs` at impl start (8a/8A.5 may drift content post-merge).
- [ ] §2 pre-conditions all pass before any 8B edit lands — both the 8A gate (SummonTrigger/DiceLocation/ItempoolItem types present, ReplicaItemContainer retired, stub `ItempoolExtraction` in place) AND the 8A.5 gate (`NonSummonEntry` declared, `NonSummon.content: String` retired, `split_itempool_entries` in `util.rs`, `classify_non_summon_entry` in `replica_item_parser.rs`, `W-REPLICA-NONSUMMON-UNCLASSIFIED` Finding code wired, T30.0 ratchet green).
- [ ] `Finding` struct widened in the first commit; `cargo build` clean with ONLY the field addition and no other change, as a separate verification step.
- [ ] Every X003 Finding constructor populates `buckets` with a `&["hero", "replica_item", "monster"]` subset (post-§3.4 rename); V020 constructors populate `buckets` with a `&["hero", "replica_item", "monster", "boss"]` subset and populate `includes_boss: true` when the set includes `"boss"`. No other rule populates either field.
- [ ] `detect_summon_pair` helper factored per §3.1 structural-smell mitigation; the three trigger-classify branches differ only in dice extraction + variant construction.
- [ ] **(B5 fix) Helper usage enforced by grep**: run `rg -c 'detect_summon_pair' compiler/src/extractor/replica_item_parser.rs` and expect ≥3 hits (one call per trigger branch: Cast, SideUse{OuterPreface}, SideUse{InnerWrapper}) plus one definition site — total ≥4. If <4, the helper was inlined at a callsite and the N-line-incantation structural smell (hook rule #3) persists. The inline fix is to extract the duplicated bytes back into the helper before the PR lands.
- [ ] Dispatch order in `extract_from_itempool` per §3.1 failure-mode matrix: summon prefilter → (if `Ok(Some)`) emit Summon + ReplicaItem; (if `Ok(None)`) delegate to 8A.5's `classify_non_summon_entry` and emit NonSummon; (if `Err`) propagate.
- [ ] T10 and T10a present and failing without the depth-aware / source-preserving code paths, passing with them. Confirmed by deliberately breaking each code path locally before landing and watching the right test fail.
- [ ] Baselines regenerated by running `cargo test`, inspecting the diff in `compiler/tests/baselines/roundtrip/*.baseline`, confirming sliceymon shows `23 -> 23` exactly (NOT `21 -> 21`, NOT `22 -> 22`). pansaer/punpuns/community show `0 -> 0`.
- [ ] `cargo run --example roundtrip_diag` reports `Status: ROUNDTRIP OK` for all four mods; sliceymon `Replicas ir1=23`.
- [ ] `rg -c 'ReplicaItemContainer|ItemPoolEntry|source_byte_range|item\.template\b|item\.name\b|item\.sticker\b' compiler/` returns 0.
- [ ] `rg -c 'NonSummon\s*\{\s*(name|content|body)\s*:' compiler/src/` returns 0 (the transitional 8A shape is fully retired — 8A.5's gate, re-checked at 8B ship).
- [ ] No substring-scan on X003/V020 Finding `message` text survives in `#[cfg(test)] mod tests` of `xref.rs` or in `compiler/tests/*.rs` — all co-fire / bucket assertions use `finding.buckets` / `finding.includes_boss`.
- [ ] No files on the "NOT 8B's" list (§4) were edited in 8B's commits. If any was, re-scope.
- [ ] Parent plan §9.2 file count updated (reflects the 8A → 8A.5 → 8B split and 8B's narrowed list) in the same PR.

---

## 9. Verification gate (parent §9.2 + 8A.5 §9 handoff inheritance)

Ship only if all pass:

- `cargo build` + `cargo clippy` clean.
- `cargo test` passes (including 8B's T1–T11, T15–T21, T22, T23, T23a, T27; plus 8A.5's T30.0–T30.15, T31, T32, T33, T34 which must stay green after 8B's prefilter lands).
- All four mods roundtrip byte-equal. Sliceymon: `replica_items.count: 23 -> 23`; `roundtrip_diag` stdout `Replicas ir1=23`. pansaer/punpuns/community: `0 -> 0` / `Replicas ir1=0`.
- `rg -c 'ItemPoolEntry|source_byte_range' compiler/src/` returns 0.
- `rg -c 'NonSummon\s*\{\s*(name|content|body)\s*:' compiler/src/` returns 0 (inherited from 8A.5; re-asserted at 8B ship).
- 8A.5's zero-Unclassified budget (T30.0) stays green — 8B's real summon prefilter must not leak a real-summon entry into the NonSummon classifier where it becomes an `Unclassified`.
- X003's typed `buckets` field is always a subset of `["hero", "replica_item", "monster"]` (post-§3.4 rename; byte-identical to V020's Pokemon-only subset); V020's typed `buckets` is a subset of `["hero", "replica_item", "monster", "boss"]`; V020's `includes_boss: bool` is true only for boss-involving collisions.
- No bucket-string substring banlist anywhere (T10 `Cast-Iron Ball` synthetic would falsely trip any coarse banlist — typed fields only).

**Dependency chain (confirmed by user this PR)**: 8A → 8A.5 → 8B → 8C. 8B cannot start until 8A.5 has landed; 8B and 8C run concurrently after 8A.5 ships.

If any item fails, fix in-place before the PR lands. Do not defer to 8c.
