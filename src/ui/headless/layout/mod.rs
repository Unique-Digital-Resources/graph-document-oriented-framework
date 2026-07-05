//! Headless layout system.
//!
//! Layout is a `System`: it listens for graph signals (UI tree changes,
//! bound document node changes) and recomputes `Bounds` on the affected
//! UI nodes. It produces no pixels — only geometry.

pub mod system;

pub use system::LayoutSystem;