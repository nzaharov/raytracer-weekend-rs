use crate::aabb::AABB;
use crate::rays::*;
use crate::vectors::*;
use crate::{hit::*, materials::Material};
use std::f32::consts::PI;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl HittableImpl for Sphere {
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
        let (u, v) = get_sphere_uv(&outward_normal);

        let hit = Hit::new(
            hit_point,
            root,
            u,
            v,
            self.material.clone(),
            ray,
            &outward_normal,
        );

        Some(hit)
    }

    fn get_b_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

pub fn get_sphere_uv(p: &Point3<f32>) -> (f32, f32) {
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;
    (phi / (2.0 * PI), theta / PI)
}
