use crate::GfxWindow;
use glutin::Event;
use std::error::Error;

pub trait Screen {
    // TODO: probably pass in some sort of collection of available screens
    // screens = Vec<Screen>
    // current_screen = Rc<Screen>
    // so maybe static vec of screens, modify when need to transition to one
    // then return the enum variant that signifies the screen to transition to
    // fn maybe_change_screen(&self) -> Option<ScreenType>
    fn maybe_change_to_screen(&self) -> Option<Box<Screen>>;
    fn process_events(&mut self, dt: f32, events: &[Event]);
    fn update(&mut self, dt: f32, gfx_window: &mut GfxWindow);
    fn window_resized(&mut self, gfx_window: &mut GfxWindow);
    fn render(&self, dt: f32, gfx_window: &mut GfxWindow) -> Result<(), Box<dyn Error>>;
}