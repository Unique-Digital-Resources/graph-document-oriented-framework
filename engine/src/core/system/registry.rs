//! Manages active background systems.

use std::sync::Arc;

use crate::core::graph::Graph;
use crate::core::scheduler::Scheduler;
use crate::core::signal::Signal;

use super::interface::System;

pub struct SystemRegistry {
    systems: Vec<Arc<dyn System>>,
}

impl Default for SystemRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemRegistry {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn register(&mut self, system: Arc<dyn System>) {
        self.systems.push(system);
    }

    /// Routes a signal to all interested systems.
    pub fn route_signal(&self, signal: &Signal, graph: &Graph, scheduler: &mut Scheduler) {
        for system in &self.systems {
            if system.filter(signal) {
                system.execute(graph, scheduler, signal);
            }
        }
    }

    pub fn count(&self) -> usize {
        self.systems.len()
    }
}