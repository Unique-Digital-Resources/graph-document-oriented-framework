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
    let mesh_id = graph.insert_node(mesh_node);

    let child_schema = children();
    let _ = graph.add_edge(&child_schema, scene_id, mesh_id);

    for _ in 0..6 {
        let mut face_node = Node::new("FaceNode");
        face_node.properties.set_persistent("color", PropertyValue::Array(vec![1.0.into(), 0.0.into(), 0.0.into()]));
        let face_id = graph.insert_node(face_node);
        let _ = graph.add_edge(&child_schema, mesh_id, face_id);
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
            "position": mesh.properties.get_value("position").unwrap(),
            "rotation": mesh.properties.get_value("rotation").unwrap(),
            "scale": mesh.properties.get_value("scale").unwrap(),
            "faces": faces
        }
    }).to_string()
}