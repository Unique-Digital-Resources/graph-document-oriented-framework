use std::sync::{Arc, Mutex};

use graph_document_framework::core::graph::Graph;
use graph_document_framework::core::node::{Node, PropertyValue};
use graph_document_framework::core::relation::Lifetime;
use graph_document_framework::core::scheduler::{Priority, Scheduler, Task};
use graph_document_framework::core::signal::Signal;
use graph_document_framework::core::system::{System, SystemRegistry};
use serde_json::json;

// 1. Create a mock System
struct ThumbnailGenerator {
    task_triggered: Arc<Mutex<bool>>,
}

impl System for ThumbnailGenerator {
    fn name(&self) -> &str {
        "ThumbnailGenerator"
    }

    fn filter(&self, signal: &Signal) -> bool {
        signal.signal_type == "NodePropertyChanged" 
            && signal.payload.get("property").and_then(|v| v.as_str()) == Some("image_data")
    }

    fn execute(&self, _graph: &Graph, scheduler: &mut Scheduler, signal: &Signal) {
        let trigger = self.task_triggered.clone();
        let node_id = signal.source.unwrap();

        let task = Task {
            id: uuid::Uuid::new_v4(),
            priority: Priority::Immediate,
            task_type: "generate_thumbnail".to_string(),
            lifetime: Lifetime::Replaceable, // If scheduled again, cancels the old one
            execute: Arc::new(move |_g| {
                let mut t = trigger.lock().unwrap();
                *t = true;
                println!("Generated thumbnail for node {}", node_id);
            }),
        };

        scheduler.schedule(task);
    }
}

#[test]
fn system_reacts_to_signal_and_scheduler_executes() {
    let mut graph = Graph::new();
    let mut scheduler = Scheduler::new();
    let mut registry = SystemRegistry::new();

    let triggered = Arc::new(Mutex::new(false));
    let system = Arc::new(ThumbnailGenerator {
        task_triggered: triggered.clone(),
    });
    registry.register(system);

    // Create an image node
    let node_id = graph.insert_node(
        Node::new("ImageNode")
            .set_persistent("image_data", PropertyValue::string("bytes..."))
    );

    // Simulate a signal arriving from the EventBus
    let signal = Signal::new("NodePropertyChanged", graph_document_framework::core::signal::EmitTiming::Immediate)
        .with_source(node_id)
        .with_payload(json!({"property": "image_data"}));

    // Route signal to systems
    registry.route_signal(&signal, &graph, &mut scheduler);
    
    // Task should be scheduled, but not yet executed (Normal priority)
    assert_eq!(scheduler.pending_count(), 1);
    assert!(!*triggered.lock().unwrap());

    // Simulate a rapid second change. Because it's Replaceable, it should cancel the first task.
    let signal2 = Signal::new("NodePropertyChanged", graph_document_framework::core::signal::EmitTiming::Immediate)
        .with_source(node_id)
        .with_payload(json!({"property": "image_data"}));
    registry.route_signal(&signal2, &graph, &mut scheduler);
    
    // Should still only have 1 task pending because the first was cancelled
    assert_eq!(scheduler.pending_count(), 1);
    assert!(!*triggered.lock().unwrap());

    // Force a tick. Since priority is Immediate, tick() will execute it.
    scheduler.tick(&graph);

    assert!(*triggered.lock().unwrap());
}