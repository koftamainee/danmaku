use glam::Vec2;
use crate::bullet::BulletKey;

pub enum MotionKind {
    None,
    Sinusoidal { amplitude: f32, frequency: f32, phase: f32 },
    Lerp {
        initial_speed: f32,
        target_speed: f32,
        initial_angle: f32,
        target_angle: f32,
        duration: u32,
        easing: EasingKind,
    },
}

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
                -2.0_f32.powf(10.0 * t - 10.0) * (t * 10.0 - 10.75) * (2.0 * std::f32::consts::PI / 3.0).sin()
            }
        }
    }
}

pub struct PolarBullet {
    pub position: Vec2,
    pub lifetime: Option<u32>,
    pub parent: Option<BulletKey>,
    pub parent_offset: Vec2,
    pub speed: f32,
    pub acceleration: f32,
    pub min_speed: Option<f32>,
    pub max_speed: Option<f32>,
    pub angle: f32,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
    pub min_angular_velocity: Option<f32>,
    pub max_angular_velocity: Option<f32>,
    pub age: u32,
    pub motion: MotionKind,
}

pub struct ControlledBullet {
    pub position: Vec2,
    pub age: u32,
    pub lifetime: Option<u32>,
    pub on_update: Option<Box<dyn FnMut(&mut Self, BulletKey)>>,
}

impl ControlledBullet {
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Vec2::new(x, y);
    }

    pub fn kill(&mut self) {
        self.lifetime = Some(0);
    }

    pub fn set_lifetime(&mut self, frames: u32) {
        self.lifetime = Some(frames);
    }
}

pub enum Bullet {
    Polar(PolarBullet),
    Controlled(ControlledBullet),
}

impl Bullet {
    pub fn root(x: f32, y: f32) -> PolarBulletBuilder {
        PolarBulletBuilder {
            position: Vec2::new(x, y),
            lifetime: None,
            parent: None,
            parent_offset: Vec2::ZERO,
            speed: 0.0,
            acceleration: 0.0,
            min_speed: None,
            max_speed: None,
            angle: 0.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            min_angular_velocity: None,
            max_angular_velocity: None,
            motion: MotionKind::None,
        }
    }

    pub fn child(parent: BulletKey, offset: Vec2) -> PolarBulletBuilder {
        PolarBulletBuilder {
            position: Vec2::ZERO,
            lifetime: None,
            parent: Some(parent),
            parent_offset: offset,
            speed: 0.0,
            acceleration: 0.0,
            min_speed: None,
            max_speed: None,
            angle: 0.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            min_angular_velocity: None,
            max_angular_velocity: None,
            motion: MotionKind::None,
        }
    }

    pub fn controlled(
        x: f32,
        y: f32,
        cb: impl FnMut(&mut ControlledBullet, BulletKey) + 'static,
    ) -> ControlledBulletBuilder {
        ControlledBulletBuilder {
            position: Vec2::new(x, y),
            lifetime: None,
            on_update: Some(Box::new(cb)),
        }
    }

    pub fn position(&self) -> Vec2 {
        match self {
            Bullet::Polar(p) => p.position,
            Bullet::Controlled(c) => c.position,
        }
    }

    pub fn lifetime(&self) -> Option<u32> {
        match self {
            Bullet::Polar(p) => p.lifetime,
            Bullet::Controlled(c) => c.lifetime,
        }
    }

    pub fn set_lifetime(&mut self, v: Option<u32>) {
        match self {
            Bullet::Polar(p) => p.lifetime = v,
            Bullet::Controlled(c) => c.lifetime = v,
        }
    }

    pub fn is_root(&self) -> bool {
        match self {
            Bullet::Polar(p) => p.parent.is_none(),
            Bullet::Controlled(_) => true,
        }
    }

    pub fn is_child(&self) -> bool {
        !self.is_root()
    }
}

pub struct PolarBulletBuilder {
    position: Vec2,
    lifetime: Option<u32>,
    parent: Option<BulletKey>,
    parent_offset: Vec2,
    speed: f32,
    acceleration: f32,
    min_speed: Option<f32>,
    max_speed: Option<f32>,
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,
    min_angular_velocity: Option<f32>,
    max_angular_velocity: Option<f32>,
    motion: MotionKind,
}

impl PolarBulletBuilder {
    pub fn speed(mut self, v: f32) -> Self {
        self.speed = v;
        self
    }

    pub fn accel(mut self, v: f32) -> Self {
        self.acceleration = v;
        self
    }

    pub fn min_speed(mut self, v: f32) -> Self {
        self.min_speed = Some(v);
        self
    }

    pub fn max_speed(mut self, v: f32) -> Self {
        self.max_speed = Some(v);
        self
    }

    pub fn angle(mut self, v: f32) -> Self {
        self.angle = v;
        self
    }

    pub fn angular_velocity(mut self, v: f32) -> Self {
        self.angular_velocity = v;
        self
    }

    pub fn angular_acceleration(mut self, v: f32) -> Self {
        self.angular_acceleration = v;
        self
    }

    pub fn min_angular_velocity(mut self, v: f32) -> Self {
        self.min_angular_velocity = Some(v);
        self
    }

    pub fn max_angular_velocity(mut self, v: f32) -> Self {
        self.max_angular_velocity = Some(v);
        self
    }

    pub fn lifetime(mut self, v: u32) -> Self {
        self.lifetime = Some(v);
        self
    }

    pub fn motion(mut self, v: MotionKind) -> Self {
        self.motion = v;
        self
    }

    pub fn build(self) -> Bullet {
        Bullet::Polar(PolarBullet {
            position: self.position,
            lifetime: self.lifetime,
            parent: self.parent,
            parent_offset: self.parent_offset,
            speed: self.speed,
            acceleration: self.acceleration,
            min_speed: self.min_speed,
            max_speed: self.max_speed,
            angle: self.angle,
            angular_velocity: self.angular_velocity,
            angular_acceleration: self.angular_acceleration,
            min_angular_velocity: self.min_angular_velocity,
            max_angular_velocity: self.max_angular_velocity,
            age: 0,
            motion: self.motion,
        })
    }
}

pub struct ControlledBulletBuilder {
    position: Vec2,
    lifetime: Option<u32>,
    on_update: Option<Box<dyn FnMut(&mut ControlledBullet, BulletKey)>>,
}

impl ControlledBulletBuilder {
    pub fn lifetime(mut self, v: u32) -> Self {
        self.lifetime = Some(v);
        self
    }

    pub fn build(self) -> Bullet {
        Bullet::Controlled(ControlledBullet {
            position: self.position,
            age: 0,
            lifetime: self.lifetime,
            on_update: self.on_update,
        })
    }
}
