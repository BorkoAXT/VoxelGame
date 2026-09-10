use glam::{Mat4, Vec3};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, Buffer, BufferUsages, Device, Queue, RenderPass, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use super::vertex::Vertex;
use crate::{constants::INDICES_ARRAY, types::color::Color};
pub struct Cube {
    vertex_buffer: Buffer,
    index_buffer: Buffer,

    uniform_buffer: Buffer,
    pub bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,

    pub position: Vec3,
    pub color: Color,
}
impl Cube {
    pub fn new(device: &Device, position: Vec3, color: Color) -> Self {
        let vertices = Vertex::build_vertices(Vec3::ZERO, color.as_rgba_unit());
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Cube index buffer"),
            contents: bytemuck::cast_slice(&INDICES_ARRAY),
            usage: BufferUsages::INDEX,
        });
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Cube vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        let model_matrix = Mat4::from_translation(position);
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Cube uniform buffer"),
            contents: bytemuck::cast_slice(&[model_matrix]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Cube bind group layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: (wgpu::BufferBindingType::Uniform),
                    has_dynamic_offset: (false),
                    min_binding_size: (None),
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Cube bind group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        Self {
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            bind_group_layout,
            bind_group,
            position,
            color,
        }
    }
    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..INDICES_ARRAY.len() as u32, 0, 0..1);
    }
    pub fn set_position(&mut self, queue: &Queue) {
        let model_matrix = Mat4::from_translation(self.position);
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[model_matrix]),
        );
    }
}
