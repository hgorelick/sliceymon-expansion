# Pre-guide archive (frozen 2026-04-20)

Everything in this directory was created **before** `reference/textmod_guide.md` was added to the project on 2026-04-10 (commit `3c5d741`). The guide is now the authoritative format spec — when it disagrees with anything in this archive, the guide wins.

These files are kept solely for project archaeology: how we got from the original Sliceymon mod to the current Rust compiler. **Do not cite anything here as a source of truth.** In particular:

- `SLICEYMON_AUDIT.md` frames everything by line numbers in the old `textmod.txt`. Face IDs and property codes here may be wrong, incomplete, or have been re-classified by the parser.
- `tools/*.js` is the dead JS tooling pipeline (validator, hero generator, sprite encoder, assembler). The Rust compiler in `compiler/` replaces all of it.
- `tools/sprite_encodings.json` and `tools/sprites/` are the pre-guide sprite registry. The compiler now harvests sprite encodings from `working-mods/*.txt` directly.
- `generated/` are the pre-compiler hand-generated hero lines. Almost certainly out of date vs the guide.
- `textmod.txt`, `textmod_expanded.txt`, `textmod_heroes_only.txt` are the pre-compiler hand-assembled mods. Use `working-mods/sliceymon.txt` (byte-identical to the original `textmod.txt`) and the compiler's build output instead.
- `plans/` contains pre-guide plans (`EXPANSION_PLAN`, `FULL_ROSTER`, `hero_designs_batch*`, `monster_boss_designs`, `BUILDER_PLAN`, `OVERHAUL_PLAN`, `OVERHAUL_NOTES`, `SPELL_REDESIGN`, `TEMPLATE_PROPERTIES`, `UNIFIED_HERO_SCHEMA_PLAN`, `UNIFIED_MOD_SCHEMA_PLAN`, `VALIDATOR_PLAN`, `IDEAS`, `COMPILER_FIX_PLAN`, `SLICEYMON_PLUS_PLAN`). Designs and Face IDs may be wrong; the workflow they describe (hand-edit `textmod_expanded.txt`, run `node tools/*.js`) is dead.

Active sources of truth live outside this directory:

- `reference/textmod_guide.md` — format spec
- `compiler/src/` — extractor → IR → builder, plus `xref.rs` for cross-reference semantic checks
- `working-mods/*.txt` — the four roundtrip-target mods
- `plans/AUTHORING_ERGONOMICS_PLAN.md`, `plans/PIPELINE_FIDELITY_PLAN.md`, `plans/TEXTMOD_API_INTEGRATION_PLAN.md` — current plans
