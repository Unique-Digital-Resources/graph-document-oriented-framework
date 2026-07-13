use cgmath::{Matrix4, Point3, Vector3, Deg, Rad, PerspectiveFov, Quaternion, InnerSpace, EuclideanSpace, SquareMatrix, Rotation3};
use engine::core::graph::storage::Graph;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
    pub inv_view_proj: [[f32; 4]; 4],
    pub cam_pos: [f32; 4],
}

pub struct CameraController {
    pub position: cgmath::Vector3<f32>,
    pub target: cgmath::Vector3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub fov: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            position: (5.0, 5.0, 5.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: (0.0, 1.0, 0.0).into(),
            fov: 45.0,
        }
    }

    pub fn process_input(&mut self, dx: f32, dy: f32, zoom_delta: f32, interaction: &str) {
        let sensitivity = 0.01;
        
        if interaction == "orbit" {
            let mut dir = self.position - self.target;
            let right = dir.cross(self.up).normalize();
            
            let yaw_rot = Quaternion::from_axis_angle(self.up.normalize(), Rad(-dx * sensitivity));
            dir = yaw_rot * dir;
            
            let pitch_rot = Quaternion::from_axis_angle(right, Rad(-dy * sensitivity));
            dir = pitch_rot * dir;
            
            self.position = self.target + dir;
        } else if interaction == "pan" {
            let dir = self.position - self.target;
            let right = dir.cross(self.up).normalize();
            let true_up = right.cross(dir).normalize();
            
            let pan_speed = 0.01 * dir.magnitude();
            
            self.position -= right * (dx * pan_speed);
            self.target -= right * (dx * pan_speed);
            
            self.position += true_up * (dy * pan_speed);
            self.target += true_up * (dy * pan_speed);
        } else if interaction == "zoom" {
            let dir = self.position - self.target;
            let zoom_speed = 0.1;
            let new_len = dir.magnitude() + (zoom_delta * zoom_speed);
            
            if new_len > 0.1 {
                self.position = self.target + dir.normalize() * new_len;
            }
        }
    }

    pub fn build_uniform(&self, aspect: f32) -> CameraUniform {
        let view = Matrix4::look_at_rh(
            Point3::from_vec(self.position),
            Point3::from_vec(self.target),
            self.up,
        );
        
        // Use cgmath::perspective directly which returns a Matrix4
        let proj = cgmath::perspective(Rad::from(Deg(self.fov)), aspect, 0.1, 1000.0);
        
        let view_proj = proj * view;
        let inv_view_proj = view_proj.invert().unwrap_or(Matrix4::identity());

        CameraUniform {
            view_proj: view_proj.into(),
            inv_view_proj: inv_view_proj.into(),
            cam_pos: [self.position.x, self.position.y, self.position.z, 1.0],
        }
    }
}