use crate::hittable::{HitRecord, Hittable, HittableV2};
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Self {
        Self {
            center: center.clone(),
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = self.center.clone() - ray.origin().clone();
        let a = ray.direction().norm_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        // find the nearest root that lies in the acceptable range.
        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrt_d) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p.clone() - self.center.clone()) / self.radius;
        rec.set_face_normal(&ray, &outward_normal);

        true
    }
}

impl HittableV2 for Sphere {
    fn hit_v2(
        &self,
        ray: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = self.center.clone() - ray.origin().clone();
        let a = ray.direction().norm_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        // find the nearest root that lies in the acceptable range.
        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p.clone() - self.center.clone()) / self.radius;
        rec.set_face_normal(&ray, &outward_normal);

        true
    }
}
