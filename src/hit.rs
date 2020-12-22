use crate::rays::Ray;
use crate::vectors::{Point3, Vec3};

pub struct Hit {
    pub point: Point3<f32>,
    pub normal: Vec3<f32>,
    pub t: f32,
    pub is_front_facing: bool,
}

impl Hit {
    pub fn new(point: Point3<f32>, t: f32, ray: &Ray, outward_normal: &Vec3<f32>) -> Self {
        let is_front_facing = ray.direction().dot(outward_normal) < 0.0;
        let normal = match is_front_facing {
            true => outward_normal.to_owned(),
            false => -outward_normal.to_owned(),
        };

        Self {
            point,
            t,
            normal,
            is_front_facing,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
