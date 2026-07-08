//! The `ctx` object passed to plugins.

use crate::core::command::CommandRegistry;
use crate::core::graph::Graph;
use crate::core::scheduler::Scheduler;
use crate::core::signal::EventBus;

// FIX: Point to the correct module path
use crate::plugin::permissions::sandbox::Sandbox;
use crate::plugin::permissions::policy::{Permission, PermissionDenied};

/// A restricted context provided to plugins during initialization and execution.
/// It prevents direct mutation of the graph and enforces the sandbox.
pub struct PluginContext<'a> {
    pub plugin_id: String,
    pub graph: &'a Graph,
    pub sandbox: &'a Sandbox,
    pub command_registry: &'a CommandRegistry,
    // EventBus and Scheduler are mutable because plugins might emit signals 
    // or schedule tasks directly during initialization.
    pub event_bus: &'a mut EventBus,
    pub scheduler: &'a mut Scheduler,
}

impl<'a> PluginContext<'a> {
    pub fn new(
        plugin_id: &str,
        graph: &'a Graph,
        sandbox: &'a Sandbox,
        command_registry: &'a CommandRegistry,
        event_bus: &'a mut EventBus,
        scheduler: &'a mut Scheduler,
    ) -> Self {
        Self {
            plugin_id: plugin_id.to_string(),
            graph,
            sandbox,
            command_registry,
            event_bus,
            scheduler,
        }
    }

    /// Helper to verify permissions before performing an action.
    pub fn require_permission(&self, perm: Permission) -> Result<(), PermissionDenied> {
        self.sandbox.check(&self.plugin_id, perm)
    }

    // Example of a sandboxed API method
    pub fn try_read_file(&self) -> Result<String, PermissionDenied> {
        self.require_permission(Permission::Filesystem)?;
        // Real implementation would interact with OS via io/persistence
        Ok("File contents".to_string())
    }
}