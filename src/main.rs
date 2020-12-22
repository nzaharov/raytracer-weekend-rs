#![warn(clippy::all)]
use image::RgbImage;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const FILENAME: &str = "output/test.png";

fn main() {
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let r = x as f32 / (WIDTH - 1) as f32;
            let g = y as f32 / (HEIGHT - 1) as f32;
            let b = 0.25_f32;

            img.put_pixel(
                y,
                x,
                [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8].into(),
            );
        }
    }

    img.save(FILENAME).unwrap();
}
