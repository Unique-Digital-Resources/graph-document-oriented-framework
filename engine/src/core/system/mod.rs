//! Pillar 4 (Sub-system) — Systems.
//!
//! Long-running background services that observe the graph via Signals
//! and schedule work via the Scheduler.

pub mod interface;
pub mod registry;

pub use interface::System;
pub use registry::SystemRegistry;