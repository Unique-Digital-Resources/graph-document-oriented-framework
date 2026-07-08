//! Pillar 3 (Sub-system) — History & Transactions.
//!
//! Manages the Undo/Redo stacks and groups multiple command executions
//! into a single, atomic history step via Transactions.

pub mod r#macro;
pub mod snapshot;
pub mod stack;

pub use r#macro::Transaction;
pub use snapshot::GraphSnapshot;
pub use stack::{HistoryEntry, HistoryStack};