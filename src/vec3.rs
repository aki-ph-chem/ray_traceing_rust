use crate::utl;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

/// 3-dimentional vector with utility
#[derive(Debug, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    /// initialize from slice [f64; 3]
    pub fn from_slice(slice: [f64; 3]) -> Self {
        Self { e: slice }
    }

    /// initialize from other value of Vec3
    pub fn new_unit_vec(other: Self) -> Self {
        let norm_other = other.norm();
        other / norm_other
    }

    /// initialize with random values between 0.0 and 1.0
    pub fn random() -> Self {
        let mut random = utl::Random::new();
        Self {
            e: [
                random.random_f64(),
                random.random_f64(),
                random.random_f64(),
            ],
        }
    }

    /// initialize with random values between min and max
    pub fn random_by_range(min: f64, max: f64) -> Self {
        let mut random = utl::Random::new();
        Self {
            e: [
                random.random_f64_range(min, max),
                random.random_f64_range(min, max),
                random.random_f64_range(min, max),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_by_range(-1.0, 1.0);
            if p.norm() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();

        if on_unit_sphere.dot(&normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::new_unit_vec(Self::random_in_unit_sphere())
    }

    pub fn reflect(v: &Self, n: &Self) -> Self {
        v.clone() - 2.0 * v.dot(&n) * n.clone()
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut random = utl::Random::new();
        loop {
            let p = Vec3::from_slice([
                random.random_f64_range(-1.0, 1.0),
                random.random_f64_range(-1.0, 1.0),
                0.0,
            ]);
            if p.norm_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn refract(uv: &Self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-uv.dot(&n)).min(1.0);
        let r_out_perp = etai_over_etat * (uv.clone() + cos_theta * n.clone());
        let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs().sqrt()) * n.clone();

        r_out_perp + r_out_parallel
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    /// get the value of the square of the l_2 norm
    pub fn norm_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// get the value of the l_2 norm
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y() < s && self.z() < s
    }

    /// normalize self
    pub fn normalize(&mut self) {
        *self /= self.norm();
    }

    /// sum of comenents x, y, z
    pub fn sum(&self) -> f64 {
        self.x() + self.y() + self.z()
    }

    /// calc dot product between self and other
    pub fn dot(&self, other: &Self) -> f64 {
        self.e
            .iter()
            .zip(other.e.iter())
            .fold(0.0, |res, (u, v)| res + u * v)
    }

    /// calc cross product between self and other
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }
}

/// calc dot product between two vectors lhs, rhs
pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.e
        .iter()
        .zip(rhs.e.iter())
        .fold(0.0, |res, (u, v)| res + u * v)
}

/// calc cross product between two vectors lhs, rhs
pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        ],
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [rhs * self.e[0], rhs * self.e[1], rhs * self.e[2]],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        let vec = Vec3::from_slice([1.0, 2.0, 3.0]);
        let vec_neg = -vec;
        let vec_ans = Vec3::from_slice([-1.0, -2.0, -3.0]);

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec_neg.x() - vec_ans.x()).abs() < epsilon
                && (vec_ans.y() - vec_neg.y()).abs() < epsilon
                && (vec_ans.z() - vec_neg.z()).abs() < epsilon
        );
    }

    #[test]
    fn test_index() {
        let vec = Vec3::from_slice([1.0, 2.0, 3.0]);
        let ans = [1.0, 2.0, 3.0];

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec[0] - ans[0]).abs() < epsilon
                && (vec[1] - ans[1]).abs() < epsilon
                && (vec[2] - ans[2]).abs() < epsilon
        );
    }

    #[test]
    fn test_index_mut() {
        let mut vec = Vec3::from_slice([1.0, 2.0, 3.0]);
        let ans = [11.0, 22.0, 33.0];

        vec[0] = 11.0;
        vec[1] = 22.0;
        vec[2] = 33.0;

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec[0] - ans[0]).abs() < epsilon
                && (vec[1] - ans[1]).abs() < epsilon
                && (vec[2] - ans[2]).abs() < epsilon
        );
    }
    #[test]
    fn test_add() {
        let vec_1 = Vec3::from_slice([1.0, 2.0, 3.0]);
        let vec_2 = Vec3::from_slice([1.5, 2.5, 3.5]);
        let ans = Vec3::from_slice([2.5, 4.5, 6.5]);

        let vec_3 = vec_1 + vec_2;

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec_3[0] - ans[0]).abs() < epsilon
                && (vec_3[1] - ans[1]).abs() < epsilon
                && (vec_3[2] - ans[2]).abs() < epsilon
        );
    }

    #[test]
    fn test_mul_1() {
        let vec_1 = Vec3::from_slice([1.0, 2.0, 2.0]);
        let vec_2 = Vec3::from_slice([1.5, 2.5, 3.5]);
        let ans = Vec3::from_slice([1.5, 5.0, 7.0]);

        let vec_3 = vec_1 * vec_2;

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec_3[0] - ans[0]).abs() < epsilon
                && (vec_3[1] - ans[1]).abs() < epsilon
                && (vec_3[2] - ans[2]).abs() < epsilon
        );
    }

    #[test]
    fn test_mul_2() {
        let vec_1 = Vec3::from_slice([1.0, 2.0, 3.0]);
        let ans = Vec3::from_slice([2.0, 4.0, 6.0]);

        let vec_3 = vec_1.clone() * 2.0;
        let vec_4 = 2.0 * vec_1;

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec_3[0] - ans[0]).abs() < epsilon
                && (vec_3[1] - ans[1]).abs() < epsilon
                && (vec_3[2] - ans[2]).abs() < epsilon
        );
        assert!(
            (vec_4[0] - ans[0]).abs() < epsilon
                && (vec_4[1] - ans[1]).abs() < epsilon
                && (vec_4[2] - ans[2]).abs() < epsilon
        );
    }

    #[test]
    fn test_div() {
        let vec_1 = Vec3::from_slice([2.0, 4.0, 6.0]);
        let ans = Vec3::from_slice([1.0, 2.0, 3.0]);

        let vec_2 = vec_1 / 2.0;

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec_2[0] - ans[0]).abs() < epsilon
                && (vec_2[1] - ans[1]).abs() < epsilon
                && (vec_2[2] - ans[2]).abs() < epsilon
        );
    }

    #[test]
    fn test_sub() {
        let vec_1 = Vec3::from_slice([1.5, 2.5, 3.5]);
        let vec_2 = Vec3::from_slice([1.0, 2.0, 3.0]);
        let ans = Vec3::from_slice([0.5, 0.5, 0.5]);

        let vec_3 = vec_1 - vec_2;

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec_3[0] - ans[0]).abs() < epsilon
                && (vec_3[1] - ans[1]).abs() < epsilon
                && (vec_3[2] - ans[2]).abs() < epsilon
        );
    }

    #[test]
    fn test_add_asign() {
        let mut vec_1 = Vec3::from_slice([1.0, 2.0, 3.0]);
        let vec_2 = Vec3::from_slice([1.5, 2.5, 3.5]);
        let ans = Vec3::from_slice([2.5, 4.5, 6.5]);

        println!("vec_1: {:?}", vec_1);
        vec_1 += vec_2;

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec_1[0] - ans[0]).abs() < epsilon
                && (vec_1[1] - ans[1]).abs() < epsilon
                && (vec_1[2] - ans[2]).abs() < epsilon
        );
    }

    #[test]
    fn test_mul_asign() {
        let mut vec_1 = Vec3::from_slice([1.0, 2.0, 3.0]);
        let ans = Vec3::from_slice([2.0, 4.0, 6.0]);
        vec_1 *= 2.0;

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec_1[0] - ans[0]).abs() < epsilon
                && (vec_1[1] - ans[1]).abs() < epsilon
                && (vec_1[2] - ans[2]).abs() < epsilon
        );
    }

    #[test]
    fn test_div_asign() {
        let mut vec_1 = Vec3::from_slice([2.0, 4.0, 6.0]);
        let ans = Vec3::from_slice([1.0, 2.0, 3.0]);
        vec_1 /= 2.0;

        let epsilon = std::f64::EPSILON;
        assert!(
            (vec_1[0] - ans[0]).abs() < epsilon
                && (vec_1[1] - ans[1]).abs() < epsilon
                && (vec_1[2] - ans[2]).abs() < epsilon
        );
    }

    #[test]
    fn test_dot() {
        let vec_1 = Vec3::from_slice([1.0, 2.0, 3.0]);
        let vec_2 = Vec3::from_slice([2.0, 4.0, 6.0]);
        let ans = 28.0;

        let dot_product_1 = vec_1.dot(&vec_2);
        let dot_product_2 = dot(&vec_1, &vec_2);

        let epsilon = std::f64::EPSILON;
        assert!((ans - dot_product_1).abs() < epsilon);
        assert!((ans - dot_product_2).abs() < epsilon);
    }

    #[test]
    fn test_random() {
        let vec_random_01 = Vec3::random();
        for v in vec_random_01.e {
            assert!(v >= 0.0 && v < 1.0);
        }

        let (min, max) = (1.5, 5.6);
        let vec_random_range = Vec3::random_by_range(min, max);
        for v in vec_random_range.e {
            assert!(v >= min && v < max);
        }
    }

    #[test]
    fn test_random_in_unit_sphere() {
        let vec_in_unit_sphere = Vec3::random_in_unit_sphere();
        assert!(vec_in_unit_sphere.norm_squared() < 1.0);

        let vec_to_unit_sphere = Vec3::random_unit_vector();
        assert!((vec_to_unit_sphere.norm_squared() - 1.0).abs() < std::f64::EPSILON);
    }
}
