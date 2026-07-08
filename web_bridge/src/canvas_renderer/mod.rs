//! Alternative renderer: Generates Canvas/WebGPU draw calls instead of DOM.

pub mod draw_calls;
pub mod painter;

pub use draw_calls::DrawCall;
pub use painter::Painter;