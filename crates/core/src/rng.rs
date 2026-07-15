pub struct Rng {
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
}

fn splitmix32(state: &mut u32) -> u32 {
    *state = state.wrapping_add(0x9e3779b9);
    let mut z = *state;
    z = (z ^ (z >> 16)).wrapping_mul(0x21f0aaad);
    z = (z ^ (z >> 15)).wrapping_mul(0x735a2d97);
    z = z ^ (z >> 15);
    z
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        let mut state = seed;
        let s0 = splitmix32(&mut state);
        let s1 = splitmix32(&mut state);
        let s2 = splitmix32(&mut state);
        let s3 = splitmix32(&mut state);
        Self { s0, s1, s2, s3 }
    }

    pub fn next_u32(&mut self) -> u32 {
        let result = self.s0.wrapping_add(self.s3).rotate_left(7).wrapping_add(self.s0);
        self.s1 ^= self.s0;
        self.s0 = self.s0.rotate_left(9);
        self.s2 ^= self.s1;
        self.s3 = self.s3.rotate_left(11);
        result
    }

    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }

    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min + 1) as u32;
        min + (self.next_u32() % range) as i32
    }
}

pub fn angle(rng: &mut Rng) -> f32 {
    rng.range_f32(0.0, std::f32::consts::TAU)
}

pub fn direction(rng: &mut Rng) -> (f32, f32) {
    let a = angle(rng);
    (a.cos(), a.sin())
}

pub fn bool(rng: &mut Rng) -> bool {
    rng.next_u32() & 1 == 0
}

pub fn sign(rng: &mut Rng) -> f32 {
    if bool(rng) { 1.0 } else { -1.0 }
}
