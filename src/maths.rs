pub fn lerp(start: f32, t: f32, end: f32) -> f32 {
    (1.0 - t) * start + t * end
}

pub fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = t / (d / 2.0);
    if t < 1.0 {
        c / 2.0 * (t * t * t) + b
    } else {
        let t = t - 2.0;
        c / 2.0 * (t * t * t + 2.0) + b
    }
}
