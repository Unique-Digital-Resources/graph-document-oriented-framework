//! Bindings: one-way references from UI nodes to document (data) nodes.
//!
//! This is the seam where the View Graph meets the Document Graph. The
//! rule is strict and one-directional:
//!
//! ```text
//!     UI Node  ──references──▶  Document Node
//!     Document Node  ✗──never──  UI Node
//! ```
//!
//! Bindings are *not* edges in either graph. They live in a separate
//! registry so that:
//! * The document graph never sees UI ids.
//! * Document serialization naturally ignores UI bindings.
//! * The binding layer can be rebuilt cheaply when the document changes.

use std::collections::HashMap;

use crate::core::node::node::NodeId;
use crate::ui::headless::nodes::ui_node::UiNodeId;

/// A single binding. Describes *which* document node a UI node is bound to,
/// and *how* (loosely) it should react when that document node changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub ui_node: UiNodeId,
    pub document_node: NodeId,
    /// Optional property name on the document node that this UI node
    /// renders. `None` means the UI node observes the document node as a
    /// whole (e.g. an inspector panel).
    pub property: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingKind {
    /// UI displays the value of a document property.
    Display,
    /// UI edits the value of a document property (two-way at the UX level,
    /// but every write still goes through a Command).
    Editor,
    /// UI represents the document node structurally (e.g. a list row).
    Structural,
    /// UI triggers a command targeting the document node (e.g. a button).
    Action,
}

#[derive(Default)]
pub struct BindingRegistry {
    /// UI node -> binding.
    by_ui: HashMap<UiNodeId, (Binding, BindingKind)>,
    /// Document node -> UI nodes bound to it. Used to invalidate UI when
    /// a document node changes.
    by_doc: HashMap<NodeId, Vec<UiNodeId>>,
}

impl BindingRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create or replace a binding. Removes any previous binding for the
    /// same UI node first.
    pub fn bind(&mut self, binding: Binding, kind: BindingKind) {
        // Unbind first to keep the reverse index consistent.
        if let Some((old, _)) = self.by_ui.get(&binding.ui_node).cloned() {
            if let Some(vec) = self.by_doc.get_mut(&old.document_node) {
                vec.retain(|id| *id != binding.ui_node);
            }
        }
        self.by_doc
            .entry(binding.document_node)
            .or_default()
            .push(binding.ui_node);
        self.by_ui.insert(binding.ui_node, (binding, kind));
    }

    pub fn unbind(&mut self, ui_node: UiNodeId) {
        if let Some((old, _)) = self.by_ui.remove(&ui_node) {
            if let Some(vec) = self.by_doc.get_mut(&old.document_node) {
                vec.retain(|id| *id != ui_node);
            }
        }
    }

    pub fn get(&self, ui_node: UiNodeId) -> Option<(&Binding, BindingKind)> {
        self.by_ui.get(&ui_node).map(|(b, k)| (b, *k))
    }

    /// All UI nodes bound to a given document node. Used by the layout
    /// system and renderers to know which UI subtrees need refreshing
    /// after a document mutation.
    pub fn ui_for_document(&self, doc: NodeId) -> &[UiNodeId] {
        self.by_doc
            .get(&doc)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Remove every binding pointing at `doc`. Called when a document
    /// node is deleted — the corresponding UI nodes are then destroyed by
    /// the view graph.
    pub fn purge_document_node(&mut self, doc: NodeId) -> Vec<UiNodeId> {
        let ui_ids = self.by_doc.remove(&doc).unwrap_or_default();
        for id in &ui_ids {
            self.by_ui.remove(id);
        }
        ui_ids
    }
}