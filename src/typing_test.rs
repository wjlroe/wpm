use std::time::{Duration, Instant};

#[derive(Clone, Debug, Default)]
pub struct TypingTest {
    pub words: Vec<String>,
    pub entered_text: String,
    pub backspaces: i32,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    pub duration: Option<Duration>,
    ended: bool,
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
        if !self.ended {
            if let Some(_) = self.entered_text.pop() {
                self.backspaces += 1;
                self.update();
            }
        }
    }

    fn update(&mut self) {
        if !self.ended && self.start_time.is_none() {
            self.start();
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn end(&mut self) {
        self.end_time = Some(Instant::now());
        self.ended = true;
    }
}
