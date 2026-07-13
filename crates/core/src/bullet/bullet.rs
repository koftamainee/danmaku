use glam::Vec2;
use crate::bullet::BulletKey;

pub enum BulletKind {
    Root {
        speed: f32,
        acceleration: f32,
        min_speed: f32,
        max_speed: f32,
        angle: f32,
        angular_velocity: f32,
        angular_acceleration: f32,
        min_angular_velocity: f32,
        max_angular_velocity: f32,
    },
    Child {
        parent: BulletKey,
        parent_offset: Vec2,
        angular_velocity: f32,
    },
}

impl Default for BulletKind {
    fn default() -> Self {
        Self::Root {
            speed: 0.0,
            acceleration: 0.0,
            min_speed: 0.0,
            max_speed: 0.0,
            angle: 0.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            min_angular_velocity: 0.0,
            max_angular_velocity: 0.0,
        }
    }
}

pub struct Bullet {
    pub position: Vec2,
    pub lifetime: i32,
    pub kind: BulletKind,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            lifetime: 0,
            kind: BulletKind::default(),
        }
    }
}