use crate::config::Config;
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
    current_screen: Box<dyn Screen>,
    bg_switch_label: Label,
    config: Config,
}

const MAX_FRAME_TIME: Duration = Duration::from_millis(33);

impl<'a> App<'a> {
    pub fn new(event_loop: &EventsLoop, config: Config) -> Self {
        let mut gfx_window = GfxWindow::default_win_size(event_loop);
        let screen = screens::TestScreen::new(&mut gfx_window, &config);
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
            config,
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

    fn window_resized(&mut self) {
        self.gfx_window.resize();
        self.current_screen.window_resized(&mut self.gfx_window);
        self.recalc_label_positions();
    }

    fn mouse_click(&mut self, position: Vector2<f32>) {
        self.current_screen.mouse_click(dbg!(position));

        if self.bg_switch_label.rect.contains_point(position) {
            swap_colors();
        }
    }

    fn process_event(&mut self, event: &Event) -> bool {
        let mut update_and_render = false;
        match event {
            Event::WindowEvent {
                event: win_event, ..
            } => match win_event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => self.running = false,
                WindowEvent::Resized(new_logical_size) => {
                    self.gfx_window.logical_size = *new_logical_size;
                    self.window_resized();
                    update_and_render = true;
                }
                WindowEvent::HiDpiFactorChanged(new_dpi) => {
                    self.gfx_window.dpi = *new_dpi;
                    self.window_resized();
                    update_and_render = true;
                }
                WindowEvent::Moved(_) => {
                    self.gfx_window.update_monitor();
                    update_and_render = true;
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
                    update_and_render = true;
                }
                _ => {}
            },
            _ => {}
        };
        update_and_render
            || self
                .current_screen
                .process_event(&event, &mut self.gfx_window)
    }

    fn update(&mut self, dt: f32) {
        let physical_mouse = self.mouse_position.to_physical(self.gfx_window.dpi);
        self.render_screen = self.current_screen.update(
            dt,
            vec2(physical_mouse.x as f32, physical_mouse.y as f32),
            &self.config,
            &mut self.gfx_window,
        );

        if let Some(new_screen) = self
            .current_screen
            .maybe_change_to_screen(&mut self.gfx_window, &self.config)
        {
            self.current_screen = new_screen
        }
    }

    fn render(&mut self, dt: f32) -> Result<(), Box<dyn Error>> {
        self.current_screen.render(dt, &mut self.gfx_window)?;

        {
            self.gfx_window.queue_label(&self.bg_switch_label);
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

    pub fn run(&mut self, event_loop: &mut EventsLoop) -> Result<(), Box<dyn Error>> {
        self.window_resized(); // Ensure calculations are completed, without relying on a resize event
        let mut last_frame_time = Instant::now();
        let event_proxy = event_loop.create_proxy();

        thread::spawn(move || loop {
            let _ = event_proxy.wakeup();
            thread::sleep(MAX_FRAME_TIME);
        });

        event_loop.run_forever(move |event| {
            let mut update_and_render = self.process_event(&event);
            let elapsed = last_frame_time.elapsed();
            if elapsed >= MAX_FRAME_TIME {
                update_and_render = true;
            }

            if update_and_render {
                last_frame_time = Instant::now();
                let dt = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1e-9;

                self.update(dt);
                let _ = self.render(dt);
            }

            if self.running {
                ControlFlow::Continue
            } else {
                ControlFlow::Break
            }
        });

        Ok(())
    }
}
