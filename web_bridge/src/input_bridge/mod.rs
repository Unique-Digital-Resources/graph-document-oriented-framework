//! Catches user inputs (mouse clicks, keystrokes) from the frontend
//! and translates them into Framework Commands.

pub mod listeners;
pub mod dispatcher;

pub use listeners::{DomEvent, EventListener};
pub use dispatcher::InputDispatcher;