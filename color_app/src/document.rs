use engine::core::graph::Graph;
use engine::core::node::Node;
use engine::core::node::properties::PropertyValue;
use engine::core::relation::presets::children;
use uuid::Uuid;

pub fn init_document(graph: &mut Graph) -> Uuid {
    let scene = graph.insert_node(Node::new("SceneRoot"));

    let mut rect = Node::new("RectTestNode");
    rect.properties.set_persistent("color", PropertyValue::Array(vec![
        0.0.into(), 100.0.into(), 50.0.into(), 1.0.into()
    ]));
    rect.properties.set_persistent("harmony_colors", PropertyValue::Array(vec![]));
    rect.properties.set_persistent("color_mode", PropertyValue::String("direct".to_string()));
    
    let rect_id = graph.insert_node(rect);
    let _ = graph.add_edge(&children(), scene, rect_id);

    let mut palette = Node::new("PaletteNode");
    // FIX: Store plates as a JSON string to easily handle complex nested arrays
    let initial_plates = serde_json::json!([
        { "id": "plate-1", "name": "Palette 1", "colors": [], "harmonies": [] },
        { "id": "plate-2", "name": "Favorites", "colors": [], "harmonies": [] }
    ]).to_string();
    palette.properties.set_persistent("plates_json", PropertyValue::String(initial_plates));
    
    let palette_id = graph.insert_node(palette);
    let _ = graph.add_edge(&children(), scene, palette_id);

    rect_id
}