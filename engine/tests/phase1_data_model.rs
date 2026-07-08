use graph_document_framework::core::graph::{Graph, GraphError};
use graph_document_framework::core::node::{Node, TypeId};
use graph_document_framework::core::relation::RelationPresetRegistry;

//use graph_document_framework::core::relation::{
//    Ownership, RelationPresetRegistry, Topology,
//};

// use graph_document_framework::core::relation::{
//    presets, RelationPresetRegistry, RelationSchema, Topology, Cardinality, Ownership, Propagation, Evaluation, Lifetime, Persistence
//};

#[test]
fn graph_engine_enforces_tree_and_allows_dag() {
    let mut graph = Graph::new();
    let registry = RelationPresetRegistry::with_core_presets();

    let children_schema = registry.get("CHILDREN").unwrap();
    let dep_schema = registry.get("DEPENDENCY").unwrap();

    // Create nodes
    let root = graph.insert_node(Node::new("TaskNode"));
    let child1 = graph.insert_node(Node::new("TaskNode"));
    let child2 = graph.insert_node(Node::new("TaskNode"));
    let grandchild = graph.insert_node(Node::new("TaskNode"));

    // 1. Test CHILDREN (Tree Topology)
    // Root -> Child1 -> Grandchild (Valid)
    assert!(graph.add_edge(children_schema, root, child1).is_ok());
    assert!(graph.add_edge(children_schema, child1, grandchild).is_ok());

    // Attempt to create a cycle: Grandchild -> Root (Should fail)
    let err = graph.add_edge(children_schema, grandchild, root).unwrap_err();
    assert!(matches!(err, GraphError::ValidationError(_)));

    // Attempt to give Grandchild a second parent: Root -> Grandchild (Should fail cardinality/tree check)
    let err = graph.add_edge(children_schema, root, grandchild).unwrap_err();
    assert!(matches!(err, GraphError::ValidationError(_)));

    // 2. Test DEPENDENCY (DAG Topology)
    // child2 depends on child1 (Valid)
    assert!(graph.add_edge(dep_schema, child2, child1).is_ok());
    
    // child1 depends on child2 (Valid individually, but creates a cycle, should fail)
    let err = graph.add_edge(dep_schema, child1, child2).unwrap_err();
    assert!(matches!(err, GraphError::ValidationError(_)));

    // 3. Test Queries
    use graph_document_framework::core::graph::queries::GraphQuery;
    let query = GraphQuery::new(&graph);

    let children_of_root = query.children(root);
    assert_eq!(children_of_root.len(), 1);
    assert_eq!(children_of_root[0], child1);

    let ancestors = query.find_ancestors(grandchild, "CHILDREN");
    assert!(ancestors.contains(&child1));
    assert!(ancestors.contains(&root));

    let task_type = TypeId::new("TaskNode");
    let all_tasks = query.find_by_type(&task_type);
    assert_eq!(all_tasks.len(), 4);
}

#[test]
fn topological_sort_works_on_dag() {
    use graph_document_framework::core::graph::traversal::topological_sort;
    
    let mut graph = Graph::new();
    let registry = RelationPresetRegistry::with_core_presets();
    let dep_schema = registry.get("DEPENDENCY").unwrap();

    // A -> B -> C
    let a = graph.insert_node(Node::new("Node"));
    let b = graph.insert_node(Node::new("Node"));
    let c = graph.insert_node(Node::new("Node"));

    graph.add_edge(dep_schema, a, b).unwrap();
    graph.add_edge(dep_schema, b, c).unwrap();

    let sorted = topological_sort(&graph, "DEPENDENCY").unwrap();
    
    // A must come before B, B before C
    let pos_a = sorted.iter().position(|&x| x == a).unwrap();
    let pos_b = sorted.iter().position(|&x| x == b).unwrap();
    let pos_c = sorted.iter().position(|&x| x == c).unwrap();

    assert!(pos_a < pos_b);
    assert!(pos_b < pos_c);
}