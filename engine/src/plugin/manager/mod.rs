//! Plugin lifecycle and dependency management.

pub mod dependencies;
pub mod lifecycle;

pub use dependencies::resolve_plugin_order;
pub use lifecycle::{Plugin, PluginManager, PluginManifest, PluginState};