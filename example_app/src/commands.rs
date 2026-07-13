// example_app/src/commands.rs
use engine::core::command::{CommandDefinition, CommandRegistry};
use engine::core::graph::Graph;
use engine::core::node::properties::PropertyValue;
use engine::core::signal::EventBus;
use uuid::Uuid;
use std::sync::Arc;

fn json_array_to_prop_array(val: &serde_json::Value) -> PropertyValue {
    if let Some(arr) = val.as_array() {
        let props: Vec<PropertyValue> = arr.iter().map(|v| {
            if let Some(f) = v.as_f64() { PropertyValue::Float(f) }
            else if let Some(i) = v.as_i64() { PropertyValue::Int(i) }
            else { PropertyValue::Float(0.0) }
        }).collect();
        PropertyValue::Array(props)
    } else { PropertyValue::Array(vec![]) }
}

pub fn register_commands(registry: &mut CommandRegistry) {
    let exec_transform = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let mesh_id_str = p["mesh_id"].as_str().ok_or("mesh_id missing")?;
        let mesh_id = Uuid::parse_str(mesh_id_str).map_err(|e| e.to_string())?;
        let prop = p["property"].as_str().ok_or("property missing")?;
        let val = p.get("value").ok_or("value missing")?;
        let prop_val = json_array_to_prop_array(val);
        if let Some(node) = g.get_node_mut(mesh_id) {
            node.properties.set_persistent(prop, prop_val);
            Ok(())
        } else { Err("Mesh node not found".into()) }
    });
    registry.register(CommandDefinition::new("SetMeshTransform", "Set Transform", "Updates mesh transform").with_execute(exec_transform));

    // Updated to mutate an array element inside the MeshNode
    let exec_color = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let mesh_id_str = p["mesh_id"].as_str().ok_or("mesh_id missing")?;
        let mesh_id = Uuid::parse_str(mesh_id_str).map_err(|e| e.to_string())?;
        let face_index = p["face_index"].as_u64().ok_or("face_index missing")? as usize;
        let val = p.get("value").ok_or("value missing")?;
        let new_color = json_array_to_prop_array(val);

        if let Some(node) = g.get_node_mut(mesh_id) {
            if let Some(PropertyValue::Array(colors)) = node.properties.get_value_mut("face_colors") {
                if face_index < colors.len() {
                    colors[face_index] = new_color;
                    return Ok(());
                }
            }
            Err("Face colors array not found or index out of bounds".into())
        } else { Err("Mesh node not found".into()) }
    });
    registry.register(CommandDefinition::new("SetFaceColor", "Set Face Color", "Updates face color").with_execute(exec_color));
}