use wasm_bindgen::prelude::*;
use engine::core::command::pipeline::CommandPipeline;
use engine::core::graph::Graph;
use engine::core::history::HistoryStack;
use engine::core::node::Node;
use engine::core::node::properties::PropertyValue;
use engine::core::signal::EventBus;
use engine::scripting::parser::lexer::Lexer;
use engine::scripting::parser::syntax_tree::DslParser;
use engine::scripting::runtime::compiler::DslCompiler;
use headless_ui::view_graph::storage::ViewGraph;
use web_bridge::dom_mapper::DomMapper;
use web_bridge::input_bridge::{DomEvent, InputDispatcher};
use std::sync::Arc;
use uuid::Uuid;

mod ui;

struct AppState {
    graph: Graph,
    view: ViewGraph,
    registry: engine::core::command::registry::CommandRegistry,
    bus: EventBus,
    history: HistoryStack,
}

thread_local! {
    static STATE: std::cell::RefCell<Option<AppState>> = std::cell::RefCell::new(None);
}

/// Helper to parse a string "1.0,2.0,3.0" into PropertyValue::Array
fn parse_vec3_string(val: &str) -> PropertyValue {
    let floats: Vec<PropertyValue> = val.split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .map(PropertyValue::Float)
        .collect();
    PropertyValue::Array(floats)
}

#[wasm_bindgen]
pub fn init_app() {
    console_error_panic_hook::set_once();
    
    let mut graph = Graph::new();
    let mut view = ViewGraph::new();
    let mut registry = engine::core::command::registry::CommandRegistry::new();
    let mut bus = EventBus::new();
    let mut history = HistoryStack::new();

    // 1. Load and compile the DSL Script!
    let script = include_str!("../scripts/3d_app.gdf");
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();
    
    let mut parser = DslParser::new(tokens);
    if let Ok(ast_nodes) = parser.parse_program() {
        let compiled_schemas = DslCompiler::compile(ast_nodes);
        
        // 2. Instantiate nodes using the compiled schemas
        for schema in compiled_schemas {
            if schema.type_id == "SceneNode" || schema.type_id == "MeshNode" || schema.type_id == "FaceNode" {
                let mut node = Node::new(&schema.type_id);
                
                // Apply default properties from the DSL!
                for prop in schema.properties {
                    // The DSL currently parses "1.0,1.0,1.0" as a string.
                    // We intercept it here to turn it into an Array.
                    if prop.name == "position" || prop.name == "rotation" || prop.name == "scale" || prop.name == "color" {
                        if let PropertyValue::String(s) = &prop.default_value {
                            let val = parse_vec3_string(s);
                            node.properties.set_persistent(&prop.name, val);
                        }
                    } else {
                        node.properties.set_persistent(&prop.name, prop.default_value.clone());
                    }
                }
                graph.insert_node(node);
            }
        }
    }

    // 3. Wire up the graph relations
    let scene = graph.iter_nodes().find(|n| n.type_id.as_str() == "SceneNode").map(|n| n.id).unwrap();
    let mesh = graph.iter_nodes().find(|n| n.type_id.as_str() == "MeshNode").map(|n| n.id).unwrap();
    
    let child_schema = engine::core::relation::presets::children();
    let _ = graph.add_edge(&child_schema, scene, mesh);
    
    // Create 6 faces for the mesh
    let face_schema = graph.iter_nodes().find(|n| n.type_id.as_str() == "FaceNode").map(|n| n.clone()).unwrap();
    for _ in 0..6 {
        // Create a new node with a fresh UUID, but copy the properties from the DSL schema
        let mut new_face = Node::new("FaceNode");
        new_face.properties = face_schema.properties.clone();
        
        let face = graph.insert_node(new_face);
        let _ = graph.add_edge(&child_schema, mesh, face);
    }

    // 4. Register Commands (Still in Rust until DSL supports commands)
    let exec_transform = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let mesh_id_str = p["mesh_id"].as_str().ok_or("mesh_id missing")?;
        let mesh_id = Uuid::parse_str(mesh_id_str).map_err(|e| e.to_string())?;
        let prop = p["property"].as_str().ok_or("property missing")?;
        let val = p.get("value").ok_or("value missing")?;
        
        // Convert JSON array to PropertyValue::Array
        let arr = val.as_array().ok_or("value not array")?;
        let props: Vec<PropertyValue> = arr.iter().filter_map(|v| v.as_f64().map(PropertyValue::Float)).collect();
        let prop_val = PropertyValue::Array(props);

        if let Some(node) = g.get_node_mut(mesh_id) {
            node.properties.set_persistent(prop, prop_val);
            Ok(())
        } else {
            Err("Mesh node not found".into())
        }
    });
    registry.register(engine::core::command::CommandDefinition::new("SetMeshTransform", "Set Transform", "Updates mesh transform").with_execute(exec_transform));

    let exec_color = Arc::new(|g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let face_id_str = p["face_id"].as_str().ok_or("face_id missing")?;
        let face_id = Uuid::parse_str(face_id_str).map_err(|e| e.to_string())?;
        let val = p.get("value").ok_or("value missing")?;
        
        let arr = val.as_array().ok_or("value not array")?;
        let props: Vec<PropertyValue> = arr.iter().filter_map(|v| v.as_f64().map(PropertyValue::Float)).collect();
        let prop_val = PropertyValue::Array(props);

        if let Some(node) = g.get_node_mut(face_id) {
            node.properties.set_persistent("color", prop_val);
            Ok(())
        } else {
            Err("Face node not found".into())
        }
    });
    registry.register(engine::core::command::CommandDefinition::new("SetFaceColor", "Set Face Color", "Updates face color").with_execute(exec_color));

    // 5. Setup UI
    ui::build_ui(&mut view, &graph);

    STATE.with(|s| {
        *s.borrow_mut() = Some(AppState { graph, view, registry, bus, history });
    });
}

#[wasm_bindgen]
pub fn get_ui_state() -> String {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(state) = state.as_mut() {
            ui::sync_ui(&mut state.view, &state.graph);
            if let Some(dom) = DomMapper::map(&state.view) {
                return serde_json::to_string(&dom).unwrap_or_else(|_| "{}".to_string());
            }
        }
        "{}".to_string()
    })
}

#[wasm_bindgen]
pub fn get_3d_scene() -> String {
    STATE.with(|s| {
        let state = s.borrow();
        if let Some(state) = state.as_ref() {
            return ui::get_scene_json(&state.graph);
        }
        "{}".to_string()
    })
}

#[wasm_bindgen]
pub fn handle_dom_event(event_json: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let state = match state.as_mut() {
            Some(st) => st,
            None => return,
        };
        
        let event = match serde_json::from_str::<serde_json::Value>(event_json) {
            Ok(e) => e,
            Err(_) => return,
        };

        let target_str = match event["target"].as_str() {
            Some(t) => t,
            None => return,
        };
        let target = match uuid::Uuid::parse_str(target_str) {
            Ok(u) => u,
            Err(_) => return,
        };
        
        let event_type = match event["type"].as_str() {
            Some(t) => t,
            None => return,
        };
        
        let dom_event = match event_type {
            "click" => DomEvent::Click { target },
            "custom" => {
                let command_id = match event["command_id"].as_str() {
                    Some(c) => c.to_string(),
                    None => return,
                };
                DomEvent::Custom {
                    target,
                    command_id,
                    params: event["params"].clone(),
                }
            },
            _ => return
        };

        let mut pipeline = CommandPipeline::new(
            &state.registry, &mut state.graph, &mut state.bus, &mut state.history
        );
        
        let _ = InputDispatcher::dispatch(dom_event, &state.view, &mut pipeline);
    });
}