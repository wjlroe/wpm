mod animation;
pub mod app;
mod colours;
mod fonts;
mod gfx_window;
mod input;
mod label;
mod layout;
mod maths;
mod quad;
mod rect;
mod screen;
mod screens;
mod storage;
mod typing_result;
mod typing_state;
mod typing_test;
mod words;

pub use animation::Animation;
pub use app::App;
pub use colours::*;
pub use fonts::*;
pub use gfx_window::GfxWindow;
pub use input::*;
pub use label::Label;
pub use maths::*;
pub use quad::*;
pub use rect::Rect;
pub use screen::Screen;
pub use screens::*;
pub use typing_result::TypingResult;
pub use typing_state::TypingState;
pub use typing_test::{EnteredWord, TypingTest};

pub const UI_TEXT_BUTTON_SIZE: f32 = 68.0;
