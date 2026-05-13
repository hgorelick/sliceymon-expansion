# Chunk: `nonsummon-typed-schema` — Typed `NonSummonEntry` IR

**Slug:** `nonsummon-typed-schema`
**Feature:** `compiler-invariant-completions`
**Status:** needs-user-input (header normalized 2026-05-13 post platform-foundations / `/plans/` retirement; body still carries legacy `8A` / `8B` / `plans/CHUNK_8_*` framing pending `/plan-author --rewrite` per the engineering plan's Pre-Foundation precondition)
**PR:** —
**Depends on:** — (the upstream transitional shape ships in this same atomic commit; no prior chunk needed since `/plans/CHUNK_8A_*` was retired)
**Blocks:** `typed-summon-extractor` (consumes `NonSummonEntry` typed sum + `SummonClassification::Typed(_)` shape at the symbol level)
**Brief:** [`../brief.md`](../brief.md) · **Engineering plan:** [`../engineering-plan.md`](../engineering-plan.md) · **Sister sidecar:** [`./nonsummon-typed-schema-decisions.md`](./nonsummon-typed-schema-decisions.md) · **SPEC anchors:** §3.2 (no raw passthrough), §3.3 (permissive extract / self-contained IR), §3.7 (no deferred replacement)

## Goal

Replace `ItempoolItem::NonSummon { name, tier, content: String }` with `NonSummon(NonSummonEntry)` — a fully typed recursive sum that maps every byte of every observed non-summon itempool entry into typed fields, closing the SPEC §3.2 raw-passthrough debt the parent atomic rewrite tracked as transitional.

## Brief link

- **SPEC §3.2:** every byte in a textmod must parse into typed IR or fail extract. Transitional `content: String` in `ItempoolItem::NonSummon` is the residual violation.
- **SPEC §3.3:** any valid textmod extracts; novel shapes demote to a paired `Finding` + a typed pressure-valve variant, never a raw escape.
- **SPEC §3.7:** no deferred replacement; the typed schema lands in one atomic commit with every callsite migrated.
- **Non-goal honored:** no parallel representation of typed-summon + stringly-typed non-summon (forbidden by `CLAUDE.md` "No deferred correctness").

## Context pack

**Read first:**
- `compiler/src/ir/mod.rs` — current `ItempoolItem` shape; the variant being retired.
- `compiler/src/extractor/replica_item_parser.rs` — 8A stub `extract_from_itempool` returning a single bundled-content `NonSummon`; the surface this chunk replaces.
- `compiler/src/builder/replica_item_emitter.rs` — `emit_itempool` NonSummon branch; the emitter rewrite landing in the same chunk.
- `working-mods/{sliceymon,pansaer,punpuns,community}.txt` — corpus authority. Every variant is anchored by a verbatim quoted substring; line numbers are stale (pansaer is monoline ~350KB) and must be re-verified at impl start via `rg -nF '<verbatim>'`.

**Reference:**
- `reference/textmod_guide.md` — format authority for itempool body shape, `+`-split semantics, paren-depth, `ritemx` / `t.jinx` / `hat.<Template>.sd.…` / `learn.…` shapes.
- `personas/ai-development.md` — single-concern, atomic-rewrite, evidence-first chunk discipline.
- `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` §3.1, §3.3 — parent-plan IR target and SPEC §3.2 closure debt.

**Conventions / patterns to follow:**
- Source-byte preserving everywhere (no case normalization, no registry lookup, no canonical reordering of source-order suffixes).
- One emit arm per `NonSummonEntry` variant; shared helpers (`emit_trailer`, `emit_composition_component`, `emit_img`, `emit_ritemx`) factor cross-variant work — no duplicated incantations across emit arms.
- Paren-depth-aware splits via `util::split_at_depth0` / `util::slice_before_chain_and_cast`.
- Authoring API rejects the pressure-valve variant; only extract may produce it.

## Factoring Contract

**Owns (writes)** — exact paths this chunk creates or modifies. Each entry: `path` — what changes.

- `compiler/src/ir/mod.rs` — replace `ItempoolItem::NonSummon { name, tier, content }` with `NonSummon(NonSummonEntry)`; add the typed-IR types declared in the §"IR schema" block below plus the `pub const MAX_NONSUMMON_DEPTH_OBSERVED: u8` constant. The IR schema block is the canonical authority for every variant set — prose anywhere else in this plan that names members of a set names them by enum-name, never by re-listing.
- `compiler/src/extractor/replica_item_parser.rs` — replace 8A stub body with a two-stage classifier: `classify_summon_entry` (signature pinned, body stubbed for 8B) → `classify_non_summon_entry` (typed dispatch over corpus-evidenced shapes). Also introduces the `SummonClassification` enum declaration with the `Typed(_)` variant only — `classify_summon_entry`'s `Ok(None)` stub return type requires the enum to typecheck on landing. 8B widens the `Typed`-arm payload but does NOT add new top-level variants. The trigger-side analog of `NonSummonEntry::Unclassified` does NOT ship: every observed itempool summon in the four working mods matches one of the three corpus-evidenced wrapper shapes (Cast / SideUse{OuterPreface} / SideUse{InnerWrapper}), so a `SummonClassification::Unclassified` variant would be unevidenced per chunk-impl checklist rule 3. The non-summon side's `Unclassified` variant survives this rule via SPEC §3.3 permissive-extract necessity (the non-summon classifier walks arbitrary item bytes); the summon side does not have that necessity (a new wrapper shape implies new engine behavior, not new content).
- `compiler/src/extractor/mod.rs` — behavioral change in the `ModifierType::ItemPool` arm; no signature change.
- `compiler/src/builder/replica_item_emitter.rs` — add `emit_non_summon_entry`, `emit_trailer`, `emit_composition_component`, `emit_img`, `emit_ritemx`, `emit_post_i_chain_siblings` (the named helper for `SideDef`'s post-`i_chain` `#sidesc` / `#facade` sibling emission, byte-position-distinct from the `i_chain` `Sidesc` / `Facade` element emission per the Top Hat / Hardhat anchors); one match arm per `NonSummonEntry` variant.
- `compiler/src/builder/structural_emitter.rs` — confirm `StructuralContent::ItemPool` arm dispatches to `emit_itempool`; delete any leftover 8A `body.clone()` transitional code.
- `compiler/src/authoring/non_summon_entry.rs` — NEW; typed authoring builders per typed variant. No builder for the `Unclassified` variant (extract-only).
- `compiler/src/util.rs` — add `pub fn split_itempool_entries(body: &str) -> Vec<&str>` as a thin wrapper around the existing `split_at_depth0(body, '+')` (verified at `compiler/src/util.rs:68`, `separator: char` parameter). The named symbol ships unconditionally — `plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md` §2 pre-conditions greps for `fn split_itempool_entries\b`; reusing `split_at_depth0` directly without the wrapper would fail 8B's pre-condition gate.
- `compiler/examples/itempool_entry_shapes.rs` — NEW; corpus walker emitting per-fingerprint shape audit to `target/itempool-shape-audit.txt`. Retained as tracked tooling, not a throwaway script.
- `compiler/tests/integration_tests.rs` — add T30.0 (Unclassified-budget ratchet), T30.1–T30.20 (per typed variant), T30.21 (trailer-emit-order), T30.22 + T30.22a (depth bound: walker audit + extract-time check), T31 (cross-variant distinctness), T32 (serde round-trip per variant), T33 (four-mod typed round-trip), T34 (authoring excludes `Unclassified`), T35 (summon-stub returns `Ok(None)`).
- `compiler/tests/roundtrip_baseline.rs` — regenerate baselines so per-mod NonSummon entry counts become non-zero (previously bundled into a single `content: String`).

**Reads (no writes)** — files this chunk depends on but does not modify.

- `working-mods/{sliceymon,pansaer,punpuns,community}.txt` — corpus authority for variant evidence and round-trip targets.
- `reference/textmod_guide.md` — format spec.
- `compiler/src/ir/mod.rs` (already in Owns) — `pub struct ReplicaItem` and `pub enum ItempoolItem` shapes are preserved unchanged by this chunk; only the `ItempoolItem::NonSummon { name, tier, content: String }` variant is replaced with `NonSummon(NonSummonEntry)`.

**Forward-binding contracts (zero PR-diff this chunk).**

- `compiler/src/xref.rs` — `rg -n 'ItempoolItem::NonSummon|NonSummon\s*\{' compiler/src/xref.rs` returns zero match-arms; the variant-shape change forces nothing to migrate. Forward contract: any future xref rule that reads non-summon entry bytes must route through the typed `NonSummonEntry` accessors landed here, not through a re-introduced `content: String`.
- `compiler/src/ir/ops.rs` — `rg -n 'ItempoolItem::NonSummon|NonSummon\s*\{' compiler/src/ir/ops.rs` returns zero match-arms. Forward contract: any future CRUD op that searches by entry name must read the typed `TrailerSuffixKV::Name(_)` entry from `NonSummonTrailer.suffixes`, never a `content: String`.

**Forbidden** — paths explicitly off-limits to this chunk.

- `compiler/src/extractor/replica_item_parser.rs::classify_summon_entry` body — owned by 8B. This chunk pins the symbol + signature + a trivial `Ok(None)` stub; 8B replaces the body only.
- Any `archive/pre-guide/` content — predates `reference/textmod_guide.md`; not authoritative.
- `plans/CHUNK_8B_*` — 8B does not start until this chunk merges.

**Single concern**

> Retire the transitional `NonSummon.content: String` raw-passthrough field by adopting a typed recursive `NonSummonEntry` IR across every forced-migration callsite in one atomic commit.

**No scaffolding**

- [ ] Confirmed: `NonSummonEntry` and every component type is consumed by either the classifier (extract path), the emitter (build path), or `authoring/non_summon_entry.rs` (authoring path) on landing.
- [ ] Confirmed: `examples/itempool_entry_shapes.rs` exposes `pub fn walk_itempool_entries`; T30.0 imports the library function directly (NOT shelling out to `cargo run --example`); the example's `fn main` is the audit-file writer (see §"Phase-A walker outcomes").
- [ ] Confirmed: every typed variant has at least one T30.N test.

**Abstraction earns its place**

- [x] N/A — no new cross-chunk abstraction. Every type added serves the single representation change; the `emit_*` helpers are factored within one file to avoid the "pasted incantation" smell, not promoted to a module-level abstraction.

## IR schema (`compiler/src/ir/mod.rs`)

This block is the canonical authority for every variant set named in this chunk. Prose elsewhere references variants by enum-name; member lists are not duplicated.

```rust
pub enum ItempoolItem {
    /// Index into `ModIR.replica_items` (summon path; 8B populates).
    Summon(usize),
    /// Non-summon itempool entry. Typed recursive schema below.
    NonSummon(NonSummonEntry),
}

/// Every byte of every corpus non-summon entry maps into one of these variants.
/// Source-byte preserving; no `content: String` escape outside `Unclassified`.
pub enum NonSummonEntry {
    /// Bare base-game item name; pool-membership only.
    BareBaseGameRef { name: String },
    /// `#`-joined components plus shared trailer. Subsumes bare-keyword,
    /// bare-learn, bare-sentinel-composed, and unwrapped-vs-paren-wrapped
    /// shapes (`outer_wrap` discriminates).
    Composition { outer_wrap: OuterWrap, components: Vec<CompositionComponent>, trailer: NonSummonTrailer },
    /// `ritemx.<hex>` head with optional `.part`/`.splice`/`.m` and accessory chain.
    /// Splice payload is itself a typed entry (recursive).
    Ritemx { outer_wrap: OuterWrap, head: RitemxRef, tail_chain: Vec<CompositionComponent>, trailer: NonSummonTrailer },
    /// `t.jinx.<body>` template-jinx composition.
    TemplateJinx { outer_wrap: OuterWrap, template: TemplateRef, body: Box<NonSummonEntry>, trailer: NonSummonTrailer },
    /// `t.<Template>.abilitydata.(<body>)` — non-cast inline ability.
    /// (8B's `SummonTrigger::Cast` is the summon-routed cousin; not this variant.)
    InlineAbility { outer_wrap: OuterWrap, template: TemplateRef, ability: AbilityBody, tail_chain: Vec<CompositionComponent>, trailer: NonSummonTrailer },
    /// `self[.<scope>].allitem.<body>.…` recursive composition.
    AllitemComposition { outer_wrap: OuterWrap, scope: AllitemScope, body: Box<NonSummonEntry>, tail_chain: Vec<CompositionComponent>, trailer: NonSummonTrailer },
    /// SPEC §3.3 permissive-extract pressure-valve; paired with `W-REPLICA-NONSUMMON-UNCLASSIFIED`
    /// Finding at extract time; forbidden in authoring; T30.0 enforces zero-occurrence budget.
    Unclassified { source_bytes: String },
}

pub enum OuterWrap {
    Unwrapped,
    SingleParen,                   // `(X)…`
    /// `((…)#(…)).<trailer>` — trailer is POST all closing parens.
    /// Anchor: community Therapy `((hat.gambler)#(demon claw)#(self.left.hyperboned)).n.Therapy.tier.0.…`.
    DoubleParenTrailerOuter,
    /// `((…)#(…).<trailer>))` — trailer is BETWEEN the two outer paren pairs.
    /// Anchor: community Rube Goldberg Machine `((hat.(zm.…).n.The Contraption.…#(handcuffs.part.1).n.Rube Goldberg Machine.…))`.
    DoubleParenTrailerBetween,
    // A future `DoubleParenTrailerInside` variant ships only if the Phase-A
    // walker observes the trailer-inside-innermost-paren pattern (see
    // §"Phase-A walker outcomes" → `double_paren_trailer_inside_observed`).
}

pub enum CompositionComponent {
    BaseGameRef { name: String },
    Keyword { keyword: String, suffixes: Vec<String> },
    Learn { spell: String },
    /// `cast.<spell>.abilitydata.(<body>)` — cast WITH inline ability body.
    /// Anchor: community Mental Defense (see §"Corpus evidence"; itempool
    /// context confirmed at implementation start per §"Corpus evidence"
    /// re-verification rule). Shape-distinct from `Ability { template, body }`
    /// (which has `t.<Template>` head, not `cast.<spell>` head).
    /// A bare `cast.<spell>` (no `.abilitydata.`) i_chain element is NOT
    /// shipped in this chunk: the only corpus-evidenced bare-cast occurrence
    /// lives in heropool context (punpuns Sphere `i.left.cast.Slam` inside
    /// an abilitydata body), routed through `replica_item_parser`'s heropool
    /// branch — not through `classify_non_summon_entry`. Per the
    /// "every IR variant discriminator must have a corpus instance for the
    /// path that owns it" rule, the bare-`Cast` variant ships only in the PR
    /// that surfaces the first itempool-context bare-cast (variant + corpus
    /// anchor + per-variant test as one landing event).
    CastWithAbility { spell: String, ability: AbilityBody },
    Ritemx(RitemxRef),
    SideDef {
        template: TemplateRef,
        /// Every corpus `SideDef` anchor (Whistle, Top Hat, Hardhat) carries
        /// `.sd.<faces>`; non-Option matches that evidence and matches
        /// `AbilityBody.dice` discipline. If a future corpus instance lacks
        /// `.sd.`, reintroduce `Option` in the same PR with the verbatim
        /// anchor + per-variant test.
        dice: DiceFaces,
        i_chain: Vec<CompositionComponent>,
        // `#sidesc.<text>` post-i_chain sibling form. The `i.<scope>.sidesc.<text>`
        // i_chain form lives in `i_chain` as `Scoped { body: Sidesc(text) }`.
        // Permissive-extract / strict-authoring: extract allows both surfaces
        // populated simultaneously; authoring rejects (see Decision §
        // "Three-sidesc-surfaces invariant"; T30.16c witnesses).
        sidesc: Option<String>,
        // `#facade.<name>:<params>` post-i_chain sibling. Same dual-surface
        // invariant as `sidesc`.
        facade: Option<FacadeRef>,
    },
    Scoped { scope: ScopeSpec, body: Box<CompositionComponent> },
    Nested(Box<NonSummonEntry>),
    Jinx { body: Box<NonSummonEntry> },
    Ability { template: TemplateRef, body: AbilityBody },
    Vase(VaseOp),
    /// `Void` / `uy` sentinel tokens when composed (e.g. `Void#…`, `uy#hat.(…)`).
    /// Closed enum — no `Other(String)` escape; widen on Phase-A surprise
    /// in the same PR that surfaces the new sentinel.
    Sentinel(SentinelToken),
    /// `facade.<name>:<params>` token as an i_chain or tail_chain
    /// element. Typically wrapped in `Scoped { scope, body: Facade(...) }`
    /// when scoped (Whistle: `i.top.facade.bas157:0:-20:0`).
    Facade(FacadeRef),
    /// `sidesc.<text>` token as an i_chain element. Typically wrapped in
    /// `Scoped { scope, body: Sidesc(text) }` when scoped (Top Hat:
    /// `i.top.sidesc.Heal self [pips] [light]vitality[cu]…`).
    Sidesc(String),
}

pub struct RitemxRef {
    pub hex: String,
    /// `unpack.ritemx.<hex>…` is the only observed prefix shape in current
    /// corpus (community Ash Of War). A future PR that surfaces a second
    /// prefix (e.g. `repack.`) replaces this `bool` with a typed enum
    /// + per-variant test in the same landing event.
    pub unpack_prefix: bool,
    pub part: Option<u8>,
    pub splice: Option<Box<NonSummonEntry>>,
    pub multiplier: Option<i8>,
}

pub enum SentinelToken { Void, Uy /* widen on Phase-A surprise; never Other(String) */ }

pub struct TemplateRef { pub name: String }

pub enum AllitemScope { Self_, SelfT1, SelfT2, SelfEt2, SelfEt3 /* widen on Phase-A surprise; never `Other(String)` */ }

pub enum ScopeSpec {
    Self_, SelfT1, SelfT2, SelfEt2, Left, Right, Right2, Right3, Right5,
    Top, Bot, Mid, Mid2, Mid4, Left2, Topbot, Row, Col, Rightmost,
    /* widen on Phase-A surprise; never `Other(String)` */
}

pub struct AbilityBody {
    pub template: TemplateRef,
    pub dice: DiceFaces,
    pub i_chain: Vec<CompositionComponent>,
    pub ability_name: Option<String>,
    pub img: Option<ImgPayload>,
    pub sidescs: Vec<String>,
    pub facade: Option<FacadeRef>,
}

/// `vase.<verb>` operations. Each variant's corpus anchor lives in §"Corpus evidence";
/// the implementer verifies typed payload against Phase-A walker output before merging.
pub enum VaseOp {
    Ch { target: String },                                  // `vase.ch.<bytes>`
    Delevel { body: Box<NonSummonEntry> },                  // `vase.delevel.i.<chain>` — Switcheroo
    LevelUp { body: Box<NonSummonEntry> },                  // `vase.level up.i.<chain>.n.<name>` — closed verb encoding;
                                                            // a future corpus `vase.level <X>` with X != "up" reopens this
                                                            // as `Level { name_arg, body }` in the same PR + per-variant test.
    Survive { suffixes_pre: Vec<TrailerSuffixKV>, body: Box<NonSummonEntry> }, // Exp Bar
    /* widen on Phase-A surprise; never `Other(String)`. Note: `vase.add` corpus
     * occurrences are all `t.vase.add.((replica.…))` summon shapes routed
     * through 8B's path; no non-summon-itempool-position `VaseOp::Add` corpus
     * instance exists at chunk land. The variant ships in the PR that surfaces
     * the first one (variant + corpus anchor + per-variant test). */
}

pub struct FacadeRef { pub name: String, pub params: Vec<i16> }

/// Trailer is a Vec of typed suffixes in source-byte emit order. Vec (not named
/// fields) because corpus shows three distinct trailer-key orders across four
/// anchored entries: Consolation Prize / Blue World share `n.doc.tier.img.hsv`;
/// Therapy is `n.tier.doc.img`; Seeing Red is `tier.n.doc.img.hue`. Named fields
/// cannot encode emit order, and a side-channel `emit_order` would force every
/// emitter to consult two fields. Authoring helpers expose convenience constructors.
pub struct NonSummonTrailer { pub suffixes: Vec<TrailerSuffixKV> }

/// Every trailer-suffix key observed in the corpus, in typed form.
/// Invariant: every NonSummonEntry's trailer contains exactly one
/// `Name(_)` entry — extract enforces; authoring asserts.
/// Repeating keys are encoded as multiple Vec entries at their source-byte
/// positions. Anchor: community Rube Goldberg's outer trailer
/// `…draw.wand grips:-2:2.draw.lead weight:4:0.tier.0…` shows two `.draw.`
/// suffixes on one entry. `.b.<value>` repetition has no current corpus
/// anchor; if the implementer's pre-impl `rg -F '.b.' working-mods/*.txt`
/// shows zero on-entry repetition, the cell-per-occurrence semantics still
/// hold for `.draw.` — `.b.` ships as the single-cell common case.
pub enum TrailerSuffixKV {
    Name(String),
    Tier(i8),
    ModTier(i8),
    Img(ImgPayload),
    Doc(String),                // raw textmod-escape bytes; non-invariant-bearing
    Hue(i16),
    Hsv(i16, i16, i16),
    Hsl(i16, i16, i16),
    B(String),                  // each `.b.<value>` is a separate Vec entry
    P(PPayload),
    Draw(DrawOp),               // each `.draw.<spec>` is a separate Vec entry
    Rect(String),
    Speech(String),
    SpeechAlt(String),
    Mn(String),
    Part(u8),
    Hidden,
    Temporary,
    /* widen on Phase-A surprise; never `Other(String)` */
}

pub enum ImgPayload {
    Raw { bytes: String },
    NamedRef { name: String, transforms: Vec<ImgTransform> },
}
pub enum ImgTransform { Hue(i16), Hsv(i16, i16, i16), Hsl(i16, i16, i16), DrawOverlay(DrawOp) /* widen */ }
pub struct DrawOp { pub source: ImgRefOrRaw, pub x: i16, pub y: i16 }
pub enum ImgRefOrRaw { Ref(String), Raw(String) }
pub struct PPayload { pub fg: String, pub bg: String, pub alpha: i16 }

/// IR-level invariant (read by extractor + walker + integration tests).
/// Constant lives in `ir/mod.rs`, not in the extractor — examples and tests
/// must import it cleanly. Value derived from the Phase-A walker run at
/// chunk land + ratchet headroom of 1 (`max_observed_at_ship + 1`).
pub const MAX_NONSUMMON_DEPTH_OBSERVED: u8 = /* set at impl from walker output */;
```

### Extractor-internal types (`compiler/src/extractor/replica_item_parser.rs`)

`SummonClassification` is the return shape of `classify_summon_entry`'s `Ok(Some(_))` arm. It never enters the persisted IR, so it lives in the extractor module rather than `ir/mod.rs`. Shipped this chunk so the `Ok(None)` stub typechecks; 8B widens the `Typed` payload but does not add new top-level variants. No `Unclassified` variant: the corpus has zero conjunctive-pair-but-unrecognized-wrapper instances; the no-unevidenced-variants rule forbids shipping the variant.

```rust
pub enum SummonClassification {
    Typed(/* 8B-owned payload type; this chunk pins only the discriminator */),
}
```

### Canonical variant set (single authority)

The IR enums above are the canonical authority. `NonSummonEntry` enumerates the top-level entry shapes (the `Unclassified` SPEC §3.3 pressure-valve is one of them). `CompositionComponent` enumerates the i_chain / tail_chain components. Prose elsewhere in this plan, in sibling plans, and in implementation code references variants by enum-name; member lists are not duplicated.

The `(template + faces + i-chain + sidesc + facade)` "side-def with sidesc" corpus shape (e.g. pansaer's Whistle, Top Hat, Hardhat) maps into `Composition` with a `Scoped { body: Box(SideDef { … }) }` component, NOT a top-level variant — `CompositionComponent::SideDef` already carries the same fields, and a separate top-level variant fails the "let the implementation be the authority" rule. `OuterWrap` distinguishes the two corpus-evidenced double-paren shapes (`DoubleParenTrailerOuter` vs `DoubleParenTrailerBetween`) at the type level; the speculative trailer-inside fourth variant is gated through §"Phase-A walker outcomes", not through schema speculation.

### Corpus evidence — verbatim anchors

Every typed variant must have ≥1 corpus instance before ship. Anchors below are verbatim quoted substrings; line numbers are stale (pansaer.txt is monoline) and must be re-verified at impl start via `rg -nF '<verbatim>' working-mods/<mod>.txt`. The Phase-A corpus walker is the authoritative re-confirmation; any walker-vs-anchor disagreement halts implementation until this section is rewritten. Hit counts are NOT embedded — they decay the moment a mod lands; the implementer's pre-impl re-run of each `rg` is the canonical count.

- **BareBaseGameRef** — pansaer pool 1 tail: `…tier.0)+Amnesia+Broken Spirit+Compulsion+Parasite+Pharaoh Curse+…&Hidden.mn.Tier 0 and Lower Items`. Anchor: `rg -nF 'Amnesia+Broken Spirit' working-mods/pansaer.txt`.
- **Composition (unwrapped)** — community: `self.j2k#leather vest.m.2.n.Cat Ears.doc.mini.img.wolf ears.hsv.0:30:20.tier.0`. Anchor: `rg -nF 'self.j2k#leather vest' working-mods/community.txt`.
- **Composition (paren-wrapped)** — community: `(self.unpack.Boss Smash^99#k.unusable#Camomile#k.stasis).n.Consolation Prize.doc.bord.tier.0.img.Bond Certificate.hsv.0:0:50`. Anchor: `rg -nF 'Consolation Prize' working-mods/community.txt`.
- **Composition (single keyword component, paren-wrapped)** — community: `(k.death.n.Destiny.tier.0.img.Sapphire Skull.doc.kas333 l mini l ajfish)`. Anchor: `rg -nF 'k.death.n.Destiny' working-mods/community.txt`.
- **Composition (sentinel-composed)** — pansaer pool 1: `(Void#rightmost.k.potion.n.Empty Bottle.img.ite272.hsv.0:-70:0.tier.0)`. Anchor: `rg -nF 'Empty Bottle' working-mods/pansaer.txt`. Also community: `uy#hat.(Ace.sd.1-5:2-5:3-5:7-5:9-5:6-5.n.5 damage X).n.Treasure Map.…`. Anchor: `rg -nF 'Treasure Map' working-mods/community.txt`.
- **Composition (single learn component)** — pansaer: `(learn.Poke.n.Learn Poke.img.Poke.tier.2)`. Anchor: `rg -nF 'learn.Poke.n.Learn Poke' working-mods/pansaer.txt`.
- **Ritemx (plain)** — community: `ritemx.fb71.n.Blue World.doc.kas333.tier.0.img.coin.hsv.40:50:0`. Anchor: `rg -nF 'ritemx.fb71' working-mods/community.txt`.
- **Ritemx (with `unpack.` prefix)** — community: `unpack.ritemx.bf25.n.Ash Of War.doc.mini.tier.0.img.Powdered Mana.hsv.40:-90:0`. Anchor: `rg -nF 'unpack.ritemx.bf25' working-mods/community.txt`.
- **Ritemx (paren-wrapped, trailer outside)** — community: `(ritemx.a348).tier.0.n.Seeing Red.doc.Sefcear.img.sapphire skull.hue.42`. Anchor: `rg -nF 'Seeing Red' working-mods/community.txt`.
- **Ritemx (`.splice.` recursive payload, rename)** — sliceymon outer pool: `ritemx.158948.splice.Blindfold.tier.0.n.Power Herb.img.<bytes>` (capital `B`) and `ritemx.1768a.splice.t.Boar.tier.1.n.Focus Sash.img.<bytes>`. The same `ritemx.158948` entry also appears in a vase-replicated inner sub-pool with a lowercase splice payload: `ritemx.158948.splice.blindfold.tier.0.n.Power Herb.…`. Both casings must round-trip byte-equal — the byte-preserving classifier never normalizes case. Anchors: `rg -nF 'ritemx.158948.splice.Blindfold' working-mods/sliceymon.txt`, `rg -nF 'ritemx.158948.splice.blindfold' working-mods/sliceymon.txt`, `rg -nF 'ritemx.1768a' working-mods/sliceymon.txt`.
- **Ritemx (`#`-joined multi-ritemx + accessory + trailer)** — community: `(ritemx.5770.part.0)#(ritemx.431.part.0)#…#k.rainbow.n.Rainbow hat.doc.punpun.img.2ed000…tier.0`. Anchor: `rg -nF 'Rainbow hat' working-mods/community.txt`.
- **TemplateJinx** — community: `(t.jinx.unpack.et2.Summon.Slimelet).n.The Flu.tier.0.doc.rorbee.img.slimed.hue.-10`. Anchor: `rg -nF 'The Flu' working-mods/community.txt`. Also `t.jinx.allitem.(k.enduring#(all.twin daggers.n.mini circus)#k.pain#k.unusable).n.Mini Circus.…`. Anchor: `rg -nF 'Mini Circus' working-mods/community.txt`.
- **InlineAbility** — community: `(t.zm.abilitydata.(statue.sd.176-1:0-0:0-0:0-0:76-0:0-0.n.Spamming.img.<bytes>)).doc.kas333.n.Spam.tier.0`. Anchor: `rg -nF 'abilitydata' working-mods/community.txt`.
- **AllitemComposition** — community: `self.(t1.allitem.t.jinx.(Summon.Wolf.i.Twisted Bar.t.jinx.monster.fluctuate.img.dice.p.fff:111:90.n.Chaos Dice).n.Summon Chaos Dice.img.dice).n.Pandoras Cube.doc.rorbee.tier.0.img.golden d6.hue.30`. Anchor: `rg -nF 'Pandoras Cube' working-mods/community.txt`.
- **OuterWrap::DoubleParenTrailerBetween** — community Rube Goldberg Machine: `((hat.(zm.sd.181-0:…).n.The Contraption.i.Pharaoh Curse.part.1.m.4#k.groooooowth#k.singleuse.i.left.k.inflictdeath)#(handcuffs.part.1).n.Rube Goldberg Machine.…))`. Trailer (`.n.Rube Goldberg Machine.…`) sits BETWEEN the two outer paren pairs (after the inner `)` boundary, before the outermost `))`). Anchor: `rg -nF 'Rube Goldberg Machine' working-mods/community.txt`.
- **OuterWrap::DoubleParenTrailerOuter** — community Therapy: `((hat.gambler)#(demon claw)#(self.left.hyperboned)).n.Therapy.tier.0.doc.posalla.img.Bone Charm`. Trailer (`.n.Therapy.…`) sits POST all closing parens. Anchor: `rg -nF 'Therapy' working-mods/community.txt`.
- **Composition with `SideDef` component (i_chain Facade form, sidesc: None)** — pansaer pool 3 Whistle: `(Void#Leather Vest#top.hat.(Thief.sd.0-0:0-0:157-0.i.Blindfold.i.k.descend.i.top.facade.bas157:0:-20:0).n.Whistle.img.<bytes>.tier.2)`. Maps to `Composition { outer_wrap: SingleParen, components: [Sentinel{"Void"}, BaseGameRef{"Leather Vest"}, Scoped { scope: Top, body: Box(SideDef { template: TemplateRef{"Thief"}, dice: Some(<3-face faces>), i_chain: [BaseGameRef{"Blindfold"}, Keyword{"descend",[]}, Scoped{Top, Box(Facade(FacadeRef{"bas157", [0,-20,0]}))}], sidesc: None, facade: None }) }], trailer: [Name("Whistle"), Img(…), Tier(2)] }`. Anchor: `rg -nF 'Whistle' working-mods/pansaer.txt`. Proves `CompositionComponent::Facade` for the i_chain `i.top.facade.<name>:<params>` form.
- **Composition with `SideDef` component (sidesc Some, sibling `#facade` form)** — pansaer pool 3 Top Hat: `(Void#top.hat.(Thief.sd.0-0:0-0:187-1:0-0:0-0:0-0.i.k.heal.i.k.vitality.i.k.cleanse.i.top.sidesc.Heal self [pips] [light]vitality[cu] [light]cleanse[cu][nokeyword]#facade.kas33:0).n.Top Hat.img.<bytes>.tier.1)`. Maps to `Composition { outer_wrap: SingleParen, components: [Sentinel{"Void"}, Scoped{ scope: Top, body: Box(SideDef { template: TemplateRef{"Thief"}, dice: Some(<6-face faces>), i_chain: [Keyword{"heal",[]}, Keyword{"vitality",[]}, Keyword{"cleanse",[]}, Scoped{Top, Box(Sidesc("Heal self [pips] [light]vitality[cu] [light]cleanse[cu][nokeyword]"))}], sidesc: None, facade: Some(FacadeRef{"kas33", [0]}) }) }], trailer: [Name("Top Hat"), Img(…), Tier(1)] }`. Anchor: `rg -nF 'Top Hat' working-mods/pansaer.txt`. Proves `CompositionComponent::Sidesc` for the i_chain `i.top.sidesc.<text>` form AND `SideDef.facade: Some(...)` for the post-i_chain `#facade` sibling form. The two sidesc surfaces are byte-position-distinct, not redundant — this entry's `i_chain` ends with a Sidesc element while `SideDef.sidesc` is `None`.
- **Composition with `SideDef` component (Hardhat — third anchor exercising both forms)** — pansaer Hardhat: `(Void#top.hat.(Thief.sd.0-0:0-0:187-6.i.k.shield.i.top.sidesc.Shield self [pips]#facade.Bal1:0).n.Hardhat.img.<bytes>.tier.2)`. Same shape as Top Hat: `i_chain` carries the `Scoped{Top, Sidesc("Shield self [pips]")}` element; `SideDef.facade: Some(FacadeRef{"Bal1", [0]})` carries the post-i_chain sibling. Anchor: `rg -nF 'Hardhat' working-mods/pansaer.txt`.
- **Composition with `CastWithAbility` component (cast WITH inline ability body, itempool-context — implementer re-verifies surrounding bytes pre-impl)** — community Mental Defense: `(mid.mid.hat.thief.i.(cast.sthief.abilitydata.(mage.sd.128-2.i.mid.left.hat.ace.i.sticker.enchanted shield#togtime#togtarg#togfri.i.pendulum.i.mid.sticker.self.sandstorm^1#togtarg#togtime#togfri)).i.facade.ite34:40).n.Mental Defense.tier.0.img.big shield.hue.40`. The `cast.sthief.abilitydata.(...)` token at depth-0 inside the outer `i.(...)` is a `CompositionComponent::CastWithAbility { spell: "sthief", ability: AbilityBody { template: TemplateRef{"mage"}, dice: <1-face faces>, i_chain: [...], ... } }` element. Anchor: `rg -nF 'cast.sthief.abilitydata' working-mods/community.txt`. Itempool-context status is re-verified at implementation start with surrounding-byte greps (`rg -B<n> -A<n> 'Mental Defense' working-mods/community.txt` showing the entry is inside an `^itempool` body); if Mental Defense turns out to live in heropool / abilitydata sub-body context, T30.16b demotes to a synthetic-only fixture and `CastWithAbility` becomes a deferred-landing variant (parallel to the bare-`Cast` no-ship rationale below). Shape-distinct from `Ability { template, body }` (which has `t.<Template>` head, not `cast.<spell>` head).
- **Bare `cast.<spell>` i_chain element (no `.abilitydata.`) — NOT shipped this chunk; rationale anchor** — punpuns Sphere replica: `((heropool.Thief+…+(replica.Sphere.abilitydata.(Mage.sd.0-0:0-0:187-1:187-1.i.left.cast.Slam#sidesc.[pips] damage [yellow] heavy.i.topbot.k.nothing.…)).n.Sphere…))`. The `i.left.cast.Slam` lives in **heropool** context, routed through `replica_item_parser`'s heropool branch, NOT through `classify_non_summon_entry`. Per the no-unevidenced-variants rule, the bare-`Cast` `CompositionComponent` variant is NOT shipped this chunk — the path that owns the variant has zero corpus instances at chunk land. When a future mod surfaces a bare `i.<scope>.cast.<spell>` inside an itempool body, the discovering PR adds the variant + corpus anchor + per-variant test as one landing event. Anchor: `rg -nF 'cast.Slam' working-mods/punpuns.txt`.
- **VaseOp Delevel / LevelUp (community Switcheroo)** — `(t.summon.add.Summon.vase.delevel.i.t.vase.level up.i.dead crow.n.ignore me)).n.ignore me.img.uy.mn.ignore me)#clearicon#cleardesc.n.Switcheroo.doc.…`. The outer `Summon.vase.delevel.i.<chain>` carries `VaseOp::Delevel { body: Box(...) }`; the inner `t.vase.level up.i.dead crow.n.ignore me` is a recursive `Vase(LevelUp { body: Box(BareBaseGameRef{"dead crow"} with trailer Name("ignore me")) })` — closed verb encoding (the only observed value of the post-`level` token is `up`). Anchor: `rg -nF 'vase.delevel' working-mods/community.txt`.
- **VaseOp Survive (community Exp Bar)** — `(…vase.survive.part.1.mn.Exp&hidden).i.Dead Crow).n. .img.glass heart)#cleardesc)#clearicon.doc.All heroes gain 2 empty hp permanently after combat[nh][grey]kas333.n.Exp Bar.tier.6.img.taxes.p.777:0f0:10`. The `vase.survive.part.1.mn.Exp&hidden` carries `VaseOp::Survive { suffixes_pre: [Part(1), Mn("Exp&hidden")], body: Box(BareBaseGameRef{"Dead Crow"} with trailer [Name(" "), Img(NamedRef{"glass heart",[]})]) }`. Anchor: `rg -nF 'vase.survive' working-mods/community.txt`.
- **AllitemScope SelfT2 / SelfEt3 anchors** — community Energy Drink: `((self.t1.(allitem.(eye of horus.n.   )))#(self.t2.allitem.(eye of horus.m.-1.n.   ))#clearicon.n.Energy Drink.img.Titanbane Potion.hsv.80:0:-10.b.000.tier.3.doc.kas333)`. Community Two Weeks Notice: `(brittle.part.1#(t.vase.ch.q1~1~1~g.mn.hero random color)#(self.et3.allitem.(hat.(ace.sd.13-0:13-0:13-0:13-0:13-0:13-0.n.Die Cantrip).n.Die))#clearicon.n.Two Weeks Notice.…)`. Anchors: `rg -nF 'self.t2.allitem' working-mods/community.txt`, `rg -nF 'self.et3.allitem' working-mods/community.txt`.
- **Unclassified** — by construction has no corpus instance at ship time (T30.0 enforces zero-budget). Variant exists so the extractor cannot panic on a future novel shape; firing on current corpus is a same-PR widening trigger.

**Per-enum variant-anchor verification (chunk-impl checklist rule 3).** Every variant of `ScopeSpec`, `TrailerSuffixKV`, and `ImgTransform` not explicitly anchored above must be verified pre-ship. Per the chunk-impl checklist, every IR variant discriminator must have a corpus instance pre-ship — un-evidenced variants are deleted, not shipped as hypotheses. The implementer runs `rg -nFc '<variant-token>' working-mods/*.txt` for each variant prior to authoring the enum and either anchors it inline (extending §"Corpus evidence" with the verbatim hit) or removes the variant. The IR schema block above is the canonical authority for the variant set; this paragraph references each enum by name, never by re-listing members. The Phase-A walker's `unmapped_count` covers `NonSummonEntry` discriminators only; the leaf-enum variants of the three named enums sit below the walker's discriminator surface, so the walker does not catch their un-evidence. Recorded in the PR body as "per-enum anchor table" per the AC item below.

### Outer-modifier cases that are NOT itempool entries

`itempool.Void.part.0.mn.Clear Itempool` (sliceymon) is the OUTER `StructuralContent::ItemPool { items: vec![], outer_name: Some("Clear Itempool") }` modifier with no entries at all — not a `NonSummonEntry`. SPEC §3.2 still applies: the outer modifier round-trips through the typed `ItemPool` IR (8A wired this; this chunk preserves it).

### Two-stage classifier dispatch

`extract_from_itempool` receives an itempool body. After paren-depth-0 `+`-split, each entry routes through:

1. `classify_summon_entry(entry, modifier_index) -> Result<Option<SummonClassification>, CompilerError>` — signature pinned and stub `Ok(None)` body shipped in this chunk; 8B replaces the body. Routing on `Ok(Some(SummonClassification::Typed(_)))` (typed summon variant matching one of the three corpus-evidenced wrappers), `Ok(None)` (not a summon — delegate to non-summon classifier), or `Err(_)` (parser-internal-bug or I/O error — never fires on corpus).
2. `classify_non_summon_entry(entry, modifier_index) -> Result<NonSummonEntry, CompilerError>` — typed dispatch over the corpus-evidenced variants. No registry lookup, no case normalization, no `SpriteId` reach. Unrecognized shapes emit a `W-REPLICA-NONSUMMON-UNCLASSIFIED` Finding paired with `Ok(NonSummonEntry::Unclassified { source_bytes })`. `Err(_)` is reserved for parser-internal-bug / I/O errors and never fires on corpus.

**Permissive-extract routing convention (non-summon side).** The non-summon classifier routes its permissive-extract pressure-valve through `Ok(NonSummonEntry::Unclassified { source_bytes })` paired with a Finding, NOT through `Err`. SPEC §3.3 requires the extractor never panics on a novel non-summon shape AND SPEC §3.2 requires every byte to round-trip through the IR. Routing a novel non-summon shape through `Err` would satisfy §3.3 but not §3.2 — the entry would never enter the IR, so `cargo run -- check <mod> --round-trip` would have no IR to re-emit.

**Why no symmetric trigger-side pressure-valve.** The summon classifier checks for the conjunctive (egg + vase-add) pair AND one of three engine-defined wrapper shapes (Cast / SideUse{OuterPreface} / SideUse{InnerWrapper}). All four working mods carry exactly the three wrapper shapes; a fourth wrapper would imply a new engine behavior, not new content. Per chunk-impl checklist rule 3 ("zero instances for a variant means the variant is a hypothesis masquerading as a model — delete the variant"), no `SummonClassification::Unclassified` variant ships, no `W-REPLICA-TRIGGER-UNCLASSIFIED` Finding is wired, and no T9c-style trigger-side pressure-valve test ships in this chunk. If a future mod ever surfaces a fourth wrapper, the discovering PR adds the typed variant + corpus anchor + per-variant test; the absence here is the ratchet that makes that PR a deliberate landing event. `Option` discriminates "is-summon"; `Result::Err` is reserved for parser-internal-bug / I/O errors and never fires on corpus.

### Phase-A walker outcomes

The Phase-A walker (`compiler/examples/itempool_entry_shapes.rs`) is the corpus-evidence audit. Its public surface is `pub fn walk_itempool_entries(mod_paths: &[&Path]) -> WalkAudit` (struct shape pinned below) plus a thin `fn main()` that calls it and writes `target/itempool-shape-audit.txt`. T30.0 imports `walk_itempool_entries` as a library function and asserts its output directly — the test does NOT shell out to `cargo run --example`.

**Fingerprint shape.** `EntryFingerprint` hashes only the structural bytes of an entry, not the variable bytes (so two entries with the same shape and different names collapse to one fingerprint). Inputs to the hash:

- Outer-wrap pattern: every `OuterWrap` variant declared in the IR schema (lines 124-136 are the authority; variant names are not re-listed here). The speculative `DoubleParenTrailerInside` enters the alphabet only after `double_paren_trailer_inside_observed=true` — see §`OuterWrap::DoubleParenTrailerInside` decision rule below.
- **Trailer-position discriminator** (computed from the typed `OuterWrap` variant — cross-validates the type-level discriminator). Encoded as one of `outer` (matches `OuterWrap::DoubleParenTrailerOuter` — Therapy), `between_pairs` (matches `OuterWrap::DoubleParenTrailerBetween` — Rube Goldberg), or `inner` (speculative; only emitted if the walker observes the shape, which also triggers `double_paren_trailer_inside_observed: true`). The fingerprint reads the typed `OuterWrap` and emits the snake_case audit token; an implementation that lost a discriminator at the IR level (e.g. collapsed both DoubleParen subvariants) would fail T30.13/T30.13a's variant-identity assertions before this fingerprint slot mattered.
- Head-token signature: the first depth-0 token after the wrap (e.g. `ritemx.<hex>`, `t.jinx`, `self.allitem`, `hat.<template>`, a sentinel like `Void`, a base-game name, etc.) reduced to its discriminator (`ritemx`, `t.jinx`, `self.allitem`, `hat.<template>`, `Sentinel`, `BareName`).
- Trailer-suffix-key **sequence** (NOT sorted set): every key in `TrailerSuffixKV` order in **source-byte order**, encoded as the joined string `n|tier|img|hue|...`. Source-byte order is the load-bearing input — corpus shows three distinct trailer-key orders across four anchored entries (Consolation Prize, Therapy, Seeing Red, Blue World — Consolation Prize and Blue World share order); collapsing to a sorted set would mask source-vs-IR divergence at audit time and let `unmapped=0` co-exist with a silent round-trip break.
- Component shape signature: for `Composition` / `Ritemx` tail-chain entries, the shape pattern of each component as the corresponding `CompositionComponent` variant-name, joined with `|`. (See `pub enum CompositionComponent` for the variant set.)
- Recursion-depth observed: integer depth count for the deepest recursive descent in the entry (across all `Box<NonSummonEntry>` / `Box<CompositionComponent>` boundaries).

**Walker public surface (Rust shapes pinned).** T30.0 and T30.22 import these directly; the audit-file is the human-readable byproduct, not the test surface:

```rust
pub struct WalkAudit {
    pub fingerprints: Vec<EntryFingerprint>,
    pub total_entries: u32,
    pub unmapped_count: u32,
    pub max_depth_observed: u8,
    pub double_paren_trailer_inside_observed: bool,
}
pub struct EntryFingerprint {
    /// Structural-byte digest. 16 bytes (128 bits) closes the adversarial
    /// collision space for T30.22a's depth-overflow synthetic — a 64-bit
    /// digest is birthday-safe for ~5,000 corpus entries but not for
    /// adversarially-constructed inputs that can be tuned to collide.
    pub digest: [u8; 16],
    pub count: u32,
    pub variant: VariantTag,
    pub depth: u8,
    /// 120-byte cap. Truncate via byte-slice with UTF-8 fallback:
    /// `s.as_bytes().get(..120).map(|b| std::str::from_utf8(b).unwrap_or("<invalid utf8>")).unwrap_or(s).to_string()`.
    /// Corpus is ASCII-only (CLAUDE.md); the fallback path is unreachable
    /// on corpus and exists only for adversarial fixtures.
    pub example_preview: String,
}
pub enum VariantTag { Mapped(NonSummonEntryDiscriminant), Unmapped }
pub fn walk_itempool_entries(mod_paths: &[&std::path::Path]) -> WalkAudit;
```

`NonSummonEntryDiscriminant` is the unit-only mirror of `NonSummonEntry`'s discriminator (derive a stripped-payload enum or use `std::mem::discriminant`). T30.0 / T30.22 / T30.22a assert against this shape directly — no `target/itempool-shape-audit.txt` re-parsing from tests.

**`fingerprints` ordering.** The Vec is sorted lexicographic by `digest` ascending, ties broken by `count` descending, ties broken by `example_preview` ascending. Walker writes audit-file lines in this order. Determinism is load-bearing: two walker runs over the same corpus must produce byte-identical audit files (otherwise CI baselines are non-deterministic).

**Audit file format.** `target/itempool-shape-audit.txt` is line-oriented; one fingerprint per line:

```
FP:<8-hex-digest>  COUNT:<n>  VARIANT:<NonSummonEntry-variant-name|UNMAPPED>  DEPTH:<n>  EXAMPLE:<120-byte-preview>
```

Plus a trailing `TOTALS: fingerprints=<n> entries=<n> unmapped=<n> max_depth_observed=<n> double_paren_trailer_inside_observed=<true|false>` line. `cargo run --example itempool_entry_shapes` exits 0 iff the file writes cleanly. T30.0 enforces `unmapped=0`. The §"Recursion depth bound" subsection enforces a corpus-bounded `max_depth_observed`. The `double_paren_trailer_inside_observed` boolean is the gate documented in §"`OuterWrap::DoubleParenTrailerInside` decision rule".

The audit file is human-readable PR-body documentation. Tests assert against `WalkAudit` directly via `walk_itempool_entries`; audit-file regeneration is part of the PR's manual evidence, not the CI test gate.

### Recursion depth bound (corpus-asserted)

Seven recursive `Box`-payload sites form the recursion graph: `CompositionComponent::Nested`, `CompositionComponent::Jinx`, `CompositionComponent::Scoped`, `RitemxRef.splice`, every `VaseOp` Box-payload variant (the IR schema enum at lines 232-244 is the authority — variant names are not re-listed here), `TemplateJinx.body`, `AllitemComposition.body`. The compiler targets web/mobile (WASM) per `personas/architecture.md`; native-stack-size budgets do not apply.

Defense: corpus is empirically shallow (Pandoras Cube tops out near three levels). The constant `MAX_NONSUMMON_DEPTH_OBSERVED` ships in `compiler/src/ir/mod.rs` as a `pub const u8` (it is an IR-level invariant on the recursive-sum, not extractor-internal: examples and tests both import it directly), derived from the Phase-A walker run at chunk land + ratchet headroom of 1 (`max_observed_at_ship + 1`). Headroom of 1 means any new corpus depth-increase becomes a deliberate landing event in the SAME PR that adds the mod — looser headroom turns the test into an accepting oracle that green-lights silent corpus-depth growth.

**Load-bearing extract-time check.** `classify_non_summon_entry` and every recursive sub-fn (`classify_composition_components`, `classify_ritemx_tail`, `classify_template_jinx_body`, `classify_inline_ability`, `classify_allitem_body`, `classify_vase_op`, `classify_scoped_component`) accept a `depth: u8` parameter and return `Err(CompilerError::DepthExceeded { depth, max: ir::MAX_NONSUMMON_DEPTH_OBSERVED, source_bytes_preview })` when `depth >= MAX_NONSUMMON_DEPTH_OBSERVED`. Depth-increment rule (one rule, no per-Vec-element ambiguity): increment at every recursive descent into a `Box<NonSummonEntry>` site, every `Box<CompositionComponent>` site, AND once on entry to an `AbilityBody` (the `Ability` / `CastWithAbility` / `InlineAbility` shoulder). `AbilityBody.i_chain` is a `Vec<CompositionComponent>` — its elements do NOT increment depth per-element; per-element CompositionComponent recursion increments only when its own Box payload descends. The error is reserved for malformed/adversarial inputs and never fires on the four working mods — T30.22a (Box<NonSummonEntry> route) and T30.22b (AbilityBody-shoulder route) together assert every corpus entry classifies with `depth < MAX_NONSUMMON_DEPTH_OBSERVED` strictly. T30.22 (the walker audit) is the second-line ratchet on top of the extract-time check; all three must hold.

Two checks defend two surfaces with non-redundant roles: T30.22a is the **chunk-land ratchet** (corpus + adversarial — runs `classify_non_summon_entry` directly against synthetic depth-overflow input AND every corpus entry); T30.22 is the **future-PR ratchet** (a future mod that adds a deeper entry without bumping `MAX_NONSUMMON_DEPTH_OBSERVED` trips T30.22 on the same PR that adds the mod, because the walker reports `audit.max_depth_observed` independently). The extract-time check defends SPEC §3.4 (WASM stack); the walker audit defends SPEC §3.6 / §3.7 (corpus-depth growth ratchet). Audit-time-only would have constituted deferred correctness against SPEC §3.4 — SPEC §3.7: "'we'll fix it in a follow-up' are invalid justifications."

**`OuterWrap::DoubleParenTrailerInside` decision rule.** The walker emits an explicit boolean in the audit-file TOTALS line: `double_paren_trailer_inside_observed=true|false` (snake_case, matching `max_depth_observed=`). If true, the next round of this chunk widens `OuterWrap` to a fourth variant and lands the new variant + a T30.13b regression test in the SAME PR that updates the audit. If false, the IR enum stays at three variants and the PR body records `double_paren_trailer_inside_observed=false (walker run at <HEAD-sha>; <n> double-paren entries surveyed; zero matched the trailer-inside pattern)`. Hypothesis variants do not ride.

### `Unclassified` retirement protocol

- **T30.0 budget is ZERO across the four working mods** — pass predicate `unclassified_count == 0`, not `<= N`. Any current-corpus occurrence is a same-PR widening trigger, never a tolerance.
- **No authoring builder for `Unclassified`** — `compiler/src/authoring/non_summon_entry.rs` exposes builders for every typed variant; `Unclassified` is constructible only by extract.
- **Stderr warning on every encounter** — `#[serde(serialize_with = "ser_unclassified_warn")]` emits a `W-REPLICA-NONSUMMON-UNCLASSIFIED` line on every serialization. The defect is hot, not silent.
- **Retirement gate**: when a future working mod surfaces an `Unclassified` shape, the same PR that adds the mod widens this chunk's corpus-evidence section, adds the typed variant, extends the classifier, and the failure disappears.

### Pressure-valve tradeoff (chunk-impl checklist exception)

The chunk-implementation checklist requires "every IR variant discriminator must have at least one corpus instance per variant before it ships." `NonSummonEntry::Unclassified` ships with zero corpus instances by construction (T30.0 ratchets the count to zero across all four working mods at chunk land). This is a deliberate, defended exception.

- **Why an IR variant, not a `CompilerError`.** SPEC §3.3 requires permissive extract; SPEC §3.2 requires every byte to round-trip through the IR. `Err(CompilerError::UnclassifiedNonSummon)` satisfies §3.3 but not §3.2 — the entry would never enter the IR. The IR variant is the only construction that satisfies BOTH when corpus surprises the classifier. The variant's effective alphabet is {synthetic-only on `main`, corpus-occurring on novel-mod PRs}; the synthetic alphabet is exercised by the non-summon Unclassified test fixtures (no trigger-side analog ships, per §"Two-stage classifier dispatch — Why no symmetric trigger-side pressure-valve").
- **Same-PR widening protocol when T30.0 fires.** (a) bisect to identify the corpus shape; (b) add the shape to §"Corpus evidence"; (c) extend `classify_non_summon_entry` to type the shape; (d) add a T30.N test for the new typed variant; (e) verify T30.0 returns to zero. Steps (a)–(e) ship in the SAME PR.
- **No registry of "carve-out variants".** A registry is a defect attractor — it lets the next plan author cite the registry as license for a second carve-out without earning a §F-class defense. The defense lives here, anchored to SPEC §3.2 and §3.3.

### No registry reach

The classifier never:
- Looks up a name in any base-game item / Pokemon / keyword registry to decide BareBaseGameRef. Membership is "the entry is a single token at depth 0 with no `.`, no `#`, no `(`, no trailer"; `name` is verbatim bytes.
- Applies case normalization.
- Reaches for `SpriteId` / face-compat / any derived lookup table.
- Uses non-paren-aware `.`-splits.

Every test below pairs corpus-bytes input with a synthetic-bytes input that would classify differently if the classifier reached for a derived / canonical / registry source — source-vs-IR divergence enforced per variant, not once at the end.

## Tests to add

TDD — write each test before authoring the code it exercises. Every test names a corpus-bytes input plus a synthetic-bytes pair.

**Unit:**

- `non_summon_entry_serde_roundtrips_all_variants` (T32) — every variant serializes and deserializes through JSON and YAML to `==`. Catches dropped `Deserialize` wiring on later-added variants.
- `variants_do_not_collapse` (T31) — for each pair `(a, b)` of distinct `NonSummonEntry` instances (one per shipped variant), `emit_non_summon_entry(&a) != emit_non_summon_entry(&b)`. Pairwise IR `!=` is trivially satisfied by `#[derive(PartialEq)]` on enum discriminator distinctness; emit-bytes inequality is what "two variants emit the same typed shape" actually means and catches an emitter that lost a variant's discriminating bytes (e.g. forgot to emit `ritemx.` prefix, making `Ritemx` emit identically to `BareBaseGameRef`).
- `non_summon_entry_authoring_does_not_expose_unclassified` (T34) — TWO independent witnesses: (a) compile-time deny: `compiler/src/authoring/non_summon_entry.rs` carries a module-scope `#[deny(unused_imports)]` and does NOT import `NonSummonEntry::Unclassified` — any future builder that names the variant fails to compile; (b) integration grep: `rg -n 'NonSummonEntry::Unclassified|::Unclassified\s*\{|Unclassified\s*\{\s*source_bytes' compiler/src/authoring/` returns 0. Both must hold at chunk land — single-pattern grep on its own is an accepting oracle.
- `classify_summon_entry_stub_returns_ok_none_unconditionally` (T35) — locks the 8B handoff contract: this chunk ships `classify_summon_entry` with body `Ok(None)`; for ten synthetic inputs covering each non-summon corpus shape category, the stub returns `Ok(None)`. 8B's body replacement starts from this fixed point. No sister test on the trigger-side Finding is required — the trigger-side `Unclassified` variant and `W-REPLICA-TRIGGER-UNCLASSIFIED` Finding are not wired in this chunk (chunk-impl checklist rule 3, see §"Two-stage classifier dispatch — Why no symmetric trigger-side pressure-valve"); 8B's existing failure-mode-matrix coverage stands.

**Integration (per-variant corpus round-trip; one test per shipped typed variant; corpus + synthetic pair each):**

- `non_summon_bare_base_game_ref_amnesia_roundtrips_byte_equal` (T30.1) — corpus: pansaer's `Amnesia`. Synthetic: `NotABaseGameItem` between two `+` fences. Both classify as `BareBaseGameRef` and round-trip byte-equal — proves no item-registry reach.
- `non_summon_composition_consolation_prize_roundtrips_byte_equal` (T30.2) — corpus: community `Consolation Prize`. Synthetic: same-shape composition with a fabricated keyword token (`k.notarealkeyword`) that is not in any registry.
- `non_summon_composition_single_keyword_destiny_roundtrips_byte_equal` (T30.3) — corpus: community `(k.death.n.Destiny.…)`. Synthetic: `(k.fakeword.n.SyntheticName.tier.0.img.placeholder)`.
- `non_summon_composition_single_learn_poke_roundtrips_byte_equal` (T30.4) — corpus: pansaer `(learn.Poke.n.Learn Poke.…)`. Synthetic: `(learn.NotASpell.n.Synthetic Learn.img.placeholder.tier.2)`.
- `non_summon_ritemx_blue_world_roundtrips_byte_equal` (T30.5) — corpus: community `ritemx.fb71` plain trailer. Synthetic: `ritemx.deadbeef.n.Synthetic.tier.0`.
- `non_summon_ritemx_unpack_ash_of_war_roundtrips_byte_equal` (T30.6) — corpus: community `unpack.ritemx.bf25.n.Ash Of War.…`. Synthetic A: `unpack.ritemx.beef0.n.Synthetic Unpack.tier.0`. Synthetic B (registry-reach guard): `unpack.ritemx.0.n.Zero Hex.tier.0` — zero-byte ritemx hex, not in any conceivable lookup table; classifier MUST type as `Ritemx { head: RitemxRef { hex: "0", prefix: Some(RitemxPrefix::Unpack), … }, … }` byte-preserving. Both synthetics round-trip byte-equal — proves no ritemx-hex registry reach AND no `unpack`-prefix whitelist.
- `non_summon_ritemx_paren_wrapped_seeing_red_roundtrips_byte_equal` (T30.7) — corpus: community `(ritemx.a348).tier.0.n.Seeing Red.…`. Proves `OuterWrap::SingleParen` + trailer-outside (single-paren case; the double-paren cases are split between T30.13 and T30.13a); synthetic: `(ritemx.beef1).tier.0.n.Synthetic Outside`.
- `non_summon_ritemx_splice_rename_power_herb_roundtrips_byte_equal` (T30.8) — TWO corpus inputs (both must round-trip byte-equal): outer-pool `ritemx.158948.splice.Blindfold.tier.0.n.Power Herb.…` (capital `B`) AND vase-replicated inner-pool `ritemx.158948.splice.blindfold.tier.0.n.Power Herb.…` (lowercase `b`). The byte-preserving classifier must NOT case-normalize either; the test asserts both extractions produce distinct typed IRs (the inner `splice` payload differs by exactly one byte) AND each emits back to its source bytes verbatim. Synthetic: `ritemx.beef2.splice.SyntheticInner.tier.0.n.Synthetic Outer.img.placeholder`. Proves recursive `.splice.<body>` typing AND case fidelity.
- `non_summon_ritemx_hash_joined_keyword_hell_roundtrips_byte_equal` (T30.9) — corpus: community `((ritemx.b45a)#(ritemx.bcfd)#…).n.Keyword Hell 2.…`. Synthetic: `((ritemx.beef3)#(ritemx.beef4)).n.Synthetic Hell.tier.0.img.placeholder`.
- `non_summon_template_jinx_the_flu_roundtrips_byte_equal` (T30.10) — corpus: community `(t.jinx.unpack.et2.Summon.Slimelet).n.The Flu.…`. Synthetic: `(t.jinx.unpack.et2.Summon.Synthetic).n.Synthetic Jinx.tier.0`.
- `non_summon_inline_ability_spam_roundtrips_byte_equal` (T30.11) — corpus: community `(t.zm.abilitydata.(statue.sd.176-1:0-0:….n.Spamming.img.<bytes>)).…n.Spam.tier.0`. Synthetic: `(t.zm.abilitydata.(statue.sd.0-0:0-0:0-0:0-0:0-0:1-0.n.Synthetic Inner.img.placeholder)).n.Synthetic Outer.tier.0`.
- `non_summon_allitem_composition_pandoras_cube_roundtrips_byte_equal` (T30.12) — corpus: community `self.(t1.allitem.t.jinx.(Summon.Wolf.…n.Chaos Dice).n.Summon Chaos Dice.img.dice).n.Pandoras Cube.…`. Synthetic: deep-nested allitem with two depth-0 `.n.` sites and one depth-2 `.n.` site, all distinct synthetic names — proves IR preserves all three.
- `non_summon_outer_wrap_double_paren_trailer_between_rube_goldberg_roundtrips_byte_equal` (T30.13) — corpus: community `((hat.(…).n.The Contraption.…)#(handcuffs.part.1).n.Rube Goldberg Machine.…))`. Asserts the entry classifies with `outer_wrap: OuterWrap::DoubleParenTrailerBetween` and round-trips byte-equal. Proves the trailer-between-paren-pairs typed discriminator.
- `non_summon_outer_wrap_double_paren_trailer_outer_therapy_roundtrips_byte_equal` (T30.13a) — corpus: community `((hat.gambler)#(demon claw)#(self.left.hyperboned)).n.Therapy.tier.0.doc.posalla.img.Bone Charm`. Asserts the entry classifies with `outer_wrap: OuterWrap::DoubleParenTrailerOuter` and round-trips byte-equal. Companion to T30.13 — together they witness the two double-paren trailer-position shapes are byte-position-distinct, not redundant; an implementation that collapses them into a single `DoubleParen` discriminator fails T30.13a's variant-identity assertion.
- `non_summon_composition_with_side_def_component_whistle_roundtrips_byte_equal` (T30.14) — corpus: pansaer Whistle (see §"Corpus evidence" for full anchor). Asserts the entry classifies as `Composition { …, Scoped { Top, SideDef { …, i_chain: [BaseGameRef, Keyword, Scoped{Top, Facade}], sidesc: None, facade: None } } }` and round-trips byte-equal. Synthetic: same shape with synthetic dice faces, synthetic facade params, synthetic name. Proves `CompositionComponent::Facade` for the i_chain `i.top.facade.<name>:<params>` form.
- `non_summon_composition_top_hat_sidesc_some_roundtrips_byte_equal` (T30.15) — corpus: pansaer Top Hat. Asserts `i_chain` ends with a `Scoped{Top, Sidesc(...)}` element AND `SideDef.facade: Some(...)`. Synthetic: same shape with synthetic sidesc text containing escape sequences, synthetic facade name. Proves `CompositionComponent::Sidesc` for the i_chain form AND `SideDef.facade: Some(...)` for the post-i_chain `#facade` sibling form. Companion to T30.14 — together they witness that the two facade surfaces (i_chain element vs `#`-joined sibling) are byte-position-distinct, not redundant.
- `non_summon_cast_with_ability_mental_defense_roundtrips_byte_equal` (T30.16b) — corpus: community Mental Defense (itempool-context, see §"Corpus evidence"). Asserts the entry classifies with a `CompositionComponent::CastWithAbility { spell: "sthief", ability: AbilityBody { template: TemplateRef{"mage"}, dice: Some(<1-face faces from sd.128-2>), i_chain: [...], ... } }` element AND round-trips byte-equal. Synthetic: pair with `cast.NotAnInRegistrySpell.abilitydata.(Mage.sd.0-0:0-0:0-0:0-0:0-0:1-0.i.left.k.nothing).n.Synthetic Cast.tier.0` — both classify as `CastWithAbility` (no spell-registry reach, no template-registry reach) and round-trip byte-equal.
- `non_summon_dual_sidesc_authoring_rejects_at_construction` (T30.16c) — authoring-side witness for the permissive-extract / strict-authoring sidesc invariant: the `compiler/src/authoring/non_summon_entry.rs` `SideDef` builder MUST panic (or return an authoring-side error) when constructed with both `sidesc: Some(text1)` AND an `i_chain` containing a `Scoped { body: Box(Sidesc(text2)) }` element. The test attempts the dual-Some construction and asserts the constructor rejects it. Extract path (T30.14, T30.15) covers the permissive side: each anchor populates exactly one of the two surfaces, and the IR allows both populated simultaneously without an extract-time panic — but the authoring API gates the dual-Some combo as an unrepresentable invalid state. Sister to T34's `Unclassified`-builder-absent witness — same pattern (extract permissive, authoring strict), same SPEC §3.2/§3.3 anchor.
- `non_summon_allitem_self_t2_energy_drink_roundtrips_byte_equal` (T30.17) — corpus: community Energy Drink. Asserts at least one `AllitemComposition { scope: AllitemScope::SelfT2, … }` appears in the IR and the entry round-trips byte-equal. Synthetic: same shape with `self.t2.allitem.(synthetic.n.   )`. Proves `AllitemScope::SelfT2`.
- `non_summon_allitem_self_et3_two_weeks_notice_roundtrips_byte_equal` (T30.18) — corpus: community Two Weeks Notice. Asserts the entry contains an `AllitemComposition { scope: AllitemScope::SelfEt3, … }` and round-trips byte-equal. Synthetic pair: same shape with synthetic dice and synthetic name.
- `non_summon_vase_delevel_levelup_switcheroo_roundtrips_byte_equal` (T30.19) — corpus: community Switcheroo. Asserts the IR contains a nested `VaseOp::Delevel { body: Box(...) }` whose payload itself contains a `VaseOp::LevelUp { body: Box(BareBaseGameRef{"dead crow"} with trailer Name("ignore me")) }` (closed verb encoding — the only observed value of the post-`level` token is `up`), and round-trips byte-equal. Synthetic: deeper nesting with three `vase.level up` recursion levels — proves the recursive Box payload typing without re-introducing parametric `name_arg`.
- `non_summon_vase_survive_exp_bar_roundtrips_byte_equal` (T30.20) — corpus: community Exp Bar. Asserts `VaseOp::Survive { suffixes_pre: [Part(1), Mn("Exp&hidden")], body: Box(BareBaseGameRef{"Dead Crow"} with trailer [Name(" "), Img(NamedRef{"glass heart",[]})]) }`. Synthetic: same shape with synthetic suffix-pre values.
- `non_summon_trailer_emit_order_preserved_byte_equal` (T30.21) — TWO synthetic inputs that share the same trailer-key SET but differ in source-byte ORDER: input A `…n.Same.tier.0.doc.X.img.Y` and input B `…tier.0.n.Same.doc.X.img.Y`. Both must extract to distinct `NonSummonTrailer.suffixes: Vec<TrailerSuffixKV>` (order differs) AND each must `emit_non_summon_entry` to its source bytes. Source-vs-IR divergence test for the trailer-emit-order invariant — fails the named-field-struct schema, passes the Vec schema.
- `phase_a_walker_max_depth_observed_below_threshold` (T30.22) — runs `walk_itempool_entries` against the four working mods, asserts `audit.max_depth_observed < MAX_NONSUMMON_DEPTH_OBSERVED` where the constant is derived from `max_observed_at_ship + 1` (ratchet-headroom; const is in `compiler/src/ir/mod.rs` per D6 — IR-level invariant on the recursive-sum, imported by examples and tests directly). Walker-asserted shallow-bound defense for the seven recursive-Box sites; failure prompts a deliberate constant bump in the SAME PR that adds the deeper mod. Second-line ratchet on top of T30.22a / T30.22b.
- `non_summon_extract_time_depth_check_rejects_synthetic_overflow` (T30.22a) — extract-time depth-check witness for the `Box<NonSummonEntry>` recursion route. Two synthetic inputs: (a) a hand-constructed entry with recursion depth `MAX_NONSUMMON_DEPTH_OBSERVED + 1` (deeply nested `t.jinx.allitem.t.jinx.allitem.…` chain crafted to exceed the ratchet via `TemplateJinx.body → AllitemComposition.body → TemplateJinx.body → …`); `classify_non_summon_entry` returns `Err(CompilerError::DepthExceeded { depth, max, source_bytes_preview })` and never panics; (b) every corpus entry from the four working mods classifies with `depth < MAX_NONSUMMON_DEPTH_OBSERVED` strictly (no `Err(DepthExceeded)` on corpus). The error is reserved for adversarial/malformed inputs; SPEC §3.4 (WASM-readiness) defended at extract time, not just audit time.
- `non_summon_extract_time_depth_check_rejects_abilitybody_shoulder_overflow` (T30.22b) — extract-time depth-check witness for the `AbilityBody`-shoulder recursion route. Synthetic input: a hand-constructed entry chaining `cast.X.abilitydata.(Y.i.cast.X.abilitydata.(Y.i.…))` at depth `MAX_NONSUMMON_DEPTH_OBSERVED + 1` via the `CompositionComponent::CastWithAbility → AbilityBody → i_chain → CompositionComponent::CastWithAbility` shoulder; `classify_non_summon_entry` returns `Err(CompilerError::DepthExceeded { depth, max, source_bytes_preview })` and never panics. Companion to T30.22a — together they witness both Box-payload routes and AbilityBody-shoulder routes; an implementer who threads `depth` through Box descents but not through the `AbilityBody` shoulder passes T30.22a and fails T30.22b. The depth-increment rule pinned in §"Recursion depth bound" (one increment on entry to `AbilityBody`, none per-i_chain-element) is the load-bearing input.

**Cross-corpus and ratchet:**

- `working_mods_produce_zero_unclassified_entries_and_each_entry_round_trips_byte_equal` (T30.0) — runs `extract_from_itempool` against every itempool body in every working mod via the Phase-A walker (`walk_itempool_entries`). Asserts the conjunction (ALL THREE must hold per entry — none alone is sufficient): (a) `matches!(entry, NonSummonEntry::Unclassified { .. })` is false (variant identity); (b) `emit_non_summon_entry(&entry) == source_bytes` (per-entry round-trip — the typed variant correctly captures the source bytes); (c) **head-token-mutation source-vs-IR divergence guard** — for each entry, classify a corpus-wide-mutated copy where one byte of the head token is case-flipped (e.g. `ritemx.fb71` → `ritemx.Fb71`, `t.jinx.unpack.…` → `t.Jinx.unpack.…`, `Amnesia` → `aMnesia`); the mutated input must classify to the same `NonSummonEntryDiscriminant` AND `emit_non_summon_entry(&mutated) == mutated_source_bytes`. A registry-backed implementation either rejects the mutated form or normalizes it, failing this third conjunct on every corpus entry — making the source-vs-IR divergence ratchet corpus-wide, not per-handcrafted-synthetic. Failure message structure: (1) entry's 120-byte preview; (2) variant identity from `NonSummonEntryDiscriminant`; (3) IF variant == `Unclassified`: report 'classifier did not type this shape'; ELSE IF emit_bytes != source_bytes: report the byte-diff; ELSE IF case-mutation conjunct failed: report which mutation flipped which output. The three cases are mutually exclusive.
- `all_four_mods_roundtrip_byte_equal_with_typed_non_summon` (T33) — extracts each working mod, asserts no `NonSummonEntry::Unclassified` anywhere in the IR, and asserts byte-equal round-trip. Second-line ratchet on top of T30.0.

**Inherited from parent:**

- `tm_or_accessory_shape_demotes_to_typed_non_summon` (T6) — TMs and accessories that pass 8B's summon-shape prefilter but fail the conjunctive detector classify as some typed `NonSummonEntry` variant — specifically NOT `Unclassified`. The variant identity asserted is the one the corpus-fingerprint matches; the test fails if the entry demotes to `Unclassified` (accepting-oracle guard).
- `half_summon_demotes_to_typed_non_summon` (T7) — entry with `hat.egg.` head but no matching vase-add pair demotes to typed non-summon, not `Unclassified`. Same variant-identity assertion as T6.
- `summon_index_stable_across_non_summon_removal` (T28) — removing a `NonSummon` entry from a pool does not renumber `Summon(i)` indices, because `NonSummon` entries do not enter the `replica_items` vec.

T9c (the synthetic conjunctive-pair-but-unrecognized-wrapper test) was inherited from the parent plan during prior rounds; it is dropped from this chunk's surface because the trigger-side `Unclassified` variant is not shipped (no corpus evidence; chunk-impl checklist rule 3). 8B owns whatever conjunctive-pair-but-unrecognized-wrapper handling its body chooses; this chunk neither requires nor forbids the test.

## Acceptance criteria

Each item names a command that exits 0, a test that passes by name, a file with a named symbol, a green gate, or a corpus byte-equal round-trip.

- [ ] `cargo build` exits 0.
- [ ] `cargo clippy` exits 0.
- [ ] `cargo test` exits 0 (covers T30.0, T30.1–T30.13, T30.13a, T30.14–T30.15, T30.16b, T30.16c, T30.17–T30.22, T30.22a, T31, T32, T33, T34, T35, T6, T7, T28).
- [ ] Test `working_mods_produce_zero_unclassified_entries_and_each_entry_round_trips_byte_equal` passes — every entry in every working mod classifies as a non-`Unclassified` typed variant AND `emit_non_summon_entry` of that variant equals the source bytes verbatim.
- [ ] Test `classify_summon_entry_stub_returns_ok_none_unconditionally` passes — 8B handoff contract is locked.
- [ ] Test `all_four_mods_roundtrip_byte_equal_with_typed_non_summon` passes — every working mod round-trips byte-equal with no `Unclassified` in the IR.
- [ ] `cargo run --example roundtrip_diag` reports `Status: ROUNDTRIP OK` for sliceymon, pansaer, punpuns, and community.
- [ ] `cargo run --example itempool_entry_shapes` exits 0 and writes `target/itempool-shape-audit.txt` with `unmapped=0` AND `max_depth_observed < MAX_NONSUMMON_DEPTH_OBSERVED` in the trailing TOTALS line.
- [ ] Test `non_summon_trailer_emit_order_preserved_byte_equal` passes — TWO synthetic inputs sharing the same trailer-key set but differing in source-byte order extract to distinct `NonSummonTrailer.suffixes: Vec<TrailerSuffixKV>` AND each round-trips byte-equal (Vec-pivot source-vs-IR divergence guard).
- [ ] Test `phase_a_walker_max_depth_observed_below_threshold` passes — corpus-bounded recursion-depth assertion via the walker's `max_depth_observed` field.
- [ ] Tests `non_summon_composition_top_hat_sidesc_some_roundtrips_byte_equal` (T30.15) and `non_summon_cast_with_ability_mental_defense_roundtrips_byte_equal` (T30.16b) pass — proving `CompositionComponent::Sidesc` and `CompositionComponent::CastWithAbility` each round-trip without registry reach.
- [ ] Tests `non_summon_allitem_self_t2_energy_drink_roundtrips_byte_equal` (T30.17), `non_summon_allitem_self_et3_two_weeks_notice_roundtrips_byte_equal` (T30.18), `non_summon_vase_delevel_levelup_switcheroo_roundtrips_byte_equal` (T30.19), `non_summon_vase_survive_exp_bar_roundtrips_byte_equal` (T30.20) pass — proving `AllitemScope::SelfT2`, `AllitemScope::SelfEt3`, `VaseOp::Delevel`, `VaseOp::LevelUp`, `VaseOp::Survive` are corpus-evidenced and round-trip.
- [ ] File `compiler/src/ir/mod.rs` exports symbol `NonSummonEntry`.
- [ ] File `compiler/src/ir/mod.rs` exports symbol `NonSummonTrailer`.
- [ ] File `compiler/src/extractor/replica_item_parser.rs` exports symbol `classify_non_summon_entry`.
- [ ] File `compiler/src/extractor/replica_item_parser.rs` exports symbol `classify_summon_entry` with body `Ok(None)`.
- [ ] File `compiler/src/builder/replica_item_emitter.rs` exports symbol `emit_non_summon_entry`.
- [ ] File `compiler/src/authoring/non_summon_entry.rs` exists with typed builders (one per typed variant other than `Unclassified`).
- [ ] File `compiler/examples/itempool_entry_shapes.rs` exists with `fn main` that walks the four working mods AND exports `pub fn walk_itempool_entries(mod_paths: &[&Path]) -> WalkAudit` plus the public types `WalkAudit`, `EntryFingerprint`, `VariantTag`, `NonSummonEntryDiscriminant`. T30.0 / T30.22 / T30.22a import these directly.
- [ ] PR body carries a per-enum variant-anchor table covering `ScopeSpec`, `TrailerSuffixKV`, `ImgTransform` (and any other multi-variant enum the implementer adds): one row per variant with the `rg -nFc` hit count across `working-mods/*.txt` and the verbatim corpus snippet (or a deletion line for zero-hit variants). Variants with zero corpus hits across the four mods are NOT shipped — they are deleted from the enum in the same PR per the chunk-impl checklist rule 3.
- [ ] `rg -c 'NonSummon\s*\{\s*(name|content|body)\s*:' compiler/src/` returns 0 (8A transitional shape fully retired; this grep subsumes the narrower `content: String`-specific check).
- [ ] `rg -n 'String' compiler/src/ir/mod.rs` — every hit audited in the PR body resolves to one of the `String`-typed fields declared in the §"IR schema" code block above (the IR schema block is the canonical authority; this AC item references it by name, never by re-listing fields). Stringless enums declared in §"IR schema" stay stringless. Any new `String` hit not anchored in the §"IR schema" block fails this gate.
- [ ] `rg -c 'NonSummonEntry\b' compiler/src/` returns ≥2 (declaration plus at least one consumer in extract / emit / xref / ops / authoring).
- [ ] `rg -n 'W-REPLICA-NONSUMMON-UNCLASSIFIED' compiler/src/` returns ≥1 (Finding code wired).
- [ ] `rg -n 'Unclassified.*source_bytes' compiler/src/ir/mod.rs` returns 1 (sole documented raw-byte field, ratcheted by T30.0).
- [ ] T34 two-witness check: (a) `compiler/src/authoring/non_summon_entry.rs` carries module-scope `#[deny(unused_imports)]` AND does not import `NonSummonEntry::Unclassified` (compile-time guard); (b) `rg -n 'NonSummonEntry::Unclassified|::Unclassified\s*\{|Unclassified\s*\{\s*source_bytes' compiler/src/authoring/` returns 0 (integration grep). Both must hold.
- [ ] Parent and sibling plan prose amendments land in the same PR (cited by literal repo-relative path per `CLAUDE.md` "describing what was edited in a plan file" carve-out):
    - `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` §3.1 / §3.3 "transitional / open debt" clause on `ItempoolItem::NonSummon { content: String }` is rewritten to reference this chunk's closure.
    - Every "15 evidenced variants (V1–V15)" / "15 evidenced variants" / "typed 15-variant recursive sum" / "T30.1–T30.15" reference in `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` AND `plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md` is rewritten to reference `NonSummonEntry` / `CompositionComponent` by enum-name (the IR enums are the canonical authority; sibling plans must not re-list members) and to reference the expanded test set "T30.1–T30.22" where the older numbering appears.
    - `plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md` `SummonClassification` destructure form: line 111's struct-pattern `Ok(Some(SummonClassification { replica_item, /* … */ }))` is rewritten to enum-variant form `Ok(Some(SummonClassification::Typed(_)))` so 8B's body destructure typechecks against this chunk's enum declaration. 8B's existing "this Finding code is NOT wired in 8B" / BACKLOG decision and the failure-mode-matrix routing row for the conjunctive-pair-but-unrecognized-wrapper case stay as-is — both are correct under the chunk-impl checklist rule 3 dropping of the trigger-side pressure-valve here.
    - Every `8A\.5\b` identifier in `plans/CHUNK_8B_REPLICA_EXTRACTOR_XREF.md` AND `plans/CHUNK_8_REPLICA_ITEM_TRIGGER_PLAN.md` is rewritten to either the literal path `plans/nonsummon-typed-schema.md` (when describing edits to that file per `CLAUDE.md` "describing what was edited in a plan file" carve-out) or to a content paraphrase ("the typed-NonSummon classifier", "the typed-NonSummon emitter") when referring to behavior. Stale-handle `8A.5` references are forbidden post-PR-close.
    - The grep gate `rg -n '15 evidenced variants|V1[–-]V15|T30\.1\s*[–-]\s*T30\.(1[5-9]|20)|typed 15-variant|SummonClassification\s*\{|pub\s+struct\s+SummonClassification|8A\.5' plans/*.md` returns 0 at PR-close. (`SummonClassification\s*\{` catches the struct-destructure pattern; `pub\s+struct\s+SummonClassification` catches a fresh struct re-declaration that brace-spans newlines escaping a line-oriented match; `8A\.5` without word-boundary catches stale-handle extensions like `8A.5b`/`8A.5c`; `T30\.1\s*[–-]\s*T30\.(1[5-9]|20)` catches both the original "T30.1–T30.15" range and any intermediate-iteration range that doesn't reach the current "T30.1–T30.22"; the gate scans all `plans/*.md` files, not just the two named siblings, to catch any plan that imports the stale identifier.)
- [ ] File `compiler/src/extractor/replica_item_parser.rs` exports `pub enum SummonClassification` with the `Typed(_)` variant only — `rg -n 'pub enum SummonClassification' compiler/src/extractor/replica_item_parser.rs` returns 1; `rg -n 'pub struct SummonClassification' compiler/src/` returns 0; `rg -n 'SummonClassification::Unclassified' compiler/src/` returns 0 (no trigger-side pressure-valve variant per chunk-impl checklist rule 3). The struct-vs-enum disagreement with sibling 8B (its line 111 destructure pattern) is resolved by this AC item plus the cross-chunk amendment item above.
- [ ] PR body enumerates each load-bearing design decision (`OuterWrap` distinguishes the two corpus-evidenced double-paren trailer-position shapes via typed variants; `Unclassified` is paired with a Finding and forbidden in authoring; `Other(String)` rejected on every enum that could carry it; bare-keyword and bare-learn entries collapse into `Composition` with single-component bodies; `(ritemx.a348).tier.0.n.…` parses to `Ritemx { outer_wrap: SingleParen, tail_chain: [], … }` with the trailer parsed from post-paren bytes; round-by-round resolved decisions live in `plans/nonsummon-typed-schema-decisions.md`) with the corpus anchor that justifies it.
- [ ] PR body records the §"Phase-A walker outcomes" `double_paren_trailer_inside_observed` boolean verbatim from `target/itempool-shape-audit.txt` (snake_case, matching the audit-file TOTALS line format pinned in §"Audit file format") — if `true`, the variant + T30.13b ride in the same PR; if `false`, neither rides and the PR body cites the audit-file line.

## Review checklist

- [ ] Read `target/itempool-shape-audit.txt` end-to-end; confirm every fingerprint maps to a shipped typed variant or to the documented `Unclassified` zero-occurrence path.
- [ ] Spot-check three random non-summon entries per working mod: extract → IR JSON → re-emit; compare extracted bytes to original (manual byte-diff). No drift in trailer suffix order, no drift in `+`-order of multi-component compositions, no drift in `.draw.` ordering when multiple draws appear.
- [ ] Read every emit arm in `replica_item_emitter.rs`; confirm each one reaches `emit_trailer` / `emit_composition_component` / `emit_img` / `emit_ritemx` / `emit_post_i_chain_siblings` (the last for the SideDef post-`i_chain` `#sidesc` / `#facade` sibling emission) and contains no inline trailer-emit, no inline component-emit, no inline post-i_chain-sibling emit. Pasted-incantation smell check.
- [ ] Read every classifier branch in `classify_non_summon_entry`; confirm none reach for a registry, none `to_lowercase()` / `to_uppercase()`, none consult `SpriteId` or face-compat tables.
- [ ] Read `authoring/non_summon_entry.rs`; confirm the module exports no `Unclassified` constructor (T34 manual witness).
- [ ] Confirm parent plan §3.1 / §3.3 prose update lands in this PR's diff.
- [ ] Re-run `cargo run --example roundtrip_diag` after rebase; confirm all four mods still report `Status: ROUNDTRIP OK`.

## Design decisions

These judgment calls were resolved by the user in conversation and are recorded here as load-bearing per `CLAUDE.md` evidence rule (5). Round-by-round resolved decisions live in `plans/nonsummon-typed-schema-decisions.md`; this section carries only the decisions whose rationale must travel with the implementation prose.

### Extract-time depth check — ship now

`classify_non_summon_entry` reads `MAX_NONSUMMON_DEPTH_OBSERVED` at every recursive descent and returns `Err(CompilerError::DepthExceeded { depth, max })` when exceeded.

Audit-time-only would have constituted deferred correctness against SPEC §3.4 (WASM-readiness) under SPEC §3.7 ("'we'll fix it in a follow-up' are invalid justifications"). The constant is load-bearing — read by both the extractor and T30.22's walker audit. A recursive `depth: u8` parameter threads through every classifier sub-fn; small but non-zero blast radius is the SPEC §3.7 cost.

### Three-sidesc-surfaces invariant — permissive extract, strict authoring

The IR allows `SideDef.sidesc: Some(text1)` AND `i_chain` containing `Scoped { body: Box(Sidesc(text2)) }` simultaneously. Extract never panics on a novel corpus shape that populates both surfaces. The authoring API in `compiler/src/authoring/non_summon_entry.rs` rejects the dual-Some combo at construction time (assert + panic in the SideDef builder when both surfaces are populated).

This matches the `Unclassified` precedent (SPEC §3.2 typed presence + SPEC §3.3 permissive extract + authoring strict-validation): IR-level permissiveness at extract preserves the round-trip invariant on novel shapes; authoring-level strictness keeps the typed builders hallucination-free. Test T30.16c witnesses the authoring-side rejection.

## Out of scope

- 8B's `classify_summon_entry` body (this chunk pins the symbol + signature + `Ok(None)` stub only).
- Typing of `TrailerSuffixKV::Doc(_)` escape sequences (`[n]`, `[grey]`, `[plusfive]`, etc.). The doc payload is human-readable flavor with textmod presentation tokens; no xref rule, no CRUD op, no emit-time structural logic reads into `Doc`. If a future xref rule reaches into the doc bytes, the variant payload becomes a latent SPEC §3.2 surface and must be closed at that time — out of scope here.
- Authoring-side validation of cross-variant invariants (e.g. "a `Composition` with a `Sentinel` component must compose with at least one non-sentinel component"). The authoring layer in this chunk exposes typed builders; semantic validation is xref's job and remains parent §3.3's responsibility.
- New working-mod additions beyond the four current `working-mods/*.txt`. Future mods that surface novel shapes trigger same-PR widening of this chunk's typed-variant set under the retirement protocol; landing those mods is not part of this chunk.
- Rewriting `archive/pre-guide/` content (not authoritative; predates `reference/textmod_guide.md`).
