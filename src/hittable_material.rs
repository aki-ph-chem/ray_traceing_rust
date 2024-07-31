use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecordMat {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<RefCell<dyn Material>>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecordMat {
    pub fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            mat: None,
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

pub trait HittableMat {
    fn hit_mat(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecordMat) -> bool;
}
