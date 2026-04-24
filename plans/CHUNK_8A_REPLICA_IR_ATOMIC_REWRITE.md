# Chunk 8a — Atomic IR rewrite for trigger-based ReplicaItem

> **(A4 fix) File-count exception — declared upfront.** This chunk touches
> approximately 15 files across `compiler/src/{ir,extractor,builder,xref,ops}`
> plus tests. The `personas/ai-development.md` ≤5-file-per-chunk rule is WAIVED
> because the IR field rewrite (`name → target_pokemon`, `template → trigger`,
> field retirements, `StructuralContent::ItemPool { body, items }` →
> `{ items: Vec<ItempoolItem> }`) breaks every callsite simultaneously — no
> partial split produces a compiling intermediate state. The 'no parallel
> types' principle (`CLAUDE.md`) takes precedence over the file-count soft
> limit; every shipped file-touch is justified by the atomic-rewrite
> constraint. The declaration lives at the top so reviewers cannot mistake
> the exception for unacknowledged scope creep.
>
> **Parent plan**: `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` (§9.1). This file is the **one-shot unit** for sub-chunk 8a — it contains every fact an implementer needs without re-reading the parent plan, and its scope stops at the atomic IR rewrite. 8b (real extractor + xref widening) and 8c (SPEC prose + plan prose + optional roundtrip_diag helper) are out of scope here and ship in separate plan files.
>
> **Authority rule**: every symbol reference in this plan is **anchored by verbatim quote** from the source file as of the authoring Read, not by line number — line numbers drift, quotes do not. Every grep check is keyed on the quoted string.
>
> **Size exception (declared up front)**: one atomic commit that covers every callsite made incompatible by the IR schema change. AI-dev persona's ≤5-file rule is honored at the spirit level — deleting `ItemPoolEntry`, `ReplicaItem.{name,template,sticker,sd,abilitydata}`, and `ModifierType::Legendary` and replacing `StructuralContent::ItemPool { body, items }` with `{ items: Vec<ItempoolItem> }` breaks every callsite simultaneously. `cargo build` can only be clean again after **every** callsite is updated in the same commit (no parallel representations per `CLAUDE.md`). Any smaller split produces non-compiling intermediate states — a violation of the principle this plan enforces. Test files count too. 8a ships a **transitional** `ItempoolItem::NonSummon { name: String, tier: Option<i8>, content: String }` variant (raw-passthrough `content`); closing the SPEC §3.2 raw-passthrough violation is the scope of sibling chunk **8A.5** (`plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`), which ships BEFORE 8b.
>
> **Dependency chain**: 8a → 8A.5 → 8b. 8a lands the trigger-IR + atomic callsite rewrite with a transitional raw-passthrough NonSummon; 8A.5 lands the typed `NonSummonEntry` sum that closes the SPEC §3.2 violation; 8b lands the real per-entry classifier + xref widening. 8b cannot start until 8A.5 has landed.
>
> **Out of scope (enumerated, not "future work")**: the typed `NonSummonEntry` sum that replaces `content: String` (**8A.5**), per-variant NonSummon round-trip guards (T30a–T30e — ship with the typed schema in **8A.5**), the real per-entry itempool classifier (8b), xref `Finding` widening + X003 bucket-routing rewrite (8b), xref bucket-label unification (8b, ruled separately under §9), baseline regeneration for non-zero `Replicas` counts (8b), `SPEC.md` rewrites (8c), foundations-plan rewrites (8c), `.claude/settings.json` hook bullet (8c), new round-trip tests that require the real parser (8b: T1, T2, T2a, T2b, T3–T7, T8, T9, T9a, T9b, T9c, T10, T10a, T11, T15–T23a, T27, T28). 8a ships **only** retirement-greps (T12/T12a/T13/T14) + stub source-preservation (T12b) + authoring-builder compile-guards (T24/T25/T25a/T26) + source-vs-IR divergence guard (T26a) + new-enum compile-guards (T29a/T29b) + a single transitional NonSummon round-trip guard (T30 — byte-equal via raw `content`, strengthened by 8A.5). The stub extractor makes zero `ReplicaItem` entries, so `roundtrip_diag` reports `Replicas ir1=0 ir2=0` — the 8a gate accepts that; 8b fixes it to 23.

## 1. Scope

Replace the pre-Chunk-8 IR shape for Pokemon-summon items (`ReplicaItemContainer::{Capture, Legendary}` + `ReplicaItem.{name, template, sticker, sd}`) with the new trigger-based shape (`SummonTrigger::{SideUse{dice, dice_location}, Cast{dice}}` + renamed/typed fields) in a **single compiling commit**. Retire every dead gate, variant, parser function, and helper that models zero corpus instances (per `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` §1.3). Ship a byte-preserving stub extractor that keeps the itempool opaque so all four `working-mods/*.txt` still roundtrip byte-equal.

**Plan chain (8a → 8A.5 → 8b)**: non-summon itempool entries are the DOMINANT shape across the corpus — 3 of 4 working-mods (pansaer, punpuns, community) have zero summons; only sliceymon carries 23 summon envelopes across its 16 pools. 8a's trigger-IR is a sliceymon-specific specialization layered on top of the base non-summon content. Closing the SPEC §3.2 raw-passthrough violation for NonSummon entries — the dominant-shape work — ships in **8A.5** (`plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`), which retypes `ItempoolItem::NonSummon { content: String }` into a typed `NonSummonEntry` sum. 8a's own scope remains the atomic retirement + trigger IR (`SummonTrigger` / `DiceLocation` / `ItempoolItem` enum) + transitional NonSummon passthrough; 8b ships the real per-entry classifier + xref widening after 8A.5 has landed.

Result at the end of 8a: the IR, the stub extractor, the emitter, the authoring builder, and every internal callsite all speak the new shape. The real per-entry extractor arrives in 8b (after 8A.5 retypes NonSummon). The stub is not a half-finished implementation — it is the deliberate byte-equal pass-through shipped in the same spirit as the existing pre-Chunk-8 behavior (`StructuralContent::ItemPool { body, items: Vec<ItemPoolEntry> }` already stored the pool body opaquely; 8a moves that opacity into a single `ItempoolItem::NonSummon { name: "", tier: None, content: <entire body> }` entry without multiplying representation).

## 2. Pre-conditions

Parent plan `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` §2 establishes the hard gate: **"Chunks 5, 7, and 9 must land on `main` before Chunk 8 starts."** 8a inherits that gate verbatim — 8a is not narrower than the parent on pre-conditions, so halting on any one of the three is required.

```bash
# Chunk 9 helper — real parser prerequisite (8a stub does not call it, but 8b's
# landed-on-main gate requires it per parent §2). Post-merge baseline this
# session = 7 hits; any 0 means Chunk 9 was reverted.
rg -c 'slice_before_chain_and_cast' compiler/src/util.rs          # expect ≥1

# Chunk 7 (panic/unwrap elimination). 8a's rewrites of replica_item_parser.rs
# and replica_item_emitter.rs must inherit Chunk 7's error patterns (new
# CompilerError constructors). Post-merge baseline this session = 116
# `.unwrap()` / `.expect(` hits across `extractor/` + `builder/`. Record the
# actual count on the 8a PR.
rg -c '\.unwrap\(\)|\.expect\(' compiler/src/extractor/ compiler/src/builder/

# Chunk 5 (generate_hero_item_pool) — parent §2 requires Chunk 5 to land before
# Chunk 8 starts. Post-merge baseline this session = 0 hits; the `fn
# generate_hero_item_pool` function is NOT present in `builder/derived.rs`.
# §3.8's "new authoring" branch therefore activates, BUT only after the user
# explicitly relaxes the parent gate for this PR (noted on the PR body). In the
# absence of an explicit relaxation, 8a halts.
rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs
```

**Halt condition**: if ANY of the three greps disagrees with its baseline above and the user has not explicitly relaxed the parent pre-condition on the PR, stop. The parent plan treats all three as equal blockers; 8a does not have the authority to weaken that. The only narrower-than-parent behavior here is §3.8's conditional implementation shape — it describes how 8a adapts **if** the user relaxes the Chunk 5 gate, but does not itself license the relaxation.

**(A2 fix) Nested `hat.egg.(` — explicit 8A vs 8B split.** Red Orb (Groudon) carries a nested `hat.egg.(wolf.n.Geyser...` inside its outer `hat.egg.dragon.n.Groudon` body (verified 2026-04-24 via `rg -nF 'hat.egg.(wolf' working-mods/sliceymon.txt` returning line 117). 8A ships the stub unchanged — the stub does not extract any `ReplicaItem`, so the nested case is latent and invisible. The gate for 8B is documented HERE so 8B's §2 can reference it:

```bash
# 8B gate (restated here; enforced in 8B §2 via fix B4):
rg -nF 'hat.egg.(wolf' working-mods/sliceymon.txt | wc -l                    # expect ≥1 (Red Orb latent case)
# If 8B starts with this >0 AND ReplicaItem has no nested-egg field, 8B must
# widen the schema (new commit) before producing any `Summon(i)` for the Red
# Orb entry. Emitting a partial ReplicaItem with silently-lost nested bytes
# is raw-passthrough by omission — SPEC §3.2 violation.
```

8A's scope: ship the stub; the doc-comment on `ReplicaItem` in §3.2 already flags the widening obligation. 8B's scope: either widen the schema in the same commit as the real extractor, or demote Red Orb to `Unclassified` + Finding (8A.5's hatch) until the schema lands.

**Anchor discipline**: every file/symbol reference below is a **verbatim quoted substring** from the source as it stood when this plan was authored. Line numbers drift across merges; quotes do not. Every pre-condition and every edit uses grep-for-quoted-string, not grep-for-line-number. The specific quotes used as anchors (and their Read evidence gathered for this plan authoring) are enumerated inline at each §3.N sub-section.

**(A1 fix) Corpus grep evidence for the new enum variants (inline — both must pass at implementation start):**

```bash
# SideUse{OuterPreface} — dice live BEFORE the wrapper, in a flat preface `hat.replica.Thief.n.<Pokemon>`.
# Every SideUse{OuterPreface} entry has one of these. Verified 2026-04-24.
rg -oF 'hat.replica.Thief.n.' working-mods/sliceymon.txt | wc -l            # expect 18

# SideUse{InnerWrapper} — no outer preface; dice live inside the wrapper's inner `i.(hat.Thief.sd.<faces>)`.
# Verified 2026-04-24: exactly 1 hit, corresponding to Master Ball? (Arceus).
rg -o 'i\.\(hat\.Thief\.sd\.' working-mods/sliceymon.txt | wc -l            # expect 1

# Cast trigger — outer `cast.sthief.abilitydata` marker, 4 ITEMPOOL-SCOPED entries.
# Verified 2026-04-24: 4 hits global in sliceymon (all itempool); 3 in punpuns (all boss-block; excluded by §3.3 rule 1); 1 in community (non-itempool; excluded); 0 in pansaer.
rg -oF 'cast.sthief.abilitydata' working-mods/sliceymon.txt | wc -l          # expect 4
```

Invariant: 18 + 1 + 4 = 23 (parent §1.1 total summon envelopes). If any of these counts diverges at impl start, the enum variant set must be widened or narrowed before the atomic rewrite ships.

## 3. What ships

### 3.1 Retirements (zero-corpus-user code, deleted in 8a)

**Baseline (Read this session)**: as of authoring, `grep -rn "ReplicaItemContainer" compiler/` returns a single hit — a comment at `xref.rs`: `// The former `capture` bucket was removed along with `ReplicaItemContainer``. The `ReplicaItemContainer` enum, the `ReplicaItem.container` field, the `parse_simple` / `parse_with_ability` fns, and the `ModifierType::ReplicaItem{,WithAbility}` variants are **already retired upstream** (Chunk 9 or earlier). The retirements remaining in 8a's atomic commit are catalogued below, each anchored by verbatim quote.

All symbols retired in 8a have **zero** corpus instances across the four working mods. Any non-zero count halts the chunk.

```bash
# Zero top-level `item.` modifiers (the real anti-gate is top-level, not raw
# substring — raw substring matches `.item.` inside nested bodies like
# `ritemx.` / `alliteme`).
rg -o '^item\.|[,!+]item\.[a-z]' working-mods/*.txt | head               # expect empty
# No Rust fn names leaked into mods.
rg -c 'parse_legendary|parse_simple|parse_with_ability' working-mods/*.txt  # expect 0
```

**Scope note on `cast.sthief.abilitydata`** (corpus audit — Cast trigger variant = 4 entries, itempool-scoped only):
- `rg -o 'cast\.sthief\.abilitydata' working-mods/sliceymon.txt | wc -l` → **4** (all 4 on sliceymon lines 115/117, both Summons itempools).
- `rg -o 'cast\.sthief\.abilitydata' working-mods/punpuns.txt | wc -l` → **3** (all three inside boss BooleanPhase ability bodies at lines 82/84/112 — NOT itempool members).
- `rg -o 'cast\.sthief\.abilitydata' working-mods/community.txt | wc -l` → **1** (Mental Defense standalone item at line 223 — NOT an itempool member).
- `rg -o 'cast\.sthief\.abilitydata' working-mods/pansaer.txt | wc -l` → **0**.
Global total = **8**; in-scope (itempool-summon) total = **4**. An 8a implementer who sees 8 hits and concludes the plan is wrong has misread scope. The `cast.sthief` bytes outside itempools classify via existing non-itempool routes (Hero for punpuns' boss abilities; Structural / generic non-summon item for Mental Defense) and are NOT the business of `extract_from_itempool`.

**In `compiler/src/extractor/classifier.rs`** — anchor by quoted string `if starts_with_ci(modifier, "item.")`:

Read of `classifier.rs` this session confirms the exact surviving gate is:

```
    if starts_with_ci(modifier, "item.") {
        return Ok(ModifierType::Legendary);
    }
```

Delete this gate (the only `ModifierType::Legendary` producer) and delete the `ModifierType::Legendary` variant from the enum declaration `pub enum ModifierType`. Replace with a typed error that rejects top-level `item.` input (no working mod uses this shape — zero-instance corpus):

```rust
// Replaces the `if starts_with_ci(modifier, "item.") { return
// Ok(ModifierType::Legendary); }` gate. The four working mods contain zero
// top-level `item.` modifiers; any future mod that uses this shape is a new
// corpus that needs a design decision, not a silent fallthrough.
if starts_with_ci(modifier, "item.") {
    let preview: String = modifier.chars().take(120).collect();
    return Err(CompilerError::classify(
        modifier_index,
        preview,
        "Top-level `item.<…>` modifiers are not currently modeled. \
         Summon items belong inside `itempool.((…))` envelopes. \
         See plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md §1.2.",
    ));
}
```

`CompilerError::classify` signature (Read this session from `compiler/src/error.rs`): `pub fn classify(modifier_index: usize, preview: impl Into<String>, message: impl Into<String>) -> Self` (three positional args). The `preview` idiom (`modifier.chars().take(120).collect()`) matches the pattern already in this file — anchor on `let preview: String = modifier.chars().take(120).collect();` if you need to see it inline.

**In `compiler/src/extractor/replica_item_parser.rs`** — anchor by quoted string `pub fn parse_legendary`:

Read this session confirms the surviving fn is `pub fn parse_legendary(modifier: &str, modifier_index: usize) -> Result<ReplicaItem, CompilerError>` with test helpers named `parses_legendary_from_top_level_item`, `legendary_name_is_last_depth0_n_before_ability`, `legendary_hp_ignores_chain_interior_sidesc`, `legendary_color_ignores_chain_interior_sidesc`, `legendary_sd_ignores_chain_interior_sidesc`, `legendary_img_ignores_chain_interior_sidesc`, `legendary_ignores_abilitydata_interior_hp_color_sd_img`, `legendary_without_item_prefix_propagates_error`. Delete the fn AND every one of those `#[test]` blocks — all 8 tests assert properties of the retired `parse_legendary`; they have no replacement in 8a because no parser is called in the 8a stub. New module content: one public fn `extract_from_itempool` (stub signature in §3.4) and one supporting type `ItempoolExtraction`. No private helpers in 8a (the real parser and its helpers ship in 8b).

**In `compiler/src/ir/mod.rs`** — anchor by quoted strings `pub struct ReplicaItem`, `pub struct ItemPoolEntry`, `pub enum StructuralContent`:

Read this session confirms the surviving shape:

```
pub struct ReplicaItem {
    /// Inner character name (used for .mn. suffix)
    pub name: String,
    pub template: String,
    pub hp: Option<u16>,
    pub sd: DiceFaces,
    pub sprite: SpriteId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abilitydata: Option<AbilityData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_modifiers: Option<ModifierChain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggle_flags: Option<String>,
    #[serde(default, skip_serializing_if = "Source::is_base")]
    pub source: Source,
}
```

Rewrite the struct per §3.2: retire the fields `template`, `sd`, `sticker`, `abilitydata`; rename `name` → `target_pokemon` (see §3.3 field note); add `container_name`, `trigger: SummonTrigger`, `enemy_template`, `team_template`, `sticker_stack: Option<ModifierChain>`. All six field mutations land in the same commit.

Read this session also confirms `pub struct ItemPoolEntry { pub name: String, pub tier: Option<i8>, pub content: String }` — the legacy non-typed entry wrapper. Delete this struct. Replace the `StructuralContent::ItemPool { body: String, items: Vec<ItemPoolEntry> }` variant (anchor by verbatim quote `ItemPool {`) with `StructuralContent::ItemPool { items: Vec<ItempoolItem> }` — no `body: String` field; pure-IR rebuild per §3.5. Every destructure site (anchor by quoted string `StructuralContent::ItemPool {` across `compiler/src/`) migrates atomically. Read this session confirms the destructure sites today:

- `compiler/src/builder/structural_emitter.rs`: `StructuralContent::ItemPool { body, .. } => body.clone(),`
- `compiler/src/extractor/structural_parser.rs`: the `parse_itempool` body returns `StructuralContent::ItemPool { body: raw.to_string(), items }` (two construction sites inside the fn — one at the short-circuit, one at the tail).

The signature of the emit dispatch for `StructuralContent::ItemPool` MUST evolve to accept `&ir.replica_items` and the pool name — see §3.5.1.

No tests in `ir/mod.rs` reference `ReplicaItemContainer` today (Read this session: `grep "ReplicaItemContainer" ir/mod.rs` → 0 hits). No removal needed for that enum; the new-enum compile-guards for `SummonTrigger`/`DiceLocation`/`ItempoolItem` ship in their place per §4.

**Grep-only verifications at end of 8a** (word-anchored — bare `parse_simple` substring-matches `parse_simple_choice` in `compiler/src/extractor/phase_parser.rs`, a legitimate non-retired fn; use `\b...\b`):
```bash
rg -c 'ReplicaItemContainer\b' compiler/                                        # expect 0 (src + tests + examples)
rg -c 'ItemPoolEntry\b' compiler/                                               # expect 0
rg -c 'ModifierType::Legendary\b' compiler/src/                                 # expect 0
rg -c '\b(parse_legendary|parse_simple|parse_with_ability)\b' compiler/         # expect 0
rg -c '\bitem\.(template|name|sticker|sd)\b' compiler/src/                      # expect 0 (every callsite migrated)
```

The greps above are the body of retirement tests T12, T12a, T13, T14 — see §4 below. Both the bash greps and the Rust `recursive_grep` helper in §4 must apply the same word-boundary discipline. Note that T12 / T12a guard against regressions of upstream-retired symbols (`ReplicaItemContainer`, `ModifierType::ReplicaItem{,WithAbility}`); they are not load-bearing against 8a's own deletions. The 8a-specific retirement checks are T13 (`parse_legendary` family), T14 (`ModifierType::Legendary`), and the bare-field rename check `\bitem\.(template|name|sticker|sd)\b`. Those three are the signals that 8a's edits landed.

### 3.2 New IR shape (`compiler/src/ir/mod.rs`)

The body below is the canonical `ReplicaItem` spec for 8a. Copy it verbatim as doc-comment + struct; do **not** deviate or add fields. If implementation surfaces a corpus sub-block not modeled, widen the struct **in the same commit** — no `extras: Vec<RawSubBlock>`, no `raw_remainder: String`, no escape hatch (per `CLAUDE.md` "no deferred correctness" + `SPEC.md` §3.3).

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
/// pattern is raw passthrough with extra steps.
///
/// **Known 8b widening obligation — nested `hat.egg.`.** The Red Orb (Groudon)
/// entry on sliceymon line 117 contains a NESTED `hat.egg.(wolf.n.Geyser.sd.…`
/// inside the outer `hat.egg.dragon.n.Groudon` body (at wrapper depth 4+).
/// The single `enemy_template: String` field below captures only the outer
/// `dragon`; the nested Geyser sub-egg is NOT covered by the current field
/// list. 8a's stub does not classify this as a ReplicaItem (everything is
/// `NonSummon { content }`), so the violation is latent. 8b MUST widen the
/// struct — e.g. an `Option<NestedEgg>` field or a typed `enemy: EnemyEgg`
/// newtype that carries nested sub-eggs — before producing any `Summon(i)`
/// entry whose body contains a nested `hat.egg.`. The alternative (emitting
/// a partial ReplicaItem plus silently-lost nested bytes) is raw-passthrough
/// by omission and violates the no-escape-hatch rule above.
///
/// **8a stub note**: the stub `extract_from_itempool` (§3.4) never produces
/// a `ReplicaItem` — every entry is demoted to `ItempoolItem::NonSummon`.
/// The struct's field population is exercised by the authoring-builder
/// tests (T24–T26) and by compile-guards; real extraction arrives in 8b.
pub struct ReplicaItem {
    pub container_name: String,         // "Great Ball", "Silver Wing", "Master Ball?", "Master Ball" (no `?`), ...; never None; preserve trailing `?` and trailing whitespace from source bytes
    pub target_pokemon: String,         // "Ivysaur", "Lugia", "Ho Oh" (with space), "Arceus", ...; preserved as source bytes, no case normalization
    pub trigger: SummonTrigger,
    pub enemy_template: String,         // source-byte preserved: "Wolf", "wolf", "Dragon" (capital, Master Ball?), "dragon" (lowercase, all 4 Cast summons on lines 115/117)
    pub team_template: String,          // source-byte preserved per corpus distribution: "housecat"×18 (SideUse/OuterPreface), "Housecat"×1 (SideUse/InnerWrapper), "prodigy"×3 + "Statue"×1 (Cast). Emitters route `item.team_template` — no literals.
    pub tier: Option<u8>,
    pub hp: Option<u16>,                // matches existing ReplicaItem.hp type
    pub color: Option<char>,            // matches existing ReplicaItem.color type; `Color` is not a repo type
    pub sprite: SpriteId,
    pub sticker_stack: Option<ModifierChain>, // RENAME + TYPE CHANGE from `sticker: Option<String>` — see §3.3 field note for the sticker-chain round-trip requirement
    pub speech: Option<String>,
    pub doc: Option<String>,
    pub toggle_flags: Option<String>,   // matches existing ReplicaItem.toggle_flags shape
    pub item_modifiers: Option<ModifierChain>,
    // (A3 fix) RETIRED in 8A (migration, not design): the pre-Chunk-8
    // `pub abilitydata: Option<AbilityData>` field (currently present in
    // `compiler/src/ir/mod.rs`'s `pub struct ReplicaItem` per Read this
    // session) does NOT survive 8A. Rationale: corpus walk of all 4
    // cast.sthief.abilitydata bodies (parent §1.1) shows zero depth-0
    // `.n.<spell_name>` — AbilityData (which requires a non-Optional
    // `name: String`) does not fit the cast.sthief shape. Cast-specific
    // source-byte universals live as emitter constants (§3.5). If a future
    // corpus surfaces abilitydata-shaped bytes on a ReplicaItem, widen the
    // relevant SummonTrigger variant in the same PR — do NOT re-introduce
    // `Option<AbilityData>` next to the trigger (parallel-type pattern,
    // forbidden by CLAUDE.md "no deferred correctness").
    pub source: Source,
}

/// The player action that triggers the summon. Two variants — `SideUse` (use
/// a thief side) and `Cast` (cast a thief spell) — capture the only two
/// distinct game mechanics observed in the corpus. Historical "OnWrapped"
/// (Master Ball?) is NOT a third variant: the engine reads `hat.Thief.sd.<faces>`
/// identically whether the dice live on the outer preface or inside the
/// wrapper, so it is the same player-action with a different source-byte
/// encoding — captured by the `dice_location` sub-discriminator on SideUse.
///
/// Dice payload uses the same field name across both variants so every
/// consumer routes through `trigger.dice_faces()` (see `impl` block below).
/// Variant-branching for `ability` (Cast-only) and for the wrapper-shape
/// decision (which emit path — OuterPreface / InnerWrapper / Cast) is
/// legitimate and required at the emitter; variant-branching for dice access
/// is forbidden.
pub enum SummonTrigger {
    /// Player uses a thief-side during a fight. Corpus counts (parent §1.1):
    ///   `OuterPreface` = 18 (every Ball entry except `Master Ball?`),
    ///   `InnerWrapper` = 1 (`Master Ball?` only).
    SideUse {
        dice: DiceFaces,
        dice_location: DiceLocation,
    },

    /// Player casts a thief-spell. The summon fires on cast. Corpus count: 4
    /// (Rainbow Wing + Silver Wing in Summons Part 1; Blue Orb + Red Orb in
    /// Summons Part 2).
    ///
    /// Payload: `dice: DiceFaces` — the PER-ITEM inner
    /// `.i.hat.(replica.thief.sd.<faces>)` chain segment, NOT the outer
    /// `thief.sd.<UNIVERSAL>`. Outer cast-template (`"thief"`) and outer
    /// cast-dice (`"182-25:0:0:0:76-0:0"`) are EMITTER LITERALS declared in
    /// `builder/replica_item_emitter.rs` (§3.5). Widening contract: if a
    /// future corpus entry has a different outer template or depth-0 `.n.`,
    /// lift the constants into variant fields in the same PR.
    Cast { dice: DiceFaces },
}

/// Where the dice live in the source bytes for a `SideUse` summon.
/// **Source-shape sub-axis, not a player-action.** Both locations produce
/// identical engine behavior; the discriminator exists only to make
/// extract → build round-trip byte-equal.
pub enum DiceLocation {
    /// Outer flat preface: `hat.replica.Thief.n.<Pokemon>.sd.<faces>` sits
    /// BEFORE the wrapper's opening `hat.(replica.Thief.i.(all.(...`. 18
    /// corpus entries.
    OuterPreface,
    /// Inner wrapper: no outer preface; dice live inside the wrapper's egg
    /// body as `.i.(hat.Thief.sd.<faces>)` (capital `Thief`, case-preserving).
    /// 1 corpus entry (`Master Ball?` summoning Arceus).
    InnerWrapper,
}

impl SummonTrigger {
    /// Shared accessor — every consumer (emitter, xref, authoring) routes
    /// dice through this method. Variant-branching for dice access is
    /// forbidden; this is the hook rule against duplicated incantations.
    pub fn dice_faces(&self) -> &DiceFaces {
        match self {
            SummonTrigger::SideUse { dice, .. } => dice,
            SummonTrigger::Cast    { dice }     => dice,
        }
    }
}

/// Typed sum for `StructuralContent::ItemPool.items`. Every itempool entry is
/// one of:
///   - a summon (index into `ModIR.replica_items`), or
///   - a NON-summon entry (everything else in an itempool — base-game refs,
///     multipliers, ritemx refs, splices, inline definitions).
///
/// **TRANSITIONAL raw-passthrough form (8a only).** The `NonSummon` variant
/// below carries a raw `content: String` field — a KNOWN, TRACKED SPEC §3.2
/// violation that 8a ships intentionally open. Sibling chunk **8A.5**
/// (`plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`) replaces `NonSummon { name,
/// tier, content }` with a typed `NonSummonEntry` sum
/// (`BaseGameRef` / `MultiplierRef` / `RitemxRef` / `Splice` / `Inline`)
/// before 8b starts. The split exists because the typed-schema work was too
/// large to include atomically in 8a; the violation is time-boxed to the
/// 8a → 8A.5 window and 8b is blocked until 8A.5 lands.
pub enum ItempoolItem {
    /// Index into `ModIR.replica_items`. Index stability is enforced by
    /// `ir::ops::remove_replica_item` (see §3.7).
    Summon(usize),
    /// Non-summon itempool entry — transitional raw-passthrough. `name` is
    /// the entry's inline `.n.<name>` where one exists (empty `String` for
    /// the 8a stub's whole-pool passthrough); `tier` is the entry's
    /// `.tier.<n>` where one exists; `content` is the verbatim entry body
    /// bytes. 8A.5 replaces this variant with the typed `NonSummonEntry`
    /// sum.
    NonSummon { name: String, tier: Option<i8>, content: String },
}
```

The `derive` set on `ItempoolItem` must match the rest of `ir/mod.rs`. Read this session of `ir/mod.rs` confirms the pattern on peer types: `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]` (same line preceding `pub struct ReplicaItem`). Apply that exact derive line to the new enum. The `Default` derive is NOT applied (peer enums/sums in this file do not derive `Default`).

**Why this ships transitionally in 8a (user decision, not deferred correctness)**: an earlier draft of this plan folded the typed `NonSummonEntry` sum into 8a. The user ruled that the typed-schema work is too large to include atomically in 8a and must ship as sibling chunk **8A.5** (`plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`). 8a's `content: String` is therefore an intentional, time-boxed raw-passthrough — the SPEC §3.2 violation is named and tracked (see §7 and §9), and 8b is gated on 8A.5 landing so the violation cannot outlive the 8a → 8A.5 → 8b chain. This is TRACKED KNOWN scope, not deferred-correctness: the replacement is specified (8A.5) and its ship order (before 8b) is binding, not aspirational.

### 3.3 Field-level notes (the invariants that prevent drift)

- **`container_name` never `None`.** Every corpus summon item carries a `.n.<ItemName>` at the outer `vase.(add.((replica.…))).n.<name>.tier.<n>` level. Trailing `?` and trailing whitespace are part of source bytes — preserve verbatim. `Master Ball` (no `?`) and `Master Ball?` are TWO distinct entries with overlapping surface-name root in sliceymon Pokeballs Part 1.
- **`target_pokemon`, `enemy_template`, `team_template` are source-preserving.** No `.trim()`, no case normalization, no registry lookup. Case distribution observed in sliceymon (re-verify at implementation start via `rg -o 'hat\.egg\.(d|D)ragon' working-mods/sliceymon.txt` and `rg -o 'vase\.\(add\.\(\(replica\.[A-Za-z]+' working-mods/sliceymon.txt`):
  - `enemy_template`: lowercase `dragon` appears 4× on lines 115/117 — **ALL FOUR are in-scope Cast summons** (Ho Oh, Lugia, Kyogre, Groudon). Capital `Dragon` appears 1× inside the itempool — the `Master Ball?` SideUse/InnerWrapper entry (Arceus) on line 111. Capital `Wolf` dominates other SideUse/OuterPreface entries. (An earlier draft of this note claimed "lowercase dragon appears only in a boss-block hit" — that was incorrect and is corrected here against raw corpus bytes.)
  - `team_template`: `housecat` (lowercase) × 18 — all SideUse/OuterPreface entries. `Housecat` (capital) × 1 — Master Ball? InnerWrapper. `prodigy` (lowercase) × 3 — Rainbow Wing, Silver Wing, Blue Orb (Cast). `Statue` (capital) × 1 — Red Orb (Cast). Total 23 = the summon count. An emitter hardcoding `"prodigy"` for Cast would silently diverge on Red Orb; `item.team_template` must be the source of truth everywhere (emit_cast, emit_sideuse_*).
  An extractor that lowercased these would silently fail 8b's T2a byte-equality round-trip. 8a's stub does not exercise this, but the IR field doc-comments must still say so — they are load-bearing for 8b and for authoring.
- **Identity rename `name → target_pokemon` is a clarity rename, not a contract change.** The current `ReplicaItem.name: String` field (Read this session from `ir/mod.rs` — doc-comment: `/// Inner character name (used for .mn. suffix)`) carries the inner `.mn.<Pokemon>` value. X003's owner-map keying buckets by this field (anchored by quoted string `\.push\(\(item\.name\.clone\(\), "legendary"\)\)` at the xref owner-map construction site — Read this session confirms two hits, one each in `build_name_owners_map` and the X003 cross-category `name_owners`). **The rename is atomic across all readers** — every xref / ops / merge reader on `ReplicaItem` migrates in 8a because the struct-field rename is atomic with every reader.
  - Full xref anchor-by-quote set (each quote verified this session):
    - `.push((item.name.clone(), "legendary"))` — owner-map build inside `check_duplicate_pokemon_buckets`.
    - `.push((item.name.clone(), "replica_item"))` — second owner-map (the mixed-label surface §9 §F-critiques flags; see §9 for the `legendary` vs `replica_item` bucket-label asymmetry).
    - `format!("replica_items[{}].sd", item.name)` — xref field-path inside the face-compat collector.
    - `&item.sd` — the `DiceFaces` reference passed alongside the field-path quote above.
    - `item.template.as_str()` — the face-template-compat key passed to `iter_dice_faces`.
    - `ir.replica_items.iter().any(|r| r.name.to_lowercase() == lower)` — hero→replica and boss→replica V020 cross-category checks (two hits).
  - Full ops anchor-by-quote set:
    - `fn find_name_category(&self, name: &str)` body: `self.replica_items.iter().any(|r| r.name.to_lowercase() == lower)` — cross-category name collision detector, the canonical reader.
    - `pub fn add_replica_item(&mut self, item: ReplicaItem)` body: `self.find_name_category(&item.name)` + two `item.name.clone()` calls feeding `CompilerError::duplicate_name`.
    - `pub fn remove_replica_item(&mut self, name: &str)` body: `self.replica_items.iter().position(|r| r.name.to_lowercase() == lower)`.
    - Test helper (`#[cfg(test)] fn make_replica_item` inside `ir/ops.rs`): the field initializer line `name: name.into()`.
  - Merge anchor-by-quote:
    - `base.replica_items.iter().position(|r| r.name == item.name)` — the name-match for overlay → base replica-item replacement inside `ir::merge`.
  - Every callsite above rewrites from `.name` → `.target_pokemon` atomically. **Verification-time note**: per parent plan §1.3 and the `roundtrip_diag` output in §6, current `Replicas ir1=0` — no extraction happens today against the new shape, so the rename has no behavioral delta until 8b populates extraction. The verification is purely to sanity-check the reader's mental model of the field before touching callsites.
- **`template` field deletion + split replacement.** The retired `template: String` field carried different literal values at different callsites. Post-8a:
  - Emitter (`builder/replica_item_emitter.rs`, anchor by quoted string `item.template` — delete every reference) emits the literal `"Thief"` (capital, matches sliceymon source bytes).
  - Face-compat lookup (xref, anchor by quoted string `item.template.as_str()`) passes the literal `"thief"` (lowercase — face-template-compat key). **xref bucket-routing rewrite lives in 8b**; 8a only deletes the field and stubs the literal so the call compiles. **Decision: 8a stubs the literal** — it's a two-character edit that keeps 8a compiling and matches the correct 8b end-state; it does not widen 8a's scope because no new test or behavior rides on it. (Alternative — shipping the xref field-path rewrites in 8a too — is rejected because X003's bucket logic rewrite is what belongs to 8b and pulling the field-path edit in isolation would split the xref touch across two commits for no compile-state benefit.)
  - **Case-asymmetry obligation for 8b.** The emitter writes `"Thief"` (capital) and xref looks up `"thief"` (lowercase). Today this is invisible because `X016_TEMPLATE_RESTRICTIONS` is empty (no face is template-gated by the guide); once a restriction keyed on either case is added, the asymmetry silently diverges between emission and xref. 8b MUST resolve this by either (a) making `check_face_template_compat_with_table`'s `template.starts_with(prefix)` check case-insensitive, OR (b) routing both consumers through one `impl ReplicaItem { fn template_bytes(&self) -> &str }` that returns a single canonical form with a documented case convention. Shipping 8b without resolving this leaves a latent source-vs-IR divergence that an IR-vs-IR roundtrip test cannot catch.
  - Test fixtures that assert `parsed.template` / `item.template`: all live inside the `parse_legendary` test block (Read this session enumerates 8 `fn legendary_*` tests in `replica_item_parser.rs` — all deleted whole in §3.1) and inside the `replica_item_emitter.rs` legacy test block whose fixtures build the flat `ReplicaItem` shape (anchor by quoted string `use crate::extractor::replica_item_parser::parse_legendary` — Read this session shows two such lines inside `replica_item_emitter.rs`; both are in tests that round-trip the retired shape, deleted whole per §3.6).
- **`sticker` → `sticker_stack` rename + type change `Option<String>` → `Option<ModifierChain>`.** The existing chain parser accepts `.sticker.` segments (Read this session confirms `.sticker.` handling in `ir/mod.rs` — lines inside `pub enum ModifierSegment` and the chain parser, anchored by quoted strings `.sticker.`, `pub fn emit_chain`, etc. The chain parser is pre-existing; 8a's rename does not re-author it). **8a does not exercise round-trip** — no sticker-chain round-trip test ships in 8a because there is no real extractor to feed one. The authoring-builder tests (T24–T26) in 8a construct `ReplicaItem` values with `sticker_stack: None` to keep the commit compiling; the corpus round-trip assertion lands in 8b as T2b. **8b's T2b must round-trip BOTH case forms observed in the Caterpie entry** (sliceymon line 113, visible in this session's grep output: `sticker.(right.hat.Statue)` appears in the entry; and the `sticker.(right.hat.statue)` lowercase form in the inner vase-add body — the `awk 'NR==113' | head -c 300` print shows the outer-preface `Statue` form, and the full-line content verified by the broader sliceymon grep demonstrates both). A chain parser that preserves only one form silently corrupts the other. **(A7 fix) The "rollback to `Option<String>`" escape is REVOKED.** Per `CLAUDE.md` "no parallel types" + "no deferred correctness", the sticker type change cannot be partially rolled back in 8B — it is atomic with 8A's IR commit. Correct framing: **8B's T2b is a GATE, not a post-hoc escape.** If T2b cannot round-trip both `Statue` (capital) and `statue` (lowercase) forms byte-equal through `ModifierChain`, the `ModifierChain` type is inadequate for sticker payloads and **8B must design an alternative in the SAME commit as its T2b landing**: a custom `StickerChain` newtype, a manual case-preserving segment parser, or a payload-shape extension to `ModifierChain` that preserves case-per-segment. Reverting `sticker_stack: Option<ModifierChain>` to `sticker: Option<String>` after 8A ships would require re-applying 8A's atomic commit in reverse across every callsite (xref, ops, merge, tests) — mechanically equivalent to reverting 8A entirely, and explicitly forbidden. 8A's type change is load-bearing; 8B's job is to make the parser fit it, not to widen the type.
- **`sprite: SpriteId` is not `Option<SpriteId>`.** Every corpus summon item carries a `.img.<bytes>`. The sprite identity field takes the exact `.n.` bytes verbatim (no `target_pokemon.to_lowercase()` — per Chunk 3b lesson 1). 8a's authoring builder tests construct via `SpriteId::owned(name_bytes, img_bytes)` — signature Read this session from `compiler/src/authoring/sprite.rs`: `pub fn owned(name: impl Into<String>, img_data: impl Into<String>) -> Self`. Both args are `impl Into<String>` — passing `vec![]` is a type error; use `""` for a synthetic empty sprite.

### 3.4 Stub extractor (`compiler/src/extractor/replica_item_parser.rs`)

The 8a module body is the following (verbatim; 8b replaces it with the real parser):

```rust
use crate::error::CompilerError;
use crate::ir::{ItempoolItem, ReplicaItem};

/// **8a stub (transitional)**: returns one
/// `ItempoolItem::NonSummon { name: String::new(), tier: None, content: body.to_string() }`
/// per itempool, carrying the ENTIRE body verbatim inside `content`. This
/// preserves byte-equal round-trip with zero `ReplicaItem` extraction.
///
/// This is a KNOWN, TRACKED SPEC §3.2 raw-passthrough violation (see §7 and
/// §9) that 8A.5 (`plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`) closes by
/// replacing `NonSummon { name, tier, content }` with a typed
/// `NonSummonEntry` sum before 8b starts.
///
/// The empty `name` and `None` `tier` combined with a non-empty `content`
/// act as the **stub sentinel** that §3.5's `emit_itempool` detects to skip
/// the `.n.<name>.tier.<n>` prefix it would otherwise synthesize. Without
/// the sentinel, the stub would emit a stray `.n..tier.None` prefix and
/// diff every working mod's itempool on round-trip.
///
/// 8b replaces this with the real per-entry classifier (assuming 8A.5 has
/// already retyped the variant).
///
/// The signature is the final 8b shape (not a narrower 8a-only signature) so
/// that `extractor/mod.rs` and `structural_parser.rs` wire against the real
/// surface now — 8b is a body replacement, not a signature change.
pub fn extract_from_itempool(
    body: &str,
    _modifier_index: usize,
    _next_replica_index: usize,
) -> Result<ItempoolExtraction, CompilerError> {
    Ok(ItempoolExtraction {
        new_replica_items: Vec::new(),
        items: vec![ItempoolItem::NonSummon {
            name: String::new(),
            tier: None,
            content: body.to_string(),
        }],
    })
}

#[derive(Debug, Clone)]
pub struct ItempoolExtraction {
    pub new_replica_items: Vec<ReplicaItem>,
    pub items: Vec<ItempoolItem>,
}
```

Why this is not a parallel representation: the `NonSummon { name: "", tier: None, content: <whole pool> }` entry is a single `ItempoolItem` sitting where the legacy `ItemPoolEntry { content: String }` sat. One surface, one shape. When 8A.5 retypes the variant and 8b populates `new_replica_items` and splits per-entry, the same `Vec<ItempoolItem>` surface carries mixed `Summon(i)` and typed `NonSummon` entries — no surface migration needed at 8b. This is the behavior-preserving minimum 8a ships under the transitional schema.

Why the signature takes `next_replica_index` even though 8a never uses it: 8b needs it (summons get `ItempoolItem::Summon(next_replica_index + …)` indices), and wiring it in the 8a signature means `extractor/mod.rs` calls the final 8b-shaped API now. A narrower 8a signature would require a second callsite edit in 8b that provides no compile-state benefit.

### 3.5 Emitter rewrite (`compiler/src/builder/replica_item_emitter.rs`)

**Full rewrite.** The existing module emits `ReplicaItemContainer::{Capture, Legendary}` shapes and is entirely retired — the `legendary_emit_parse_roundtrip_*` tests at former lines 315 and 369 are deleted whole per §3.6 (their fixtures build `ReplicaItemContainer` values; after the enum deletion, they cannot compile).

New module public surface (complete for 8a — 8b ships no new functions here):

```rust
use crate::error::CompilerError;
use crate::ir::{ItempoolItem, ReplicaItem, SummonTrigger, DiceLocation};

/// Emitter constants — source-byte universals observed in all 4 corpus Cast
/// entries (Rainbow Wing, Silver Wing, Blue Orb, Red Orb). If a future corpus
/// entry shows variation, lift these into SummonTrigger::Cast fields in the
/// same PR (widening contract — see `ir/mod.rs` Cast doc).
const CAST_SPELL_TEMPLATE: &str = "thief";
const CAST_SPELL_DICE:     &str = "182-25:0:0:0:76-0:0";

/// Emit a single summon entry (trigger-shape + shared payload), without any
/// itempool wrapping. Shared payload goes through one private helper — no
/// N-line incantation duplicated across trigger arms.
pub fn emit_replica_item(item: &ReplicaItem) -> String {
    match &item.trigger {
        SummonTrigger::SideUse { dice, dice_location: DiceLocation::OuterPreface } => {
            emit_sideuse_outer(item, dice)
        }
        SummonTrigger::SideUse { dice, dice_location: DiceLocation::InnerWrapper } => {
            emit_sideuse_inner(item, dice)
        }
        SummonTrigger::Cast { dice } => {
            emit_cast(item, dice)
        }
    }
}

/// Pure-IR rebuild of an entire `itempool.((…)).n.<pool_name>` modifier.
/// Walks `items` in source order, joins with `+` at paren-depth 0 (verified
/// against the corpus this session: sliceymon line 113 and punpuns line 32
/// both use `+` at depth 0 as entry separator; the inner `#` characters are
/// depth-1 sub-separators inside hat-chain composition).
///
/// For `Summon(i)`, emits `emit_replica_item(&replica_items[i])`.
/// For `NonSummon { name, tier, content }` (transitional raw-passthrough —
/// 8A.5 replaces this with typed-variant dispatch), emits:
///   * **Stub sentinel path** (`name.is_empty() && tier.is_none()`) — emit
///     `content` verbatim. This is the 8a stub's whole-pool passthrough.
///   * **Populated path** (any `name` or `tier` present — reserved for
///     future callers before 8A.5 retypes) — emit
///     `<content>.n.<name>` followed by optional `.tier.<t>`.
///
/// **8a behavior**: `items` is always
/// `[NonSummon { name: "", tier: None, content: <entire body> }]` (stub
/// parser output), so this function emits `content` verbatim and the pool
/// round-trips byte-equal against source. 8A.5 retypes the variant and 8b
/// surfaces real Summon/NonSummon mixes.
///
/// The sentinel behavior is checked by T12b in `compiler/tests/retirements.rs`
/// — see §4. Without it, the stub would synthesize a stray `.n.` prefix after
/// the whole body and diff every working mod round-trip.
pub fn emit_itempool(
    items: &[ItempoolItem],
    replica_items: &[ReplicaItem],
    pool_name: &str,
) -> String { /* … */ }
```

Private helpers (not exported; shape specified below — implementer writes the string concatenation):

- `fn emit_shared_payload(item: &ReplicaItem) -> String` — handles `hat.egg.<enemy_template>.n.<target_pokemon>…` and `vase.(add.((replica.<team_template>.n.<target_pokemon>…)).mn.<target_pokemon>))` (with `.mn.` OUTSIDE the innermost `replica.(…)` but INSIDE the outer `vase.(add.((…)))` parens per parent plan §1.1), plus `sticker_stack` (via `ModifierChain::emit`), `speech`, `doc`, `img` (via `SpriteId::emit` or equivalent — verify `rg -n 'pub fn' compiler/src/authoring/sprite.rs`). One private fn used by all three emit paths.
- `fn emit_sideuse_outer(item: &ReplicaItem, dice: &DiceFaces) -> String` — outer flat preface `hat.replica.Thief.n.<target_pokemon>.sd.<dice>` + wrapped `hat.(replica.Thief.i.(all.(left.hat.egg.(<shared_payload>))))`. Capital `"Thief"` literal replaces the retired `item.template` field.
- `fn emit_sideuse_inner(item: &ReplicaItem, dice: &DiceFaces) -> String` — no outer preface; wrapped `hat.(replica.Thief.i.(all.(left.hat.egg.(<shared_payload>.i.(hat.Thief.sd.<dice>)))))`.
- `fn emit_cast(item: &ReplicaItem, dice: &DiceFaces) -> String` — wrapped `hat.(replica.Thief.i.(all.(cast.sthief.abilitydata.({CAST_SPELL_TEMPLATE}.sd.{CAST_SPELL_DICE}.i.(mid.hat.egg.(<shared_payload>.hp.<hp>.i.hat.(replica.thief.sd.<dice>.i.<item_modifiers>)…)))))`. Capital `Thief` on the outer `hat.(replica.Thief` wrapper; lowercase `thief` on the inner `replica.thief.sd.<dice>` segment — both case-preserving per corpus (parent §1.1).

Dice access at all callsites goes through `item.trigger.dice_faces()` where the surrounding code does not already destructure the variant. Inside the emit helpers above, variant-destructuring binds `dice` locally — no `dice_faces()` call needed. This is the legitimate-branching case, not the forbidden one.

**8a test coverage for the emitter**: zero corpus round-trip tests in 8a (those are 8b's T1–T5a). 8a exercises the emitter ONLY through the authoring-builder tests (T26) that construct a `ReplicaItem` and call `emit_replica_item`, asserting the output is a syntactically valid string (no specific byte-equality vs a hand-crafted fixture is required in 8a — that's T26's scope in 8b, where a fixture ties the emitter to corpus bytes). If an 8a-writer is tempted to add a corpus-fixture test here, **stop** — it belongs to 8b and the parser does not produce the fixture input yet.

### 3.5.1 `structural_emitter` signature evolution

The pre-8a `structural_emitter` handles `StructuralContent::ItemPool { body, .. }` by cloning the body string (Read this session confirms: the surviving arm in `compiler/src/builder/structural_emitter.rs` is `StructuralContent::ItemPool { body, .. } => body.clone(),` — part of a one-line-per-variant dispatch table in which EVERY variant uses `body.clone()`). The new `emit_itempool(items, replica_items, pool_name)` requires two arguments the current fn does not have. The signature evolution is:

1. **Current surface** — anchor by quoted string `pub fn emit_structural` (or whatever `^pub fn` surviving in the file). Read this session confirms the body is a `match` over `&modifier.content: &StructuralContent` with one-arm-per-variant; every arm today is the literal `body.clone()` pattern.
2. **New surface**: add a `&[ReplicaItem]` (or `&ModIR`) parameter so the `ItemPool` arm can pass `&ir.replica_items` into `emit_itempool`. Pool name is already carried by `StructuralModifier.name: Option<String>` (Read this session confirms the field exists via the `structural_parser::extract_structural_name` fn and the one-line `util::extract_mn_name(raw)` body — the structural modifier surface already carries this extracted name). No new IR field required — pass `modifier.name.as_deref().unwrap_or("")` (or handle `None` per §3.5 pool-name contract) into `emit_itempool`.
3. **Caller chain**: every caller of the renamed `emit_replica_item` and any caller of `emit_structural` must pass `&ir` / `&ir.replica_items` through. Anchor by quoted string `replica_item_emitter::emit` (Read this session confirms two hits: one in `compiler/src/builder/mod.rs` and one in `compiler/src/lib.rs`). Grep at implementation time to enumerate all callers; none should be missed by the atomic rewrite.

This signature change IS in 8a's scope because the alternative (keep the old signature, have `emit_structural` look up replica_items some other way) would require either a global/thread-local `ModIR` reference — unacceptable per WASM-safety — or a body-clone passthrough that re-introduces the raw-string path. The §3.5 emit function takes the typed list because the IR owns the structural content; emit must dereference the typed list to produce bytes. Passing `&ir` through the structural emitter is the only correct plumbing.

### 3.6 Deleted emitter tests

Delete entirely (do not rewrite; their fixtures build the flat-shape `ReplicaItem` that §3.1 retires). Anchor by quoted strings — the test function names and their `parse_legendary` imports are the load-bearing identifiers, not line numbers.

Read of `compiler/src/builder/replica_item_emitter.rs` this session confirms exactly two occurrences of the quoted substring `use crate::extractor::replica_item_parser::parse_legendary` inside the file's `#[cfg(test)]` block, followed shortly by the quoted substring `parse_legendary(&emitted, 0).expect("emit(Legendary) round-trips through parse_legendary")`. Both tests round-trip the retired flat shape through the retired `parse_legendary` — neither can compile once §3.1 lands. Delete whole (test function + its supporting fixture lines).

Additionally, the `parse_legendary`-family tests inside `compiler/src/extractor/replica_item_parser.rs` — enumerated by Read this session as 8 `#[test]` blocks with names `parses_legendary_from_top_level_item`, `legendary_name_is_last_depth0_n_before_ability`, `legendary_hp_ignores_chain_interior_sidesc`, `legendary_color_ignores_chain_interior_sidesc`, `legendary_sd_ignores_chain_interior_sidesc`, `legendary_img_ignores_chain_interior_sidesc`, `legendary_ignores_abilitydata_interior_hp_color_sd_img`, `legendary_without_item_prefix_propagates_error` — are also deleted whole per §3.1 (their `parse_legendary` callers disappear atomically with the fn). Listing them here so §3.6's deletion scope is the full set `{legendary-shape emitter tests} ∪ {legendary-shape parser tests}`.

The trigger-based round-trip tests ship in 8b (T1–T5a). The `abilitydata.is_some()`-asserting test `legendary_ignores_abilitydata_interior_hp_color_sd_img` inside `replica_item_parser.rs` relies on the retired `ReplicaItem.abilitydata` field (Read this session confirms the field's existence today); deleting the field in §3.1 deletes this test atomically, no rewrite.

### 3.7 `ir/ops.rs` migration

Edits (anchored by quoted string — Read this session confirms every anchor):

1. **`use` import at the top of the file** — anchor by quoted string `use super::{ModIR, Hero, ReplicaItem, Monster, Boss, Source};` (Read this session confirms this is the exact import line; no `ReplicaItemContainer` is currently imported since it was retired upstream). Add `ItempoolItem`, `SummonTrigger`, `DiceLocation`, `StructuralContent` (the last needed for the re-index routine below); leave the others untouched.
2. **`make_replica_item` test helper** (anchored by `fn make_replica_item`): rewrite to construct the new shape:
   ```rust
   #[cfg(test)]
   pub(crate) fn make_replica_item(name: &str) -> ReplicaItem {
       ReplicaItem {
           container_name: "Test Ball".to_string(),
           target_pokemon: name.to_string(),
           trigger: SummonTrigger::SideUse {
               dice: DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1"),
               dice_location: DiceLocation::OuterPreface,
           },
           enemy_template: "Wolf".to_string(),
           team_template: "housecat".to_string(),
           tier: Some(1),
           hp: None,
           color: None,
           // SpriteId::owned signature (verified at compiler/src/authoring/sprite.rs:80):
           //   pub fn owned(name: impl Into<String>, img_data: impl Into<String>)
           // Both params are `impl Into<String>` — NOT Vec<u8>. Passing `vec![]`
           // is a type error. Use `""` for a synthetic empty sprite.
           sprite: SpriteId::owned(name.to_string(), ""),
           sticker_stack: None,
           speech: None,
           doc: None,
           toggle_flags: None,
           item_modifiers: None,
           source: Source::default(),
       }
   }
   ```
   Defaults are sane-not-corpus (no specific Pokemon); the `5+`-existing-tests keying on `make_replica_item` keep compiling. The `DiceFaces::parse` literal is a well-formed synthetic dice string; verify the exact parse shape at `compiler/src/ir/mod.rs:44` (`rg -n 'impl DiceFaces' compiler/src/ir/mod.rs`) — if the parser rejects the literal above, pick a corpus-valid string that compiles.
3. **`add_replica_item`** — anchor by quoted string `pub fn add_replica_item(&mut self, item: ReplicaItem)`. Read this session confirms the body currently calls `self.find_name_category(&item.name)` and emits `item.name.clone()` twice into a `CompilerError::duplicate_name(...)`. Switch both `item.name` → `item.target_pokemon`.
4. **`remove_replica_item`** — anchor by quoted string `pub fn remove_replica_item(&mut self, name: &str) -> Result<(), CompilerError>`. Read this session confirms the body contains `let pos = self.replica_items.iter().position(|r| r.name.to_lowercase() == lower);` — this is the production fn. The test fn `fn remove_replica_item_by_name()` (anchor by that quoted string) is a *caller* of `remove_replica_item`, not a separate production fn; its body-side references (e.g. `ir.remove_replica_item("Pikachu")`) stay unchanged. Rewrite the production fn per the index-stability invariant.
   ```rust
   impl ModIR {
       pub fn remove_replica_item(&mut self, name: &str) -> Result<(), CompilerError> {
           // Step 1: find the index of the item being removed.
           // CompilerError::not_found signature (verified this session in
           // `compiler/src/error.rs`):
           //   pub fn not_found(type_name: impl Into<String>, key: impl Into<String>) -> Self
           // i.e. TWO arguments, not a single format!() string.
           // Note the existing behavior uses case-insensitive match (`r.name.to_lowercase() == name.to_lowercase()`);
           // the new target_pokemon matcher MUST preserve that semantic — the existing
           // `#[test] fn remove_replica_item_by_name()` body inside `ir/ops.rs`
           // depends on it, and the source-byte-preserving `target_pokemon` field
           // holds case-sensitive bytes.
           let lower = name.to_lowercase();
           let j = match self.replica_items.iter().position(|r| r.target_pokemon.to_lowercase() == lower) {
               Some(j) => j,
               None    => return Err(CompilerError::not_found("replica item", name.to_string())),
           };

           // Step 2: remove from the flat list.
           self.replica_items.remove(j);

           // Step 3: re-index every ItempoolItem::Summon(i) in every ItemPool
           // structural. This is NOT optional — without it, builds emit the wrong
           // replica or panic on out-of-bounds. T28 (shipped in 8b) pins this.
           for structural in self.structurals.iter_mut() {
               if let StructuralContent::ItemPool { items } = &mut structural.content {
                   items.retain_mut(|entry| match entry {
                       ItempoolItem::Summon(i) if *i == j => false,           // drop
                       ItempoolItem::Summon(i) if *i >  j => { *i -= 1; true } // shift down
                       _ => true,                                              // keep as-is
                   });
               }
           }

           // (A6 fix) Post-removal bounds invariant — no Summon(i) points out of
           // bounds after the shift. Preserves CLAUDE.md "no new panics" by
           // converting a silent out-of-bounds bug into an `Err` at the CRUD
           // boundary. If this assertion ever fires, the upstream shift logic
           // is incorrect and emit-time will panic on `self.replica_items[i]`;
           // catch it here, not there.
           for structural in self.structurals.iter() {
               if let StructuralContent::ItemPool { items } = &structural.content {
                   for item in items {
                       if let ItempoolItem::Summon(i) = item {
                           if *i >= self.replica_items.len() {
                               return Err(CompilerError::internal(format!(
                                   "ReplicaItem index {} out of bounds after removal (len={})",
                                   i, self.replica_items.len()
                               )));
                           }
                       }
                   }
               }
           }
           Ok(())
       }
   }
   ```
   T28 (the test that pins this routine) is in **8b**, not 8a, because it needs real `ItempoolItem::Summon(i)` entries that only 8b's real parser produces. 8a's invariant is carried by the routine's own code plus `cargo clippy` + compile. The decision to defer T28 is explicit, not accidental — the alternative (writing T28 in 8a with hand-constructed IR) would duplicate the test that 8b needs anyway, against hand-constructed fixtures rather than real parser output. 8a's scope ends at "the routine compiles and is internally consistent"; 8b's scope picks up "the routine preserves parser output under CRUD".
5. **`StructuralContent::ItemPool` destructuring callsites** — anchor by quoted string `StructuralContent::ItemPool`. Every match arm destructures `{ body, items }` → `{ items }` now. Verify no surviving reference to `body` on an `ItemPool`-arm inside `ops.rs` (anchor by quoted string `.body` on `ItemPool` destructures — the derived-structural strip / merge code uses `.body` on other `StructuralContent` variants, those stay unchanged in 8a; only `ItemPool`'s `body` field retires).

### 3.8 `builder/derived.rs` — Author `generate_hero_item_pool` against the trigger-based IR shape (A9 fix: unconditional)

Pre-condition check at implementation start:
```bash
rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs
```

**(A9 fix — pre-A9 conditional bullets deleted here; state collapsed to the single "author from scratch" branch below.)** This is NO LONGER conditional. Chunk 5 is merged (`git log`: commit `975da96`, PR #12, "Merge pull request #12 from hgorelick/feat/chunk-5-merge-derived-strip"). `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` returns 0 (verified 2026-04-24 this session). PR #12 shipped other Chunk-5 work (provenance-gated strip, `merge_with_overlay`) but did NOT ship the function. The §2 "user relaxation" language is retracted — Chunk 5's gate is satisfied (PR #12 is on main); 8A simply authors the function against the trigger-based IR shape, no conditional. **The unconditional scope:**

- Walk `ir.replica_items`; for each where `matches!(item.trigger, SummonTrigger::SideUse { .. })` (both `OuterPreface` and `InnerWrapper` route into the hero pool — `dice_location` is a source-shape discriminator, not a game-mechanic axis); emit the hero-bound `itempool.` modifier keyed on `item.target_pokemon`. `Cast` entries are skipped (their own emission path per §3.5).
- Signature matches the foundations-plan spec at `plans/PLATFORM_FOUNDATIONS_PLAN.md` (grep: `rg -n 'generate_hero_item_pool' plans/PLATFORM_FOUNDATIONS_PLAN.md`).
- Test: `derived::hero_item_pool_matches_sliceymon_via_trigger` — byte-match sliceymon's hero-bound `itempool.` source bytes.
- §2 pre-condition grep is a status check, not a halt gate: `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` → expect 0 at 8A start, ≥1 at 8A completion.

### 3.9 Authoring builder (`compiler/src/authoring/replica_item.rs` — new file)

Module surface:

```rust
use crate::ir::{DiceFaces, DiceLocation, ModifierChain, ReplicaItem, Source, SummonTrigger};
use crate::authoring::SpriteId;
use std::marker::PhantomData;

// Type-state flags for compile-time enforcement.
pub struct NoDice;
pub struct HasDice;

/// SideUse builder. Type-state flag `D` tracks whether dice have been set.
/// `.build()` is only available when `D = HasDice`.
pub struct SideUseBuilder<D> {
    container_name: Option<String>,
    target_pokemon: Option<String>,
    enemy_template: Option<String>,
    team_template: Option<String>,
    dice: Option<DiceFaces>,
    dice_location: DiceLocation,
    // … remaining fields (sprite, tier, hp, color, sticker_stack, speech, doc,
    //     toggle_flags, item_modifiers, source) set via builder methods
    _state: PhantomData<D>,
}

impl SideUseBuilder<NoDice> {
    pub fn new(container_name: impl Into<String>, target_pokemon: impl Into<String>) -> Self { /* … */ }
    pub fn dice(self, faces: DiceFaces) -> SideUseBuilder<HasDice> { /* … */ }
    pub fn dice_location(mut self, loc: DiceLocation) -> Self { self.dice_location = loc; self }
    // … setters for enemy_template, team_template, sprite, tier, hp, color,
    //     sticker_stack, speech, doc, toggle_flags, item_modifiers, source
}

impl SideUseBuilder<HasDice> {
    pub fn build(self) -> ReplicaItem {
        ReplicaItem {
            container_name: self.container_name.expect("container_name required"),
            target_pokemon: self.target_pokemon.expect("target_pokemon required"),
            trigger: SummonTrigger::SideUse {
                dice: self.dice.expect("unreachable — HasDice"),
                dice_location: self.dice_location,
            },
            // … rest
        }
    }
}

/// Cast builder. Same type-state pattern. Cast carries NO AbilityData field
/// (corpus bodies have zero depth-0 `.n.<spell_name>` — parent plan §1.1).
/// The outer cast-template and outer cast-dice are emitter literals in
/// `builder/replica_item_emitter.rs`; builder exposes no knob for them.
pub struct CastBuilder<D> { /* fields mirror SideUseBuilder minus dice_location */ }

impl CastBuilder<NoDice> { /* new, dice (→ HasDice), setters */ }
impl CastBuilder<HasDice> {
    pub fn build(self) -> ReplicaItem {
        ReplicaItem {
            container_name: self.container_name.expect("container_name required"),
            target_pokemon: self.target_pokemon.expect("target_pokemon required"),
            trigger: SummonTrigger::Cast { dice: self.dice.expect("unreachable") },
            // … rest
        }
    }
}
```

The `authoring/mod.rs` gains `pub mod replica_item;` and `pub use replica_item::{SideUseBuilder, CastBuilder, NoDice, HasDice};`. Verify existing `authoring/mod.rs` shape first (`rg -n '^pub (mod|use)' compiler/src/authoring/mod.rs`) — the re-export pattern must match existing sprite / face-id re-exports; if that file uses a different convention, follow it instead.

**Why PhantomData and not a runtime check**: the parent plan T25 explicitly says `.build()` is only available when dice are set, and this is the pattern called out in the parent plan for the SideUse builder. Runtime `Result<_, CompilerError>` on `.build()` is the fallback if PhantomData cannot be made to work for the full field set — if implementation hits a roadblock (e.g. `Clone` bound conflicts with field types that are not `Clone`), fall back to a runtime check and document the fallback on the PR. The type-state form is the preferred end-state.

## 4. Tests shipped in 8a

Four test groups. Every test is enumerated; any omission is a scope creep (add to 8b instead) or a gap (halt and escalate).

### Retirement greps (`compiler/tests/retirements.rs` — new flat integration-test file)

Per parent plan §5, retirement greps live in a new integration-test file, not `build.rs` (which is forbidden — coupling `cargo build` success to retirement absence drifts the WASM build surface).

```rust
// compiler/tests/retirements.rs
use std::fs;
use std::path::Path;

fn recursive_grep(root: &Path, pattern: &str) -> Vec<(String, usize)> {
    // walk every *.rs under root; for each line containing `pattern`, record
    // (relative_path, line_number). Pure std — no external crates.
    // Skip anything under `target/`.
    /* … */
}

#[test]
fn grep_crate_for_replica_item_container_enum() {
    // T12
    let hits = recursive_grep(Path::new("src"), "ReplicaItemContainer");
    let test_hits = recursive_grep(Path::new("tests"), "ReplicaItemContainer");
    assert_eq!(hits.len() + test_hits.len(), 0, "ReplicaItemContainer still referenced: {:?}", [hits, test_hits]);
}

#[test]
fn grep_crate_for_parse_legendary() {
    // T13
    let hits = recursive_grep(Path::new("src"), "parse_legendary");
    assert_eq!(hits.len(), 0, "parse_legendary still referenced: {:?}", hits);
}

#[test]
fn grep_crate_for_modifier_type_legendary() {
    // T14
    let hits = recursive_grep(Path::new("src"), "ModifierType::Legendary");
    assert_eq!(hits.len(), 0, "ModifierType::Legendary still referenced: {:?}", hits);
}

#[test]
fn grep_crate_for_modifier_type_replica_item_variants() {
    // T12a
    let hits_a = recursive_grep(Path::new("src"), "ModifierType::ReplicaItem ");
    let hits_b = recursive_grep(Path::new("src"), "ModifierType::ReplicaItemWithAbility");
    // Exclude the deleted-variant comment hits by checking the full substring is a type reference.
    let total = hits_a.len() + hits_b.len();
    assert_eq!(total, 0, "ModifierType::ReplicaItem{{,WithAbility}} still referenced");
}
```

Integration tests execute from `compiler/` as the working dir (verify `rg -n 'workspace|cargo' compiler/Cargo.toml` for the layout; the `Path::new("src")` reference is relative to the crate's manifest dir at test time).

### Authoring builder compile-guards (`compiler/src/authoring/replica_item.rs` `#[cfg(test)]` block)

T24 — `authoring::replica_item_builder_sideuse`:
```rust
#[test]
fn replica_item_builder_sideuse() {
    let item = SideUseBuilder::new("Great Ball", "Ivysaur")
        .enemy_template("Wolf")
        .team_template("housecat")
        .sprite(SpriteId::owned("ivysaur", ""))   // SpriteId::owned takes two `impl Into<String>` — not Vec<u8>
        .dice(DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1"))
        .dice_location(DiceLocation::OuterPreface)
        .build();

    assert_eq!(item.container_name, "Great Ball");
    assert_eq!(item.target_pokemon, "Ivysaur");
    assert!(matches!(item.trigger, SummonTrigger::SideUse { dice_location: DiceLocation::OuterPreface, .. }));
}
```

T25 — `authoring::replica_item_builder_cast` (the AbilityData-absence guard is a retirement-grep test T25a in `retirements.rs`, NOT a compile-time check — a grep that only humans run at PR review is not enforced by `cargo test`; the earlier draft's "compile-time guard" claim was aspirational and is retracted):
```rust
#[test]
fn replica_item_builder_cast() {
    let item = CastBuilder::new("Silver Wing", "Lugia")
        .enemy_template("Wolf")
        .team_template("housecat")
        .sprite(SpriteId::owned("lugia", ""))      // SpriteId::owned takes two `impl Into<String>` — not Vec<u8>
        .dice(DiceFaces::parse("15-20:15-20:36-10:36-10:184-5:184-5"))
        .build();

    assert_eq!(item.container_name, "Silver Wing");
    assert!(matches!(item.trigger, SummonTrigger::Cast { .. }));
}
```

T25a — `retirements::grep_authoring_replica_item_for_abilitydata` (the actual, `cargo test`-enforced AbilityData-absence guard; ships alongside T12–T14 in `compiler/tests/retirements.rs`):
```rust
#[test]
fn grep_authoring_replica_item_for_abilitydata() {
    // T25a: the builder deliberately exposes no `abilitydata()` method and no
    // `AbilityData` field — cast.sthief.abilitydata bodies have zero depth-0
    // `.n.<spell_name>` (parent plan §1.1). This test makes the absence
    // load-bearing under `cargo test`, not just PR review.
    let hits_a = recursive_grep(Path::new("src/authoring/replica_item.rs"), "abilitydata");
    let hits_b = recursive_grep(Path::new("src/authoring/replica_item.rs"), "AbilityData");
    assert_eq!(
        hits_a.len() + hits_b.len(), 0,
        "AbilityData re-introduced in authoring/replica_item.rs: {:?}",
        [hits_a, hits_b]
    );
}
```

T26 — `authoring::replica_item_emits_inside_itempool`:
```rust
#[test]
fn replica_item_emits_inside_itempool() {
    let item = SideUseBuilder::new("Poke Ball", "Pikachu")
        .enemy_template("Wolf")
        .team_template("housecat")
        .sprite(SpriteId::owned("pikachu", ""))    // SpriteId::owned takes two `impl Into<String>` — not Vec<u8>
        .dice(DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1"))
        .dice_location(DiceLocation::OuterPreface)
        .build();

    // Emit as a single entry — assert the output contains the expected
    // structural markers. This is NOT a corpus round-trip (that's 8b's T1);
    // it proves the emitter produces syntactically valid output for a
    // builder-constructed IR, which is the scope 8a can exercise.
    let emitted = crate::builder::replica_item_emitter::emit_replica_item(&item);
    assert!(emitted.contains("hat.replica.Thief.n.Pikachu"),
        "SideUse outer preface emit missing; got: {}", emitted);
    assert!(emitted.contains("vase.(add.((replica.housecat.n.Pikachu"),
        "vase-add pair missing; got: {}", emitted);
    assert!(emitted.contains(".mn.Pikachu"),
        ".mn. tag missing; got: {}", emitted);
}
```

T26 is deliberately string-containment, not byte-equality. Byte-equality against a hand-crafted fixture would pin implementation details (exact ordering of `.hp.` / `.col.` segments, exact `.img.` byte layout) that the corpus round-trip (8b T1) validates authoritatively. 8a's scope is "emitter produces structurally correct output for authoring-constructed input"; 8b's scope is "emitter + extractor round-trip byte-equal against corpus".

**T26a — source-vs-IR divergence guard (ships in 8a).** Per the plan-review hook rule ("Prove your tests catch source-vs-IR divergence, not just IR-vs-IR"), T26's containment assertions are stable under idempotent emit-only round-trips — they pass even if the emitter silently reaches for a registry for `target_pokemon` or `team_template`. Ship T26a in the same `#[cfg(test)]` block with inputs that would be interpreted differently if any consumer substituted a canonical/registry byte-source for the IR field bytes:

```rust
#[test]
fn replica_item_emitter_preserves_source_byte_templates() {
    // Construct a SideUse item whose team_template = "Housecat" (capital)
    // while the Pokemon name is "Ivysaur" — a combination the corpus never
    // emits (Ivysaur's corpus team_template is lowercase "housecat"). If
    // the emitter reached for a canonical lowercasing or registry-keyed
    // lookup, the emitted `vase.(add.((replica.housecat.n.Ivysaur` would
    // LOSE the capital `H` the builder set. Source-preserving emit must
    // honour the builder's bytes verbatim.
    let item = SideUseBuilder::new("Synthetic Ball", "Ivysaur")
        .enemy_template("Wolf")
        .team_template("Housecat")        // deliberately non-corpus casing
        .sprite(SpriteId::owned("ivysaur", ""))  // `impl Into<String>` for both args (not Vec<u8>)
        .dice(DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1"))
        .dice_location(DiceLocation::OuterPreface)
        .build();

    let emitted = crate::builder::replica_item_emitter::emit_replica_item(&item);
    assert!(
        emitted.contains("vase.(add.((replica.Housecat.n.Ivysaur"),
        "emitter must route team_template bytes verbatim — no case normalization, \
         no registry lookup. Got: {}", emitted
    );
    // And the symmetric Cast guard: a Cast entry whose team_template = "prodigy"
    // (corpus-valid for Ho Oh / Lugia / Blue Orb) combined with target_pokemon
    // = "Groudon" (whose corpus entry uses "Statue") must emit exactly what
    // the IR says, not what the corpus prefers for that Pokemon.
    let cast_item = CastBuilder::new("Synthetic Orb", "Groudon")
        .enemy_template("dragon")
        .team_template("prodigy")         // deliberately non-corpus pairing
        .sprite(SpriteId::owned("groudon", ""))  // `impl Into<String>` for both args (not Vec<u8>)
        .dice(DiceFaces::parse("36-10:36-10:0:0:36-10:0"))
        .build();
    let cast_emitted = crate::builder::replica_item_emitter::emit_replica_item(&cast_item);
    assert!(
        cast_emitted.contains("vase.(add.((replica.prodigy.n.Groudon"),
        "Cast emitter must route team_template from IR — not a per-Pokemon \
         registry default. Got: {}", cast_emitted
    );
}
```

T26a would FAIL if the emitter ever lowercases `team_template`, consults a Pokemon-to-template registry, or hardcodes `"housecat"` / `"prodigy"` on any code path. A lint-clean 8a emitter that uses only `item.team_template` passes trivially. This is the source-vs-IR test that an IR-vs-IR roundtrip could not catch.

### T2a specification (A5 fix — CONTRACT ships in 8a, IMPLEMENTATION ships in 8B)

T2a is called out multiple times above as the case-preservation round-trip test. It ships as an `#[test]` only in 8B (because 8B's real parser is needed to exercise it), but its contract is authored here so 8A and 8B agree on the test surface before 8B starts:

> **T2a (spec — 8B authors `#[test]`):** Extract the Red Orb entry from sliceymon (corpus-anchored via `rg -nF 'hat.egg.dragon.n.Groudon' working-mods/sliceymon.txt`); assert `matches!(item.trigger, SummonTrigger::Cast { .. })`, `item.enemy_template == "dragon"` (lowercase preserved verbatim). Emit via `emit_replica_item`; assert the emitted bytes for the `hat.egg.<enemy_template>.n.<target_pokemon>` fragment byte-equal the input bytes for that fragment. Repeat with a SYNTHETIC ReplicaItem constructed via the authoring builder: `enemy_template = "Dragon"` (capital D); emit; assert the emitted bytes contain `hat.egg.Dragon.n.` exactly (capital preserved). A classifier / emitter that normalizes case, reaches for a registry, or canonicalizes bytes fails either the corpus-byte assertion or the synthetic-byte assertion (not both — either failure is sufficient evidence of source-vs-IR drift).

Rationale for splitting the spec from the implementation: the corpus input for T2a requires 8B's real extractor (8A's stub never produces a `ReplicaItem`), but the invariant (source-byte case preservation in `enemy_template`) is baked into 8A's IR struct field docs (see §3.3 enemy_template notes). Pinning the contract in 8A prevents 8B from inadvertently weakening the test to an IR-equality round-trip.

### Transitional NonSummon round-trip guard (T30 — single test, byte-equal via raw `content`)

The transitional `ItempoolItem::NonSummon { name: String, tier: Option<i8>, content: String }` shape (§3.2) ships in 8a as a raw-passthrough: the extractor writes the entire pool body into `content`, and the emitter re-emits it verbatim. Per-variant round-trip proof is **not** 8a's scope — that ships with the typed `NonSummonEntry` sum in **8A.5** (`plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`). 8a ships a single T30 test that proves the raw-passthrough contract holds: a hand-constructed `NonSummon { name: "", tier: None, content: <body> }` round-trips byte-equal through `emit_itempool`.

```rust
// compiler/tests/retirements.rs (in the same file as T12/T12a/…)

#[test]
fn non_summon_transitional_raw_passthrough_roundtrips() {
    // T30: the 8a transitional NonSummon shape round-trips byte-equal via the
    // raw `content` field. This is the stub-sentinel path (name empty, tier
    // None, content carries the entire pool body). 8A.5 retypes the variant
    // into a typed `NonSummonEntry` sum and replaces T30 with per-variant
    // round-trip tests; until 8A.5 lands, T30 is the only test standing
    // between the transitional shape and silent corruption.
    use textmod_compiler::builder::replica_item_emitter::emit_itempool;
    use textmod_compiler::ir::ItempoolItem;

    // Corpus-sourced fragment from sliceymon line 67 Upgrade Pool (abbreviated
    // img for fixture readability; byte-equality is the property under test,
    // not the specific bytes).
    let body = "((ritemx.1697d.part.0)#(ocular amulet)#(Citrine Ring)).n.Upgrade.tier.3.img.SHORTSPRITE";
    let items = vec![ItempoolItem::NonSummon {
        name: String::new(),
        tier: None,
        content: body.to_string(),
    }];
    let replica_items: Vec<textmod_compiler::ir::ReplicaItem> = Vec::new();
    let emitted = emit_itempool(&items, &replica_items, "Test Pool");
    // Emitter contract: sentinel NonSummon (empty name, None tier) emits
    // `content` verbatim — no `.n.` / `.tier.` prefix synthesis.
    assert!(
        emitted.contains(body),
        "transitional NonSummon raw-passthrough must preserve content byte-equal; got: {}",
        emitted
    );
}
```

T30 is intentionally narrower than the per-variant typed-schema tests that ship in 8A.5. The narrower claim reflects the narrower invariant of the transitional shape: `content` is opaque, so the only correctness property is byte-equal preservation, and T12b (source-vs-IR divergence guard) already pins the extractor-side preservation. T30 pins the emitter-side preservation, completing the pair.

### Source-vs-IR divergence test (T12b — MANDATORY per hook rule)

The hook rule is non-negotiable: "prove your tests catch source-vs-IR divergence, not just IR-vs-IR." The 8a stub is trivially idempotent (stub extractor emits `NonSummon { content: body.to_string() }`; stub emitter re-emits that `content` verbatim), so every roundtrip-equality test passes even if the stub silently normalizes / drops / corrupts bytes. T26 is IR→emit→contains; retirement greps are source-only (absence). Neither catches "the stub reached for a derived / canonical / registry source instead of the input bytes."

T12b — `retirements::stub_preserves_itempool_body_byte_equal` ships in `compiler/tests/retirements.rs` alongside T12/T12a/T13/T14/T25a:

```rust
#[test]
fn stub_preserves_itempool_body_byte_equal() {
    // The 8a stub's sole behavioral promise is: NonSummon.content IS the input
    // body, byte-equal, no normalization. Feed three shapes the stub could
    // plausibly "canonicalize" if it reached for a derived source — if the
    // assertion fails, the stub is reading from a registry/hash/cache rather
    // than source bytes, and the hook-rule failure mode has slipped in.
    use textmod_compiler::extractor::replica_item_parser::extract_from_itempool;
    use textmod_compiler::ir::ItempoolItem;

    for body in [
        "(ritemx.1697d.part.0).n.A.tier.1",
        "(ritemx.1697d.part.0).n.a.tier.1",    // case-diff: a canonicalizer would fold
        "(ritemx.1697d.part.0).n.A.tier.1 ",   // trailing space: a normalizer would trim
    ] {
        let got = extract_from_itempool(body, 0, 0).expect("stub must never fail");
        assert_eq!(got.new_replica_items.len(), 0,
            "8a stub never populates Summon entries");
        assert_eq!(got.items.len(), 1,
            "8a stub always emits exactly one NonSummon per pool");
        match &got.items[0] {
            ItempoolItem::NonSummon { content, .. } => {
                assert_eq!(content, body,
                    "NonSummon.content must be BYTE-EQUAL to input. Divergence \
                     means the stub reached for a derived / normalized source — \
                     this is the exact failure mode the hook rule exists to catch.");
            }
            ItempoolItem::Summon(_) => panic!("8a stub must never produce Summon"),
        }
    }
}
```

This is the one 8a test that distinguishes "source bytes preserved" from "canonical shape recomputed." Without it, the entire 8a gate passes even if the stub silently rewrites `content` to `""`, to a registry-canonical form, or to anything else that happens to round-trip by accident through the equally-broken emitter.

### Retirement source-code deletions (no separate test; verified by compilation)

- `parse_legendary`, `parse_simple`, `parse_with_ability` tests deleted whole (they test deleted fns; `cargo test` would fail to compile them).
- `legendary_emit_parse_roundtrip_with_all_fields` and `legendary_emit_parse_roundtrip_with_item_modifiers` deleted whole (§3.6).
- `xref::x003_distinguishes_capture_from_legendary_buckets` and any sibling message-prose test live in 8b, NOT 8a (the rewrite of X003's routing is 8b scope; 8a only stubs `item.template` → `"thief"` literal at the call site to keep compilation green).

### New-enum compile-guard tests (T29a/T29b — `compiler/src/ir/mod.rs` `#[cfg(test)]` block)

`ReplicaItemContainer` (the old enum that pinned Capture/Legendary) was retired upstream with its round-trip / variant compile-guard tests (Read this session: `grep "ReplicaItemContainer" ir/mod.rs` returns 0 hits). The new enums (`SummonTrigger`, `DiceLocation`, `ItempoolItem`) need equivalent compile-guard coverage — not deferred to 8b, because the derive set and constructibility are 8a-scope concerns (they affect whether the IR compiles at all, and whether serde JSON dumps in `cargo run -- extract` work). 8A.5 extends this coverage to the typed `NonSummonEntry` sum and `InlineBody` struct when it retypes the `NonSummon` variant. This is a scope-sibling of T24/T25/T26, not a source-vs-IR test.

```rust
#[cfg(test)]
mod new_enum_compile_guards {
    use super::*;

    // T29a: every SummonTrigger variant is constructible and equality-sensible.
    #[test]
    fn summon_trigger_variants_compile_and_eq() {
        let dice = DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1");
        let a = SummonTrigger::SideUse { dice: dice.clone(), dice_location: DiceLocation::OuterPreface };
        let b = SummonTrigger::SideUse { dice: dice.clone(), dice_location: DiceLocation::InnerWrapper };
        let c = SummonTrigger::Cast    { dice: dice.clone() };
        assert_ne!(a, b, "OuterPreface vs InnerWrapper must be distinct");
        assert_ne!(a, c, "SideUse vs Cast must be distinct");
        // dice_faces() shared accessor returns the same payload across variants.
        assert_eq!(a.dice_faces(), &dice);
        assert_eq!(b.dice_faces(), &dice);
        assert_eq!(c.dice_faces(), &dice);
    }

    // T29b: ItempoolItem's two transitional variants (Summon index, and the
    // transitional raw-passthrough NonSummon { name, tier, content }) are
    // constructible, equality-sensible, and serde-roundtrippable. 8A.5
    // retypes NonSummon into a typed NonSummonEntry sum and extends this
    // coverage to each typed variant; until then, this test pins the
    // transitional shape.
    #[test]
    fn itempool_item_variants_compile_and_eq() {
        let summon    = ItempoolItem::Summon(0);
        let nonsummon = ItempoolItem::NonSummon {
            name: String::new(),
            tier: None,
            content: "hat.Dragon Egg".into(),
        };
        assert_ne!(summon, nonsummon, "Summon vs NonSummon must be distinct");
        assert_eq!(summon, ItempoolItem::Summon(0),
            "Summon(0) equals itself");
        assert_ne!(summon, ItempoolItem::Summon(1),
            "Summon(0) vs Summon(1) must be distinct");

        // Serde round-trip — anchors the `#[derive(..., Serialize, Deserialize,
        // JsonSchema)]` contract cited at §3.2. Breaks loudly if a future
        // edit drops Deserialize, which would silently break `extract`
        // output.
        for item in [summon, nonsummon] {
            let j = serde_json::to_string(&item).expect("ItempoolItem serializes");
            let r: ItempoolItem = serde_json::from_str(&j).expect("ItempoolItem deserializes");
            assert_eq!(item, r);
        }

        // SummonTrigger serde round-trip too — catches a dropped Deserialize
        // on SummonTrigger or DiceLocation.
        let t = SummonTrigger::Cast { dice: DiceFaces::parse("1-1:2-1:3-1:4-1:5-1:6-1") };
        let j = serde_json::to_string(&t).expect("SummonTrigger serializes");
        let r: SummonTrigger = serde_json::from_str(&j).expect("SummonTrigger deserializes");
        assert_eq!(t, r);
    }
}
```

These two tests replace the structural-invariant coverage that the deleted `ReplicaItemContainer` tests provided for the old enum — constructibility, variant-distinctness, and serde contract — transposed to the new enums. They do NOT substitute for the source-vs-IR tests (T10/T10a) that ship in 8b; they cover the orthogonal concern of "does the new IR schema itself hold together under the same guarantees the old schema had?" If 8a ships without T29a/T29b, a future edit that silently weakens the derive set on `SummonTrigger` / `DiceLocation` / `ItempoolItem` would not be caught until 8b's corpus tests ran — pushing a compile/schema regression weeks downstream.

**Naming note**: the T29-series number is chosen to avoid collision with the parent plan's T27/T28 (source-vs-IR and index-stability tests, both 8b scope). 8a's new tests are tracked in a distinct numeric band (T12a/T12b/T25a/T26a/T29a–b) so cross-references in 8b/8c cannot conflate structural compile-guards with source-vs-IR coverage.

## 5. Files touched

Cross-reference with parent plan §4; the table below is the authoritative 8a subset (rows from parent §4 that belong to 8b or 8c are omitted). Every row anchor is a verbatim quote from a Read made this session — no line numbers. The retirements.rs row ships the single transitional T30 round-trip guard; per-variant T30a–T30e guards land in 8A.5 with the typed `NonSummonEntry` sum.

| File | 8a change |
|---|---|
| `compiler/src/extractor/classifier.rs` | Delete the `ModifierType::Legendary` variant from `pub enum ModifierType`. Delete the gate `if starts_with_ci(modifier, "item.") { return Ok(ModifierType::Legendary); }`. Add the typed `CompilerError::classify` error path for surviving top-level `item.` input per §3.1. (`ModifierType::ReplicaItem{,WithAbility}` are already retired upstream per this session's Read of the enum declaration.) |
| `compiler/src/extractor/replica_item_parser.rs` | Delete `pub fn parse_legendary(modifier: &str, modifier_index: usize) -> Result<ReplicaItem, CompilerError>` and every `#[test]` fn whose name starts with `legendary_` or `parses_legendary_` (Read this session enumerates 8). New module body is the §3.4 stub: single `pub fn extract_from_itempool` + `pub struct ItempoolExtraction`. |
| `compiler/src/extractor/mod.rs` | Delete the dispatch arm `ModifierType::Legendary => { replica_items.push(replica_item_parser::parse_legendary(modifier, i)?); … }` (anchor by quoted string `ModifierType::Legendary` inside the dispatch `match`). Route `ModifierType::ItemPool` through `replica_item_parser::extract_from_itempool`; append returned `new_replica_items` (empty in 8a) to `ModIR.replica_items`; store returned `items: Vec<ItempoolItem>` on the `StructuralContent::ItemPool` entry. |
| `compiler/src/extractor/structural_parser.rs` | Delete the `ItemPoolEntry` construction loop (anchor by quoted string `items.push(ItemPoolEntry {`). Remove `ItemPoolEntry` from the top-of-file `use crate::ir::{…}` (anchor by quoted string `use crate::ir::{ItemPoolEntry, StructuralContent, StructuralType};`). `structural_parser.rs` no longer owns itempool entry parsing — that moves to `extractor/mod.rs` via the stub `extract_from_itempool`. The `parse_itempool` fn itself is deleted (the two returning statements `return StructuralContent::ItemPool { body: raw.to_string(), items };` become dead code post-rewrite). |
| `compiler/src/ir/mod.rs` | Delete `pub struct ItemPoolEntry`. Rewrite `pub struct ReplicaItem` per §3.2 (new fields: `container_name`, `target_pokemon`, `trigger`, `enemy_template`, `team_template`, `sticker_stack: Option<ModifierChain>`; retire `name`, `template`, `sticker`, `sd`, and `abilitydata: Option<AbilityData>`). Add `pub enum SummonTrigger`, `pub enum DiceLocation`, `pub enum ItempoolItem` (with transitional `NonSummon { name: String, tier: Option<i8>, content: String }` per §3.2 — 8A.5 retypes this into a `NonSummonEntry` sum), `impl SummonTrigger { fn dice_faces }`. Replace `StructuralContent::ItemPool { body, items: Vec<ItemPoolEntry> }` with `StructuralContent::ItemPool { items: Vec<ItempoolItem> }`. No `ReplicaItemContainer` work (already retired upstream). |
| `compiler/src/ir/merge.rs` | Anchor by quoted string `base.replica_items.iter().position(|r| r.name == item.name)` (Read this session). Rewrite to `base.replica_items.iter().position(|r| r.target_pokemon == item.target_pokemon)`. Merge mutates `replica_items` directly (not via ops.rs), so the field rename cannot be deferred. This is the only edit in merge.rs. |
| `compiler/src/ir/ops.rs` | `use` import: the current line is `use super::{ModIR, Hero, ReplicaItem, Monster, Boss, Source};` (Read this session) — add `ItempoolItem, SummonTrigger, DiceLocation, StructuralContent`. Rewrite the `#[cfg(test)] fn make_replica_item(name: &str) -> ReplicaItem` helper per §3.7 item 2 (the current body starts with `name: name.into()` — rebuild to the new struct literal shape). Migrate every `r.name` / `&item.name` / `item.name.clone()` reader on `replica_items` — anchors from this session's Read: `fn find_name_category(&self, name: &str)` body `.any(|r| r.name.to_lowercase() == lower)`; `pub fn add_replica_item(&mut self, item: ReplicaItem)` body `self.find_name_category(&item.name)` + two `item.name.clone()` calls; `pub fn remove_replica_item(&mut self, name: &str)` body `.position(|r| r.name.to_lowercase() == lower)`. Rewrite the production `remove_replica_item` per §3.7 item 4. Keep the `fn remove_replica_item_by_name` test name unchanged (it is a caller, not a separate production fn). Update every `StructuralContent::ItemPool { body, items }` destructure to `{ items }`. |
| `compiler/src/builder/replica_item_emitter.rs` | Full rewrite per §3.5. Current public signature (Read this session): `pub fn emit(item: &ReplicaItem) -> Result<String, CompilerError>` with a private `fn emit_legendary(item: &ReplicaItem) -> Result<String, CompilerError>`. Rename to `emit_replica_item` AND change signature to non-fallible `-> String` per §3.5. Add `emit_itempool(items, replica_items, pool_name) -> String` + three private trigger-variant helpers + the constants `CAST_SPELL_TEMPLATE` / `CAST_SPELL_DICE`. Literal `"Thief"` (capital) at every `hat.(replica.Thief.…` emission site replaces the retired `item.template` field. Delete every `#[test]` whose body contains `use crate::extractor::replica_item_parser::parse_legendary` (Read this session confirms two such tests inside the file's `#[cfg(test)]` block). The rename ripples to `builder/mod.rs` and `lib.rs` (separate rows below). |
| `compiler/src/builder/structural_emitter.rs` | The current arm (Read this session) is `StructuralContent::ItemPool { body, .. } => body.clone(),` inside a one-arm-per-variant dispatch table. Replace with `replica_item_emitter::emit_itempool(items, &ir.replica_items, pool_name)`. **Signature evolution required** — `emit_structural` (or whichever fn covers this match arm) must accept `&ir.replica_items` and a pool name; see §3.5.1 for the full call-chain spec. Every other `StructuralContent` arm is unchanged in 8a. |
| `compiler/src/builder/mod.rs` | Anchor by quoted string `modifiers.push(replica_item_emitter::emit(item)?);` (Read this session). Rewrite to `modifiers.push(replica_item_emitter::emit_replica_item(item));` (drop the `?`; new signature is non-fallible per §3.5). If §3.5.1 evolves `emit_structural`'s signature, pass `&ir.replica_items` through this callsite too. |
| `compiler/src/lib.rs` | Anchor by quoted string `builder::replica_item_emitter::emit(item)` (Read this session). Rewrite to call the renamed `emit_replica_item`. If the existing `pub fn build_replica_item` returns `Result<…, CompilerError>`, wrap the infallible inner call (`Ok(...)`); keep the outer return type stable. Match whatever convention adjacent lib helpers (`build_hero`, `build_monster`) use. |
| `compiler/src/xref.rs` | `ReplicaItem.{name,sd,template,abilitydata}` are all retired in 8a; every xref reader must migrate atomically. Full 8a edit list, each anchor a verbatim quote Read this session:<br/>• `.push((item.name.clone(), "legendary"))` → `.push((item.target_pokemon.clone(), "legendary"))` (owner-map build inside X003's bucket collection). Keep the `"legendary"` label — see §9 for the bucket-label unification question; that rewrite ships in 8b.<br/>• `.push((item.name.clone(), "replica_item"))` → `.push((item.target_pokemon.clone(), "replica_item"))` (second owner-map).<br/>• `format!("replica_items[{}].sd", item.name)` → `format!("replica_items[{}].sd", item.target_pokemon)`.<br/>• `&item.sd` → `item.trigger.dice_faces()`.<br/>• `item.template.as_str()` → literal `"thief"` (face-template-compat key).<br/>• Both `ir.replica_items.iter().any(|r| r.name.to_lowercase() == lower)` calls (hero V020, boss V020) → `.any(|r| r.target_pokemon.to_lowercase() == lower)`.<br/>• Test helper `fn make_replica_item(name: &str) -> ReplicaItem` (Read this session — currently at the file's `#[cfg(test)]` block, initializes `name: name.to_string(), template: "Slime".to_string(), hp: Some(4), sd: DiceFaces { faces: vec![DiceFace::Blank] }, …, abilitydata: None, …`): rewrite body to construct the new `SummonTrigger::SideUse { dice, dice_location: OuterPreface }` shape with `target_pokemon`/`container_name`/`enemy_template`/`team_template`/`sticker_stack`. Callers (`ir.replica_items.push(make_replica_item("Pikachu"))`, etc.) continue to compile unchanged.<br/>X003 match-arm bodies, `Finding` shape, and V020 message-prose all stay 8b. |
| `compiler/src/builder/derived.rs` | Per §3.8 — Chunk 5 pre-condition grep returns 0 this session (`fn generate_hero_item_pool` absent), so the "new authoring" branch activates (only if the user relaxes the Chunk 5 gate; otherwise 8a halts). Scope adapts to the §2 pre-condition grep. |
| `compiler/src/authoring/replica_item.rs` | **New file** per §3.9. `SideUseBuilder<NoDice\|HasDice>` + `CastBuilder<NoDice\|HasDice>` + `NoDice` / `HasDice` marker structs. `#[cfg(test)]` block with T24 + T25 + T26 + T26a (source-vs-IR divergence guard — see §4). |
| `compiler/src/authoring/mod.rs` | Add `pub mod replica_item;` + `pub use replica_item::{SideUseBuilder, CastBuilder, NoDice, HasDice};` (follow the existing re-export convention — anchor by quoted string `pub mod sprite` or whatever `^pub mod` precedes the new addition). |
| `compiler/tests/build_options_tests.rs` | Anchor by quoted string `ir.replica_items.push(ReplicaItem {` (Read this session confirms the `v020_cross_category_source_is_global` test body constructs the current flat shape: `name: "Pikachu".to_string(), template: "Slime".to_string(), hp: Some(4), sd: DiceFaces { faces: vec![DiceFace::Blank] }, sprite: SpriteId::owned("pikachu", ""), color: None, tier: None, doc: None, speech: None, abilitydata: None, item_modifiers: None, sticker: None, toggle_flags: None, source: Source::Base`). Rewrite the struct literal to the new shape (`container_name`, `target_pokemon`, `trigger: SummonTrigger::SideUse { … }`, `enemy_template`, `team_template`, `sticker_stack: None`, etc.). No `ReplicaItemContainer` work (already retired upstream). |
| `compiler/tests/integration_tests.rs` | At implementation time, anchor by quoted strings `parse_simple`, `parse_with_ability`, `parse_legendary`, `ReplicaItemContainer` — any hit is 8a scope and must be rewritten against `extract_from_itempool` via `ModifierType::ItemPool`. This session's earlier tribunal audit reported zero matches in the post-merge tree; the row stays as a safety net for implementation-time verification. |

Also created: `compiler/tests/retirements.rs` (new integration-test file; T12, T12a, T13, T14, T12b, T25a, and the single transitional T30 NonSummon round-trip guard — see §4 below). T30 pins the emitter-side byte-equal preservation of the transitional `NonSummon { name: "", tier: None, content: <body> }` shape; T12b pins the extractor-side preservation. Per-variant round-trip guards (T30a–T30e) land with the typed `NonSummonEntry` sum in 8A.5, not here.

Total atomic artifacts: the production code changes + `authoring/replica_item.rs` (new) + `authoring/mod.rs` (re-export) + the two test files (`tests/build_options_tests.rs` rewrite + `tests/retirements.rs` new) + conditional `tests/integration_tests.rs` + conditional `builder/derived.rs`. Test files count toward the blast radius; the `≤5-file` AI-dev soft limit does NOT carve them out. The justifying exception is the atomic-rewrite rule declared in §1 — every callsite must be updated in the same commit or `cargo build` cannot be green.

Parent plan §4 rows **not** in 8a's scope (confirm none accidentally drift in):
- `compiler/tests/roundtrip_baseline.rs` — 8b (baselines are regenerated to reflect `replica_items.count: 23 -> 23`).
- `compiler/tests/correctness_tests.rs` — 8a deletes any test asserting `parse_legendary` reachability; re-grep before landing (`rg -c 'parse_legendary|ReplicaItemContainer' compiler/tests/correctness_tests.rs`). If hits, delete in 8a; if zero, file is out of scope.
- `SPEC.md`, `plans/PLATFORM_FOUNDATIONS_PLAN.md`, `.claude/settings.json` — 8c.
- `compiler/examples/roundtrip_diag.rs`, `compiler/examples/drift_audit.rs` — 8c.

The parent §4 row for `compiler/src/util.rs` is scoped **only** if Chunk 9 did not land on `main`; per §2 pre-condition, 8a halts if `slice_before_chain_and_cast` is missing, so the util.rs row is never 8a's responsibility.

## 6. Verification gate (8a → 8b handoff)

All must be green before any 8b commit lands. Re-run from a clean tree after the final 8a commit.

```bash
# All commands run from `compiler/` unless noted.

# 1. Build + lint clean.
~/.cargo/bin/cargo build
~/.cargo/bin/cargo clippy -- -D warnings

# 2. All tests pass (retirement greps + authoring builders + existing).
~/.cargo/bin/cargo test

# 3. All four mods roundtrip byte-equal with Replicas=0 (stub parser output).
~/.cargo/bin/cargo run --example roundtrip_diag
# Expected stdout format (verified at compiler/examples/roundtrip_diag.rs:85-87):
#   Status: ROUNDTRIP OK
#   Replicas  ir1=   0 ir2=   0 delta=+0
# (NOTE: two spaces after "Replicas" per `{:4}` width in the example — the
# earlier "Replicas ir1=0" quote was misformatted.)
# Expected on sliceymon, pansaer, punpuns, community. The NON-zero Replicas
# count (23 for sliceymon) arrives in 8b — not an 8a defect.

# 4. Retirement greps (bodies of T12/T12a/T13/T14/T25a/T12b; word-anchored —
#    bare `parse_simple` false-positives `parse_simple_choice` in
#    phase_parser.rs. Note: rg regex uses `|` for alternation, NOT `\|` (sed
#    syntax; matches a literal `|` character).
rg -c 'ReplicaItemContainer\b' compiler/                                        # expect 0 (src + tests + examples)
rg -c 'ItemPoolEntry\b' compiler/                                               # expect 0
rg -c '\b(parse_legendary|parse_simple|parse_with_ability)\b' compiler/         # expect 0
rg -c 'ModifierType::(Legendary|ReplicaItem(WithAbility)?)\b' compiler/src/     # expect 0
rg -c '\bitem\.(template|name|sticker|sd)\b' compiler/src/                      # expect 0

# 5. Zero AbilityData references to SummonTrigger (confirms the §3.2 widening
#    contract: Cast carries no AbilityData).
rg -c 'SummonTrigger::Cast\s*\{[^}]*ability' compiler/                          # expect 0
rg -c 'abilitydata|AbilityData' compiler/src/authoring/replica_item.rs          # expect 0

# 6. Builder dispatch reaches the new emitter (bare `|`, rg regex; not `\|`).
rg -c 'emit_replica_item|emit_itempool' compiler/src/builder/                   # expect ≥2
```

**Failure mode**: if the roundtrip emits a diff on any mod, the stub's `NonSummon { name: "", tier: None, content: <body> }` is not being re-emitted verbatim — the emitter's `emit_itempool` sentinel detection (empty `name` + `None` `tier` → emit `content` verbatim) is wrong for the stub case. Fix before 8b; do not proceed.

**Non-failure**: `roundtrip_diag` reporting `Replicas  ir1=0 ir2=0 delta=+0` on sliceymon is **expected** in 8a. The "23" count is the 8b target. An 8a reviewer who sees 0 and flags it is confused about scope — point them here.

## 7. Structural check (per hook rules)

- **Collapses two paths with different invariants?** **Yes — temporarily, and explicitly, and only for the life of the 8a stub.** The 8a stub extractor collapses the "N +-joined entries per pool" invariant into "one `NonSummon { name: \"\", tier: None, content: <whole body> }` per pool holding the entire body". The strict-authoring path (via `SideUseBuilder` / `CastBuilder`) still produces the rich typed shape; the permissive-extract path temporarily produces a trivially-idempotent single-entry list. This IS the permissive-vs-strict collapse the hook rule warns about. It is NOT masquerading as an implementation detail — §3.4 names it as "8a stub" and §6 calls out `Replicas ir1=0 ir2=0` as the expected output. 8b removes the collapse by shipping the real per-entry classifier (after 8A.5 retypes the variant). T12b (source-vs-IR divergence guard in retirements.rs) pins that the collapse is temporary and that the stub does not silently normalize / drop / corrupt bytes: the transitional `NonSummon.content` IS the input body, byte-equal.
- **N-line incantation duplicated across callsites?** No. Dice access routes through `SummonTrigger::dice_faces()` at every consumer site (emitter, authoring, xref stub). The emitter's three trigger arms share `emit_shared_payload` — no copy-paste per trigger.
- **Canonical set restatement?** Not in 8a. The `{hero, replica_item, monster}` set is named by SPEC §6.3 (8c rewrites it) and referenced by name from X003 / V020 (8b rewrites those). 8a does not touch either surface.
- **Evidence for every variant?** Required ≥ 1 corpus instance per retained variant, per parent plan §6. Corpus-quoted anchors for each variant live in §3.2's doc-comments; summary here:
  - `SummonTrigger::SideUse { dice_location: OuterPreface }`: 18 (Balls Part 1/2 except Master Ball?).
  - `SummonTrigger::SideUse { dice_location: InnerWrapper }`: 1 (Master Ball? on sliceymon line 111).
  - `SummonTrigger::Cast`: 4 (Rainbow Wing, Silver Wing, Blue Orb, Red Orb on sliceymon lines 115/117).
  - `ItempoolItem::Summon`: 23 total in sliceymon (zero in other mods); exercised by 8b's real parser, carried as an invariant in the typed IR here.
  - `ItempoolItem::NonSummon` (transitional raw-passthrough, `content: String`): every itempool in every working-mod. 3 of 4 mods (pansaer, punpuns, community) have ONLY NonSummon content — no summons. Corpus evidence examples: punpuns line 14 bare-name list (`Dead Crow` / `Amnesia` / …), punpuns line 32 mixed entries (`Splinter.m.3`, `ritemx.89f2`, `(ritemx.1697d.part.0)`, `(Diamond Ring.splice.Change of Heart).n.Deflect`, `hat.Dragon Egg`), community line 32 base-game refs, sliceymon line 67 Upgrade Pool. 8A.5 retypes the variant into a `NonSummonEntry` sum with per-variant corpus-anchor evidence; 8a's transitional shape requires only the single "itempools exist and have content" invariant, which every mod satisfies.
  - Retired variants (`ReplicaItemContainer`, `ItemPoolEntry`, `ModifierType::{Legendary, ReplicaItem, ReplicaItemWithAbility}`, `parse_{simple, with_ability, legendary}`): 0 instances each. Retirement is total, not "in progress".
- **Raw passthrough?** **8a ships with SPEC §3.2's raw-passthrough violation intentionally open for itempool non-summon entries.** The transitional `ItempoolItem::NonSummon { name: String, tier: Option<i8>, content: String }` variant (§3.2) carries an opaque `content: String` that violates SPEC §3.2's "every IR field must be derivable from typed corpus bytes". This is a TRACKED KNOWN violation, not deferred correctness — sibling chunk **8A.5** (`plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`) replaces the variant with a typed `NonSummonEntry` sum before 8b starts; the violation cannot outlive the 8a → 8A.5 → 8b chain. See §7 (risks) and §8 (ship-blocker) for the binding ordering. Summon-side fields (on `ReplicaItem`) remain exhaustive — no `extras: Vec<RawSubBlock>` on `ReplicaItem`, no `raw_remainder: String`. Per the user's corpus framing, non-summon entries are the DOMINANT shape across working-mods (3 of 4 mods have zero summons; only sliceymon carries 23), so closing this violation in 8A.5 is the plan chain's dominant-shape work — 8a's trigger-IR is a sliceymon-specific specialization on top.
- **Source-vs-IR test present?** 8a's tests are structural (retirement greps + authoring compile-guards) PLUS the new **T26a source-vs-IR divergence guard** (§4). T26a builds synthetic `ReplicaItem` values with deliberately non-corpus `team_template` casing / Pokemon pairings and asserts the emitter routes those IR bytes verbatim — failing loudly if any emit code path substitutes a registry-canonical or derived value for the builder's source bytes. The deeper source-vs-IR tests that prove classification reads inner payload not name (T10, T10a) still ship in 8b where they have a real parser to exercise. 8a's authoring-builder tests (T24–T26) guard **construction** correctness; T26a guards **byte-routing** correctness — complementary, not a substitute.
- **Integration assertions encode reason?** 8a has no integration assertions beyond `roundtrip_diag` reporting `Status: ROUNDTRIP OK` — a one-line structured signal, not substring-scan on message prose.

## 8. Out of scope (explicit, not deferred work)

The following are 8b or 8c responsibilities. Any 8a commit that touches them is scope creep — halt, split, and ship the creep in its proper sub-chunk.

- **Real itempool entry classifier** (parent §3.3): paren-depth walk, summon detection (egg + vase-add pair), trigger classification (SideUse-outer / SideUse-inner / Cast), sticker-chain parsing, `Finding` emission for unclassifiable shapes. **8b.**
- **Baseline regeneration** to non-zero Replicas counts. **8b.**
- **Xref rewrite**: `Finding` widening (`buckets: Vec<&'static str>`, `includes_boss: bool`), X003 match-arm deletion, V020 predicate rewrite, test deletions. **8b.**
- **SPEC.md rewrites** (§6.3 line 246 + line 342 + line 343 + downstream mentions at lines 78, 104, 168, 188, 335 — anchor by quoted substring before editing; 8c re-verifies line numbers against the then-current file). **8c.**
- **SPEC §3.2 raw-passthrough violation closer** — the `ItempoolItem::NonSummon { content: String }` transitional variant (§3.2) is the open violation 8a ships with. Closing it is the scope of sibling chunk **`CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`** (**8A.5**), which replaces `content: String` with a typed `NonSummonEntry` sum (`BaseGameRef` / `MultiplierRef` / `RitemxRef` / `Splice` / `Inline`) plus per-variant T30a–T30e round-trip guards. 8A.5 is a **ship-blocker for 8b** — 8b cannot start until 8A.5 has landed. Dependency chain: 8a → 8A.5 → 8b.
- **`plans/PLATFORM_FOUNDATIONS_PLAN.md` rewrites** (§F7 rewrite, dep-graph edits, Chunk 5/6/8/9 block rewrites). **8c.**
- **`.claude/settings.json` fourth hook bullet.** **8c.**
- **`compiler/examples/roundtrip_diag.rs` per-trigger breakdown**. **8c** (optional cosmetic; no behavior change).
- **T1, T2, T2a, T2b, T3–T7, T8, T9, T9a, T9b, T9c, T10, T10a, T11, T15–T21, T22, T23, T23a, T27, T28** — every source-vs-IR or corpus round-trip test. **8b** (most) or 8b/8c depending on the test.

## 9. Risks (8a-specific)

- **The authoring builder's `PhantomData<HasDice>` type-state may conflict with a field that is not `Clone`.** Mitigation: the fields of `SideUseBuilder` / `CastBuilder` are all owned values (`String`, `Option<T>`, etc.) — `PhantomData<T>` is always `Copy` regardless of `T`. If a conflict surfaces (e.g. `ModifierChain` not `Clone`), the fallback is a runtime `Result<ReplicaItem, CompilerError>` `.build()` — document on the PR, do not deepen the 8a scope.
- **`make_replica_item` callers may exercise field patterns the new shape does not accept.** The existing callers in `ir/ops.rs` tests currently reference `item.name` (dropped) — Read this session confirms the current helper body uses `name: name.into()` inside the struct literal; callers that destructure on `.name` must migrate. 8a rewrites the helper and migrates the callers in the same commit (anchor by quoted string `make_replica_item(` — enumerate callers at implementation start). If a caller asserts on a field the new shape doesn't carry, delete the assertion — it was pinning the retired shape, and the assertion re-authoring belongs with `ops.rs` tests in 8a.
- **`generate_hero_item_pool` absence (§3.8 branch)**: Chunk 5's merged state omits the function (Read this session: `grep -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` = 0). 8a authors it against the new IR shape IFF the user relaxes the parent §2 pre-condition. The new-authoring path needs a byte-equality assertion against sliceymon's hero-bound ItemPool source bytes — one test, one mod fixture inside `compiler/src/builder/derived.rs` `#[cfg(test)]`. If the user does not relax the gate, 8a halts; the pre-condition check in §2 is the first step of the PR, not an afterthought.
- **Emitter round-trip with NonSummon stub output**: 8a's `emit_itempool` receives `items = [NonSummon { name: "", tier: None, content: <entire pool body> }]` from the stub. The output must be exactly the pool body verbatim (no `+` joiner prefix, since there's only one entry; no `.n.` / `.tier.` synthesized prefix, per the sentinel detection — empty `name` + `None` `tier` means emit `content` verbatim). If a surviving `+` or stray `.n.` leaks in, sliceymon's existing pools (whose bodies contain internal `+` joiners inside `content`) will still roundtrip correctly, but pansaer's (empty-body) pools and punpuns' TM-only pools may diff. Test by running `cargo run --example roundtrip_diag` against all four mods at gate-time — not just sliceymon. T12b in `retirements.rs` is the stub-side source-byte preservation guard; the round-trip gate is the emitter-side guard.
- **SPEC §3.2 raw-passthrough violation is intentionally open for the duration of 8a** (per §7 Raw passthrough bullet and §8 ship-blocker). 8A.5 closes it by retyping the `NonSummon` variant into a `NonSummonEntry` sum before 8b starts. This is a TRACKED KNOWN violation — named, scoped, time-boxed to the 8a → 8A.5 window — not deferred-correctness. An 8a reviewer who sees the `content: String` raw-passthrough and flags it without noticing the 8A.5 ship-blocker has misread the plan chain; point them to §8.
- **Bucket-label asymmetry (`"legendary"` vs `"replica_item"`) in xref.rs (decision #4 — deferred to 8b)**: (A8 fix — anchors verified 2026-04-24 this session via `rg -nF '"legendary"' compiler/src/xref.rs` returning line 205 and `rg -nF '"replica_item"' compiler/src/xref.rs` returning line 500). Read this session confirms xref currently uses TWO inconsistent bucket labels for `ReplicaItem`: the X003 owner-map uses `"legendary"` (at `.push((item.name.clone(), "legendary"));` xref.rs:205); a second owner-map used elsewhere uses `"replica_item"` (at `.push((item.name.clone(), "replica_item"));` xref.rs:500). The user ruled that bucket labels should unify to `"replica_item"`. 8a does NOT perform this unification — per parent-plan and 8B-plan scope, xref bucket-routing is 8b's territory, and 8a's scope is already maximal under the atomic-compile rule. **8b responsibility**: unify both owner-map sites to `"replica_item"`, update the X003 message-prose test (anchor by quoted string `x003[0].message.contains("legendary")` — Read this session confirms this assertion exists) and the corresponding suggestion-contains test (anchor by quoted string `suggestion.contains("hero") && suggestion.contains("legendary")`), and include a SPEC §6.3 prose update in the 8b PR (SPEC prose concerning the `{hero, legendary, monster}` canonical set — 8b may also coordinate with 8c on whether the SPEC edit belongs in 8b or 8c, but the code unification MUST land with 8b). 8a preserves the current `"legendary"` label in the owner-map site it touches (rename-only `item.name.clone()` → `item.target_pokemon.clone()`), not because 8a blesses it, but because unilaterally renaming the label in 8a without updating the paired tests and SPEC would break compile & break a downstream test in one atomic commit that 8a does not own.

## 10. Open questions

None. Every decision for 8a has corpus evidence or a CLAUDE.md-grounded rationale. If implementation surfaces a new one, escalate — do not choose silently.
