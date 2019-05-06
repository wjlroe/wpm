use lazy_static::*;
use std::collections::HashMap;

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
    pub static ref BG_COLOR: [f32; 4] = SOLARIZED_COLOR_MAP
        .get(&SolarizedColor::Base3)
        .cloned()
        .unwrap();
    pub static ref TEXT_COLOR: [f32; 4] = SOLARIZED_COLOR_MAP
        .get(&SolarizedColor::Base01)
        .cloned()
        .unwrap();
    pub static ref CORRECT_WORD_COLOR: [f32; 4] = SOLARIZED_COLOR_MAP
        .get(&SolarizedColor::Green)
        .cloned()
        .unwrap();
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
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
