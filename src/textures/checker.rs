use crate::{rays::Color, vectors::Point3};

use super::{solid_color::SolidColor, Texture, TextureImpl};

#[derive(Clone)]
pub struct Checker {
    even: Box<Texture>,
    odd: Box<Texture>,
}

impl Checker {
    pub fn from_colors(odd: Color, even: Color) -> Self {
        Self {
            even: Box::new(SolidColor::from(even).into()),
            odd: Box::new(SolidColor::from(odd).into()),
        }
    }
}

impl TextureImpl for Checker {
    fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
