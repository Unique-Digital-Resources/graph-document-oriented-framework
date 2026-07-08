//! Graph traversal algorithms: DFS, BFS, Topological Sort.

use std::collections::{HashSet, VecDeque};

use super::storage::Graph;
use crate::core::node::NodeId;

/// Depth-First Search from a starting node along a specific relation schema.
pub fn dfs(graph: &Graph, start: NodeId, schema_name: &str) -> Vec<NodeId> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut stack = vec![start];

    while let Some(current) = stack.pop() {
        if visited.insert(current) {
            result.push(current);

            if let Some(targets) = graph.get_targets_set(current, schema_name) {
                for target in targets {
                    if !visited.contains(target) {
                        stack.push(*target);
                    }
                }
            }
        }
    }
    result
}

/// Breadth-First Search from a starting node along a specific relation schema.
pub fn bfs(graph: &Graph, start: NodeId, schema_name: &str) -> Vec<NodeId> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        result.push(current);

        if let Some(targets) = graph.get_targets_set(current, schema_name) {
            for target in targets {
                if visited.insert(*target) {
                    queue.push_back(*target);
                }
            }
        }
    }
    result
}

/// Topological Sort (Kahn's algorithm) for the entire graph along a specific schema.
/// Returns `Some(sorted_list)` if successful, or `None` if a cycle is detected.
pub fn topological_sort(graph: &Graph, schema_name: &str) -> Option<Vec<NodeId>> {
    let mut in_degree: std::collections::HashMap<NodeId, usize> = std::collections::HashMap::new();
    
    // Initialize in-degrees for all nodes
    for node in graph.iter_nodes() {
        in_degree.entry(node.id).or_insert(0);
    }

    // Calculate in-degrees based on reverse edges
    for node in graph.iter_nodes() {
        if let Some(targets) = graph.get_targets_set(node.id, schema_name) {
            for target in targets {
                *in_degree.entry(*target).or_insert(0) += 1;
            }
        }
    }

    let mut queue: VecDeque<NodeId> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&id, _)| id)
        .collect();

    let mut sorted = Vec::new();

    while let Some(current) = queue.pop_front() {
        sorted.push(current);

        if let Some(targets) = graph.get_targets_set(current, schema_name) {
            for target in targets {
                if let Some(deg) = in_degree.get_mut(target) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(*target);
                    }
                }
            }
        }
    }

    if sorted.len() == in_degree.len() {
        Some(sorted)
    } else {
        None // Cycle detected
    }
}