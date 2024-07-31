use crate::hittable_material::{HitRecordMat, HittableMat};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SphereMat {
    center: Point3,
    radius: f64,
    mat: Option<Rc<RefCell<dyn Material>>>,
}

impl SphereMat {
    pub fn new(center: &Point3, radius: f64, mat: Option<Rc<RefCell<dyn Material>>>) -> Self {
        Self {
            center: center.clone(),
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl HittableMat for SphereMat {
    fn hit_mat(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecordMat) -> bool {
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
        rec.mat = self.mat.clone();

        true
    }
}
