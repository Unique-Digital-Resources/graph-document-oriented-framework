//! Concrete widget types built on top of [`UiNode`].
//!
//! Each widget is a thin struct that owns a `UiNode` and adds widget-specific
//! fields. Widgets are intentionally dumb: they hold *state*, not behavior.
//! Behavior (click handling, list population, etc.) is wired up by binding
//! widget properties to Commands via the framework's command registry.

use std::collections::HashMap;

use engine::core::node::node::NodeId;
use engine::core::node::properties::PropertyValue;

use super::ui_node::{UiNode, UiNodeRole};

// Note: CommandId is assumed to be a string newtype or similar. 
// Adjust if your core command registry uses a different type.
pub type CommandId = String; 

/// Tagged union of all known widget kinds. Useful when iterating the view
/// graph without downcasting.
#[derive(Debug, Clone)]
pub enum WidgetKind {
    Container(ContainerNode),
    Label(LabelNode),
    Button(ButtonNode),
    TextField(TextFieldNode),
    ListView(ListViewNode),
    Inspector(InspectorNode),
    Custom(CustomWidget),
}

impl WidgetKind {
    pub fn ui_node(&self) -> &UiNode {
        match self {
            WidgetKind::Container(w) => &w.base,
            WidgetKind::Label(w) => &w.base,
            WidgetKind::Button(w) => &w.base,
            WidgetKind::TextField(w) => &w.base,
            WidgetKind::ListView(w) => &w.base,
            WidgetKind::Inspector(w) => &w.base,
            WidgetKind::Custom(w) => &w.base,
        }
    }

    pub fn ui_node_mut(&mut self) -> &mut UiNode {
        match self {
            WidgetKind::Container(w) => &mut w.base,
            WidgetKind::Label(w) => &mut w.base,
            WidgetKind::Button(w) => &mut w.base,
            WidgetKind::TextField(w) => &mut w.base,
            WidgetKind::ListView(w) => &mut w.base,
            WidgetKind::Inspector(w) => &mut w.base,
            WidgetKind::Custom(w) => &mut w.base,
        }
    }
}

/// A generic container — a `div`-like node that just holds children.
#[derive(Debug, Clone)]
pub struct ContainerNode {
    pub base: UiNode,
    pub layout: ContainerLayout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerLayout {
    Stack,
    Row,
    Column,
    Grid,
    Absolute,
}

impl ContainerNode {
    pub fn new(layout: ContainerLayout) -> Self {
        let mut base = UiNode::new("Container", UiNodeRole::Container);
        base.set_property("layout", PropertyValue::String(format!("{:?}", layout)));
        Self { base, layout }
    }
}

/// A non-interactive piece of text.
#[derive(Debug, Clone)]
pub struct LabelNode {
    pub base: UiNode,
    pub text: String,
}

impl LabelNode {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let mut base = UiNode::new("Label", UiNodeRole::Label);
        base.set_property("text", PropertyValue::String(text.clone()));
        Self { base, text }
    }
}

/// A clickable button. Clicks are *not* handled here — the input bridge
/// dispatches a Command identified by `command_id` when the button is
/// activated.
#[derive(Debug, Clone)]
pub struct ButtonNode {
    pub base: UiNode,
    pub label: String,
    pub command_id: Option<CommandId>,
    pub command_params: HashMap<String, PropertyValue>,
    pub enabled: bool,
}

impl ButtonNode {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        let mut base = UiNode::new("Button", UiNodeRole::Button);
        base.set_property("label", PropertyValue::String(label.clone()));
        Self {
            base,
            label,
            command_id: None,
            command_params: HashMap::new(),
            enabled: true,
        }
    }

    pub fn bind_command(&mut self, command_id: CommandId, params: HashMap<String, PropertyValue>) {
        self.command_id = Some(command_id);
        self.command_params = params;
    }
}

/// A single-line text input.
#[derive(Debug, Clone)]
pub struct TextFieldNode {
    pub base: UiNode,
    pub value: String,
    pub placeholder: String,
    pub bound_property: Option<(NodeId, String)>,
}

impl TextFieldNode {
    pub fn new(placeholder: impl Into<String>) -> Self {
        let placeholder = placeholder.into();
        let mut base = UiNode::new("TextField", UiNodeRole::TextInput);
        base.set_property("placeholder", PropertyValue::String(placeholder.clone()));
        Self {
            base,
            value: String::new(),
            placeholder,
            bound_property: None,
        }
    }

    pub fn bind_to(&mut self, node: NodeId, property: impl Into<String>) {
        self.bound_property = Some((node, property.into()));
    }
}

/// A scrollable list. The list itself does not own its rows' data — it
/// reads them from a bound document node (typically a folder or a
/// collection-type node) via the View Graph's binding layer.
#[derive(Debug, Clone)]
pub struct ListViewNode {
    pub base: UiNode,
    pub source: Option<NodeId>,
    pub item_template: ItemTemplate,
    pub rows: Vec<crate::nodes::ui_node::UiNodeId>,
    pub scroll_offset: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemTemplate {
    LabelRow,
    ActionRow { command_id: CommandId },
    Custom(String),
}

impl ListViewNode {
    pub fn new() -> Self {
        let base = UiNode::new("ListView", UiNodeRole::List);
        Self {
            base,
            source: None,
            item_template: ItemTemplate::LabelRow,
            rows: Vec::new(),
            scroll_offset: 0.0,
        }
    }

    pub fn bind_source(&mut self, source: NodeId) {
        self.source = Some(source);
    }
}

/// An inspector panel. Renders a flat list of property editors for a
/// bound document node.
#[derive(Debug, Clone)]
pub struct InspectorNode {
    pub base: UiNode,
    pub target: Option<NodeId>,
    pub editors: HashMap<String, crate::nodes::ui_node::UiNodeId>,
}

impl InspectorNode {
    pub fn new() -> Self {
        let base = UiNode::new("Inspector", UiNodeRole::Inspector);
        Self {
            base,
            target: None,
            editors: HashMap::new(),
        }
    }

    pub fn inspect(&mut self, target: NodeId) {
        self.target = Some(target);
        self.editors.clear();
    }
}

/// A generic widget for app-specific UI (e.g., a 3D Viewport, a Color Picker, 
/// a custom graph editor). The framework passes the `kind` and `data` directly 
/// to the frontend Web Component, allowing applications to extend the UI without 
/// modifying the core framework.
#[derive(Debug, Clone)]
pub struct CustomWidget {
    pub base: UiNode,
    /// The tag name of the custom web component (e.g., "viewport-3d", "color-picker")
    pub kind: String,
    /// Arbitrary JSON payload to pass to the frontend component on initialization/update
    pub data: serde_json::Value,
    /// List of DOM events this component should listen to (e.g., ["change", "click"])
    pub event_listeners: Vec<String>,
}