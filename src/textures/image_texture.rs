use std::path::Path;

use image::{io::Reader, ImageResult, RgbImage};

use crate::{rays::Color, vectors::Point3};

use super::TextureImpl;

#[derive(Clone)]
pub struct ImageTexture {
    img: RgbImage,
}

impl ImageTexture {
    pub fn load_from_file<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let img = Reader::open(path)?.decode()?;

        Ok(Self {
            img: img.into_rgb8(),
        })
    }
}

impl TextureImpl for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Point3<f32>) -> Color {
        let width = self.img.width();
        let height = self.img.height();

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let i = (u * width as f32) as u32;
        let j = (v * height as f32) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        let x = if i >= width { width - 1 } else { i };
        let y = if j >= height { height - 1 } else { j };
        let pixel = self.img.get_pixel(x, y);

        Color::new(
            pixel.0[0] as f32 / 255.0,
            pixel.0[1] as f32 / 255.0,
            pixel.0[2] as f32 / 255.0,
        )
    }
}
