//! Phase 7: Web UI Components.
//!
//! This layer acts as the "dumb view". It reads the Headless View Graph
//! and generates rendering instructions (DOM or Canvas) and routes input
//! events back to the Command Pipeline.

pub mod dom_mapper;
pub mod js_bridge;
pub mod canvas_renderer;
pub mod input_bridge;

pub use dom_mapper::{DomMapper, DomNode};
pub use canvas_renderer::{draw_calls::DrawCall, painter::Painter};
pub use input_bridge::{listeners::{DomEvent, EventListener}, dispatcher::InputDispatcher};