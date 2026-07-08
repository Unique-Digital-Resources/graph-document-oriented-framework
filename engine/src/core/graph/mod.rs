//! Pillar 4 (Data-Model Slice) — The Dependency Graph.
//!
//! The `Graph` is the single source of truth for node storage and edge
//! topology. While individual `Node`s hold a local cache of their
//! outgoing edges (`LocalRelations`), the `Graph` holds the master
//! bidirectional edge list and enforces `RelationSchema` constraints.

pub mod index;
pub mod queries;
pub mod storage;
pub mod traversal;
pub mod validation;

pub use storage::{Graph, GraphError};