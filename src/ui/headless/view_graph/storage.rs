use std::collections::{HashMap, HashSet};

use crate::core::graph::storage::{Graph, GraphError};
use crate::core::node::node::NodeId;
use crate::core::relation::presets::children;

use crate::ui::headless::nodes::ui_node::UiNodeId;
use crate::ui::headless::nodes::widgets::WidgetKind;

const CHILDREN_SCHEMA_NAME: &str = "CHILDREN";

pub struct ViewGraph {
    graph: Graph,
    nodes: HashMap<UiNodeId, WidgetKind>,
    root: Option<UiNodeId>,
}

impl ViewGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            nodes: HashMap::new(),
            root: None,
        }
    }

    pub fn insert(&mut self, widget: WidgetKind) -> UiNodeId {
        let id = widget.ui_node().id;
        let inner = widget.ui_node().inner.clone();
        self.graph.insert_node(inner);
        self.nodes.insert(id, widget);
        id
    }

    pub fn attach(&mut self, parent: UiNodeId, child: UiNodeId) -> Result<(), ViewGraphError> {
        if !self.nodes.contains_key(&parent) {
            return Err(ViewGraphError::UnknownParent(parent));
        }
        if !self.nodes.contains_key(&child) {
            return Err(ViewGraphError::UnknownChild(child));
        }
        
        let schema = children();
        self.graph
            .add_edge(&schema, parent.0, child.0)
            .map_err(ViewGraphError::GraphError)?;
        Ok(())
    }

    pub fn detach(&mut self, child: UiNodeId) -> Result<(), ViewGraphError> {
        if !self.nodes.contains_key(&child) {
            return Err(ViewGraphError::UnknownChild(child));
        }
        
        // Manual edge removal since Graph doesn't expose remove_node_edges directly
        if let Some(parent) = self.parent(child) {
            self.graph.remove_edge(CHILDREN_SCHEMA_NAME, parent.0, child.0);
        }
        Ok(())
    }

    pub fn remove(&mut self, id: UiNodeId) -> Result<(), ViewGraphError> {
        if !self.nodes.contains_key(&id) {
            return Err(ViewGraphError::UnknownNode(id));
        }
        
        let descendants = self.collect_descendants(id.0);
        for d in descendants.iter().chain(std::iter::once(&id.0)) {
            self.nodes.remove(&UiNodeId(*d));
        }
        
        self.graph.remove_node(id.0);
            
        if self.root == Some(id) {
            self.root = None;
        }
        Ok(())
    }
    
    fn collect_descendants(&self, id: NodeId) -> Vec<NodeId> {
        let mut stack = vec![id];
        let mut visited = HashSet::new();
        while let Some(curr) = stack.pop() {
            if visited.insert(curr) {
                let children = self.graph.get_targets(curr, CHILDREN_SCHEMA_NAME);
                stack.extend(children);
            }
        }
        visited.into_iter().filter(|n| *n != id).collect()
    }

    pub fn get(&self, id: UiNodeId) -> Option<&WidgetKind> {
        self.nodes.get(&id)
    }

    pub fn get_mut(&mut self, id: UiNodeId) -> Option<&mut WidgetKind> {
        self.nodes.get_mut(&id)
    }

    pub fn children(&self, id: UiNodeId) -> Vec<UiNodeId> {
        self.graph
            .get_targets(id.0, CHILDREN_SCHEMA_NAME)
            .into_iter()
            .map(|n| UiNodeId(n))
            .collect()
    }

    pub fn parent(&self, id: UiNodeId) -> Option<UiNodeId> {
        self.graph
            .get_sources(id.0, CHILDREN_SCHEMA_NAME)
            .first()
            .copied()
            .map(|n| UiNodeId(n))
    }

    pub fn root(&self) -> Option<UiNodeId> {
        self.root
    }

    pub fn set_root(&mut self, id: UiNodeId) -> Result<(), ViewGraphError> {
        if !self.nodes.contains_key(&id) {
            return Err(ViewGraphError::UnknownNode(id));
        }
        self.root = Some(id);
        Ok(())
    }

    pub fn inner(&self) -> &Graph {
        &self.graph
    }
    
    pub fn inner_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }
}

impl Default for ViewGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewGraphError {
    UnknownParent(UiNodeId),
    UnknownChild(UiNodeId),
    UnknownNode(UiNodeId),
    GraphError(GraphError),
}

impl std::fmt::Display for ViewGraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewGraphError::UnknownParent(id) => write!(f, "unknown parent ui node: {}", id),
            ViewGraphError::UnknownChild(id) => write!(f, "unknown child ui node: {}", id),
            ViewGraphError::UnknownNode(id) => write!(f, "unknown ui node: {}", id),
            ViewGraphError::GraphError(e) => write!(f, "core graph error: {:?}", e),
        }
    }
}

impl std::error::Error for ViewGraphError {}