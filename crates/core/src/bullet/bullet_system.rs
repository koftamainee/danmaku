use std::collections::{HashMap, VecDeque, HashSet};
use glam::Vec2;
use slotmap::SlotMap;
use crate::bullet::{Bullet, BulletKey};

fn outside_screen(playfield: Vec2, position: Vec2) -> bool {
    let half = playfield * 0.5;
    let margin = half * 0.5;
    let limit = half + margin;
    position.x < -limit.x || position.x > limit.x || position.y < -limit.y || position.y > limit.y
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

        if let Some(parent) = self.slot_map[key].parent {
            self.children_of.entry(parent).or_default().push(key);
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
            if bullet.is_root() {
                bullet.speed += bullet.acceleration;

                if let Some(min) = bullet.min_speed {
                    bullet.speed = bullet.speed.max(min);
                }
                if let Some(max) = bullet.max_speed {
                    bullet.speed = bullet.speed.min(max);
                }

                bullet.angular_velocity += bullet.angular_acceleration;

                if let Some(min) = bullet.min_angular_velocity {
                    bullet.angular_velocity = bullet.angular_velocity.max(min);
                }
                if let Some(max) = bullet.max_angular_velocity {
                    bullet.angular_velocity = bullet.angular_velocity.min(max);
                }

                bullet.angle += bullet.angular_velocity;

                bullet.position.x += bullet.angle.cos() * bullet.speed;
                bullet.position.y += bullet.angle.sin() * bullet.speed;

                if outside_screen(playfield, bullet.position) {
                    to_kill.push(key);
                    continue;
                }
            }

            match &mut bullet.lifetime {
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
            if bullet.is_root() {
                continue;
            }

            if bullet.angular_velocity != 0.0 {
                let r = bullet.parent_offset.length();
                let a = bullet.parent_offset.y.atan2(bullet.parent_offset.x) + bullet.angular_velocity;
                bullet.parent_offset = Vec2::new(a.cos(), a.sin()) * r;
            }
        }

        let mut queue: VecDeque<(BulletKey, Vec2)> = VecDeque::new();
        let mut visited: HashSet<BulletKey> = HashSet::new();

        for (key, bullet) in &self.slot_map {
            if bullet.is_root() {
                queue.push_back((key, bullet.position));
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
                    Some(c) => c,
                    None => continue,
                };

                if parent_alive {
                    child.position = parent_pos + child.parent_offset;

                    if outside_screen(playfield, child.position) {
                        to_kill.push(child_key);
                        continue;
                    }
                } else {
                    child.parent = None;
                }

                let child_pos = self.slot_map.get(child_key).map(|c| c.position);
                if let Some(pos) = child_pos {
                    queue.push_back((child_key, pos));
                }
            }
        }

        for key in to_kill {
            self.kill(key);
        }

        self.render_order.retain(|k| self.slot_map.get(*k).is_some());
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

    fn remove_from_parent_list(&mut self, key: BulletKey) {
        let parent = self.slot_map.get(key).and_then(|b| b.parent);

        if let Some(parent_key) = parent {
            if let Some(siblings) = self.children_of.get_mut(&parent_key) {
                siblings.retain(|&k| k != key);
            }
        }
    }

    fn detach_children(&mut self, parent: BulletKey) {
        if let Some(children) = self.children_of.remove(&parent) {
            for child_key in children {
                if let Some(bullet) = self.slot_map.get_mut(child_key) {
                    bullet.parent = None;
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
