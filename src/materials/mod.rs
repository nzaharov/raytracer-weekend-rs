pub mod lambertian;

use rand::prelude::ThreadRng;

use crate::hit::Hit;
use crate::rays::Ray;
use crate::Color;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit<impl Material>,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)>;
}
