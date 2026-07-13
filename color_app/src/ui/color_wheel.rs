use engine::core::graph::Graph;
use engine::core::node::properties::PropertyValue;
use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;
use uuid::Uuid;

pub fn create(target: Uuid) -> CustomWidget {
    CustomWidget {
        base: UiNode::new("ColorWheel", UiNodeRole::Custom),
        kind: "color-wheel".to_string(), // Will be overwritten by sync
        data: serde_json::json!({
            "target_node": target.to_string(),
            "wheel_mode": "Ranges",
            "value": [0.0, 100.0, 50.0, 1.0]
        }),
        event_listeners: vec!["input".to_string(), "change".to_string()],
    }
}

pub fn sync(c: &mut CustomWidget, graph: &Graph, wheel_mode: &str) {
    c.data["wheel_mode"] = serde_json::Value::String(wheel_mode.to_string());
    
    // Dynamically change the wheel tag so JS renders the right class
    c.kind = match wheel_mode {
        "WheelSquare" => "square-wheel".to_string(),
        "WheelTriangle" => "triangle-wheel".to_string(),
        "Circle" => "circle-wheel".to_string(),
        "Ranges" => "ranges-wheel".to_string(),
        _ => "ranges-wheel".to_string()
    };

    if let Some(target_str) = c.data["target_node"].as_str() {
        if let Ok(target_id) = Uuid::parse_str(target_str) {
            if let Some(node) = graph.get_node(target_id) {
                if let Some(PropertyValue::Array(color_val)) = node.properties.get_value("color") {
                    let json_arr: Vec<serde_json::Value> = color_val.iter().filter_map(|v| {
                        if let PropertyValue::Float(f) = v { Some(serde_json::json!(*f)) } else { None }
                    }).collect();
                    c.data["value"] = serde_json::Value::Array(json_arr);
                }
            }
        }
    }
}