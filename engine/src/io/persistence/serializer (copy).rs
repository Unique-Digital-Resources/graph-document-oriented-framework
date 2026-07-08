//! Graph to JSON serialization.

use serde::Serialize;
//use std::collections::HashMap;

use crate::core::graph::Graph;
use crate::core::node::Node;

#[derive(Serialize)]
struct GraphFile {
    version: u32,
    nodes: Vec<Node>,
}

/// Serializes the graph to a JSON string.
/// In a production 1M node engine, this would stream to a binary format.
pub fn serialize_graph(graph: &Graph) -> Result<String, String> {
    let nodes: Vec<Node> = graph.iter_nodes().cloned().collect();
    let file = GraphFile {
        version: 1,
        nodes,
    };
    serde_json::to_string_pretty(&file).map_err(|e| e.to_string())
}