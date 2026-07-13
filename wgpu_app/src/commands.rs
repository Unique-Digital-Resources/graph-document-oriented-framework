use engine::core::graph::storage::Graph;
use engine::core::node::properties::PropertyValue;
use engine::core::signal::event_bus::EventBus;
use serde_json::Value;
use uuid::Uuid;

/// Command: CommitViewportCamera
pub fn commit_viewport_camera(
    graph: &mut Graph,
    _event_bus: &mut EventBus,
    params: Value,
) -> Result<(), String> {
    let id_str = params["camera_id"].as_str().ok_or("Missing 'camera_id'")?;
    let node_id = Uuid::parse_str(id_str).map_err(|e| e.to_string())?;

    let pos_arr = params["position"].as_array().ok_or("Missing 'position'")?;
    let target_arr = params["target"].as_array().ok_or("Missing 'target'")?;
    let up_arr = params["up"].as_array().ok_or("Missing 'up'")?;

    let to_pv_float_array = |arr: &[Value]| -> PropertyValue {
        PropertyValue::Array(
            arr.iter()
               .map(|v| PropertyValue::Float(v.as_f64().unwrap_or(0.0)))
               .collect()
        )
    };

    let node = graph.get_node_mut(node_id).ok_or("Node not found")?;
    
    // We must use properties.set_persistent directly on the node
    node.properties.set_persistent("position", to_pv_float_array(pos_arr));
    node.properties.set_persistent("target", to_pv_float_array(target_arr));
    node.properties.set_persistent("up", to_pv_float_array(up_arr));
    node.touch();

    Ok(())
}