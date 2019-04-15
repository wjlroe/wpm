use crate::layout::ElementLayout;
use crate::*;
use cgmath::*;
use gfx_glyph::{GlyphCruncher, HorizontalAlign, Layout, Scale, Section, VerticalAlign};
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

    fn update_font_metrics(&mut self, gfx_window: &mut GfxWindow) {
        let mut result_label_dim = vec2(0.0, 0.0);
        let mut result_dim = vec2(0.0, 0.0);

        let results_label_section = Section {
            font_id: gfx_window.fonts.roboto_font_id,
            scale: Scale::uniform((self.result_label_font_size * gfx_window.dpi) as f32),
            text: "AA",
            ..Section::default()
        };
        if let Some(dim) = gfx_window
            .glyph_brush
            .pixel_bounds(results_label_section)
            .map(|bounds| {
                let width = bounds.max.x - bounds.min.x;
                let height = bounds.max.y - bounds.min.y;
                vec2(width as f32 / 2.0, height as f32)
            })
        {
            result_label_dim = dim;
        }

        let results_section = Section {
            font_id: gfx_window.fonts.iosevka_font_id,
            scale: Scale::uniform((self.result_font_size * gfx_window.dpi) as f32),
            text: "000",
            ..Section::default()
        };
        if let Some(dim) = gfx_window
            .glyph_brush
            .pixel_bounds(results_section)
            .map(|bounds| {
                let width = bounds.max.x - bounds.min.x;
                let height = bounds.max.y - bounds.min.y;
                vec2(width as f32, height as f32)
            })
        {
            result_dim = dim;
        }

        self.result_label_pos_and_bounds.bounds =
            vec2(20.0 * result_label_dim.x, 1.5 * result_label_dim.y);
        self.result_pos_and_bounds.bounds = vec2(1.5 * result_dim.x, 1.5 * result_dim.y);

        let mut vertical_layout = ElementLayout::vertical(gfx_window.window_dim());
        let result_label_elem = vertical_layout.add_bounds(self.result_label_pos_and_bounds.bounds);
        vertical_layout.calc_positions();
        self.result_label_pos_and_bounds.position =
            vertical_layout.element_position(result_label_elem);

        self.result_pos_and_bounds.position.y = self.result_label_pos_and_bounds.position.y;
        self.result_pos_and_bounds.position.x = self.result_label_pos_and_bounds.position.x
            + self.result_label_pos_and_bounds.bounds.x
            + self.result_pos_and_bounds.bounds.x;
    }
}

impl Screen for ResultsScreen {
    fn maybe_change_to_screen(&self) -> Option<Box<Screen>> {
        None
    }

    fn process_events(&mut self, _dt: f32, _events: &[Event]) {}

    fn update(&mut self, _dt: f32, gfx_window: &mut GfxWindow) {
        // animate the WPM figure counting upwards
        if self.need_font_recalc {
            self.update_font_metrics(gfx_window);
            self.need_font_recalc = false;
        }
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
        let result_label = Section {
            text: "Words per minute:",
            color: BLACK,
            font_id: gfx_window.fonts.roboto_font_id,
            scale: Scale::uniform((self.result_label_font_size * gfx_window.dpi) as f32),
            bounds: self.result_label_pos_and_bounds.bounds.into(),
            screen_position: self.result_label_pos_and_bounds.position.into(),
            ..Section::default()
        };
        gfx_window.glyph_brush.queue(result_label);

        gfx_window.draw_quad(*RESULT_BG, &self.result_pos_and_bounds, 1.0);
        let result_text = format!("{}", self.typing_result.wpm);
        let result = Section {
            text: &result_text,
            color: BLACK,
            font_id: gfx_window.fonts.iosevka_font_id,
            scale: Scale::uniform((self.result_font_size * gfx_window.dpi) as f32),
            bounds: self.result_pos_and_bounds.bounds.into(),
            screen_position: self.result_pos_and_bounds.position.into(),
            ..Section::default()
        };
        gfx_window.glyph_brush.queue(result);

        gfx_window.glyph_brush.draw_queued(
            &mut gfx_window.encoder,
            &gfx_window.quad_bundle.data.out_color,
            &gfx_window.quad_bundle.data.out_depth,
        )?;

        Ok(())
    }
}
