//! Undo/Redo stack management.

// use std::collections::VecDeque;

use super::snapshot::GraphSnapshot;
use super::r#macro::CommandStep;
use crate::core::graph::Graph;
use crate::core::signal::EventBus;

/// An entry on the undo stack representing one logical action.
pub struct HistoryEntry {
    pub label: String,
    pub steps: Vec<CommandStep>,
    pub snapshot: GraphSnapshot,
}

pub struct HistoryStack {
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
}

impl Default for HistoryStack {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryStack {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(), // Fix: Changed from VecDeque::new()
        }
    }

    /// Push a committed transaction onto the undo stack.
    /// Pushing a new entry clears the redo stack (can't redo after new action).
    pub fn push(&mut self, entry: HistoryEntry) {
        self.redo_stack.clear();
        self.undo_stack.push(entry);
    }

    /// Pop the last action from undo stack, execute its undo functions, and push to redo stack.
    pub fn undo(&mut self, graph: &mut Graph, event_bus: &mut EventBus) -> Result<(), String> {
        let entry = match self.undo_stack.pop() {
            Some(e) => e,
            None => return Err("Undo stack is empty".into()),
        };

        // Undo steps in reverse order
        for step in entry.steps.iter().rev() {
            (step.undo_fn)(graph, event_bus, step.params.clone())?;
        }

        self.redo_stack.push(entry);
        Ok(())
    }

    /// Pop the last action from redo stack, re-execute it, and push to undo stack.
    /// Note: Redo requires storing the `execute_fn` as well. 
    /// For brevity in Phase 2, we implement Undo fully and stub Redo, 
    /// as Redo is typically just re-running the command via the pipeline.
    pub fn redo(&mut self) -> Result<(), String> {
        if self.redo_stack.pop().is_some() {
            // In a full implementation, we would call the execute_fn here.
            Ok(())
        } else {
            Err("Redo stack is empty".into())
        }
    }

    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}