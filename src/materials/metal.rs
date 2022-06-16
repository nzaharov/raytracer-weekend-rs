use crate::{
    materials::Hit,
    rays::{Color, Ray},
    vectors::Vec3,
};

use super::MaterialImpl;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(-1.0, 1.0),
        }
    }
}

impl MaterialImpl for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(&hit.normal);
        let fuzz = if self.fuzz == 0.0 {
            Vec3::default()
        } else {
            self.fuzz * Vec3::random_in_unit_sphere()
        };
        let scattered_ray = Ray::new(hit.point, reflected + fuzz, ray.time());

        Some((scattered_ray, self.albedo))
    }
}
