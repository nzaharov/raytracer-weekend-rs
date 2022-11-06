// Copy paste from xy_rect.rs

use std::sync::Arc;

use crate::{
    aabb::AABB,
    hit::{Hit, HittableImpl},
    materials::Material,
    rays::Ray,
    vectors::{Point3, Vec3},
};

const PADDING: f32 = 0.0001;

#[derive(Clone)]
pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32, // y axis position
    pub material: Arc<Material>,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Arc<Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl HittableImpl for XZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let z = ray.origin().z() + t * ray.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let p = ray.at(t);

        Some(Hit::new(
            p,
            t,
            u,
            v,
            self.material.clone(),
            ray,
            &outward_normal,
        ))
    }

    fn get_b_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Y
        // dimension a small amount.
        let p1 = Point3::new(self.x0, self.k - PADDING, self.z0);
        let p2 = Point3::new(self.x1, self.k + PADDING, self.z1);
        Some(AABB::new(p1, p2))
    }
}
