//! High-level Query API for the graph.

use super::storage::Graph;
use crate::core::node::{NodeId, TypeId};

/// A read-only view into the graph for executing queries.
pub struct GraphQuery<'a> {
    graph: &'a Graph,
}

impl<'a> GraphQuery<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }

    /// Equivalent to DSL: `relation[id].schema_name`
    pub fn relations(&self, id: NodeId, schema_name: &str) -> Vec<NodeId> {
        self.graph.get_targets(id, schema_name)
    }

    /// Equivalent to DSL: `node[id].children`
    pub fn children(&self, id: NodeId) -> Vec<NodeId> {
        self.relations(id, "CHILDREN")
    }

    /// Equivalent to DSL: `node[id].dependencies`
    pub fn dependencies(&self, id: NodeId) -> Vec<NodeId> {
        self.relations(id, "DEPENDENCY")
    }

    /// Find all nodes of a specific type.
    pub fn find_by_type(&self, type_id: &TypeId) -> Vec<NodeId> {
        self.graph.index().get_by_type(type_id)
    }

    /// Find all nodes containing a specific tag.
    pub fn find_by_tag(&self, tag: &str) -> Vec<NodeId> {
        self.graph.index().get_by_tag(tag)
    }

    /// Find all ancestors of a node via a specific schema (e.g., all parents).
    pub fn find_ancestors(&self, id: NodeId, schema_name: &str) -> Vec<NodeId> {
        let mut ancestors = Vec::new();
        let mut queue = vec![id];
        let mut visited = std::collections::HashSet::new();
        visited.insert(id);

        while let Some(current) = queue.pop() {
            let sources = self.graph.get_sources(current, schema_name);
            for source in sources {
                if visited.insert(source) {
                    ancestors.push(source);
                    queue.push(source);
                }
            }
        }
        ancestors
    }
}