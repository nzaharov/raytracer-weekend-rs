#![warn(clippy::all)]
mod rays;
mod vectors;

use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use rays::{Color, Ray};
use vectors::{Point3, Vec3};

const FILENAME: &str = "output/test.png";

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

            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, direction);

            img.put_pixel(x, HEIGHT - 1 - y, ray_color(ray).into());
        }
    }

    progress.finish_with_message("Done!");

    println!("\nSaving image...");

    img.save(FILENAME).unwrap();

    println!("Finished!");
}

fn ray_color(ray: Ray) -> Color {
    let hit_location = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray);

    if hit_location > 0.0 {
        let normal: Vec3<f32> = ray.at(hit_location) - Vec3::new(0.0, 0.0, -1.0);
        let unit_normal = normal.unit_vector();
        // normalize to [0,1]
        let normalized: Vec3<f32> = 0.5 * (unit_normal + 1.0);

        return normalized;
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let start_value = Color::new(1.0, 1.0, 1.0);
    let end_value = Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * start_value + t * end_value
}

fn hit_sphere(center: Point3<f32>, radius: f32, ray: &Ray) -> f32 {
    // t2b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r2=0
    let oc: Vec3<f32> = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let half_b = oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    (-half_b - discriminant.sqrt()) / a
}
