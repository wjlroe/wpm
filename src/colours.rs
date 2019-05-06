use lazy_static::*;
use std::cell::RefCell;
use std::collections::HashMap;

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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum SolarizedColor {
    Base03,
    Base02,
    Base01,
    Base00,
    Base0,
    Base1,
    Base2,
    Base3,
    Yellow,
    Orange,
    Red,
    Magenta,
    Violet,
    Blue,
    Cyan,
    Green,
}

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

lazy_static! {
    pub static ref SOLARIZED_COLOR_MAP: HashMap<SolarizedColor, ColorArray> = {
        let triples = [
            (SolarizedColor::Base03, (0, 43, 54)),
            (SolarizedColor::Base02, (7, 54, 66)),
            (SolarizedColor::Base01, (88, 110, 117)),
            (SolarizedColor::Base00, (101, 123, 131)),
            (SolarizedColor::Base0, (131, 148, 150)),
            (SolarizedColor::Base1, (147, 161, 161)),
            (SolarizedColor::Base2, (238, 232, 213)),
            (SolarizedColor::Base3, (253, 246, 227)),
            (SolarizedColor::Yellow, (181, 137, 0)),
            (SolarizedColor::Orange, (203, 75, 22)),
            (SolarizedColor::Red, (220, 50, 47)),
            (SolarizedColor::Magenta, (211, 54, 130)),
            (SolarizedColor::Violet, (108, 113, 196)),
            (SolarizedColor::Blue, (38, 139, 210)),
            (SolarizedColor::Cyan, (42, 161, 152)),
            (SolarizedColor::Green, (133, 153, 0)),
        ];

        let mut m = HashMap::new();
        for (color_name, color_triple) in triples.iter() {
            let color_array = [
                color_triple.0 as f32 / 255.0,
                color_triple.1 as f32 / 255.0,
                color_triple.2 as f32 / 255.0,
                1.0,
            ];
            m.insert(*color_name, color_array);
        }
        m
    };
    pub static ref INCORRECT_WORD_COLOR: [f32; 4] = SOLARIZED_COLOR_MAP
        .get(&SolarizedColor::Red)
        .cloned()
        .unwrap();
    pub static ref INPUT_OUTLINE_COLOR: [f32; 4] = SOLARIZED_COLOR_MAP
        .get(&SolarizedColor::Magenta)
        .cloned()
        .unwrap();
    pub static ref TIMER_OUTLINE_COLOR: [f32; 4] = SOLARIZED_COLOR_MAP
        .get(&SolarizedColor::Cyan)
        .cloned()
        .unwrap();
    pub static ref TIMER_COLOR: [f32; 4] = SOLARIZED_COLOR_MAP
        .get(&SolarizedColor::Violet)
        .cloned()
        .unwrap();
}

pub const TRANSPARENT: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
