//! `FaceId` + `Pips` newtypes for the authoring layer (SPEC §3.6, §6.1).
//!
//! `FaceId` wraps a `u16` and is constructed either via a generated `pub const`
//! (authoring path — hallucination-free) or via `try_new`, which returns
//! `FaceIdValue::Known` for corpus-known IDs and `FaceIdValue::Unknown(raw)` for
//! anything else. The permissive `Unknown` variant preserves SPEC §3.3
//! (any valid textmod extracts) while the authoring-visible consts stay
//! restricted to the corpus-derived whitelist.
//!
//! `Pips` wraps `i16`. The corpus contains negative pips (`13--1`), so the
//! newtype accepts the full `i16` range with no corpus-derived bound.

use serde::{Deserialize, Serialize};

/// A corpus-whitelisted FaceID. The tuple field is private to this module;
/// callers construct values via the generated `FaceId::*` consts (authoring
/// path), `FaceIdValue::try_new` (permissive parse), or `FaceId::try_from`
/// (strict parse). Direct tuple-literal construction is impossible outside
/// `authoring::face_id`, which is the point — it makes hallucinated IDs a
/// compile error per SPEC §6.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "u16", into = "u16")]
pub struct FaceId(u16);

impl FaceId {
    /// Raw `u16` accessor. The tuple field is private so that `FaceId(N)` can
    /// only be constructed from within this module — i.e. via the generated
    /// consts or the `FaceIdValue::try_new` path.
    pub const fn raw(self) -> u16 {
        self.0
    }
}

/// Corpus-known faceIDs get `FaceId` consts; unknown-but-format-valid IDs are
/// carried through as `Unknown(raw)` so extraction can never fail a roundtrip
/// on a never-before-seen FaceID. Authoring callers work with `FaceId` directly
/// and can only produce `Known` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "u16", into = "u16")]
pub enum FaceIdValue {
    /// FaceID that was in the corpus whitelist at build time.
    Known(FaceId),
    /// FaceID that is not in the whitelist but is valid per the textmod format.
    /// Preserves byte-identical roundtrip; `xref` emits an `X017` warning.
    Unknown(u16),
}

impl FaceIdValue {
    /// Resolve a raw `u16` to `Known(FaceId)` if it is in the corpus whitelist,
    /// else `Unknown(raw)`. Never fails — the whitelist is advisory.
    pub fn try_new(raw: u16) -> Self {
        if KNOWN_FACE_IDS.binary_search(&raw).is_ok() {
            FaceIdValue::Known(FaceId(raw))
        } else {
            FaceIdValue::Unknown(raw)
        }
    }

    /// Raw `u16` — identical for `Known` and `Unknown`, since the newtype
    /// wraps a single value either way.
    pub const fn raw(self) -> u16 {
        match self {
            FaceIdValue::Known(f) => f.0,
            FaceIdValue::Unknown(r) => r,
        }
    }

    /// `true` iff this value resolved to a corpus-whitelisted `FaceId`.
    pub const fn is_known(self) -> bool {
        matches!(self, FaceIdValue::Known(_))
    }
}

impl From<FaceIdValue> for u16 {
    fn from(v: FaceIdValue) -> u16 {
        v.raw()
    }
}

impl TryFrom<u16> for FaceIdValue {
    type Error = core::convert::Infallible;
    fn try_from(raw: u16) -> Result<Self, Self::Error> {
        Ok(FaceIdValue::try_new(raw))
    }
}

impl From<FaceId> for u16 {
    fn from(f: FaceId) -> u16 {
        f.0
    }
}

impl TryFrom<u16> for FaceId {
    type Error = FaceIdError;
    /// Strict constructor: fails if `raw` is not in the corpus whitelist.
    /// The authoring layer should prefer the generated `FaceId::*` consts over
    /// this path; `TryFrom<u16>` is the serde deserialize entrypoint and the
    /// escape hatch for callers that genuinely have a raw `u16` they believe
    /// is corpus-known (e.g. a round-trip re-hydration).
    fn try_from(raw: u16) -> Result<Self, Self::Error> {
        if KNOWN_FACE_IDS.binary_search(&raw).is_ok() {
            Ok(FaceId(raw))
        } else {
            Err(FaceIdError { raw })
        }
    }
}

/// Error returned by `FaceId::try_from` when `raw` is not in the corpus
/// whitelist. `FaceIdValue::try_new` is the permissive alternative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaceIdError {
    /// The raw `u16` that failed the whitelist check.
    pub raw: u16,
}

impl core::fmt::Display for FaceIdError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "FaceId {} is not in the corpus whitelist; use FaceIdValue::try_new for permissive parsing",
            self.raw
        )
    }
}

impl std::error::Error for FaceIdError {}

// -- Pips --------------------------------------------------------------------

/// Newtype around `i16` for the pip magnitude on a `DiceFace::Active`.
/// The textmod format allows negative pips (`13--1` — FaceID 13, -1 pips)
/// so the underlying type is signed and the newtype imposes no bound
/// beyond `i16::MIN..=i16::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "i16", into = "i16")]
pub struct Pips(i16);

impl Pips {
    /// Accepts the full `i16` range — no corpus-derived bound. Negative pips
    /// exist in the corpus (e.g. `13--1`) and the textmod format does not
    /// constrain pip magnitude. Returns `Result` for API symmetry with
    /// `FaceId::try_from`; the error type is `Infallible`.
    pub const fn try_new(raw: i16) -> Result<Self, core::convert::Infallible> {
        Ok(Pips(raw))
    }

    /// Infallible constructor — use this in authoring paths where the
    /// pip value is known at compile time.
    pub const fn new(raw: i16) -> Self {
        Pips(raw)
    }

    /// Raw `i16` accessor.
    pub const fn raw(self) -> i16 {
        self.0
    }
}

impl From<Pips> for i16 {
    fn from(p: Pips) -> i16 {
        p.0
    }
}

impl TryFrom<i16> for Pips {
    type Error = core::convert::Infallible;
    fn try_from(raw: i16) -> Result<Self, Self::Error> {
        Ok(Pips(raw))
    }
}

// -- Generated consts --------------------------------------------------------
//
// The generator emits `impl FaceId { pub const NAME: FaceId = FaceId(N); ... }`
// plus a `pub const KNOWN_FACE_IDS: &[u16]` slice used for whitelist lookup.
// Textual inclusion (not submodule) is required so `FaceId(N)` can access the
// private tuple field.
include!(concat!(env!("OUT_DIR"), "/face_id_generated.rs"));

// -- Tests -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn face_id_try_new_known() {
        // 34 (DAMAGE_ALL_OR_SELF) is curated + present in the corpus.
        let v = FaceIdValue::try_new(34);
        assert_eq!(v, FaceIdValue::Known(FaceId::DAMAGE_ALL_OR_SELF));
        assert!(v.is_known());
        assert_eq!(v.raw(), 34);
    }

    #[test]
    fn face_id_try_new_unknown() {
        let v = FaceIdValue::try_new(9999);
        assert_eq!(v, FaceIdValue::Unknown(9999));
        assert!(!v.is_known());
        assert_eq!(v.raw(), 9999);
    }

    #[test]
    fn pips_try_new_accepts_i16_range() {
        assert_eq!(Pips::try_new(i16::MIN).unwrap().raw(), i16::MIN);
        assert_eq!(Pips::try_new(-1).unwrap().raw(), -1);
        assert_eq!(Pips::try_new(0).unwrap().raw(), 0);
        assert_eq!(Pips::try_new(i16::MAX).unwrap().raw(), i16::MAX);
    }

    #[test]
    fn known_face_ids_is_sorted_for_binary_search() {
        assert!(KNOWN_FACE_IDS.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn face_id_tryfrom_unknown_errors() {
        let err = FaceId::try_from(9999u16).unwrap_err();
        assert_eq!(err.raw, 9999);
    }

    #[test]
    fn face_id_serde_known_roundtrip() {
        let v = FaceIdValue::Known(FaceId::DAMAGE_BASIC);
        let j = serde_json::to_string(&v).unwrap();
        let back: FaceIdValue = serde_json::from_str(&j).unwrap();
        assert_eq!(v, back);
    }

    #[test]
    fn face_id_serde_unknown_roundtrip() {
        let v = FaceIdValue::Unknown(9999);
        let j = serde_json::to_string(&v).unwrap();
        let back: FaceIdValue = serde_json::from_str(&j).unwrap();
        assert_eq!(v, back);
    }
}
