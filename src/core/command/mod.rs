//! Pillar 3 — Commands.
//!
//! Commands are the *only* way to mutate the graph. They represent
//! executable behavior and contain the execution pipeline.

pub mod pipeline;
pub mod registry;
pub mod transaction;

pub use pipeline::CommandPipeline;
pub use registry::{CommandDefinition, CommandRegistry};
pub use transaction::Transaction;