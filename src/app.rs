use crate::screens;
use crate::*;
use cgmath::*;
use glutin::dpi::*;
use glutin::*;
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

pub struct App<'a> {
    running: bool,
    gfx_window: GfxWindow<'a>,
    mouse_position: LogicalPosition,
    render_screen: bool,
    current_screen: Box<Screen>,
    bg_switch_label: Label,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let mut gfx_window = GfxWindow::default();
        let screen = screens::TestScreen::new(&mut gfx_window);
        let bg_switch_label = Label::new(
            32.0, // FIXME: what font size?
            gfx_window.fonts.iosevka_font_id,
            VIOLET,
            String::from("Switch BG color"), // FIXME: icon instead?
            &mut gfx_window,
        );
        App {
            running: true,
            gfx_window,
            mouse_position: LogicalPosition::new(0.0, 0.0),
            render_screen: true,
            current_screen: Box::new(screen),
            bg_switch_label,
        }
    }

    fn recalc_label_positions(&mut self) {
        // position bg_switch_label at the bottom of the screen
        let border_size = 35.0;
        let (win_width, win_height) = self.gfx_window.window_dim().into();
        self.bg_switch_label.rect.position.x =
            win_width - self.bg_switch_label.rect.bounds.x - border_size;
        self.bg_switch_label.rect.position.y =
            win_height - self.bg_switch_label.rect.bounds.y - border_size;
    }

    fn window_resized(&mut self, dt: f32) {
        self.gfx_window.resize();
        self.current_screen.window_resized(&mut self.gfx_window);
        self.recalc_label_positions();
        self.render_screen = true;
        let _ = self.render(dt);
    }

    fn mouse_click(&mut self, position: Vector2<f32>) {
        self.current_screen.mouse_click(position);

        if self.bg_switch_label.rect.contains_point(position) {
            swap_colors();
        }
    }

    fn process_events(&mut self, dt: f32) {
        let mut events = vec![];

        self.gfx_window.get_events(&mut events);

        for event in events.iter() {
            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => self.running = false,
                    WindowEvent::Resized(new_logical_size) => {
                        self.gfx_window.logical_size = *new_logical_size;
                        self.window_resized(dt);
                    }
                    WindowEvent::HiDpiFactorChanged(new_dpi) => {
                        self.gfx_window.dpi = *new_dpi;
                        self.window_resized(dt);
                    }
                    WindowEvent::Moved(_) => {
                        self.gfx_window.update_monitor();
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.mouse_position = *position;
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        ..
                    } => {
                        let physical_mouse = self.mouse_position.to_physical(self.gfx_window.dpi);
                        self.mouse_click(vec2(physical_mouse.x as f32, physical_mouse.y as f32));
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        self.current_screen.process_events(dt, &events);
    }

    fn update(&mut self, dt: f32) {
        let physical_mouse = self.mouse_position.to_physical(self.gfx_window.dpi);
        self.render_screen = self.current_screen.update(
            dt,
            vec2(physical_mouse.x as f32, physical_mouse.y as f32),
            &mut self.gfx_window,
        );

        if let Some(new_screen) = self
            .current_screen
            .maybe_change_to_screen(&mut self.gfx_window)
        {
            self.current_screen = new_screen
        } else if !self.render_screen {
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn render(&mut self, dt: f32) -> Result<(), Box<dyn Error>> {
        self.current_screen.render(dt, &mut self.gfx_window)?;

        {
            let mut switch_section = self.bg_switch_label.section(&mut self.gfx_window);
            switch_section.bounds = self.bg_switch_label.rect.bounds.into();
            switch_section.screen_position = self.bg_switch_label.rect.position.into();
            self.gfx_window.glyph_brush.queue(switch_section);
            self.gfx_window
                .glyph_brush
                .use_queue()
                .depth_target(&self.gfx_window.quad_bundle.data.out_depth)
                .draw(
                    &mut self.gfx_window.encoder,
                    &self.gfx_window.quad_bundle.data.out_color,
                )?;
        }

        self.gfx_window.end_frame()?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut last_frame_time = Instant::now();

        while self.running {
            let elapsed = last_frame_time.elapsed();
            last_frame_time = Instant::now();
            let dt = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1e-9;

            self.process_events(dt);
            self.update(dt);
            self.render(dt)?;
        }

        Ok(())
    }
}
