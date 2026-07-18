use glam::Vec2;
use crate::bullet::BulletKey;
use crate::tween::EasingKind;
use crate::sprite_handle::SpriteHandle;

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

pub struct BulletCommon {
    pub position: Vec2,
    pub sprite: SpriteHandle,
    pub lifetime: Option<u32>,
    pub age: u32,
}

pub struct PolarBullet {
    pub common: BulletCommon,
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
    pub motion: MotionKind,
}

pub struct ControlledBullet {
    pub common: BulletCommon,
    pub on_update: Option<Box<dyn FnMut(&mut Self, BulletKey)>>,
}

impl ControlledBullet {
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.common.position = Vec2::new(x, y);
    }

    pub fn kill(&mut self) {
        self.common.lifetime = Some(0);
    }

    pub fn set_lifetime(&mut self, frames: u32) {
        self.common.lifetime = Some(frames);
    }
}

pub enum Bullet {
    Polar(PolarBullet),
    Controlled(ControlledBullet),
}

impl Bullet {
    pub fn root(x: f32, y: f32, sprite: SpriteHandle) -> PolarBulletBuilder {
        PolarBulletBuilder {
            common: BulletCommon {
                position: Vec2::new(x, y),
                sprite,
                lifetime: None,
                age: 0,
            },
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

    pub fn child(parent: BulletKey, offset: Vec2, sprite: SpriteHandle) -> PolarBulletBuilder {
        PolarBulletBuilder {
            common: BulletCommon {
                position: Vec2::ZERO,
                sprite,
                lifetime: None,
                age: 0,
            },
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
        sprite: SpriteHandle,
        cb: impl FnMut(&mut ControlledBullet, BulletKey) + 'static,
    ) -> ControlledBulletBuilder {
        ControlledBulletBuilder {
            common: BulletCommon {
                position: Vec2::new(x, y),
                sprite,
                lifetime: None,
                age: 0,
            },
            on_update: Some(Box::new(cb)),
        }
    }

    pub fn position(&self) -> Vec2 {
        match self {
            Bullet::Polar(p) => p.common.position,
            Bullet::Controlled(c) => c.common.position,
        }
    }

    pub fn sprite(&self) -> SpriteHandle {
        match self {
            Bullet::Polar(p) => p.common.sprite,
            Bullet::Controlled(c) => c.common.sprite,
        }
    }

    pub fn lifetime(&self) -> Option<u32> {
        match self {
            Bullet::Polar(p) => p.common.lifetime,
            Bullet::Controlled(c) => c.common.lifetime,
        }
    }

    pub fn set_lifetime(&mut self, v: Option<u32>) {
        match self {
            Bullet::Polar(p) => p.common.lifetime = v,
            Bullet::Controlled(c) => c.common.lifetime = v,
        }
    }

    pub fn rotation(&self) -> f32 {
        match self {
            Bullet::Polar(p) => p.angle,
            Bullet::Controlled(_) => 0.0,
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
    common: BulletCommon,
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
        self.common.lifetime = Some(v);
        self
    }

    pub fn motion(mut self, v: MotionKind) -> Self {
        self.motion = v;
        self
    }

    pub fn build(self) -> Bullet {
        Bullet::Polar(PolarBullet {
            common: self.common,
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
            motion: self.motion,
        })
    }
}

pub struct ControlledBulletBuilder {
    common: BulletCommon,
    on_update: Option<Box<dyn FnMut(&mut ControlledBullet, BulletKey)>>,
}

impl ControlledBulletBuilder {
    pub fn lifetime(mut self, v: u32) -> Self {
        self.common.lifetime = Some(v);
        self
    }

    pub fn build(self) -> Bullet {
        Bullet::Controlled(ControlledBullet {
            common: self.common,
            on_update: self.on_update,
        })
    }
}
