# Engineering Personas

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

**Design doc**: `plans/BUILDER_PLAN.md`

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
- working-mods/sliceymon.txt (capture lines)
- SLICEYMON_AUDIT.md (property codes)
- plans/BUILDER_PLAN.md (Capture IR type)

Requirements:
- Parse capture modifiers into Capture structs
- Handle all captures in sliceymon.txt

Constraints:
- Return Result, never panic
- No std::fs (WASM-safe)

Verification:
- [ ] All sliceymon captures parse successfully
- [ ] Round-trip: parse -> emit -> parse produces identical Capture
```

## Recommended Combinations

| Task | Personas | Key Files |
|------|----------|-----------|
| Implement a parser | Rust engineer | working-mods/*.txt, SLICEYMON_AUDIT.md |
| Implement an emitter | Rust engineer + code reviewer | textmod.txt, BUILDER_PLAN.md |
| Design IR types | Architecture | BUILDER_PLAN.md, SLICEYMON_AUDIT.md |
| Write tests | Testing + Rust engineer | working-mods/*.txt, BUILDER_PLAN.md |
| Review parser code | Code reviewer | working-mods/*.txt (test against all 3 mods) |
| Plan WASM integration | Frontend + architecture | compiler/src/lib.rs |
| Audit dependencies | Security | Cargo.toml, Cargo.lock |
| Fuzz the parser | Security + testing | compiler/src/extractor/ |
| Design hero balance | Game design | plans/hero_designs_batch*.md |
| Plan a new phase | AI development | BUILDER_PLAN.md |

## Source of Truth Files

| File | Purpose | When to Read |
|------|---------|--------------|
| `plans/BUILDER_PLAN.md` | Compiler design, IR types, TDD phases | All architecture/implementation work |
| `SLICEYMON_AUDIT.md` | Textmod format, property codes, Face IDs | Parser/emitter work |
| `textmod.txt` | Original mod baseline | Understanding correct output format |
| `working-mods/*.txt` | Three test mods (pansaer, punpuns, sliceymon) | Testing, round-trip validation |
| `tools/sprite_encodings.json` | Sprite name -> encoding mapping | Builder sprite resolution |
| `CLAUDE.md` | Format rules, validation requirements | All mod content work |
| `plans/hero_designs_batch*.md` | Hero designs for Sliceymon+ | Hero implementation |

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
