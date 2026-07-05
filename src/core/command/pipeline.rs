//! Command Execution Pipeline.
//!
//! Enforces the strict unidirectional flow:
//! `Permissions -> Transaction -> Execute -> Signals -> History`

use serde_json::Value;

use crate::core::graph::{Graph, GraphError};
use crate::core::history::{HistoryStack, Transaction};
use crate::core::signal::{EmitTiming, EventBus, Signal};

use super::registry::CommandRegistry;

pub struct CommandPipeline<'a> {
    registry: &'a CommandRegistry,
    graph: &'a mut Graph,
    event_bus: &'a mut EventBus,
    history: &'a mut HistoryStack,
    active_transaction: Option<Transaction>,
}

impl<'a> CommandPipeline<'a> {
    pub fn new(
        registry: &'a CommandRegistry,
        graph: &'a mut Graph,
        event_bus: &'a mut EventBus,
        history: &'a mut HistoryStack,
    ) -> Self {
        Self {
            registry,
            graph,
            event_bus,
            history,
            active_transaction: None,
        }
    }

    /// Begin an explicit transaction. All subsequent commands will be batched.
    pub fn begin_transaction(&mut self, label: impl Into<String>) {
        if self.active_transaction.is_none() {
            self.active_transaction = Some(Transaction::new(label));
        }
    }

    /// Commit the explicit transaction, pushing it to the history stack.
    pub fn commit_transaction(&mut self) -> Result<(), PipelineError> {
        if let Some(tx) = self.active_transaction.take() {
            let entry = tx.commit();
            self.history.push(entry);
            self.event_bus.flush_transactional();
        }
        Ok(())
    }

    /// Execute a command by ID with the given JSON parameters.
    pub fn execute(&mut self, command_id: &str, params: Value) -> Result<(), PipelineError> {
        // 1. Lookup Command
        let definition = self
            .registry
            .get(command_id)
            .ok_or_else(|| PipelineError::CommandNotFound(command_id.into()))?
            .clone(); // Clone the definition (Arc clones)

        let is_implicit_tx = self.active_transaction.is_none();

        // 2. Start implicit transaction if none is active
        if is_implicit_tx {
            self.active_transaction = Some(Transaction::new(definition.label.clone()));
        }

        let tx = self.active_transaction.as_mut().unwrap();

        // 3. Execute
        let result = (definition.execute)(self.graph, self.event_bus, params.clone())
            .map_err(PipelineError::ExecutionError);

        match result {
            Ok(_) => {
                // 4. Add undo step to transaction
                tx.add_step(definition.undo.clone(), params.clone());

                // 5. Emit Transactional Signal
                let signal = Signal::new("CommandExecuted", EmitTiming::Transactional)
                    .with_payload(serde_json::json!({
                        "command_id": command_id,
                        "params": params
                    }));
                self.event_bus.emit(signal);

                // 6. If implicit transaction, commit immediately
                if is_implicit_tx {
                    self.commit_transaction()?;
                }
                Ok(())
            }
            Err(e) => {
                // Rollback: restore snapshot and discard transaction
                if let Some(tx) = self.active_transaction.take() {
                    tx.snapshot.restore(self.graph);
                }
                Err(e)
            }
        }
    }
}

#[derive(Debug)]
pub enum PipelineError {
    CommandNotFound(String),
    ExecutionError(String),
    GraphError(GraphError),
}

impl From<GraphError> for PipelineError {
    fn from(e: GraphError) -> Self {
        PipelineError::GraphError(e)
    }
}