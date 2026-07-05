//! A list of serializable drawing instructions.
//! 
//! These can be passed to an HTML5 `<canvas>` 2D context or a WebGPU pipeline.

#[derive(Debug, Clone, serde::Serialize)]
pub enum DrawCall {
    ClearRect(f32, f32, f32, f32),
    FillRect {
        x: f32, y: f32, w: f32, h: f32,
        color: String,
    },
    StrokeRect {
        x: f32, y: f32, w: f32, h: f32,
        color: String,
        width: f32,
    },
    FillText {
        text: String,
        x: f32, y: f32,
        font: String,
        color: String,
    },
}