use crate::words;
use crate::*;
use cgmath::Vector2;
use rand;
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};
use wgpu_glyph::{FontId, OwnedSectionText, OwnedVariedSection, Scale};

const SAMPLE_WORDS: usize = 300; // num of words to sample - should be less than highest WPM

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EnteredWord {
    Correct,
    Incorrect,
}

#[derive(Clone, Debug, Default)]
pub struct TypingTest {
    pub words: Vec<String>,
    next_word: usize,
    pub words_entered: Vec<EnteredWord>,
    pub entered_text: String,
    pub backspaces: i32,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub duration: Option<Duration>,
    pub ended: bool,
    pub word_colors: Vec<ColorArray>,
    skip_num: usize,
}

impl TypingTest {
    pub fn has_started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn is_done(&self) -> Option<bool> {
        if let Some(duration) = self.duration {
            if let Some(start_time) = self.start_time {
                let elapsed = start_time.elapsed();
                if elapsed >= duration {
                    return Some(true);
                } else {
                    return Some(false);
                }
            }
        }
        None
    }

    fn time_left(&self) -> Option<Duration> {
        if let Some(false) = self.is_done() {
            let elapsed = self.start_time.unwrap().elapsed();
            Some(self.duration.unwrap() - elapsed)
        } else {
            None
        }
    }

    pub fn remaining_time_string(&self) -> Option<String> {
        self.time_left().map(|remaining| {
            let all_seconds = remaining.as_secs();
            let mins = all_seconds / 60;
            let seconds = all_seconds % 60;
            format!("{:0>2}:{:0>2}", mins, seconds)
        })
    }

    pub fn typed_char(&mut self, typed_char: char) -> bool {
        let mut word_ended = false;
        if !self.ended {
            self.entered_text.push(typed_char);
            let num_words = self.words_entered.len();
            self.update();
            if self.words_entered.len() > num_words {
                word_ended = true;
            }
        }
        word_ended
    }

    pub fn backspace(&mut self) {
        if !self.ended && self.entered_text.pop().is_some() {
            self.backspaces += 1;
            self.update();
        }
    }

    fn update(&mut self) {
        if !self.ended && self.start_time.is_none() {
            self.start();
        }

        if !self.ended {
            self.update_words();
        }
    }

    fn update_words(&mut self) {
        if self.entered_text.ends_with(' ') || Some(true) == self.is_done() {
            // just entered a space
            let entered_word = self.entered_text.trim();
            if !entered_word.is_empty() {
                let assessment =
                    if Some(entered_word) == self.words.get(self.next_word).map(String::as_str) {
                        EnteredWord::Correct
                    } else {
                        EnteredWord::Incorrect
                    };
                self.words_entered.push(assessment);
                self.word_colors[self.next_word] = if assessment == EnteredWord::Correct {
                    CORRECT_WORD_COLOR
                } else {
                    INCORRECT_WORD_COLOR
                };
                if let Some(word_color) = self.word_colors.get_mut(self.next_word + 1) {
                    *word_color = NEXT_WORD_COLOR;
                }
                self.entered_text.clear();
                self.next_word += 1;
            }
        }
    }

    pub fn correct_so_far(&self) -> bool {
        if let Some(next_word) = self.words.get(self.next_word) {
            if self.entered_text.len() > next_word.len() {
                return false;
            }
            let relevant_next_word_chars = &next_word[..self.entered_text.len()];
            relevant_next_word_chars == self.entered_text
        } else {
            false
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn end(&mut self) {
        self.update();
        self.end_time = Some(Instant::now());
        self.ended = true;
    }

    pub fn result(&self) -> TypingResult {
        let mut correct_words = 0;
        let mut incorrect_words = 0;
        for word in &self.words_entered {
            match word {
                EnteredWord::Correct => correct_words += 1,
                EnteredWord::Incorrect => incorrect_words += 1,
            };
        }
        TypingResult::new(
            correct_words,
            incorrect_words,
            self.backspaces,
            self.duration.unwrap(),
        )
    }

    pub fn set_words(&mut self, words: Vec<String>) {
        let num_words = words.len();
        self.words = words;
        self.word_colors = vec![TEXT_COLOR; num_words];
    }

    pub fn set_skip_num(&mut self, skip_num: usize) {
        self.skip_num = skip_num;
    }

    pub fn words_str(&self) -> String {
        self.words.join(" ")
    }

    pub fn words_as_sections(&self, font_id: FontId, scale: f32) -> Vec<OwnedSectionText> {
        let mut sections = vec![];
        for (word_idx, word) in self.words.iter().enumerate().skip(self.skip_num) {
            let color = self.word_colors[word_idx];
            sections.push(OwnedSectionText {
                text: word.to_owned(),
                color,
                font_id,
                scale: Scale::uniform(scale),
            });
            sections.push(OwnedSectionText {
                text: String::from(" "),
                color: TEXT_COLOR,
                font_id,
                scale: Scale::uniform(scale),
                ..OwnedSectionText::default()
            });
        }
        sections
    }

    pub fn words_as_varied_section(
        &self,
        bounds: Vector2<f32>,
        position: Vector2<f32>,
        font_scale: f32,
        font_id: FontId,
    ) -> OwnedVariedSection {
        let sections = self.words_as_sections(font_id, font_scale);
        OwnedVariedSection {
            text: sections,
            bounds: bounds.into(),
            screen_position: position.into(),
            z: 1.0,
            ..OwnedVariedSection::default()
        }
    }

    pub fn top200(&mut self) {
        let mut rng = &mut rand::thread_rng();
        let sample = words::top_200::words();
        let test_words = sample
            .choose_multiple(&mut rng, SAMPLE_WORDS)
            .cloned()
            .collect();
        self.set_words(test_words);
    }
}
