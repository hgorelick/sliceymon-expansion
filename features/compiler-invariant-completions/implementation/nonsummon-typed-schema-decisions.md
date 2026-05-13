# Decisions log: `nonsummon-typed-schema`

Round-by-round resolved decisions on the typed-`NonSummon` chunk plan. Future review rounds reference this file as the durable record of closed surfaces; the chunk plan ([`nonsummon-typed-schema.md`](./nonsummon-typed-schema.md)) carries only the decisions whose rationale must travel with the implementation prose.

Each entry: the question, the user-arbitrated pick, the corpus / SPEC / chunk-impl-checklist anchor that licensed the pick. Entries DO NOT decay; future personas re-prosecuting a closed surface must name a new repo-state justification (a new corpus instance, a SPEC change, a guide change) before the question can be re-opened.

---

## Round 5 (2026-05-06)

### D1 (round 5): Drop the bare-`Cast` `CompositionComponent` variant

- **Question:** Override the named "Cast variant evidence path" decision and drop bare-`Cast` (no `.abilitydata.`) from the IR?
- **Pick:** Drop. Remove `CompositionComponent::Cast { spell }`, T30.16a (the synthetic-only authoring witness), the bare-`Cast` authoring builder, and the §"Cast variant evidence path" decision section. Keep `CompositionComponent::CastWithAbility` (Mental Defense corpus instance, see D5).
- **Anchor:** Chunk-impl checklist rule 3 — every IR variant discriminator must have a corpus instance for the path that owns it. The only bare-`Cast` corpus instance lives in heropool context (punpuns Sphere `i.left.cast.Slam` inside an abilitydata body), routed through `replica_item_parser`'s heropool branch — NOT through `classify_non_summon_entry`. The non-summon path that owns `CompositionComponent` had zero corpus instances at chunk land.
- **Same-PR widening protocol:** when a future mod surfaces a bare itempool `i.<scope>.cast.<spell>`, the discovering PR adds the variant + corpus anchor + per-variant test as one landing event.

### D2 (round 5): Apply chunk-impl checklist rule 3 strictly to the other unevidenced variants / Options

- **Question:** Drop the remaining unevidenced cases (Sentinel String escape, parametric `Level.name_arg`, `SideDef.dice` Option None case, `VaseOp::Add`, `RitemxPrefix` one-variant enum)?
- **Pick:** Drop / collapse all. Specifically:
  - `Sentinel { token: String }` → `Sentinel(SentinelToken)` with `pub enum SentinelToken { Void, Uy /* widen on Phase-A surprise; never Other(String) */ }`. Anchors: pansaer Whistle `Void#Leather Vest#…`, community Treasure Map `uy#hat.(…)`.
  - `VaseOp::Level { name_arg: String, body }` → `VaseOp::LevelUp { body }`. Anchor: only observed value of the post-`level` token is `up` (community Switcheroo `t.vase.level up.i.dead crow.n.ignore me`).
  - `SideDef.dice: Option<DiceFaces>` → `SideDef.dice: DiceFaces`. Anchors: every corpus `SideDef` (pansaer Whistle, Top Hat, Hardhat) carries `.sd.<faces>`; matches `AbilityBody.dice` discipline.
  - `VaseOp::Add { target }` — dropped. Every corpus `vase.add` instance is a `t.vase.add.((replica.…))` summon shape routed through 8B's path; no non-summon-itempool-position `VaseOp::Add` corpus instance exists at chunk land.
  - `RitemxPrefix { Unpack }` enum + `prefix: Option<RitemxPrefix>` → `unpack_prefix: bool`. Anchor: `unpack.ritemx.<hex>` is the only observed prefix (community Ash Of War).
- **Anchor:** Chunk-impl checklist rule 3 — symmetric application of the same rule that governed D1. Plan's other widen-on-Phase-A enums (`AllitemScope`, `ScopeSpec`, `VaseOp`, `TrailerSuffixKV`, `ImgTransform`) already carry `never Other(String)` discipline; these five broke that discipline before D2.
- **Same-PR widening protocol:** each dropped-or-collapsed shape ships its typed form when the first corpus instance surfaces (variant / enum / Option / parametric field reintroduced + corpus anchor + per-variant test as one landing event).

### D3 (round 5): Keep current shape for runtime-enforced invariants (no type-system rewrite)

- **Question:** Push currently-runtime-enforced invariants into the type system (large IR rewrite — typed `SideDef.sidesc` enum, typed `NonSummonTrailer.name` field, eliminate `BareBaseGameRef` in favor of single-component `Composition`, typed `Mn { name, flags }` payload split)?
- **Pick:** Keep current shape. The proposed restructures are correct in principle but each cascades through every callsite, every test, and the emitter. The current panic-at-construction + permissive-extract pair holds for SPEC §3.2/§3.3.
- **Anchor:** SPEC §3.7 (correctness over convenience) does NOT mandate type-system enforcement when runtime enforcement is sound; the existing T30.16c (dual-sidesc authoring rejection) and T34 (no `Unclassified` builder) witnesses defend the invariants. Revisit when a real bug surfaces.
- **Surfaces still in scope (no rewrite this chunk):** `SideDef.sidesc: Option<String>` dual-Some surface; `NonSummonTrailer` exactly-one-`Name(_)` invariant in doc-comment; `BareBaseGameRef` vs single-component `Composition` discriminator-determinism (classifier picks one canonical shape — implementer-time concern); `VaseOp::Survive.suffixes_pre` `Mn` payload contains source-byte `&hidden`/`&temporary` flag glue.

### D4 (round 5): Widen `OuterWrap` pre-ship to encode trailer position

- **Question:** Split `OuterWrap::DoubleParen` into typed subvariants pre-ship, since both corpus shapes (Therapy / Rube Goldberg) are evidenced now?
- **Pick:** Widen. `OuterWrap::DoubleParen` → `OuterWrap::DoubleParenTrailerOuter` (Therapy anchor) + `OuterWrap::DoubleParenTrailerBetween` (Rube Goldberg anchor). T30.13 splits into T30.13 (Rube Goldberg / Between) + T30.13a (Therapy / Outer). Speculative `DoubleParenTrailerInside` stays gated on `double_paren_trailer_inside_observed=true` from the walker.
- **Anchor:** SPEC §3.2 (no raw passthrough) — collapsing two corpus-distinct trailer-position shapes into one IR discriminator forces the emitter to reconstruct trailer position from a side-channel (exactly the failure mode the plan polices for trailer-key order via the `Vec<TrailerSuffixKV>` pivot). Both subvariants have current corpus instances; chunk-impl rule 3 satisfied.

### D5 (round 5): Defer Mental Defense itempool-context verification to implementation start

- **Question:** Surface Mental Defense's itempool-context as an explicit pre-impl AC verification step, or defer to existing impl-time corpus re-verification?
- **Pick:** Defer. The plan already commits to corpus re-verification at implementation start ("line numbers are stale (pansaer is monoline ~350KB) and must be re-verified at impl start via `rg -nF '<verbatim>'`"). The Mental Defense surrounding-byte check is one item that re-verification owns. If the check shows Mental Defense is heropool / abilitydata-sub-body context, T30.16b demotes to a synthetic-only fixture and `CastWithAbility` becomes a deferred-landing variant parallel to D1's bare-`Cast` rationale.
- **Anchor:** Plan's existing impl-time corpus re-verification commitment (§"Context pack — Read first" bullet on `working-mods/*.txt`).

### D6 (round 5): Move two declarations to canonical locations

- **Question:** Relocate `SummonClassification` (currently inside the §"IR schema (compiler/src/ir/mod.rs)" code-fence but doc-comment says it lives in the extractor) and `MAX_NONSUMMON_DEPTH_OBSERVED` (currently in `extractor/replica_item_parser.rs` but read by walker / examples / tests)?
- **Pick:** Both moves.
  - `SummonClassification` enum declaration moves OUT of the §"IR schema" code-fence into a new sibling §"Extractor-internal types (`compiler/src/extractor/replica_item_parser.rs`)" subsection. The IR-schema fence committed to `ir/mod.rs`-only contents; extractor-internal types violated that.
  - `MAX_NONSUMMON_DEPTH_OBSERVED` declaration moves to `compiler/src/ir/mod.rs` as `pub const u8`. Rationale: it is an IR-level invariant on the recursive-sum (depth bound), and walker (in `compiler/examples/`) + integration tests need to import it cleanly — neither can import an extractor-internal `pub(crate)` const, and exposing the threshold as `pub` from the extractor leaks an extractor-internal into multiple downstream surfaces.
- **Anchor:** `personas/architecture.md` "module boundary" rules; SPEC §3.4 (WASM-readiness) framing the constant as IR-level rather than extractor-internal.

### D7 (round 5): Decline SPEC.md amendment in this chunk's scope

- **Question:** Add a SPEC.md §3.3 amendment to this PR's scope formalizing the permissive-extract / strict-authoring split as a project-level invariant?
- **Pick:** Decline. The chunk's two uses of the pattern (`Unclassified` pressure-valve, `SideDef` dual-sidesc) are already defended locally with SPEC §3.2 / §3.3 anchors; the §"Pressure-valve tradeoff" "no registry of carve-outs" rule already requires future re-uses to earn their own §F-class defense. SPEC.md amendment is scope creep that doesn't change implementation; future plans that re-use the pattern carry the responsibility, not this chunk.
- **Anchor:** Plan's existing §"Pressure-valve tradeoff — No registry of 'carve-out variants'" rule.

---

## Re-prosecution rule

Future review rounds prosecute a closed surface ONLY by naming a new repo-state justification:

- A new corpus instance contradicting the rationale (e.g. a future mod surfacing a non-summon-itempool `vase.add` reopens D2's `VaseOp::Add` drop).
- A SPEC.md change altering the load-bearing anchor (e.g. SPEC §3.3 rewriting permissive-extract semantics reopens D7).
- A `reference/textmod_guide.md` revision contradicting a corpus-derived inference (the guide wins per `CLAUDE.md`).
- A `personas/*.md` rule change altering the persona stance the pick depended on.

Personas re-prosecuting without naming such a justification have their finding downgraded to OPEN_QUESTION with the prior `user_decision` surfaced verbatim, then auto-retracted unless the user re-opens.
