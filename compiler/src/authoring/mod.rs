//! Authoring layer — the only supported path from human/LLM intent to an IR value.
//!
//! Per SPEC §6.1, this module is the single entry point for constructing IR programmatically
//! (Path B / Path C). It exists to make hallucinated values (invalid Face IDs, unknown sprite
//! names, impossible container shapes) a compile error rather than a runtime surprise.
//!
//! This module is currently a skeleton. Subsequent chunks of `PLATFORM_FOUNDATIONS_PLAN.md`
//! populate it with `FaceId` / `Pips` / `SpriteId` newtypes and their registries;
//! `AUTHORING_ERGONOMICS_PLAN.md` adds the chainable builders and `HeroReplica` on top.
