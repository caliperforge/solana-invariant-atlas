//! Thin driver over the pinned `cf-invariants-anchor` crates.
//!
//! The atlas does not fork or copy the library; it consumes the
//! canonical crates as git dependencies pinned at a single rev (see
//! `docs/design_notes.md`). This crate exposes exactly the three
//! operations the atlas references and docs need, so nothing here
//! duplicates upstream logic.

use cf_invariants_anchor_core::{ContractSurface, InvariantCandidate, Scorecard};
use cf_invariants_anchor_emit::Target;
use cf_invariants_anchor_suggest::ClassRegistry;

/// Suggest ranked invariant candidates for a parsed Anchor surface,
/// using the default (heuristic) class registry. No AI call.
pub fn suggest(surface: &ContractSurface) -> Vec<InvariantCandidate> {
    ClassRegistry::default().propose_all(surface)
}

/// Parse an emit-target name. Accepted: `crucible`, `trident`.
pub fn parse_target(name: &str) -> Option<Target> {
    match name {
        "crucible" => Some(Target::Crucible),
        "trident" => Some(Target::Trident),
        _ => None,
    }
}

/// Render one candidate as fixture source for the given target.
pub fn render(surface: &ContractSurface, candidate: &InvariantCandidate, target: Target) -> String {
    cf_invariants_anchor_emit::render(surface, candidate, target)
}

/// Render a scorecard as markdown (used to post-process CI artifacts).
pub fn scorecard_markdown(scorecard: &Scorecard) -> String {
    cf_invariants_anchor_report::render_markdown(scorecard)
}
