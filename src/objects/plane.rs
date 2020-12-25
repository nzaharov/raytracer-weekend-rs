use crate::aabb::AAAB;
use crate::{
    hit::{Hit, Hittable},
    materials::Material,
    rays::Ray,
    vectors::{Point3, Vec3},
};
use std::sync::Arc;

pub struct Plane {
    pub p1: Point3<f32>,
    pub p2: Point3<f32>,
    pub normal: Vec3<f32>,
    pub material: Arc<dyn Material>,
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let denominator = ray.direction().dot(&self.normal);
        if denominator == 0.0 {
            // ray is parallel to the plane
            return None;
        }

        let t = ((self.p2 - ray.origin()).dot(&self.normal)) / denominator;

        if t < t_min || t > t_max {
            // outside of line of sight
            return None;
        }

        Some(Hit::new(
            ray.at(t),
            t,
            self.material.clone(),
            &ray,
            &self.normal,
        ))
    }

    fn get_b_box(&self, _time0: f32, _time1: f32) -> Option<AAAB> {
        Some(AAAB::new(self.p1, self.p1))
    }
}
