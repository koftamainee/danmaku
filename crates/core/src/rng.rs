pub struct Rng {
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let mut state = seed;
        Self {
            s0: splitmix64(&mut state),
            s1: splitmix64(&mut state),
            s2: splitmix64(&mut state),
            s3: splitmix64(&mut state),
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let result = self.s0.wrapping_add(self.s3).rotate_left(23).wrapping_add(self.s0);
        let t = self.s1 << 17;
        self.s2 ^= self.s0;
        self.s3 ^= self.s1;
        self.s1 ^= self.s2;
        self.s0 ^= self.s3;
        self.s2 ^= t;
        self.s3 = self.s3.rotate_left(45);
        result
    }

    pub fn next<T: Sample>(&mut self) -> T {
        T::sample(self)
    }

    pub fn range<T: Sample + PartialOrd>(&mut self, min: T, max: T) -> T {
        T::range(self, min, max)
    }
}

pub trait Sample: Copy {
    fn sample(rng: &mut Rng) -> Self;
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self;
}

impl Sample for u8 {
    fn sample(rng: &mut Rng) -> Self {
        rng.next_u64() as u8
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        let range = (max as u64 - min as u64 + 1) as u64;
        (min as u64 + rng.next_u64() % range) as u8
    }
}

impl Sample for u16 {
    fn sample(rng: &mut Rng) -> Self {
        rng.next_u64() as u16
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        let range = (max as u64 - min as u64 + 1) as u64;
        (min as u64 + rng.next_u64() % range) as u16
    }
}

impl Sample for u32 {
    fn sample(rng: &mut Rng) -> Self {
        rng.next_u64() as u32
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        let range = (max as u64 - min as u64 + 1) as u64;
        (min as u64 + rng.next_u64() % range) as u32
    }
}

impl Sample for u64 {
    fn sample(rng: &mut Rng) -> Self {
        rng.next_u64()
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        let range = max - min + 1;
        min + rng.next_u64() % range
    }
}

impl Sample for i8 {
    fn sample(rng: &mut Rng) -> Self {
        rng.next_u64() as i8
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        let range = (max as u64 - min as u64 + 1) as u64;
        (min as u64 + rng.next_u64() % range) as i8
    }
}

impl Sample for i16 {
    fn sample(rng: &mut Rng) -> Self {
        rng.next_u64() as i16
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        let range = (max as u64 - min as u64 + 1) as u64;
        (min as u64 + rng.next_u64() % range) as i16
    }
}

impl Sample for i32 {
    fn sample(rng: &mut Rng) -> Self {
        rng.next_u64() as i32
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        let range = (max as i64 - min as i64 + 1) as u64;
        (min as i64 + (rng.next_u64() % range) as i64) as i32
    }
}

impl Sample for i64 {
    fn sample(rng: &mut Rng) -> Self {
        rng.next_u64() as i64
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        let range = (max as u64).wrapping_sub(min as u64).wrapping_add(1);
        (min as u64).wrapping_add(rng.next_u64() % range) as i64
    }
}

impl Sample for f32 {
    fn sample(rng: &mut Rng) -> Self {
        (rng.next_u64() as f64 / u64::MAX as f64) as f32
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        min + (max - min) * Self::sample(rng)
    }
}

impl Sample for f64 {
    fn sample(rng: &mut Rng) -> Self {
        rng.next_u64() as f64 / u64::MAX as f64
    }
    fn range(rng: &mut Rng, min: Self, max: Self) -> Self {
        min + (max - min) * Self::sample(rng)
    }
}

pub fn angle(rng: &mut Rng) -> f32 {
    rng.range(0.0f32, std::f32::consts::TAU)
}

pub fn direction(rng: &mut Rng) -> (f32, f32) {
    let a = angle(rng);
    (a.cos(), a.sin())
}

pub fn bool(rng: &mut Rng) -> bool {
    rng.next::<u64>() & 1 == 0
}

pub fn sign(rng: &mut Rng) -> f32 {
    if bool(rng) { 1.0 } else { -1.0 }
}
