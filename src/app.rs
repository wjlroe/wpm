use crate::screen::Screen;
use crate::screens;
use crate::*;
use glutin::dpi::*;
use glutin::*;
use std::error::Error;
use std::time::Instant;

pub struct App<'a> {
    running: bool,
    gfx_window: GfxWindow<'a>,
    mouse_position: LogicalPosition,
    current_screen: Box<dyn Screen>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            running: true,
            gfx_window: GfxWindow::new(),
            mouse_position: LogicalPosition::new(0.0, 0.0),
            current_screen: Box::new(screens::TestScreen::new()),
        }
    }

    fn window_resized(&mut self, dt: f32) {
        self.gfx_window.resize();
        self.current_screen.window_resized(&mut self.gfx_window);
        let _ = self.render(dt);
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
                    _ => {}
                },
                _ => {}
            }
        }
        self.current_screen.process_events(dt, &events);
    }

    fn update(&mut self, dt: f32) {
        self.current_screen.update(dt, &mut self.gfx_window);
    }

    fn render(&mut self, dt: f32) -> Result<(), Box<dyn Error>> {
        self.current_screen.render(dt, &mut self.gfx_window)?;
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
