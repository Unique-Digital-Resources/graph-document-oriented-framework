//! # Graph Document Framework
//!
//! A general-purpose, document-oriented application framework built around a
//! dependency graph and five first-class pillars:
//!
//! 1. **Nodes**           — data containers (this crate)
//! 2. **Signals**         — event bus (Phase 2)
//! 3. **Commands**        — the only mutators of the graph (Phase 2)
//! 4. **Dependency Graph** — relations, traversal, scheduler (Phase 1 + 3)
//! 5. **Plugin System**   — extensibility (Phase 4)
//!
//! ## Architectural invariants (from PRD §"Architectural Rules")
//!
//! - The graph is **immutable except through the Command pipeline**.
//! - Invalidation is push-based (reactive); recomputation is pull-based (scheduled).
//! - Commands = intent (future), Signals = fact (past), Systems = continuous (present).
//! - The **Document** is the atomic unit of persistence.
//! - UI nodes reference data nodes; data nodes never reference UI nodes.

pub mod core;
pub mod io;
pub mod plugin;
pub mod scripting;
//pub mod ui;

pub use core::{command, graph, history, node, relation, scheduler, signal, system};