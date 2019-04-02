use crate::TypingResult;
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug)]
pub enum EnteredWord {
    Correct,
    Incorrect,
}

#[derive(Clone, Debug, Default)]
pub struct TypingTest {
    pub words: Vec<String>,
    next_word: usize,
    pub words_entered: Vec<EnteredWord>,
    entered_text: String,
    pub backspaces: i32,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub duration: Option<Duration>,
    pub ended: bool,
}

impl TypingTest {
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

    pub fn remining_time_string(&self) -> Option<String> {
        self.time_left().map(|remaining| {
            let all_seconds = remaining.as_secs();
            let mins = all_seconds / 60;
            let seconds = all_seconds % 60;
            format!("{:0>2}:{:0>2}", mins, seconds)
        })
    }

    pub fn typed_char(&mut self, typed_char: char) {
        if !self.ended {
            self.entered_text.push(typed_char);
            self.update();
        }
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
                let assessment = if Some(entered_word)
                    == self.words.get(self.next_word).map(|word| word.as_str())
                {
                    EnteredWord::Correct
                } else {
                    EnteredWord::Incorrect
                };
                self.words_entered.push(assessment);
                self.entered_text.clear();
                self.next_word += 1;
            }
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
}
