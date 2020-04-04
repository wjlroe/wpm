use std::cell::RefCell;

#[derive(Copy, Clone, PartialEq)]
pub enum BackgroundColor {
    Light,
    Dark,
}

const fn default_bg_color() -> BackgroundColor {
    BackgroundColor::Light
}

impl Default for BackgroundColor {
    fn default() -> Self {
        default_bg_color()
    }
}

pub type ColorArray = [f32; 4];

macro_rules! color_array_from_rgb {
    ($red:expr, $green:expr, $blue:expr) => {
        [
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
            1.0,
        ]
    };
}

pub const BASE03: ColorArray = color_array_from_rgb!(0, 43, 54);
pub const BASE02: ColorArray = color_array_from_rgb!(7, 54, 66);
pub const BASE01: ColorArray = color_array_from_rgb!(88, 110, 117);
pub const BASE00: ColorArray = color_array_from_rgb!(101, 123, 131);
pub const BASE0: ColorArray = color_array_from_rgb!(131, 148, 150);
pub const BASE1: ColorArray = color_array_from_rgb!(147, 161, 161);
pub const BASE2: ColorArray = color_array_from_rgb!(238, 232, 213);
pub const BASE3: ColorArray = color_array_from_rgb!(253, 246, 227);
pub const YELLOW: ColorArray = color_array_from_rgb!(181, 137, 0);
pub const ORANGE: ColorArray = color_array_from_rgb!(203, 75, 22);
pub const RED: ColorArray = color_array_from_rgb!(220, 50, 47);
pub const MAGENTA: ColorArray = color_array_from_rgb!(211, 54, 130);
pub const VIOLET: ColorArray = color_array_from_rgb!(108, 113, 196);
pub const BLUE: ColorArray = color_array_from_rgb!(38, 139, 210);
pub const CYAN: ColorArray = color_array_from_rgb!(42, 161, 152);
pub const GREEN: ColorArray = color_array_from_rgb!(133, 153, 0);

pub const TEXT_COLOR: ColorArray = BASE01;
pub const CORRECT_WORD_COLOR: ColorArray = GREEN;
pub const INCORRECT_WORD_COLOR: ColorArray = RED;
pub const NEXT_WORD_COLOR: ColorArray = BLUE;
pub const INPUT_OUTLINE_COLOR: ColorArray = MAGENTA;
pub const TIMER_OUTLINE_COLOR: ColorArray = CYAN;
pub const TIMER_COLOR: ColorArray = VIOLET;

pub const LIGHT_BG_COLOR: ColorArray = BASE3;
pub const DARK_BG_COLOR: ColorArray = BASE03;

thread_local! {
    pub static CURRENT_BG_COLOR: RefCell<BackgroundColor> = RefCell::new(default_bg_color());
    pub static BG_COLOR: RefCell<ColorArray> = RefCell::new(LIGHT_BG_COLOR);
}

pub fn swap_colors() {
    let (new_bg_color, new_current_bg_color) =
        CURRENT_BG_COLOR.with(|current_bg_color| match *current_bg_color.borrow() {
            BackgroundColor::Light => (DARK_BG_COLOR, BackgroundColor::Dark),
            BackgroundColor::Dark => (LIGHT_BG_COLOR, BackgroundColor::Light),
        });
    BG_COLOR.with(|bg_color| *bg_color.borrow_mut() = new_bg_color);
    CURRENT_BG_COLOR.with(|current_bg_color| *current_bg_color.borrow_mut() = new_current_bg_color);
}

pub fn bg_color() -> ColorArray {
    BG_COLOR.with(|bg_color| *bg_color.borrow())
}

pub fn current_bg_color() -> BackgroundColor {
    CURRENT_BG_COLOR.with(|current_bg_color| *current_bg_color.borrow())
}

pub const TRANSPARENT: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
