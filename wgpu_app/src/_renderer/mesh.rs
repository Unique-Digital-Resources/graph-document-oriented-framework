use engine::core::graph::storage::Graph;
use engine::core::node::properties::PropertyValue;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl MeshVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct MeshBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl MeshBuffer {
    pub fn from_graph(device: &wgpu::Device, graph: &Graph) -> Option<Self> {
        let mesh_node = graph.iter_nodes().find(|n| n.type_id.as_str() == "MeshNode")?;

        let mut vertices = Vec::new();
        if let Some(PropertyValue::Array(arr)) = mesh_node.get("vertices") {
            let get_f = |idx: usize| -> f32 {
                if let PropertyValue::Float(v) = &arr[idx] { *v as f32 } else { 0.0 }
            };
            let mut i = 0;
            while i + 5 < arr.len() {
                vertices.push(MeshVertex {
                    position: [get_f(i), get_f(i+1), get_f(i+2)],
                    color: [get_f(i+3), get_f(i+4), get_f(i+5)],
                });
                i += 6;
            }
        }

        let mut indices = Vec::new();
        if let Some(PropertyValue::Array(arr)) = mesh_node.get("indices") {
            for v in arr {
                if let PropertyValue::Int(i) = v {
                    indices.push(*i as u32);
                }
            }
        }

        if vertices.is_empty() || indices.is_empty() { return None; }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Some(Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        })
    }
}