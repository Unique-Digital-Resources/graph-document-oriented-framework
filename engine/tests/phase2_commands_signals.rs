//use std::sync::{Arc, Mutex};

//use graph_document_framework::core::command::{CommandDefinition, CommandPipeline, CommandRegistry};
//use graph_document_framework::core::graph::Graph;
//use graph_document_framework::core::history::HistoryStack;
//use graph_document_framework::core::node::{Node, PropertyValue};
//use graph_document_framework::core::signal::{EmitTiming, EventBus, Signal};
//use serde_json::json;

use std::sync::Arc;

use graph_document_framework::core::command::{CommandDefinition, CommandPipeline, CommandRegistry};
use graph_document_framework::core::graph::Graph;
use graph_document_framework::core::history::HistoryStack;
use graph_document_framework::core::node::{Node, PropertyValue};
use graph_document_framework::core::signal::EventBus;
use serde_json::json;

#[test]
fn command_pipeline_supports_undo_and_history() {
    let mut graph = Graph::new();
    let mut event_bus = EventBus::new();
    let mut registry = CommandRegistry::new();
    let mut history = HistoryStack::new();

    // 1. Create a node to mutate
    let node_id = graph.insert_node(
        Node::new("TaskNode")
            .set_persistent("title", PropertyValue::string("Original"))
            .set_persistent("isCompleted", PropertyValue::bool(false)),
    );

    // 2. Define a command that mutates the node
    let execute_fn = Arc::new(move |graph: &mut Graph, _event_bus: &mut EventBus, params: serde_json::Value| {
        let node_id = uuid::Uuid::parse_str(params["node_id"].as_str().unwrap()).unwrap();
        
        if let Some(node) = graph.get_node_mut(node_id) {
            if let Some(val) = node.properties.get_value_mut("isCompleted") {
                *val = PropertyValue::Bool(true);
            }
            node.touch();
        }
        Ok(())
    });

    // Define an undo function that reverts the mutation
    let undo_fn = Arc::new(move |graph: &mut Graph, _event_bus: &mut EventBus, params: serde_json::Value| {
        let node_id = uuid::Uuid::parse_str(params["node_id"].as_str().unwrap()).unwrap();
        
        if let Some(node) = graph.get_node_mut(node_id) {
            if let Some(val) = node.properties.get_value_mut("isCompleted") {
                *val = PropertyValue::Bool(false);
            }
            node.touch();
        }
        Ok(())
    });

    let cmd_def = CommandDefinition::new("CompleteTask", "Complete Task", "Marks task complete")
        .with_execute(execute_fn)
        .with_undo(undo_fn);

    registry.register(cmd_def);

    // 3. Execute the command via the Pipeline
    {
        let mut pipeline = CommandPipeline::new(&registry, &mut graph, &mut event_bus, &mut history);
        let params = json!({ "node_id": node_id.to_string() });
        
        pipeline.execute("CompleteTask", params).unwrap();
    }

    // Verify mutation occurred
    assert_eq!(graph.get_node(node_id).unwrap().get("isCompleted"), Some(&PropertyValue::Bool(true)));
    assert_eq!(history.undo_count(), 1);

    // 4. Undo the command via the HistoryStack
    history.undo(&mut graph, &mut event_bus).unwrap();

    // Verify mutation was reverted
    assert_eq!(graph.get_node(node_id).unwrap().get("isCompleted"), Some(&PropertyValue::Bool(false)));
}