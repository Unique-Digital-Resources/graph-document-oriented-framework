use wasm_bindgen::prelude::*;
use engine::core::command::pipeline::CommandPipeline;
use engine::core::graph::Graph;
use engine::core::history::HistoryStack;
use engine::core::signal::EventBus;
use engine::core::command::registry::CommandRegistry;
use headless_ui::view_graph::storage::ViewGraph;
use headless_ui::nodes::widgets::WidgetKind; // ADD THIS IMPORT
use web_bridge::dom_mapper::DomMapper;
use web_bridge::input_bridge::{DomEvent, InputDispatcher};

mod document;
mod commands;
mod systems;
mod ui;

struct AppState {
    graph: Graph,
    view: ViewGraph,
    registry: CommandRegistry,
    bus: EventBus,
    history: HistoryStack,
    rect_node_id: uuid::Uuid,
}

thread_local! {
    static STATE: std::cell::RefCell<Option<AppState>> = std::cell::RefCell::new(None);
}

#[wasm_bindgen]
pub fn init_app() {
    console_error_panic_hook::set_once();
    
    let mut graph = Graph::new();
    let mut view = ViewGraph::new();
    let mut registry = CommandRegistry::new();
    
    let rect_id = document::init_document(&mut graph);
    let palette_node = graph.iter_nodes().find(|n| n.type_id.as_str() == "PaletteNode").unwrap();
    let palette_id = palette_node.id;
    
    commands::register_commands(&mut registry);
    ui::init_view(&mut view, rect_id, palette_id);

    STATE.with(|s| {
        *s.borrow_mut() = Some(AppState {
            graph, view, registry, bus: EventBus::new(), history: HistoryStack::new(), rect_node_id: rect_id,
        });
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
pub fn handle_dom_event(event_json: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(state) = state.as_mut() {
            if let Ok(event) = serde_json::from_str::<serde_json::Value>(event_json) {
                let command_id = event["command_id"].as_str().unwrap_or("").to_string();
                let params = event["params"].clone();
                
                // INTERCEPT UI Commands: Mutate ViewGraph directly
                if command_id == "SetWheelMode" {
                    if let Some(target_str) = params["target_node"].as_str() {
                        if let Ok(target) = uuid::Uuid::parse_str(target_str) {
                            // FIX: Wrap target in UiNodeId
                            if let Some(WidgetKind::Custom(c)) = state.view.get_mut(headless_ui::UiNodeId(target)) {
                                if let Some(mode) = params["mode"].as_str() {
                                    c.data["mode"] = serde_json::Value::String(mode.to_string());
                                }
                            }
                        }
                    }
                    return; 
                }

                // ROUTE Document Commands to Pipeline
                if let Some(target_str) = event["target"].as_str() {
                    if let Ok(target) = uuid::Uuid::parse_str(target_str) {
                        let dom_event = match event["type"].as_str().unwrap_or("") {
                            "click" => DomEvent::Click { target },
                            "custom" => DomEvent::Custom { target, command_id: command_id.clone(), params: params.clone() },
                            _ => return
                        };
                        let mut pipeline = CommandPipeline::new(&state.registry, &mut state.graph, &mut state.bus, &mut state.history);
                        let _ = InputDispatcher::dispatch(dom_event, &state.view, &mut pipeline);
                    }
                }
            }
        }
    });
}