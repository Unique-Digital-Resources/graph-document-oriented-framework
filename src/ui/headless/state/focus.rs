use crate::core::signal::event_bus::EventBus;
use crate::core::signal::types::{EmitTiming, Signal};
use crate::ui::headless::nodes::ui_node::{UiNodeRole, UiNodeId};
use crate::ui::headless::view_graph::storage::ViewGraph;

#[derive(Debug, Clone, Default)]
pub struct FocusState {
    focused: Option<UiNodeId>,
}

impl FocusState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn focused(&self) -> Option<UiNodeId> {
        self.focused
    }

    pub fn is_focused(&self, id: UiNodeId) -> bool {
        self.focused == Some(id)
    }

    pub fn focus(&mut self, bus: &mut EventBus, view: &ViewGraph, id: UiNodeId) {
        if !is_focusable(view, id) {
            return;
        }
        if self.focused == Some(id) {
            return;
        }
        self.focused = Some(id);
        self.emit_changed(bus);
    }

    pub fn blur(&mut self, bus: &mut EventBus) {
        if self.focused.is_none() {
            return;
        }
        self.focused = None;
        self.emit_changed(bus);
    }

    pub fn focus_next(&mut self, bus: &mut EventBus, view: &ViewGraph) {
        let order = focusable_order(view);
        if order.is_empty() {
            self.blur(bus);
            return;
        }
        let next = match self.focused {
            None => order[0],
            Some(cur) => {
                let idx = order.iter().position(|x| *x == cur).map(|i| i + 1).unwrap_or(0);
                order[idx % order.len()]
            }
        };
        self.focus(bus, view, next);
    }

    pub fn on_node_removed(&mut self, bus: &mut EventBus, view: &ViewGraph, id: UiNodeId) {
        if self.focused == Some(id) {
            self.focused = None;
            self.focus_next(bus, view);
        }
    }

    fn emit_changed(&self, bus: &mut EventBus) {
        let payload = serde_json::json!({
            "focused": self.focused,
        });
        
        let signal = Signal::new("FocusChanged", EmitTiming::Immediate)
            .with_payload(payload);
            
        bus.emit(signal);
    }
}

fn is_focusable(view: &ViewGraph, id: UiNodeId) -> bool {
    let widget = match view.get(id) {
        Some(w) => w,
        None => return false,
    };
    let node = widget.ui_node();
    if !node.is_interactive() {
        return false;
    }
    matches!(
        node.role,
        UiNodeRole::Button | UiNodeRole::TextInput | UiNodeRole::List | UiNodeRole::ListItem | UiNodeRole::Inspector
    )
}

fn focusable_order(view: &ViewGraph) -> Vec<UiNodeId> {
    let mut out = Vec::new();
    if let Some(root) = view.root() {
        dfs_collect(view, root, &mut out);
    }
    out
}

fn dfs_collect(view: &ViewGraph, id: UiNodeId, out: &mut Vec<UiNodeId>) {
    if is_focusable(view, id) {
        out.push(id);
    }
    for child in view.children(id) {
        dfs_collect(view, child, out);
    }
}