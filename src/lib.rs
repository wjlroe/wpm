pub mod app;
mod colours;
mod layout;
mod maths;
mod typing_result;
mod typing_test;

pub use app::App;
pub use colours::*;
pub use maths::*;
pub use typing_result::TypingResult;
pub use typing_test::{EnteredWord, TypingTest};
