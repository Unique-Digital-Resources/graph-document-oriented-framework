use engine::core::graph::storage::Graph;
use engine::core::node::node::Node;
use engine::core::node::properties::PropertyValue;

/// Initializes the headless UI graph for the 3D viewport
pub fn initialize_ui(graph: &mut Graph, camera_node_id: &str) {
    // 1. Create the root UI container
    let ui_root_node = Node::new("ContainerNode")
        .set_persistent("layout", PropertyValue::String("Stack".to_string()));
    let ui_root = graph.insert_node(ui_root_node);

    // 2. Create the Viewport 3D Widget
    let data_binding = serde_json::json!({
        "camera_id": camera_node_id
    }).to_string();

    let viewport_node = Node::new("Viewport3DWidget")
        .set_transient("tag", PropertyValue::String("viewport-3d".to_string()))
        .set_transient("data", PropertyValue::String(data_binding));
    
    let viewport_ui_id = graph.insert_node(viewport_node);

    // 3. Attach viewport to UI root
    // graph.add_edge(&CHILDREN_SCHEMA, ui_root, viewport_ui_id);
}