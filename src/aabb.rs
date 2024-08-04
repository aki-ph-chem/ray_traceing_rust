use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

/// Axis-aligned bounding box
#[derive(Clone)]
pub struct AaBb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AaBb {
    /// initialize by default Interval values
    pub fn new() -> Self {
        Self {
            x: Interval::new(),
            y: Interval::new(),
            z: Interval::new(),
        }
    }

    /// initialize by tree Interval values x, y, z
    pub fn new_by_values(x: &Interval, y: &Interval, z: &Interval) -> Self {
        Self {
            x: x.clone(),
            y: y.clone(),
            z: z.clone(),
        }
    }

    /// initialize by two Point3 values a, b
    pub fn new_by_two_points(a: &Point3, b: &Point3) -> Self {
        let mut aabb = Self::new();

        aabb.x = if a[0] <= b[0] {
            Interval::new_by_value(a[0], b[0])
        } else {
            Interval::new_by_value(b[0], a[0])
        };
        aabb.y = if a[1] <= b[1] {
            Interval::new_by_value(a[1], b[1])
        } else {
            Interval::new_by_value(b[1], a[1])
        };
        aabb.z = if a[2] <= b[2] {
            Interval::new_by_value(a[2], b[2])
        } else {
            Interval::new_by_value(b[2], a[2])
        };

        aabb
    }

    pub fn new_by_two_aabb(box_0: &Self, box_1: &Self) -> Self {
        let mut aabb = Self::new();
        aabb.x = Interval::new_by_two_intervals(&box_0.x, &box_1.x);
        aabb.y = Interval::new_by_two_intervals(&box_0.y, &box_1.y);
        aabb.z = Interval::new_by_two_intervals(&box_0.z, &box_1.z);

        aabb
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_direction[axis];

            let (t_0, t_1) = (
                (ax.min - ray_origin[axis]) * adinv,
                (ax.max - ray_origin[axis]) * adinv,
            );

            if t_0 < t_1 {
                if t_0 > ray_t.min {
                    ray_t.min = t_0;
                }
                if t_1 < ray_t.max {
                    ray_t.max = t_1;
                }
            } else {
                if t_1 > ray_t.min {
                    ray_t.min = t_0;
                }
                if t_0 < ray_t.max {
                    ray_t.max = t_1;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
