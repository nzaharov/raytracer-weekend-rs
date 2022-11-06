use crate::{
    hit::Hit,
    rays::{Color, Ray},
    textures::{solid_color::SolidColor, Texture, TextureImpl},
    vectors::Point3,
};

use super::MaterialImpl;

pub struct DiffuseLight {
    emission: Texture,
}

impl DiffuseLight {
    pub fn with_color(color: Color) -> Self {
        Self {
            emission: SolidColor::from(color).into(),
        }
    }
}

impl MaterialImpl for DiffuseLight {
    fn emit(&self, u: f32, v: f32, p: &Point3<f32>) -> Color {
        self.emission.value(u, v, p)
    }

    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<(Ray, Color)> {
        None
    }
}
