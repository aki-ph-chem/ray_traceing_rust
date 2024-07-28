use crate::hittable::{HitRecord, Hittable};
use std::cell::RefCell;
use std::rc::Rc;

struct HittableList<T: Hittable> {
    pub objects: Vec<RefCell<Rc<T>>>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn from_object(object: RefCell<Rc<T>>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: RefCell<Rc<T>>) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut HitRecord,
    ) -> bool {
        let mut tmp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closet_so_far = ray_tmax;

        for object in &self.objects {
            if object
                .borrow()
                .as_ref()
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
