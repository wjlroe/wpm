use crate::GfxWindow;
use cgmath::*;
use glutin::Event;
use std::error::Error;

pub trait Screen {
    // TODO: probably pass in some sort of collection of available screens
    // screens = Vec<Screen>
    // screen_stack = Vec<Screen> - push screens on, pop then off to go back
    // current_screen = Rc<Screen>
    // so maybe static vec of screens, modify when need to transition to one
    // then return the enum variant that signifies the screen to transition to
    // fn maybe_change_screen(&self) -> Option<ScreenType>
    fn maybe_change_to_screen(&self, gfx_window: &mut GfxWindow) -> Option<Box<Screen>>;
    fn process_event(&mut self, _event: &Event, _gfx_window: &mut GfxWindow) -> bool {
        false
    }
    fn mouse_click(&mut self, _position: Vector2<f32>) {}
    fn update(&mut self, dt: f32, mouse_position: Vector2<f32>, gfx_window: &mut GfxWindow)
        -> bool;
    fn window_resized(&mut self, gfx_window: &mut GfxWindow);
    fn render(&self, dt: f32, gfx_window: &mut GfxWindow) -> Result<(), Box<dyn Error>>;
}
