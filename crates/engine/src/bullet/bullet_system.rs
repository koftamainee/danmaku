use std::collections::{HashMap, HashSet, VecDeque};
use glam::Vec2;
use slotmap::SlotMap;
use crate::bullet::{Bullet, BulletKey, MotionKind};
use crate::sprite_handle::SpriteHandle;
use crate::tween::apply_easing;

fn outside_screen(playfield: Vec2, position: Vec2) -> bool {
    let half = playfield * 0.5;
    let margin = half * 0.5;
    let limit = half + margin;
    position.x < -limit.x || position.x > limit.x || position.y < -limit.y || position.y > limit.y
}

pub struct RenderInstance {
    pub position: Vec2,
    pub rotation: f32,
    pub sprite: SpriteHandle,
}

pub struct BulletSystem {
    slot_map: SlotMap<BulletKey, Bullet>,
    playfield: Vec2,
    children_of: HashMap<BulletKey, Vec<BulletKey>>,
    render_order: Vec<BulletKey>,
}

impl BulletSystem {
    pub fn new(initial_capacity: usize, playfield: Vec2) -> Self {
        BulletSystem {
            slot_map: SlotMap::with_capacity_and_key(initial_capacity),
            playfield,
            children_of: HashMap::new(),
            render_order: Vec::new(),
        }
    }

    pub fn playfield(&self) -> Vec2 {
        self.playfield
    }

    pub fn spawn(&mut self, init: Bullet) -> BulletKey {
        let key = self.slot_map.insert(init);

        if let Bullet::Polar(ref polar) = self.slot_map[key] {
            if let Some(parent) = polar.parent {
                self.children_of.entry(parent).or_default().push(key);
            }
        }

        self.render_order.push(key);
        key
    }

    pub fn kill(&mut self, key: BulletKey) -> Option<Bullet> {
        self.remove_from_parent_list(key);
        self.detach_children(key);
        self.children_of.remove(&key);
        self.slot_map.remove(key)
    }

    pub fn get(&self, key: BulletKey) -> Option<&Bullet> {
        self.slot_map.get(key)
    }

    pub fn get_mut(&mut self, key: BulletKey) -> Option<&mut Bullet> {
        self.slot_map.get_mut(key)
    }

    pub fn update(&mut self) {
        let mut to_kill: Vec<BulletKey> = Vec::new();
        let playfield = self.playfield;

        for (key, bullet) in self.slot_map.iter_mut() {
            if let Bullet::Polar(polar) = bullet {
                if polar.parent.is_none() {
                    polar.speed += polar.acceleration;

                    if let Some(min) = polar.min_speed {
                        polar.speed = polar.speed.max(min);
                    }
                    if let Some(max) = polar.max_speed {
                        polar.speed = polar.speed.min(max);
                    }

                    polar.angular_velocity += polar.angular_acceleration;

                    if let Some(min) = polar.min_angular_velocity {
                        polar.angular_velocity = polar.angular_velocity.max(min);
                    }
                    if let Some(max) = polar.max_angular_velocity {
                        polar.angular_velocity = polar.angular_velocity.min(max);
                    }

                    polar.angle += polar.angular_velocity;

                    polar.common.position.x += polar.angle.cos() * polar.speed;
                    polar.common.position.y += polar.angle.sin() * polar.speed;

                    match &polar.motion {
                        MotionKind::Sinusoidal {
                            amplitude,
                            frequency,
                            phase,
                        } => {
                            let perp = Vec2::new(-polar.angle.sin(), polar.angle.cos());
                            let offset = ((polar.common.age as f32 * frequency) + phase).sin()
                                * amplitude;
                            polar.common.position += perp * offset;
                        }
                        MotionKind::Lerp {
                            initial_speed,
                            target_speed,
                            initial_angle,
                            target_angle,
                            duration,
                            easing,
                        } => {
                            let t = (polar.common.age as f32 / *duration as f32).min(1.0);
                            let t = apply_easing(t, easing);
                            polar.speed =
                                initial_speed + (target_speed - initial_speed) * t;
                            polar.angle =
                                initial_angle + (target_angle - initial_angle) * t;
                        }
                        MotionKind::None => {}
                    }

                    if outside_screen(playfield, polar.common.position) {
                        to_kill.push(key);
                        continue;
                    }
                }
            }
        }

        let controlled_keys: Vec<BulletKey> = self
            .slot_map
            .iter()
            .filter_map(|(key, bullet)| matches!(bullet, Bullet::Controlled(_)).then_some(key))
            .collect();

        for key in controlled_keys {
            if let Some(Bullet::Controlled(ctrl)) = self.slot_map.get_mut(key) {
                let mut cb = ctrl.on_update.take();
                if let Some(ref mut cb) = cb {
                    cb(ctrl, key);
                }
                ctrl.on_update = cb;
            }
        }

        for (key, bullet) in self.slot_map.iter_mut() {
            let lifetime = match bullet {
                Bullet::Polar(p) => &mut p.common.lifetime,
                Bullet::Controlled(c) => &mut c.common.lifetime,
            };

            match lifetime {
                None => {}
                Some(1) => {
                    to_kill.push(key);
                }
                Some(n) => {
                    *n -= 1;
                }
            }
        }

        for (_, bullet) in self.slot_map.iter_mut() {
            match bullet {
                Bullet::Polar(p) => p.common.age += 1,
                Bullet::Controlled(c) => c.common.age += 1,
            }
        }

        for (_, bullet) in self.slot_map.iter_mut() {
            if let Bullet::Polar(polar) = bullet {
                if polar.parent.is_some() && polar.angular_velocity != 0.0 {
                    let r = polar.parent_offset.length();
                    let a = polar.parent_offset.y.atan2(polar.parent_offset.x)
                        + polar.angular_velocity;
                    polar.parent_offset = Vec2::new(a.cos(), a.sin()) * r;
                }
            }
        }

        let mut queue: VecDeque<(BulletKey, Vec2)> = VecDeque::new();
        let mut visited: HashSet<BulletKey> = HashSet::new();

        for (key, bullet) in &self.slot_map {
            if bullet.is_root() {
                let pos = bullet.position();
                queue.push_back((key, pos));
                visited.insert(key);
            }
        }

        while let Some((parent_key, parent_pos)) = queue.pop_front() {
            let children = match self.children_of.get(&parent_key) {
                Some(c) => c.clone(),
                None => continue,
            };

            for child_key in children {
                if visited.contains(&child_key) {
                    continue;
                }
                visited.insert(child_key);

                let parent_alive = self.slot_map.contains_key(parent_key);

                let child = match self.slot_map.get_mut(child_key) {
                    Some(Bullet::Polar(c)) => c,
                    _ => continue,
                };

                if parent_alive {
                    child.common.position = parent_pos + child.parent_offset;

                    if outside_screen(playfield, child.common.position) {
                        to_kill.push(child_key);
                        continue;
                    }
                } else {
                    child.parent = None;
                }

                let child_pos = self
                    .slot_map
                    .get(child_key)
                    .map(|c| c.position());
                if let Some(pos) = child_pos {
                    queue.push_back((child_key, pos));
                }
            }
        }

        for key in to_kill {
            self.kill(key);
        }

        self.render_order
            .retain(|k| self.slot_map.get(*k).is_some());
    }

    pub fn clear(&mut self) {
        self.slot_map.clear();
        self.children_of.clear();
        self.render_order.clear();
    }

    pub fn len(&self) -> usize {
        self.slot_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slot_map.is_empty()
    }

    pub fn render_order(&self) -> &[BulletKey] {
        &self.render_order
    }

    pub fn render_instances(&self) -> impl Iterator<Item = RenderInstance> + '_ {
        self.render_order.iter().filter_map(move |&key| {
            self.slot_map.get(key).map(|b| RenderInstance {
                position: b.position(),
                rotation: b.rotation(),
                sprite: b.sprite(),
            })
        })
    }

    pub fn iter(&self) -> slotmap::basic::Iter<'_, BulletKey, Bullet> {
        self.slot_map.iter()
    }

    pub fn iter_mut(&mut self) -> slotmap::basic::IterMut<'_, BulletKey, Bullet> {
        self.slot_map.iter_mut()
    }

    pub fn values(&self) -> slotmap::basic::Values<'_, BulletKey, Bullet> {
        self.slot_map.values()
    }

    pub fn values_mut(&mut self) -> slotmap::basic::ValuesMut<'_, BulletKey, Bullet> {
        self.slot_map.values_mut()
    }

    pub fn set_speed(&mut self, key: BulletKey, v: f32) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.speed = v;
        }
    }

    pub fn set_accel(&mut self, key: BulletKey, v: f32) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.acceleration = v;
        }
    }

    pub fn set_angle(&mut self, key: BulletKey, angle: f32) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.angle = angle;
        }
    }

    pub fn set_angular_velocity(&mut self, key: BulletKey, v: f32) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.angular_velocity = v;
        }
    }

    pub fn set_angular_acceleration(&mut self, key: BulletKey, v: f32) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.angular_acceleration = v;
        }
    }

    pub fn set_min_speed(&mut self, key: BulletKey, v: f32) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.min_speed = Some(v);
        }
    }

    pub fn set_max_speed(&mut self, key: BulletKey, v: f32) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.max_speed = Some(v);
        }
    }

    pub fn set_min_angular_velocity(&mut self, key: BulletKey, v: f32) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.min_angular_velocity = Some(v);
        }
    }

    pub fn set_max_angular_velocity(&mut self, key: BulletKey, v: f32) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.max_angular_velocity = Some(v);
        }
    }

    pub fn set_lifetime(&mut self, key: BulletKey, frames: u32) {
        if let Some(bullet) = self.slot_map.get_mut(key) {
            bullet.set_lifetime(Some(frames));
        }
    }

    pub fn set_position(&mut self, key: BulletKey, x: f32, y: f32) {
        if let Some(bullet) = self.slot_map.get_mut(key) {
            match bullet {
                Bullet::Polar(p) => p.common.position = Vec2::new(x, y),
                Bullet::Controlled(c) => c.common.position = Vec2::new(x, y),
            }
        }
    }

    pub fn set_parent(&mut self, child: BulletKey, parent: BulletKey, offset: Vec2) {
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(child) {
            p.parent = Some(parent);
            p.parent_offset = offset;
        }
        if let Some(parent_pos) = self.slot_map.get(parent).map(|b| b.position()) {
            if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(child) {
                p.common.position = parent_pos + offset;
            }
        }
        self.children_of.entry(parent).or_default().push(child);
    }

    pub fn detach(&mut self, key: BulletKey) {
        self.remove_from_parent_list(key);
        if let Some(Bullet::Polar(p)) = self.slot_map.get_mut(key) {
            p.parent = None;
            p.parent_offset = Vec2::ZERO;
        }
    }

    fn remove_from_parent_list(&mut self, key: BulletKey) {
        let parent = match self.slot_map.get(key) {
            Some(Bullet::Polar(polar)) => polar.parent,
            _ => None,
        };

        if let Some(parent_key) = parent {
            if let Some(siblings) = self.children_of.get_mut(&parent_key) {
                siblings.retain(|&k| k != key);
            }
        }
    }

    fn detach_children(&mut self, parent: BulletKey) {
        if let Some(children) = self.children_of.remove(&parent) {
            for child_key in children {
                if let Some(Bullet::Polar(polar)) = self.slot_map.get_mut(child_key) {
                    polar.parent = None;
                }
            }
        }
    }
}

impl IntoIterator for BulletSystem {
    type Item = (BulletKey, Bullet);
    type IntoIter = slotmap::basic::IntoIter<BulletKey, Bullet>;

    fn into_iter(self) -> Self::IntoIter {
        self.slot_map.into_iter()
    }
}

impl<'a> IntoIterator for &'a BulletSystem {
    type Item = (BulletKey, &'a Bullet);
    type IntoIter = slotmap::basic::Iter<'a, BulletKey, Bullet>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut BulletSystem {
    type Item = (BulletKey, &'a mut Bullet);
    type IntoIter = slotmap::basic::IterMut<'a, BulletKey, Bullet>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
