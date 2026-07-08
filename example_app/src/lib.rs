// example_app/src/lib.rs
mod document;
mod commands;
mod ui;

use wasm_bindgen::prelude::*;
use engine::core::command::pipeline::CommandPipeline;
use engine::core::graph::Graph;
use engine::core::history::HistoryStack;
use engine::core::signal::EventBus;
use headless_ui::view_graph::storage::ViewGraph;
use web_bridge::dom_mapper::DomMapper;
use web_bridge::input_bridge::{DomEvent, InputDispatcher};

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

#[wasm_bindgen]
pub fn init_app() {
    console_error_panic_hook::set_once();
    
    let mut graph = Graph::new();
    let mut view = ViewGraph::new();
    let mut registry = engine::core::command::registry::CommandRegistry::new();
    
    document::init_scene(&mut graph);
    commands::register_commands(&mut registry);
    ui::build_ui(&mut view, &graph);

    STATE.with(|s| {
        *s.borrow_mut() = Some(AppState {
            graph,
            view,
            registry,
            bus: EventBus::new(),
            history: HistoryStack::new(),
        });
    });
}

#[wasm_bindgen]
pub fn get_ui_state() -> String {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(state) = state.as_mut() {
            // Sync the document graph state to the view graph before mapping
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
            return document::get_scene_json(&state.graph);
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