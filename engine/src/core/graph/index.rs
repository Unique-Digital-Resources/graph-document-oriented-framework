//! Secondary indexes for O(1) type and tag queries.

use std::collections::{HashMap, HashSet};

use crate::core::node::{Node, NodeId, TypeId};

#[derive(Default)]
pub struct GraphIndex {
    by_type: HashMap<TypeId, HashSet<NodeId>>,
    by_tag: HashMap<String, HashSet<NodeId>>,
}

impl GraphIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn index_node(&mut self, node: &Node) {
        self.by_type
            .entry(node.type_id.clone())
            .or_default()
            .insert(node.id);

        for tag in node.metadata.tags() {
            self.by_tag
                .entry(tag.to_string())
                .or_default()
                .insert(node.id);
        }
    }

    pub fn remove_node(&mut self, node: &Node) {
        if let Some(ids) = self.by_type.get_mut(&node.type_id) {
            ids.remove(&node.id);
        }

        for tag in node.metadata.tags() {
            if let Some(ids) = self.by_tag.get_mut(tag) {
                ids.remove(&node.id);
            }
        }
    }

    pub fn get_by_type(&self, type_id: &TypeId) -> Vec<NodeId> {
        self.by_type
            .get(type_id)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn get_by_tag(&self, tag: &str) -> Vec<NodeId> {
        self.by_tag
            .get(tag)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }
}