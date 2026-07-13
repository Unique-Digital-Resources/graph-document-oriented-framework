use engine::core::graph::Graph;
use engine::core::node::properties::PropertyValue;
use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;
use uuid::Uuid;

fn create_base(target: Uuid, kind: &str, prop: &str, index: usize, min: f64, max: f64) -> CustomWidget {
    CustomWidget {
        base: UiNode::new("RangeSlider", UiNodeRole::Custom),
        kind: kind.to_string(),
        data: serde_json::json!({
            "target_node": target.to_string(),
            "prop": prop,
            "index": index,
            "min": min,
            "max": max,
            "value": 0.0
        }),
        event_listeners: vec!["input".to_string(), "change".to_string()],
    }
}

pub fn create_hue(target: Uuid) -> CustomWidget { create_base(target, "hue-slider", "color", 0, 0.0, 360.0) }
pub fn create_sat(target: Uuid) -> CustomWidget { create_base(target, "sat-slider", "color", 1, 0.0, 100.0) }
pub fn create_light(target: Uuid) -> CustomWidget { create_base(target, "light-slider", "color", 2, 0.0, 100.0) }
pub fn create_alpha(target: Uuid) -> CustomWidget { create_base(target, "alpha-slider", "color", 3, 0.0, 1.0) }

pub fn sync(c: &mut CustomWidget, graph: &Graph, wheel_mode: &str) {
    c.data["wheel_mode"] = serde_json::Value::String(wheel_mode.to_string());
    
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