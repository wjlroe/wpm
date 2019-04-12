use crate::*;
use cgmath::*;
use gfx::traits::FactoryExt;
use gfx::*;
use gfx_device_gl;
use gfx_glyph::*;
use glutin::dpi::*;
use glutin::Api::OpenGl;
use glutin::*;
use std::error::Error;

pub struct GfxWindow<'a> {
    pub event_loop: EventsLoop,
    pub logical_size: LogicalSize,
    pub physical_size: PhysicalSize,
    pub window_dim: (u16, u16),
    pub monitor: MonitorId,
    pub dpi: f64,
    pub fonts: Fonts,
    pub window: WindowedContext,
    pub device: gfx_device_gl::Device,
    pub quad_bundle:
        pso::bundle::Bundle<gfx_device_gl::Resources, pipe::Data<gfx_device_gl::Resources>>,
    pub glyph_brush: GlyphBrush<'a, gfx_device_gl::Resources, gfx_device_gl::Factory>,
    pub encoder: Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
}

impl<'a> GfxWindow<'a> {
    pub fn new() -> Self {
        let event_loop = EventsLoop::new();
        let logical_size = LogicalSize::new(768.0, 576.0);
        let monitor = event_loop.get_primary_monitor();
        let dpi = monitor.get_hidpi_factor();
        let physical_size = logical_size.to_physical(dpi);

        let window_builder = WindowBuilder::new()
            .with_title("wpm")
            .with_dimensions(logical_size);
        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(OpenGl, (4, 3)))
            .with_gl_profile(GlProfile::Core)
            .with_vsync(true);
        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(
                window_builder,
                context,
                &event_loop,
            )
            .expect("init gfx_window_glutin should work!");

        let (width, height, ..) = main_color.get_dimensions();

        let quad_pso = factory
            .create_pipeline_simple(
                include_bytes!("shaders/quad_150_core.vert"),
                include_bytes!("shaders/quad_150_core.frag"),
                pipe::new(),
            )
            .expect("quad pso construction to work");
        let (quad_vbuf, quad_slice) =
            factory.create_vertex_buffer_with_slice(&QUAD, &QUAD_INDICES as &[u16]);
        let quad_data = pipe::Data {
            vbuf: quad_vbuf,
            locals: factory.create_constant_buffer(3),
            out_color: main_color,
            out_depth: main_depth,
        };
        let quad_bundle = pso::bundle::Bundle::new(quad_slice, quad_pso, quad_data);

        let mut fonts = Fonts::default();

        let mut glyph_brush =
            GlyphBrushBuilder::using_font_bytes(include_bytes!("iosevka-regular.ttf") as &[u8])
                .initial_cache_size((512, 512))
                .depth_test(preset::depth::LESS_EQUAL_WRITE)
                .build(factory.clone());
        fonts.iosevka_font_id = FontId::default();
        fonts.roboto_font_id =
            glyph_brush.add_font_bytes(include_bytes!("Roboto-Regular.ttf") as &[u8]);

        let encoder: Encoder<_, _> = factory.create_command_buffer().into();

        Self {
            event_loop,
            logical_size,
            physical_size,
            window_dim: (width, height),
            monitor,
            dpi,
            fonts,
            window,
            device,
            quad_bundle,
            glyph_brush,
            encoder,
        }
    }

    pub fn resize(&mut self) {
        self.physical_size = self.logical_size.to_physical(self.dpi);
        self.window.resize(self.physical_size);
        gfx_window_glutin::update_views(
            &self.window,
            &mut self.quad_bundle.data.out_color,
            &mut self.quad_bundle.data.out_depth,
        );
        let (width, height, ..) = self.quad_bundle.data.out_color.get_dimensions();
        self.window_dim = (width, height);
    }

    pub fn get_events(&mut self, events: &mut Vec<Event>) {
        self.event_loop.poll_events(|event| events.push(event));
    }

    pub fn update_monitor(&mut self) {
        self.monitor = self.window.get_current_monitor();
    }

    pub fn window_dim(&self) -> Vector2<f32> {
        (f32::from(self.window_dim.0), f32::from(self.window_dim.1)).into()
    }

    pub fn end_frame(&mut self) -> Result<(), Box<dyn Error>> {
        // end of frame stuff now
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers()?;
        self.device.cleanup();
        Ok(())
    }

    pub fn draw_quad(&mut self, color: [f32; 4], rect: &rect::Rect, z: f32) {
        let window_dim = self.window_dim();
        draw_quad(
            &mut self.quad_bundle,
            rect,
            color,
            z,
            window_dim,
            &mut self.encoder,
        );
    }
}
