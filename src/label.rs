use crate::*;
use cgmath::*;
use gfx_glyph::{FontId, GlyphCruncher, Scale, Section};

#[derive(Default)]
pub struct Label {
    pub font_size: f32,
    pub font_id: FontId,
    pub color: ColorArray,
    pub rect: Rect,
    pub text: String,
}

impl Label {
    pub fn new(
        font_size: f32,
        font_id: FontId,
        color: ColorArray,
        text: String,
        gfx_window: &mut GfxWindow,
    ) -> Self {
        let mut label = Self {
            font_size,
            font_id,
            color,
            text,
            ..Label::default()
        };
        label.recalc(gfx_window);
        label
    }

    // FIXME: rename this section_without_bounds_or_position() because I forget this is that!
    pub fn section(&self, gfx_window: &mut GfxWindow) -> Section {
        Section {
            font_id: self.font_id,
            color: self.color,
            scale: Scale::uniform(self.font_size * gfx_window.dpi as f32),
            text: &self.text,
            ..Section::default()
        }
    }

    pub fn recalc(&mut self, gfx_window: &mut GfxWindow) {
        let section = self.section(gfx_window);
        if let Some(dim) = gfx_window.glyph_brush.pixel_bounds(section).map(|bounds| {
            let width = bounds.max.x;
            let height = bounds.max.y;
            vec2(width as f32, height as f32)
        }) {
            self.rect.bounds = dim;
        }
    }
}
