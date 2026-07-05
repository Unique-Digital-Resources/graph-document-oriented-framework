//! Command Execution Pipeline.
//!
//! Enforces the strict unidirectional flow:
//! `Permissions -> Transaction -> Execute -> Signals -> History`
//!
//! Note: `Transaction` and `History` are stubbed here as simple success paths.
//! They will get their own dedicated state management in files 15-18.

use serde_json::Value;

use crate::core::graph::{Graph, GraphError};
use crate::core::signal::{EmitTiming, EventBus, Signal};

use super::registry::CommandRegistry;

pub struct CommandPipeline<'a> {
    registry: &'a CommandRegistry,
    graph: &'a mut Graph,
    event_bus: &'a mut EventBus,
}

impl<'a> CommandPipeline<'a> {
    pub fn new(
        registry: &'a CommandRegistry,
        graph: &'a mut Graph,
        event_bus: &'a mut EventBus,
    ) -> Self {
        Self {
            registry,
            graph,
            event_bus,
        }
    }

    /// Execute a command by ID with the given JSON parameters.
    pub fn execute(&mut self, command_id: &str, params: Value) -> Result<(), PipelineError> {
        // 1. Lookup Command
        let definition = self
            .registry
            .get(command_id)
            .ok_or_else(|| PipelineError::CommandNotFound(command_id.into()))?;

        // We clone the Arc to execute outside the immutable borrow of registry
        let execute_fn = definition.execute.clone();

        // 2. Permissions Check (Mocked for now)
        // self.check_permissions(definition)?;

        // 3. Transaction Begin (Mocked for now)
        // let tx = self.begin_transaction()?;

        // 4. Execute
        // If execute fails, we would roll back the transaction here.
        (execute_fn)(self.graph, self.event_bus, params.clone()).map_err(PipelineError::ExecutionError)?;

        // 5. Emit Transactional Signal
        let signal = Signal::new("CommandExecuted", EmitTiming::Transactional)
            .with_payload(serde_json::json!({
                "command_id": command_id,
                "params": params
            }));
        self.event_bus.emit(signal);

        // 6. Commit Transaction & Flush Signals (Mocked)
        // self.commit_transaction(tx)?;
        self.event_bus.flush_transactional();

        // 7. Push to History Stack (Mocked for now)
        // self.history.push(definition.undo.clone(), params);

        Ok(())
    }
}

#[derive(Debug)]
pub enum PipelineError {
    CommandNotFound(String),
    PermissionDenied(String),
    ExecutionError(String),
    GraphError(GraphError),
}

impl From<GraphError> for PipelineError {
    fn from(e: GraphError) -> Self {
        PipelineError::GraphError(e)
    }
}