use engine::core::node::node::Node;
use engine::core::node::properties::PropertyValue;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GridUniform {
    pub plane_y: f32,
    pub base_size: f32,
    pub color: [f32; 3],
    pub fade_dist: f32,
    pub _padding: [f32; 2],
}

impl GridUniform {
    pub fn from_node(node: &Node) -> Self {
        let get_f = |prop: &str| -> f32 {
            if let Some(PropertyValue::Float(v)) = node.get(prop) { *v as f32 } else { 0.0 }
        };
        
        let mut color = [0.8, 0.8, 0.8];
        if let Some(PropertyValue::Array(arr)) = node.get("color") {
            if arr.len() >= 3 {
                for i in 0..3 {
                    if let PropertyValue::Float(v) = &arr[i] { color[i] = *v as f32; }
                }
            }
        }

        Self {
            plane_y: get_f("plane_y"),
            base_size: get_f("base_size"),
            color,
            fade_dist: get_f("fade_dist"),
            _padding: [0.0, 0.0],
        }
    }
}