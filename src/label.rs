use crate::*;
use cgmath::*;
use gfx_glyph::{
    BuiltInLineBreaker, FontId, GlyphCruncher, HorizontalAlign, Layout, OwnedSectionText,
    OwnedVariedSection, Scale, Section, VerticalAlign,
};

#[derive(Default)]
pub struct Label {
    pub font_size: f32,
    pub font_id: FontId,
    pub color: ColorArray,
    pub rect: Rect,
    pub text: String,
    pub layout: Layout<BuiltInLineBreaker>,
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
            layout: Layout::default(),
            ..Label::default()
        };
        label.recalc(gfx_window);
        label
    }

    fn section_without_bounds_or_position(&self, gfx_window: &mut GfxWindow) -> Section {
        Section {
            font_id: self.font_id,
            color: self.color,
            scale: Scale::uniform(self.font_size * gfx_window.dpi as f32),
            text: &self.text,
            layout: self.layout,
            ..Section::default()
        }
    }

    pub fn section(&self, gfx_window: &mut GfxWindow) -> Section {
        let mut section = self.section_without_bounds_or_position(gfx_window);
        section.bounds = self.rect.bounds.into();
        section.screen_position = self.screen_position().into();
        section
    }

    pub fn varied_section(&self, sections: Vec<OwnedSectionText>) -> OwnedVariedSection {
        OwnedVariedSection {
            text: sections,
            bounds: self.rect.bounds.into(),
            screen_position: self.screen_position().into(),
            ..OwnedVariedSection::default()
        }
    }

    pub fn recalc(&mut self, gfx_window: &mut GfxWindow) {
        let section = self.section_without_bounds_or_position(gfx_window);
        if let Some(dim) = gfx_window.glyph_brush.glyph_bounds(section).map(|bounds| {
            let width = bounds.max.x;
            let height = bounds.max.y;
            vec2(width, height)
        }) {
            self.rect.bounds = dim;
        }
    }

    pub fn last_glyph_rect(&self, gfx_window: &mut GfxWindow) -> Option<Rect> {
        let section = self.section(gfx_window);
        gfx_window
            .glyph_brush
            .glyphs(section)
            .last()
            .and_then(|positioned_glyph| {
                if let Some(old_rect) = positioned_glyph.pixel_bounding_box() {
                    let mut rect = Rect::default();
                    let width = old_rect.max.x - old_rect.min.x;
                    let height = old_rect.max.y - old_rect.min.y;
                    // This is very tight to the glyph, so tiny for 'n' and tall for 'j' etc.
                    rect.bounds = vec2(width as f32, height as f32);
                    let pos = positioned_glyph.position();
                    rect.position = vec2(pos.x, pos.y);
                    Some(rect)
                } else {
                    None
                }
            })
    }

    pub fn with_layout(mut self, layout: Layout<BuiltInLineBreaker>) -> Self {
        self.layout = layout;
        self
    }

    pub fn set_text(&mut self, text: String, _gfx_window: &mut GfxWindow) {
        self.text = text;
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn screen_position(&self) -> Vector2<f32> {
        match self.layout {
            Layout::SingleLine {
                v_align: VerticalAlign::Center,
                h_align: HorizontalAlign::Center,
                ..
            } => self.rect.center_point(),
            Layout::SingleLine {
                v_align: VerticalAlign::Center,
                ..
            } => self.rect.center_y(),
            Layout::SingleLine {
                h_align: HorizontalAlign::Center,
                ..
            } => self.rect.center_x(),
            _ => self.rect.position,
        }
    }
}
