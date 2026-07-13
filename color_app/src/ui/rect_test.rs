use engine::core::graph::Graph;
use engine::core::node::properties::PropertyValue;
use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;
use uuid::Uuid;

pub fn create(target: Uuid) -> CustomWidget {
    CustomWidget {
        base: UiNode::new("RectTest", UiNodeRole::Custom),
        kind: "rect-test".to_string(),
        data: serde_json::json!({
            "target_node": target.to_string(),
            "color": [0.0, 100.0, 50.0, 1.0],
            "color_mode": "direct"
        }),
        event_listeners: vec![],
    }
}

pub fn sync(c: &mut CustomWidget, graph: &Graph) {
    if let Some(target_str) = c.data["target_node"].as_str() {
        if let Ok(target_id) = Uuid::parse_str(target_str) {
            if let Some(node) = graph.get_node(target_id) {
                if let Some(PropertyValue::Array(color_val)) = node.properties.get_value("color") {
                    let json_arr: Vec<serde_json::Value> = color_val.iter().filter_map(|v| {
                        if let PropertyValue::Float(f) = v { Some(serde_json::json!(*f)) } else { None }
                    }).collect();
                    c.data["color"] = serde_json::Value::Array(json_arr);
                }
                if let Some(PropertyValue::String(mode)) = node.properties.get_value("color_mode") {
                    c.data["color_mode"] = serde_json::Value::String(mode.clone());
                }
            }
        }
    }
}