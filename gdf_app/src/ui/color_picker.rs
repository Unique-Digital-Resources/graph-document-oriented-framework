use engine::core::graph::Graph;
use engine::core::node::NodeId;
use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;

pub struct ColorPickerNode;

impl ColorPickerNode {
    pub fn new(target_node: NodeId, graph: &Graph) -> CustomWidget {
        let val = graph.get_node(target_node).and_then(|n| n.properties.get_value("color"));
        CustomWidget {
            base: UiNode::new("ColorPicker", UiNodeRole::Custom),
            kind: "color-picker".to_string(),
            data: serde_json::json!({
                "target_node": target_node.to_string(),
                "value": val
            }),
            event_listeners: vec!["change".to_string()],
        }
    }
}