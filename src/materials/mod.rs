pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

use enum_dispatch::enum_dispatch;

use crate::rays::Ray;
use crate::{hit::Hit, rays::Color, vectors::Point3};
use dielectric::*;
use diffuse_light::*;
use lambertian::*;
use metal::*;

#[enum_dispatch]
pub trait MaterialImpl {
    fn emit(&self, _u: f32, _v: f32, _p: &Point3<f32>) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}

#[enum_dispatch(MaterialImpl)]
pub enum Material {
    Dielectric,
    Lambertian,
    Metal,
    DiffuseLight,
}

impl Clone for Material {
    fn clone(&self) -> Self {
        match self {
            Self::Dielectric(arg0) => Self::Dielectric(*arg0),
            Self::Lambertian(arg0) => Self::Lambertian(arg0.clone()),
            Self::Metal(arg0) => Self::Metal(*arg0),
            Self::DiffuseLight(arg0) => Self::DiffuseLight(arg0.clone()),
        }
    }
}
