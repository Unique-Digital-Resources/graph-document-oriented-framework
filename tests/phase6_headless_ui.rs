use graph_document_framework::core::graph::Graph;
use graph_document_framework::core::node::Node;
use graph_document_framework::core::scheduler::Scheduler;
use graph_document_framework::core::signal::EventBus;
use graph_document_framework::core::signal::types::{EmitTiming, Signal};
use graph_document_framework::system::System;

use graph_document_framework::ui::headless::nodes::widgets::{ButtonNode, ContainerLayout, ContainerNode, WidgetKind};
use graph_document_framework::ui::headless::view_graph::bindings::{Binding, BindingKind, BindingRegistry};
use graph_document_framework::ui::headless::view_graph::storage::ViewGraph;
use graph_document_framework::ui::headless::layout::system::LayoutSystem;
use graph_document_framework::ui::headless::state::selection::SelectionState;
use graph_document_framework::ui::headless::state::focus::FocusState;

use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[test]
fn view_graph_construction_and_bindings() {
    let mut view = ViewGraph::new();
    
    // 1. Create a mock Document Node
    let doc_node = Node::new("TaskNode");
    let doc_id = doc_node.id;

    // 2. Create UI Widgets
    let root_widget = ContainerNode::new(ContainerLayout::Row);
    let root_id = view.insert(WidgetKind::Container(root_widget));
    
    let button_widget = ButtonNode::new("Complete Task");
    let button_id = view.insert(WidgetKind::Button(button_widget));
    
    // 3. Attach and set root
    view.set_root(root_id).unwrap();
    view.attach(root_id, button_id).unwrap();

    // Verify tree structure
    assert_eq!(view.parent(button_id), Some(root_id));
    assert!(view.children(root_id).contains(&button_id));

    // 4. Test Bindings
    let mut bindings = BindingRegistry::new();
    bindings.bind(
        Binding { ui_node: button_id, document_node: doc_id, property: None },
        BindingKind::Action,
    );
    
    // UI references Document, Document does not reference UI
    assert_eq!(bindings.ui_for_document(doc_id), &[button_id]);
    assert!(bindings.get(button_id).is_some());
}

#[test]
fn layout_system_calculates_bounds() {
    let mut view = ViewGraph::new();
    
    // Setup a Row container with two buttons
    let root_widget = ContainerNode::new(ContainerLayout::Row);
    let root_id = view.insert(WidgetKind::Container(root_widget));
    view.set_root(root_id).unwrap();
    
    let btn1_id = view.insert(WidgetKind::Button(ButtonNode::new("Btn 1")));
    let btn2_id = view.insert(WidgetKind::Button(ButtonNode::new("Btn 2")));
    
    view.attach(root_id, btn1_id).unwrap();
    view.attach(root_id, btn2_id).unwrap();

    // Wrap view graph in Arc<Mutex> for the System trait
    let view_arc = Arc::new(Mutex::new(view));
    let mut layout_system = LayoutSystem::new(view_arc.clone());
    
    // Set viewport to 1000x800
    layout_system.set_viewport(1000.0, 800.0);

    // Execute Layout System manually (simulating scheduler tick)
    let graph = Graph::new();
    let mut scheduler = Scheduler::new();
    let signal = Signal::new("UiNodeAdded", EmitTiming::Immediate);
    layout_system.execute(&graph, &mut scheduler, &signal);

    let view = view_arc.lock().unwrap();
    let root_bounds = view.get(root_id).unwrap().ui_node().bounds;
    let b1_bounds = view.get(btn1_id).unwrap().ui_node().bounds;
    let b2_bounds = view.get(btn2_id).unwrap().ui_node().bounds;

    // Root should take full viewport
    assert_eq!(root_bounds.width, 1000.0);
    assert_eq!(root_bounds.height, 800.0);

    // Row layout splits width evenly among 2 children (500.0 each)
    assert_eq!(b1_bounds.width, 500.0);
    assert_eq!(b2_bounds.width, 500.0);
    
    // Graph traversal order is non-deterministic (HashSet), so we just check
    // that one button starts at 0.0 and the other at 500.0
    assert_ne!(b1_bounds.x, b2_bounds.x);
    assert!(b1_bounds.x == 0.0 || b1_bounds.x == 500.0);
    assert!(b2_bounds.x == 0.0 || b2_bounds.x == 500.0);
}

#[test]
fn selection_and_focus_state() {
    let mut bus = EventBus::new();
    let mut selection = SelectionState::new();
    
    let doc_id1 = Uuid::new_v4();
    let doc_id2 = Uuid::new_v4();

    // Test Selection
    selection.select(&mut bus, doc_id1);
    assert!(selection.is_selected(doc_id1));
    assert!(!selection.is_selected(doc_id2));
    assert_eq!(selection.primary(), Some(doc_id1));

    selection.toggle(&mut bus, doc_id2);
    assert!(selection.is_selected(doc_id1));
    assert!(selection.is_selected(doc_id2));
    assert_eq!(selection.primary(), Some(doc_id2)); // toggled changes primary

    selection.clear(&mut bus);
    assert!(selection.is_empty());

    // Test Focus
    let mut view = ViewGraph::new();
    let btn_widget = ButtonNode::new("Focus Me");
    let btn_id = view.insert(WidgetKind::Button(btn_widget));
    view.set_root(btn_id).unwrap(); // Root is focusable in this test

    let mut focus = FocusState::new();
    assert!(focus.focused().is_none());

    focus.focus(&mut bus, &view, btn_id);
    assert_eq!(focus.focused(), Some(btn_id));

    focus.blur(&mut bus);
    assert!(focus.focused().is_none());
}