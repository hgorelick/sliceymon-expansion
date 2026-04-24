# Chunk 8c: Prose — SPEC + foundations-plan alignment (post-Chunk-9 Legendary-only shape)

**Parent plans**: §3.6 and §3.7 of `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md`. 8A (`plans/CHUNK_8A_REPLICA_IR_ATOMIC_REWRITE.md`) is the upstream gate for any IR-shape-sensitive prose; 8C does not ship IR edits.

**Authority contract (C7 fix)**: this plan **executes** the parent's §3.6/§3.7 SPEC
and plan-prose amendments. Chunk 9 (landed 2026-04-23) retired the IR-shape
work the parent originally scoped to Chunk 8A/8A.5, leaving only prose
delivery for this chunk. 8C's scope is that prose delivery; every edit here
(a) restates the parent's prose intent verbatim, (b) targets a shipped (or
about-to-ship) IR shape, or (c) references the parent by §. It introduces
**no new field, enum variant, invariant, hook rule, or SPEC clause** beyond
what the parent already fixed. The prior "strictly narrower than the parent"
framing was misleading — 8C is the parent's designated delivery vehicle for
§3.6/§3.7, not a narrowing of them.

**Prose-only**: 8C ships no Rust library edits under `compiler/src/` and no test files. An optional helper-line edit to `compiler/examples/roundtrip_diag.rs` is permitted as diagnostic-readability support; any `compiler/src/` change that surfaces while implementing 8C is a plan defect — fold it into 8A, do not smuggle it in here.

**Ships as a single one-shot sub-chunk**: SPEC.md prose updates + `PLATFORM_FOUNDATIONS_PLAN.md` normative-hit cleanups + optional `roundtrip_diag.rs` helper + optional upstreaming of the worktree-local `Edit|Write|NotebookEdit` hook. One PR, one `cargo test` + `cargo run --example roundtrip_diag` + `python3 -c 'import json; json.load(...)'` gate.

---

## 1. Overview

### 1.1 What this chunk does

Chunk 9 (landed on `main`, merge commit `6d00941`) deleted `ReplicaItemContainer`, the `Capture` variant, and the `parse_simple` / `parse_with_ability` / `emit_simple` / `emit_with_ability` paths per chunk-impl rule 3 (zero corpus instance for any Capture-shape `ReplicaItem`). `ReplicaItem` models Legendaries only; Captures route as `ItemPool` structurals at the classifier. Four `working-mods/*.txt` all report `Replicas ir1=0` via `cargo run --example roundtrip_diag` in this worktree — the Legendary-only shape is the landed state.

`SPEC.md` still describes `Capture` and `Legendary` as **kinds** of `ReplicaItem` (glossary lines 350-351, uniqueness-bucket parenthetical at line 254, emission-order parenthetical at line 104, lossless-types parenthetical at line 176, classifier ASCII at line 78, CRUD comment at line 196, Modifier glossary at line 343). The foundations plan has already been partly rewritten around the Chunk 9 state at §F7 (line 365) and the Chunk 6 block at line 720, but SPEC.md has not.

8C closes that SPEC drift. The deliverables are:

1. **`SPEC.md`** — parent §3.6. Drop `Capture / Legendary`-kind glossary entry; rewrite the `ReplicaItem` glossary entry to describe the shipped Legendary-only shape; drop the `(captures, legendaries)` / `(captures / legendaries)` / `(captures and legendaries)` parentheticals at lines 104, 176, 196, 254; retarget the classifier-ASCII `capture` token at line 78 to `replica_item`; retarget the Modifier-glossary `capture` token at line 343 to `replica_item`. Keep line 20's `capturables` (game-flavor survivor, parent-§3.6 exempted).
2. **`plans/PLATFORM_FOUNDATIONS_PLAN.md`** — parent §3.7. Chunk 9 already updated §F7, the Chunk 6 block, §F1, and R2; 8C confirms no normative-retired-type hit remains outside the historical "superseded by Chunk 9" references. Any surviving stale-normative hit (not wrapped in a "superseded" / "deleted in" reference) is rewritten to the Legendary-only shape.
3. **`.claude/settings.json`** (repo-root, *committed*) — optional upstreaming of the worktree-local `Edit|Write|NotebookEdit` evidence-rule hook (present in `.worktrees/chunk-8-replica-trigger-redesign/.claude/settings.json`, absent from the committed file at repo root). Keep or drop based on user preference; no new bullet or matcher is authored here.
4. **`compiler/examples/roundtrip_diag.rs`** — optional single-line helper. Diagnostic readability only (note: "Replicas are Legendary-only post-Chunk-9"). Prose-support, not behavioral; if the existing output is already readable, skip.

### 1.2 What this chunk does NOT do

- **No IR shape change.** 8A owns any future IR shape; Chunk 9 already shipped the Legendary-only `ReplicaItem`. 8C touching `compiler/src/ir/mod.rs` = plan defect.
- **No extractor/emitter/xref/ops edits.** 8C touching any file under `compiler/src/{extractor,builder,xref,ops}` = plan defect.
- **No new test file.** 8C is prose; `#[test]` additions belong to the chunk whose implementation they exercise.
- **No new `PreToolUse:Read` hook bullet.** The committed repo-root `.claude/settings.json` already carries six bullets in its `PreToolUse:Read` `additionalContext` string, including the corpus-grounding rule ("Every IR variant discriminator must have at least one corpus instance per variant before it ships…"). 8C does not author a seventh bullet, does not restructure the checklist, and does not reorder the existing six.
- **No SPEC §3.2 typed-item-schema / raw-passthrough work.** That lives in 8A.
- **No resurrection of trigger-based IR prose.** `SummonTrigger`, `DiceLocation`, and `ItempoolItem` do not exist on `main` today (0 hits under `compiler/src/ir/mod.rs` in this worktree). Any SPEC wording 8C writes must match the shipped Legendary-only shape, not a hypothetical trigger-based shape.

### 1.3 Checkpoint + parallel map

- **Total sub-chunks**: 1. No internal chunking.
- **Parallel with**: any compiler/src/ work in 8A / 8B that does not touch these prose files. Zero file overlap with `compiler/src/` or `compiler/tests/`.
- **Upstream (must be on `main`)**: Chunk 9 (already landed). 8A is **not** blocking for 8C.
- **Wall-clock**: single round.

---

## 2. Pre-conditions (verify before any 8C edit lands)

All must be true on the working tree at 8C start.

**Working-directory convention (C3 fix):** all `cargo …` commands below assume
`cwd = compiler/`. The repo-root directory has no `Cargo.toml`; invoking
`cargo run --example roundtrip_diag` from repo root fails. From repo root,
either `cd compiler && <cmd>` or pass `--manifest-path compiler/Cargo.toml`.
`rg` / `jq` / `python3` commands below use paths from repo root (so they work
unchanged regardless of `cwd`).

```bash
# 1. Chunk 9 retirements held — ReplicaItemContainer + Capture parsers must be gone under compiler/src/.
# NOTE (C1 fix): a plain `ReplicaItemContainer` substring grep returns 1 hit on main today —
# a comment at xref.rs:946 ("// The former `capture` bucket was removed along with `ReplicaItemContainer`").
# The invariant is "no *definitions or usages* survive", not "no mention of the word" — historical
# comments are durable traceability. Use a definition/usage-only grep so the gate doesn't false-positive
# on comment text. If comments are retired later, tighten this grep in the same PR.
rg -c 'pub enum ReplicaItemContainer|ReplicaItemContainer::' compiler/src/              # expect 0 (definitions / usages only; comments permitted)
rg -c 'parse_simple|parse_with_ability|emit_simple|emit_with_ability' compiler/src/     # expect 0
# NOTE (C2 fix): this gate is FORWARD-LOOKING — it returns 0 only AFTER 8A lands (which retires
# `ModifierType::Legendary`). As of 2026-04-24 this grep returns ≥1 (classifier.rs:18 still has the
# variant). 8C runs after 8A, so this pre-condition is valid at 8C start; running it pre-8A will
# fail by design. 8C cannot start pre-8A.
rg -c 'ModifierType::(Legendary|ReplicaItem(WithAbility)?)\b' compiler/src/             # expect 0 (Chunk 9 deleted ReplicaItem{,WithAbility}; 8A deletes Legendary)

# 2. Trigger-based types do NOT exist on main — any prose 8C writes must not cite them.
rg -n 'pub enum SummonTrigger|pub enum DiceLocation|pub enum ItempoolItem' compiler/src/ir/mod.rs  # expect 0

# 3. Replicas-Legendary-only state holds for all four mods.
cargo run --example roundtrip_diag 2>&1 | grep -c 'Status: ROUNDTRIP OK'                # expect 4
cargo run --example roundtrip_diag 2>&1 | grep -c 'Replicas  ir1=   0'                  # expect 4 (all four mods report 0 replicas)

# 4. SPEC.md drift sites — verbatim strings 8C targets.
# NOTE (C6 fix): line numbers below are VERIFIED this session (2026-04-24) against current SPEC.md
# via `rg -nF '<verbatim>' SPEC.md`. Implementation MUST re-run every `rg -nF` at impl start and
# apply each replacement at EVERY line number the grep returns — if a future SPEC edit duplicates
# a glossary line, the replacement must land at BOTH matches. If any grep returns 0 hits, SPEC has
# drifted further; reconcile before landing 8C (do NOT fall back to line-number targeting).
rg -Fn 'classifier → type parsers (hero/capture/monster/' SPEC.md                       # current: 1 hit at line 78
rg -Fn '→ heroes → items → replica items (captures, legendaries)' SPEC.md              # current: 1 hit at line 104
rg -Fn 'replica items (captures, legendaries), monsters, bosses' SPEC.md                # current: 1 hit at line 176
rg -Fn 'captures and legendaries are both ReplicaItem kinds' SPEC.md                    # current: 1 hit at line 196
rg -Fn 'replica items (captures / legendaries), monsters. CRUD operations enforce this' SPEC.md  # current: 1 hit at line 254 (the §6.3 uniqueness invariant). An earlier tribunal draft cited lines 246 AND 254 — verified this session, only 254 matches; earlier draft was wrong. If a future SPEC edit duplicates the parenthetical at another line, apply the edit at every match.
rg -Fn '(one hero, capture, monster, structural, etc.)' SPEC.md                         # current: 1 hit at line 343
rg -Fn '**Capturable / Legendary** — Kinds of' SPEC.md                                  # current: 1 hit at line 351 (glossary entry to delete)
rg -Fn 'IR type for items that summon a Pokemon as a unit. Two kinds' SPEC.md           # current: 1 hit at line 350. An earlier tribunal draft cited lines 342/343/350/351 as overlapping; verified this session, the ReplicaItem glossary entry appears exactly once. If a duplicate surfaces, it is a SPEC authoring defect — dedupe in the same 8C commit.

# 5. Foundations plan — Chunk 9 in-tree edits confirmed (no 8C rewrite needed in these blocks).
rg -c 'SUPERSEDED by Chunk 9' plans/PLATFORM_FOUNDATIONS_PLAN.md                        # expect >0
rg -c '§F7.*ReplicaItemContainer.*SUPERSEDED' plans/PLATFORM_FOUNDATIONS_PLAN.md        # expect >0

# 6. Committed repo-root hook — six PreToolUse:Read bullets already present.
python3 -c 'import json; json.load(open(".claude/settings.json"))' && echo 'settings JSON parses'   # must succeed
jq -r '.hooks.PreToolUse[0].hooks[0].command' ../../.claude/settings.json | grep -c 'corpus instance per variant'   # expect 1 (from the committed repo-root copy)
```

If any expected-≥1 count returns 0 or any expected-0 count returns ≥1, either Chunk 9 regressed or SPEC drifted further — reconcile before landing 8C.

---

## 3. Scope — what ships

### 3.1 SPEC.md edits

File: `SPEC.md`. Eight in-tree hits catalogued by verbatim quote (re-grep at impl time; line numbers drift). Every replacement is a narrowing of the parent's §3.6 "drop `Capture` / `Legendary` kind language where it describes the IR type; keep game-flavor vocabulary untouched" mandate, aligned to the Chunk 9 Legendary-only shape.

**Hits to rewrite** (each identified by a verbatim quote from the current `SPEC.md`):

1. **Classifier ASCII (current text: `classifier → type parsers (hero/capture/monster/`)**
   - Replacement: `classifier → type parsers (hero/replica_item/monster/`
   - Rationale (C5 fix): parent §3.6 retargets the `capture` token in the SPEC classifier ASCII diagram to the canonical IR bucket label `replica_item` — aligning user-facing prose with the X003/V020 canonical-set. This edit does not describe classifier internals (those returned `ModifierType::ReplicaItem*` historically and currently return `ModifierType::Legendary` pre-8A); it describes the bucket name visible to spec readers. `itempool` (the classifier type) would be the wrong token — it is the modifier wrapper, not the IR bucket.

2. **Emission-order block (current text: `→ heroes → items → replica items (captures, legendaries)`)**
   - Replacement: `→ heroes → items → replica items`
   - Rationale: the `(captures, legendaries)` parenthetical described a two-kind enum that no longer exists. Drop the parenthetical; leave `replica items` as the canonical name.

3. **Lossless-types list (current text: `heroes, replica items (captures, legendaries), monsters, bosses, structurals, chains, fights, phases, rewards, level scopes — all roundtrip through fields.`)**
   - Replacement: `heroes, replica items, monsters, bosses, structurals, chains, fights, phases, rewards, level scopes — all roundtrip through fields.`
   - Rationale: same drop; preserves the rest of the list verbatim.

4. **CRUD comment (current text: `// ... same for Monster, Boss, ReplicaItem (captures and legendaries are both ReplicaItem kinds)`)**
   - Replacement: `// ... same for Monster, Boss, ReplicaItem`
   - Rationale: post-Chunk-9 there is exactly one `ReplicaItem` shape (Legendary); the parenthetical described a vanished discriminator.

5. **§6.3 uniqueness invariant (current text: `A Pokemon may exist in **at most one** of: heroes, replica items (captures / legendaries), monsters. CRUD operations enforce this; the author cannot accidentally bypass it.`)**
   - Replacement: `A Pokemon may exist in **at most one** of: heroes, replica items, monsters. CRUD operations enforce this; the author cannot accidentally bypass it.`
   - Rationale: the load-bearing "at most one" invariant and its CRUD-enforcement sentence survive verbatim; only the IR-kinds parenthetical is dropped. The uniqueness bucket set `{hero, replica_item, monster}` is the canonical declaration that X003 references.

6. **Modifier glossary (current text: `**Modifier** — One comma-separated entry in a textmod (one hero, capture, monster, structural, etc.).`)**
   - Replacement: `**Modifier** — One comma-separated entry in a textmod (one hero, replica_item, monster, structural, etc.).`
   - Rationale: align glossary's example modifier-kind token set with the IR bucket labels.

7. **ReplicaItem glossary (current text: `**ReplicaItem** — IR type for items that summon a Pokemon as a unit. Two kinds: **Capture** (one-shot, mid-fight, via ball-style item) and **Legendary** (persistent ally with spell). Both share the same IR struct with a kind discriminant; "capturable" and "legendary" are *kinds*, not separate IR types.`)**
   - Replacement (C4 fix — verbatim from parent `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` §3.6): `**ReplicaItem** — IR type for items that summon a Pokemon as a monster; on defeat, the Pokemon joins the team for the rest of the run. Every ReplicaItem comes from an entry inside an itempool.((...)) modifier and shares the same summon → defeat → team-join pipeline. Discriminated by \`SummonTrigger\`: **SideUse** (player uses a thief-side; dice live either on an outer flat preface — e.g. Poke Ball — or inside the wrapper — e.g. Master Ball? — captured by the \`dice_location\` sub-discriminator) and **Cast** (equipping grants a spell via cast.sthief.abilitydata; e.g. Silver Wing). All trigger variants carry identical summon/defeat/team-join payload; the discriminator sits on SummonTrigger, not on a container-position enum.`
   - Rationale (C4 fix): parent §3.6 prescribes this rewrite verbatim. Matches the post-8A IR (`ReplicaItem { trigger: SummonTrigger, ... }`). The prior draft ("persistent ally (Legendary) with a cast spell" + citation of `item.legendary.(…)`) both (a) conflicted with parent §3.6 and (b) referenced a modifier shape that does not exist in the corpus — `item.legendary.(…)` appears zero times across `working-mods/*.txt`, so using it as the glossary anchor would entrench a retired-state view of the IR.
   - **Ordering note**: because the parent-§3.6 verbatim text names `SummonTrigger` / SideUse / Cast / dice_location — the post-8A shape — Edit #7 cannot land before 8A ships. 8C's overall ordering is post-8A; §1.3's "Upstream" clause must be tightened accordingly (see fix to §1.3 / §1.1 — Chunk 9 is necessary but NOT sufficient; 8A is also required).

8. **Capturable / Legendary glossary (current text: `- **Capturable / Legendary** — Kinds of \`ReplicaItem\` (see above). User-facing vocabulary only.`)**
   - Replacement: **delete the entire line**. The "kinds" framing is gone; "Capturable" survives as a game-flavor word elsewhere in SPEC (line 20, "capturables"), but no glossary entry is needed for a non-existent IR-type discriminator.

**Preserved verbatim (do not touch):**
- Line 20's `capturables` (`The Sliceymon+ expansion (~100 Pokemon authored as heroes / capturables / monsters / bosses) is the proving ground that exercises every feature.`). This is game-mechanic flavor vocabulary, parent §3.6-exempted.

**Structural smell check** (hook rule #3, checklist bullet 1): the ReplicaItem-glossary rewrite (edit #7) and the §6.3 uniqueness-invariant parenthetical drop (edit #5) cover **two different invariants** — the glossary entry narrows the IR type description; the §6.3 edit narrows the uniqueness-bucket prose. They are correctly left as **two edits**. The canonical set `{hero, replica_item, monster}` is named in edit #5 and referenced elsewhere by name — not re-listed (checklist bullet 2 "name the set once").

### 3.2 plans/PLATFORM_FOUNDATIONS_PLAN.md edits

File: `plans/PLATFORM_FOUNDATIONS_PLAN.md`. Chunk 9 already rewrote the load-bearing sites: §F7 now begins `### F7. ReplicaItemContainer enum — SUPERSEDED by Chunk 9 (2026-04-23)`; the Chunk 6 block at line 720 carries `⚠️  SUPERSEDED BY CHUNK 9 (2026-04-23)` in its header; R2 at line 902 has been rewritten to the Legendary-only ruling; §F1's `ReplicaItem` row at line 93 already reads `Post-Chunk-9, ReplicaItem models Legendaries only (no container field)`; and Final Verification lines 878 and 890 already describe the post-Chunk-9 state. 8C does **not** re-rewrite any of these.

8C's scope in this file is narrow: **confirm no stale-normative retired-type hit survives outside a "superseded" / "deleted in Chunk 9" / archived-traceability framing.** The exact retention contract:

- **Permitted**: any `ReplicaItemContainer` / `Capture { name }` / `ReplicaItemKind` mention inside a block already wrapped with `SUPERSEDED by Chunk 9`, `deleted in Chunk 9`, `archived`, or a strikethrough-struck `~~…~~` historical traceability bullet. These are durable history — Chunk 9's own §F7 (current text: `**Do not resurrect**: the chunk-impl rule is explicit. If a future corpus instance appears (e.g. if \`reference/textmod_guide.md\` grows a case that requires a typed Capture replica), model the new variant against that instance's actual shape — not this section's pre-chunk-9 design.`) is the template.

- **Forbidden**: any `ReplicaItemContainer` / `Capture { name }` / `ReplicaItemKind` mention that reads as a **live normative claim** about the current IR shape (e.g. a `Dependencies:` line claiming a Chunk depends on the enum; a code block defining `pub enum ReplicaItemContainer`; a table row asserting `ReplicaItem.container: ReplicaItemContainer` without a "pre-Chunk-9" or "archived" qualifier).

Re-grep at impl time:

```bash
rg -n 'ReplicaItemContainer|ReplicaItemKind' plans/PLATFORM_FOUNDATIONS_PLAN.md
```

For each hit, read the surrounding 5-10 lines and verify it sits inside a historical / superseded / archived framing. If any hit reads as live normative prose, rewrite it in-place to either (a) add a `(pre-Chunk-9; see §F7 header for current state)` qualifier, or (b) drop the retired-type token and substitute the Legendary-only description (template: R2 at line 902, or §F7 at line 369).

**Do not attempt:** a sweeping rewrite of every historical mention. Chunk 9's in-tree updates intentionally kept historical bullets (Chunk 6 block lines 731-737, 740-742, 860-867) with strikethrough + chunk-9-deletion attribution so future readers can reconstruct the decision chain. Stripping those would destroy traceability.

### 3.3 .claude/settings.json — repo-root vs worktree-local

**State (C8 fix — verified 2026-04-24 via `jq -r '.hooks.PreToolUse | length' <path>`):**
Repo-root `.claude/settings.json` has 1 `PreToolUse` hook entry (matcher `Read`).
Worktree-local `.claude/settings.json` has 2 `PreToolUse` hook entries
(matchers `Read` and `Edit|Write|NotebookEdit`). The optional 8C task is to
upstream the second (`Edit|Write|NotebookEdit`) hook to repo-root so every
worktree and every clone benefits from the evidence-rule enforcement, not
just this worktree. Details below preserved verbatim from the pre-C8 draft.

**Detailed state:**

- `/Users/hgorelick/Documents/slice-and-dice/.claude/settings.json` (the committed repo-root file) defines one `PreToolUse:Read` hook whose `additionalContext` string contains six bullets. The third bullet reads: `Every IR variant discriminator must have at least one corpus instance per variant before it ships. Grep working-mods for an instance of each variant. Zero instances for a variant means the variant is a hypothesis masquerading as a model — delete the variant rather than carry unevidenced cases. A rule (xref, V-rule, authoring gate) authored against an unevidenced variant compounds the defect.`

- `/Users/hgorelick/Documents/slice-and-dice/.worktrees/chunk-8-replica-trigger-redesign/.claude/settings.json` (the worktree-local copy) additionally defines a second `PreToolUse` entry with `matcher: "Edit|Write|NotebookEdit"` whose `additionalContext` string encodes the evidence rule (`Evidence rule (non-negotiable, applies to EVERY file in this project …)`).

The corpus-grounding rule 8C's original scope called for is **already landed** as bullet #3 of the committed hook. No bullet authoring is required.

**Optional 8C task**: upstream the worktree-local `Edit|Write|NotebookEdit` evidence-rule hook to the repo-root `.claude/settings.json` so every worktree and every clone benefits from the evidence-rule enforcement, not just this worktree. This is a single-insert edit — copy the worktree-local `matcher` + `hooks[0].command` block into the committed file's `hooks.PreToolUse` array. Verification:

```bash
python3 -c 'import json; json.load(open(".claude/settings.json"))' && echo 'settings JSON parses'
jq -r '.hooks.PreToolUse[1].matcher' .claude/settings.json   # expect "Edit|Write|NotebookEdit" if the upstreaming landed
jq -r '.hooks.PreToolUse[1].hooks[0].command' .claude/settings.json | grep -c 'Evidence rule'   # expect 1 if upstreaming landed
```

If the user prefers to keep the evidence-rule hook worktree-local (so it ships with the Chunk 8 branch and no other), drop this task entirely and 8C ships no `.claude/settings.json` edit.

### 3.4 compiler/examples/roundtrip_diag.rs — optional diagnostic helper

File: `compiler/examples/roundtrip_diag.rs`. The current `Replicas  ir1=... ir2=... delta=...` output line already surfaces the per-mod replica count (0 for all four mods today). 8C's optional edit is a one-line clarifying comment or a one-line supplementary `println!` noting "Replicas are Legendary-only post-Chunk-9" so a reader does not have to grep for context.

**If editing**: keep it single-line, prose-only, and unconditional (no match on IR content). Example shape:

```rust
    println!("    (Replicas are Legendary-only post-Chunk-9; Captures route as ItemPool structurals.)");
```

Placed immediately after the existing `Replicas  ir1=...` println. No `use` additions, no new enum matches, no behavioral change.

**If skipping**: 8C ships with no change to this file. The existing output is sufficient.

---

## 4. Files touched

| # | File | Change |
|---|---|---|
| 1 | `SPEC.md` | Per §3.1. Seven in-place rewrites (edits #1-#7) + one full-line deletion (edit #8). Line 20's `capturables` preserved verbatim. |
| 2 | `plans/PLATFORM_FOUNDATIONS_PLAN.md` | Per §3.2. Verify-only pass over `ReplicaItemContainer` / `ReplicaItemKind` / `Capture { name }` hits; rewrite any that read as live-normative (without "superseded" / "archived" framing). Expect 0 rewrites if Chunk 9's in-tree updates remain clean. |
| 3 | `.claude/settings.json` (repo-root, committed) | **Optional**. Upstream the worktree-local `Edit|Write|NotebookEdit` evidence-rule hook. If skipped, 0 edits. |
| 4 | `compiler/examples/roundtrip_diag.rs` | **Optional**. Single-line diagnostic helper noting the Legendary-only state. If skipped, 0 edits. |

**Size**: 1 mandatory file (SPEC.md) + 1 verify-only file (PLATFORM_FOUNDATIONS_PLAN.md, likely 0 edits) + 2 optional files. Well inside the AI-dev persona's guidance.

---

## 5. Enumerated verification checks

8C is prose-only; every check below is `rg` / `cargo run` / `python3 -c` / `jq`, not a `#[test]`.

### 5.1 SPEC.md grep-clean

- [ ] **V1.** `rg -Fn 'classifier → type parsers (hero/capture/monster/' SPEC.md` returns 0 hits. (Edit #1 landed.)
- [ ] **V2.** `rg -Fn '(captures, legendaries)' SPEC.md` returns 0 hits. (Edits #2, #3 dropped the parentheticals.)
- [ ] **V3.** `rg -Fn '(captures / legendaries)' SPEC.md` returns 0 hits. (Edit #5 dropped the parenthetical in §6.3.)
- [ ] **V4.** `rg -Fn 'captures and legendaries are both ReplicaItem kinds' SPEC.md` returns 0 hits. (Edit #4 dropped the CRUD comment gloss.)
- [ ] **V5.** `rg -Fn '(one hero, capture, monster, structural, etc.)' SPEC.md` returns 0 hits. (Edit #6 retargeted `capture` → `replica_item`.)
- [ ] **V6.** `rg -Fn '**Capturable / Legendary** — Kinds of' SPEC.md` returns 0 hits. (Edit #8 deleted the glossary entry.)
- [ ] **V7.** `rg -Fn 'IR type for items that summon a Pokemon as a unit. Two kinds' SPEC.md` returns 0 hits. (Edit #7 rewrote the ReplicaItem glossary.)
- [ ] **V8.** `rg -Fn 'capturables / monsters / bosses' SPEC.md` returns 1 hit. (Line 20 preserved verbatim.)
- [ ] **V9.** SPEC §6.3's "at most one" invariant and its CRUD-enforcement sentence survive verbatim: `rg -Fc 'at **most one** of: heroes, replica items, monsters. CRUD operations enforce this; the author cannot accidentally bypass it.' SPEC.md` returns 1.
- [ ] **V10.** Every SPEC replacement token `replica_item` (edits #1, #6) and `replica items` (edits #2, #3, #5) landed exactly once per site: `rg -Fn 'hero/replica_item/monster/' SPEC.md` returns 1; `rg -Fn '(one hero, replica_item, monster, structural, etc.)' SPEC.md` returns 1.

### 5.2 plans/PLATFORM_FOUNDATIONS_PLAN.md grep-clean

- [ ] **V11.** `rg -n 'ReplicaItemContainer|ReplicaItemKind' plans/PLATFORM_FOUNDATIONS_PLAN.md` — every hit must sit inside a `SUPERSEDED by Chunk 9` / `deleted in Chunk 9` / `archived` / `~~…~~` historical framing, or 8C must have rewritten it in §3.2's pass. Spot-check each hit's 5-line neighborhood.
- [ ] **V12.** `rg -c 'SUPERSEDED by Chunk 9' plans/PLATFORM_FOUNDATIONS_PLAN.md` returns >0. (Chunk 9's in-tree wrapper still present.)
- [ ] **V13.** R2's post-Chunk-9 ruling preserved verbatim: `rg -Fc 'Ruling (as of 2026-04-23, superseding the original Chunk 6 ruling): \`ReplicaItem\` models Legendaries only' plans/PLATFORM_FOUNDATIONS_PLAN.md` returns 1.

### 5.3 .claude/settings.json hook (only if §3.3 upstreaming lands)

- [ ] **V14.** `python3 -c 'import json; json.load(open(".claude/settings.json"))'` succeeds (JSON parses).
- [ ] **V15.** `jq -r '.hooks.PreToolUse[0].hooks[0].command' .claude/settings.json | grep -c 'corpus instance per variant'` returns 1 (existing bullet #3 unchanged).
- [ ] **V16.** If the `Edit|Write|NotebookEdit` hook was upstreamed: `jq -r '.hooks.PreToolUse[1].matcher' .claude/settings.json` returns `Edit|Write|NotebookEdit` and `jq -r '.hooks.PreToolUse[1].hooks[0].command' .claude/settings.json | grep -c 'Evidence rule'` returns 1. If upstreaming was skipped: `jq '.hooks.PreToolUse | length' .claude/settings.json` returns 1 (the committed file is unchanged; V16 is a no-op).

### 5.4 compiler/examples/roundtrip_diag.rs (only if §3.4 helper lands)

- [ ] **V17.** `cargo run --example roundtrip_diag` runs clean (no compile error, no panic).
- [ ] **V18.** `cargo run --example roundtrip_diag 2>&1 | grep -c 'Status: ROUNDTRIP OK'` returns 4 (all four mods still round-trip — 8C adds no behavioral change).
- [ ] **V19.** If a single-line helper landed: `cargo run --example roundtrip_diag 2>&1 | grep -c 'Legendary-only post-Chunk-9'` returns 4 (one per mod). If skipped, V19 is a no-op.

### 5.5 Full-tree final greps

- [ ] **V20.** `cargo test` passes (8C ships no Rust code edits to `compiler/src/` or `compiler/tests/`; the only `cargo` surface touched is `compiler/examples/roundtrip_diag.rs` if §3.4 lands).
- [ ] **V21.** `cargo build` + `cargo clippy` clean.

---

## 6. Structural check (hook rules)

### 6.1 Authority diff (hook rule #1 / checklist bullet 1)

Against parent `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md`:

- **§3.6 (SPEC edits)**: restated here as §3.1. Every replacement string is a narrowing of the parent's "drop IR-kind parentheticals; preserve uniqueness invariant; preserve game-flavor vocabulary" mandate, aligned to the Chunk 9 Legendary-only shape. **No contradiction** — this plan is strictly narrower than the parent's §3.6 in exactly the direction Chunk 9 already shipped.
- **§3.7 (plan-layer edits)**: restated here as §3.2 as a verify-only pass (not a rewrite), because Chunk 9's in-tree updates already did the normative rewrite. **No contradiction.**
- **Parent §9.3 (sub-chunk 8c scope)**: the file set is narrower (4 files max, 1 mandatory) than the parent's original 5-file set because the hook-bullet authoring task and the `drift_audit.rs` verification are now moot. **No contradiction** — parent's "if relevant" caveat on drift_audit permits skipping.

### 6.2 Source-vs-IR test present (hook rule #2 / checklist bullet 4)

8C is prose-only and ships no `#[test]`. The rule's intent — "an input that would be interpreted differently if the code reached for a derived / canonical / registry data source instead of the source bytes" — is already codified in the committed repo-root `.claude/settings.json` hook bullet #3 + bullet #4. 8C adds no authoring here.

### 6.3 Structural smells (hook rule #3 / checklist bullets 1, 2, 6)

- **Collapsing different-invariant paths**: edit #5 (§6.3 uniqueness parenthetical) and edit #7 (ReplicaItem glossary) address two different invariants and are preserved as two distinct edits (§3.1 structural-smell note). Not collapsed.
- **Duplicated N-line incantations**: the SPEC parenthetical-drop edits (#2, #3, #5) each touch a single site with one-line replacements. No N-line incantation pasted.
- **Canonical set named once**: the bucket set `{hero, replica_item, monster}` is named once in edit #5 and referenced by name in edit #6. Not re-listed.
- **Implementation as authority (checklist bullet 2 final clause)**: the ReplicaItem-glossary rewrite (edit #7) aligns SPEC prose to the landed `ReplicaItem` struct in `compiler/src/ir/mod.rs` (Legendary-only), not to a hypothesized trigger-based shape. Implementation is the authority; the prose follows.
- **No unevidenced variants introduced (checklist bullet 3)**: every SPEC and plan target in 8C describes the shipped Chunk 9 IR shape. No hypothesized variant is written into prose.

### 6.4 Parallel file check

8C files (§4 above):
- `SPEC.md`
- `plans/PLATFORM_FOUNDATIONS_PLAN.md`
- `.claude/settings.json` (optional)
- `compiler/examples/roundtrip_diag.rs` (optional)

8A / 8B live under `compiler/src/` + `compiler/tests/`. **Zero overlap** with 8C's file set.

---

## 7. Risks

- **SPEC.md hit-set drift.** §2 pre-condition #4 captures the eight verbatim-quote anchors. If a quote no longer matches (e.g. an intervening SPEC edit rewrote line 254's wording), reconcile before landing — do not fall back to line-number targeting.
- **PLATFORM_FOUNDATIONS_PLAN.md surviving normative hit.** §3.2's verify-only pass may surface a stale-normative mention that Chunk 9's in-tree updates missed. Rewrite in place per the template; do not sweep historical traceability bullets.
- **Hook upstreaming risk.** If §3.3 upstreaming lands, the committed `.claude/settings.json` gains a second `PreToolUse` entry. Verify V14 (JSON parses) and V16 (both entries present with correct matchers). A mis-copied `\n` escape or em-dash in the command string can leave JSON valid but the rendered bullet misaligned.
- **roundtrip_diag helper readability.** If §3.4's single-line helper lands, ensure it does not clutter the existing `Status: ROUNDTRIP OK` / `Replicas ir1=…` formatting. If it does, drop the helper.

---

## 8. Self-verification checklist (AI executes before completion)

- [ ] §2 pre-conditions all pass before any 8C edit lands. If §2 #1 or #2 fails, Chunk 9 regressed — reopen it, do not paper over in 8C.
- [ ] Every SPEC.md replacement string in §3.1 is sourced from `compiler/src/ir/mod.rs`'s landed `ReplicaItem` struct shape or from `plans/PLATFORM_FOUNDATIONS_PLAN.md` R2 / §F7. No 8C-authored hypothesis.
- [ ] §3.2's verify-only pass surfaces zero live-normative retired-type hits, or every hit is rewritten in place.
- [ ] §3.3 is either completed (evidence-rule hook upstreamed) or skipped entirely. No half-landed state.
- [ ] §3.4 is either completed (single-line helper present) or skipped entirely.
- [ ] V1-V21 all pass (V16, V19 are conditional; no-ops if §3.3 / §3.4 skipped).
- [ ] `cargo build` + `cargo clippy` clean; `cargo test` passes.
- [ ] No file outside §4's list is touched. In particular, no edit to `compiler/src/**`, `compiler/tests/**`, `compiler/build.rs`, `compiler/Cargo.toml`, `compiler/examples/*` other than `roundtrip_diag.rs` (if §3.4 lands).
- [ ] No `#[test]` added anywhere.

---

## 9. Verification gate

Ship only if all pass:

- **V1-V10** (SPEC.md grep-clean): retired-type parentheticals dropped; glossary rewritten; line-20 game-flavor `capturables` preserved verbatim; §6.3 uniqueness-invariant sentence verbatim; replacement tokens present.
- **V11-V13** (foundations-plan grep-clean): every `ReplicaItemContainer` / `ReplicaItemKind` hit sits in a historical / superseded / archived frame; R2's post-Chunk-9 ruling verbatim.
- **V14-V16** (.claude/settings.json, conditional): JSON parses; existing bullet #3 unchanged; Edit|Write|NotebookEdit hook present (if upstreamed) or absent (if skipped).
- **V17-V19** (roundtrip_diag, conditional): builds + runs clean; all four mods still ROUNDTRIP OK; helper line present (if landed) or absent (if skipped).
- **V20-V21** (full-tree): `cargo test` / `cargo build` / `cargo clippy` all pass.

If any gate item fails, fix in-place before the PR lands.
