#![warn(clippy::all)]
mod rays;
mod vectors;

use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use rays::{Color, Ray};
use vectors::{Point3, Vec3};

const FILENAME: &str = "output/test.png";

fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let start_value = Color::new(1.0, 1.0, 1.0);
    let end_value = Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * start_value + t * end_value
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 400_u32;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    // Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Point3::<f32>::default();
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Progress bar init
    let progress = ProgressBar::new(HEIGHT.into());
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:80.white/white} {pos:>7}/{len:7} {msg}"),
    );

    println!("\nProcessing lines...\n");

    for y in (0..HEIGHT).rev() {
        progress.inc(1);
        for x in 0..WIDTH {
            let u = x as f32 / (WIDTH - 1) as f32;
            let v = y as f32 / (HEIGHT - 1) as f32;

            let gradient_direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let bg_ray = Ray::new(origin, gradient_direction);

            img.put_pixel(x, HEIGHT - 1 - y, ray_color(bg_ray).into());
        }
    }

    progress.finish_with_message("Done!");

    println!("\nSaving image...");

    img.save(FILENAME).unwrap();

    println!("Finished!");
}
