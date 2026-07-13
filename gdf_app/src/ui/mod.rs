use engine::core::graph::Graph;
use headless_ui::view_graph::storage::ViewGraph;
use headless_ui::nodes::widgets::{WidgetKind, ContainerNode, ContainerLayout};
use headless_ui::nodes::ui_node::Bounds;

pub mod viewport;
pub mod vector3_input;
pub mod color_picker;

pub fn build_ui(view: &mut ViewGraph, graph: &Graph) {
    let mut root_widget = ContainerNode::new(ContainerLayout::Row);
    root_widget.base.bounds = Bounds::new(0.0, 0.0, 1000.0, 600.0);
    let root_id = view.insert(WidgetKind::Container(root_widget));
    let _ = view.set_root(root_id);

    // Viewport
    let mut vp_widget = viewport::Viewport3DNode::new();
    vp_widget.base.bounds = Bounds::new(0.0, 0.0, 700.0, 600.0);
    let vp_id = view.insert(WidgetKind::Custom(vp_widget));
    let _ = view.attach(root_id, vp_id);

    // Inspector
    let mut insp_widget = ContainerNode::new(ContainerLayout::Column);
    insp_widget.base.bounds = Bounds::new(700.0, 0.0, 300.0, 600.0);
    let insp_id = view.insert(WidgetKind::Container(insp_widget));
    let _ = view.attach(root_id, insp_id);

    // Transform Inputs
    let mesh = graph.iter_nodes().find(|n| n.type_id.as_str() == "MeshNode").unwrap();
    for prop in ["position", "rotation", "scale"] {
        let input = vector3_input::Vector3InputNode::new(mesh.id, prop, graph);
        let id = view.insert(WidgetKind::Custom(input));
        let _ = view.attach(insp_id, id);
    }

    // Color Picker (Face 4 - The front face)
    let face_ids = graph.get_targets(mesh.id, "CHILDREN");
    let color_input = color_picker::ColorPickerNode::new(face_ids[4], graph); 
    let col_id = view.insert(WidgetKind::Custom(color_input));
    let _ = view.attach(insp_id, col_id);
}

/// Syncs the Document Graph state into the View Graph before rendering.
pub fn sync_ui(view: &mut ViewGraph, graph: &Graph) {
    let mut stack = match view.root() {
        Some(r) => vec![r],
        None => return,
    };
    
    while let Some(id) = stack.pop() {
        if let Some(widget) = view.get_mut(id) {
            if let WidgetKind::Custom(c) = widget {
                if let Some(target_str) = c.data["target_node"].as_str() {
                    if let Ok(target_id) = uuid::Uuid::parse_str(target_str) {
                        if let Some(node) = graph.get_node(target_id) {
                            if c.kind == "vector3-input" {
                                if let Some(prop) = c.data["property"].as_str() {
                                    if let Some(val) = node.properties.get_value(prop) {
                                        c.data["value"] = serde_json::to_value(val).unwrap_or(serde_json::Value::Null);
                                    }
                                }
                            } else if c.kind == "color-picker" {
                                if let Some(val) = node.properties.get_value("color") {
                                    c.data["value"] = serde_json::to_value(val).unwrap_or(serde_json::Value::Null);
                                }
                            }
                        }
                    }
                }
            }
        }
        stack.extend(view.children(id));
    }
}

pub fn get_scene_json(graph: &Graph) -> String {
    let mesh = match graph.iter_nodes().find(|n| n.type_id.as_str() == "MeshNode") {
        Some(m) => m,
        None => return "{}".to_string(),
    };
    
    let face_ids = graph.get_targets(mesh.id, "CHILDREN");
    
    let faces: Vec<serde_json::Value> = face_ids.iter().map(|fid| {
        let face = graph.get_node(*fid).unwrap();
        serde_json::json!({
            "id": fid.to_string(),
            "color": face.properties.get_value("color").unwrap()
        })
    }).collect();

    serde_json::json!({
        "mesh": {
            "id": mesh.id.to_string(),
            "position": mesh.properties.get_value("position").unwrap_or(&engine::core::node::properties::PropertyValue::Null),
            "rotation": mesh.properties.get_value("rotation").unwrap_or(&engine::core::node::properties::PropertyValue::Null),
            "scale": mesh.properties.get_value("scale").unwrap_or(&engine::core::node::properties::PropertyValue::Null),
            "faces": faces
        }
    }).to_string()
}