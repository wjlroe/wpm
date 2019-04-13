use crate::*;
use glutin::*;
use lazy_static::*;
use std::error::Error;

lazy_static! {
    static ref RESULT_BG: [f32; 4] = SOLARIZED_COLOR_MAP
        .get(&SolarizedColor::Magenta)
        .cloned()
        .unwrap();
}

#[derive(Default)]
pub struct ResultsScreen {
    need_font_recalc: bool,
    result_font_size: f64,
    result_pos_and_bounds: Rect,
    result_label_font_size: f64,
    result_label_pos_and_bounds: Rect,
    typing_result: TypingResult,
}

impl ResultsScreen {
    pub fn new(typing_result: TypingResult) -> Self {
        Self {
            need_font_recalc: true,
            typing_result,
            result_font_size: 48.0,
            result_label_font_size: 40.0,
            ..ResultsScreen::default()
        }
    }
}

impl Screen for ResultsScreen {
    fn maybe_change_to_screen(&self) -> Option<Box<Screen>> {
        None
    }

    fn process_events(&mut self, _dt: f32, _events: &[Event]) {}

    fn update(&mut self, _dt: f32, _gfx_window: &mut GfxWindow) {
        // animate the WPM figure counting upwards
    }

    fn window_resized(&mut self, _gfx_window: &mut GfxWindow) {}

    fn render(&self, _dt: f32, gfx_window: &mut GfxWindow) -> Result<(), Box<dyn Error>> {
        gfx_window
            .encoder
            .clear(&gfx_window.quad_bundle.data.out_color, BG_COLOR);
        gfx_window
            .encoder
            .clear_depth(&gfx_window.quad_bundle.data.out_depth, 1.0);

        gfx_window.draw_quad(TYPING_BG, &self.result_label_pos_and_bounds, 1.0);
        gfx_window.draw_quad(*RESULT_BG, &self.result_pos_and_bounds, 1.0);

        Ok(())
    }
}
