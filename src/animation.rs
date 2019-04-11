use crate::ease_in_out;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Animation {
    start: f32,
    end: f32,
    t: f32,
    duration: f32,
}

impl Animation {
    pub fn new(start: f32, end: f32, duration: f32) -> Self {
        Self {
            start,
            end,
            duration,
            t: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.t += dt;
    }

    pub fn is_over(&self) -> bool {
        self.t >= self.duration
    }

    pub fn current(&self) -> f32 {
        ease_in_out(self.t, self.start, self.end, self.duration)
    }
}
