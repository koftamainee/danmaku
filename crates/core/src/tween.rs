pub enum EasingKind {
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    Bounce,
    Elastic,
}

pub fn apply_easing(t: f32, kind: &EasingKind) -> f32 {
    let t = t.clamp(0.0, 1.0);
    match kind {
        EasingKind::Linear => t,
        EasingKind::QuadIn => t * t,
        EasingKind::QuadOut => 1.0 - (1.0 - t) * (1.0 - t),
        EasingKind::QuadInOut => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            }
        }
        EasingKind::CubicIn => t * t * t,
        EasingKind::CubicOut => 1.0 - (1.0 - t).powi(3),
        EasingKind::Bounce => {
            if t < 1.0 / 2.75 {
                7.5625 * t * t
            } else if t < 2.0 / 2.75 {
                let t = t - 1.5 / 2.75;
                7.5625 * t * t + 0.75
            } else if t < 2.5 / 2.75 {
                let t = t - 2.25 / 2.75;
                7.5625 * t * t + 0.9375
            } else {
                let t = t - 2.625 / 2.75;
                7.5625 * t * t + 0.984375
            }
        }
        EasingKind::Elastic => {
            if t == 0.0 || t == 1.0 {
                t
            } else {
                -2.0_f32.powf(10.0 * t - 10.0)
                    * (t * 10.0 - 10.75)
                    * (2.0 * std::f32::consts::PI / 3.0).sin()
            }
        }
    }
}
