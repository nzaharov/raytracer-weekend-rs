use crate::materials::Hit;
use crate::Color;
use crate::Material;
use crate::Ray;
use crate::ThreadRng;
use crate::Vec3;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit: &Hit<impl Material>,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        // Alternative diffusion with Vec3::random_in_hemisphere is a bit faster
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector(rng);
        // Catch degenerate scatter direction (->0)
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered_ray = Ray::new(hit.point, scatter_direction);

        Some((scattered_ray, self.albedo))
    }
}
