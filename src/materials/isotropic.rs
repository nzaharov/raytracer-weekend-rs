use crate::{
    hit::Hit,
    rays::{Color, Ray},
    textures::{solid_color::SolidColor, Texture, TextureImpl},
    vectors::Vec3,
};

use super::MaterialImpl;

#[derive(Clone)]
pub struct Isotropic {
    albedo: Texture,
}

impl Isotropic {
    pub fn with_color(color: Color) -> Self {
        Self {
            albedo: SolidColor::from(color).into(),
        }
    }
}

impl MaterialImpl for Isotropic {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let scattered = Ray::new(hit.point, Vec3::random_in_unit_sphere(), ray.time());
        let attenuation = self.albedo.value(hit.u, hit.v, &hit.point);
        Some((scattered, attenuation))
    }
}
