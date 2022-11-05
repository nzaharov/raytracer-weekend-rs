pub mod checker;
pub mod noise;
mod perlin;
pub mod solid_color;

use enum_dispatch::enum_dispatch;

use crate::{rays::Color, vectors::Point3};
use checker::*;
use noise::*;
use solid_color::*;

#[enum_dispatch]
pub trait TextureImpl {
    fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> Color;
}

#[enum_dispatch(TextureImpl)]
pub enum Texture {
    SolidColor,
    Checker,
    Noise,
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        match self {
            Self::SolidColor(arg0) => Self::SolidColor(arg0.clone()),
            Self::Checker(arg0) => Self::Checker(arg0.clone()),
            Self::Noise(arg0) => Self::Noise(arg0.clone()),
        }
    }
}
