//! Pillar 2 — Signals.
//!
//! Signals are notifications ("Something happened"). They do not perform
//! actions. They are emitted *after* a Command successfully mutates the graph.

pub mod event_bus;
pub mod types;

pub use event_bus::EventBus;
pub use types::{EmitTiming, Signal};