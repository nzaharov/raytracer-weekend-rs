use crate::Color;
use crate::Material;
use crate::Ray;
use crate::ThreadRng;
use crate::Vec3;
use crate::{clamp, materials::Hit};

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: clamp(fuzz, -1.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(&hit.normal);
        let fuzz = if self.fuzz == 0.0 {
            Vec3::default()
        } else {
            self.fuzz * Vec3::random_in_unit_sphere(rng)
        };
        let scattered_ray = Ray::new(hit.point, reflected + fuzz);

        Some((scattered_ray, self.albedo))
    }
}
