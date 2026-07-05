//! Phase 4 — Plugin System.

pub mod api;
pub mod manager;
pub mod permissions;

pub use manager::{Plugin, PluginManager, PluginManifest, PluginState};