use rand;
use rand::Rng;

// constans
pub mod constans {
    pub const INFINITY: f64 = std::f64::MAX;
    pub const PI: f64 = std::f64::consts::PI;
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_i32(min: f64, max: f64) -> i32 {
    (min + (max - min) as f64 * random_f64()) as i32
}

pub struct Random {
    rng: rand::rngs::ThreadRng,
}

impl Random {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn random_f64(&mut self) -> f64 {
        self.rng.gen_range(0.0..1.0)
    }

    pub fn random_f64_range(&mut self, min: f64, max: f64) -> f64 {
        self.rng.gen_range(min..max)
    }
}
