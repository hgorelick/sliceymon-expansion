# Plan: Fix every defect found by the chunk-8 tribunal

## Context

The user asked for a review of the five chunk-8 plan files
(`CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` + four sub-plans `8A`, `8A5`, `8B`, `8C`)
to ensure they are coherent, correct, and grounded in repo/corpus/guide reality.

A multi-persona adversarial tribunal (5 plans × 3 personas = 15 agents) was run,
followed by a verification pass (3 Explore agents) that ground-truthed every
contested claim against actual repo state. The findings below are the surviving,
verified defects — reviewer hallucinations have been retracted.

Per user directive: **no priority tiers — every finding is critical and must be
fixed.** This plan edits only the five plan `.md` files. No compiler code is
touched; implementation of the plans themselves is out of scope.

The plans are not yet executed (8A, 8A5, 8B have not shipped; Chunk 9 has
landed), so these edits straighten the plans before implementation begins.

## Ground-truth snapshot (verified this round)

| Claim | Source | Status |
|---|---|---|
| `xref.rs:205` pushes bucket label `"legendary"` (X003) | grep | ✓ |
| `xref.rs:500` pushes bucket label `"replica_item"` (V020) | grep | ✓ |
| `xref.rs:946` contains comment `// …`ReplicaItemContainer`` | Read | ✓ — 8C pre-cond grep returns 1, not 0 |
| `Finding {` sites: 13 in xref.rs, 3 outside (lib.rs:84, ir/merge.rs:79, build_options_tests.rs:277) | grep | ✓ (all use `..Default::default()`) |
| X003 prose-scan assertions at xref.rs:963,964,965 | Read | ✓ (`hero`, `legendary` substrings) |
| `SummonTrigger`, `DiceLocation`, `ItempoolItem`, `NonSummonEntry`, `ItempoolExtraction` all absent from `compiler/src/` | grep | ✓ (expected — 8A not landed) |
| Current `ReplicaItem` struct has fields `name, template, hp, sd, sprite, color, tier, doc, speech, abilitydata, item_modifiers, sticker, toggle_flags, source` | Read | ✓ |
| SPEC.md line 78 reads `classifier → type parsers (hero/capture/monster/` | Read | ✓ |
| SPEC.md lines 246 AND 254 BOTH contain `(captures / legendaries)` parenthetical | Read | ✓ — parent §3.6 targets 246; 8C §2 targets 254. Must reconcile. |
| SPEC.md lines 342/343/350/351 overlap into a cluster of ReplicaItem glossary lines | Read | ✓ — diff carefully to avoid double-application |
| `ModifierType` enum still has `Legendary` variant (classifier.rs:18) | Read | ✓ — 8C pre-cond #1 grep `rg -c 'ModifierType::(Legendary\|…)' compiler/src/` returns ≥1, not 0 |
| pansaer.txt is a 350KB monoline file | Bash | ✓ — every "pansaer line N" citation is unreliable |
| V15 corpus citations (`learn.Poke`, `learn.Bandage` in pansaer) not found via grep | grep | ⚠ evidence gap; needs Phase-A walk or variant deletion |
| community.txt:13, :82, :130, :103 confirm V2, V4, V8, V13 respectively | grep | ✓ partial |
| Repo-root `.claude/settings.json` has 1 PreToolUse hook (`Read`); worktree has 2 (`Read` + `Edit|Write|NotebookEdit`) | jq | ✓ — 8C §3.3 optional-upstream task is valid |
| Retracted reviewer claims: "only one bucket label exists" (false — two exist); "no `Finding {` sites in repo" (false — 16 exist) | — | ✓ retracted |

## Cross-cutting fixes (apply to multiple plans)

### XC-1. pansaer line-number citations

pansaer.txt is a single-line file (verified this conversation: the 8A5
verification agent reported pansaer is a ~350 KB monoline; every "pansaer line
N" citation for N ≥ 2 resolves to line 1 and is useless as an anchor). Every
plan that cites `pansaer line N` must be rewritten to use a verbatim grep
anchor (`rg -nF '<substring>' working-mods/pansaer.txt`) or an explicit
"byte-offset via Phase-A walk" deferral. Affected files: `8`, `8A`, `8A5`.
Each occurrence is listed in that plan's fix section below.

### XC-2. Pokemon-specific field name `target_pokemon` must become generic

**User rule (MEMORY.md):** "IR types must use generic names, not mod-specific
(`ReplicaItem` not `Capture`/`Legendary`)." User reinforced this turn: "B1
references 'pokemon' in actual code that makes this whole thing specific to
sliceymon and not generic — it's ok for it to be in prose since the eventual
goal is sliceymon+".

The 8A plan introduces `ReplicaItem.target_pokemon: String` — a Pokemon-specific
field name baked into IR code. This violates the rule. Prose that says
"Pokemon" (glossary, doc-comments, test fixtures named after Pokemon) is fine;
the **field identifier** in Rust source is not.

**Recommended generic name:** `summon_name` (parallels `SummonTrigger`;
describes "the name of the thing the item summons"; not bound to Pokemon).

Alternatives if the user prefers a different tone:
- `target_name` — minimal delta from `target_pokemon`; descriptive
- `entity_name` — pairs naturally with `enemy_template` terminology

**Pick one before execution.** This plan assumes `summon_name` until the user
overrides.

**Cascade required** (all downstream plans must flip in the same commit as
8A's IR rewrite):

- `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` — master plan references
  `target_pokemon` in the struct definition (§3.2 line ~231), field-note prose
  (§3.3 line ~359), payload emitter shared-helper spec (§3.4 lines ~432–435),
  xref rename table (§3.5 lines ~450–451), derived.rs migration spec (§3.7 line
  ~514, §4 line ~542), test assertions (§5 lines ~577, 581, 594, 624), §9 file
  list (lines ~700, 703, 726, 731). Grep the whole file for `target_pokemon`
  and rename each hit.
- `plans/CHUNK_8A_REPLICA_IR_ATOMIC_REWRITE.md` — struct doc-comments (§3.2
  lines ~170–206), field-note prose (§3.3 lines ~329–349), rename table
  callsite migration (§3.1 lines ~336–349), authoring-builder code samples
  (§3.6 lines ~521, 545–557, 618, 629, 640, 661), test samples (§5 lines
  ~743, 810–839, 596–599). Grep `target_pokemon` and rename.
- `plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md` — pre-condition grep
  (§2 line ~63), `detect_summon_pair` extraction spec (§3.1 line ~120),
  xref rename table (§3.4 line ~174), and any other `target_pokemon` hit.
  Grep and rename.

**Emitter/extractor behavior is unchanged** — only the field identifier flips.
No corpus byte ever mentions "pokemon"; the semantic content (source-byte
preservation of the inner `.n.<name>` bytes) is orthogonal to the Rust field
name. Confirmed by earlier verification-agent read of sliceymon.txt lines
that the `.n.` token is always followed by Pokemon names **as data**, never
by any Rust identifier reference.

### XC-3. "legendary" lifecycle — clarify the plan is removal-forward, not retention

User raised the concern this turn: "multiple spots reference `legendary` which
I thought is being removed?"

The word `"legendary"` appears in this plan in three distinct contexts, all
consistent with removal:

1. **Current-state description (inevitable):** xref.rs:205 today pushes bucket
   label `"legendary"`; classifier.rs:18 today has `ModifierType::Legendary`
   variant. These are the facts the removal is operating on. Any fix spec that
   says "rename `"legendary"` → `"replica_item"`" or "delete
   `ModifierType::Legendary`" must quote the current-state literal to be
   actionable. (Sites in this plan: A8, B1 left-hand side of rename tables.)
2. **Removal direction (the fix itself):** bucket rename at xref.rs:205,
   assertion deletions at xref.rs:963–965, variant deletion of
   `ModifierType::Legendary`. (Sites: B1, B2, C2 pre-condition grep.)
3. **SPEC prose quoted from parent authority (user-permitted):** parent
   §3.6's glossary text acknowledges Legendary as a player-facing concept —
   per user "it's ok for it to be in prose." (Sites: C4 verbatim quote of
   parent §3.6.)

**Post-state (after all fixes land and all chunks execute):** zero occurrences
of `"legendary"` in `compiler/src/` source or tests. Verification grep:

```bash
rg -nFc '"legendary"' compiler/src/ compiler/tests/                          # expect 0
rg -nc 'ModifierType::Legendary\b' compiler/src/                             # expect 0
rg -n 'message.contains("legendary")' compiler/                              # expect 0
```

SPEC prose and user-facing game flavor may continue to use the word per user
allowance.

---

## Fixes — `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` (master)

### M1. Title foregrounds the SPEC amendment

Current title buries that this chunk retires SPEC's Capture/Legendary kind
axis. Rewrite title to: `Chunk 8: Typed itempool + trigger-based ReplicaItem
redesign + SPEC amendment (retire Capture/Legendary kinds) + 3-bucket
X003/V020 unification`.

### M2. Add explicit 23-vs-25 corpus-audit pre-flight to §9.0

§1.1 reconciles the 2 out-of-scope `vase.(add.((replica.` hits in boss blocks
inside prose; §9.0 should have the concrete pre-flight greps so the invariant
is checkable at implementation start. Add to §9.0:

```bash
rg -nFc 'vase.(add.((replica.' working-mods/sliceymon.txt     # expect 25
# Of those 25, exactly 23 must be inside itempool modifiers:
python3 -c "
import re
text = open('working-mods/sliceymon.txt').read()
# Count only vase.(add.((replica. occurrences whose enclosing modifier starts with 'itempool.'
# ...concrete helper logic embedded in the example
" 2>/dev/null || true
rg -nFc 'ph.b.' working-mods/sliceymon.txt                    # boss-block lines carry the 2 out-of-scope hits
# Gate: if (total != 25) OR (in-itempool != 23) OR (in-boss-blocks != 2), halt and re-audit.
```

### M3. Collapse Chunk 5 conditional — PR #12 merged, `generate_hero_item_pool` still absent

Verified 2026-04-24:
- Chunk 5 is merged (`975da96`: `Merge pull request #12 from hgorelick/feat/chunk-5-merge-derived-strip`).
- `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` → **0 hits**.
  PR #12 shipped other Chunk-5 work (provenance-gated strip, `merge_with_overlay`)
  but did NOT include the function.

The master plan and 8A §3.8 both wrote conditional branches for "is
`generate_hero_item_pool` in-tree?" This is now a deterministic state, not a
conditional. Rewrite:

- **Master §2 Pre-conditions.** Replace the "Chunk 5 must land on `main` before
  Chunk 8 starts; NOT yet in-tree as of Round 2 audit" paragraph with: "Chunk 5
  is merged (PR #12, commit `975da96`). PR #12 did NOT include
  `fn generate_hero_item_pool`. 8A authors the function against the new
  trigger-based IR shape; no migration step is required because there is no
  prior-shape implementation to migrate. The §9.0 pre-condition grep
  `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs` should
  return 0 at 8A start, ≥1 at 8A completion."
- **Master §4 / §9.0.** Remove the "Chunk 5 migration patch" language from the
  `builder/derived.rs` row. Replace with: "8A authors `fn generate_hero_item_pool`
  from scratch against the trigger-based IR shape (`matches!(item.trigger,
  SummonTrigger::SideUse { .. })` routes into the hero pool keyed by the
  summoned-entity name; `Cast` is skipped)."
- **8A fix A9** (below) mirrors this: delete §3.8's conditional branch, replace
  with unconditional "authoring from scratch" scope.

### M4. Require source-vs-IR divergence tests (T30b/T30c), not just round-trip

Current §5 T30 proves `extract → emit` idempotence on raw bytes. It does not
prove the emitter reads typed fields. Add to §5 a required cohort for 8B:

- **T30b**: extract a non-summon entry with `.tier.3`; mutate `ir.items[i].tier
  = Some(7)`; rebuild; assert output contains `.tier.7` (proves emitter reads
  typed field, not cached source bytes).
- **T30c**: extract a Cast summon; mutate `ir.replica_items[i].target_pokemon
  = "Synthetic"`; rebuild; assert output contains `.n.Synthetic.` and does NOT
  contain the original Pokemon name.

### M5. Add Finding-shape widening audit to §3.5

Before `Finding` gains `buckets` / `includes_boss`, a distinct audit commit
must enumerate every construction site, confirm each uses `..Default::default()`,
and round-trip a sample through serde/schemars.

Add to §3.5 (Finding shape widening section): "Audit step (new commit before
the widening): enumerate the 16 `Finding {` sites (13 in xref.rs, 1 each in
lib.rs, ir/merge.rs, build_options_tests.rs). Confirm each uses
`..Default::default()`. Assert `serde_json::to_string + from_str` round-trips a
Finding with populated `buckets` and `includes_boss`. Assert `schemars` emits
the new fields with `default` + `skip_serializing_if`."

### M6. pansaer line-number citations: replace with grep anchors

Audit every `pansaer line N` reference in master. Each must become a verbatim
grep anchor. Sites (to grep at edit time, not hard-coded here):

```bash
rg -n 'pansaer line' plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md
```

For each hit, replace with `pansaer (grep anchor: \`rg -nF '<verbatim_substring>'
working-mods/pansaer.txt\`)`.

---

## Fixes — `plans/CHUNK_8A_REPLICA_IR_ATOMIC_REWRITE.md`

### A1. Embed DiceLocation corpus grep evidence inline in §2 or §3.2

Plan asserts `OuterPreface = 18`, `InnerWrapper = 1` without inline grep
evidence. Add to §2 pre-conditions:

```bash
# Corpus evidence for new enum variants (both must pass at implementation start):
rg -nFo 'hat.replica.Thief.n.' working-mods/sliceymon.txt | wc -l            # expect 18 (OuterPreface base count; exclude boss-block hits)
rg -nF 'hat.(replica.Thief.i.(all.(left.hat.Thief.sd.' working-mods/sliceymon.txt | wc -l  # expect 1 (InnerWrapper — Master Ball?)
```

If either count diverges, the enum variant set must be widened or narrowed
before the atomic rewrite ships.

### A2. Explicit decision on nested `hat.egg` in Red Orb (Groudon)

Sliceymon line 117 has a nested `hat.egg.(wolf.n.Geyser...)` inside the outer
`hat.egg.dragon.n.Groudon`. The new `ReplicaItem.enemy_template: String` holds
only the outer template.

**Decision:** 8A ships the stub unchanged; 8B cannot emit `Summon(i)` for any
entry whose body contains a nested `hat.egg.(` until the schema is widened.
Add to 8A §2:

```bash
# 8B gate (documented here; enforced in 8B's pre-conditions):
rg -nF 'hat.egg.(wolf' working-mods/sliceymon.txt | wc -l                    # expect ≥1 (Red Orb latent case)
# If 8B starts with this > 0 and ReplicaItem has no nested-egg field, 8B must
# widen the schema (new commit) before real extraction.
```

Add the same gate to 8B's §2 (see fix B4).

### A3. Reframe `abilitydata` retirement in §3.2 doc-comment

Plan §3.2 doc-comment says "No top-level `abilitydata` field" — reads like
design. Rewrite as explicit migration: "Field `abilitydata: Option<AbilityData>`
is RETIRED in 8a (see §3.1). Current IR carries it; new IR does not. Rationale:
the 4 corpus `cast.sthief.abilitydata` bodies have zero depth-0
`.n.<spell_name>`, so `AbilityData` (non-Optional `name: String`) does not fit
the cast shape."

### A4. Declare file-count exception upfront in §0

Plan touches ~15 files. `ai-development.md` rule: ≤5 files per chunk.
Legitimate exception (atomic IR rewrite — partial commits don't compile), but
must be declared. Add to §0:

"**File-count exception (declared up front):** this chunk touches approximately
15 files across `compiler/src/{ir,extractor,builder,xref,ops}` plus tests. The
≤5 file rule is waived because the IR field rewrite (`name → target_pokemon`,
`template → trigger`, field retirements) breaks every callsite simultaneously;
no partial split compiles. The 'no parallel types' principle (CLAUDE.md) takes
precedence over the file-count soft limit."

### A5. Define T2a in 8A, don't defer to 8B

Plan §8 lists T2a (case-preservation round-trip) as out-of-scope but refers to
it as authoritative elsewhere. Define it concretely in 8A §5 as a *spec*
(8B authors the implementation, but the test contract lives in 8A):

- **T2a (spec; 8B implements):** Extract the Red Orb entry; assert `trigger =
  Cast { dice }`, `enemy_template == "dragon"` (lowercase preserved). Emit;
  assert output byte-equals input for the template substring. Repeat with a
  synthetic entry `hat.egg.Dragon.n.Capitalized` and assert
  `enemy_template == "Dragon"` (capital preserved). Proves no registry lookup,
  no normalization.

### A6. Add post-removal bounds-check to §3.7 re-index routine

The `retain_mut` loop shifts `Summon(i)` indices without validating the
resulting indices remain in bounds. Add an assertion after the loop:

```rust
// After re-index pass: no Summon(i) points out of bounds.
for structural in self.structurals.iter() {
    if let StructuralContent::ItemPool { items, .. } = &structural.content {
        for item in items {
            if let ItempoolItem::Summon(i) = item {
                if *i >= self.replica_items.len() {
                    return Err(CompilerError::internal(format!(
                        "ReplicaItem index {} out of bounds after removal (len={})",
                        i, self.replica_items.len())));
                }
            }
        }
    }
}
```

Also confirms CLAUDE.md "no new panics" invariant.

### A7. Clarify `sticker_stack` type change is non-reversible

Plan §3.3 treats the `sticker: Option<String>` → `sticker_stack:
Option<ModifierChain>` rollback as "strictly smaller than 8a's atomic commit."
Per SPEC §3.7 "no parallel types," the rollback is *not* feasible — it requires
reverting 8a entirely.

Rewrite §3.3 sticker bullet: "8B's T2b is a GATE, not a post-hoc. If T2b
cannot round-trip both `Statue` and `statue` forms byte-equal, the
`ModifierChain` type is inadequate and 8B must design an alternative
(custom `StickerChain` newtype, or manual case-preserving segment parser)
in 8B's same commit. 8a's type change cannot be rolled back piecewise."

### A8. Bucket-label asymmetry — clarify that two sites exist

Plan §9 correctly says two sites. One reviewer round was wrong here; the plan
is correct. Keep the text but attach the verified anchor:

```bash
# Verified this session:
rg -nF '"legendary"' compiler/src/xref.rs   # xref.rs:205 (X003/check_duplicate_pokemon_buckets)
rg -nF '"replica_item"' compiler/src/xref.rs  # xref.rs:500 (V020/check_cross_category_names)
```

### A9. Collapse §3.8 conditional — author `generate_hero_item_pool` unconditionally

Verified: Chunk 5 PR #12 merged (`975da96`), function `fn
generate_hero_item_pool` absent from `compiler/src/builder/derived.rs` (grep
returned 0 hits). The §3.8 conditional ("≥1 hit → migrate; 0 hits → author")
collapses to the 0-hit branch.

Rewrite 8A §3.8: delete the "if ≥1 hit" migration branch and its prose.
Replace with a single unconditional subsection titled **"§3.8 Author
`generate_hero_item_pool` against the trigger-based IR shape"**. Content:

- Walk `ir.replica_items`; for each where `matches!(item.trigger,
  SummonTrigger::SideUse { .. })` (both `OuterPreface` and `InnerWrapper`
  sub-variants route into the hero pool — `dice_location` is a source-shape
  discriminator, not a game-mechanic axis); emit the hero-bound `itempool.`
  modifier keyed on the summoned-entity field (see fix A11 for the field name).
  `Cast` entries are skipped (they have their own emission path per §3.4).
- Signature matches the foundations-plan spec at
  `plans/PLATFORM_FOUNDATIONS_PLAN.md` (grep: `rg -n 'generate_hero_item_pool'
  plans/PLATFORM_FOUNDATIONS_PLAN.md`).
- Test: `derived::hero_item_pool_matches_sliceymon_via_trigger` — byte-match
  sliceymon's hero-bound ItemPool source bytes.

Also update 8A §2: replace the Chunk 5 halt-condition grep with a one-line
status check: `rg -c 'fn generate_hero_item_pool' compiler/src/builder/derived.rs
# expect 0 at 8A start, ≥1 at 8A completion`.

### A10. pansaer line-number citations (cross-cutting M6)

Audit 8A for `pansaer line N` references; replace with grep anchors.

---

## Fixes — `plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md`

### A5-1. Resolve V8/V15 prose-vs-schema contradiction

§3.1 prose says "no separate Keyword variant in the final schema" yet the
`CompositionComponent` enum declares `Keyword { … }` and `Learn { … }`. The
collapse is at the **`NonSummonEntry` top level only** — these are still
valid component types.

Rewrite the §3.1 V8/V15 commentary (lines ~181–187): "V8 and V15 collapse at
the `NonSummonEntry` level (no top-level `NonSummonEntry::Keyword` or
`::Learn` variant). Within a `Composition`, `CompositionComponent::Keyword`
and `CompositionComponent::Learn` ARE declared — they are the sub-component
types that represent the bare-head case when `components.len() == 1`.
Design decision D-D formalizes the entry-level collapse, not a component-level
deletion."

### A5-2. Rename T30.9 / T30.13 / T30.15 to reflect component/sub-pattern testing

These tests don't test distinct top-level variants; they test Composition
sub-patterns and `OuterWrap` variants. Rename:

- `T30.9 keyword_composition_destiny` → `T30.9a composition_single_keyword_component_destiny`
- `T30.13 double_paren_rube_goldberg_machine` → `T30.13a outer_wrap_double_paren_rube_goldberg`
- `T30.15 learn_spell_learn_poke` → `T30.15a composition_single_learn_component_poke` (if V15 survives verification — see A5-6)

Update the per-test description to say "exercises `Composition` with a
single `CompositionComponent::Keyword` component" (9a) / "exercises
`OuterWrap::DoubleParen` with trailer outside inner paren" (13a) / "exercises
`Composition` with a single `CompositionComponent::Learn` component" (15a).

### A5-3. Resolve V12 entry-vs-modifier conflation

V12's sliceymon instance (`itempool.Void.part.0.mn.Clear Itempool`) is an
**outer modifier with zero items**, not an itempool entry. §3.3's rule
"every non-summon entry carries a `.n.<name>…` trailer" is inconsistent with
V12 as listed.

Rewrite §1.3 V12: "V12 — DELETED as an `ItempoolItem` variant. Bare-sentinel
itempool modifiers (`itempool.Void.part.0.mn.Clear Itempool`) are not entries
— they are the outer `StructuralContent::ItemPool { items: Vec<…>, outer_name:
Option<String> }` record with `items: vec![]` and `outer_name: Some(\"Clear
Itempool\")`. When a sentinel (`Void`, `uy`) appears *composed* with other
content inside an entry, it classifies as `Composition` with a
`CompositionComponent::Sentinel { token }` component."

Add to §3.1 after the `ItempoolItem` enum: a clarifying note that the
`StructuralContent::ItemPool` (authored by 8A) carries the outer-name case via
`outer_name: Option<String>` with empty `items`.

### A5-4. Verify `OuterWrap::DoubleParenTrailerInside` has corpus evidence or delete it

Plan declares two variants `DoubleParen` and `DoubleParenTrailerInside`. Comment
on `DoubleParen` says trailer "MAY sit inside or outside." Both cannot be true.

Add pre-implementation corpus audit to §2:

```bash
# DoubleParen — trailer OUTSIDE innermost paren (inside outer): expect ≥1 hit
rg -n '\(\([^)]*\)\)\.n\.' working-mods/*.txt | wc -l                       # expect ≥1
# DoubleParenTrailerInside — trailer INSIDE innermost paren: corpus grep
rg -n '\(\([^)]*\.n\.[^)]*\)\)' working-mods/*.txt | wc -l                  # if 0, DELETE the variant
```

Rule: if the second grep returns 0, delete `DoubleParenTrailerInside` from
`OuterWrap` and update the `DoubleParen` doc-comment to drop the "MAY sit
inside or outside" language.

### A5-5. `Unclassified` escape-hatch policy — explicit ship gate

Plan ships `NonSummonEntry::Unclassified { source_bytes: String }` permanently.
Multiple reviewers flagged this as SPEC §3.2 raw-passthrough.

**Decision:** Keep the variant, but ship with a **strict ship gate**: T30.0
asserts zero `Unclassified` entries across all four working mods; the variant
is allowed to *exist* as a SPEC §3.3 permissive-extract hatch but must never
fire on corpus bytes. Add to §2:

```bash
# Ship gate (runs in CI, not just at implementation start):
cargo test --test roundtrip_baseline working_mods_produce_zero_unclassified_entries  # must pass
```

And add to §3.5: "If the gate fails on a future corpus, the variant set **must
be widened in the same PR** (not a follow-up). The hatch exists so the
extractor does not panic on a novel shape, not so novel shapes ship
unclassified."

### A5-6. V15 evidence gap — Phase-A verification or deletion

Plan cites `learn.Poke` and `learn.Bandage` in pansaer pool 3; grep did not
find them in corpus head output. pansaer monoline prevents naive line lookup,
so this is an evidence gap, not a definitive absence.

Add to §2 pre-conditions:

```bash
# V15 evidence verification (hook rule #2: zero corpus instances → delete variant)
rg -noF 'learn.Poke' working-mods/pansaer.txt | wc -l                        # expect ≥1
rg -noF 'learn.Bandage' working-mods/pansaer.txt | wc -l                     # expect ≥1
rg -noF 'learn.' working-mods/*.txt | wc -l                                  # if total is 0, V15 is hypothesis — delete variant and T30.15a
```

Rule: if no `learn.<name>` shape exists in the corpus, delete V15 from §1.3,
delete T30.15a, and remove `CompositionComponent::Learn` from §3.1. Do this
BEFORE any 8A.5 code is authored.

### A5-7. Delete `Other(String)` escape hatches in `AllitemScope` and `ScopeSpec`

Plan declares `Other(String)` variants with a comment "widen in same commit
if reached." This contradicts the atomic-rewrite principle — either the
Phase-A walk is exhaustive (variants are dead code) or the variants are raw-string
escape hatches (violate SPEC §3.2).

Rewrite §3.1: delete `Other(String)` from both `AllitemScope` and `ScopeSpec`
enums. Add to §3.2.1 Phase-A walk: "If the walk surfaces a scope token not
enumerated in `AllitemScope` or `ScopeSpec`, halt and extend the enum in the
same PR before any classifier code is authored. An exhaustive walk is a
prerequisite for the ship."

### A5-8. Specify Phase-A walker output format concretely

Plan names `target/itempool-shape-audit.txt` but gives no format. Add to
§3.2.1:

```text
For each distinct fingerprint:
  FINGERPRINT: <salient-token-list, order-preserved>
  COUNT: <N>
  VARIANT_MAPPING: <V-N label> | UNMAPPED
  SAMPLES:
    1. <120-char verbatim entry preview>
    2. <120-char verbatim entry preview>
    3. <120-char verbatim entry preview>
  ---

If any fingerprint has VARIANT_MAPPING: UNMAPPED, halt. Extend §1.3 with a
new V-N entry (corpus instance + description) and the IR schema in §3.1,
THEN re-run the walk. Loop until zero UNMAPPED.
```

### A5-9. Document `Sentinel` component in §1.3 prose

`CompositionComponent::Sentinel { token }` appears in §3.1 but is never
explained in §1.3's V-variant prose. Add a "Component types" sub-subsection
before V1 listing every `CompositionComponent` variant and which V-N shapes
use each.

### A5-10. Per-test synthetic-pair spec in §5.1

§5.1 says each T30.N has a "synthetic pair" proving source-vs-IR divergence
but doesn't specify the pairs. Add to each T30.N description the exact
synthetic pair.

Example for T30.1:
> **T30.1** `bare_base_game_ref_amnesia` — corpus: pansaer's `Amnesia` (grep
> anchor: `rg -noF 'Amnesia' working-mods/pansaer.txt`). **Synthetic**:
> `NotABaseGameItem` between two `+` delimiters, no wrapper, no suffixes. Both
> must parse to `NonSummonEntry::BareBaseGameRef { name }` and emit byte-equal
> on round-trip. Proves classifier does not reach for a base-game item
> registry.

Repeat per test. (This is content-authoring work inside the plan file; the
full rewrites go in the implementation pass.)

### A5-11. pansaer line-number citations (cross-cutting M6)

Replace every `pansaer line N` or `pansaer pool N` citation with a grep anchor
(`rg -nF '<verbatim>' working-mods/pansaer.txt`) or an explicit
Phase-A-walk deferral. Same applies to cited `community.txt:N` sites whose line
numbers were not verified this session (V3, V4 extras, V7, V9, V10, V11, V14)
— either verify now or replace with grep anchors.

---

## Fixes — `plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md`

### B1. Resolve bucket-label rename coordination with 8A

Plan §3.4 edit table has a conditional "After 8a: also swap `item.name` →
`item.target_pokemon`". This creates an impossible task if 8B ships before 8A.

**Decision:** 8B cannot start until 8A lands (already declared in §2 via the
3-hit grep for `SummonTrigger`/`DiceLocation`/`ItempoolItem`). Remove the
conditional branch; state the post-8A-landing state as the single target.

Rewrite §3.4 table row:

```
| Site                                                       | Change                                                     |
| `.push((item.name.clone(), "legendary"))` at xref.rs:205  | Rename label to `"replica_item"` AND access field          |
|                                                            | via `item.target_pokemon.clone()` (8A renamed `name`).     |
| `.push((item.name.clone(), "replica_item"))` at xref.rs:500 | Swap field access to `item.target_pokemon.clone()` only;  |
|                                                            | label already canonical.                                   |
```

### B2. Move X003 prose-scan assertions to typed `buckets` field

Three confirmed X003 test assertions at xref.rs:963–965 scan message prose
(`hero`, `legendary`). They must migrate to typed `buckets` field. Add to §3.6:

```
Edit xref.rs:963–965 in the same commit that widens `Finding`:

  - assert!(x003[0].message.contains("Pikachu"));       // keep (asserts entity name prose)
  - assert!(x003[0].message.contains("hero"));          // DELETE
  - assert!(x003[0].message.contains("legendary"));     // DELETE
  + assert_eq!(
  +     x003[0].buckets.as_slice(),
  +     &["hero", "replica_item"]
  + );
```

Also audit all 10 `message.contains` sites in xref.rs (lines 848, 897, 963,
964, 965, 1070, 1111, 1112, 1231, 1232, 1280) and migrate every bucket-label
substring match to typed `buckets`. Entity-name substring assertions (e.g.,
`"Pikachu"`, `"Goblin"`, `"9999"`) may stay — they are the Finding's payload,
not a set-label test.

### B3. Demote `W-REPLICA-TRIGGER-UNCLASSIFIED` to conditional backlog

Plan introduces this Finding code for "conjunctive pair present + unknown
wrapper shape" — zero corpus instances found. Hook rule #3: zero instances →
hypothesis → don't ship.

Rewrite §3.1 "Finding on unclassifiable wrapper shape" bullet:

> "**BACKLOG (not 8B scope):** parent §3.3 rule 2(d) sketches
> `W-REPLICA-TRIGGER-UNCLASSIFIED` for conjunctive pair + unknown wrapper.
> Corpus has zero such instances (verified via `rg` in pre-conditions). Do
> NOT wire this Finding code until a corpus entry exhibits the shape. Remove
> the row from §3.1's failure-mode matrix. If 8B's tests somehow surface the
> shape during implementation, halt and author a new chunk; do not smuggle
> the code in under 8B."

Corresponding matrix-row deletion is implied.

### B4. Nested `hat.egg` gate (inherits from 8A A2)

Add to §2 pre-conditions:

```bash
# Red Orb latent case — if the corpus has nested hat.egg inside an outer
# hat.egg, 8B's real extractor cannot emit Summon(i) for it until the
# ReplicaItem schema carries a nested-egg field. Halt if hits > 0 and
# schema is not widened.
rg -nF 'hat.egg.(wolf' working-mods/sliceymon.txt | wc -l                    # expect ≥1 (Red Orb)
rg -n 'pub enemy_template.*String' compiler/src/ir/mod.rs                    # confirms 8A's shape
# Gate: if Red Orb count ≥1 AND ReplicaItem has no nested-egg field, 8B must
# widen the schema in the same commit before real extraction.
```

### B5. Add structural-helper enforcement for `detect_summon_pair`

§3.1 names `detect_summon_pair` as a private helper to avoid N-line
incantation across three trigger branches. Nothing enforces that the three
branches actually call it.

Add to §8 self-verification:

```bash
rg -c 'detect_summon_pair' compiler/src/extractor/replica_item_parser.rs  # expect ≥3
# (one call per branch: Cast, SideUse/OuterPreface, SideUse/InnerWrapper)
# If <3, the helper was inlined and structural smell persists.
```

### B6. Encode X003/V020 co-fire invariant in T20/T21

T20/T21 as written accept any co-fire. The invariant is specific. Rewrite:

```rust
// T20/T21 co-fire invariant:
// X003 fires iff collision includes ≥2 of {hero, replica_item, monster}.
// V020 fires iff collision includes ≥2 of {hero, replica_item, monster, boss}.
// Both fire iff collision spans ≥1 Pokemon bucket AND boss bucket.
if !x003.is_empty() && !v020.is_empty() {
    for f in &x003 {
        assert!(f.buckets.iter().all(|b|
            ["hero", "replica_item", "monster"].contains(b)));
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

### B7. Strengthen T10 — source-vs-IR divergence with trigger-misleading container names

Current T10 swaps container + payload and asserts payload wins. A hybrid
heuristic "if container contains 'Ball' prefer SideUse, else payload" could
still pass. Rewrite T10:

- Synthetic A: container `"Thief Ball"` (contains `Ball`) with a Cast-shape
  payload (`cast.sthief.abilitydata…`). Assert classifier returns `Cast`,
  not `SideUse`.
- Synthetic B: container `"Cast Item"` (contains `Cast`) with a SideUse-shape
  payload (`hat.replica.Thief.n.<Pokemon>.sd…`). Assert classifier returns
  `SideUse { OuterPreface }`, not `Cast`.

These actively falsify both "name-substring wins" and "name-substring is a
fallback" heuristics.

### B8. Exhaustive failure-mode matrix test

§3.1 enumerates a 5-row matrix of (entry shape → prefilter result → NonSummon
result → emitted ItempoolItem → Finding). No single test walks every row.
Add:

```rust
#[test]
fn failure_mode_matrix_exhaustive() {
    // Rows from §3.1:
    //   1. conjunctive pair + recognized wrapper → Summon(i) + ReplicaItem, no Finding
    //   2. (removed per B3: W-REPLICA-TRIGGER-UNCLASSIFIED is BACKLOG)
    //   3. no conjunctive pair → NonSummon(…), no Finding
    //   4. no pair + Phase-A UNMAPPED → Unclassified + W-REPLICA-NONSUMMON-UNCLASSIFIED
    //   5. Cast with missing inner dice → Err(CompilerError::classify)
    let cases = matrix_cases();  // (entry_bytes, expected_item, expected_findings)
    for (i, (entry, expected)) in cases.iter().enumerate() {
        let got = extract_from_itempool(entry, /* indices */ 0, 0);
        assert_eq!(got, *expected, "matrix row {} mismatch", i + 1);
    }
}
```

### B9. Remove redundant "Files explicitly NOT in 8B's list" prose

§4's explicit file-exclusion list duplicates the dependency-chain narrative.
Collapse to one sentence: "8A ships the IR schema and authoring builders; 8A.5
ships the NonSummonEntry typed schema, emitter dispatch, and the
`NonSummon(NonSummonEntry)` retype. 8B does not edit any file owned by those
chunks."

### B10. Finding construction audit — verify 16, not 17

Plan §3.3 says "all 16 construction sites must be audited." Verified count:
**16** (13 in xref.rs + 1 each in lib.rs:84, ir/merge.rs:79,
build_options_tests.rs:277). Confirmed correct. Keep as-is; add anchor:

```bash
grep -rn 'Finding {' compiler/src/ compiler/tests/ | wc -l                  # expect 16
```

---

## Fixes — `plans/CHUNK_8C_PROSE_SPEC_HOOKS.md`

### C1. Fix pre-condition #1 grep — exclude comments

Current grep: `rg -c 'ReplicaItemContainer' compiler/src/  # expect 0`
Actual state: 1 hit (comment at xref.rs:946).

Replace with definition-only grep:

```bash
rg -c 'pub enum ReplicaItemContainer|ReplicaItemContainer::' compiler/src/   # expect 0 (definitions / usages only; comments permitted)
```

### C2. Fix pre-condition #1 grep — `ModifierType::Legendary` handling

Current grep: `rg -c 'ModifierType::(Legendary|ReplicaItem(WithAbility)?)\b' compiler/src/  # expect 0`
Actual state: `ModifierType::Legendary` is still present at classifier.rs:18
(Chunk 9 did not retire it; that was 8A's responsibility).

Since 8C is planned to run **after** 8A lands, this pre-condition is correct
as a forward-looking gate. Clarify in §2: "This grep returns 0 only after 8A
has shipped (which retires `ModifierType::Legendary`). Before 8A, this gate
will fail by design; 8C cannot start pre-8A."

### C3. Add `cwd = compiler/` directive to §2

Several §2 pre-conditions invoke `cargo run --example roundtrip_diag`. Cargo
requires the compiler/ directory. Add to §2 opening:

> "All cargo commands below assume `cwd = compiler/`. From repo root: `cd
> compiler && <cmd>`."

### C4. Rewrite Edit #7 to match parent §3.6 authority

Current Edit #7 describes ReplicaItem as "persistent ally (Legendary) with a
cast spell" and cites a non-existent `item.legendary.(…)` shape. This
contradicts parent §3.6 and the shipped Chunk-9 IR.

Replace Edit #7's "Replacement:" block with the parent §3.6 verbatim text:

> `**ReplicaItem** — IR type for items that summon a Pokemon as a monster; on
> defeat, the Pokemon joins the team for the rest of the run. Every ReplicaItem
> comes from an entry inside an itempool.((...)) modifier and shares the same
> summon → defeat → team-join pipeline. Discriminated by `SummonTrigger`:
> **SideUse** (player uses a thief-side; dice live either on an outer flat
> preface — e.g. Poke Ball — or inside the wrapper — e.g. Master Ball? —
> captured by the `dice_location` sub-discriminator) and **Cast** (equipping
> grants a spell via cast.sthief.abilitydata; e.g. Silver Wing). All trigger
> variants carry identical summon/defeat/team-join payload; the discriminator
> sits on SummonTrigger, not on a container-position enum.`

Replace Edit #7's "Rationale:" with: "Parent §3.6 prescribes this rewrite
verbatim. Matches the post-8A IR (`ReplicaItem { trigger: SummonTrigger, ...
}`)."

### C5. Edit #1 token — keep `replica_item`, align rationale

Parent §3.6 derived/downstream mentions specify `replica_item` for the
classifier ASCII retargeting (line 78). The prose is naming the **IR bucket**,
not the classifier type. One reviewer proposed `itempool` — that's the
classifier type, not the IR bucket, and it drifts from canonical-set rules.

Keep Edit #1's replacement `classifier → type parsers (hero/replica_item/monster/`.
Rewrite rationale to drop the compiler-internals gloss:

> "Parent §3.6 retargets the `capture` token in the SPEC classifier ASCII
> diagram to the canonical IR bucket label `replica_item` — aligning
> user-facing prose with the X003/V020 canonical-set. This edit does not
> describe classifier internals; it describes the bucket name visible to spec
> readers."

### C6. Reconcile line-number targets — 246 vs 254, 342/350

SPEC lines 246 AND 254 both contain `(captures / legendaries)`. Parent §3.6
targets line 246; 8C §2 targets line 254. Lines 342, 343, 350, 351 contain
overlapping/duplicate ReplicaItem glossary.

**Decision:** at implementation start, re-grep SPEC for every verbatim
replacement target and list exact line numbers. 8C plan lists what *should* be
targeted (both 246 and 254 if both contain the parenthetical; both glossary
entries if duplicated). Update §2 pre-conditions to:

```bash
rg -nF 'replica items (captures / legendaries)' SPEC.md                      # report all line numbers
rg -nF 'IR type for items that summon a Pokemon as a unit' SPEC.md           # report all line numbers
rg -nF '(captures, legendaries)' SPEC.md                                     # report all line numbers
```

And update §3.1 edits to say "Apply this replacement at **every** line the
grep returns, not just line N."

If duplicated glossary lines 342 vs 350 etc. are a SPEC.md authoring defect
(not intentional duplication), fix the deduplication in the same 8C commit
and note it in the plan as scope-in.

### C7. Rewrite authority-contract framing in §1

Current: "this plan is strictly narrower than the parent's §3.6/§3.7."
Reality: 8C executes the parent's mandate; Chunk 9 retiring IR work in
advance happens to narrow the execution, but the claim of "narrower" is
misleading when the plan is the designated delivery vehicle.

Rewrite §1 opening to:

> "This plan executes the parent's §3.6/§3.7 SPEC and plan-prose amendments.
> Chunk 9 (landed 2026-04-23) retired the IR-shape work the parent originally
> scoped to Chunks 8A/8A.5, leaving only prose. 8C's scope is the prose
> delivery; every edit here (a) restates the parent's prose intent verbatim,
> (b) targets the shipped Legendary-only IR shape, or (c) references the
> parent by §. It introduces **no new field, enum variant, invariant, hook
> rule, or SPEC clause** beyond what the parent already fixed."

### C8. Clarify §3.3 settings.json upstream status (non-speculation)

Worktree has 2 PreToolUse hooks (Read, Edit|Write|NotebookEdit); repo-root has
1 (Read only). The upstreaming task is valid.

Update §3.3 opening: "**State (verified 2026-04-24):** repo-root
`.claude/settings.json` has 1 PreToolUse hook (`Read` matcher). Worktree-local
`.claude/settings.json` has 2 (`Read` + `Edit|Write|NotebookEdit`). The
optional task is to upstream the second hook."

---

## Critical files to modify

```
plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md   — fixes M1–M6 (6 edits)
plans/CHUNK_8A_REPLICA_IR_ATOMIC_REWRITE.md  — fixes A1–A10 (10 edits)
plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md    — fixes A5-1 through A5-11 (11 edits)
plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md     — fixes B1–B10 (10 edits)
plans/CHUNK_8C_PROSE_SPEC_HOOKS.md           — fixes C1–C8 (8 edits)
```

No compiler code is modified. The only tool invocations needed are `Edit`
(for targeted replacements) and `Write` (if a plan section is fully rewritten).

## Approach — execution order

1. **Apply non-interacting fixes across all five plans in parallel** (each
   fix is a local edit inside its plan file). Start with `8C` (smallest —
   8 edits, all prose), then `8B` (10 edits, mostly additions), then `8A5`
   (11 edits, some involve enum-level rewrites), `8A` (10 edits including
   §3.8 deletion), `8` master (6 edits).
2. **Verify the cross-cutting fix** (pansaer line-numbers → grep anchors)
   by final grep across all five plan files after edits land.
3. **Re-run `/plan-review`** on the updated plan set to confirm the tribunal
   clears.

## Verification

After edits land, run these checks from repo root:

```bash
# 1. Every cross-plan reference to "pansaer line N" is eliminated.
rg -n 'pansaer line [0-9]' plans/CHUNK_8*.md                                 # expect 0

# 2. Every ground-truth grep from the fixes above resolves.
rg -c 'pub enum SummonTrigger' compiler/src/ir/mod.rs                        # expect 0 (pre-8A) OR ≥1 (post-8A)
rg -nF '"legendary"' compiler/src/xref.rs                                    # xref.rs:205
rg -nF '"replica_item"' compiler/src/xref.rs                                 # xref.rs:500
rg -c 'ReplicaItemContainer' compiler/src/                                   # 1 (comment at xref.rs:946)
rg -c 'pub enum ReplicaItemContainer|ReplicaItemContainer::' compiler/src/   # 0 (the C1 refined grep)

# 3. Each plan's §2 pre-conditions list is internally consistent — run the
# plan's own pre-flight block and confirm it passes or clearly halts.

# 4. Re-run /plan-review with same personas (architecture, slice-and-dice-design,
# code-reviewer) on all five plans and confirm zero critical findings remain.
/plan-review plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md \
             plans/CHUNK_8A_REPLICA_IR_ATOMIC_REWRITE.md \
             plans/CHUNK_8A5_NONSUMMON_TYPED_SCHEMA.md \
             plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md \
             plans/CHUNK_8C_PROSE_SPEC_HOOKS.md
```

The second tribunal pass is the end-to-end test: a clean return (zero critical
findings) across all plans × personas is the exit criterion.

## Out of scope

- No compiler code edits. The plans are still un-executed; fixing them does
  not advance implementation.
- No changes to CLAUDE.md, SPEC.md, or reference/textmod_guide.md (those are
  touched only by the *executed* plans, not by the plan-fix pass).
- No changes to `.claude/settings.json` (hook configuration is 8C's optional
  task to execute later).
- No new Rust tests authored; test specs in plan fixes are prose authored
  inside the plan files, not compiled code.
