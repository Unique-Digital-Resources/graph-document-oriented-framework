//! Pillar 4 (Sub-system) — Scheduler.
//!
//! Controls when and how background work executes. Manages priorities,
//! dirty propagation, and task cancellation.

pub mod cancellation;
pub mod dirty_propagation;
pub mod queue;

pub use cancellation::CancellationManager;
pub use dirty_propagation::propagate_dirty;
pub use queue::{Priority, Scheduler, Task, TaskFn};