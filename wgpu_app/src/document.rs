use engine::core::graph::storage::Graph;
use engine::core::node::node::Node;
use engine::core::node::properties::PropertyValue;

pub fn initialize_document() -> Graph {
    let mut graph = Graph::new();

    let scene_node = Node::new("SceneNode")
        .set_persistent("name", PropertyValue::String("Main Scene".to_string()));
    graph.insert_node(scene_node);

    let cam_node = Node::new("CameraNode")
        .set_persistent("position", PropertyValue::Array(vec![
            PropertyValue::Float(5.0), PropertyValue::Float(5.0), PropertyValue::Float(5.0)
        ]))
        .set_persistent("target", PropertyValue::Array(vec![
            PropertyValue::Float(0.0), PropertyValue::Float(0.0), PropertyValue::Float(0.0)
        ]))
        .set_persistent("up", PropertyValue::Array(vec![
            PropertyValue::Float(0.0), PropertyValue::Float(1.0), PropertyValue::Float(0.0)
        ]))
        .set_persistent("fov", PropertyValue::Float(45.0))
        .set_transient("is_dragging", PropertyValue::Bool(false));
    graph.insert_node(cam_node);

    let grid_node = Node::new("GridSettingsNode")
        .set_persistent("base_size", PropertyValue::Float(1.0))
        .set_persistent("plane_y", PropertyValue::Float(0.0))
        .set_persistent("color", PropertyValue::Array(vec![
            PropertyValue::Float(0.8), PropertyValue::Float(0.8), PropertyValue::Float(0.8)
        ]))
        .set_persistent("fade_dist", PropertyValue::Float(1000.0));
    graph.insert_node(grid_node);

    let vertices = generate_cube_vertices();
    let mut vert_props = Vec::new();
    for v in vertices {
        vert_props.push(PropertyValue::Float(v as f64));
    }

    let indices = generate_cube_indices();
    let mut idx_props = Vec::new();
    for i in indices {
        idx_props.push(PropertyValue::Int(i as i64));
    }

    let mesh_node = Node::new("MeshNode")
        .set_persistent("name", PropertyValue::String("Cube".to_string()))
        .set_persistent("position", PropertyValue::Array(vec![
            PropertyValue::Float(0.0), PropertyValue::Float(0.5), PropertyValue::Float(0.0)
        ]))
        .set_persistent("vertices", PropertyValue::Array(vert_props))
        .set_persistent("indices", PropertyValue::Array(idx_props));
    graph.insert_node(mesh_node);

    graph
}

fn generate_cube_vertices() -> Vec<f32> {
    vec![
        -0.5, -0.5,  0.5,  1.0, 0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0, 0.0,
         0.5,  0.5, -0.5,  0.0, 1.0, 0.0,
         0.5, -0.5, -0.5,  0.0, 1.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 0.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 0.0, 1.0,
         0.5,  0.5,  0.5,  0.0, 0.0, 1.0,
         0.5,  0.5, -0.5,  0.0, 0.0, 1.0,
        -0.5, -0.5, -0.5,  1.0, 1.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 1.0, 0.0,
        -0.5, -0.5,  0.5,  1.0, 1.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 0.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0, 1.0,
    ]
}

fn generate_cube_indices() -> Vec<u32> {
    vec![
        0,  1,  2,    0,  2,  3,
        4,  5,  6,    4,  6,  7,
        8,  9,  10,   8,  10, 11,
        12, 13, 14,   12, 14, 15,
        16, 17, 18,   16, 18, 19,
        20, 21, 22,   20, 22, 23,
    ]
}