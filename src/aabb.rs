use std::mem::swap;

use crate::{rays::Ray, Point3};

#[derive(Clone, Copy)]
pub struct AAAB {
    min: Point3<f32>,
    max: Point3<f32>,
}

impl AAAB {
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> Point3<f32> {
        self.min
    }

    pub fn max(&self) -> Point3<f32> {
        self.max
    }

    pub fn is_in(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for dim in 0..3 {
            let inv_d = 1.0 / ray.direction().get(dim).unwrap();
            let mut t0 = (self.min().get(dim).unwrap() - ray.origin().get(dim).unwrap()) * inv_d;
            let mut t1 = (self.max().get(dim).unwrap() - ray.origin().get(dim).unwrap()) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_min >= t_max {
                return false;
            }
        }

        true
    }

    pub fn new_surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Point3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );

        let big = Point3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );

        Self::new(small, big)
    }
}
