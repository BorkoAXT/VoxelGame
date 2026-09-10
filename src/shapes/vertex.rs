use glam::Vec3;

use crate::{constants::HALF_VOXEL, types::color::Color};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color_rgba: [f32; 4],
}
impl Vertex {
    pub fn build_vertices(position: Vec3, color_rgba: [f32; 4]) -> [Vertex; 8] {
        let h = HALF_VOXEL;
        let offsets = [
            Vec3::new(-h, -h, -h),
            Vec3::new(h, -h, -h),
            Vec3::new(h, h, -h),
            Vec3::new(-h, h, -h),
            Vec3::new(-h, -h, h),
            Vec3::new(h, -h, h),
            Vec3::new(h, h, h),
            Vec3::new(-h, h, h),
        ];
        offsets.map(|offset| Vertex {
            position: (position + offset).into(),
            color_rgba: color_rgba,
        })
    }
}
