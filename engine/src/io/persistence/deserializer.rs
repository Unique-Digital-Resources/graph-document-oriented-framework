//! Graph deserialization (JSON and Binary/MessagePack).

use serde::Deserialize;
use crate::core::graph::Graph;
use crate::core::node::Node;

#[derive(Deserialize)]
pub struct GraphFile {
    #[allow(dead_code)]
    version: u32,
    nodes: Vec<Node>,
}

/// Deserializes a JSON string into a Graph.
pub fn deserialize_graph(data: &str) -> Result<Graph, String> {
    let file: GraphFile = serde_json::from_str(data).map_err(|e| e.to_string())?;
    
    // Pre-allocate memory for the nodes
    let mut graph = Graph::with_capacity(file.nodes.len());
    for node in file.nodes {
        graph.insert_node(node);
    }
    Ok(graph)
}

/// Deserializes a MessagePack binary buffer into a Graph.
pub fn deserialize_graph_binary(data: &[u8]) -> Result<Graph, String> {
    let file: GraphFile = rmp_serde::from_slice(data).map_err(|e| e.to_string())?;
    
    // Pre-allocate memory for the nodes
    let mut graph = Graph::with_capacity(file.nodes.len());
    for node in file.nodes {
        graph.insert_node(node);
    }
    Ok(graph)
}