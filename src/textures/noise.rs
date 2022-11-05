use crate::{rays::Color, vectors::Point3};

use super::perlin::Perlin;
use super::TextureImpl;

#[derive(Default, Clone)]
pub struct Noise(Perlin);

impl TextureImpl for Noise {
    fn value(&self, _u: f32, _v: f32, p: &Point3<f32>) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.0.noise(p)
    }
}
