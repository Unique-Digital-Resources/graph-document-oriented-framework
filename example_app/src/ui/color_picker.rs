// example_app/src/ui/color_picker.rs
use engine::core::graph::Graph;
use engine::core::node::NodeId;
use engine::core::node::properties::PropertyValue;
use headless_ui::nodes::ui_node::{UiNode, UiNodeRole};
use headless_ui::nodes::widgets::CustomWidget;

pub struct ColorPickerNode;

impl ColorPickerNode {
    // Now takes mesh_id and face_index
    pub fn new(mesh_id: NodeId, face_index: usize, graph: &Graph) -> CustomWidget {
        let val = if let Some(node) = graph.get_node(mesh_id) {
            if let Some(PropertyValue::Array(colors)) = node.properties.get_value("face_colors") {
                colors.get(face_index).cloned()
            } else { None }
        } else { None };

        CustomWidget {
            base: UiNode::new("ColorPicker", UiNodeRole::Custom),
            kind: "color-picker".to_string(),
            data: serde_json::json!({
                "target_node": mesh_id.to_string(),
                "face_index": face_index,
                "value": val
            }),
            // Listen to 'input' for real-time updates!
            event_listeners: vec!["input".to_string()], 
        }
    }
}