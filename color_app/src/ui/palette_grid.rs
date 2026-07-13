use engine::core::graph::Graph;
use engine::core::node::properties::PropertyValue;
use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;
use uuid::Uuid;

pub fn create(target_palette: Uuid) -> CustomWidget {
    CustomWidget {
        base: UiNode::new("PaletteGrid", UiNodeRole::Custom),
        kind: "palette-grid".to_string(),
        data: serde_json::json!({
            "target_node": target_palette.to_string(),
            "plates_json": "[]",
            "document_color": [0.0, 100.0, 50.0, 1.0] 
        }),
        event_listeners: vec!["click".to_string(), "change".to_string()],
    }
}

pub fn sync(c: &mut CustomWidget, graph: &Graph) {
    // Sync Palette State
    if let Some(target_str) = c.data["target_node"].as_str() {
        if let Ok(target_id) = Uuid::parse_str(target_str) {
            if let Some(node) = graph.get_node(target_id) {
                if let Some(PropertyValue::String(val)) = node.properties.get_value("plates_json") {
                    c.data["plates_json"] = serde_json::Value::String(val.clone());
                }
            }
        }
    }

    // Sync Document Color (search graph for RectTestNode)
    let rect_node_id = graph.iter_nodes().find(|n| n.type_id.as_str() == "RectTestNode").map(|n| n.id);
    if let Some(rect_id) = rect_node_id {
        if let Some(rect_node) = graph.get_node(rect_id) {
            if let Some(PropertyValue::Array(color_val)) = rect_node.properties.get_value("color") {
                let json_arr: Vec<serde_json::Value> = color_val.iter().filter_map(|v| {
                    if let PropertyValue::Float(f) = v { Some(serde_json::json!(*f)) } else { None }
                }).collect();
                c.data["document_color"] = serde_json::Value::Array(json_arr);
            }
        }
    }
}