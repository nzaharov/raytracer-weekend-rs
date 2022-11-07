use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::{
    aabb::AABB,
    hit::{Hit, Hittable, HittableImpl},
    materials::{isotropic::Isotropic, Material},
    rays::{Color, Ray},
    vectors::Vec3,
};

pub struct ConstantMedium {
    boundary: Box<Hittable>,
    inv_neg_density: f32,
    phase_function: Arc<Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Hittable, density: f32, color: Color) -> Self {
        Self {
            boundary: boundary.into(),
            inv_neg_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::with_color(color).into()),
        }
    }
}

impl HittableImpl for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut hit1 = self.boundary.hit(ray, -f32::INFINITY, f32::INFINITY)?;
        let mut hit2 = self.boundary.hit(ray, hit1.t + 0.0001, f32::INFINITY)?;

        if hit1.t < t_min {
            hit1.t = t_min
        }
        if hit2.t > t_max {
            hit2.t = t_max
        }

        if hit1.t >= hit2.t {
            return None;
        }

        if hit1.t < 0.0 {
            hit1.t = 0.0;
        }

        let ray_len = ray.direction().norm();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_len;
        let hit_distance = self.inv_neg_density * thread_rng().gen::<f32>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_len;
        let point = ray.at(t);
        let normal = Vec3::new(1.0, 0.0, 0.0);

        Some(Hit::new(
            point,
            t,
            0.0,
            0.0,
            self.phase_function.clone(),
            ray,
            &normal,
        ))
    }

    fn get_b_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.boundary.get_b_box(time0, time1)
    }
}
