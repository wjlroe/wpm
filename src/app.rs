use crate::layout::Layout;
use crate::*;
use cgmath::*;
use gfx::traits::FactoryExt;
use gfx::{self, *};
use gfx_glyph::*;
use glutin::dpi::*;
use glutin::Api::OpenGl;
use glutin::*;
use std::error::Error;
use std::time::{Duration, Instant};

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
    color: [f32; 4] = "u_Color",
  }

  pipeline pipe {
    vbuf: VertexBuffer<Vertex> = (),
    locals: ConstantBuffer<Locals> = "Locals",
    out_color: BlendTarget<ColorFormat> = ("Target0", state::ColorMask::all(), preset::blend::ALPHA),
    out_depth: DepthTarget<DepthFormat> = preset::depth::LESS_EQUAL_WRITE,
  }
}

#[derive(Clone, Default)]
struct TypingState {
    per_line_height: f32,
    current_line_offset_t: f32,
    current_line_offset_max_t: f32, // time in seconds to animate
    animating: bool,
    current_word_idx: usize,
    first_word_idx_per_line: Vec<usize>,
    word_idx_at_start_of_line: usize,
    word_idx_at_prev_line: usize,
    num_words: usize,
}

impl TypingState {
    fn update(&mut self, dt: f32) {
        if self.animating {
            self.current_line_offset_t += dt;
            if self.current_line_offset_t >= self.current_line_offset_max_t {
                self.animating = false;
                self.current_line_offset_t = 0.0;
            }
        }
    }

    fn skip_num(&self) -> usize {
        if self.animating {
            self.word_idx_at_prev_line
        } else {
            if self.word_idx_at_start_of_line > 0 {
                self.word_idx_at_start_of_line
            } else {
                0
            }
        }
    }

    fn offset(&self) -> f32 {
        if self.animating {
            ease_in_out(
                self.current_line_offset_t,
                0.0,
                self.per_line_height,
                self.current_line_offset_max_t,
            )
        } else {
            0.0
        }
    }

    fn transform(&self, window_dim: Vector2<f32>) -> Matrix4<f32> {
        Matrix4::from_translation(vec3(0.0, self.offset() / (window_dim.y / 2.0), 0.0))
    }

    fn start_animation(&mut self) {
        self.animating = true;
        self.current_line_offset_t = 0.0;
        self.current_line_offset_max_t = 1.5; // n second animation
    }

    fn next_word(&mut self) {
        assert!(
            self.num_lines() > 0,
            "there should be more than zero lines!"
        );
        assert!(self.num_words > 0, "there should be more than zero words!");
        if self.current_word_idx < self.num_words - 1 {
            self.current_word_idx += 1;
            assert!(
                self.per_line_height > 0.0,
                "per_line_height should be non-zero!"
            );
            if self
                .first_word_idx_per_line
                .contains(&self.current_word_idx)
            {
                self.start_animation();
                self.word_idx_at_prev_line = self.word_idx_at_start_of_line;
                self.word_idx_at_start_of_line = self.current_word_idx;
            }
        }
    }

    fn num_lines(&self) -> usize {
        self.first_word_idx_per_line.len()
    }
}

#[derive(Copy, Clone, Debug)]
struct PositionAndBounds {
    position: Vector2<f32>,
    bounds: Vector2<f32>,
}

impl Default for PositionAndBounds {
    fn default() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            bounds: vec2(0.0, 0.0),
        }
    }
}

impl PositionAndBounds {
    fn contains_point(&self, point: Vector2<f32>) -> bool {
        point.x >= self.position.x
            && point.x <= self.position.x + self.bounds.x
            && point.y >= self.position.y
            && point.y <= self.position.y + self.bounds.y
    }
}

pub struct App<'a> {
    running: bool,
    event_loop: EventsLoop,
    logical_size: LogicalSize,
    physical_size: PhysicalSize,
    window_dim: (u16, u16),
    monitor: MonitorId,
    dpi: f64,
    timer_font_size: f64,
    timer_pos_and_bounds: PositionAndBounds,
    typing_font_size: f64,
    typing_pos_and_bounds: PositionAndBounds,
    input_pos_and_bounds: PositionAndBounds,
    iosevka_font_id: FontId,
    roboto_font_id: FontId,
    gfx_window: WindowedContext,
    device: gfx_device_gl::Device,
    main_color: handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    main_depth: handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
    quad_pso: pso::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    quad_slice: Slice<gfx_device_gl::Resources>,
    quad_data: pipe::Data<gfx_device_gl::Resources>,
    glyph_brush: GlyphBrush<'a, gfx_device_gl::Resources, gfx_device_gl::Factory>,
    encoder: Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    typing_test: Option<TypingTest>,
    typing_result: Option<TypingResult>,
    typing_state: TypingState,
    mouse_position: LogicalPosition,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let event_loop = EventsLoop::new();
        let logical_size = LogicalSize::new(768.0, 576.0);
        let monitor = event_loop.get_primary_monitor();
        let dpi = monitor.get_hidpi_factor();
        let physical_size = logical_size.to_physical(dpi);
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

        let mut app = App {
            running: true,
            event_loop,
            logical_size,
            physical_size,
            window_dim: (width, height),
            monitor,
            dpi,
            timer_font_size,
            timer_pos_and_bounds: PositionAndBounds::default(),
            typing_font_size,
            typing_pos_and_bounds: PositionAndBounds::default(),
            input_pos_and_bounds: PositionAndBounds::default(),
            iosevka_font_id,
            roboto_font_id,
            gfx_window,
            device,
            main_color,
            main_depth,
            quad_pso,
            quad_slice,
            quad_data,
            glyph_brush,
            encoder,
            typing_test: None,
            typing_result: None,
            typing_state: TypingState::default(),
            mouse_position: LogicalPosition::new(0.0, 0.0),
        };
        app.update_font_metrics();
        app.start_test();
        app
    }

    fn start_test(&mut self) {
        let mut typing_test = TypingTest::default();
        // FIXME: move into TypingTest and generate from list of common words
        let wordlist = "also|sentence|stop|she|men|see|been|from|we|follow|but|mother|too|form|this|went|to|then|show|have|only|now|around|help|family|old|write|grow|also|over|together|city|end|quite|with|might|eat|four|where|hard|their|take|year|see|place|leave|too|too|is|other|near|around|saw|did|into|question|work|between|your|face|without|tree|as|girl|if|enough|stop|still|put|on|side|there|hear|large|more|be|there|took|some|into|off|down|so|is|tell|way|large|thing|earth|move|their|much|list|small|family|know|under|try|mean|above|end|was|what|night|them|most|good|example|left|mile|that|why|give|because|sea|above|boy|has|go|book|later|eat|land|about|line|life|said|often|really|the|at|without|large|should|away|end|no|oil|any|while|being|before|away|from|light|found|other|open|below|sound|began|come|night|year|world|start|that|it|after|and|show|every|find|old|while|school|your|point|often|example|children|up|found|then|quickly|some|still|again|our|world|may|group|help|point|own|around|make|than|look|girl|sometimes|hand|idea|change|people|get|page|the|own|it's|land|play|last|kind|eye|once|write|you|are|young|take|found|up|once|white|thought|answer|next|still|hand|state|air|food|don't|story|say|of|they|through|keep|far|should|different|eye|been|such|few|through|close|before|below|question|word|and|mother|along|number|miss|sound|her|boy|soon|car|seem|make|food|left|call|where|after|did|answer|write|there|got|mile|line|number|feet|America|earth|it's|find|get|me|home|cut|say|again|home|play|light|give|my|most|will|went|turn|sound|name|could|let|almost|head|carry|look|work|turn|letter|come|new|spell|mountain|move|children|air|live|this|hear|or|every|these|song|can|move|watch|which|picture|own|was|right|does|need|important|river|some|had|after|or|man|study|should|part|would|and|by|watch|earth|head";
        let words = wordlist
            .split('|')
            .map(|word| word.to_string())
            .collect::<Vec<_>>();
        typing_test.words = words;
        typing_test.duration = Some(Duration::from_secs(60));
        self.typing_test = Some(typing_test);
    }

    fn update_font_metrics(&mut self) {
        let mut timer_character_dim = vec2(0.0, 0.0);
        let mut typing_character_dim = vec2(0.0, 0.0);

        let timer_section = Section {
            font_id: self.iosevka_font_id,
            scale: Scale::uniform((self.timer_font_size * self.dpi) as f32),
            text: "0",
            ..Section::default()
        };
        if let Some(dim) = self.glyph_brush.pixel_bounds(timer_section).map(|bounds| {
            let width = bounds.max.x - bounds.min.x;
            let height = bounds.max.y - bounds.min.y;
            vec2(width as f32, height as f32)
        }) {
            timer_character_dim = dim;
        }

        {
            let typed_section = Section {
                font_id: self.roboto_font_id,
                scale: Scale::uniform((self.typing_font_size * self.dpi) as f32),
                text: "A",
                ..Section::default()
            };
            if let Some(dim) = self.glyph_brush.pixel_bounds(typed_section).map(|bounds| {
                let width = bounds.max.x - bounds.min.x;
                let height = bounds.max.y - bounds.min.y;
                vec2(width as f32, height as f32)
            }) {
                typing_character_dim = dim;
            }
        }

        {
            let typed_section = Section {
                font_id: self.roboto_font_id,
                scale: Scale::uniform((self.typing_font_size * self.dpi) as f32),
                text: "A\nA",
                ..Section::default()
            };
            if let Some(dim) = self.glyph_brush.pixel_bounds(typed_section).map(|bounds| {
                let width = bounds.max.x - bounds.min.x;
                let height = bounds.max.y - bounds.min.y;
                vec2(width as f32, height as f32)
            }) {
                typing_character_dim.y = dim.y / 2.0;
            }
        }

        {
            self.typing_pos_and_bounds.bounds =
                vec2(30.0 * typing_character_dim.x, 2.5 * typing_character_dim.y);

            self.input_pos_and_bounds.bounds =
                vec2(30.0 * typing_character_dim.x, 1.5 * typing_character_dim.y);

            let mut vertical_layout = Layout::vertical(self.window_dim());
            let typing_elem = vertical_layout.add_bounds(self.typing_pos_and_bounds.bounds);
            let input_elem = vertical_layout.add_bounds(self.input_pos_and_bounds.bounds);
            vertical_layout.calc_positions();
            self.typing_pos_and_bounds.position = vertical_layout.element_position(typing_elem);
            self.input_pos_and_bounds.position = vertical_layout.element_position(input_elem);

            Layout::center_horizontally(
                self.window_dim(),
                self.typing_pos_and_bounds.bounds,
                &mut self.typing_pos_and_bounds.position,
            );
            Layout::center_horizontally(
                self.window_dim(),
                self.typing_pos_and_bounds.bounds,
                &mut self.input_pos_and_bounds.position,
            );
        }

        {
            if let Some(typing_test) = self.typing_test.as_ref() {
                // calculate by glyphs and detecting y differences...

                self.typing_state = TypingState::default();

                let bounds = vec2(self.typing_pos_and_bounds.bounds.x, 10000.0);
                let typed_section = Section {
                    font_id: self.roboto_font_id,
                    bounds: bounds.into(),
                    scale: Scale::uniform((self.typing_font_size * self.dpi) as f32),
                    text: &typing_test.words_str(),
                    ..Section::default()
                };
                let mut glyph_iter = self.glyph_brush.glyphs(typed_section);
                let mut current_y = 0.0;
                if let Some(glyph_position) = glyph_iter.next().map(|glyph| glyph.position()) {
                    current_y = glyph_position.y;
                }

                let mut glyph_y = current_y;
                for (word_idx, word) in typing_test.words.iter().enumerate() {
                    if word_idx > 0 {
                        // Get the first character/glyph for the word
                        if let Some(glyph_position) =
                            glyph_iter.next().map(|glyph| glyph.position())
                        {
                            glyph_y = glyph_position.y;
                        } else {
                            assert!(false, "we are missing a glyph for this word!");
                        }
                    }
                    if glyph_y != current_y {
                        self.typing_state.first_word_idx_per_line.push(word_idx);
                        if self.typing_state.per_line_height < 0.001 {
                            // TODO: if we calculate per_line_height here, we don't need to do that in the A\nA section above
                            self.typing_state.per_line_height = glyph_y - current_y;
                            // FIXME: these are different! 48.0 vs. 39.0
                            // assert_eq!(self.typing_state.per_line_height, typing_character_dim.y);
                        }
                        current_y = dbg!(glyph_y);
                    }
                    let char_count = word.chars().count();
                    // skip past all other characters in the word
                    // this assumes 1 glyph per character
                    // FIXME: for multi-lingual unicode support, we'll need to be cleverer about glyphs/chars
                    for _ in 1..char_count {
                        let _ = glyph_iter.next().expect("shouldn't run out of glyphs");
                    }
                    self.typing_state.num_words += 1;
                }
                println!(
                    "first_word_idxes: {:?}",
                    self.typing_state.first_word_idx_per_line
                );
            }
        }
    }

    fn window_resized(&mut self, dt: f32) {
        self.physical_size = self.logical_size.to_physical(self.dpi);
        self.gfx_window.resize(self.physical_size);
        gfx_window_glutin::update_views(
            &self.gfx_window,
            &mut self.main_color,
            &mut self.main_depth,
        );
        let (width, height, ..) = self.main_color.get_dimensions();
        self.window_dim = (width, height);
        self.quad_data.out_color = self.main_color.clone();
        self.quad_data.out_depth = self.main_depth.clone();
        self.update_font_metrics();
        let _ = self.render(dt);
    }

    fn type_char(&mut self, typed_char: char) {
        if self.typing_test.is_none() {
            self.start_test();
        }

        if let Some(typing_test) = &mut self.typing_test {
            if typing_test.typed_char(typed_char) {
                self.typing_state.next_word();
            }
        }
    }

    fn type_backspace(&mut self) {
        if let Some(typing_test) = &mut self.typing_test {
            typing_test.backspace();
        }
    }

    fn process_events(&mut self, dt: f32) {
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
                            state: ElementState::Released,
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
                        self.window_resized(dt);
                    }
                    WindowEvent::HiDpiFactorChanged(new_dpi) => {
                        self.dpi = new_dpi;
                        self.window_resized(dt);
                    }
                    WindowEvent::Moved(_) => {
                        self.monitor = self.gfx_window.get_current_monitor();
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.mouse_position = position;
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        ..
                    } => {
                        let real_mouse = self.mouse_position.to_physical(self.dpi);
                        // test if that's inside the typing window...
                        if self
                            .typing_pos_and_bounds
                            .contains_point(vec2(real_mouse.x as f32, real_mouse.y as f32))
                        {
                            println!("you clicked within the typing window!!!");
                            println!("typing_pos_and_bounds: {:?}", self.typing_pos_and_bounds);
                            println!("mouse click! logical: {:?}", self.mouse_position);
                            println!("real_mouse: {:?}", real_mouse);
                            println!(
                                "x offset within the box: {}",
                                real_mouse.x as f32 - self.typing_pos_and_bounds.position.x
                            );
                        } else {
                            println!("you clicked outside the typing window");
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn update_typing_test(&mut self) {
        if let Some(typing_test) = &mut self.typing_test {
            if !typing_test.ended {
                if let Some(true) = typing_test.is_done() {
                    println!("Typing test is done!");
                    typing_test.end();

                    self.typing_result = Some(typing_test.result());
                    println!("Result: {:?}", self.typing_result);
                }
            }
        }
    }

    fn window_dim(&self) -> Vector2<f32> {
        (f32::from(self.window_dim.0), f32::from(self.window_dim.1)).into()
    }

    fn draw_quad(&mut self, color: [f32; 4], bounds: Vector2<f32>, position: Vector2<f32>) {
        let window_dim = self.window_dim();
        let transform = bounds_and_position_as_matrix(window_dim, bounds, position);

        let locals = Locals {
            color,
            transform: transform.into(),
        };
        self.encoder
            .update_constant_buffer(&self.quad_data.locals, &locals);
        self.encoder
            .draw(&self.quad_slice, &self.quad_pso, &self.quad_data);
    }

    fn typed_section(&self, typing_test: &'a TypingTest, skip_num: usize) -> VariedSection<'a> {
        typing_test.words_as_varied_section(
            skip_num,
            self.typing_pos_and_bounds.bounds,
            self.typing_pos_and_bounds.position,
            (self.typing_font_size * self.dpi) as f32,
            self.roboto_font_id,
        )
    }

    fn render(&mut self, dt: f32) -> Result<(), Box<dyn Error>> {
        self.encoder.clear(&self.main_color, BG_COLOR);
        self.encoder.clear_depth(&self.main_depth, 1.0);

        self.draw_quad(
            TYPING_BG,
            self.typing_pos_and_bounds.bounds,
            self.typing_pos_and_bounds.position,
        );
        self.draw_quad(
            INPUT_BG,
            self.input_pos_and_bounds.bounds,
            self.input_pos_and_bounds.position,
        );

        if let Some(typing_test) = self.typing_test.as_ref() {
            // TODO: skip the full entered lines before current word...
            self.typing_state.update(dt);
            let skip_num = self.typing_state.skip_num();
            self.glyph_brush
                .queue(self.typed_section(typing_test, skip_num));
            self.glyph_brush.draw_queued_with_transform(
                self.typing_state.transform(self.window_dim()).into(),
                &mut self.encoder,
                &self.main_color,
                &self.main_depth,
            )?;

            let input_section = Section {
                text: &typing_test.entered_text,
                color: PENDING_WORD_COLOR,
                font_id: self.roboto_font_id,
                scale: Scale::uniform((self.typing_font_size * self.dpi) as f32),
                bounds: self.input_pos_and_bounds.bounds.into(),
                screen_position: self.input_pos_and_bounds.position.into(),
                ..Section::default()
            };
            self.glyph_brush.queue(input_section);

            // Render clock countdown timer
            if let Some(time_remaining) = typing_test.remaining_time_string() {
                // TODO: position and bounds should be set
                let time_section = Section {
                    text: &time_remaining,
                    font_id: self.iosevka_font_id,
                    scale: Scale::uniform((self.timer_font_size * self.dpi) as f32),
                    ..Section::default()
                };
                self.glyph_brush.queue(time_section);
            }

            self.glyph_brush
                .draw_queued(&mut self.encoder, &self.main_color, &self.main_depth)?;
        }

        // end of frame stuff now
        self.encoder.flush(&mut self.device);
        self.gfx_window.swap_buffers()?;
        self.device.cleanup();

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut last_frame_time = Instant::now();

        while self.running {
            let elapsed = last_frame_time.elapsed();
            last_frame_time = Instant::now();
            let dt = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1e-9;

            self.process_events(dt);
            self.update_typing_test();
            self.render(dt)?;
        }

        Ok(())
    }
}

fn bounds_and_position_as_matrix(
    window_dim: Vector2<f32>,
    bounds: Vector2<f32>,
    position: Vector2<f32>,
) -> Matrix4<f32> {
    let scale =
        Matrix4::from_nonuniform_scale(bounds.x / window_dim.x, bounds.y / window_dim.y, 1.0);

    let x_move = 2.0 * (position.x + bounds.x / 2.0 - window_dim.x / 2.0) / window_dim.x;
    let y_move = -2.0 * (position.y + bounds.y / 2.0 - window_dim.y / 2.0) / window_dim.y;
    let translation = Matrix4::from_translation(vec3(x_move, y_move, 0.0));

    let transform = translation * scale; // scale then translate

    transform
}

#[test]
fn test_bounds_and_position_as_matrix() {
    fn vec4_from_2(vec: Vector2<f32>) -> Vector4<f32> {
        vec4(vec.x, vec.y, 1.0, 1.0)
    }

    {
        // quad half the size of the screen, positioned at the top-left of the screen
        let transform =
            bounds_and_position_as_matrix(vec2(200.0, 100.0), vec2(100.0, 50.0), vec2(0.0, 0.0));

        assert_eq!(
            vec4_from_2(vec2(-1.0, 0.0)),
            transform * vec4_from_2(vec2(-1.0, -1.0)) // top-left coord
        );
        assert_eq!(
            vec4_from_2(vec2(0.0, 0.0)),
            transform * vec4_from_2(vec2(1.0, -1.0)) // top-right coord
        );
        assert_eq!(
            vec4_from_2(vec2(0.0, 1.0)),
            transform * vec4_from_2(vec2(1.0, 1.0)) // bottom-right coord
        );
        assert_eq!(
            vec4_from_2(vec2(-1.0, 1.0)),
            transform * vec4_from_2(vec2(-1.0, 1.0)) // bottom-left coord
        );
    }

    {
        // quad half the size of the screen, positioned at the bottom-right of the screen
        let transform =
            bounds_and_position_as_matrix(vec2(200.0, 100.0), vec2(100.0, 50.0), vec2(100.0, 50.0));

        assert_eq!(
            vec4_from_2(vec2(0.0, -1.0)),
            transform * vec4_from_2(vec2(-1.0, -1.0)) // top-left coord
        );
        assert_eq!(
            vec4_from_2(vec2(1.0, -1.0)),
            transform * vec4_from_2(vec2(1.0, -1.0)) // top-right coord
        );
        assert_eq!(
            vec4_from_2(vec2(1.0, 0.0)),
            transform * vec4_from_2(vec2(1.0, 1.0)) // bottom-right coord
        );
        assert_eq!(
            vec4_from_2(vec2(0.0, 0.0)),
            transform * vec4_from_2(vec2(-1.0, 1.0)) // bottom-left coord
        );
    }

    {
        let transform =
            bounds_and_position_as_matrix(vec2(100.0, 50.0), vec2(100.0, 50.0), vec2(0.0, 0.0));

        // All points on the unit quad should remain the same for a quad filling the screen
        for vertex in &QUAD {
            let coord = vec4_from_2(vertex.pos.into());
            assert_eq!(coord, transform * coord);
        }
    }
}
