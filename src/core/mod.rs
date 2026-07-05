//! The headless engine core. Has zero knowledge of `ui/`, `io/`, or `plugin/`.

pub mod command;
pub mod graph;
pub mod history;
pub mod node;
pub mod relation;
pub mod scheduler;
pub mod signal;
pub mod system;