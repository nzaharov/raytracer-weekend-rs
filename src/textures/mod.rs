pub mod solid_color;

use enum_dispatch::enum_dispatch;

use crate::{rays::Color, vectors::Point3};
use solid_color::*;

#[enum_dispatch]
pub trait TextureImpl: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> Color;
}

#[derive(Clone, Copy)]
#[enum_dispatch(TextureImpl)]
pub enum Texture {
    SolidColor,
}
