use super::camera::Camera;
use glam::Vec3;
use std::default::Default;
use winit::keyboard::KeyCode;
#[derive(Default)]
pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_sprinting: bool,
    sprinting_multiplier: f32,
}
impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed: speed,
            sprinting_multiplier: 1.0,
            ..Default::default()
        }
    }
    pub fn handle_key(&mut self, code: KeyCode, state: winit::event::ElementState) -> bool {
        match code {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.is_forward_pressed = state.is_pressed();
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_backward_pressed = state.is_pressed();
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = state.is_pressed();
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.is_left_pressed = state.is_pressed();
                true
            }
            KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                self.is_sprinting = state.is_pressed();
                if self.is_sprinting {
                    self.sprinting_multiplier = 2.0;
                    return true;
                } else {
                    self.sprinting_multiplier = 1.0;
                    return false;
                }
            }
            _ => false,
        }
    }
    pub fn update_camera(&self, camera: &mut Camera, dt: f32) {
        let yaw = camera.yaw.as_radians();
        let pitch = camera.pitch.as_radians();

        let forward = Vec3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        let right = forward.cross(camera.up).normalize();

        let mut movement = Vec3::ZERO;

        if self.is_forward_pressed {
            movement += forward;
        }

        if self.is_backward_pressed {
            movement -= forward;
        }

        if self.is_right_pressed {
            movement += right;
        }

        if self.is_left_pressed {
            movement -= right;
        }

        if movement.length_squared() > 0.0 {
            camera.eye += movement.normalize() * self.speed * dt * self.sprinting_multiplier;
        }
        camera.target = camera.eye + forward;
    }
}
