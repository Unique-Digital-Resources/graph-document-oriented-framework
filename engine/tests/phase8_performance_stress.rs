use graph_document_framework::core::graph::Graph;
use graph_document_framework::core::node::Node;
use graph_document_framework::core::node::properties::PropertyValue;
use graph_document_framework::core::relation::presets::children;
use graph_document_framework::io::persistence::{serialize_graph, serialize_graph_binary, deserialize_graph_binary};
use std::time::Instant;

const NODE_COUNT: usize = 100_000;

#[test]
fn stress_test_graph_operations_optimized() {
    // 1. Pre-allocate memory for the graph
    let mut graph = Graph::with_capacity(NODE_COUNT);
    let mut node_ids = Vec::with_capacity(NODE_COUNT);

    println!("\n--- Optimized Performance Stress Test ({} nodes) ---", NODE_COUNT);

    // 2. Benchmark Node Insertion
    let now = Instant::now();
    for _ in 0..NODE_COUNT {
        let node = Node::new("TaskNode")
            .set_persistent("title", PropertyValue::String("Buy milk".into()))
            .set_persistent("completed", PropertyValue::Bool(false));
        let id = graph.insert_node(node);
        node_ids.push(id);
    }
    println!("1. Node Insertion:      {:?}", now.elapsed());
    assert_eq!(graph.node_count(), NODE_COUNT);

    // 3. Benchmark Edge Insertion (Unchecked for bulk loading)
    let now = Instant::now();
    let child_schema = children();
    for i in 1..NODE_COUNT {
        let parent_idx = (i - 1) / 2;
        let _ = graph.add_edge_unchecked(&child_schema, node_ids[parent_idx], node_ids[i]);
    }
    println!("2. Edge Insertion:      {:?} (Unchecked)", now.elapsed());

    // 4. Benchmark Graph Traversal (DFS)
    let now = Instant::now();
    let mut visited_count = 0;
    let mut stack = vec![node_ids[0]];
    while let Some(curr_id) = stack.pop() {
        visited_count += 1;
        stack.extend(graph.get_targets(curr_id, "CHILDREN"));
    }
    println!("3. Tree Traversal:      {:?}", now.elapsed());
    assert_eq!(visited_count, NODE_COUNT);

    // 5. Benchmark Serialization (JSON vs Binary)
    let now = Instant::now();
    let json = serialize_graph(&graph).expect("JSON failed");
    let json_mb = json.len() as f64 / 1_048_576.0;
    println!("4. JSON Serialization:  {:?} (Size: {:.2} MB)", now.elapsed(), json_mb);

    let now = Instant::now();
    let bin = serialize_graph_binary(&graph).expect("Binary failed");
    let bin_mb = bin.len() as f64 / 1_048_576.0;
    println!("5. Bin Serialization:   {:?} (Size: {:.2} MB)", now.elapsed(), bin_mb);

    // 6. Benchmark Binary Deserialization
    let now = Instant::now();
    let _decoded_graph = deserialize_graph_binary(&bin).expect("Binary decode failed");
    println!("6. Bin Deserialization: {:?}", now.elapsed());

    // The binary format should be significantly smaller and faster
    assert!(bin.len() < json.len());
    
    println!("------------------------------------------------------\n");
}