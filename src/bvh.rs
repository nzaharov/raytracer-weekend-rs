use crate::hit::HitList;
use crate::{aabb::AAAB, hit::Hittable};
use crate::{hit::Hit, rays::Ray};
use rand::{thread_rng, Rng};
use std::{cmp::Ordering, sync::Arc};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    b_box: AAAB,
}

unsafe impl Send for BVHNode {}
unsafe impl Sync for BVHNode {}

impl BVHNode {
    pub fn new(list: &mut HitList, time0: f32, time1: f32) -> Self {
        Self::init(list, 0, list.len(), time0, time1)
    }

    pub fn init(objects: &mut HitList, start: usize, end: usize, time0: f32, time1: f32) -> Self {
        let axis = thread_rng().gen_range(0..=2);

        let object_span = end - start;

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if object_span == 1 {
            left = objects.get(start).unwrap().clone();
            right = left.clone();
        } else if object_span == 2 {
            let obj_a = objects.get(start).unwrap();
            let obj_b = objects.get(start + 1).unwrap();
            if Self::box_compare(obj_a, obj_b, axis) {
                left = obj_a.clone();
                right = obj_b.clone();
            } else {
                left = obj_b.clone();
                right = obj_a.clone();
            };
        } else {
            objects.sort_by(|a, b| match Self::box_compare(a, b, axis) {
                true => Ordering::Less,
                false => Ordering::Greater,
            });

            let middle = start + object_span / 2;
            left = Arc::new(Self::init(objects, start, middle, time0, time1));
            right = Arc::new(Self::init(objects, middle, end, time0, time1));
        }

        let box_left = left
            .get_b_box(time0, time1)
            .expect("No bounding box in node");
        let box_right = right
            .get_b_box(time0, time1)
            .expect("No bounding box in node");
        let b_box = AAAB::new_surrounding_box(box_left, box_right);

        Self { left, right, b_box }
    }

    fn box_compare(obj_a: &Arc<dyn Hittable>, obj_b: &Arc<dyn Hittable>, axis: u32) -> bool {
        let box_a = obj_a.get_b_box(0.0, 0.0).expect("No bounding box in node");
        let box_b = obj_b.get_b_box(0.0, 0.0).expect("No bounding box in node");

        box_a.min().get(axis) < box_b.min().get(axis)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if !self.b_box.is_in(ray, t_min, t_max) {
            return None;
        }

        let left_hit = self.left.hit(ray, t_min, t_max);
        let t_max = match left_hit {
            Some(ref hit) => hit.t,
            None => t_max,
        };
        let right_hit = self.right.hit(ray, t_min, t_max);

        if right_hit.is_some() {
            right_hit
        } else {
            left_hit
        }
    }

    fn get_b_box(&self, _time0: f32, _time1: f32) -> Option<AAAB> {
        Some(self.b_box)
    }
}
