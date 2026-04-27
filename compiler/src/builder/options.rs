//! Build-time options — source filter over `build_with`.
//!
//! Per the 2026-04-22 "BuildOptions + provenance-aware findings" ruling:
//! `build(ir)` is a thin wrapper over
//! `build_with(ir, &BuildOptions::default())`. Callers that want to emit only
//! a subset of entities (e.g. only overlay additions on top of a base mod)
//! construct a non-default `BuildOptions` with a non-`All` `SourceFilter`.
//!
//! The filter is checked once per content-emission site in `builder/mod.rs`;
//! derived structurals are regenerated from the post-filter content set and
//! do not carry their own filter.

use crate::ir::Source;

/// Options that tune `build_with`. Default admits every entity.
#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub include: SourceFilter,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            include: SourceFilter::All,
        }
    }
}

/// Which `Source` values should be emitted.
#[derive(Debug, Clone)]
pub enum SourceFilter {
    /// Emit every entity regardless of `source`.
    All,
    /// Emit only entities whose `source` is in the set.
    Only(SourceSet),
    /// Emit every entity whose `source` is NOT in the set.
    Exclude(SourceSet),
}

/// Bitmask-backed set of `Source` variants. Fits any subset of
/// `{Base, Custom, Overlay}` in a single byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSet(u8);

impl SourceSet {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn all() -> Self {
        Self(0b111)
    }

    pub const fn single(s: Source) -> Self {
        Self(1 << Self::bit(s))
    }

    pub const fn contains(self, s: Source) -> bool {
        self.0 & (1 << Self::bit(s)) != 0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    const fn bit(s: Source) -> u8 {
        match s {
            Source::Base => 0,
            Source::Custom => 1,
            Source::Overlay => 2,
        }
    }
}

impl FromIterator<Source> for SourceSet {
    fn from_iter<I: IntoIterator<Item = Source>>(iter: I) -> Self {
        iter.into_iter()
            .fold(Self::empty(), |acc, s| acc.union(Self::single(s)))
    }
}

impl SourceFilter {
    /// True if the filter admits entities with the given `Source`.
    pub const fn admits(&self, s: Source) -> bool {
        match self {
            SourceFilter::All => true,
            SourceFilter::Only(set) => set.contains(s),
            SourceFilter::Exclude(set) => !set.contains(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time proof that `SourceFilter::admits` is `const fn`.
    // If admits were downgraded to a non-const fn, this line fails to build.
    const _SOURCE_FILTER_ADMITS_IS_CONST: bool = SourceFilter::All.admits(Source::Base);

    #[test]
    fn source_filter_admits_const() {
        assert!(_SOURCE_FILTER_ADMITS_IS_CONST);
    }

    #[test]
    fn build_options_default_is_all() {
        let opts = BuildOptions::default();
        assert!(opts.include.admits(Source::Base));
        assert!(opts.include.admits(Source::Custom));
        assert!(opts.include.admits(Source::Overlay));
    }

    #[test]
    fn source_set_single_and_union() {
        let s = SourceSet::single(Source::Base);
        assert!(s.contains(Source::Base));
        assert!(!s.contains(Source::Custom));
        assert!(!s.contains(Source::Overlay));

        let both = s.union(SourceSet::single(Source::Custom));
        assert!(both.contains(Source::Base));
        assert!(both.contains(Source::Custom));
        assert!(!both.contains(Source::Overlay));

        assert_eq!(SourceSet::all().0, 0b111);
        assert_eq!(SourceSet::empty().0, 0);
    }

    #[test]
    fn source_set_from_iter() {
        let s: SourceSet = [Source::Base, Source::Overlay].into_iter().collect();
        assert!(s.contains(Source::Base));
        assert!(!s.contains(Source::Custom));
        assert!(s.contains(Source::Overlay));
    }

    #[test]
    fn filter_only_admits_only_members() {
        let f = SourceFilter::Only(SourceSet::single(Source::Custom));
        assert!(!f.admits(Source::Base));
        assert!(f.admits(Source::Custom));
        assert!(!f.admits(Source::Overlay));
    }

    #[test]
    fn filter_exclude_admits_non_members() {
        let f = SourceFilter::Exclude(SourceSet::single(Source::Base));
        assert!(!f.admits(Source::Base));
        assert!(f.admits(Source::Custom));
        assert!(f.admits(Source::Overlay));
    }
}
