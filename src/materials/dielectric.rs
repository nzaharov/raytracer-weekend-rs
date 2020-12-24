use rand::thread_rng;

use crate::materials::Hit;
use crate::Color;
use crate::Material;
use crate::Ray;
use crate::Rng;

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    fn get_reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
        // Schlick
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.is_front_facing {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = ray.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let should_reflect =
            Self::get_reflectance(cos_theta, refraction_ratio) > thread_rng().gen();

        let direction = if cannot_refract || should_reflect {
            unit_direction.reflect(&hit.normal)
        } else {
            unit_direction.refract(&hit.normal, refraction_ratio)
        };

        let scattered_ray = Ray::new(hit.point, direction, ray.time());

        Some((scattered_ray, attenuation))
    }
}
