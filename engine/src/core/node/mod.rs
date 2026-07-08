//! Pillar 1 — Nodes.
//!
//! A `Node` is a pure data container: an ID, a type, properties, local
//! relation pointers, and metadata. Nodes hold **no behavior**; behavior
//! lives in Commands (Pillar 3).

pub mod metadata;
pub mod node;
pub mod properties;

pub use metadata::Metadata;
pub use node::{LocalRelations, Node, NodeId, TypeId};
pub use properties::{Properties, Property, PropertyKind, PropertyValue};