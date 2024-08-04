use crate::aabb::AaBb;
use crate::hittable_material::{HitRecordMat, HittableAaBb, HittableMat};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::cell::RefCell;
use std::rc::Rc;

/// sphere with AaBb
pub struct SphereAaBb {
    center_1: Point3,
    radius: f64,
    mat: Option<Rc<RefCell<dyn Material>>>,
    is_moving: bool,
    center_vec: Vec3,
    hbox: AaBb,
}

impl SphereAaBb {
    pub fn new_stationary(
        center: &Point3,
        radius: f64,
        mat: Option<Rc<RefCell<dyn Material>>>,
    ) -> Self {
        let mut sphere = Self {
            center_1: center.clone(),
            radius: radius.max(0.0),
            mat,
            is_moving: false,
            center_vec: Point3::new(),
            hbox: AaBb::new(),
        };

        let rvec = Vec3::from_slice([radius, radius, radius]);
        sphere.hbox =
            AaBb::new_by_two_points(&(center.clone() - rvec.clone()), &(center.clone() + rvec));

        sphere
    }

    pub fn new_moving(
        center_1: &Point3,
        center_2: &Point3,
        radius: f64,
        mat: Option<Rc<RefCell<dyn Material>>>,
    ) -> Self {
        let mut sphere = Self {
            center_1: center_1.clone(),
            radius: radius.max(0.0),
            mat,
            is_moving: true,
            center_vec: center_2.clone() - center_1.clone(),
            hbox: AaBb::new(),
        };

        let rvec = Vec3::from_slice([radius, radius, radius]);
        let box_1 = AaBb::new_by_two_points(
            &(center_1.clone() - rvec.clone()),
            &(center_1.clone() + rvec.clone()),
        );
        let box_2 = AaBb::new_by_two_points(
            &(center_2.clone() - rvec.clone()),
            &(center_2.clone() + rvec.clone()),
        );
        sphere.hbox = AaBb::new_by_two_aabb(&box_1, &box_2);

        sphere
    }

    fn sphere_center(&self, time: f64) -> Point3 {
        self.center_1.clone() + time * self.center_vec.clone()
    }
}

impl HittableAaBb for SphereAaBb {
    fn hit_aabb(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecordMat) -> bool {
        let center = if self.is_moving {
            self.sphere_center(ray.time())
        } else {
            self.center_1.clone()
        };

        let oc = center.clone() - ray.origin().clone();
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
        let outward_normal = (rec.p.clone() - center.clone()) / self.radius;
        rec.set_face_normal(&ray, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> AaBb {
        self.hbox.clone()
    }
}
