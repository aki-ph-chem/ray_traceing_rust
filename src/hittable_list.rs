use crate::hittable::{HitRecord, Hittable, HittableV2};
use crate::hittable_material::{HitRecordMat, HittableMat};
use crate::interval::Interval;
use crate::ray::Ray;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HittableList<T> {
    pub objects: Vec<Rc<RefCell<T>>>,
}

impl<T> HittableList<T> {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn from_object(object: Rc<RefCell<T>>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<RefCell<T>>) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closet_so_far = ray_tmax;

        for object in &self.objects {
            if object
                .as_ref()
                .borrow()
                .hit(&ray, ray_tmin, closet_so_far, &mut tmp_rec)
            {
                hit_anything = true;
                closet_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }

        hit_anything
    }
}

impl<T: HittableV2> HittableV2 for HittableList<T> {
    fn hit_v2(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closet_so_far = ray_t.max;

        for object in &self.objects {
            if object.as_ref().borrow().hit_v2(
                &ray,
                Interval::new_by_value(ray_t.min, closet_so_far),
                &mut tmp_rec,
            ) {
                hit_anything = true;
                closet_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }

        hit_anything
    }
}
impl<T: HittableMat> HittableMat for HittableList<T> {
    fn hit_mat(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecordMat) -> bool {
        let mut tmp_rec = HitRecordMat::new();
        let mut hit_anything = false;
        let mut closet_so_far = ray_t.max;

        for object in &self.objects {
            if object.as_ref().borrow().hit_mat(
                &ray,
                Interval::new_by_value(ray_t.min, closet_so_far),
                &mut tmp_rec,
            ) {
                hit_anything = true;
                closet_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }

        hit_anything
    }
}
