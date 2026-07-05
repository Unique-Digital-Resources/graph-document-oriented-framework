use std::collections::HashSet;

use crate::core::node::node::NodeId;
use crate::core::signal::event_bus::EventBus;
use crate::core::signal::types::{EmitTiming, Signal};

#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    selected: HashSet<NodeId>,
    primary: Option<NodeId>,
}

impl SelectionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select(&mut self, bus: &mut EventBus, id: NodeId) {
        self.selected.clear();
        self.selected.insert(id);
        self.primary = Some(id);
        self.emit_changed(bus);
    }

    pub fn toggle(&mut self, bus: &mut EventBus, id: NodeId) {
        if self.selected.contains(&id) {
            self.selected.remove(&id);
            if self.primary == Some(id) {
                self.primary = self.selected.iter().next().copied();
            }
        } else {
            self.selected.insert(id);
            self.primary = Some(id);
        }
        self.emit_changed(bus);
    }

    pub fn clear(&mut self, bus: &mut EventBus) {
        if self.selected.is_empty() {
            return;
        }
        self.selected.clear();
        self.primary = None;
        self.emit_changed(bus);
    }

    pub fn is_selected(&self, id: NodeId) -> bool {
        self.selected.contains(&id)
    }

    pub fn selected(&self) -> &HashSet<NodeId> {
        &self.selected
    }

    pub fn primary(&self) -> Option<NodeId> {
        self.primary
    }

    // Add this missing method:
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }

    pub fn on_node_deleted(&mut self, bus: &mut EventBus, id: NodeId) {
        if self.selected.remove(&id) {
            if self.primary == Some(id) {
                self.primary = self.selected.iter().next().copied();
            }
            self.emit_changed(bus);
        }
    }

    fn emit_changed(&self, bus: &mut EventBus) {
        let payload = serde_json::json!({
            "count": self.selected.len(),
            "primary": self.primary,
        });
        
        let signal = Signal::new("SelectionChanged", EmitTiming::Immediate)
            .with_payload(payload);
            
        bus.emit(signal);
    }
}