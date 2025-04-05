use crate::*;
use cgmath::*;
use wgpu::util::DeviceExt;
use wgpu::*;
use winit::dpi::*;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use std::error::Error;

pub struct GfxWindow<'a> {
    pub logical_size: LogicalSize<f64>,
    pub physical_size: PhysicalSize<u32>,
    pub window_dim: (u32, u32),
    pub dpi: f64,
    pub fonts: Fonts,
    pub window: Window,
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub swap_chain: SwapChain,
    pub swap_chain_desc: SwapChainDescriptor,
    pub render_pipeline: RenderPipeline,
    pub glyph_brush: wgpu_glyph::GlyphBrush<'a, wgpu::Device, wgpu::Queue>,
}

impl<'a> GfxWindow<'a> {
    pub fn default_win_size(event_loop: &EventLoop<()>) -> Self {
        Self::new(768.0, 576.0, event_loop)
    }

    pub fn new(win_width: f64, win_height: f64, event_loop: &EventLoop<()>) -> Self {
        let logical_size = LogicalSize::new(win_width, win_height);
        let window = WindowBuilder::new()
            .with_title("wpm")
            .with_inner_size(logical_size)
            .build(event_loop)
            .expect("Failed to create window");

        let physical_size = window.inner_size();
        let dpi = window.scale_factor();

        let instance = Instance::new(Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = futures::executor::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        }))
        .expect("Failed to find an appropriate adapter");

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                label: None,
            },
            None,
        ))
        .expect("Failed to create device");

        let swap_chain_desc = SwapChainDescriptor {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: physical_size.width,
            height: physical_size.height,
            present_mode: PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let shader = device.create_shader_module(&include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[ColorTargetState {
                    format: swap_chain_desc.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                }],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let fonts = Fonts::default();

        let glyph_brush = wgpu_glyph::GlyphBrushBuilder::using_font_bytes(include_bytes!(
            "fonts/iosevka-regular.ttf"
        ) as &[u8])
        .build(&device, swap_chain_desc.format);

        Self {
            logical_size,
            physical_size,
            window_dim: (physical_size.width, physical_size.height),
            dpi,
            fonts,
            window,
            surface,
            device,
            queue,
            swap_chain,
            swap_chain_desc,
            render_pipeline,
            glyph_brush,
        }
    }

    pub fn resize(&mut self) {
        self.physical_size = self.window.inner_size();
        self.swap_chain_desc.width = self.physical_size.width;
        self.swap_chain_desc.height = self.physical_size.height;
        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_desc);
        self.window_dim = (self.physical_size.width, self.physical_size.height);
    }

    pub fn window_dim(&self) -> Vector2<f32> {
        (self.window_dim.0 as f32, self.window_dim.1 as f32).into()
    }

    pub fn end_frame(&mut self) -> Result<(), Box<dyn Error>> {
        let frame = self
            .swap_chain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    pub fn draw_quad(&mut self, color: [f32; 4], rect: &rect::Rect, z: f32) {
        // Implement draw_quad using wgpu
    }

    pub fn draw_outline(&mut self, color: [f32; 4], rect: &rect::Rect, z: f32, thickness: f32) {
        // Implement draw_outline using wgpu
    }

    pub fn back_label(&mut self) -> Label {
        let mut label = Label::new(
            UI_TEXT_BUTTON_SIZE,
            self.fonts.iosevka_font_id,
            TEXT_COLOR,
            String::from("←"),
            self,
        )
        .with_layout(
            wgpu_glyph::Layout::default_single_line()
                .v_align(wgpu_glyph::VerticalAlign::Center)
                .h_align(wgpu_glyph::HorizontalAlign::Center),
        );
        label.rect.bounds.x *= 1.5;
        label
    }

    pub fn queue_label(&mut self, label: &Label) {
        let section = label.section(self);
        self.glyph_brush.queue(section);
    }
}
