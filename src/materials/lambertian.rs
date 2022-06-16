use std::sync::Arc;

use crate::{
    materials::Hit,
    materials::MaterialImpl,
    rays::{Color, Ray},
    textures::{solid_color::SolidColor, Texture, TextureImpl},
    vectors::Vec3,
};

pub struct Lambertian {
    albedo: Arc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from(albedo).into()),
        }
    }

    pub fn with_texture(texture: Arc<Texture>) -> Self {
        Self { albedo: texture }
    }
}

impl MaterialImpl for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        // Alternative diffusion with Vec3::random_in_hemisphere is a bit faster
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        // Catch degenerate scatter direction (->0)
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered_ray = Ray::new(hit.point, scatter_direction, ray.time());

        Some((scattered_ray, self.albedo.value(hit.u, hit.v, &hit.point)))
    }
}
