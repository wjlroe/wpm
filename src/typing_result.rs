use std::time::Duration;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TypingResult {
    pub correct_words: i32,
    pub incorrect_words: i32,
    pub backspaces: i32,
    pub wpm: i32,
}

impl TypingResult {
    pub fn new(
        correct_words: i32,
        incorrect_words: i32,
        backspaces: i32,
        duration: Duration,
    ) -> Self {
        let wpm = (correct_words as f64 / (duration.as_secs() as f64 / 60.0)).floor() as i32;

        let typing_result = Self {
            correct_words,
            incorrect_words,
            backspaces,
            wpm,
        };

        typing_result
    }
}
