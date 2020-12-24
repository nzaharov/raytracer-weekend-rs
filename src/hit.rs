use crate::vectors::{Point3, Vec3};
use crate::{materials::Material, rays::Ray};
use std::rc::Rc;

pub struct Hit {
    pub point: Point3<f32>,
    pub normal: Vec3<f32>,
    pub t: f32,
    pub is_front_facing: bool,
    pub material: Rc<dyn Material>,
}

impl Hit {
    pub fn new(
        point: Point3<f32>,
        t: f32,
        material: Rc<dyn Material>,
        ray: &Ray,
        outward_normal: &Vec3<f32>,
    ) -> Self {
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
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct HitList(Vec<Rc<dyn Hittable>>);

impl HitList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.0.push(obj);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Hittable for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_hit_t = t_max;
        let mut current_hit: Option<Hit> = None;

        for obj in self.0.iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest_hit_t) {
                closest_hit_t = hit.t;
                current_hit = Some(hit);
            }
        }

        current_hit
    }
}
