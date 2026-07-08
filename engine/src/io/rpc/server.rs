//! WebSocket/HTTP server setup.

use crate::core::command::CommandRegistry;
use crate::core::graph::Graph;
use crate::core::history::HistoryStack;
use crate::core::signal::EventBus;

use super::command_router;
use super::query_router;

pub struct RpcServer<'a> {
    graph: &'a mut Graph,
    event_bus: &'a mut EventBus,
    history: &'a mut HistoryStack,
    registry: &'a CommandRegistry,
}

impl<'a> RpcServer<'a> {
    pub fn new(
        graph: &'a mut Graph,
        event_bus: &'a mut EventBus,
        history: &'a mut HistoryStack,
        registry: &'a CommandRegistry,
    ) -> Self {
        Self { graph, event_bus, history, registry }
    }

    /// Mock endpoint for receiving an external query.
    pub fn handle_query_http(&self, request_json: &str) -> Result<String, String> {
        let query = crate::core::graph::queries::GraphQuery::new(self.graph);
        query_router::handle_query_request(&query, request_json)
    }

    /// Mock endpoint for receiving an external command.
    pub fn handle_command_ws(&mut self, request_json: &str) -> Result<String, String> {
        let mut pipeline = crate::core::command::CommandPipeline::new(
            self.registry,
            self.graph,
            self.event_bus,
            self.history,
        );
        command_router::handle_command_request(&mut pipeline, request_json)
    }
}