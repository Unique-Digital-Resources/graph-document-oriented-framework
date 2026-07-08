//! Enforces RelationSchema rules (Topology, Cardinality) before mutation.

use super::storage::{Graph, GraphError};
use crate::core::node::NodeId;
use crate::core::relation::{Cardinality, Topology};

/// Checks if adding `source -> target` is legal under the schema.
pub fn validate_edge(
    graph: &Graph,
    schema: &crate::core::relation::RelationSchema,
    source: NodeId,
    target: NodeId,
) -> Result<(), GraphError> {
    // 1. Prevent self-loops unless Graph topology explicitly allows it
    if source == target && !schema.topology.allows_cycles() {
        return Err(GraphError::ValidationError(
            "Self-loops are not allowed in Tree/DAG topologies".into(),
        ));
    }

    // 2. Cardinality checks
    match schema.cardinality {
        Cardinality::OneToOne => {
            if let Some(targets) = graph.get_targets_set(source, &schema.name) {
                if !targets.is_empty() {
                    return Err(GraphError::ValidationError(format!(
                        "Cardinality 1:1 violated: source {:?} already has a target for {}",
                        source, schema.name
                    )));
                }
            }
            let sources = graph.get_sources(target, &schema.name);
            if !sources.is_empty() {
                return Err(GraphError::ValidationError(format!(
                    "Cardinality 1:1 violated: target {:?} already has a source for {}",
                    target, schema.name
                )));
            }
        }
        Cardinality::OneToMany => {
            let sources = graph.get_sources(target, &schema.name);
            if !sources.is_empty() {
                return Err(GraphError::ValidationError(format!(
                    "Cardinality 1:N violated: target {:?} already has a parent for {}",
                    target, schema.name
                )));
            }
        }
        Cardinality::ManyToMany => {
            // No restrictions on count
        }
    }

    // 3. Topology checks (Cycle prevention)
    match schema.topology {
        Topology::Tree => {
            // A node cannot have multiple parents (already checked in 1:N cardinality usually)
            // It also cannot create a cycle.
            if creates_cycle(graph, &schema.name, source, target) {
                return Err(GraphError::ValidationError(
                    "Tree topology violated: cycle detected".into(),
                ));
            }
        }
        Topology::DAG => {
            if creates_cycle(graph, &schema.name, source, target) {
                return Err(GraphError::ValidationError(
                    "DAG topology violated: cycle detected".into(),
                ));
            }
        }
        Topology::Graph => {
            // Cycles are allowed
        }
    }

    Ok(())
}

/// Detects if adding source -> target creates a cycle.
/// This is true if we can already reach `source` by traversing forward from `target`.
fn creates_cycle(graph: &Graph, schema_name: &str, source: NodeId, target: NodeId) -> bool {
    let mut stack = vec![target];
    let mut visited = std::collections::HashSet::new();
    visited.insert(target);

    while let Some(current) = stack.pop() {
        if current == source {
            return true;
        }

        if let Some(targets) = graph.get_targets_set(current, schema_name) {
            for next in targets {
                if visited.insert(*next) {
                    stack.push(*next);
                }
            }
        }
    }

    false
}