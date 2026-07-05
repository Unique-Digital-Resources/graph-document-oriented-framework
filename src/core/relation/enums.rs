//! Low-level relation dimensions.
//!
//! These seven enums are the **orthogonal axes** along which every
//! relation is defined. The DSL never asks developers to spell these
//! out per-relation; instead, a `define relation CHILDREN { ... }`
//! block combines them into a named preset (see `presets.rs`), and
//! node declarations reference the preset name.
//!
//! The enums are deliberately `Copy + Eq + Hash` so they can be used
//! as switch arms and as parts of a `RelationSchema` cache key.

use serde::{Deserialize, Serialize};

/// Shape of the subgraph the relation is allowed to form.
///
/// - `Tree`  — strict hierarchy; every target has at most one source.
/// - `DAG`   — directed, acyclic; targets may have many sources.
/// - `Graph` — anything goes, including cycles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Topology {
    Tree,
    DAG,
    Graph,
}

impl Topology {
    /// `true` if cycles are permitted under this topology.
    #[inline]
    pub fn allows_cycles(self) -> bool {
        matches!(self, Self::Graph)
    }

    /// `true` if a target may have more than one source of this
    /// relation. Trees forbid this; DAGs and Graphs allow it.
    #[inline]
    pub fn allows_multiple_parents(self) -> bool {
        matches!(self, Self::DAG | Self::Graph)
    }
}

/// How many sources may point at how many targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cardinality {
    /// `1:1` — exactly one source ↔ one target.
    OneToOne,
    /// `1:N` — one source ↔ many targets.
    OneToMany,
    /// `N:M` — many sources ↔ many targets.
    ManyToMany,
}

impl Cardinality {
    /// `true` if a single source may have multiple targets.
    #[inline]
    pub fn allows_multiple_targets(self) -> bool {
        matches!(self, Self::OneToMany | Self::ManyToMany)
    }

    /// `true` if a single target may have multiple sources.
    #[inline]
    pub fn allows_multiple_sources(self) -> bool {
        matches!(self, Self::ManyToMany)
    }
}

/// Direction in which signals / dirty marks flow along the edge.
///
/// Used by the scheduler (Phase 3) to decide whom to invalidate
/// when a node mutates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Propagation {
    /// No propagation. (REFERENCE)
    None,
    /// Source → Target. (CHILDREN: parent change affects children)
    Forward,
    /// Target → Source. (BLOCKED_BY: blocker state affects blocked)
    Backward,
    /// Both directions.
    Bidirectional,
}

/// When dependent state is recomputed after a mutation.
///
/// Pairs with `Propagation`: propagation invalidates, evaluation
/// decides *when* the recompute happens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Evaluation {
    /// Synchronous, on the same tick as the mutation.
    Immediate,
    /// Batched into the next scheduler tick.
    Deferred,
    /// Only when something actually queries the dependent value.
    Lazy,
    /// Offloaded to a background thread via the scheduler.
    Async,
}

/// Lifecycle / strength of an edge instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lifetime {
    /// Lives until explicitly removed.
    Persistent,
    /// Can be cancelled mid-flight (e.g. thumbnail generation).
    Cancelable,
    /// Newer edge of the same schema supersedes the older one.
    Replaceable,
    /// Does not keep the target alive; becomes null if the target is
    /// deleted. (REFERENCE)
    Weak,
}

impl Lifetime {
    #[inline]
    pub fn is_weak(self) -> bool {
        matches!(self, Self::Weak)
    }
}

/// Whether the edge itself is written to disk with the document.
///
/// Note: this is the **edge's** persistence, not the target node's.
/// A `REFERENCE` edge is `Saved` even though the target node has its
/// own independent lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Persistence {
    /// Serialized with the document.
    Saved,
    /// In-memory only (e.g. transient UI hover links).
    Transient,
    /// Never stored; recomputed on load.
    Derived,
}

impl Persistence {
    #[inline]
    pub fn is_saved(self) -> bool {
        matches!(self, Self::Saved)
    }
}

/// Who controls the lifecycle of the target with respect to the
/// source. Drives cascade-delete and copy semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ownership {
    /// No ownership. (REFERENCE, DEPENDENCY)
    None,
    /// Source owns the target. Deleting the source deletes the
    /// target. Copying the source copies the target. (CHILDREN)
    Containment,
    /// Co-ownership: target is destroyed when the last shared owner
    /// is removed.
    Shared,
    /// Non-owning borrow; the source may use the target but does not
    /// affect its lifetime. Distinct from `None` in that the borrow
    /// checker (graph validator) treats it as a runtime borrow.
    Borrowed,
}

impl Ownership {
    /// `true` if deleting the source should cascade to the target.
    #[inline]
    pub fn owns_target(self) -> bool {
        matches!(self, Self::Containment)
    }

    /// `true` if the source participates in ownership at all.
    #[inline]
    pub fn is_owner(self) -> bool {
        matches!(self, Self::Containment | Self::Shared)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topology_rules() {
        assert!(!Topology::Tree.allows_cycles());
        assert!(!Topology::Tree.allows_multiple_parents());
        assert!(!Topology::DAG.allows_cycles());
        assert!(Topology::DAG.allows_multiple_parents());
        assert!(Topology::Graph.allows_cycles());
        assert!(Topology::Graph.allows_multiple_parents());
    }

    #[test]
    fn cardinality_rules() {
        assert!(!Cardinality::OneToOne.allows_multiple_targets());
        assert!(!Cardinality::OneToOne.allows_multiple_sources());
        assert!(Cardinality::OneToMany.allows_multiple_targets());
        assert!(!Cardinality::OneToMany.allows_multiple_sources());
        assert!(Cardinality::ManyToMany.allows_multiple_targets());
        assert!(Cardinality::ManyToMany.allows_multiple_sources());
    }

    #[test]
    fn ownership_cascade() {
        assert!(Ownership::Containment.owns_target());
        assert!(!Ownership::None.owns_target());
        assert!(Ownership::Shared.is_owner());
        assert!(!Ownership::Borrowed.is_owner());
    }

    #[test]
    fn persistence_and_lifetime_helpers() {
        assert!(Persistence::Saved.is_saved());
        assert!(!Persistence::Transient.is_saved());
        assert!(Lifetime::Weak.is_weak());
        assert!(!Lifetime::Persistent.is_weak());
    }
}