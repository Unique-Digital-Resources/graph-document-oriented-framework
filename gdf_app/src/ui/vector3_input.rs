use engine::core::graph::Graph;
use engine::core::node::NodeId;
use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;

pub struct Vector3InputNode;

impl Vector3InputNode {
    pub fn new(target_node: NodeId, property: &str, graph: &Graph) -> CustomWidget {
        let val = graph.get_node(target_node).and_then(|n| n.properties.get_value(property));
        CustomWidget {
            base: UiNode::new("Vector3Input", UiNodeRole::Custom),
            kind: "vector3-input".to_string(),
            data: serde_json::json!({
                "target_node": target_node.to_string(),
                "property": property,
                "value": val
            }),
            event_listeners: vec!["change".to_string()],
        }
    }
}