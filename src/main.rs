#![warn(clippy::all)]
mod camera;
mod hit;
mod objects;
mod rays;
mod vectors;

use std::f32::INFINITY;

use camera::Camera;
use hit::{HitList, Hittable};
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use objects::sphere::Sphere;
use rand::{thread_rng, Rng};
use rays::{Color, Ray};
use vectors::Point3;

const FILENAME: &str = "output/test.png";

fn main() {
    let mut rng = thread_rng();
    // Start timer
    let now = std::time::Instant::now();

    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLE_SIZE: u32 = 100;

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    // Scene
    let mut scene = HitList::new();
    let sphere1 = Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let sphere2 = Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };
    let sphere3 = Sphere {
        center: Point3::new(1.0, 0.0, -2.0),
        radius: 0.5,
    };
    scene.add(&sphere1);
    scene.add(&sphere2);
    scene.add(&sphere3);

    // Camera
    let camera = Camera::new();

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
            let mut color = Color::default();
            for _ in 0..SAMPLE_SIZE {
                let u = (x as f32 + rng.gen::<f32>()) / (WIDTH - 1) as f32;
                let v = (y as f32 + rng.gen::<f32>()) / (HEIGHT - 1) as f32;

                let ray = camera.get_ray(u, v);
                color += ray_color(ray, &scene);
            }

            let color = multisample_pixel(color, SAMPLE_SIZE);

            img.put_pixel(x, HEIGHT - 1 - y, color);
        }
    }

    progress.finish_with_message("Done!");

    println!("\nSaving image...");

    img.save(FILENAME).expect("Could not save image");

    println!("Finished in {} ms", now.elapsed().as_millis());
}

fn multisample_pixel(color: Color, sample_size: u32) -> Rgb<u8> {
    let scale = 1.0 / sample_size as f32;
    let color = color * scale;
    let (r, g, b) = (color.x(), color.y(), color.z());

    Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ])
}

fn ray_color(ray: Ray, scene: &HitList<impl Hittable>) -> Color {
    if let Some(hit) = scene.hit(&ray, 0.0, INFINITY) {
        return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let start_value = Color::new(1.0, 1.0, 1.0);
    let end_value = Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * start_value + t * end_value
}

// temporary implementation until the stabilized clamp is released
fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
