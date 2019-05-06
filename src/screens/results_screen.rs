use crate::layout::ElementLayout;
use crate::screens;
use crate::*;
use cgmath::*;
use std::error::Error;

const NORMAL_LABEL_FONT_SIZE: f32 = 32.0;
const HEADLINE_LABEL_FONT_SIZE: f32 = 48.0;
const HEADLINE_VALUE_FONT_SIZE: f32 = 48.0;

#[derive(Default)]
pub struct ResultsScreen {
    need_font_recalc: bool,
    go_back: bool,
    wpm_label: Label,
    wpm_value: Label,
    correct_label: Label,
    correct_value: Label,
    incorrect_label: Label,
    incorrect_value: Label,
    backspaces_label: Label,
    backspaces_value: Label,
    back_label: Label,
}

impl ResultsScreen {
    pub fn new(typing_result: TypingResult, gfx_window: &mut GfxWindow) -> Self {
        Self {
            need_font_recalc: true,
            go_back: false,
            wpm_label: Label::new(
                HEADLINE_LABEL_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                *TEXT_COLOR,
                String::from("Words per minute"),
                gfx_window,
            ),
            wpm_value: Label::new(
                HEADLINE_VALUE_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                *TEXT_COLOR,
                format!("{}", typing_result.wpm),
                gfx_window,
            ),
            correct_label: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                *TEXT_COLOR,
                String::from("Correct words"),
                gfx_window,
            ),
            correct_value: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                *TEXT_COLOR,
                format!("{}", typing_result.correct_words),
                gfx_window,
            ),
            incorrect_label: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                *TEXT_COLOR,
                String::from("Incorrect words"),
                gfx_window,
            ),
            incorrect_value: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                *TEXT_COLOR,
                format!("{}", typing_result.incorrect_words),
                gfx_window,
            ),
            backspaces_label: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                *TEXT_COLOR,
                String::from("Backspaces"),
                gfx_window,
            ),
            backspaces_value: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                *TEXT_COLOR,
                format!("{}", typing_result.backspaces),
                gfx_window,
            ),
            back_label: gfx_window.back_label(),
        }
    }

    fn update_font_metrics(&mut self, gfx_window: &mut GfxWindow) {
        let longest_width_of_labels = vec![
            &self.wpm_label,
            &self.correct_label,
            &self.incorrect_label,
            &self.backspaces_label,
        ]
        .iter()
        .map(|label| label.rect.bounds.x)
        .max_by(|width_a, width_b| {
            width_a
                .partial_cmp(width_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(0.0);

        let longest_width_of_values = vec![
            &self.wpm_value,
            &self.correct_value,
            &self.incorrect_value,
            &self.backspaces_value,
        ]
        .iter()
        .map(|label| label.rect.bounds.x)
        .max_by(|width_a, width_b| {
            width_a
                .partial_cmp(width_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(0.0);

        let line_width = longest_width_of_labels + longest_width_of_values;

        let mut result_rect = Rect::default();
        result_rect.bounds.y = f32::max(self.wpm_label.rect.bounds.y, self.wpm_value.rect.bounds.y);
        result_rect.bounds.x = line_width;

        let mut correct_rect = Rect::default();
        correct_rect.bounds.y = f32::max(
            self.correct_label.rect.bounds.y,
            self.correct_value.rect.bounds.y,
        );
        correct_rect.bounds.x = line_width;

        let mut incorrect_rect = Rect::default();
        incorrect_rect.bounds.y = f32::max(
            self.incorrect_label.rect.bounds.y,
            self.incorrect_value.rect.bounds.y,
        );
        incorrect_rect.bounds.x = line_width;

        let mut backspaces_rect = Rect::default();
        backspaces_rect.bounds.y = f32::max(
            self.backspaces_label.rect.bounds.y,
            self.backspaces_value.rect.bounds.y,
        );
        backspaces_rect.bounds.x = line_width;

        let padding_rect = vec2(line_width, 5.0);

        let mut vertical_layout = ElementLayout::vertical(gfx_window.window_dim());
        let result_rect_elem = vertical_layout.add_bounds(result_rect.bounds);
        let _ = vertical_layout.add_bounds(padding_rect);
        let correct_rect_elem = vertical_layout.add_bounds(correct_rect.bounds);
        let _ = vertical_layout.add_bounds(padding_rect);
        let incorrect_rect_elem = vertical_layout.add_bounds(incorrect_rect.bounds);
        let _ = vertical_layout.add_bounds(padding_rect);
        let backspaces_rect_elem = vertical_layout.add_bounds(backspaces_rect.bounds);
        vertical_layout.calc_positions();
        self.wpm_label.rect.position = vertical_layout.element_position(result_rect_elem);
        self.correct_label.rect.position = vertical_layout.element_position(correct_rect_elem);
        self.incorrect_label.rect.position = vertical_layout.element_position(incorrect_rect_elem);
        self.backspaces_label.rect.position =
            vertical_layout.element_position(backspaces_rect_elem);
        self.wpm_value.rect.position.y = self.wpm_label.rect.position.y;
        self.correct_value.rect.position.y = self.correct_label.rect.position.y;
        self.incorrect_value.rect.position.y = self.incorrect_label.rect.position.y;
        self.backspaces_value.rect.position.y = self.backspaces_label.rect.position.y;

        let mut horizontal_layout = ElementLayout::horizontal(gfx_window.window_dim());
        let result_rect_elem = horizontal_layout.add_bounds(result_rect.bounds);
        horizontal_layout.calc_positions();
        let left_margin = horizontal_layout.element_position(result_rect_elem).x;

        self.wpm_label.rect.position.x = left_margin;
        self.correct_label.rect.position.x = left_margin;
        self.incorrect_label.rect.position.x = left_margin;
        self.backspaces_label.rect.position.x = left_margin;

        let vertical_padding = 15.0;

        self.wpm_value.rect.position.x = left_margin + vertical_padding + longest_width_of_labels;
        self.correct_value.rect.position.x =
            left_margin + vertical_padding + longest_width_of_labels;
        self.incorrect_value.rect.position.x =
            left_margin + vertical_padding + longest_width_of_labels;
        self.backspaces_value.rect.position.x =
            left_margin + vertical_padding + longest_width_of_labels;

        self.back_label.rect.position = vec2(20.0, 20.0);
    }
}

impl Screen for ResultsScreen {
    fn maybe_change_to_screen(&self, gfx_window: &mut GfxWindow) -> Option<Box<Screen>> {
        if self.go_back {
            let screen = screens::TestScreen::new(gfx_window);
            Some(Box::new(screen))
        } else {
            None
        }
    }

    fn mouse_click(&mut self, position: Vector2<f32>) {
        if self.back_label.rect.contains_point(position) {
            self.go_back = true;
        }
    }

    fn update(
        &mut self,
        _dt: f32,
        _mouse_position: Vector2<f32>,
        gfx_window: &mut GfxWindow,
    ) -> bool {
        // animate the WPM figure counting upwards
        if self.need_font_recalc {
            self.update_font_metrics(gfx_window);
            self.need_font_recalc = false;
            true
        } else {
            false
        }
    }

    fn window_resized(&mut self, gfx_window: &mut GfxWindow) {
        self.update_font_metrics(gfx_window);
    }

    fn render(&self, _dt: f32, gfx_window: &mut GfxWindow) -> Result<(), Box<dyn Error>> {
        gfx_window
            .encoder
            .clear(&gfx_window.quad_bundle.data.out_color, bg_color());
        gfx_window
            .encoder
            .clear_depth(&gfx_window.quad_bundle.data.out_depth, 1.0);

        gfx_window.queue_ui_label(&self.back_label);

        let labels = vec![
            &self.wpm_label,
            &self.wpm_value,
            &self.correct_label,
            &self.correct_value,
            &self.incorrect_label,
            &self.incorrect_value,
            &self.backspaces_label,
            &self.backspaces_value,
        ];
        let mut quad_color = [1.0, 1.0, 1.0, 1.0];
        let mut outline_color = [128.0 / 256.0, 0.0, 128.0 / 156.0, 1.0];
        for label in labels {
            let mut section = label.section(gfx_window);
            section.bounds = label.rect.bounds.into();
            section.screen_position = label.rect.position.into();
            quad_color[0] -= 0.1;
            quad_color[1] -= 0.1;
            quad_color[2] -= 0.1;
            outline_color[0] -= 0.1;
            outline_color[2] -= 0.1;
            gfx_window.draw_quad(quad_color, &label.rect, 1.0);
            gfx_window.draw_outline(outline_color, &label.rect, 1.0 - 0.1, 3.0);
            gfx_window.glyph_brush.queue(section);
        }

        gfx_window.glyph_brush.draw_queued(
            &mut gfx_window.encoder,
            &gfx_window.quad_bundle.data.out_color,
            &gfx_window.quad_bundle.data.out_depth,
        )?;

        Ok(())
    }
}
