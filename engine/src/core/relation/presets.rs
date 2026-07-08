//! High-level relation presets ("macros").
//!
//! A `RelationSchema` is a frozen bundle of the seven low-level enums
//! plus a name. Presets like `CHILDREN`, `REFERENCE`, `DEPENDENCY`
//! are the *only* thing node declarations should reference:
//!
//! ```text
//! define node TaskNode {
//!     subtasks: relation CHILDREN -> TaskNode
//!     assignee: relation REFERENCE -> UserNode
//!     blockedBy: relation DEPENDENCY -> TaskNode
//! }
//! ```
//!
//! Plugins may register additional presets (e.g. `BLOCKED_BY`) by
//! calling `RelationPresetRegistry::register` with a custom
//! `RelationSchema`. This is exactly the mechanism the DSL
//! `define relation BLOCKED_BY { ... }` block compiles down to.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::enums::{
    Cardinality, Evaluation, Lifetime, Ownership, Persistence, Propagation, Topology,
};

/// The complete behavioral specification of a named relation type.
///
/// `RelationSchema` instances are immutable once registered and serve
/// as the cache key the graph validator uses to decide whether a
/// candidate edge is legal (e.g. "does adding this edge create a
/// cycle in a DAG?").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelationSchema {
    /// `"CHILDREN"`, `"REFERENCE"`, `"DEPENDENCY"`, `"BLOCKED_BY"`, …
    pub name: String,
    pub topology: Topology,
    pub cardinality: Cardinality,
    pub ownership: Ownership,
    pub propagation: Propagation,
    pub evaluation: Evaluation,
    pub lifetime: Lifetime,
    pub persistence: Persistence,
}

impl RelationSchema {
    /// Begin a builder. Defaults are the most permissive settings so
    /// that forgetting to set a field never silently over-constrains.
    pub fn builder(name: impl Into<String>) -> RelationSchemaBuilder {
        RelationSchemaBuilder {
            name: name.into(),
            topology: Topology::Graph,
            cardinality: Cardinality::ManyToMany,
            ownership: Ownership::None,
            propagation: Propagation::None,
            evaluation: Evaluation::Immediate,
            lifetime: Lifetime::Persistent,
            persistence: Persistence::Saved,
        }
    }
}

/// Fluent builder used both by the preset constructors below and by
/// the DSL compiler when it encounters a `define relation FOO { … }`
/// block.
#[derive(Debug, Clone)]
pub struct RelationSchemaBuilder {
    name: String,
    topology: Topology,
    cardinality: Cardinality,
    ownership: Ownership,
    propagation: Propagation,
    evaluation: Evaluation,
    lifetime: Lifetime,
    persistence: Persistence,
}

impl RelationSchemaBuilder {
    pub fn topology(mut self, v: Topology) -> Self {
        self.topology = v;
        self
    }
    pub fn cardinality(mut self, v: Cardinality) -> Self {
        self.cardinality = v;
        self
    }
    pub fn ownership(mut self, v: Ownership) -> Self {
        self.ownership = v;
        self
    }
    pub fn propagation(mut self, v: Propagation) -> Self {
        self.propagation = v;
        self
    }
    pub fn evaluation(mut self, v: Evaluation) -> Self {
        self.evaluation = v;
        self
    }
    pub fn lifetime(mut self, v: Lifetime) -> Self {
        self.lifetime = v;
        self
    }
    pub fn persistence(mut self, v: Persistence) -> Self {
        self.persistence = v;
        self
    }

    #[must_use]
    pub fn build(self) -> RelationSchema {
        RelationSchema {
            name: self.name,
            topology: self.topology,
            cardinality: self.cardinality,
            ownership: self.ownership,
            propagation: self.propagation,
            evaluation: self.evaluation,
            lifetime: self.lifetime,
            persistence: self.persistence,
        }
    }
}

// ---------------------------------------------------------------------
// Core presets — mirror the DSL spec verbatim.
// ---------------------------------------------------------------------

/// `CHILDREN` — parent owns its children.
///
/// ```text
/// define relation CHILDREN {
///     topology:    Tree
///     cardinality: 1:N
///     ownership:   Containment      // Parent owns the child's lifecycle
///     propagation: Forward          // Changes in parent affect children
///     evaluation:  Immediate
///     lifetime:    Persistent
///     persistence: Saved
/// }
/// ```
///
/// Graph consequences:
/// - Deleting a parent cascades to children (`Containment`).
/// - A child cannot have two parents (`Tree`).
/// - Mutating the parent marks children dirty (`Forward`).
pub fn children() -> RelationSchema {
    RelationSchema::builder("CHILDREN")
        .topology(Topology::Tree)
        .cardinality(Cardinality::OneToMany)
        .ownership(Ownership::Containment)
        .propagation(Propagation::Forward)
        .evaluation(Evaluation::Immediate)
        .lifetime(Lifetime::Persistent)
        .persistence(Persistence::Saved)
        .build()
}

/// `REFERENCE` — weak pointer to another node.
///
/// ```text
/// define relation REFERENCE {
///     topology:    Graph             // Can point anywhere, even cycles
///     cardinality: 1:1
///     ownership:   None              // Does not own the target
///     propagation: None
///     evaluation:  Lazy
///     lifetime:    Weak              // Target deletion nulls the reference
///     persistence: Saved
/// }
/// ```
pub fn reference() -> RelationSchema {
    RelationSchema::builder("REFERENCE")
        .topology(Topology::Graph)
        .cardinality(Cardinality::OneToOne)
        .ownership(Ownership::None)
        .propagation(Propagation::None)
        .evaluation(Evaluation::Lazy)
        .lifetime(Lifetime::Weak)
        .persistence(Persistence::Saved)
        .build()
}

/// `DEPENDENCY` — generic "depends-on" edge.
///
/// ```text
/// define relation DEPENDENCY {
///     topology:    DAG               // Must be acyclic
///     cardinality: N:M
///     ownership:   None
///     propagation: Backward          // Target state affects source
///     evaluation:  Deferred
///     lifetime:    Persistent
///     persistence: Saved
/// }
/// ```
pub fn dependency() -> RelationSchema {
    RelationSchema::builder("DEPENDENCY")
        .topology(Topology::DAG)
        .cardinality(Cardinality::ManyToMany)
        .ownership(Ownership::None)
        .propagation(Propagation::Backward)
        .evaluation(Evaluation::Deferred)
        .lifetime(Lifetime::Persistent)
        .persistence(Persistence::Saved)
        .build()
}

/// `BLOCKED_BY` — the canonical plugin-defined preset from the
/// developer-experience doc.
///
/// Semantically a specialization of `DEPENDENCY` (DAG + Backward +
/// N:M) but given a distinct name so the DSL can spell
/// `blockedBy: relation BLOCKED_BY -> TaskNode`. Demonstrates the
/// pattern a plugin follows to ship its own high-level relation.
pub fn blocked_by() -> RelationSchema {
    RelationSchema::builder("BLOCKED_BY")
        .topology(Topology::DAG)
        .cardinality(Cardinality::ManyToMany)
        .ownership(Ownership::None)
        .propagation(Propagation::Backward)
        .evaluation(Evaluation::Deferred)
        .lifetime(Lifetime::Persistent)
        .persistence(Persistence::Saved)
        .build()
}

// ---------------------------------------------------------------------
// Preset registry — the symbol table the DSL parser resolves names
// against. Plugins add entries here at load time.
// ---------------------------------------------------------------------

/// Lookup table mapping preset names (`"CHILDREN"`, …) to their
/// `RelationSchema`. Populated at startup with the four core presets
/// and extended by plugins via `register`.
#[derive(Debug, Clone, Default)]
pub struct RelationPresetRegistry {
    schemas: HashMap<String, RelationSchema>,
}

impl RelationPresetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pre-loaded with `CHILDREN`, `REFERENCE`, `DEPENDENCY`,
    /// `BLOCKED_BY`. The framework bootstraps with this; plugins
    /// add more.
    pub fn with_core_presets() -> Self {
        let mut me = Self::new();
        me.register(children());
        me.register(reference());
        me.register(dependency());
        me.register(blocked_by());
        me
    }

    /// Register a preset. If a preset with the same name already
    /// exists it is replaced. Plugins should call this during their
    /// `initialize()` lifecycle hook (Phase 4).
    pub fn register(&mut self, schema: RelationSchema) {
        self.schemas.insert(schema.name.clone(), schema);
    }

    pub fn get(&self, name: &str) -> Option<&RelationSchema> {
        self.schemas.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.schemas.contains_key(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &RelationSchema)> {
        self.schemas.iter()
    }

    pub fn len(&self) -> usize {
        self.schemas.len()
    }

    pub fn is_empty(&self) -> bool {
        self.schemas.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn children_matches_spec_exactly() {
        let s = children();
        assert_eq!(s.name, "CHILDREN");
        assert_eq!(s.topology, Topology::Tree);
        assert_eq!(s.cardinality, Cardinality::OneToMany);
        assert_eq!(s.ownership, Ownership::Containment);
        assert_eq!(s.propagation, Propagation::Forward);
        assert_eq!(s.evaluation, Evaluation::Immediate);
        assert_eq!(s.lifetime, Lifetime::Persistent);
        assert_eq!(s.persistence, Persistence::Saved);
    }

    #[test]
    fn reference_is_weak_and_allows_cycles() {
        let s = reference();
        assert!(s.topology.allows_cycles());
        assert!(s.lifetime.is_weak());
        assert!(!s.ownership.owns_target());
        assert_eq!(s.cardinality, Cardinality::OneToOne);
    }

    #[test]
    fn dependency_is_acyclic_and_backwards() {
        let s = dependency();
        assert!(!s.topology.allows_cycles());
        assert_eq!(s.propagation, Propagation::Backward);
        assert_eq!(s.cardinality, Cardinality::ManyToMany);
    }

    #[test]
    fn blocked_by_is_a_dependency_specialization() {
        let s = blocked_by();
        let d = dependency();
        // Same semantics, different name.
        assert_eq!(s.topology, d.topology);
        assert_eq!(s.cardinality, d.cardinality);
        assert_eq!(s.propagation, d.propagation);
        assert_eq!(s.evaluation, d.evaluation);
        assert_ne!(s.name, d.name);
    }

    #[test]
    fn builder_can_produce_custom_preset() {
        // A plugin does this:
        let schema = RelationSchema::builder("REQUIRES")
            .topology(Topology::DAG)
            .cardinality(Cardinality::ManyToMany)
            .propagation(Propagation::Backward)
            .build();
        assert_eq!(schema.name, "REQUIRES");
        assert_eq!(schema.evaluation, Evaluation::Immediate); // default retained
    }

    #[test]
    fn registry_resolves_core_presets() {
        let reg = RelationPresetRegistry::with_core_presets();
        assert_eq!(reg.len(), 4);
        assert!(reg.contains("CHILDREN"));
        assert!(reg.contains("REFERENCE"));
        assert!(reg.contains("DEPENDENCY"));
        assert!(reg.contains("BLOCKED_BY"));
        assert!(!reg.contains("NON_EXISTENT"));

        let s = reg.get("CHILDREN").unwrap();
        assert_eq!(s.ownership, Ownership::Containment);
    }

    #[test]
    fn registry_supports_plugin_registration() {
        let mut reg = RelationPresetRegistry::with_core_presets();
        let custom = RelationSchema::builder("HOVER_LINK")
            .topology(Topology::Graph)
            .cardinality(Cardinality::OneToOne)
            .lifetime(Lifetime::Weak)
            .persistence(Persistence::Transient)
            .build();
        reg.register(custom);

        assert!(reg.contains("HOVER_LINK"));
        let s = reg.get("HOVER_LINK").unwrap();
        assert_eq!(s.persistence, Persistence::Transient);
    }

    #[test]
    fn registry_overwrite_replaces_existing() {
        let mut reg = RelationPresetRegistry::new();
        reg.register(children());
        let original = reg.get("CHILDREN").unwrap().clone();

        // A plugin redefines CHILDREN with different semantics.
        let replacement = RelationSchema::builder("CHILDREN")
            .topology(Topology::DAG)
            .cardinality(Cardinality::ManyToMany)
            .build();
        reg.register(replacement);

        let now = reg.get("CHILDREN").unwrap();
        assert_ne!(now.topology, original.topology);
        assert_eq!(now.topology, Topology::DAG);
    }
}