# Engineering Personas

> **Spec**: [`SPEC.md`](../SPEC.md) is the project source of truth. Read it before invoking any persona — every persona's guidance is scoped to and bounded by the invariants defined there.

This folder contains AI persona files representing expert principal engineers with different areas of expertise, tailored for the Sliceymon+ textmod compiler project. Reference these personas individually or in combination to get specialized guidance.

## Available Personas

| Persona | File | Expertise |
|---------|------|-----------|
| Architecture | [architecture.md](architecture.md) | Compiler pipeline design, IR design, module boundaries, WASM portability |
| Rust Engineer | [backend.md](backend.md) | Rust implementation, parser/emitter patterns, serde, error handling, CLI |
| Code Reviewer | [code-reviewer.md](code-reviewer.md) | Format correctness, round-trip fidelity, parser edge cases, Rust safety |
| Frontend / WASM | [frontend.md](frontend.md) | Rust-to-WASM compilation, browser mod builder, wasm-bindgen, JS interop |
| Security | [security.md](security.md) | Input validation, parser robustness, WASM safety, supply chain, fuzzing |
| Testing | [testing.md](testing.md) | TDD progression, round-trip testing, Rust test patterns, assertion quality |
| AI Development | [ai-development.md](ai-development.md) | Prompt engineering, one-shot completion, chunked plans, AI verification |
| Game Design | [slice-and-dice-design.md](slice-and-dice-design.md) | Slice & Dice mechanics, balance, tier budgets, Face IDs, keywords |

## Project Context

**What we're building**: A Rust textmod compiler that parses Slice & Dice textmods into a structured IR and compiles an IR back into valid, pasteable textmod text. Compiles to WASM for a future browser-based mod builder.

**Why Rust**: Strong type system for IR correctness, WASM compilation target, proper CLI tool.

**Format spec**: `reference/textmod_guide.md`. **IR schema**: `compiler/src/ir/mod.rs`.

## How to Use

### Individual Persona

```
Use the Rust engineer persona from personas/backend.md to help me implement the hero parser.
```

### Combined Personas

```
Use the architecture and testing personas to help me design the round-trip test strategy.
```

### With Task Context

```
Use the Rust engineer persona.

Task: Implement capture_parser.rs

Files to Read First:
- working-mods/sliceymon.txt (replica-item modifier lines)
- reference/textmod_guide.md (property codes, summon-envelope semantics)
- compiler/src/ir/mod.rs (ReplicaItem IR type, SummonTrigger enum)

Requirements:
- Parse replica-item modifiers into ReplicaItem structs (trigger: SummonTrigger)
- Handle all replica-item entries in sliceymon.txt

Constraints:
- Return Result, never panic
- No std::fs (WASM-safe)

Verification:
- [ ] All sliceymon replica items parse successfully
- [ ] Round-trip: parse -> emit -> parse produces identical ReplicaItem
```

## Recommended Combinations

| Task | Personas | Key Files |
|------|----------|-----------|
| Implement a parser | Rust engineer | working-mods/*.txt, reference/textmod_guide.md |
| Implement an emitter | Rust engineer + code reviewer | working-mods/*.txt, reference/textmod_guide.md |
| Design IR types | Architecture | compiler/src/ir/mod.rs, reference/textmod_guide.md |
| Write tests | Testing + Rust engineer | working-mods/*.txt, compiler/tests/ |
| Review parser code | Code reviewer | working-mods/*.txt (roundtrip across all 4 mods) |
| Plan WASM integration | Frontend + architecture | compiler/src/lib.rs |
| Audit dependencies | Security | compiler/Cargo.toml, compiler/Cargo.lock |
| Fuzz the parser | Security + testing | compiler/src/extractor/ |
| Design hero/monster/boss | Game design | reference/textmod_guide.md, personas/slice-and-dice-design.md |
| Plan a new phase | AI development | reference/textmod_guide.md (phase syntax), compiler/src/extractor/phase_parser.rs |

## Source of Truth Files

| File | Purpose | When to Read |
|------|---------|--------------|
| `reference/textmod_guide.md` | Textmod format spec — Face IDs, property codes, semantics | All parser/emitter work |
| `compiler/src/ir/mod.rs` | IR type definitions — the mod schema | Architecture/implementation work |
| `compiler/src/{extractor,builder}/` | How the IR maps to/from textmod | Parser/emitter work |
| `working-mods/*.txt` | Four reference mods (pansaer, punpuns, sliceymon, community) | Testing, round-trip validation, sprite corpus |
| `CLAUDE.md` | Working principles, correctness bar | All work |

For active plans (which go stale once executed), see `plans/`. Use them as roadmaps, not sources of truth.

## Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Language | Rust | Compiler implementation |
| Serialization | serde + serde_json | IR <-> JSON |
| CLI | clap (derive) | extract / build subcommands |
| File matching | glob | IR directory traversal |
| WASM target | wasm-bindgen (future) | Browser mod builder |
| Test harness | cargo test + assert_cmd | Unit + integration + CLI tests |

## Key Architectural Invariant

```
extract(build(extract(mod))) == extract(mod)
```

Semantic IR comparison, not string diff. Tested against all three working mods. This is the single most important property of the compiler — every persona enforces it from their perspective.

## Maintaining Personas

Update personas when:

| Trigger | Action |
|---------|--------|
| New Rust patterns established | Add to Rust engineer examples |
| New IR types added | Update architecture and testing |
| New test mods added | Update testing and code reviewer |
| WASM build issues discovered | Update frontend and security |
| Parser edge cases found | Add to code reviewer |
| AI failure patterns | Document in relevant persona |
