use super::super::typehelpers::rad::Rad;
use glam::{Mat4, Vec3};
#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub sensitivity: f32,
    // degrees for moving the camera sideways
    pub yaw: Rad,
    // degrees for moving the camera up/down
    pub pitch: Rad,
}
impl Camera {
    pub fn new(
        eye: Vec3,
        target: Vec3,
        up: Vec3,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
        yaw: Rad,
        pitch: Rad,
    ) -> Self {
        return Self {
            eye: eye,
            target: target,
            up: up,
            aspect: aspect,
            fovy: fovy,
            zfar: zfar,
            znear: znear,
            sensitivity: 0.002,
            yaw: yaw,
            pitch: pitch,
        };
    }
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        // right-handed view for the camera.
        let view = glam::camera::rh::view::look_at_mat4(self.eye, self.target, self.up);

        // DirectX projection perspective (goes from 0 to 1.0 instead of the OpenGL -1.0 to 1.0)
        let proj = glam::camera::rh::proj::directx::perspective(
            self.fovy.to_radians(),
            self.aspect,
            self.znear,
            self.zfar,
        );
        return proj * view;
    }
    pub fn set_aspect(&mut self, new_aspect: f32) {
        self.aspect = new_aspect;
    }
}
