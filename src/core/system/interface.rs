//! The System trait definition.

use crate::core::graph::Graph;
use crate::core::scheduler::Scheduler;
use crate::core::signal::Signal;

/// A long-running background service.
pub trait System: Send + Sync {
    /// Name of the system (e.g., "ThumbnailGenerator")
    fn name(&self) -> &str;

    /// Returns true if this system should react to the given signal.
    fn filter(&self, signal: &Signal) -> bool;

    /// The logic to run when filtered. Typically schedules tasks on the Scheduler.
    fn execute(&self, graph: &Graph, scheduler: &mut Scheduler, signal: &Signal);
}