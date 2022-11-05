use crate::{rays::Color, vectors::Point3};

use super::perlin::Perlin;
use super::TextureImpl;

#[derive(Default, Clone)]
pub struct Noise {
    noise: Perlin,
    scale: f32,
}

impl Noise {
    pub fn new(scale: f32) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
}

impl TextureImpl for Noise {
    fn value(&self, _u: f32, _v: f32, p: &Point3<f32>) -> Color {
        let scale = self.scale * *p;
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(&scale)
    }
}
