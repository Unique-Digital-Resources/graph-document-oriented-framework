//! JSON to Graph deserialization.

use serde::Deserialize;
use crate::core::graph::Graph;
use crate::core::node::Node;

#[derive(Deserialize)]
struct GraphFile {
		#[allow(dead_code)]
    version: u32,
    nodes: Vec<Node>,
}

/// Deserializes a JSON string into a Graph.
pub fn deserialize_graph(data: &str) -> Result<Graph, String> {
    let file: GraphFile = serde_json::from_str(data).map_err(|e| e.to_string())?;
    
    let mut graph = Graph::new();
    for node in file.nodes {
        graph.insert_node(node);
    }
    Ok(graph)
}