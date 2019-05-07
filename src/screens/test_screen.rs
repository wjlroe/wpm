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

#[derive(Default)]
pub struct TestScreen {
    need_font_recalc: bool,
    timer_font_size: f64,
    timer_pos_and_bounds: Rect,
    typing_font_size: f64,
    typing_pos_and_bounds: Rect,
    typing_mask_pos_and_bounds: Rect,
    input_pos_and_bounds: Rect,
    typing_test: TypingTest,
    typing_state: TypingState,
    show_listing_label: Label,
    goto_listing: bool,
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
        let mut test_screen = Self {
            need_font_recalc: true,
            timer_font_size: 48.0,
            typing_font_size: 32.0,
            show_listing_label,
            ..TestScreen::default()
        };
        test_screen.start_test();
        test_screen
    }

    fn start_test(&mut self) {
        // FIXME: move into TypingTest and generate from list of common words
        let wordlist = "also|sentence|stop|she|men|see|been|from|we|follow|but|mother|too|form|this|went|to|then|show|have|only|now|around|help|family|old|write|grow|also|over|together|city|end|quite|with|might|eat|four|where|hard|their|take|year|see|place|leave|too|too|is|other|near|around|saw|did|into|question|work|between|your|face|without|tree|as|girl|if|enough|stop|still|put|on|side|there|hear|large|more|be|there|took|some|into|off|down|so|is|tell|way|large|thing|earth|move|their|much|list|small|family|know|under|try|mean|above|end|was|what|night|them|most|good|example|left|mile|that|why|give|because|sea|above|boy|has|go|book|later|eat|land|about|line|life|said|often|really|the|at|without|large|should|away|end|no|oil|any|while|being|before|away|from|light|found|other|open|below|sound|began|come|night|year|world|start|that|it|after|and|show|every|find|old|while|school|your|point|often|example|children|up|found|then|quickly|some|still|again|our|world|may|group|help|point|own|around|make|than|look|girl|sometimes|hand|idea|change|people|get|page|the|own|it's|land|play|last|kind|eye|once|write|you|are|young|take|found|up|once|white|thought|answer|next|still|hand|state|air|food|don't|story|say|of|they|through|keep|far|should|different|eye|been|such|few|through|close|before|below|question|word|and|mother|along|number|miss|sound|her|boy|soon|car|seem|make|food|left|call|where|after|did|answer|write|there|got|mile|line|number|feet|America|earth|it's|find|get|me|home|cut|say|again|home|play|light|give|my|most|will|went|turn|sound|name|could|let|almost|head|carry|look|work|turn|letter|come|new|spell|mountain|move|children|air|live|this|hear|or|every|these|song|can|move|watch|which|picture|own|was|right|does|need|important|river|some|had|after|or|man|study|should|part|would|and|by|watch|earth|head";
        let words = wordlist
            .split('|')
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        self.typing_test.words = words;
        self.typing_test.duration = Some(Duration::from_secs(60));
    }

    fn type_char(&mut self, typed_char: char) {
        if self.typing_test.typed_char(typed_char) {
            self.typing_state.next_word();
        }
    }

    fn type_backspace(&mut self) {
        self.typing_test.backspace();
    }

    fn update_font_metrics(&mut self, gfx_window: &mut GfxWindow) {
        let left_and_top_padding = 15.0;

        self.show_listing_label.rect.position.x = left_and_top_padding;
        self.show_listing_label.rect.position.y = left_and_top_padding;

        let mut timer_character_dim = vec2(0.0, 0.0);
        let mut typing_character_dim = vec2(0.0, 0.0);

        let timer_section = Section {
            font_id: gfx_window.fonts.iosevka_font_id,
            scale: Scale::uniform((self.timer_font_size * gfx_window.dpi) as f32),
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
                scale: Scale::uniform((self.typing_font_size * gfx_window.dpi) as f32),
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
                scale: Scale::uniform((self.typing_font_size * gfx_window.dpi) as f32),
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

            self.input_pos_and_bounds.bounds = vec2(30.0 * typing_character_dim.x, input_height);

            let mut vertical_layout = ElementLayout::vertical(gfx_window.window_dim());
            let typing_elem = vertical_layout.add_bounds(self.typing_pos_and_bounds.bounds);
            let input_elem = vertical_layout.add_bounds(self.input_pos_and_bounds.bounds);
            vertical_layout.calc_positions();
            self.typing_pos_and_bounds.position = vertical_layout.element_position(typing_elem);
            self.input_pos_and_bounds.position = vertical_layout.element_position(input_elem);

            ElementLayout::center_horizontally(
                gfx_window.window_dim(),
                self.typing_pos_and_bounds.bounds,
                &mut self.typing_pos_and_bounds.position,
            );
            ElementLayout::center_horizontally(
                gfx_window.window_dim(),
                self.typing_pos_and_bounds.bounds,
                &mut self.input_pos_and_bounds.position,
            );

            let timer_width = 1.1 * timer_character_dim.x;
            self.timer_pos_and_bounds.bounds = vec2(timer_width, input_height);
            self.timer_pos_and_bounds.position = vec2(
                self.typing_pos_and_bounds.position.x + self.typing_pos_and_bounds.bounds.x
                    - timer_width,
                self.input_pos_and_bounds.position.y,
            );

            self.input_pos_and_bounds.bounds.x -= self.timer_pos_and_bounds.bounds.x;

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
                scale: Scale::uniform((self.typing_font_size * gfx_window.dpi) as f32),
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
    }
}

impl Screen for TestScreen {
    fn maybe_change_to_screen(&self, gfx_window: &mut GfxWindow) -> Option<Box<Screen>> {
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

    fn process_events(&mut self, _dt: f32, events: &[Event]) {
        for event in events.iter() {
            if let Event::WindowEvent {
                event: win_event, ..
            } = event
            {
                match win_event {
                    WindowEvent::ReceivedCharacter(typed_char) if !typed_char.is_control() => {
                        self.type_char(*typed_char);
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
                                self.type_backspace();
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
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

        gfx_window.draw_outline(INPUT_OUTLINE_COLOR, &self.input_pos_and_bounds, 0.8, 3.0);
        gfx_window.draw_quad(TRANSPARENT, &self.typing_mask_pos_and_bounds, 0.5);
        gfx_window.draw_outline(TIMER_OUTLINE_COLOR, &self.timer_pos_and_bounds, 1.0, 3.0);

        let skip_num = self.typing_state.skip_num();
        let typed_section = self.typing_test.words_as_varied_section(
            skip_num,
            self.typing_pos_and_bounds.bounds + vec2(0.0, self.typing_state.offset()),
            self.typing_pos_and_bounds.position,
            (self.typing_font_size * gfx_window.dpi) as f32,
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

        let input_layout = Layout::default_single_line().v_align(VerticalAlign::Center);
        let input_section = Section {
            text: &self.typing_test.entered_text,
            color: CORRECT_WORD_COLOR,
            font_id: gfx_window.fonts.roboto_font_id,
            scale: Scale::uniform((self.typing_font_size * gfx_window.dpi) as f32),
            bounds: self.input_pos_and_bounds.bounds.into(),
            screen_position: self.input_pos_and_bounds.center_y().into(),
            layout: input_layout,
            ..Section::default()
        };
        gfx_window.glyph_brush.queue(input_section);

        // Render clock countdown timer
        if let Some(time_remaining) = self.typing_test.remaining_time_string() {
            let layout = Layout::default_single_line()
                .h_align(HorizontalAlign::Center)
                .v_align(VerticalAlign::Center);
            let time_section = Section {
                text: &time_remaining,
                font_id: gfx_window.fonts.iosevka_font_id,
                scale: Scale::uniform((self.timer_font_size * gfx_window.dpi) as f32),
                bounds: self.timer_pos_and_bounds.bounds.into(),
                screen_position: self.timer_pos_and_bounds.center_point().into(),
                color: TIMER_COLOR,
                layout,
                ..Section::default()
            };
            gfx_window.glyph_brush.queue(time_section);
        }

        let listing_button_section = self.show_listing_label.section(gfx_window);
        gfx_window.glyph_brush.queue(listing_button_section);

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
