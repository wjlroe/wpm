pub mod app;
mod colours;
mod layout;
mod typing_result;
mod typing_test;

pub use app::App;
pub use colours::*;
pub use typing_result::TypingResult;
pub use typing_test::{EnteredWord, TypingTest};
