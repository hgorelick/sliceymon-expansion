/// Opt-out vehicle marker trait for the IR-shape corpus-evidence audit.
///
/// Attaching an `InternalStateEnum` marker to a pub enum `X` reachable from
/// `ModIR` attests: **`X` is NOT input-bytes-determined; `X` is exempt from
/// the corpus-evidence walk.** The audit's default for every Layer-1 pub enum
/// in audit scope that is also Layer-2-reachable from `ModIR` is: must be
/// corpus-evidenced, registry-listed, or brief-deferred. Per `decisions.md`
/// 2026-05-14 "Vehicle marker semantics (opt-out, not opt-in)".
///
/// Vehicle-attachment is class-defined: every chunk that introduces a new
/// `ModIR`-reachable internal-state pub enum attaches the marker near that
/// enum's declaration. The trait carries no methods, no associated types, no
/// super-trait bounds, and no generic parameters by design (`decisions.md`
/// 2026-05-13 "Audit chunk implementation form"); the empty body is the
/// contract.
///
/// **Per-impl rustdoc obligation is the only check on attestation
/// correctness.** `audit-harness`'s `syn`-walker mechanically enforces
/// *presence* of at least one outer `#[doc = ...]` attribute (`AttrStyle::Outer`
/// AND the attribute path resolves to `doc`) on every marker impl-block
/// (`decisions.md` 2026-05-15 point 2). The doc-comment's *content* is
/// human-review-only — an empty or vacuous attestation ships a silent bypass
/// and is exactly what review must catch.
///
/// **Outer-`///` placement is mandatory.** Place the attestation comment
/// immediately above the impl-block's opening line. Inner `//!` doc-comments
/// inside the impl block attach with `AttrStyle::Inner` and do NOT satisfy
/// the attestation; `///` doc-comments placed between `{` and `}` either are
/// a parse error (empty impl bodies have no next item) or attach to a nested
/// item that does not exist here.
///
/// **Walker match key is the trait path's last segment**, not bare-identifier
/// text (`decisions.md` 2026-05-15 point 1). Both the bare attachment form and
/// path-qualified attachment forms (via `crate::audit_markers::` or
/// `textmod_compiler::audit_markers::`) resolve under the same
/// `syn::Path::segments.last()` match key. The project convention is the bare
/// form via the canonical crate-root re-export path
/// `textmod_compiler::InternalStateEnum` (`decisions.md` 2026-05-15 point 3);
/// path-qualified forms remain valid Rust.
pub trait InternalStateEnum {}
