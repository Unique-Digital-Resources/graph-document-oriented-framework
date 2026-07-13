// example_app/src/document.rs
use engine::core::graph::Graph;
use engine::core::node::Node;
use engine::core::node::properties::PropertyValue;
use engine::core::relation::presets::children;

pub fn init_scene(graph: &mut Graph) {
    let scene_node = Node::new("SceneNode");
    let scene_id = graph.insert_node(scene_node);

    let mut mesh_node = Node::new("MeshNode");
    mesh_node.properties.set_persistent("position", PropertyValue::Array(vec![0.0.into(), 0.0.into(), 0.0.into()]));
    mesh_node.properties.set_persistent("rotation", PropertyValue::Array(vec![0.0.into(), 0.0.into(), 0.0.into()]));
    mesh_node.properties.set_persistent("scale", PropertyValue::Array(vec![1.0.into(), 1.0.into(), 1.0.into()]));
    
    // Store face colors as an array of arrays. 6 faces, default red [1, 0, 0].
    let face_colors = PropertyValue::Array((0..6).map(|_| {
        PropertyValue::Array(vec![1.0.into(), 0.0.into(), 0.0.into()])
    }).collect());
    mesh_node.properties.set_persistent("face_colors", face_colors);
    
    let mesh_id = graph.insert_node(mesh_node);

    let child_schema = children();
    let _ = graph.add_edge(&child_schema, scene_id, mesh_id);
}

pub fn get_scene_json(graph: &Graph) -> String {
    let mesh = match graph.iter_nodes().find(|n| n.type_id.as_str() == "MeshNode") {
        Some(m) => m,
        None => return "{}".to_string(),
    };

    serde_json::json!({
        "mesh": {
            "id": mesh.id.to_string(),
            "position": mesh.properties.get_value("position").unwrap(),
            "rotation": mesh.properties.get_value("rotation").unwrap(),
            "scale": mesh.properties.get_value("scale").unwrap(),
            "face_colors": mesh.properties.get_value("face_colors").unwrap()
        }
    }).to_string()
}