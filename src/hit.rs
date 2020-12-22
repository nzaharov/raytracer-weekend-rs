use crate::rays::Ray;
use crate::vectors::{Point3, Vec3};

pub struct Hit {
    pub point: Point3<f32>,
    pub normal: Vec3<f32>,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
