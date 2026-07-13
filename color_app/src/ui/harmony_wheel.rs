use engine::core::graph::Graph;
use engine::core::node::properties::PropertyValue;
use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;
use uuid::Uuid;

pub fn create(target: Uuid) -> CustomWidget {
    CustomWidget {
        base: UiNode::new("HarmonyWheel", UiNodeRole::Custom),
        kind: "harmony-wheel".to_string(),
        data: serde_json::json!({
            "target_node": target.to_string(),
            "rule": "none",
            "ruler_enabled": false,
            "ruler_mode": "lines",
            "selected_thumb_index": 0,
            "harmony_colors": []
        }),
        event_listeners: vec!["input".to_string(), "change".to_string()],
    }
}

pub fn sync(c: &mut CustomWidget, graph: &Graph, wheel_mode: &str) {
    c.data["wheel_mode"] = serde_json::Value::String(wheel_mode.to_string());

    if let Some(target_str) = c.data["target_node"].as_str() {
        if let Ok(target_id) = Uuid::parse_str(target_str) {
            if let Some(node) = graph.get_node(target_id) {
                if let Some(PropertyValue::Array(val)) = node.properties.get_value("harmony_colors") {
                    let json_arr: Vec<serde_json::Value> = val.iter().map(|thumb_val| {
                        if let PropertyValue::Array(inner) = thumb_val {
                            serde_json::Value::Array(inner.iter().filter_map(|x| {
                                if let PropertyValue::Float(f) = x { Some(serde_json::json!(*f)) } else { None }
                            }).collect())
                        } else {
                            serde_json::Value::Null
                        }
                    }).collect();
                    c.data["harmony_colors"] = serde_json::Value::Array(json_arr);
                }
            }
        }
    }
}