use crate::materials::Hit;
use crate::Color;
use crate::Material;
use crate::Ray;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Option<(Ray, Color)> {
        todo!()
    }
}
