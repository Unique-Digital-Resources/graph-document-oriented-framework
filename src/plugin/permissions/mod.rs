//! Plugin permission system.

pub mod policy;
pub mod sandbox;

pub use policy::{Permission, PermissionDenied};
pub use sandbox::Sandbox;