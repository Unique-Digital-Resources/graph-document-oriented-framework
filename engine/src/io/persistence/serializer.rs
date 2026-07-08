//! Graph serialization (JSON and Binary/MessagePack).

use serde::Serialize;
use crate::core::graph::Graph;
use crate::core::node::Node;

#[derive(Serialize)]
struct GraphFile {
    version: u32,
    nodes: Vec<Node>,
}

/// Serializes the graph to a JSON string.
pub fn serialize_graph(graph: &Graph) -> Result<String, String> {
    let nodes: Vec<Node> = graph.iter_nodes().cloned().collect();
    let file = GraphFile {
        version: 1,
        nodes,
    };
    serde_json::to_string_pretty(&file).map_err(|e| e.to_string())
}

/// Serializes the graph to a compact binary format (MessagePack).
/// Significantly faster and smaller than JSON.
pub fn serialize_graph_binary(graph: &Graph) -> Result<Vec<u8>, String> {
    let nodes: Vec<Node> = graph.iter_nodes().cloned().collect();
    let file = GraphFile {
        version: 1,
        nodes,
    };
    
    let mut buf = Vec::new();
    file.serialize(&mut rmp_serde::Serializer::new(&mut buf))
        .map_err(|e| e.to_string())?;
    Ok(buf)
}