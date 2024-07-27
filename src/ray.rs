use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            orig: Point3::new(),
            dir: Vec3::new(),
        }
    }

    pub fn from_origin_dir(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + t * self.dir.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {
        let origin = Point3::from_slice([0.0, 0.0, 0.0]);
        let direction = Point3::from_slice([1.0, 1.0, 1.0]);
        let ray = Ray::from_origin_dir(&origin, &direction);

        let ans_2 = Point3::from_slice([2.0, 2.0, 2.0]);
        let ans_10 = Point3::from_slice([10.0, 10.0, 10.0]);

        let epsilon = std::f64::EPSILON;
        assert!((ans_2 - ray.at(2.0)).norm_squared() < epsilon);
        assert!((ans_10 - ray.at(10.0)).norm_squared() < epsilon);
    }
}
