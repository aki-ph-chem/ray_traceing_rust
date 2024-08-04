use crate::utl;

#[derive(Debug, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new() -> Self {
        Interval {
            min: utl::constans::INFINITY,
            max: -utl::constans::INFINITY,
        }
    }

    pub fn new_by_value(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }

        x
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;

        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

pub mod consts {
    use crate::interval::Interval;
    use crate::utl;

    pub const EMPTY: Interval = Interval {
        min: utl::constans::INFINITY,
        max: -utl::constans::INFINITY,
    };
    pub const UNIVERSE: Interval = Interval {
        min: -utl::constans::INFINITY,
        max: utl::constans::INFINITY,
    };
}
