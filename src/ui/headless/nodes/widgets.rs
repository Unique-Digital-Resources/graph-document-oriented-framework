use std::collections::HashMap;

use crate::core::node::node::NodeId;
use crate::core::node::properties::PropertyValue;

use super::ui_node::{UiNode, UiNodeRole, UiNodeId};

pub type CommandId = String; 

#[derive(Debug, Clone)]
pub enum WidgetKind {
    Container(ContainerNode),
    Label(LabelNode),
    Button(ButtonNode),
    TextField(TextFieldNode),
    ListView(ListViewNode),
    Inspector(InspectorNode),
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
        }
    }
}

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

#[derive(Debug, Clone)]
pub struct ListViewNode {
    pub base: UiNode,
    pub source: Option<NodeId>,
    pub item_template: ItemTemplate,
    pub rows: Vec<UiNodeId>,
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

#[derive(Debug, Clone)]
pub struct InspectorNode {
    pub base: UiNode,
    pub target: Option<NodeId>,
    pub editors: HashMap<String, UiNodeId>,
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