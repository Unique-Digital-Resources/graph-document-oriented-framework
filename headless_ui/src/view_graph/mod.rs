//! The View Graph: a separate subgraph that holds UI nodes.
//!
//! The View Graph is structurally a `Tree` (UI is hierarchical), but it
//! also tracks *bindings* — one-way references from UI nodes to document
//! (data) nodes. Document nodes never reference UI nodes.

pub mod storage;
pub mod bindings;

pub use storage::ViewGraph;
pub use bindings::{Binding, BindingRegistry};