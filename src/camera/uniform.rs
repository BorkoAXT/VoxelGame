use super::camera::Camera;
use glam::Mat4;
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Camera_Uniform {
    pub view_projection: [[f32; 4]; 4],
}
impl Camera_Uniform {
    pub fn new() -> Self {
        Self {
            view_projection: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
    pub fn update(&mut self, camera: &Camera) {
        self.view_projection = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}
