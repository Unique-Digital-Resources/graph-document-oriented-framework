struct CameraUniform {
    view_proj: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    cam_pos: vec4<f32>,
};

struct GridUniform {
    plane_y: f32,
    base_size: f32,
    color: vec3<f32>,
    fade_dist: f32,
    _padding: vec2<f32>,
};

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(1) @binding(0) var<uniform> grid: GridUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// Full-screen triangle vertex shader
@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VertexOutput {
    var out: VertexOutput;
    // Generate a triangle that covers the whole screen
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0)
    );
    
    let pos = positions[vid];
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = pos * 0.5 + 0.5;
    return out;
}

fn gridLine(coord: f32, size: f32, thickness: f32) -> f32 {
    let c = coord / size;
    let f = abs(fract(c) - 0.5);
    let af = f * 2.0;
    
    // fwidth equivalent in WGSL is dpdx/dpdy
    let w = max(thickness * (abs(dpdx(c)) + abs(dpdy(c))), 1e-6);
    return 1.0 - smoothstep(0.0, w, af);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Reconstruct view ray
    let ndc = vec4<f32>(in.uv * 2.0 - 1.0, -1.0, 1.0);
    var ray_eye = camera.inv_view_proj * ndc;
    ray_eye = vec4<f32>(ray_eye.xyz, 0.0);
    let ray_world = normalize((camera.inv_view_proj * ray_eye).xyz);
    let cam_pos = camera.cam_pos.xyz;

    // Intersect with plane y = grid.plane_y
    let t = (grid.plane_y - cam_pos.y) / ray_world.y;
    
    if (t <= 0.0) {
        discard;
    }
    
    let world_pos = cam_pos + t * ray_world;

    // Grid lines
    let minor = max(gridLine(world_pos.x, grid.base_size, 0.7), gridLine(world_pos.z, grid.base_size, 0.7));
    let major = max(gridLine(world_pos.x, grid.base_size * 10.0, 1.5), gridLine(world_pos.z, grid.base_size * 10.0, 1.5));

    var intensity = max(minor * 0.6, major);
    
    // Distance fade
    let dist = length(world_pos - cam_pos);
    intensity *= 1.0 - smoothstep(grid.fade_dist * 0.5, grid.fade_dist, dist);

    return vec4<f32>(grid.color, intensity);
}