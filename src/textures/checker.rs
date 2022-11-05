use crate::{rays::Color, vectors::Point3};

use super::{solid_color::SolidColor, TextureImpl};

#[derive(Clone, Copy)]
pub struct Checker {
    even: SolidColor,
    odd: SolidColor,
}

impl Checker {
    pub fn from_colors(odd: Color, even: Color) -> Self {
        Self {
            even: SolidColor::from(even).into(),
            odd: SolidColor::from(odd).into(),
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
