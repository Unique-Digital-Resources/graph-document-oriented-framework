pub mod document;
pub mod commands;
pub mod ui;

use engine::core::graph::storage::Graph;
use engine::core::command::registry::{CommandRegistry, CommandDefinition};
use engine::core::command::pipeline::CommandPipeline;
use engine::core::signal::event_bus::EventBus;
use engine::core::history::stack::HistoryStack;
use std::sync::Arc;

pub struct WgpuApp {
    pub graph: Graph,
    pub registry: CommandRegistry,
    pub event_bus: EventBus,
    pub history: HistoryStack,
}

impl WgpuApp {
    pub fn new() -> Self {
        let graph = document::initialize_document();
        let mut registry = CommandRegistry::new();
        let event_bus = EventBus::new();
        let history = HistoryStack::new();

        let commit_cam_cmd = CommandDefinition::new(
            "CommitViewportCamera",
            "Commit Viewport Camera",
            "Commits the viewport camera state to the graph for undo/save."
        )
        .with_execute(Arc::new(commands::commit_viewport_camera));
        
        registry.register(commit_cam_cmd);

        Self {
            graph,
            registry,
            event_bus,
            history,
        }
    }

    pub fn create_pipeline(&mut self) -> CommandPipeline<'_> {
        CommandPipeline::new(
            &self.registry,
            &mut self.graph,
            &mut self.event_bus,
            &mut self.history,
        )
    }
}