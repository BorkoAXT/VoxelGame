use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
    pub const RED: Self = Self(255.0, 0.0, 0.0, 255.0);
    pub const GREEN: Self = Self(0.0, 255.0, 0.0, 255.0);
    pub const BLUE: Self = Self(0.0, 0.0, 255.0, 255.0);
    pub const BLACK: Self = Self(0.0, 0.0, 0.0, 255.0);
    pub const WHITE: Self = Self(255.0, 255.0, 255.0, 255.0);
    pub const TRANSPARENT: Self = Self(0.0, 0.0, 0.0, 0.0);
    pub const SKY: Self = Self(130.0, 200.0, 229.0, 255.0);
    pub const BROWN: Self = Self(164.0, 110.0, 81.0, 255.0);

    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self(red, green, blue, 255.0)
    }
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self(red, green, blue, alpha)
    }
    pub fn as_rgba_unit(&self) -> [f32; 4] {
        [
            self.0 / 255.0,
            self.1 / 255.0,
            self.2 / 255.0,
            self.3 / 255.0,
        ]
    }
    pub fn as_wgpu_color(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.0 as f64 / 255.0,
            g: self.1 as f64 / 255.0,
            b: self.2 as f64 / 255.0,
            a: self.3 as f64 / 255.0,
        }
    }
}
