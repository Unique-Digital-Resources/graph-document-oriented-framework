//! In-memory graph storage with O(1) node lookups and bidirectional edges.

use std::collections::{HashMap, HashSet};

use crate::core::node::{Node, NodeId};
use crate::core::relation::RelationSchema;

use super::index::GraphIndex;
use super::validation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    NodeNotFound(NodeId),
    ValidationError(String),
}

/// The master graph container.
pub struct Graph {
    nodes: HashMap<NodeId, Node>,
    /// Forward edges: source -> (schema_name -> targets)
    forward: HashMap<NodeId, HashMap<String, HashSet<NodeId>>>,
    /// Reverse edges: target -> (schema_name -> sources)
    /// Used for O(1) parent lookups and cascade deletes.
    reverse: HashMap<NodeId, HashMap<String, HashSet<NodeId>>>,
    /// Secondary indexes for fast type and tag queries.
    index: GraphIndex,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            forward: HashMap::new(),
            reverse: HashMap::new(),
            index: GraphIndex::new(),
        }
    }

    /// Pre-allocate memory for a large number of nodes. 
    /// Prevents HashMap from constantly resizing during bulk inserts.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: HashMap::with_capacity(capacity),
            forward: HashMap::with_capacity(capacity),
            reverse: HashMap::with_capacity(capacity),
            index: GraphIndex::new(),
        }
    }

    // --- Node Management ---

    pub fn insert_node(&mut self, node: Node) -> NodeId {
        let id = node.id;
        self.index.index_node(&node);
        self.nodes.insert(id, node);
        id
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn remove_node(&mut self, id: NodeId) -> Option<Node> {
        let node = self.nodes.remove(&id)?;
        self.index.remove_node(&node);

        // Clean up forward edges (children/dependencies)
        if let Some(schemas) = self.forward.remove(&id) {
            for (_schema, targets) in schemas {
                for target in targets {
                    if let Some(rev) = self.reverse.get_mut(&target) {
                        for srcs in rev.values_mut() {
                            srcs.remove(&id);
                        }
                    }
                }
            }
        }

        // Clean up reverse edges (parents/references)
        if let Some(schemas) = self.reverse.remove(&id) {
            for (_schema, sources) in schemas {
                for source in sources {
                    if let Some(fwd) = self.forward.get_mut(&source) {
                        for tgts in fwd.values_mut() {
                            tgts.remove(&id);
                        }
                    }
                    // Also clean the local cache on the source node
                    if let Some(src_node) = self.nodes.get_mut(&source) {
                        src_node.relations.remove_all_targets(id);
                    }
                }
            }
        }

        Some(node)
    }

    pub fn contains_node(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    // --- Edge Management ---

    /// Adds an edge, enforcing the rules of the provided `RelationSchema`.
    pub fn add_edge(
        &mut self,
        schema: &RelationSchema,
        source: NodeId,
        target: NodeId,
    ) -> Result<(), GraphError> {
        if !self.nodes.contains_key(&source) {
            return Err(GraphError::NodeNotFound(source));
        }
        if !self.nodes.contains_key(&target) {
            return Err(GraphError::NodeNotFound(target));
        }

        validation::validate_edge(self, schema, source, target)?;

        // Update global forward/reverse maps
        self.forward
            .entry(source)
            .or_default()
            .entry(schema.name.clone())
            .or_default()
            .insert(target);

        self.reverse
            .entry(target)
            .or_default()
            .entry(schema.name.clone())
            .or_default()
            .insert(source);

        // Update local node cache
        if let Some(src_node) = self.nodes.get_mut(&source) {
            src_node.relations.add(&schema.name, target);
        }

        Ok(())
    }

    /// Adds an edge WITHOUT running validation (cycle detection, cardinality).
    /// Use this for bulk imports or loading from trusted save files.
    pub fn add_edge_unchecked(
        &mut self,
        schema: &RelationSchema,
        source: NodeId,
        target: NodeId,
    ) -> Result<(), GraphError> {
        if !self.nodes.contains_key(&source) {
            return Err(GraphError::NodeNotFound(source));
        }
        if !self.nodes.contains_key(&target) {
            return Err(GraphError::NodeNotFound(target));
        }

        // Update global forward/reverse maps
        self.forward
            .entry(source)
            .or_default()
            .entry(schema.name.clone())
            .or_default()
            .insert(target);

        self.reverse
            .entry(target)
            .or_default()
            .entry(schema.name.clone())
            .or_default()
            .insert(source);

        // Update local node cache
        if let Some(src_node) = self.nodes.get_mut(&source) {
            src_node.relations.add(&schema.name, target);
        }

        Ok(())
    }

    pub fn remove_edge(&mut self, schema_name: &str, source: NodeId, target: NodeId) -> bool {
        let mut removed = false;

        if let Some(schemas) = self.forward.get_mut(&source) {
            if let Some(targets) = schemas.get_mut(schema_name) {
                removed = targets.remove(&target);
                if targets.is_empty() {
                    schemas.remove(schema_name);
                }
            }
            if schemas.is_empty() {
                self.forward.remove(&source);
            }
        }

        if let Some(schemas) = self.reverse.get_mut(&target) {
            if let Some(sources) = schemas.get_mut(schema_name) {
                sources.remove(&source);
                if sources.is_empty() {
                    schemas.remove(schema_name);
                }
            }
            if schemas.is_empty() {
                self.reverse.remove(&target);
            }
        }

        if removed {
            if let Some(src_node) = self.nodes.get_mut(&source) {
                src_node.relations.remove(schema_name, target);
            }
        }

        removed
    }

    // --- Internal Accessors (for traversal, validation, queries) ---

    /// Returns a cloned Vec of targets. 
    /// Cloning a Vec of UUIDs is cheap and avoids lifetime issues with temporary HashSets.
    pub fn get_targets(&self, source: NodeId, schema_name: &str) -> Vec<NodeId> {
        self.forward
            .get(&source)
            .and_then(|s| s.get(schema_name))
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    }
    
    /// Returns targets as a HashSet for fast internal lookups
    pub(crate) fn get_targets_set(
        &self,
        source: NodeId,
        schema_name: &str,
    ) -> Option<&HashSet<NodeId>> {
        self.forward
            .get(&source)
            .and_then(|s| s.get(schema_name))
    }

    pub fn get_sources(&self, target: NodeId, schema_name: &str) -> Vec<NodeId> {
        self.reverse
            .get(&target)
            .and_then(|s| s.get(schema_name))
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    }

    pub(crate) fn index(&self) -> &GraphIndex {
        &self.index
    }
    
    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Restores a node from a snapshot. Used during transaction rollback.
    pub fn restore_node(&mut self, id: NodeId, node: Node) {
        self.index.remove_node(&node);
        self.index.index_node(&node);
        self.nodes.insert(id, node);
    }

    pub fn iter_nodes_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.nodes.values_mut()
    }
}