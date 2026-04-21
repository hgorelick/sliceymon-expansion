#![deny(missing_docs)]
//! Authoring layer — the only supported path from human/LLM intent to an IR value.
//!
//! Per SPEC §6.1, this module is the single entry point for constructing IR programmatically
//! (Path B / Path C). It exists to make hallucinated values (invalid Face IDs, unknown sprite
//! names, impossible container shapes) a compile error rather than a runtime surprise.
//!
//! Chunk 2 populates the `face_id` submodule (`FaceId`, `FaceIdValue`, `Pips`).
//! Subsequent chunks add `SpriteId` + chainable builders per
//! `PLATFORM_FOUNDATIONS_PLAN.md` / `AUTHORING_ERGONOMICS_PLAN.md`.

pub mod face_id;

pub use face_id::{FaceId, FaceIdError, FaceIdValue, Pips, KNOWN_FACE_IDS};
