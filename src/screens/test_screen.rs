use crate::layout::ElementLayout;
use crate::*;
use cgmath::*;
use gfx_glyph::{
    GlyphCruncher, HorizontalAlign, Layout, PositionedGlyph, Scale, Section, VerticalAlign,
};
use glutin::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use std::error::Error;
use std::time::Duration;

const LISTING_BUTTON_FONT_SIZE: f32 = 68.0;
const INPUT_FONT_SIZE: f32 = 32.0;
const REFERENCE_FONT_SIZE: f32 = 32.0;
const TIMER_FONT_SIZE: f32 = 48.0;
const INPUT_CURSOR_COLOR: ColorArray = BLUE;
const REFERENCE_CURSOR_COLOR: ColorArray = BLUE;

#[derive(Default)]
pub struct TestScreen {
    need_font_recalc: bool,
    timer_pos_and_bounds: Rect,
    typing_pos_and_bounds: Rect,
    typing_mask_pos_and_bounds: Rect,
    input_label: Label,
    typing_test: TypingTest,
    typing_state: TypingState,
    show_listing_label: Label,
    goto_listing: bool,
    input_cursor: Rect,
    reference_cursor: Rect,
    input_cursor_size: Label,
    reference_cursor_size: Label,
}

impl TestScreen {
    pub fn new(gfx_window: &mut GfxWindow) -> Self {
        let mut show_listing_label = Label::new(
            LISTING_BUTTON_FONT_SIZE,
            gfx_window.fonts.iosevka_font_id,
            TEXT_COLOR,
            String::from("≡"),
            gfx_window,
        )
        .with_layout(
            Layout::default_single_line()
                .v_align(VerticalAlign::Center)
                .h_align(HorizontalAlign::Center),
        );
        show_listing_label.rect.bounds.x *= 1.5;
        let input_label = Label::new(
            INPUT_FONT_SIZE,
            gfx_window.fonts.roboto_font_id,
            CORRECT_WORD_COLOR,
            String::from(""),
            gfx_window,
        )
        .with_layout(Layout::default_single_line().v_align(VerticalAlign::Center));
        let input_cursor_size = Label::new(
            INPUT_FONT_SIZE,
            gfx_window.fonts.roboto_font_id,
            TEXT_COLOR,
            String::from("L"),
            gfx_window,
        )
        .with_layout(Layout::default_single_line().v_align(VerticalAlign::Center));
        let mut test_screen = Self {
            need_font_recalc: true,
            show_listing_label,
            input_label,
            input_cursor_size,
            reference_cursor_size: Label::new(
                REFERENCE_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                TEXT_COLOR,
                String::from("L"),
                gfx_window,
            ),
            ..TestScreen::default()
        };
        test_screen.start_test();
        test_screen
    }

    fn start_test(&mut self) {
        self.typing_test.top200();
        self.typing_test.duration = Some(Duration::from_secs(60));
    }

    fn recalc_cursors(&mut self, gfx_window: &mut GfxWindow) {
        // measure where the current cursor is and draw a rectangle around it
        // for input box
        if let Some(last_input_cursor_size_rect) =
            self.input_cursor_size.last_glyph_rect(gfx_window)
        {
            self.input_cursor = last_input_cursor_size_rect;
            self.input_cursor.position.y =
                last_input_cursor_size_rect.position.y - last_input_cursor_size_rect.bounds.y;
        }
        let left_padding = 5.0;
        if let Some(last_input_glyph_rect) = self.input_label.last_glyph_rect(gfx_window) {
            self.input_cursor.position.x = last_input_glyph_rect.right_edge() + left_padding;
        } else {
            self.input_cursor.position.x = self.input_cursor_size.rect.position.x + left_padding;
        }
        // and for reference text
    }

    fn reset_input_label_text(&mut self, gfx_window: &mut GfxWindow) {
        self.input_label
            .set_text(self.typing_test.entered_text.clone(), gfx_window);
        self.recalc_cursors(gfx_window);
    }

    fn type_char(&mut self, typed_char: char, gfx_window: &mut GfxWindow) {
        if self.typing_test.typed_char(typed_char) {
            self.typing_state.next_word();
        }
        self.reset_input_label_text(gfx_window);
    }

    fn type_backspace(&mut self, gfx_window: &mut GfxWindow) {
        self.typing_test.backspace();
        self.reset_input_label_text(gfx_window);
    }

    fn update_font_metrics(&mut self, gfx_window: &mut GfxWindow) {
        let left_and_top_padding = 15.0;

        self.show_listing_label.rect.position.x = left_and_top_padding;
        self.show_listing_label.rect.position.y = left_and_top_padding;

        let mut timer_character_dim = vec2(0.0, 0.0);
        let mut typing_character_dim = vec2(0.0, 0.0);

        let timer_section = Section {
            font_id: gfx_window.fonts.iosevka_font_id,
            scale: Scale::uniform(TIMER_FONT_SIZE * gfx_window.dpi as f32),
            text: "00:00",
            ..Section::default()
        };
        if let Some(dim) = gfx_window
            .glyph_brush
            .pixel_bounds(timer_section)
            .map(|bounds| {
                let width = bounds.max.x - bounds.min.x;
                let height = bounds.max.y - bounds.min.y;
                vec2(width as f32, height as f32)
            })
        {
            timer_character_dim = dim;
        }

        {
            let typed_section = Section {
                font_id: gfx_window.fonts.roboto_font_id,
                scale: Scale::uniform(REFERENCE_FONT_SIZE * gfx_window.dpi as f32),
                text: "AA",
                ..Section::default()
            };
            if let Some(dim) = gfx_window
                .glyph_brush
                .pixel_bounds(typed_section)
                .map(|bounds| {
                    let width = bounds.max.x - bounds.min.x;
                    let height = bounds.max.y - bounds.min.y;
                    vec2(width as f32, height as f32)
                })
            {
                typing_character_dim.x = dim.x / 2.0;
            }
        }

        {
            let typed_section = Section {
                font_id: gfx_window.fonts.roboto_font_id,
                scale: Scale::uniform(REFERENCE_FONT_SIZE * gfx_window.dpi as f32),
                text: "A\nA",
                ..Section::default()
            };
            if let Some(dim) = gfx_window
                .glyph_brush
                .pixel_bounds(typed_section)
                .map(|bounds| {
                    let width = bounds.max.x - bounds.min.x;
                    let height = bounds.max.y - bounds.min.y;
                    vec2(width as f32, height as f32)
                })
            {
                typing_character_dim.y = dim.y / 2.0;
            }
        }

        {
            let input_height = 1.5 * timer_character_dim.y;

            self.typing_pos_and_bounds.bounds =
                vec2(30.0 * typing_character_dim.x, 2.5 * typing_character_dim.y);

            let input_width = 30.0 * typing_character_dim.x;
            self.input_label.rect.bounds = vec2(input_width, input_height);
            self.input_cursor_size.rect.bounds.y = input_height;

            let mut vertical_layout = ElementLayout::vertical(gfx_window.window_dim());
            let typing_elem = vertical_layout.add_bounds(self.typing_pos_and_bounds.bounds);
            let input_elem = vertical_layout.add_bounds(self.input_label.rect.bounds);
            vertical_layout.calc_positions();
            self.typing_pos_and_bounds.position = vertical_layout.element_position(typing_elem);
            self.input_label.rect.position = vertical_layout.element_position(input_elem);

            ElementLayout::center_horizontally(
                gfx_window.window_dim(),
                self.typing_pos_and_bounds.bounds,
                &mut self.typing_pos_and_bounds.position,
            );
            ElementLayout::center_horizontally(
                gfx_window.window_dim(),
                self.typing_pos_and_bounds.bounds,
                &mut self.input_label.rect.position,
            );

            self.input_cursor_size.rect.position = self.input_label.rect.position;

            let timer_width = 1.1 * timer_character_dim.x;
            self.timer_pos_and_bounds.bounds = vec2(timer_width, input_height);
            self.timer_pos_and_bounds.position = vec2(
                self.typing_pos_and_bounds.position.x + self.typing_pos_and_bounds.bounds.x
                    - timer_width,
                self.input_label.rect.position.y,
            );

            self.input_label.rect.bounds.x -= self.timer_pos_and_bounds.bounds.x;

            self.typing_mask_pos_and_bounds = self.typing_pos_and_bounds;
            self.typing_mask_pos_and_bounds.position = self.typing_pos_and_bounds.position
                - vec2(0.0, self.typing_pos_and_bounds.bounds.y);
        }

        {
            // calculate by glyphs and detecting y differences...

            self.typing_state = TypingState::default();

            let bounds = vec2(self.typing_pos_and_bounds.bounds.x, 10000.0);
            let typed_section = Section {
                font_id: gfx_window.fonts.roboto_font_id,
                bounds: bounds.into(),
                scale: Scale::uniform(REFERENCE_FONT_SIZE * gfx_window.dpi as f32),
                text: &self.typing_test.words_str(),
                ..Section::default()
            };
            let mut glyph_iter = gfx_window.glyph_brush.glyphs(typed_section);
            let mut current_y = 0.0;
            if let Some(glyph_position) = glyph_iter.next().map(PositionedGlyph::position) {
                current_y = glyph_position.y;
            }

            let mut glyph_y = current_y;
            for (word_idx, word) in self.typing_test.words.iter().enumerate() {
                if word_idx > 0 {
                    // Get the first character/glyph for the word
                    if let Some(glyph_position) = glyph_iter.next().map(PositionedGlyph::position) {
                        glyph_y = glyph_position.y;
                    } else {
                        panic!("we are missing a glyph for this word!");
                    }
                }
                if (glyph_y - current_y).abs() >= std::f32::EPSILON {
                    self.typing_state.first_word_idx_per_line.push(word_idx);
                    if self.typing_state.per_line_height < 0.001 {
                        // TODO: if we calculate per_line_height here, we don't need to do that in the A\nA section above
                        self.typing_state.per_line_height = glyph_y - current_y;
                        // FIXME: these are different! 48.0 vs. 39.0
                        // assert_eq!(self.typing_state.per_line_height, typing_character_dim.y);
                    }
                    current_y = glyph_y;
                }
                let char_count = word.chars().count();
                // skip past all other characters in the word
                // this assumes 1 glyph per character
                // FIXME: for multi-lingual unicode support, we'll need to be cleverer about glyphs/chars
                for _ in 1..char_count {
                    let _ = glyph_iter.next().expect("shouldn't run out of glyphs");
                }
                self.typing_state.num_words += 1;
            }
        }

        // dbg!(self.input_label.rect);
        // dbg!(self.typing_mask_pos_and_bounds);

        self.recalc_cursors(gfx_window);
    }
}

impl Screen for TestScreen {
    fn maybe_change_to_screen(&self, gfx_window: &mut GfxWindow) -> Option<Box<dyn Screen>> {
        if self.goto_listing {
            Some(Box::new(ResultsListScreen::new(gfx_window)))
        } else if self.typing_test.ended {
            let typing_result = self.typing_test.result();
            match storage::save_result_to_file(&typing_result) {
                Err(error) => {
                    println!("Error saving results to file: {:?}", error);
                }
                _ => {}
            };
            Some(Box::new(ResultsScreen::new(typing_result, gfx_window)))
        } else {
            None
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

    fn mouse_click(&mut self, position: Vector2<f32>) {
        if self.show_listing_label.rect.contains_point(position) {
            self.goto_listing = true;
        }
    }

    fn window_resized(&mut self, gfx_window: &mut GfxWindow) {
        self.update_font_metrics(gfx_window);
    }

    fn update(
        &mut self,
        dt: f32,
        _mouse_position: Vector2<f32>,
        gfx_window: &mut GfxWindow,
    ) -> bool {
        let mut needs_render = if self.need_font_recalc {
            self.update_font_metrics(gfx_window);
            self.need_font_recalc = false;
            true
        } else {
            false
        };

        if !self.typing_test.ended {
            self.input_label.color = if self.typing_test.correct_so_far() {
                CORRECT_WORD_COLOR
            } else {
                INCORRECT_WORD_COLOR
            };
            needs_render = true;
            if let Some(true) = self.typing_test.is_done() {
                println!("Typing test is done!");
                self.typing_test.end();

                let typing_result = Some(self.typing_test.result());
                println!("Result: {:?}", typing_result);
            } else {
                self.typing_state.update(dt);
            }
        }

        needs_render
    }

    fn render(&self, _dt: f32, gfx_window: &mut GfxWindow) -> Result<(), Box<dyn Error>> {
        gfx_window
            .encoder
            .clear(&gfx_window.quad_bundle.data.out_color, bg_color());
        gfx_window
            .encoder
            .clear_depth(&gfx_window.quad_bundle.data.out_depth, 1.0);

        gfx_window.draw_outline(INPUT_OUTLINE_COLOR, &self.input_label.rect, 0.8, 3.0);
        gfx_window.draw_quad(TRANSPARENT, &self.typing_mask_pos_and_bounds, 0.5);
        gfx_window.draw_outline(TIMER_OUTLINE_COLOR, &self.timer_pos_and_bounds, 1.0, 3.0);

        // TODO: change cursor color for incorrectly typed characters
        gfx_window.draw_quad(INPUT_CURSOR_COLOR, &self.input_cursor, 0.5);
        gfx_window.draw_quad(REFERENCE_CURSOR_COLOR, &self.reference_cursor, 0.5);

        let skip_num = self.typing_state.skip_num();
        let typed_section = self.typing_test.words_as_varied_section(
            skip_num,
            self.typing_pos_and_bounds.bounds + vec2(0.0, self.typing_state.offset()),
            self.typing_pos_and_bounds.position,
            REFERENCE_FONT_SIZE * gfx_window.dpi as f32,
            gfx_window.fonts.roboto_font_id,
        );
        gfx_window.glyph_brush.queue(typed_section);

        let window_dim = gfx_window.window_dim();
        gfx_window
            .glyph_brush
            .use_queue()
            .transform(text_transform(
                self.typing_state.transform(window_dim),
                window_dim,
            ))
            .depth_target(&gfx_window.quad_bundle.data.out_depth)
            .draw(
                &mut gfx_window.encoder,
                &gfx_window.quad_bundle.data.out_color,
            )?;

        // FIXME: change render color based on whether word is correct or not!
        gfx_window.queue_label(&self.input_label);

        // Render clock countdown timer
        if let Some(time_remaining) = self.typing_test.remaining_time_string() {
            let layout = Layout::default_single_line()
                .h_align(HorizontalAlign::Center)
                .v_align(VerticalAlign::Center);
            let time_section = Section {
                text: &time_remaining,
                font_id: gfx_window.fonts.iosevka_font_id,
                scale: Scale::uniform(TIMER_FONT_SIZE * gfx_window.dpi as f32),
                bounds: self.timer_pos_and_bounds.bounds.into(),
                screen_position: self.timer_pos_and_bounds.center_point().into(),
                color: TIMER_COLOR,
                layout,
                ..Section::default()
            };
            gfx_window.glyph_brush.queue(time_section);
        }

        gfx_window.queue_label(&self.show_listing_label);

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
