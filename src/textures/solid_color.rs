use crate::{rays::Color, vectors::Point3};

use super::TextureImpl;

#[derive(Clone, Copy)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    #[allow(dead_code)]
    fn new(r: f32, g: f32, b: f32) -> Self {
        Color::new(r, g, b).into()
    }
}

impl From<Color> for SolidColor {
    fn from(c: Color) -> Self {
        Self { color_value: c }
    }
}

impl TextureImpl for SolidColor {
    fn value(&self, _: f32, _: f32, _: &Point3<f32>) -> Color {
        self.color_value
    }
}
