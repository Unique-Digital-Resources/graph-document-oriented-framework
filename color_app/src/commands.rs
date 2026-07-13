use engine::core::command::{CommandDefinition, CommandRegistry};
use engine::core::graph::Graph;
use engine::core::node::properties::PropertyValue;
use engine::core::signal::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use crate::systems::color_math;

pub fn register_commands(registry: &mut CommandRegistry) {
    
    // --- COLOR COMMANDS ---

    let exec_set_color_prop = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let prop = p["prop"].as_str().ok_or("prop missing")?;
        let index = p["index"].as_u64().ok_or("index missing")? as usize;
        let val = p["value"].as_f64().ok_or("value missing")?;

        if let Some(node) = g.get_node_mut(target) {
            if let Some(PropertyValue::Array(arr)) = node.properties.get_value_mut(prop) {
                if index < arr.len() {
                    arr[index] = val.into();
                }
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("SetDirectColorProp", "Set Color Property", "Updates HSLA array index").with_execute(exec_set_color_prop));

    let exec_set_hex = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let hex = p["hex"].as_str().ok_or("hex missing")?;

        if let Some(hsla) = color_math::hex_to_hsla(hex) {
            if let Some(node) = g.get_node_mut(target) {
                node.properties.set_persistent("color", PropertyValue::Array(
                    hsla.iter().map(|v| (*v).into()).collect()
                ));
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("SetDirectHexColor", "Set Hex Color", "Parses hex and updates color array").with_execute(exec_set_hex));

    let exec_set_harmony_colors = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let colors_json = p["colors"].as_array().ok_or("colors array missing")?;
        
        let new_colors: Vec<PropertyValue> = colors_json.iter().map(|c| {
            let arr = c.as_array().unwrap();
            PropertyValue::Array(arr.iter().map(|v| PropertyValue::Float(v.as_f64().unwrap())).collect())
        }).collect();

        if let Some(node) = g.get_node_mut(target) {
            node.properties.set_persistent("harmony_colors", PropertyValue::Array(new_colors));
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("SetHarmonyColors", "Set Harmony Colors", "Updates the harmony colors array").with_execute(exec_set_harmony_colors));

    // --- PALETTE COMMANDS ---

    let exec_add_plate = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        if let Some(node) = g.get_node_mut(target) {
            if let Some(PropertyValue::String(json_str)) = node.properties.get_value("plates_json") {
                let mut plates: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!([]));
                if let Some(arr) = plates.as_array_mut() {
                    let new_id = format!("plate-{}", Uuid::new_v4());
                    arr.push(serde_json::json!({ "id": new_id, "name": "New Plate", "colors": [], "harmonies": [] }));
                }
                node.properties.set_persistent("plates_json", PropertyValue::String(plates.to_string()));
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("AddPalettePlate", "Add Plate", "Creates a new palette plate").with_execute(exec_add_plate));

    let exec_remove_plate = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let plate_id = p["plate_id"].as_str().ok_or("plate_id missing")?;
        if let Some(node) = g.get_node_mut(target) {
            if let Some(PropertyValue::String(json_str)) = node.properties.get_value("plates_json") {
                let mut plates: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!([]));
                if let Some(arr) = plates.as_array_mut() {
                    arr.retain(|p| p["id"].as_str() != Some(plate_id));
                    if arr.is_empty() {
                        arr.push(serde_json::json!({ "id": "plate-1", "name": "Palette 1", "colors": [], "harmonies": [] }));
                    }
                }
                node.properties.set_persistent("plates_json", PropertyValue::String(plates.to_string()));
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("RemovePalettePlate", "Remove Plate", "Deletes a palette plate").with_execute(exec_remove_plate));

    let exec_rename_plate = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let plate_id = p["plate_id"].as_str().ok_or("plate_id missing")?;
        let new_name = p["name"].as_str().ok_or("name missing")?;
        if let Some(node) = g.get_node_mut(target) {
            if let Some(PropertyValue::String(json_str)) = node.properties.get_value("plates_json") {
                let mut plates: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!([]));
                if let Some(arr) = plates.as_array_mut() {
                    for plate in arr.iter_mut() {
                        if plate["id"].as_str() == Some(plate_id) {
                            plate["name"] = serde_json::Value::String(new_name.to_string());
                        }
                    }
                }
                node.properties.set_persistent("plates_json", PropertyValue::String(plates.to_string()));
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("RenamePalettePlate", "Rename Plate", "Renames a palette plate").with_execute(exec_rename_plate));

    let exec_add_swatch = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let plate_id = p["plate_id"].as_str().ok_or("plate_id missing")?;
        let swatch = p["swatch"].clone();
        
        if let Some(node) = g.get_node_mut(target) {
            if let Some(PropertyValue::String(json_str)) = node.properties.get_value("plates_json") {
                let mut plates: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!([]));
                if let Some(arr) = plates.as_array_mut() {
                    for plate in arr.iter_mut() {
                        if plate["id"].as_str() == Some(plate_id) {
                            if let Some(colors) = plate["colors"].as_array_mut() {
                                // FIX: Check for duplicate color
                                let mut existing_index = None;
                                for (i, c) in colors.iter().enumerate() {
                                    if *c == swatch {
                                        existing_index = Some(i);
                                        break;
                                    }
                                }
                                
                                if let Some(idx) = existing_index {
                                    // If it exists, remove it so we can move it to the front
                                    colors.remove(idx);
                                }
                                // Insert at the front (index 0)
                                colors.insert(0, swatch.clone());
                            }
                        }
                    }
                }
                node.properties.set_persistent("plates_json", PropertyValue::String(plates.to_string()));
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("AddColorToPlate", "Add Swatch", "Adds color to plate or moves to front").with_execute(exec_add_swatch));

    let exec_remove_swatch = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let plate_id = p["plate_id"].as_str().ok_or("plate_id missing")?;
        let swatch_type = p["type"].as_str().ok_or("type missing")?;
        let index = p["index"].as_u64().ok_or("index missing")? as usize;
        
        if let Some(node) = g.get_node_mut(target) {
            if let Some(PropertyValue::String(json_str)) = node.properties.get_value("plates_json") {
                let mut plates: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!([]));
                if let Some(arr) = plates.as_array_mut() {
                    for plate in arr.iter_mut() {
                        if plate["id"].as_str() == Some(plate_id) {
                            let key = if swatch_type == "color" { "colors" } else { "harmonies" };
                            if let Some(arr2) = plate[key].as_array_mut() {
                                if index < arr2.len() { arr2.remove(index); }
                            }
                        }
                    }
                }
                node.properties.set_persistent("plates_json", PropertyValue::String(plates.to_string()));
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("RemoveSwatch", "Remove Swatch", "Removes swatch from plate").with_execute(exec_remove_swatch));

    let exec_load_swatch = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let swatch = p["swatch"].clone();
        
        if let Some(node) = g.get_node_mut(target) {
            if let Some(arr) = swatch.as_array() {
                let new_colors: Vec<PropertyValue> = arr.iter().map(|v| {
                    PropertyValue::Float(v.as_f64().unwrap())
                }).collect();
                node.properties.set_persistent("color", PropertyValue::Array(new_colors));
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("LoadSwatchToDocument", "Load Swatch", "Loads swatch to document").with_execute(exec_load_swatch));

    let exec_reorder_plates = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let from = p["from"].as_u64().ok_or("from missing")? as usize;
        let to = p["to"].as_u64().ok_or("to missing")? as usize;
        if let Some(node) = g.get_node_mut(target) {
            if let Some(PropertyValue::String(json_str)) = node.properties.get_value("plates_json") {
                let mut plates: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!([]));
                if let Some(arr) = plates.as_array_mut() {
                    if from < arr.len() && to <= arr.len() {
                        let item = arr.remove(from);
                        arr.insert(to, item);
                    }
                }
                node.properties.set_persistent("plates_json", PropertyValue::String(plates.to_string()));
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("ReorderPlates", "Reorder Plates", "Reorders plates").with_execute(exec_reorder_plates));

    let exec_reorder_swatches = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let target = Uuid::parse_str(p["target_node"].as_str().ok_or("target_node missing")?).map_err(|e| e.to_string())?;
        let plate_id = p["plate_id"].as_str().ok_or("plate_id missing")?;
        let swatch_type = p["type"].as_str().ok_or("type missing")?;
        let from = p["from"].as_u64().ok_or("from missing")? as usize;
        let to = p["to"].as_u64().ok_or("to missing")? as usize;
        
        if let Some(node) = g.get_node_mut(target) {
            if let Some(PropertyValue::String(json_str)) = node.properties.get_value("plates_json") {
                let mut plates: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!([]));
                if let Some(arr) = plates.as_array_mut() {
                    for plate in arr.iter_mut() {
                        if plate["id"].as_str() == Some(plate_id) {
                            let key = if swatch_type == "color" { "colors" } else { "harmonies" };
                            if let Some(arr2) = plate[key].as_array_mut() {
                                if from < arr2.len() && to <= arr2.len() {
                                    let item = arr2.remove(from);
                                    arr2.insert(to, item);
                                }
                            }
                        }
                    }
                }
                node.properties.set_persistent("plates_json", PropertyValue::String(plates.to_string()));
            }
        }
        Ok(())
    });
    registry.register(CommandDefinition::new("ReorderSwatches", "Reorder Swatches", "Reorders swatches").with_execute(exec_reorder_swatches));
}