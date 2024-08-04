use crate::aabb::AaBb;
use crate::hittable_material::{HitRecordMat, HittableAaBb};
use crate::interval::Interval;
use crate::ray::Ray;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HittableListAaBb<T> {
    pub objects: Vec<Rc<RefCell<T>>>,
    bbox: AaBb,
}

impl<T: HittableAaBb> HittableListAaBb<T> {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: AaBb::new(),
        }
    }

    pub fn from_object(object: Rc<RefCell<T>>) -> Self {
        Self {
            objects: vec![object],
            bbox: AaBb::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<RefCell<T>>) {
        self.objects.push(object.clone());
        self.bbox = AaBb::new_by_two_aabb(&self.bbox, &object.as_ref().borrow().bounding_box());
    }
}

impl<T: HittableAaBb> HittableAaBb for HittableListAaBb<T> {
    fn hit_aabb(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecordMat) -> bool {
        let mut tmp_rec = HitRecordMat::new();
        let mut hit_anything = false;
        let mut closet_so_far = ray_t.max;

        for object in &self.objects {
            if object.as_ref().borrow().hit_aabb(
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

    fn bounding_box(&self) -> crate::aabb::AaBb {
        self.bbox.clone()
    }
}
