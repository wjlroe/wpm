use crate::{TypingResult, TypingTest};
use cgmath::*;
use gfx::traits::FactoryExt;
use gfx::{self, *};
use gfx_glyph::*;
use glutin::dpi::*;
use glutin::Api::OpenGl;
use glutin::*;
use std::error::Error;
use std::time::Duration;

const BG_COLOR: [f32; 4] = [0.22, 0.55, 0.3, 1.0];

const NO_MODS: ModifiersState = ModifiersState {
    ctrl: false,
    alt: false,
    shift: false,
    logo: false,
};

type ColorFormat = format::Rgba8;
type DepthFormat = format::Depth;

const QUAD: [Vertex; 4] = [
    Vertex { pos: [-1.0, 1.0] },
    Vertex { pos: [-1.0, -1.0] },
    Vertex { pos: [1.0, -1.0] },
    Vertex { pos: [1.0, 1.0] },
];
const QUAD_INDICES: [u16; 6] = [0u16, 1, 2, 2, 3, 0];

gfx_defines! {
  vertex Vertex {
    pos: [f32; 2] = "a_Pos",
  }

  constant Locals {
    transform: [[f32; 4]; 4] = "u_Transform",
    color: [f32; 3] = "u_Color",
  }

  pipeline pipe {
    vbuf: VertexBuffer<Vertex> = (),
    locals: ConstantBuffer<Locals> = "Locals",
    out_color: BlendTarget<ColorFormat> = ("Target0", state::ColorMask::all(), preset::blend::ALPHA),
    out_depth: DepthTarget<DepthFormat> = preset::depth::LESS_EQUAL_WRITE,
  }
}

pub struct App<'a> {
    running: bool,
    event_loop: EventsLoop,
    logical_size: LogicalSize,
    physical_size: PhysicalSize,
    monitor: MonitorId,
    dpi: f64,
    timer_font_size: f64,
    typing_font_size: f64,
    iosevka_font_id: FontId,
    roboto_font_id: FontId,
    gfx_window: WindowedContext,
    device: gfx_device_gl::Device,
    factory: gfx_device_gl::Factory,
    main_color: handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    main_depth: handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
    quad_pso: pso::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    quad_slice: Slice<gfx_device_gl::Resources>,
    quad_data: pipe::Data<gfx_device_gl::Resources>,
    glyph_brush: GlyphBrush<'a, gfx_device_gl::Resources, gfx_device_gl::Factory>,
    encoder: Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    typing_test: Option<TypingTest>,
    typing_result: Option<TypingResult>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let event_loop = EventsLoop::new();
        let logical_size = LogicalSize::new(1024.0, 768.0);
        let monitor = event_loop.get_primary_monitor();
        let dpi = monitor.get_hidpi_factor();
        let physical_size = logical_size.to_physical(dpi.into());
        let timer_font_size = 48.0;
        let typing_font_size = 32.0;

        let window_builder = WindowBuilder::new()
            .with_title("wpm")
            .with_dimensions(logical_size);
        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(OpenGl, (4, 3)))
            .with_gl_profile(GlProfile::Core)
            .with_vsync(true);
        let (gfx_window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(
                window_builder,
                context,
                &event_loop,
            )
            .expect("init gfx_window_glutin should work!");

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
            locals: factory.create_constant_buffer(2),
            out_color: main_color.clone(),
            out_depth: main_depth.clone(),
        };

        let mut glyph_brush =
            GlyphBrushBuilder::using_font_bytes(include_bytes!("iosevka-regular.ttf") as &[u8])
                .initial_cache_size((512, 512))
                .depth_test(gfx::preset::depth::LESS_EQUAL_WRITE)
                .build(factory.clone());
        let iosevka_font_id = FontId::default();
        let roboto_font_id =
            glyph_brush.add_font_bytes(include_bytes!("Roboto-Regular.ttf") as &[u8]);

        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        App {
            running: true,
            event_loop,
            logical_size,
            physical_size,
            monitor,
            dpi,
            timer_font_size,
            typing_font_size,
            iosevka_font_id,
            roboto_font_id,
            gfx_window,
            device,
            factory,
            main_color,
            main_depth,
            quad_pso,
            quad_slice,
            quad_data,
            glyph_brush,
            encoder,
            typing_test: None,
            typing_result: None,
        }
    }

    fn start_test(&mut self) {
        let mut typing_test = TypingTest::default();
        let wordlist = "also|sentence|stop|she|men|see|been|from|we|follow|but|mother|too|form|this|went|to|then|show|have|only|now|around|help|family|old|write|grow|also|over|together|city|end|quite|with|might|eat|four|where|hard|their|take|year|see|place|leave|too|too|is|other|near|around|saw|did|into|question|work|between|your|face|without|tree|as|girl|if|enough|stop|still|put|on|side|there|hear|large|more|be|there|took|some|into|off|down|so|is|tell|way|large|thing|earth|move|their|much|list|small|family|know|under|try|mean|above|end|was|what|night|them|most|good|example|left|mile|that|why|give|because|sea|above|boy|has|go|book|later|eat|land|about|line|life|said|often|really|the|at|without|large|should|away|end|no|oil|any|while|being|before|away|from|light|found|other|open|below|sound|began|come|night|year|world|start|that|it|after|and|show|every|find|old|while|school|your|point|often|example|children|up|found|then|quickly|some|still|again|our|world|may|group|help|point|own|around|make|than|look|girl|sometimes|hand|idea|change|people|get|page|the|own|it's|land|play|last|kind|eye|once|write|you|are|young|take|found|up|once|white|thought|answer|next|still|hand|state|air|food|don't|story|say|of|they|through|keep|far|should|different|eye|been|such|few|through|close|before|below|question|word|and|mother|along|number|miss|sound|her|boy|soon|car|seem|make|food|left|call|where|after|did|answer|write|there|got|mile|line|number|feet|America|earth|it's|find|get|me|home|cut|say|again|home|play|light|give|my|most|will|went|turn|sound|name|could|let|almost|head|carry|look|work|turn|letter|come|new|spell|mountain|move|children|air|live|this|hear|or|every|these|song|can|move|watch|which|picture|own|was|right|does|need|important|river|some|had|after|or|man|study|should|part|would|and|by|watch|earth|head";
        let words = wordlist
            .split('|')
            .map(|word| word.to_string())
            .collect::<Vec<_>>();
        typing_test.words = words;
        typing_test.duration = Some(Duration::from_secs(60));
        self.typing_test = Some(typing_test);
    }

    fn window_resized(&mut self) {
        self.physical_size = self.logical_size.to_physical(self.dpi.into());
        self.gfx_window.resize(self.physical_size);
        gfx_window_glutin::update_views(
            &self.gfx_window,
            &mut self.main_color,
            &mut self.main_depth,
        );
    }

    fn type_char(&mut self, typed_char: char) {
        if self.typing_test.is_none() {
            self.start_test();
        }

        if let Some(typing_test) = &mut self.typing_test {
            typing_test.typed_char(typed_char);
        }
    }

    fn type_backspace(&mut self) {
        if let Some(typing_test) = &mut self.typing_test {
            typing_test.backspace();
        }
    }

    fn process_events(&mut self) {
        let mut events = vec![];

        self.event_loop.poll_events(|event| events.push(event));

        for event in events {
            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::ReceivedCharacter(typed_char) if !typed_char.is_control() => {
                        self.type_char(typed_char);
                    }
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => self.running = false,
                    WindowEvent::KeyboardInput {
                        input: keyboard_input,
                        ..
                    } => match keyboard_input {
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Back),
                            state: ElementState::Pressed,
                            modifiers,
                            ..
                        } => {
                            if modifiers == NO_MODS {
                                self.type_backspace();
                            }
                        }
                        _ => {}
                    },
                    WindowEvent::Resized(new_logical_size) => {
                        self.logical_size = new_logical_size;
                        self.window_resized();
                    }
                    WindowEvent::HiDpiFactorChanged(new_dpi) => {
                        self.dpi = new_dpi;
                        self.window_resized();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn update_typing_test(&mut self) {
        if let Some(typing_test) = &mut self.typing_test {
            if let Some(true) = typing_test.is_done() {
                println!("Typing test is done!");
                typing_test.end();

                let result = typing_test.result();
                println!("Result: {:?}", result);
            }
        }
    }

    fn window_dim(&self) -> Vector2<f32> {
        vec2(
            self.physical_size.width as f32,
            self.physical_size.height as f32,
        )
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        self.encoder.clear(&self.main_color, BG_COLOR);
        self.encoder.clear_depth(&self.main_depth, 1.0);

        if let Some(typing_test) = self.typing_test.as_ref() {
            // Render text to type...

            // Render clock countdown timer
            if let Some(time_remaining) = typing_test.remining_time_string() {
                let time_section = Section {
                    text: &time_remaining,
                    font_id: self.iosevka_font_id,
                    scale: Scale::uniform((self.timer_font_size * self.dpi) as f32),
                    ..Section::default()
                };
                self.glyph_brush.queue(time_section);
                self.glyph_brush.draw_queued(
                    &mut self.encoder,
                    &self.main_color,
                    &self.main_depth,
                )?;
            }
        }

        #[cfg(nope)]
        {
            // draw some sort of quad thingy
            let locals = Locals {
                color,
                transform: transform.into(),
            };
            self.encoder
                .update_constant_buffer(&self.data.locals, &locals);
            self.encoder
                .draw(&self.quad_slice, &self.quad_pso, &self.data);
        }

        // end of frame stuff now
        self.encoder.flush(&mut self.device);
        self.gfx_window.swap_buffers()?;
        self.device.cleanup();

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while self.running {
            self.process_events();
            self.update_typing_test();
            self.render()?;
        }

        Ok(())
    }
}
