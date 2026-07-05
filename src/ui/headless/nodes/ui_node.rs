use std::fmt;

use serde::{Deserialize, Serialize};

use crate::core::node::node::{Node, NodeId};
use crate::core::node::properties::PropertyValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UiNodeId(pub NodeId);

impl fmt::Display for UiNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ui:{}", self.0)
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Bounds {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }
    }
}

impl Bounds {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x
            && px <= self.x + self.width
            && py >= self.y
            && py <= self.y + self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiNodeRole {
    Container,
    Button,
    TextInput,
    List,
    ListItem,
    Inspector,
    Label,
    Custom,
}

#[derive(Debug, Clone)]
pub struct UiNode {
    pub id: UiNodeId,
    pub inner: Node,
    pub bounds: Bounds,
    pub role: UiNodeRole,
    pub visible: bool,
    pub enabled: bool,
    pub transient: bool,
}

impl UiNode {
    pub fn new(type_id: impl Into<String>, role: UiNodeRole) -> Self {
        let inner = Node::new(type_id);
        Self {
            id: UiNodeId(inner.id),
            inner,
            bounds: Bounds::default(),
            role,
            visible: true,
            enabled: true,
            transient: true,
        }
    }

    pub fn set_property(&mut self, key: impl Into<String>, value: PropertyValue) {
        self.inner.properties.set_transient(key, value);
    }

    pub fn get_property(&self, key: &str) -> Option<&PropertyValue> {
        self.inner.properties.get_value(key)
    }

    pub fn is_interactive(&self) -> bool {
        self.visible && self.enabled
    }

    pub fn invalidate_layout(&mut self) {
        self.set_property("__layout_dirty", PropertyValue::Bool(true));
    }
}