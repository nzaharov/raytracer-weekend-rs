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
pub struct YZRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32, // x axis position
    pub material: Arc<Material>,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Arc<Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl HittableImpl for YZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - ray.origin().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin().y() + t * ray.direction().y();
        let z = ray.origin().z() + t * ray.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
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
        // The bounding box must have non-zero width in each dimension, so pad the X
        // dimension a small amount.
        let p1 = Point3::new(self.k - PADDING, self.y0, self.z0);
        let p2 = Point3::new(self.k - PADDING, self.y1, self.z1);
        Some(AABB::new(p1, p2))
    }
}
