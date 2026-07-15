use glam::Vec2;
use crate::bullet::BulletKey;

pub struct Bullet {
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
}

impl Bullet {
    pub fn root(x: f32, y: f32) -> BulletBuilder {
        BulletBuilder {
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
        }
    }

    pub fn child(parent: BulletKey, offset: Vec2) -> BulletBuilder {
        BulletBuilder {
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
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn is_child(&self) -> bool {
        self.parent.is_some()
    }
}

pub struct BulletBuilder {
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
}

impl BulletBuilder {
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

    pub fn build(self) -> Bullet {
        Bullet {
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
        }
    }
}
