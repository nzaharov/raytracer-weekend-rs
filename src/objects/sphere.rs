use crate::rays::*;
use crate::vectors::*;
use crate::{hit::*, materials::Material};
use std::rc::Rc;

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc: Vec3<f32> = ray.origin() - self.center;
        let a = ray.direction().norm_sqr();
        let half_b = oc.dot(&ray.direction());
        let c = oc.norm_sqr() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_descriminant = discriminant.sqrt();

        let mut root = (-half_b - sqrt_descriminant) / a; // root1
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_descriminant) / a; // root2
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        let hit = Hit::new(hit_point, root, self.material.to_owned(), &ray, &outward_normal);

        Some(hit)
    }
}
