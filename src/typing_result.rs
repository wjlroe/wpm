use chrono::offset::LocalResult;
use chrono::prelude::{DateTime, Local};
use chrono::TimeZone;
use std::time::{Duration, SystemTime};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TypingResult {
    pub correct_words: i32,
    pub incorrect_words: i32,
    pub backspaces: i32,
    pub wpm: i32,
    pub time: u64,
}

impl TypingResult {
    pub fn new(
        correct_words: i32,
        incorrect_words: i32,
        backspaces: i32,
        duration: Duration,
    ) -> Self {
        let wpm = (f64::from(correct_words) / (duration.as_secs() as f64 / 60.0)).floor() as i32;
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime to work!")
            .as_secs();

        Self {
            correct_words,
            incorrect_words,
            backspaces,
            wpm,
            time,
        }
    }

    pub fn datetime(&self) -> Option<DateTime<Local>> {
        if self.time == 0 {
            return None;
        }
        match Local.timestamp_opt(self.time as i64, 0) {
            LocalResult::Single(value) => Some(value),
            _ => None,
        }
    }
}
