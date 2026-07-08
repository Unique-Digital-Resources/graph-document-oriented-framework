//! Pillar 4 (data-model slice) — Relations.
//!
//! Two layers:
//!
//! - `enums`    — low-level dimensions (`Topology`, `Cardinality`, …).
//!                Developers **never** use these directly in node
//!                declarations.
//! - `presets`  — high-level macros (`CHILDREN`, `REFERENCE`, …)
//!                built by combining the low-level enums. Developers
//!                write `relation CHILDREN -> TaskNode` in the DSL and
//!                the parser looks the name up here.
//!
//! The graph storage / validation / traversal built on top of these
//! lives in `core/graph/*` (separate Phase-1 files).

pub mod enums;
pub mod presets;

pub use enums::{
    Cardinality, Evaluation, Lifetime, Ownership, Persistence, Propagation, Topology,
};
pub use presets::{
    blocked_by, children, dependency, reference, RelationPresetRegistry, RelationSchema,
    RelationSchemaBuilder,
};