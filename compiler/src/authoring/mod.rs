#![deny(missing_docs)]
//! Authoring layer — the only supported path from human/LLM intent to an IR value.
//!
//! Per SPEC §6.1, this module is the single entry point for constructing IR programmatically
//! (Path B / Path C). It exists to make hallucinated values (invalid Face IDs, unknown sprite
//! names, impossible container shapes) a compile error rather than a runtime surprise.
//!
//! Submodules: `face_id` (`FaceId` / `FaceIdValue` / `Pips`), `sprite` (`SpriteId` newtype +
//! build-time registry), `replica_item` (`SideUseBuilder` / `CastBuilder`).

pub mod face_id;
pub mod replica_item;
pub mod sprite;

pub use face_id::{FaceId, FaceIdError, FaceIdValue, Pips, KNOWN_FACE_IDS};
pub use replica_item::{CastBuilder, HasDice, NoDice, SideUseBuilder};
pub use sprite::SpriteId;
