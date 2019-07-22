use chrono::offset::LocalResult;
use chrono::prelude::{DateTime, Local};
use chrono::TimeZone;
use std::fmt;
use std::time::{Duration, SystemTime};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TypingResult {
    pub correct_words: i32,
    pub incorrect_words: i32,
    pub backspaces: i32,
    pub wpm: i32,
    pub time: u64,
    pub notes: String,
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
            notes: String::new(),
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

impl fmt::Display for TypingResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = if let Some(local) = self.datetime() {
            format!("{}", local)
        } else {
            format!("NO DATETIME")
        };
        write!(
            f,
            "Result: [{}], {:3}wpm (correct words: {:3}, incorrect words: {:3}, backspaces: {:3})",
            datetime, self.wpm, self.correct_words, self.incorrect_words, self.backspaces
        )
    }
}
