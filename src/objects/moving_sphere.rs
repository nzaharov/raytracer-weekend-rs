use crate::materials::Material;
use crate::{aabb::AAAB, vectors::Point3};
use crate::{
    hit::{Hit, Hittable},
    rays::Ray,
    vectors::Vec3,
};
use std::sync::Arc;

pub struct MovingSphere {
    pub center_start: Point3<f32>,
    pub center_end: Point3<f32>,
    pub time_start: f32,
    pub time_end: f32,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn center_at(&self, time: f32) -> Point3<f32> {
        let delta = (time - self.time_start) / (self.time_end - self.time_start);
        self.center_start + delta * (self.center_end - self.center_start)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc: Vec3<f32> = ray.origin() - self.center_at(ray.time());
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
        let outward_normal = (hit_point - self.center_at(ray.time())) / self.radius;
        let hit = Hit::new(
            hit_point,
            root,
            self.material.clone(),
            &ray,
            &outward_normal,
        );

        Some(hit)
    }

    fn get_b_box(&self, time0: f32, time1: f32) -> Option<AAAB> {
        let box0 = AAAB::new(
            self.center_at(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AAAB::new(
            self.center_at(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(AAAB::new_surrounding_box(box0, box1))
    }
}
