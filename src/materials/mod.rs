pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::hit::Hit;
use crate::rays::Ray;
use crate::Color;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}
