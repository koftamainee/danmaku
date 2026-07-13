use std::collections::{HashMap, VecDeque, HashSet};
use glam::Vec2;
use slotmap::SlotMap;
use crate::bullet::{Bullet, BulletKey, BulletKind};
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

        if let BulletKind::Child { parent, .. } = &self.slot_map[key].kind {
            self.children_of.entry(*parent).or_default().push(key);
        }

        self.render_order.push(key);

        key
    }

    pub fn kill(&mut self, key: BulletKey) -> Option<Bullet> {
        self.remove_from_parent_list(key);
        self.detach_descendants(key);
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
            if bullet.lifetime == 0 {
                continue;
            }

            match &mut bullet.kind {
                BulletKind::Root {
                    speed,
                    acceleration,
                    min_speed,
                    max_speed,
                    angle,
                    angular_velocity,
                    angular_acceleration,
                    min_angular_velocity,
                    max_angular_velocity,
                } => {
                    *speed += *acceleration;
                    if *min_speed != 0.0 {
                        *speed = speed.max(*min_speed);
                    }
                    if *max_speed != 0.0 {
                        *speed = speed.min(*max_speed);
                    }

                    *angular_velocity += *angular_acceleration;
                    if *min_angular_velocity != 0.0 {
                        *angular_velocity = angular_velocity.max(*min_angular_velocity);
                    }
                    if *max_angular_velocity != 0.0 {
                        *angular_velocity = angular_velocity.min(*max_angular_velocity);
                    }

                    *angle += *angular_velocity;

                    bullet.position.x += angle.cos() * *speed;
                    bullet.position.y += angle.sin() * *speed;

                    if outside_screen(playfield, bullet.position) {
                        to_kill.push(key);
                        continue;
                    }
                }
                BulletKind::Child { .. } => {}
            }

            if bullet.lifetime == 1 {
                to_kill.push(key);
            } else if bullet.lifetime > 0 {
                bullet.lifetime -= 1;
            }
        }

        for (_, bullet) in self.slot_map.iter_mut() {
            if bullet.lifetime == 0 {
                continue;
            }
            if let BulletKind::Child {
                angular_velocity,
                parent_offset,
                ..
            } = &mut bullet.kind
            {
                if *angular_velocity != 0.0 {
                    let r = parent_offset.length();
                    let a = parent_offset.y.atan2(parent_offset.x) + *angular_velocity;
                    *parent_offset = Vec2::new(a.cos(), a.sin()) * r;
                }
            }
        }

        let mut queue: VecDeque<(BulletKey, Vec2)> = VecDeque::new();
        let mut visited: HashSet<BulletKey> = HashSet::new();

        for (key, bullet) in &self.slot_map {
            if bullet.lifetime != 0 && matches!(bullet.kind, BulletKind::Root { .. }) {
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

                if let Some(child) = self.slot_map.get_mut(child_key) {
                    if let BulletKind::Child { parent_offset, .. } = &child.kind {
                        child.position = parent_pos + *parent_offset;
                    }

                    if outside_screen(playfield, child.position) {
                        to_kill.push(child_key);
                        continue;
                    }
                }

                if let Some(child) = self.slot_map.get(child_key) {
                    queue.push_back((child_key, child.position));
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
        let parent = self.slot_map.get(key).and_then(|b| match &b.kind {
            BulletKind::Child { parent, .. } => Some(*parent),
            _ => None,
        });
    
        if let Some(parent_key) = parent {
            if let Some(siblings) = self.children_of.get_mut(&parent_key) {
                siblings.retain(|&k| k != key);
            }
        }
    }

    fn detach_descendants(&mut self, root: BulletKey) {
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while let Some(parent) = queue.pop_front() {
            if let Some(children) = self.children_of.remove(&parent) {
                for child_key in children {
                    if let Some(bullet) = self.slot_map.get_mut(child_key) {
                        bullet.kind = BulletKind::default()
                    }
                    queue.push_back(child_key);
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
