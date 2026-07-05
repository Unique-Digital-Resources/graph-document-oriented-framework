//! WASM API Bridge.
//! Exposes the framework core to JavaScript via wasm-bindgen.

use wasm_bindgen::prelude::*;

use crate::core::command::{CommandDefinition, CommandRegistry};
use crate::core::command::pipeline::CommandPipeline;
use crate::core::graph::Graph;
use crate::core::history::HistoryStack;
use crate::core::node::Node;
use crate::core::node::properties::PropertyValue;
use crate::core::signal::EventBus;
use crate::ui::headless::nodes::widgets::{ButtonNode, ContainerLayout, ContainerNode, WidgetKind};
use crate::ui::headless::view_graph::storage::ViewGraph;
use crate::ui::web::dom_mapper::DomMapper;
use crate::ui::web::input_bridge::{DomEvent, InputDispatcher};

// Global state. In a real app, you might wrap this in a struct exported to JS.
struct AppState {
    graph: Graph,
    view: ViewGraph,
    registry: CommandRegistry,
    bus: EventBus,
    history: HistoryStack,
}

// A simple thread-local global state (WASM is single-threaded by default)
thread_local! {
    static STATE: std::cell::RefCell<Option<AppState>> = std::cell::RefCell::new(None);
}

/// Initializes the application. Called by JS on startup.
#[wasm_bindgen]
pub fn init_app() {
    let mut graph = Graph::new();
    let mut view = ViewGraph::new();
    
    // Setup a mock document graph
    let doc_node = Node::new("TaskNode").set_persistent("title", PropertyValue::String("Buy milk".to_string()));
    let doc_id = graph.insert_node(doc_node);

    // Setup a mock View Graph
    let root_widget = ContainerNode::new(ContainerLayout::Column);
    let root_id = view.insert(WidgetKind::Container(root_widget));
    view.set_root(root_id).unwrap();

    let mut btn = ButtonNode::new("Complete Task");
    btn.command_id = Some("CompleteTask".to_string());
    btn.command_params.insert("taskId".to_string(), PropertyValue::Uuid(doc_id));
    
    let btn_id = view.insert(WidgetKind::Button(btn));
    view.attach(root_id, btn_id).unwrap();

    // Setup command registry
    let mut registry = CommandRegistry::new();
    let exec_fn = std::sync::Arc::new(move |g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let task_id = uuid::Uuid::parse_str(p["taskId"].as_str().unwrap()).unwrap();
        if let Some(node) = g.get_node_mut(task_id) {
            node.properties.set_persistent("isCompleted", PropertyValue::Bool(true));
        }
        Ok(())
    });
    registry.register(
        CommandDefinition::new("CompleteTask", "Complete", "Completes").with_execute(exec_fn)
    );

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

/// Returns the current UI state as a JSON string for JS to render.
#[wasm_bindgen]
pub fn get_ui_state() -> String {
    STATE.with(|s| {
        let state = s.borrow();
        let state = state.as_ref().unwrap();
        
        let dom = DomMapper::map(&state.view).unwrap();
        serde_json::to_string(&dom).unwrap()
    })
}

/// Receives a DOM event from JS (e.g., a button click) and dispatches it.
#[wasm_bindgen]
pub fn handle_dom_event(event_json: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let state = state.as_mut().unwrap();
        
        if let Ok(event) = serde_json::from_str::<serde_json::Value>(event_json) {
            let target = uuid::Uuid::parse_str(event["target"].as_str().unwrap()).unwrap();
            let event_type = event["type"].as_str().unwrap();
            
            let dom_event = match event_type {
                "click" => DomEvent::Click { target },
                "input" => DomEvent::Input { 
                    target, 
                    value: event["value"].as_str().unwrap_or("").to_string() 
                },
                _ => return
            };

            let mut pipeline = CommandPipeline::new(
                &state.registry, 
                &mut state.graph, 
                &mut state.bus, 
                &mut state.history
            );
            
            let _ = InputDispatcher::dispatch(dom_event, &state.view, &mut pipeline);
        }
    });
}