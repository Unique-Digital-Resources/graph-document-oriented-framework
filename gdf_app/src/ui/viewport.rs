use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;

pub struct Viewport3DNode;

impl Viewport3DNode {
    pub fn new() -> CustomWidget {
        CustomWidget {
            base: UiNode::new("Viewport3D", UiNodeRole::Custom),
            kind: "viewport-3d".to_string(),
            data: serde_json::Value::Null,
            event_listeners: vec![],
        }
    }
}