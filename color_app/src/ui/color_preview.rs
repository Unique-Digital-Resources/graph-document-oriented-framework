use engine::core::graph::Graph;
use engine::core::node::properties::PropertyValue;
use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;
use uuid::Uuid;
use crate::systems::color_math;

pub fn create(target: Uuid) -> CustomWidget {
    CustomWidget {
        base: UiNode::new("ColorPreview", UiNodeRole::Custom),
        kind: "color-preview".to_string(),
        data: serde_json::json!({
            "target_node": target.to_string(),
            "hex": "#FF0000",
            "value": [0.0, 100.0, 50.0, 1.0]
        }),
        event_listeners: vec!["change".to_string()],
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
                    c.data["value"] = serde_json::Value::Array(json_arr);
                    
                    let hsla: Vec<f64> = color_val.iter().filter_map(|v| {
                        if let PropertyValue::Float(f) = v { Some(*f) } else { None }
                    }).collect();
                    if hsla.len() == 4 { 
                        c.data["hex"] = serde_json::Value::String(color_math::hsla_to_hex(&hsla)); 
                    }
                }
            }
        }
    }
}