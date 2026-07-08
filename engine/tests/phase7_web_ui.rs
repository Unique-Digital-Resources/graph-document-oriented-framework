use graph_document_framework::core::command::{CommandDefinition, CommandRegistry};
use graph_document_framework::core::graph::Graph;
use graph_document_framework::core::history::HistoryStack;
use graph_document_framework::core::node::Node;
use graph_document_framework::core::node::properties::PropertyValue;
use graph_document_framework::core::signal::EventBus;
use graph_document_framework::core::command::pipeline::CommandPipeline;

use graph_document_framework::ui::headless::nodes::widgets::{ButtonNode, ContainerLayout, ContainerNode, TextFieldNode, WidgetKind};
use graph_document_framework::ui::headless::view_graph::storage::ViewGraph;

use graph_document_framework::ui::web::canvas_renderer::Painter;
use graph_document_framework::ui::web::dom_mapper::DomMapper;
use graph_document_framework::ui::web::input_bridge::{DomEvent, InputDispatcher};
use graph_document_framework::ui::web::js_bridge::JS_BRIDGE_CODE;

use std::sync::Arc;
use uuid::Uuid;

#[test]
fn web_ui_rendering_and_input_dispatch() {
    let mut view = ViewGraph::new();
    let mut graph = Graph::new();
    
    // 1. Setup Document Graph
    let doc_node = Node::new("TaskNode").set_persistent("title", PropertyValue::string("Buy milk"));
    let doc_id = graph.insert_node(doc_node);

    // 2. Setup View Graph
    let root_widget = ContainerNode::new(ContainerLayout::Column);
    let root_id = view.insert(WidgetKind::Container(root_widget));
    view.set_root(root_id).unwrap();

    let mut btn = ButtonNode::new("Complete");
    btn.bind_command("CompleteTask".to_string(), std::collections::HashMap::from([
        ("taskId".to_string(), PropertyValue::Uuid(doc_id))
    ]));
    let btn_id = view.insert(WidgetKind::Button(btn));
    
    let mut tf = TextFieldNode::new("Enter title");
    tf.bind_to(doc_id, "title");
    let tf_id = view.insert(WidgetKind::TextField(tf));
    
    view.attach(root_id, btn_id).unwrap();
    view.attach(root_id, tf_id).unwrap();

    // 3. Test Canvas Rendering
    let draw_calls = Painter::render(&view);
    assert!(draw_calls.len() > 2); // ClearRect + root + button + textfield
    
    // 4. Test DOM Mapping (account for non-deterministic order)
    let dom = DomMapper::map(&view).unwrap();
    assert_eq!(dom.tag, "div"); // Root is container
    assert_eq!(dom.children.len(), 2);
    
    let btn_dom = dom.children.iter().find(|c| c.tag == "button").unwrap();
    assert_eq!(btn_dom.text, Some("Complete".to_string()));
    
    let input_dom = dom.children.iter().find(|c| c.tag == "input").unwrap();
    assert_eq!(input_dom.event_listeners, vec!["input".to_string(), "keydown".to_string()]);

    // 5. Test JS Bridge is generated
    assert!(JS_BRIDGE_CODE.contains("class GdfBridge"));

    // 6. Setup Command Pipeline
    let mut bus = EventBus::new();
    let mut history = HistoryStack::new();
    let mut registry = CommandRegistry::new();
    
    let exec_fn = Arc::new(move |g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let task_id = Uuid::parse_str(p["taskId"].as_str().unwrap()).unwrap();
        if let Some(node) = g.get_node_mut(task_id) {
            node.properties.set_persistent("isCompleted", PropertyValue::Bool(true));
        }
        Ok(())
    });
    
    let set_prop_fn = Arc::new(move |g: &mut Graph, _e: &mut EventBus, p: serde_json::Value| {
        let task_id = Uuid::parse_str(p["node_id"].as_str().unwrap()).unwrap();
        let prop = p["property"].as_str().unwrap();
        let val = p["value"].as_str().unwrap();
        if let Some(node) = g.get_node_mut(task_id) {
            node.properties.set_persistent(prop, PropertyValue::string(val));
        }
        Ok(())
    });

    registry.register(CommandDefinition::new("CompleteTask", "Complete", "Completes").with_execute(exec_fn));
    registry.register(CommandDefinition::new("SetProperty", "Set", "Sets prop").with_execute(set_prop_fn));

    // We scope the pipeline so the mutable borrow on `graph` is released 
    // before we try to assert the final state of the graph.
    {
        let mut pipeline = CommandPipeline::new(&registry, &mut graph, &mut bus, &mut history);

        // 7. Test Input Dispatch (Simulate Button Click)
        let click_event = DomEvent::Click { target: btn_id.0 };
        InputDispatcher::dispatch(click_event, &view, &mut pipeline).unwrap();

        // 8. Test Input Dispatch (Simulate Text Input)
        let input_event = DomEvent::Input { target: tf_id.0, value: "Buy coffee".to_string() };
        InputDispatcher::dispatch(input_event, &view, &mut pipeline).unwrap();
    }

    // 9. Assert final graph state
    let updated_node = graph.get_node(doc_id).unwrap();
    assert_eq!(updated_node.get("isCompleted"), Some(&PropertyValue::Bool(true)));
    assert_eq!(updated_node.get("title"), Some(&PropertyValue::string("Buy coffee")));
}