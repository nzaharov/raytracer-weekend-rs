pub mod dielectric;
pub mod lambertian;
pub mod metal;

use enum_dispatch::enum_dispatch;

use crate::rays::Ray;
use crate::{hit::Hit, rays::Color, vectors::Point3};
use dielectric::*;
use lambertian::*;
use metal::*;

#[enum_dispatch]
pub trait MaterialImpl: Send + Sync {
    fn emit(&self, _u: f32, _v: u32, _p: &Point3<f32>) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}

#[enum_dispatch(MaterialImpl)]
pub enum Material {
    Dielectric,
    Lambertian,
    Metal,
}
