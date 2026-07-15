use glam::Vec2;
use crate::bullet::{Bullet, BulletKey, BulletSystem, ControlledBullet};
use crate::motion::MotionSpec;
use crate::rng::{self, Rng};
use crate::sprite_handle::SpriteHandle;
use crate::tween::EasingKind;

pub enum AngleType {
    Absolute,
    Relative,
    Player,
}

pub struct BulletParams {
    pub x: f32,
    pub y: f32,
    pub sprite: SpriteHandle,
    pub speed: f32,
    pub angle: f32,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
    pub lifetime: Option<u32>,
    pub motion: MotionSpec,
}

pub struct ScriptApi<'a> {
    bullet_system: &'a mut BulletSystem,
    rng: &'a mut Rng,
}

impl<'a> ScriptApi<'a> {
    pub fn new(bullet_system: &'a mut BulletSystem, rng: &'a mut Rng) -> Self {
        Self { bullet_system, rng }
    }

    pub fn log(&self, msg: &str) {
        crate::log::log(msg);
    }

    pub fn rng_float(&mut self) -> f32 {
        self.rng.next()
    }

    pub fn rng_rangef(&mut self, min: f32, max: f32) -> f32 {
        self.rng.range(min, max)
    }

    pub fn rng_rangei(&mut self, min: i32, max: i32) -> i32 {
        self.rng.range(min, max)
    }

    pub fn rng_angle(&mut self) -> f32 {
        rng::angle(self.rng)
    }

    pub fn rng_angle_range(&mut self, min: f32, max: f32) -> f32 {
        self.rng.range(min, max)
    }

    pub fn rng_bool(&mut self) -> bool {
        rng::bool(self.rng)
    }

    pub fn rng_direction(&mut self) -> (f32, f32) {
        rng::direction(self.rng)
    }

    pub fn rng_sign(&mut self) -> f32 {
        rng::sign(self.rng)
    }

    pub fn bullet_spawn(&mut self, params: BulletParams) -> BulletKey {
        let motion = params.motion.resolve(params.speed, params.angle);
        let mut builder = Bullet::root(params.x, params.y, params.sprite)
            .speed(params.speed)
            .angle(params.angle)
            .angular_velocity(params.angular_velocity)
            .angular_acceleration(params.angular_acceleration)
            .motion(motion);

        if let Some(lifetime) = params.lifetime {
            builder = builder.lifetime(lifetime);
        }

        self.bullet_system.spawn(builder.build())
    }

    pub fn bullet_spawn_controlled(
        &mut self,
        x: f32,
        y: f32,
        sprite: SpriteHandle,
        cb: impl FnMut(&mut ControlledBullet, BulletKey) + 'static,
    ) -> BulletKey {
        self.bullet_system
            .spawn(Bullet::controlled(x, y, sprite, cb).build())
    }

    pub fn bullet_count(&self) -> usize {
        self.bullet_system.len()
    }

    pub fn bullet_get_all(&self) -> Vec<(BulletKey, Vec2)> {
        self.bullet_system
            .iter()
            .map(|(key, bullet)| (key, bullet.position()))
            .collect()
    }

    pub fn bullet_set_speed(&mut self, key: BulletKey, v: f32) {
        self.bullet_system.set_speed(key, v);
    }

    pub fn bullet_set_accel(&mut self, key: BulletKey, v: f32) {
        self.bullet_system.set_accel(key, v);
    }

    pub fn bullet_set_angle(&mut self, key: BulletKey, angle: f32, angle_type: AngleType) {
        match angle_type {
            AngleType::Absolute => self.bullet_system.set_angle(key, angle),
            AngleType::Relative => {
                if let Some(b) = self.bullet_system.get(key) {
                    let current = match b {
                        Bullet::Polar(p) => p.angle,
                        _ => 0.0,
                    };
                    self.bullet_system.set_angle(key, current + angle);
                }
            }
            AngleType::Player => todo!("aim at player position"),
        }
    }

    pub fn bullet_aim(&mut self, key: BulletKey, target_x: f32, target_y: f32) {
        if let Some(b) = self.bullet_system.get(key) {
            let pos = b.position();
            let angle = (target_y - pos.y).atan2(target_x - pos.x);
            self.bullet_system.set_angle(key, angle);
        }
    }

    pub fn bullet_set_angular_velocity(&mut self, key: BulletKey, v: f32) {
        self.bullet_system.set_angular_velocity(key, v);
    }

    pub fn bullet_set_angular_acceleration(&mut self, key: BulletKey, v: f32) {
        self.bullet_system.set_angular_acceleration(key, v);
    }

    pub fn bullet_set_min_speed(&mut self, key: BulletKey, v: f32) {
        self.bullet_system.set_min_speed(key, v);
    }

    pub fn bullet_set_max_speed(&mut self, key: BulletKey, v: f32) {
        self.bullet_system.set_max_speed(key, v);
    }

    pub fn bullet_set_min_angular_velocity(&mut self, key: BulletKey, v: f32) {
        self.bullet_system.set_min_angular_velocity(key, v);
    }

    pub fn bullet_set_max_angular_velocity(&mut self, key: BulletKey, v: f32) {
        self.bullet_system.set_max_angular_velocity(key, v);
    }

    pub fn bullet_set_lifetime(&mut self, key: BulletKey, frames: u32) {
        self.bullet_system.set_lifetime(key, frames);
    }

    pub fn bullet_set_position(&mut self, key: BulletKey, x: f32, y: f32) {
        self.bullet_system.set_position(key, x, y);
    }

    pub fn bullet_set_parent(&mut self, child: BulletKey, parent: BulletKey, ox: f32, oy: f32) {
        self.bullet_system
            .set_parent(child, parent, Vec2::new(ox, oy));
    }

    pub fn bullet_detach(&mut self, key: BulletKey) {
        self.bullet_system.detach(key);
    }

    pub fn bullet_kill(&mut self, key: BulletKey) {
        self.bullet_system.kill(key);
    }

    pub fn motion_none() -> MotionSpec {
        MotionSpec::None
    }

    pub fn motion_sinusoidal(amplitude: f32, frequency: f32, phase: f32) -> MotionSpec {
        MotionSpec::Sinusoidal {
            amplitude,
            frequency,
            phase,
        }
    }

    pub fn motion_lerp(
        target_speed: Option<f32>,
        target_angle: Option<f32>,
        duration: u32,
        easing: EasingKind,
    ) -> MotionSpec {
        MotionSpec::LerpTo {
            target_speed,
            target_angle,
            duration,
            easing,
        }
    }
}
