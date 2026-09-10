mod camera;
mod constants;
mod shapes;
mod types;
mod voxel_creation;

use anyhow::Result;
use camera::camera::Camera;
use camera::controller::CameraController;
use camera::uniform::Camera_Uniform;
use glam::Vec3;
use pollster::block_on;
use shapes::cube::Cube;
use shapes::vertex::Vertex;
use std::{borrow::Cow, sync::Arc, time::Instant};
use types::color::Color;
use types::radians::Radians;

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BlendState, Buffer,
    BufferAddress, BufferUsages, ColorTargetState, ColorWrites,
    CurrentSurfaceTexture::{Suboptimal, Success},
    Device, DeviceDescriptor, FragmentState, Instance, PipelineLayoutDescriptor, Queue,
    RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor,
    ShaderSource, ShaderStages, Surface, SurfaceConfiguration, VertexBufferLayout, VertexState,
    VertexStepMode,
    util::{BufferInitDescriptor, DeviceExt},
    vertex_attr_array,
};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, LogicalSize},
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::{CursorGrabMode, Window, WindowId},
};

use crate::constants::{MAX_PITCH, MIN_PITCH};
#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    surface: Option<Surface<'static>>,

    device: Option<Device>,
    queue: Option<Queue>,
    config: Option<SurfaceConfiguration>,

    camera: Option<Camera>,
    camera_uniform: Option<Camera_Uniform>,
    camera_controller: Option<CameraController>,

    camera_buffer: Option<Buffer>,
    camera_bind_group: Option<BindGroup>,

    cube: Option<Cube>,

    render_pipeline: Option<RenderPipeline>,

    last_frame: Option<Instant>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("CodingTab")
                        .with_inner_size(LogicalSize::new(1000, 860)),
                )
                .unwrap(),
        );

        let instance = Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) =
            block_on(adapter.request_device(&DeviceDescriptor::default())).unwrap();

        let size = window.inner_size();

        let config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .unwrap();

        surface.configure(&device, &config);
        window.set_cursor_visible(false);

        window
            .set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Confined))
            .unwrap();

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::ZERO,
            Vec3::Y,
            config.width as f32 / config.height as f32,
            90.0,
            0.1,
            500.0,
            Radians::from_degrees(-90.0),
            Radians::from_degrees(0.0),
        );

        let camera_uniform = Camera_Uniform::new();

        let camera_controller = CameraController::new(1.0);

        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),

            contents: bytemuck::bytes_of(&camera_uniform),

            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),

                entries: &[BindGroupLayoutEntry {
                    binding: 0,

                    visibility: ShaderStages::VERTEX,

                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,

                        has_dynamic_offset: false,

                        min_binding_size: None,
                    },

                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),

            layout: &camera_bind_group_layout,

            entries: &[BindGroupEntry {
                binding: 0,

                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Rectangle Shader"),

            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let cube = Cube::new(&device, Vec3::new(1.0, 0.0, 0.0), Color::BROWN);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),

            bind_group_layouts: &[
                Some(&camera_bind_group_layout),
                Some(&cube.bind_group_layout),
            ],

            immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),

            layout: Some(&pipeline_layout),

            vertex: VertexState {
                module: &shader,

                entry_point: Some("vs_main"),

                compilation_options: Default::default(),

                buffers: &[Some(VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as BufferAddress,

                    step_mode: VertexStepMode::Vertex,

                    attributes: &vertex_attr_array![
                        0 => Float32x3,
                        1 => Float32x4
                    ],
                })],
            },

            primitive: Default::default(),

            depth_stencil: None,

            multisample: Default::default(),

            fragment: Some(FragmentState {
                module: &shader,

                entry_point: Some("fs_main"),

                compilation_options: Default::default(),

                targets: &[Some(ColorTargetState {
                    format: config.format,

                    blend: Some(BlendState::REPLACE),

                    write_mask: ColorWrites::ALL,
                })],
            }),

            multiview_mask: None,

            cache: None,
        });

        self.last_frame = Some(Instant::now());

        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);

        self.camera = Some(camera);
        self.camera_uniform = Some(camera_uniform);
        self.camera_controller = Some(camera_controller);

        self.camera_buffer = Some(camera_buffer);
        self.camera_bind_group = Some(camera_bind_group);

        self.cube = Some(cube);

        self.render_pipeline = Some(render_pipeline);

        self.window = Some(window.clone());

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        win_event: WindowEvent,
    ) {
        match win_event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                let (Some(surface), Some(device), Some(config), Some(camera)) = (
                    &self.surface,
                    &self.device,
                    &mut self.config,
                    &mut self.camera,
                ) else {
                    return;
                };

                config.width = size.width.max(1);

                config.height = size.height.max(1);

                surface.configure(device, config);

                camera.set_aspect(config.width as f32 / config.height as f32);
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();

                let dt = self
                    .last_frame
                    .replace(now)
                    .map(|last| (now - last).as_secs_f32())
                    .unwrap_or(0.0);
                let (
                    Some(surface),
                    Some(device),
                    Some(queue),
                    Some(camera),
                    Some(camera_uniform),
                    Some(camera_controller),
                    Some(camera_buffer),
                    Some(camera_bind_group),
                    Some(cube),
                    Some(render_pipeline),
                ) = (
                    &self.surface,
                    &self.device,
                    &self.queue,
                    &mut self.camera,
                    &mut self.camera_uniform,
                    &self.camera_controller,
                    &self.camera_buffer,
                    &self.camera_bind_group,
                    &mut self.cube,
                    &self.render_pipeline,
                )
                else {
                    return;
                };

                camera_controller.update_camera(camera, dt);
                //println!("position: {}, dt: {}, target: {}", camera.position, dt, camera.target);
                camera_uniform.update(camera);

                queue.write_buffer(camera_buffer, 0, bytemuck::bytes_of(camera_uniform));

                let frame = match surface.get_current_texture() {
                    Success(frame) | Suboptimal(frame) => frame,

                    _ => return,
                };

                let view = frame.texture.create_view(&Default::default());

                let mut encoder = device.create_command_encoder(&Default::default());

                {
                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Main Render Pass"),

                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,

                            resolve_target: None,

                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(Color::SKY.as_wgpu_color()),

                                store: wgpu::StoreOp::Store,
                            },

                            depth_slice: None,
                        })],

                        depth_stencil_attachment: None,

                        timestamp_writes: None,

                        occlusion_query_set: None,

                        multiview_mask: None,
                    });

                    pass.set_pipeline(render_pipeline);
                    pass.set_bind_group(0, camera_bind_group, &[]);
                    cube.set_position(&queue);
                    cube.draw(&mut pass);
                }

                queue.submit(Some(encoder.finish()));

                queue.present(frame);

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(controller) = &mut self.camera_controller {
                    if let PhysicalKey::Code(key) = event.physical_key {
                        controller.handle_key(key, event.state);
                        let pos = self.camera.unwrap();
                        println!(
                            "camera x y z: {} {} {}",
                            pos.position.x, pos.position.y, pos.position.z
                        );
                    }
                }
            }

            _ => {}
        }
    }
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _event_id: DeviceId,
        device_event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = device_event {
            if let Some(camera) = &mut self.camera {
                let new_yaw = camera.yaw.as_radians() + dx as f32 * camera.sensitivity;
                let new_pitch = camera.pitch.as_radians() - dy as f32 * camera.sensitivity;

                camera.yaw = Radians::from_radians(new_yaw);
                camera.pitch = Radians::from_radians(new_pitch.clamp(MIN_PITCH, MAX_PITCH));

                dbg!(camera.pitch.as_degrees(), camera.yaw.as_degrees());
            }
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut app = App::default();

    event_loop.run_app(&mut app)?;

    Ok(())
}
