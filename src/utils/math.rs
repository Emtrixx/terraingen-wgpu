pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

pub fn smooth_step(t: f32) -> f32 {
    ((6. * t - 15.) * t + 10.) * t * t * t
}
