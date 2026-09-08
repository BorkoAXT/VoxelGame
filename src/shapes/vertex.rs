use glam::Vec3;

use crate::constants::{HALF_VOXEL, VOXEL_SIZE};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
impl Vertex {
    pub fn build_vertices(position: Vec3, color: [f32; 4]) -> [Vertex; 8] {
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
            color: color,
        })
    }
}
