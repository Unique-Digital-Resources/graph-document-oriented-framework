//! Transactions and Macro batching.

use super::snapshot::GraphSnapshot;
use crate::core::command::registry::UndoFn;
use serde_json::Value;

/// A single undoable step within a transaction.
pub struct CommandStep {
    pub undo_fn: UndoFn,
    pub params: Value,
}

/// An active transaction. Collects steps and snapshots until committed.
pub struct Transaction {
    pub label: String,
    pub steps: Vec<CommandStep>,
    pub snapshot: GraphSnapshot,
    pub is_active: bool,
}

impl Transaction {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            steps: Vec::new(),
            snapshot: GraphSnapshot::new(),
            is_active: true,
        }
    }

    pub fn add_step(&mut self, undo_fn: UndoFn, params: Value) {
        self.steps.push(CommandStep { undo_fn, params });
    }

    pub fn capture(&mut self, graph: &crate::core::graph::Graph, id: crate::core::node::NodeId) {
        self.snapshot.capture(graph, id);
    }

    /// Finalize the transaction into a HistoryEntry.
    pub fn commit(self) -> super::stack::HistoryEntry {
        super::stack::HistoryEntry {
            label: self.label,
            steps: self.steps,
            snapshot: self.snapshot,
        }
    }
}