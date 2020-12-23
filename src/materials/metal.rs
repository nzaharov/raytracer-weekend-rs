use crate::materials::Hit;
use crate::Color;
use crate::Material;
use crate::Ray;
use crate::ThreadRng;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(&hit.normal);
        let scattered_ray = Ray::new(hit.point, reflected);

        Some((scattered_ray, self.albedo))
    }
}
