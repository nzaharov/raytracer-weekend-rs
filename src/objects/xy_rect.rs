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
pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32, // z axis position
    pub material: Arc<Material>,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Arc<Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl HittableImpl for XYRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let y = ray.origin().y() + t * ray.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
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
        let p1 = Point3::new(self.x0, self.y0, self.k - PADDING);
        let p2 = Point3::new(self.x1, self.y1, self.k + PADDING);
        Some(AABB::new(p1, p2))
    }
}
