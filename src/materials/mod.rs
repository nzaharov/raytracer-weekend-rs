pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::hit::Hit;
use crate::rays::Ray;
use crate::Color;
use crate::Point3;

pub trait Material {
    fn emit(&self, _u: f32, _v: u32, _p: &Point3<f32>) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}
