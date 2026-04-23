# Chunk 8: `ReplicaItem` redesign — trigger-based IR, itempool extraction, retire unevidenced variants

> **Status**: draft 2026-04-22. Supersedes the V020 capture/legendary narrowing that PR #10 attempted (now closed without merging). This is the foundations plan's Chunk 8 entry.

> **Scope**: the Rust compiler's IR for Pokemon-summon items, the extractor/classifier/emitter paths that feed it, V020/X003 cross-reference rules that bucket it, and SPEC.md prose that describes it.

## 1. Authority and evidence

### 1.1 What the mod actually does

`working-mods/sliceymon.txt` was grepped entry-by-entry across every `itempool.((…))`. Two structural shapes exist, cleanly separated:

| Inner trigger shape | Count | Container names |
|---|---|---|
| `hat.replica.Thief.n.<Pokemon>.sd.<faces>…` — equipping replaces hero sides with summon-sides; summon fires when the player **uses** one of those sides during a fight | 17 | Great, Master, Poke, Ultra, Fast, Friend, Heavy, Level, Lure, Moon, Dive, Luxury, Nest, Premier, Timer, Dusk, Quick, Dream **Ball** |
| `cast.sthief.abilitydata.(thief.sd.<faces>…)` — equipping grants a spell; summon fires when the player **casts** it | 4 | Rainbow Wing, Silver Wing, Blue Orb, Red Orb |

Zero overlap. No ball uses `cast.sthief`; no wing/orb uses the `hat.replica.Thief.n.<Pokemon>` side-use shape. The two are siblings of one mechanic (summon a Pokemon as a monster → on defeat, vase-add's `replica.<team-template>.n.<Pokemon>` joins the team for the rest of the run), differing only in which player action verb triggers the summon.

**Shared payload** — both shapes, in the same position within each entry, carry identical summon/defeat/join pipeline bytes:

```
hat.egg.<enemy-template>.n.<Pokemon>.doc.<intro prose>.img.<sprite>.hp.<hp>.sd.<faces>
  .i.<sticker / facade stack>
  .i.t.vase.(add.((replica.<team-template>.n.<Pokemon>.hp.<hp>.col.<col>.tier.<tier>.sd.<faces>
    …speech.<speech>).mn.<Pokemon>))
```

### 1.2 What the mod does NOT do

Top-level `item.<Template>…` modifiers exist in sliceymon (`item.t.jinx.Summon`, `item.hat.barbarian.sd`, `item.k.growth.t`, `item.determination.t.jinx`, `item.handcuffs.part.1`, etc.) — all of them are **accessory/template definitions**, none are Pokemon-summon items.

Corpus-wide count of comma-split top-level modifiers starting with `item.`:

```
sliceymon: 0    pansaer: 0    punpuns: 0    community: 0
```

No mod in the working-mods corpus uses the shape the current `parse_legendary` gate expects.

### 1.3 Where the IR currently sits relative to that evidence

| IR / parser artifact | Corpus instances | Notes |
|---|---|---|
| `ModifierType::Legendary` (`classifier.rs`) + `starts_with_ci(modifier, "item.")` gate | 0 | Models a top-level `item.<Pokemon>…` shape that appears in no mod. |
| `parse_legendary` (`extractor/replica_item_parser.rs`) | 0 | Dispatch target of the dead gate; untested against real-mod input. |
| `ReplicaItemContainer::Legendary` variant (`ir/mod.rs`) | 0 | Carries a mechanical claim ("persistent ally with spell") that contradicts the mod — in-mod, the Wing/Orb items summon an enemy that must be defeated before joining, same as balls. |
| `ReplicaItemContainer::Capture { name }` variant | 0 via the current `ReplicaItem` route | Every Pokemon-summon entry in the corpus is **inside** `itempool.((…))`, which the extractor currently keeps as opaque structural text. `roundtrip_diag` reports `Replicas ir1=0` for all four mods. |
| `ReplicaItem` struct | 0 data instances across all four working mods | Consequence of the above: the entire IR type is exercised only by synthetic test helpers. |

### 1.4 Authority conflicts to resolve in this chunk

1. **SPEC.md line 342** says `ReplicaItem — … Two kinds: Capture (one-shot, mid-fight, via ball-style item) and Legendary (persistent ally with spell).` The corpus shows neither "one-shot mid-fight" nor "persistent ally" matches the actual mechanic. Both kinds summon an enemy that on defeat is vase-added to the team via `replica.<team-template>`; both require a kill before the Pokemon joins. The action-verb differs (`use-side` vs `cast-spell`), not the downstream pipeline.
2. **SPEC.md §6.3 line 246** says `A Pokemon may exist in at most one of: heroes, replica items (captures / legendaries), monsters.` The bucket `replica items` is singular — the parenthetical `(captures / legendaries)` is prose gloss. Chunk 6 reified the parenthetical as an IR enum (`ReplicaItemContainer`). That was an extrapolation from prose, not from mod bytes.
3. **`PLATFORM_FOUNDATIONS_PLAN.md` §F9** already records in its own Lessons block that restating a canonical set inside a §F / chunk must be identical or strictly narrower, never contradictory. This chunk records a sibling: **variants must be evidenced; zero corpus instances = zero variants.**

## 2. Dependencies

- **Chunk 6** (merged as PR #7) landed `ReplicaItemContainer::{Capture, Legendary}` to `main`. This chunk retires that enum. No merge-ordering issue — the retirement is a straightforward delete-and-replace.
- **PR #10 (V020 capture/legendary narrowing)** is closed without merging. Its only genuinely useful fix — the boss-bucket V020 invariant in `no_double_fire_on_working_mods` — is folded into this chunk's §3.5.
- **Chunk 10 (BooleanPhase phase routing)** is authored in a parallel session and scoped to `ph.b` modifiers (`PorygonItem` / `DittoItem` in sliceymon; broader BooleanPhase uses across all mods). No file overlap expected with this chunk — Chunk 10 touches a different classifier gate (`ph.b.`) and a different extractor path. Before landing, diff the Chunk 10 branch's file list against section 4 below.
- **Chunk 9 (§F10, replica-parser depth-aware scalar extraction)** operates on `parse_simple` / `parse_with_ability` / `parse_legendary` — two of which this chunk deletes and one of which becomes new code. If Chunk 9 lands first, its helper (`util::slice_before_chain_and_cast`, `depth_aware` flag on `extract_color`) **must be preserved and reused** by the new itempool-summon extractor — it solves exactly the chain-interior leakage class that summon entries will also exhibit.

## 3. What ships

### 3.1 Retirements (zero-corpus-user code)

- `compiler/src/extractor/classifier.rs`
  - Delete `ModifierType::Legendary` variant (declared near line 17).
  - Delete the gate `if starts_with_ci(modifier, "item.") { return Ok(ModifierType::Legendary); }` (line 184).
  - Route remains: `itempool.` → `ModifierType::ItemPool`, which is now the sole entry point to replica-item extraction.
- `compiler/src/extractor/replica_item_parser.rs`
  - Delete `parse_legendary` + its helper functions + all its tests.
  - Delete `parse_simple` and `parse_with_ability` in their current form — both model the "Capture inside itempool" shape assuming a single entry per modifier, which is not how itempools actually work (multiple `+`-joined entries per pool).
  - Module's new public surface is a single fn: `extract_from_itempool(body: &str, modifier_index: usize) -> (Vec<ReplicaItem>, Vec<StructuralEntry>)` described in §3.3.
- `compiler/src/ir/mod.rs`
  - Delete `ReplicaItemContainer` enum.
  - Delete any `#[cfg(test)]` helpers that build `ReplicaItemContainer::{Capture, Legendary}` values — replace with builders for the new shape.

### 3.2 New IR shape (`compiler/src/ir/mod.rs`)

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
/// Every other itempool entry (TMs, accessories, consumables, non-summon
/// replica items) stays as structural text and round-trips from source
/// bytes — the chunk does NOT try to model every itempool shape.
pub struct ReplicaItem {
    pub container_name: String,         // "Great Ball", "Silver Wing", …
    pub target_pokemon: String,         // "Ivysaur", "Lugia", …
    pub trigger: SummonTrigger,
    pub enemy_template: String,         // "Wolf" | "dragon" | "housecat" | …
    pub team_template: String,          // "housecat" | "prodigy" | …
    pub tier: Option<u8>,
    pub hp: Option<i16>,
    pub color: Option<Color>,
    pub sprite: SpriteId,
    pub sticker_stack: Vec<StickerRef>, // preserves the `.i.(Eye of Horus#Chainmail#…)` shape
    pub speech: Option<String>,
    pub doc: Option<String>,
    pub toggle_flags: ToggleFlags,
    pub item_modifiers: Option<ItemModifiers>,
    pub source: Source,
}

/// The player action that triggers the summon. Both variants share the
/// same summon → defeat → vase-add → join pipeline; they differ only in
/// the action verb.
pub enum SummonTrigger {
    /// "Ball" flavor. Equipping replaces the hero's sides with summon-sides;
    /// the summon fires when the player USES one of those sides during a
    /// fight. 17 corpus instances (Poke/Great/Master/Ultra/Fast/Friend/Heavy/
    /// Level/Lure/Moon/Dive/Luxury/Nest/Premier/Timer/Dusk/Quick/Dream Ball
    /// in sliceymon).
    SideUse { sd: DiceFaces },

    /// "Wing/Orb" flavor. Equipping grants a spell; the summon fires when
    /// the player CASTS it. 4 corpus instances (Rainbow Wing, Silver Wing,
    /// Blue Orb, Red Orb in sliceymon).
    Cast { ability: String, abilitydata_sd: DiceFaces },
}
```

Field-level notes:

- `container_name` replaces the old `Capture { name }` variant's `name` field as a plain `String`. It is never `None` — every corpus summon item carries a container name (the outermost `.n.<ItemName>.tier.<n>`).
- `target_pokemon`, `enemy_template`, `team_template` are extracted by name from the inner `hat.egg.<enemy>.n.<Pokemon>` and `vase.(add.((replica.<team>.n.<Pokemon>…)))` sub-blocks — not guessed, not looked up, not registry-gated. These are source-preserving byte fields (SPEC §3.3 permissive extract path).
- `sprite: SpriteId` uses Chunk 3b's unified sprite shape. Extraction is `SpriteId::owned(target_pokemon.to_lowercase(), img_data_bytes)` — per Chunk 3b lesson 1, do not reach for the registry.
- `item_modifiers`, `toggle_flags`, `sticker_stack` preserve the rest of the entry's structure. Exact field list shakes out during implementation — the shipped struct must carry enough state to round-trip every corpus entry byte-equal.

### 3.3 Itempool extractor extension (`compiler/src/extractor/replica_item_parser.rs`)

Replace the module contents with a single public function and its helpers:

```rust
pub fn extract_from_itempool(
    body: &str,                     // the full `itempool.((…)).n.…` modifier source
    modifier_index: usize,          // caller-provided for Finding.position wiring
) -> Result<ItempoolContent, CompilerError>;

pub struct ItempoolContent {
    pub entries: Vec<ItempoolEntry>,
    pub pool_suffix: String,        // everything after `))` (e.g. `.n.Pokeballs Part 1`)
}

pub enum ItempoolEntry {
    Summon(ReplicaItem),
    Structural(String),             // source bytes preserved verbatim
}
```

Parser rules:

1. **Entry splitting** — walk the body one paren-depth at a time; split the inner content of the outermost `itempool.((…))` on `#` and `+` at depth 0 into entries (the `+` and `#` joiners are preserved as part of the next entry's prefix, so emit can reconstruct them).
2. **Summon detection** — for each entry:
   a. Parse the outer `hat.(replica.Thief.i.(all.(…)))` wrapper; on failure, entry is Structural.
   b. Within `all.(…)`, scan depth-0 sub-blocks for a `hat.egg.<enemy_template>.n.<Pokemon>` match. If absent → Structural.
   c. Within the same wrapper, scan for a `vase.(add.((replica.<team_template>.n.<Pokemon>…)))` where `<Pokemon>` matches the egg's. If absent, or if names diverge → Structural (half-summons do not qualify).
   d. Classify trigger: if the wrapper contains `cast.<ability>.abilitydata.(…)` at depth 0 relative to the wrapper body → `SummonTrigger::Cast { ability, abilitydata_sd }`. Otherwise → `SummonTrigger::SideUse { sd }` where `sd` is the outer `hat.replica.Thief.n.<Pokemon>.sd.<faces>` faces.
   e. Extract every other field per §3.2. Emit `ItempoolEntry::Summon(ReplicaItem { … })`.
3. **Structural preservation** — any entry not matched as a summon keeps its full source bytes. `build.rs` emits it verbatim; extract/build/extract round-trip is byte-equal.
4. **Paren/depth correctness** — use Chunk 9's `util::slice_before_chain_and_cast` + `depth_aware = true` flag on scalar extractors. Scan windows are chain-and-cast-aware; chain-interior `.hp.` / `.col.` / `.sd.` / `.img.` substrings do not leak into top-level `ReplicaItem` fields (SPEC §3.3 leak class already blessed by §F10).

### 3.4 Builder / emitter (`compiler/src/builder/replica_item_emitter.rs`)

- New fn: `emit_itempool(content: &ItempoolContent) -> String` re-assembles `itempool.((entry1<sep1>entry2<sep2>…)).n.<pool_name>.tier.<n>.img.<…>`.
- For each `ItempoolEntry::Structural(bytes)` — emit bytes verbatim.
- For each `ItempoolEntry::Summon(replica_item)` — emit in the detected trigger shape:
  - `SideUse { sd }` → `hat.replica.Thief.n.<Pokemon>.sd.<faces>` + shared payload.
  - `Cast { ability, abilitydata_sd }` → `hat.(replica.Thief.i.(all.(cast.<ability>.abilitydata.(thief.sd.<faces>…))))` + shared payload.
- Shared payload emitter handles `hat.egg.<enemy>…`, `vase.(add.((replica.<team>.n.<pokemon>…)))`, sticker stack, speech, doc, img. Single helper, not duplicated per trigger.

### 3.5 Xref (`compiler/src/xref.rs`) — 3-bucket shape

SPEC §6.3's canonical Pokemon-uniqueness bucket set is `{hero, replica_item, monster}`. Both X003 and V020 must reference it by name; neither re-lists or subdivides its members.

- **X003** (`check_duplicate_pokemon_buckets`): three buckets `{hero, replica_item, monster}`. No `match &item.container` routing — `ReplicaItemContainer` is retired. Every `ReplicaItem` (SideUse or Cast) contributes to the single `replica_item` bucket.
- **V020** (`check_cross_category_names`): narrowing predicate is `pokemon_only = distinct_buckets is subset of {hero, replica_item, monster} with cardinality ≥ 2`. V020 scope: boss-involving collisions + intra-bucket duplicates. Any collision whose bucket set is entirely inside the Pokemon-uniqueness set is silenced here (X003 owns it); any collision that includes a boss bucket still fires V020.
- **Tests to keep / write**:
  - `v020_silent_on_cross_bucket_pokemon_{hero_replica, hero_monster, replica_monster, case_insensitive}` — one Replica is enough per test; the trigger variant is irrelevant and should be tested as such (see next bullet).
  - `v020_still_fires_on_boss_{hero, replica, monster}_collision`.
  - `v020_still_fires_on_intra_bucket_duplicate_{heroes, replicas, monsters, bosses}`.
  - `v020_and_x003_coexist_when_collision_spans_boss_and_pokemon_buckets` — asserts the co-fire invariant, not just rule-id equality; the predicate states "V020's message must name a boss bucket, X003's must not" and fails loudly if future code drops the boss mention.
  - `no_double_fire_on_working_mods` — boss-in-message predicate unchanged.
- **New source-vs-IR tests**:
  - `xref::x003_one_replica_bucket_across_triggers` — IR with hero "Pikachu" + SideUse replica "Pikachu" + Cast replica "Pikachu" fires X003 exactly **once** with `buckets = [hero, replica_item]` (not `[hero, sideuse, cast]`). Proof that trigger granularity did not leak into Pokemon uniqueness.
  - `xref::x003_treats_sideuse_and_cast_replicas_as_one_bucket` — IR with SideUse replica "Pikachu" + Cast replica "Pikachu" (no hero) fires X003 once with bucket set `{replica_item}` (intra-bucket duplicate), and V020 stays silent. Separate test from the cross-bucket case so bucket-merging does not ride on the hero-collision path.

### 3.6 SPEC amendments (`SPEC.md`)

- **§6.3 line 246** — rewrite the parenthetical. Options:
  - Drop it: `A Pokemon may exist in at most one of: heroes, replica items, monsters.`
  - Rename it: `A Pokemon may exist in at most one of: heroes, replica items (summon items), monsters.`
  - Preferred: drop — the bucket's internal structure is IR-level detail, not a §6.3 concern.
- **Line 342** — full rewrite:
  > **ReplicaItem** — IR type for items that summon a Pokemon as a monster; on defeat, the Pokemon joins the team for the rest of the run. Every `ReplicaItem` comes from an entry inside an `itempool.((…))` modifier and shares the same summon → defeat → team-join pipeline. Two trigger kinds, discriminated by which player action fires the summon: `SideUse` (equipping replaces the hero's sides with summon-sides; the summon fires when the player uses one of those sides during a fight — e.g. Poke Ball) and `Cast` (equipping grants a spell; the summon fires when the player casts it — e.g. Silver Wing). Both kinds carry identical summon/defeat/team-join payload; the discriminator sits on `SummonTrigger`, not on a container-position enum.
- **Derived/downstream mentions** — grep `SPEC.md` for "Capture" / "Legendary" referring to replica kinds and update; keep the words where they refer to game-mechanic flavor (copy-paste UX text), not IR variants.

### 3.7 Plan-layer updates (`plans/PLATFORM_FOUNDATIONS_PLAN.md`)

- **Chunk 6 retraction note** — append to Chunk 6's block: the `ReplicaItemContainer` enum was retired in Chunk 8 after corpus audit (zero instances of either variant via the live IR route); the container-name role moved to a plain `String` field on `ReplicaItem`, and the kind discriminator moved to `SummonTrigger` on the evidenced trigger axis.
- **Chunk 8 entry** — rewrite in full to describe this plan's scope (trigger-based IR, itempool extraction, V020/X003 3-bucket collapse). The prior Chunk 8 description (V020 capture/legendary narrowing) is superseded and does not survive as a historical note — plans are roadmaps, not changelogs (per CLAUDE.md).
- **§F9 success-criteria** — ensure bucket-set wording reads `{hero, replica_item, monster}`, named once, referenced by name downstream.
- **New "Lessons from prior chunks" entry (sibling to Chunk 3b)**:
  > **Chunk 8 (2026-04-22) — IR variants outran corpus evidence.** `ReplicaItemContainer::{Capture, Legendary}` and `ModifierType::Legendary` were authored in Chunk 6 + earlier without a single corpus instance for any of them: all four working mods contain zero top-level `item.*` modifiers (the shape `parse_legendary` gates on), and every Pokemon-summon item lives inside `itempool.((…))` where the extractor kept them opaque (`roundtrip_diag` reports `Replicas ir1=0` for all four). An in-flight V020 narrowing against the unevidenced distinction (PR #10) was abandoned in favor of retiring the variants. Chunk 8 re-roots the discriminator on trigger (`SideUse` / `Cast`), which every corpus summon item actually exhibits.
  >
  > **Takeaway:** before an IR variant discriminator ships, grep the corpus for an instance of each variant. Zero instances for a variant means the variant is a hypothesis, not a model — do not land it. A rule authored against an unevidenced variant (like the abandoned V020 split) compounds the defect. Prose in SPEC/plan is not evidence; `rg` output is.

The corpus-grounding PreToolUse hook rule (`.claude/settings.json`) that encodes this lesson is already in place; no hook update needed during this chunk.

## 4. Files touched

| File | Change |
|---|---|
| `compiler/src/ir/mod.rs` | Delete `ReplicaItemContainer` enum. Replace `ReplicaItem` struct per §3.2. Add `SummonTrigger` enum. Update `#[cfg(test)]` helpers. |
| `compiler/src/extractor/classifier.rs` | Delete `ModifierType::Legendary` variant + gate. `itempool.` routing unchanged. |
| `compiler/src/extractor/replica_item_parser.rs` | Full rewrite per §3.3. New `extract_from_itempool` entry point; `parse_legendary` / `parse_simple` / `parse_with_ability` deleted. |
| `compiler/src/extractor/mod.rs` | Route `ItemPool` modifiers through `replica_item_parser::extract_from_itempool`, merge its `ReplicaItem` output into `ModIR.replica_items`, keep `Structural` entries in the existing structural channel. |
| `compiler/src/builder/replica_item_emitter.rs` | Full rewrite per §3.4. `emit_itempool` reassembles entry order + joiners; shared payload helper. |
| `compiler/src/builder/mod.rs` | Builder dispatch updated for the new emitter shape. |
| `compiler/src/xref.rs` | Per §3.5. X003 + V020 predicates on the 3-bucket set; test suite per §3.5 test list. |
| `compiler/src/ir/ops.rs` | `add_replica_item` / `remove_replica_item` re-tested against new struct. Container-kind CRUD assertions deleted. |
| `compiler/tests/build_options_tests.rs` | `v020_cross_category_source_is_global` stays on hero+boss; any construction-site assertions referencing `ReplicaItemContainer` move to trigger-based shape. |
| `compiler/tests/roundtrip_baseline.rs` | Baselines for all four working mods re-generated. Sliceymon's baseline gains non-zero `Replicas` count (21 summon items across Parts 1+2+Summons). |
| `compiler/tests/correctness_tests.rs` | If any test asserts `parse_legendary` or `ReplicaItemContainer::Legendary` reachability, delete. |
| `compiler/examples/roundtrip_diag.rs` | No code change expected, but the reported `Replicas ir1=XX` line will now be non-zero for sliceymon. Optionally add a per-entry trigger breakdown (SideUse vs Cast counts). |
| `compiler/examples/drift_audit.rs` | Verify drift-audit still passes; add Pokemon-summon drift class if relevant. |
| `SPEC.md` | Per §3.6. |
| `plans/PLATFORM_FOUNDATIONS_PLAN.md` | Per §3.7. |

## 5. Verification — shipped tests

Numbered for reference; each must land with the chunk.

**Source-vs-IR roundtrip**
- [ ] T1. `extractor::itempool_summon_entry_sideuse_roundtrips_ivysaur` — feed the Ivysaur entry slice from sliceymon line 111 through extract → build → assert byte-equal input vs output.
- [ ] T2. `extractor::itempool_summon_entry_cast_roundtrips_lugia` — same for Silver Wing entry from line 115.
- [ ] T3. `extractor::itempool_full_pool_roundtrips_pokeballs_part_1` — full line-111 modifier (17 entries, mixed in source order). Byte-equal.
- [ ] T4. `extractor::itempool_full_pool_roundtrips_pokeballs_part_2`.
- [ ] T5. `extractor::itempool_full_pool_roundtrips_summons_part_1` (4 wings/orbs + any non-summon entries).
- [ ] T6. `extractor::non_summon_entry_stays_structural` — an itempool entry that is a TM / accessory (no `hat.egg` + `vase.(add.((replica.…)))` pair) does NOT become a `ReplicaItem`; its source bytes round-trip unchanged via `ItempoolEntry::Structural`.
- [ ] T7. `extractor::half_summon_entry_stays_structural` — an entry with `hat.egg` but no matching `vase.(add.((replica.<same-pokemon>…)))` (or with mismatched Pokemon names) stays Structural. Proves the detector is conjunctive.

**Trigger classification (corpus-complete)**
- [ ] T8. `extractor::all_ball_entries_classify_as_sideuse` — iterate every entry in sliceymon Pokeballs Part 1 + Part 2 with container name ending in `Ball` (17 total). Each → `SummonTrigger::SideUse`.
- [ ] T9. `extractor::all_wing_orb_entries_classify_as_cast` — iterate Summons Part 1 (Rainbow Wing, Silver Wing, Blue Orb, Red Orb — 4 total). Each → `SummonTrigger::Cast`.
- [ ] T10. `extractor::trigger_classification_reads_inner_payload_not_name` — synthetic IR: itempool entry whose container name is `Cast-Iron Ball` but whose inner trigger shape is SideUse. Classification yields `SummonTrigger::SideUse`. Source-vs-IR proof that classification reads payload bytes, not surface strings.
- [ ] T11. `extractor::trigger_classification_is_total` — every `ReplicaItem` produced from the four working mods has a non-`None` / non-`Unknown` trigger (no fallback variant exists).

**Retirements verified**
- [ ] T12. `grep_crate_for_replica_item_container_enum` — `rg "ReplicaItemContainer"` across `compiler/src/` and `compiler/tests/` yields zero hits.
- [ ] T13. `grep_crate_for_parse_legendary` — `rg "parse_legendary"` yields zero hits.
- [ ] T14. `grep_crate_for_modifier_type_legendary` — `rg "ModifierType::Legendary"` yields zero hits.

(Tests T12–T14 can be `compiler/tests/retirements.rs` integration tests using `std::fs` + a small static search, or tree-grep helpers in `build.rs`.)

**Xref 3-bucket shape**
- [ ] T15. `xref::x003_one_replica_bucket_across_triggers` — per §3.5.
- [ ] T16. `xref::x003_treats_sideuse_and_cast_replicas_as_one_bucket` — per §3.5.
- [ ] T17. `xref::v020_silent_on_cross_bucket_pokemon_hero_replica` — SideUse replica.
- [ ] T18. Same for `hero_monster` / `replica_monster` / `case_insensitive` pairs.
- [ ] T19. `xref::v020_still_fires_on_boss_replica_collision` — one test (not split by trigger) covers the boss-vs-replica case.
- [ ] T20. `xref::v020_and_x003_coexist_when_collision_spans_boss_and_pokemon_buckets` — asserts the invariant (V020 message names a boss bucket; X003 does not), not rule-id pair equality.
- [ ] T21. `xref::no_double_fire_on_working_mods` — re-run with the new Replica counts (sliceymon ~21 replicas) populated. Boss-in-message predicate still the sole permitted co-fire.

**Baseline regeneration**
- [ ] T22. `roundtrip_baseline` tests pass for all four mods with regenerated baselines. `Replicas` count is non-zero for sliceymon (21 across Parts 1+2+Summons) and zero for the other three.
- [ ] T23. `cargo run --example roundtrip_diag` reports `Status: ROUNDTRIP OK` for all four mods.

**Authoring-path sanity**
- [ ] T24. `authoring::replica_item_builder_sideuse` — typed constructor for a SideUse replica accepts `container_name`, `target_pokemon`, `enemy_template`, `team_template`, `sd`, etc. Compile error if `SummonTrigger` variant is omitted. Mirrors §6.1 strict/typed authoring path.
- [ ] T25. `authoring::replica_item_builder_cast` — typed constructor for Cast replica requires `ability` and `abilitydata_sd`. Compile error without either.
- [ ] T26. `authoring::replica_item_emits_inside_itempool` — calling the builder then emitting via `emit_itempool` produces a syntactically valid itempool entry (byte-equal to a hand-written reference).

## 6. Structural check (per hook rules)

- **Collapses two paths with different invariants?** Yes — deliberately. `Capture` and `Legendary` were a parallel representation of one mechanic with different action verbs. SPEC §6.3 already holds a single-bucket invariant; this chunk brings the implementation under it. Not a spec amendment, a spec *alignment*.
- **N-line incantation duplicated across callsites?** No. Itempool extraction and emission each consolidate into one helper; shared summon-payload emission sits in one private fn used by both trigger arms.
- **Canonical set restatement?** `{hero, replica_item, monster}` is named once in SPEC §6.3 and referenced by name from X003 and V020. Per the hook rule, members are not re-listed inside the chunk.
- **Evidence for every variant?** Required ≥ 1 corpus instance per retained variant. `SideUse`: 17 (balls). `Cast`: 4 (wings/orbs). Retired variants: 0 instances, gone.
- **Source-vs-IR test present?** T10 (trigger classification reads inner payload not name) and T7 (half-summon stays structural) both exercise cases where an IR-equality-only test would pass on silently wrong extraction.
- **Integration assertions encode reason?** T20 (`v020_and_x003_coexist_when_collision_spans_boss_and_pokemon_buckets`) asserts the permission invariant (V020's message names a boss bucket), not just the rule-id pair. T21 (`no_double_fire_on_working_mods`) uses the same boss-in-message predicate.

## 7. Risks

- **Itempool extraction is new parser territory.** The extractor currently keeps pools opaque; teaching it to walk `+`-and-`#`-joined entries and sub-detect summon shape requires careful paren-depth tracking. Failure mode: roundtrip drift on non-summon entries inside the same pool (e.g. the `Part 1` pool contains 17 summon entries AND a trailing `.n.Pokeballs Part 1` suffix, plus any intermediate non-summon entries). Mitigation: preserve source bytes for any entry that doesn't cleanly match the summon detector; test with the full Parts 1+2 content (T3–T5) before shipping.
- **community.txt audit.** community.txt has 756 warnings and no `ReplicaItem` today. An audit (`rg 'hat\.egg\.' working-mods/community.txt` + `rg 'vase\.\(add\.\(\(replica\.' working-mods/community.txt`) must run before landing to ensure community has no unknown summon shapes. If it does, add coverage.
- **pansaer / punpuns.** Neither has replica items today. Post-chunk: expected `Replicas ir1=0` unchanged. Regression if that changes silently.
- **Chunk 9 landing order.** If Chunk 9 (§F10) merges first, reuse its helpers. If this chunk merges first, the new summon extractor must be depth-and-chain-aware from day one — don't re-litigate the chain-interior leak class.
- **Chunk 10 file overlap.** Chunk 10 (BooleanPhase routing) authored in parallel session; expected no overlap with this chunk's files, but diff before landing.

## 8. Decisions deferred to the user (document before landing)

- D1. SPEC §6.3 parenthetical: drop the `(captures / legendaries)` gloss entirely, or rename to `(summon items)`?
- D2. `ReplicaItem` struct field list (exact): the shipped struct must carry enough state to round-trip every corpus entry byte-equal. Concrete field list shakes out during implementation; if any corpus entry carries a sub-block the struct doesn't model (e.g. arbitrary `i.<custom-sub>`), preserve it via an `extras: Vec<RawSubBlock>` side-channel — do **not** silently drop bytes.
