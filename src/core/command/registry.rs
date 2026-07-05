//! Command Registry — holds definitions and allows lookup by ID.

use std::collections::HashMap;
use std::sync::Arc;

use crate::core::graph::Graph;
use crate::core::signal::EventBus;
use serde_json::Value;

/// Function signature for command execution.
/// Takes mutable access to Graph and EventBus, plus parameters.
pub type ExecuteFn = Arc<dyn Fn(&mut Graph, &mut EventBus, Value) -> Result<(), String> + Send + Sync>;

/// Function signature for command undo logic.
pub type UndoFn = Arc<dyn Fn(&mut Graph, &mut EventBus, Value) -> Result<(), String> + Send + Sync>;

/// Definition of a command, registered by the core or plugins.
#[derive(Clone)]
pub struct CommandDefinition {
    pub id: String,
    pub label: String,
    pub description: String,
    pub execute: ExecuteFn,
    pub undo: UndoFn,
}

impl CommandDefinition {
    pub fn new(id: impl Into<String>, label: impl Into<String>, desc: impl Into<String>) -> Self {
        // Default no-op execute/undo that just returns an error if called without being set
        let err_fn: ExecuteFn = Arc::new(|_, _, _| Err("Execute function not implemented".into()));
        let err_fn_undo: UndoFn = Arc::new(|_, _, _| Err("Undo function not implemented".into()));
        
        Self {
            id: id.into(),
            label: label.into(),
            description: desc.into(),
            execute: err_fn,
            undo: err_fn_undo,
        }
    }

    pub fn with_execute(mut self, f: ExecuteFn) -> Self {
        self.execute = f;
        self
    }

    pub fn with_undo(mut self, f: UndoFn) -> Self {
        self.undo = f;
        self
    }
}

#[derive(Default)]
pub struct CommandRegistry {
    commands: HashMap<String, CommandDefinition>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, definition: CommandDefinition) {
        self.commands.insert(definition.id.clone(), definition);
    }

    pub fn get(&self, id: &str) -> Option<&CommandDefinition> {
        self.commands.get(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.commands.contains_key(id)
    }
}