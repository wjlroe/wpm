use crate::*;

pub struct ResultsScreen {
    pub typing_result: TypingResult,
}

impl ResultsScreen {
    pub fn new(typing_result: TypingResult) -> Self {
        Self { typing_result }
    }
}
