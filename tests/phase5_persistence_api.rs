use graph_document_framework::core::command::{CommandDefinition, CommandRegistry};
use graph_document_framework::core::graph::Graph;
use graph_document_framework::core::history::HistoryStack;
use graph_document_framework::core::node::{Node, PropertyValue};
use graph_document_framework::core::signal::EventBus;
use graph_document_framework::io::persistence::{deserialize_graph, serialize_graph, Migrator};
//use graph_document_framework::io::persistence::{deserialize_graph, migrate_graph, serialize_graph};
use graph_document_framework::io::rpc::server::RpcServer;
use serde_json::json;
use std::sync::Arc;

#[test]
fn persistence_and_rpc_integration() {
    let mut graph = Graph::new();
    
    // 1. Create and serialize a graph
    let node_id = graph.insert_node(
        Node::new("TaskNode")
            .set_persistent("title", PropertyValue::string("My Task"))
    );

    let json = serialize_graph(&graph).unwrap();
    assert!(json.contains("My Task"));

    // 2. Deserialize into a new graph
    let mut new_graph = deserialize_graph(&json).unwrap();
    assert_eq!(new_graph.node_count(), 1);
    assert_eq!(new_graph.get_node(node_id).unwrap().get("title"), Some(&PropertyValue::string("My Task")));

    // 3. Run a migration (v1 to v2 adds "archived" property)
    Migrator::migrate(&mut new_graph, 1, 2).unwrap();
    let migrated_node = new_graph.get_node(node_id).unwrap();
    assert!(migrated_node.properties.contains("archived"));
    assert_eq!(migrated_node.get("archived"), Some(&PropertyValue::Bool(false)));

    // 4. Setup RPC Server
    let mut event_bus = EventBus::new();
    let mut history = HistoryStack::new();
    let mut registry = CommandRegistry::new();

    // Register a command for the API to call
    let exec_fn = Arc::new(|_g: &mut Graph, _e: &mut EventBus, _p: serde_json::Value| Ok(()));
    registry.register(
        CommandDefinition::new("DeleteTask", "Delete", "Deletes task")
            .with_execute(exec_fn)
    );

    let mut server = RpcServer::new(&mut new_graph, &mut event_bus, &mut history, &mut registry);

    // 5. Test Query API
    let query_req = json!({"type": "find_by_type", "type_id": "TaskNode"}).to_string();
    let query_resp = server.handle_query_http(&query_req).unwrap();
    assert!(query_resp.contains(&node_id.to_string()));

    // 6. Test Command API
    let cmd_req = json!({"command_id": "DeleteTask", "params": {"id": node_id.to_string()}}).to_string();
    let cmd_resp = server.handle_command_ws(&cmd_req).unwrap();
    assert!(cmd_resp.contains("\"success\":true"));
}