use crate::animation::Animation;
use cgmath::*;

#[derive(Clone, Default)]
pub struct TypingState {
    pub per_line_height: f32,
    pub animation: Option<Animation>,
    pub current_word_idx: usize,
    pub first_word_idx_per_line: Vec<usize>,
    pub word_idx_at_start_of_line: usize,
    pub word_idx_at_prev_line: usize,
    pub num_words: usize,
}

impl TypingState {
    pub fn update(&mut self, dt: f32) {
        if let Some(animation) = &mut self.animation {
            animation.update(dt);
            if animation.is_over() {
                self.animation = None;
            }
        }
    }

    pub fn skip_num(&self) -> usize {
        if self.animation.is_some() {
            self.word_idx_at_prev_line
        } else {
            if self.word_idx_at_start_of_line > 0 {
                self.word_idx_at_start_of_line
            } else {
                0
            }
        }
    }

    pub fn offset(&self) -> f32 {
        if let Some(animation) = self.animation {
            animation.current()
        } else {
            0.0
        }
    }

    pub fn transform(&self, window_dim: Vector2<f32>) -> Matrix4<f32> {
        Matrix4::from_translation(vec3(0.0, self.offset() / (window_dim.y / 2.0), 0.0))
    }

    pub fn start_animation(&mut self) {
        self.animation = Some(Animation::new(0.0, self.per_line_height, 1.5));
    }

    pub fn next_word(&mut self) {
        assert!(
            self.num_lines() > 0,
            "there should be more than zero lines!"
        );
        assert!(self.num_words > 0, "there should be more than zero words!");
        if self.current_word_idx < self.num_words - 1 {
            self.current_word_idx += 1;
            assert!(
                self.per_line_height > 0.0,
                "per_line_height should be non-zero!"
            );
            if self
                .first_word_idx_per_line
                .contains(&self.current_word_idx)
            {
                self.start_animation();
                self.word_idx_at_prev_line = self.word_idx_at_start_of_line;
                self.word_idx_at_start_of_line = self.current_word_idx;
            }
        }
    }

    pub fn num_lines(&self) -> usize {
        self.first_word_idx_per_line.len()
    }
}
