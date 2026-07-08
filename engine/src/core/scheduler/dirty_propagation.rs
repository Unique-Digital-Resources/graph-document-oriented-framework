//! Dirty propagation logic.

use std::collections::HashSet;

use crate::core::graph::Graph;
use crate::core::node::NodeId;
use crate::core::relation::Propagation;

/// Marks downstream nodes as dirty based on the relation's propagation rules.
/// Returns a list of NodeIds that were marked dirty.
pub fn propagate_dirty(
    graph: &mut Graph,
    source: NodeId,
    schema_name: &str,
) -> Vec<NodeId> {
    let mut dirty_nodes = Vec::new();
    let mut visited = HashSet::new();
    let mut stack = vec![source];

    while let Some(current) = stack.pop() {
        if !visited.insert(current) {
            continue;
        }

        // Mark the current node dirty
        if let Some(node) = graph.get_node_mut(current) {
            node.metadata.dirty = true;
            dirty_nodes.push(current);
        }

        // Determine traversal direction based on Propagation rule
        // Note: In a real engine, we'd look up the RelationSchema to get the Propagation.
        // For simplicity here, we assume Forward propagation travels to targets,
        // Backward travels to sources.
        let propagation = Propagation::Forward; // Mocked for generic dirty propagation
        
        let next_nodes = match propagation {
            Propagation::Forward => graph.get_targets(current, schema_name),
            Propagation::Backward => graph.get_sources(current, schema_name),
            Propagation::Bidirectional => {
                let mut nodes = graph.get_targets(current, schema_name);
                nodes.extend(graph.get_sources(current, schema_name));
                nodes
            }
            Propagation::None => Vec::new(),
        };

        for next in next_nodes {
            if !visited.contains(&next) {
                stack.push(next);
            }
        }
    }

    dirty_nodes
}