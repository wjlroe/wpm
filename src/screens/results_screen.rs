use crate::layout::ElementLayout;
use crate::screens;
use crate::*;
use cgmath::*;
use glutin::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use std::error::Error;

const NORMAL_LABEL_FONT_SIZE: f32 = 32.0;
const HEADLINE_LABEL_FONT_SIZE: f32 = 48.0;
const HEADLINE_VALUE_FONT_SIZE: f32 = 48.0;

#[derive(Default)]
pub struct ResultsScreen {
    typing_result: TypingResult,
    unsaved_result: bool,
    need_font_recalc: bool,
    go_back: bool,
    save_result: bool,
    wpm_label: Label,
    wpm_value: Label,
    correct_label: Label,
    correct_value: Label,
    incorrect_label: Label,
    incorrect_value: Label,
    backspaces_label: Label,
    backspaces_value: Label,
    notes_label: Label,
    notes_value: Label,
    back_label: Label,
    save_label: Label,
}

impl ResultsScreen {
    pub fn new(
        typing_result: TypingResult,
        unsaved_result: bool,
        gfx_window: &mut GfxWindow,
    ) -> Self {
        Self {
            typing_result: typing_result.clone(),
            unsaved_result,
            need_font_recalc: true,
            go_back: false,
            save_result: false,
            wpm_label: Label::new(
                HEADLINE_LABEL_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                TEXT_COLOR,
                String::from("Words per minute"),
                gfx_window,
            ),
            wpm_value: Label::new(
                HEADLINE_VALUE_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                TEXT_COLOR,
                format!("{}", typing_result.wpm),
                gfx_window,
            ),
            correct_label: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                TEXT_COLOR,
                String::from("Correct words"),
                gfx_window,
            ),
            correct_value: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                TEXT_COLOR,
                format!("{}", typing_result.correct_words),
                gfx_window,
            ),
            incorrect_label: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                TEXT_COLOR,
                String::from("Incorrect words"),
                gfx_window,
            ),
            incorrect_value: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                TEXT_COLOR,
                format!("{}", typing_result.incorrect_words),
                gfx_window,
            ),
            backspaces_label: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                TEXT_COLOR,
                String::from("Backspaces"),
                gfx_window,
            ),
            backspaces_value: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                TEXT_COLOR,
                format!("{}", typing_result.backspaces),
                gfx_window,
            ),
            notes_label: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                TEXT_COLOR,
                String::from("Notes"),
                gfx_window,
            ),
            notes_value: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                TEXT_COLOR,
                typing_result.notes.clone(),
                gfx_window,
            ),
            back_label: gfx_window.back_label(),
            save_label: Label::new(
                NORMAL_LABEL_FONT_SIZE,
                gfx_window.fonts.iosevka_font_id,
                TEXT_COLOR,
                String::from("Save"),
                gfx_window,
            ),
        }
    }

    fn type_char(&mut self, typed: char, gfx_window: &mut GfxWindow) {
        self.typing_result.notes.push(typed);
        self.notes_value
            .set_text(self.typing_result.notes.clone(), gfx_window);
        self.notes_value.recalc(gfx_window);
        self.unsaved_result = true;
    }

    fn type_backspace(&mut self, gfx_window: &mut GfxWindow) {
        let _ = self.typing_result.notes.pop();
        self.notes_value
            .set_text(self.typing_result.notes.clone(), gfx_window);
        self.notes_value.recalc(gfx_window);
        self.unsaved_result = true;
    }

    fn update_font_metrics(&mut self, gfx_window: &mut GfxWindow) {
        let longest_width_of_labels = [
            &self.wpm_label,
            &self.correct_label,
            &self.incorrect_label,
            &self.backspaces_label,
            &self.notes_label,
        ]
        .iter()
        .map(|label| label.rect.bounds.x)
        .max_by(|width_a, width_b| {
            width_a
                .partial_cmp(width_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(0.0);

        let longest_width_of_values = [
            &self.wpm_value,
            &self.correct_value,
            &self.incorrect_value,
            &self.backspaces_value,
            &self.notes_value,
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

        let mut notes_rect = Rect::default();
        notes_rect.bounds.y = f32::max(
            self.notes_label.rect.bounds.y,
            self.notes_value.rect.bounds.y,
        );
        notes_rect.bounds.x = line_width;

        let mut save_rect = self.save_label.rect;

        let padding_rect = vec2(line_width, 5.0);

        let mut vertical_layout = ElementLayout::vertical(gfx_window.window_dim());
        let result_rect_elem = vertical_layout.add_bounds(result_rect.bounds);
        let _ = vertical_layout.add_bounds(padding_rect);
        let correct_rect_elem = vertical_layout.add_bounds(correct_rect.bounds);
        let _ = vertical_layout.add_bounds(padding_rect);
        let incorrect_rect_elem = vertical_layout.add_bounds(incorrect_rect.bounds);
        let _ = vertical_layout.add_bounds(padding_rect);
        let backspaces_rect_elem = vertical_layout.add_bounds(backspaces_rect.bounds);
        let _ = vertical_layout.add_bounds(padding_rect);
        let notes_rect_elem = vertical_layout.add_bounds(notes_rect.bounds);
        let _ = vertical_layout.add_bounds(padding_rect);
        let save_rect_elem = vertical_layout.add_bounds(save_rect.bounds);
        vertical_layout.calc_positions();
        self.wpm_label.rect.position = vertical_layout.element_position(result_rect_elem);
        self.correct_label.rect.position = vertical_layout.element_position(correct_rect_elem);
        self.incorrect_label.rect.position = vertical_layout.element_position(incorrect_rect_elem);
        self.backspaces_label.rect.position =
            vertical_layout.element_position(backspaces_rect_elem);
        self.notes_label.rect.position = vertical_layout.element_position(notes_rect_elem);
        self.save_label.rect.position = vertical_layout.element_position(save_rect_elem);
        self.wpm_value.rect.position.y = self.wpm_label.rect.position.y;
        self.correct_value.rect.position.y = self.correct_label.rect.position.y;
        self.incorrect_value.rect.position.y = self.incorrect_label.rect.position.y;
        self.backspaces_value.rect.position.y = self.backspaces_label.rect.position.y;
        self.notes_value.rect.position.y = self.notes_label.rect.position.y;

        let mut horizontal_layout = ElementLayout::horizontal(gfx_window.window_dim());
        let result_rect_elem = horizontal_layout.add_bounds(result_rect.bounds);
        horizontal_layout.calc_positions();
        let left_margin = horizontal_layout.element_position(result_rect_elem).x;

        self.wpm_label.rect.position.x = left_margin;
        self.correct_label.rect.position.x = left_margin;
        self.incorrect_label.rect.position.x = left_margin;
        self.backspaces_label.rect.position.x = left_margin;
        self.notes_label.rect.position.x = left_margin;
        self.save_label.rect.position.x = left_margin;

        let vertical_padding = 15.0;

        self.wpm_value.rect.position.x = left_margin + vertical_padding + longest_width_of_labels;
        self.correct_value.rect.position.x =
            left_margin + vertical_padding + longest_width_of_labels;
        self.incorrect_value.rect.position.x =
            left_margin + vertical_padding + longest_width_of_labels;
        self.backspaces_value.rect.position.x =
            left_margin + vertical_padding + longest_width_of_labels;
        self.notes_value.rect.position.x = left_margin + vertical_padding + longest_width_of_labels;

        self.back_label.rect.position = vec2(20.0, 20.0);
    }
}

impl Screen for ResultsScreen {
    fn maybe_change_to_screen(
        &self,
        gfx_window: &mut GfxWindow,
        config: &Config,
    ) -> Option<Box<dyn Screen>> {
        if self.go_back {
            let screen = screens::TestScreen::new(gfx_window, config);
            Some(Box::new(screen))
        } else if self.save_result {
            match storage::save_result_to_file(&self.typing_result) {
                Err(error) => {
                    println!("Error saving results to file: {:?}", error);
                }
                _ => {}
            };
            Some(Box::new(ResultsListScreen::new(gfx_window)))
        } else {
            None
        }
    }

    fn mouse_click(&mut self, position: Vector2<f32>) {
        if self.back_label.rect.contains_point(position) {
            self.go_back = true;
        } else if self.save_label.rect.contains_point(position) {
            // TODO: We should only display and respond to this if record is unsaved/dirty
            self.save_result = true;
        }
    }

    fn process_event(&mut self, event: &Event, gfx_window: &mut GfxWindow) -> bool {
        let mut update_and_render = false;
        if let Event::WindowEvent {
            event: win_event, ..
        } = event
        {
            match win_event {
                WindowEvent::ReceivedCharacter(typed_char) if !typed_char.is_control() => {
                    self.type_char(*typed_char, gfx_window);
                    update_and_render = true;
                }
                WindowEvent::KeyboardInput {
                    input: keyboard_input,
                    ..
                } => match keyboard_input {
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Back),
                        state: ElementState::Released,
                        modifiers,
                        ..
                    } => {
                        if *modifiers == NO_MODS {
                            self.type_backspace(gfx_window);
                            update_and_render = true;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        };
        update_and_render
    }

    fn update(
        &mut self,
        _dt: f32,
        _mouse_position: Vector2<f32>,
        _config: &Config,
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

        let labels = [
            &self.back_label, // FIXME: Move to app-level navigation
            &self.wpm_label,
            &self.wpm_value,
            &self.correct_label,
            &self.correct_value,
            &self.incorrect_label,
            &self.incorrect_value,
            &self.backspaces_label,
            &self.backspaces_value,
            &self.notes_label,
            &self.notes_value,
        ];
        for label in labels.iter() {
            gfx_window.queue_label(label);
        }

        if self.unsaved_result {
            gfx_window.queue_label(&self.save_label);
        }

        gfx_window
            .glyph_brush
            .use_queue()
            .depth_target(&gfx_window.quad_bundle.data.out_depth)
            .draw(
                &mut gfx_window.encoder,
                &gfx_window.quad_bundle.data.out_color,
            )?;

        Ok(())
    }
}
