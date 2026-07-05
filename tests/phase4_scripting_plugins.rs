use graph_document_framework::core::command::CommandRegistry;
use graph_document_framework::core::graph::Graph;
use graph_document_framework::core::scheduler::Scheduler;
use graph_document_framework::core::signal::EventBus;
use graph_document_framework::plugin::api::PluginContext;
use graph_document_framework::plugin::manager::{resolve_plugin_order, PluginManifest};
use graph_document_framework::plugin::permissions::Sandbox;
//use graph_document_framework::plugin::permissions::{policy::Permission, Sandbox};

#[test]
fn plugin_dependency_graph_resolves_and_detects_cycles() {
    let manifests = vec![
        PluginManifest {
            id: "app".to_string(),
            dependencies: vec!["core_ui".to_string(), "database".to_string()],
            ..Default::default()
        },
        PluginManifest {
            id: "core_ui".to_string(),
            dependencies: vec!["database".to_string()],
            ..Default::default()
        },
        PluginManifest {
            id: "database".to_string(),
            dependencies: vec![],
            ..Default::default()
        },
    ];

    let order = resolve_plugin_order(&manifests).unwrap();
    
    // database must come before core_ui and app
    let db_idx = order.iter().position(|x| x == "database").unwrap();
    let ui_idx = order.iter().position(|x| x == "core_ui").unwrap();
    let app_idx = order.iter().position(|x| x == "app").unwrap();
    
    assert!(db_idx < ui_idx);
    assert!(ui_idx < app_idx);

    // Test cycle detection
    let cyclic_manifests = vec![
        PluginManifest {
            id: "a".to_string(),
            dependencies: vec!["b".to_string()],
            ..Default::default()
        },
        PluginManifest {
            id: "b".to_string(),
            dependencies: vec!["a".to_string()],
            ..Default::default()
        },
    ];

    let result = resolve_plugin_order(&cyclic_manifests);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cyclic dependency"));
}

#[test]
fn sandbox_enforces_permissions_on_context() {
    let graph = Graph::new();
    let mut sandbox = Sandbox::new();
    let mut event_bus = EventBus::new();
    let mut scheduler = Scheduler::new();
    let registry = CommandRegistry::new();

    // Register a plugin WITH filesystem permissions
    sandbox.register_plugin("fs_plugin", &["filesystem".to_string()]);
    
    // Register a plugin WITHOUT filesystem permissions
    sandbox.register_plugin("no_fs_plugin", &[]);

    // Test permission allowed
    let ctx_allowed = PluginContext::new(
        "fs_plugin", &graph, &sandbox, &registry, &mut event_bus, &mut scheduler
    );
    assert!(ctx_allowed.try_read_file().is_ok());

    // Test permission denied
    let ctx_denied = PluginContext::new(
        "no_fs_plugin", &graph, &sandbox, &registry, &mut event_bus, &mut scheduler
    );
    let result = ctx_denied.try_read_file();
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.0.contains("lacks required permission: Filesystem"));
}