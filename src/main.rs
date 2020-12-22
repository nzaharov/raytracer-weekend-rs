#![warn(clippy::all)]
use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const FILENAME: &str = "output/test.png";

fn main() {
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    let progress = ProgressBar::new(HEIGHT.into());
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:80.cyan/blue} {pos:>7}/{len:7} {msg}"),
    );

    println!("Processing lines...\n");

    for y in (0..HEIGHT).rev() {
        progress.inc(1);
        for x in 0..WIDTH {
            let r = x as f32 / (WIDTH - 1) as f32;
            let g = y as f32 / (HEIGHT - 1) as f32;
            let b = 0.25_f32;

            img.put_pixel(
                y,
                x,
                [(256.0 * r) as u8, (256.0 * g) as u8, (256.0 * b) as u8].into(),
            );
        }
    }

    progress.finish_with_message("Done!");

    println!("\nSaving image...");

    img.save(FILENAME).unwrap();

    println!("Finished!");
}
