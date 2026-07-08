//! Headless UI state: selection and focus.
//!
//! Both subsystems are *stateful* but not *mutators*. They hold transient
//! state, expose read APIs for renderers, and dispatch Commands when a
//! state change should have side effects on the document graph.

pub mod selection;
pub mod focus;

pub use selection::SelectionState;
pub use focus::FocusState;