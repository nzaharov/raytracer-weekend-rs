#![warn(clippy::all)]
mod camera;
mod hit;
mod materials;
mod objects;
mod rays;
mod vectors;

use std::f32::INFINITY;

use camera::Camera;
use hit::{HitList, Hittable};
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
use objects::sphere::Sphere;
use rand::{prelude::ThreadRng, thread_rng, Rng};
use rays::{Color, Ray};
use vectors::{Point3, Vec3};

const FILENAME: &str = "output/test.png";
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const SAMPLE_SIZE: u32 = 100;
const MAX_DEPTH: u32 = 50;
const BIAS: f32 = 0.001;

fn main() {
    let mut rng = thread_rng();
    // Start timer
    let now = std::time::Instant::now();

    // Image
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    // Scene
    let mut scene = HitList::new();

    let mat_1 = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let sphere1 = Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: &mat_1,
    };
    let mat_ground = Metal::new(Color::new(0.9, 0.1, 0.1), 0.5);
    let ground = Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: &mat_ground,
    };
    let mat_2 = Lambertian::new(Color::default());
    let sphere2 = Sphere {
        center: Point3::new(1.0, 0.0, -2.0),
        radius: 0.5,
        material: &mat_2,
    };
    let mat_metal = Metal::new(Color::new(0.8, 0.8, 0.8), 0.0);
    let metal = Sphere {
        center: Point3::new(-2.0, 0.0, -1.5),
        radius: 0.5,
        material: &mat_metal,
    };
    let glass_mat = Dielectric::new(1.5);
    let crystal_ball = Sphere {
        center: Point3::new(0.27, 0.1, -0.5),
        radius: -0.05,
        material: &glass_mat,
    };
    scene.add(&sphere1);
    scene.add(&ground);
    scene.add(&sphere2);
    scene.add(&metal);
    scene.add(&crystal_ball);

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
                color += raytrace(ray, &scene, MAX_DEPTH, &mut rng);
            }

            let color = calculate_pixel_color(color, SAMPLE_SIZE);

            img.put_pixel(x, HEIGHT - 1 - y, color);
        }
    }

    progress.finish_with_message("Done!");

    println!("\nSaving image...");

    img.save(FILENAME).expect("Could not save image");

    println!("Finished in {} ms", now.elapsed().as_millis());
}

fn calculate_pixel_color(color: Color, sample_size: u32) -> Rgb<u8> {
    let scale = 1.0 / sample_size as f32;
    let (r, g, b) = (color.x(), color.y(), color.z());

    // Color correction (gamma=2.0)
    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();

    Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ])
}

fn raytrace(ray: Ray, scene: &HitList, depth: u32, rng: &mut ThreadRng) -> Color {
    // Color map (TODO: extract as material)
    // if let Some(hit) = scene.hit(&ray, 0.0, INFINITY) {
    //     return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
    // }

    if depth == 0 {
        return Color::default();
    }

    if let Some(hit) = scene.hit(&ray, 0.0 + BIAS, INFINITY) {
        return match hit.material.scatter(&ray, &hit, rng) {
            Some((scattered_ray, attenuatuion)) => {
                attenuatuion * raytrace(scattered_ray, scene, depth - 1, rng)
            }
            None => Color::default(),
        };
    }

    // Background sky gradient
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let start_value = Color::new(1.0, 1.0, 1.0);
    let end_value = Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * start_value + t * end_value
}

// temporary implementation until the stabilized clamp is released
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
