use crate::aabb::AaBb;
use crate::hittable_list_aabb::HittableListAaBb;
use crate::hittable_material::HittableAaBb;
use crate::interval::Interval;
use crate::utl;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BvhNode<T> {
    left: Option<Rc<RefCell<T>>>,
    right: Option<Rc<RefCell<T>>>,
    bbox: AaBb,
}

impl<T: HittableAaBb> BvhNode<T> {
    pub fn new(mut list: HittableListAaBb<T>) -> Self {
        let n = list.objects.len();
        Self::new_by_objects(&mut list.objects, 0, n)
    }

    pub fn new_by_objects(objects: &mut Vec<Rc<RefCell<T>>>, start: usize, end: usize) -> Self {
        let mut bvhnode = Self {
            left: None,
            right: None,
            bbox: AaBb::new(),
        };

        let axis = utl::random_i32(0.0, 2.0);
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;
        match object_span {
            1 => {
                bvhnode.left = Some(objects[start].clone());
                bvhnode.right = Some(objects[start].clone());
            }
            2 => {
                bvhnode.left = Some(objects[start].clone());
                bvhnode.right = Some(objects[start + 1].clone());
            }
            _ => {
                objects[start..end].sort_by(|a, b| comparator(a, b).unwrap());

                let mid = start + object_span / 2;
                /*
                bvhnode.left = Some(Rc::new(RefCell::new(Self::new_by_objects(
                    objects, start, mid,
                ))));
                bvhnode.right = Some(Rc::new(RefCell::new(Self::new_by_objects(
                    objects, mid, end,
                ))));
                */
            }
        }
        bvhnode.bbox = AaBb::new_by_two_aabb(
            &bvhnode
                .left
                .clone()
                .unwrap()
                .as_ref()
                .borrow()
                .bounding_box(),
            &bvhnode
                .right
                .clone()
                .unwrap()
                .as_ref()
                .borrow()
                .bounding_box(),
        );

        bvhnode
    }

    fn box_compare(a: &Rc<RefCell<T>>, b: &Rc<RefCell<T>>, axis_inex: usize) -> Option<Ordering> {
        let a_axis_interval = a
            .as_ref()
            .borrow()
            .bounding_box()
            .axis_interval(axis_inex)
            .clone();
        let b_axis_interval = b
            .as_ref()
            .borrow()
            .bounding_box()
            .axis_interval(axis_inex)
            .clone();

        a_axis_interval.min.partial_cmp(&b_axis_interval.min)
    }

    fn box_x_compare(a: &Rc<RefCell<T>>, b: &Rc<RefCell<T>>) -> Option<Ordering> {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<RefCell<T>>, b: &Rc<RefCell<T>>) -> Option<Ordering> {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<RefCell<T>>, b: &Rc<RefCell<T>>) -> Option<Ordering> {
        Self::box_compare(a, b, 2)
    }
}

impl<T: HittableAaBb> HittableAaBb for BvhNode<T> {
    fn hit_aabb(
        &self,
        ray: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable_material::HitRecordMat,
    ) -> bool {
        if !self.bbox.hit(ray, ray_t.clone()) {
            return false;
        }

        let hit_left =
            self.left
                .clone()
                .unwrap()
                .as_ref()
                .borrow()
                .hit_aabb(&ray, ray_t.clone(), rec);
        let hit_right = self.left.clone().unwrap().as_ref().borrow().hit_aabb(
            &ray,
            Interval::new_by_value(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AaBb {
        self.bbox.clone()
    }
}
