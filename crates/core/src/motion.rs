use crate::bullet::MotionKind;
use crate::tween::EasingKind;

pub enum MotionSpec {
    None,
    Sinusoidal {
        amplitude: f32,
        frequency: f32,
        phase: f32,
    },
    LerpTo {
        target_speed: Option<f32>,
        target_angle: Option<f32>,
        duration: u32,
        easing: EasingKind,
    },
}

impl MotionSpec {
    pub fn resolve(self, initial_speed: f32, initial_angle: f32) -> MotionKind {
        match self {
            MotionSpec::None => MotionKind::None,
            MotionSpec::Sinusoidal {
                amplitude,
                frequency,
                phase,
            } => MotionKind::Sinusoidal {
                amplitude,
                frequency,
                phase,
            },
            MotionSpec::LerpTo {
                target_speed,
                target_angle,
                duration,
                easing,
            } => MotionKind::Lerp {
                initial_speed,
                target_speed: target_speed.unwrap_or(initial_speed),
                initial_angle,
                target_angle: target_angle.unwrap_or(initial_angle),
                duration,
                easing,
            },
        }
    }
}
