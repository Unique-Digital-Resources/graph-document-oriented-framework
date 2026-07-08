//! Captures graph state before mutation for auto-revert functionality.

use std::collections::HashMap;

use crate::core::graph::Graph;
use crate::core::node::{Node, NodeId};

/// A partial snapshot of the graph, storing nodes before they were mutated.
/// Used to rollback a failed transaction or auto-revert a command.
#[derive(Default)]
pub struct GraphSnapshot {
    nodes: HashMap<NodeId, Node>,
}

impl GraphSnapshot {
    pub fn new() -> Self {
        Self::default()
    }

    /// Capture a node's current state into the snapshot.
    /// If the node doesn't exist, we record a tombstone (None).
    pub fn capture(&mut self, graph: &Graph, id: NodeId) {
        if !self.nodes.contains_key(&id) {
            let node_state = graph.get_node(id).cloned();
            if let Some(node) = node_state {
                self.nodes.insert(id, node);
            }
        }
    }

    /// Restore the captured nodes back into the graph.
    pub fn restore(&self, graph: &mut Graph) {
        for (id, node) in &self.nodes {
            graph.restore_node(*id, node.clone());
        }
    }
}