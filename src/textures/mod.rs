pub mod solid_color;

use crate::{rays::Color, vectors::Point3};

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> Color;
}
