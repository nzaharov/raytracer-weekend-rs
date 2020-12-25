#![warn(clippy::all)]
mod aabb;
mod bvh;
mod camera;
mod hit;
mod materials;
mod objects;
mod rays;
mod vectors;

use bvh::BVHNode;
use indicatif::MultiProgress;
use std::sync::Arc;
use std::{f32::INFINITY, sync::mpsc, thread};

use camera::Camera;
use hit::{HitList, Hittable};
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
use objects::{moving_sphere::MovingSphere, sphere::Sphere};
use rand::{thread_rng, Rng};
use rays::{Color, Ray};
use vectors::{Point3, Vec3};

const FILENAME: &str = "output/test.png";
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const SAMPLE_SIZE: u32 = 100;
const MAX_DEPTH: u32 = 50;
const BIAS: f32 = 0.001;

const THREADS: usize = 12;

fn main() {
    // Start timer
    let now = std::time::Instant::now();

    // Image
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    // Scene
    let mut list = generate_random_scene();
    let scene = BVHNode::new(&mut list, 0.0, 1.0);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        focus_distance,
    );

    // Chunks
    println!("Preparing chunks...");
    let chunks = (0..HEIGHT)
        .collect::<Vec<u32>>()
        .iter()
        .map(|h| (0..WIDTH).map(|w| (*h, w)).collect::<Vec<(u32, u32)>>())
        .enumerate()
        .fold(vec![Vec::new(); THREADS], |mut acc, (i, band)| {
            acc.get_mut(i % THREADS).unwrap().extend(band);
            acc
        });
    println!("Chunks prepared!");

    // Progress bar init
    let multibar = MultiProgress::new();
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");

    println!("\nStarting workers...\n");

    let (tx, rx) = mpsc::channel::<Vec<(u32, u32, image::Rgb<u8>)>>();

    let scene_arc = Arc::new(scene);
    let tx_arc = Arc::new(tx);
    for (_, chunk) in chunks.into_iter().enumerate() {
        let scene_arc = scene_arc.clone();
        let sender = mpsc::Sender::clone(&tx_arc);

        let progress = multibar.add(ProgressBar::new(chunk.len() as u64));
        progress.set_style(style.clone());

        thread::spawn(move || {
            let mut rng = thread_rng();
            let pixels = chunk
                .into_iter()
                .map(|(y, x)| {
                    progress.inc(1);

                    let mut color = Color::default();
                    for _ in 0..SAMPLE_SIZE {
                        let u = (x as f32 + rng.gen::<f32>()) / (WIDTH - 1) as f32;
                        let v = (y as f32 + rng.gen::<f32>()) / (HEIGHT - 1) as f32;

                        let ray = camera.get_ray(u, v);
                        color += raytrace(ray, &scene_arc, MAX_DEPTH);
                    }

                    let color = calculate_pixel_color(color, SAMPLE_SIZE);

                    (x, HEIGHT - 1 - y, color)
                })
                .collect();

            sender.send(pixels).unwrap();
            progress.finish_with_message("Done!");
            drop(sender);
        });
    }
    drop(tx_arc);

    multibar.join().unwrap();

    for pixels in rx {
        for pixel in pixels {
            img.put_pixel(pixel.0, pixel.1, pixel.2);
        }
    }

    // let mut rng = thread_rng();
    // for y in (0..HEIGHT).rev() {
    //     for x in 0..WIDTH {
    //         let mut color = Color::default();
    //         for _ in 0..SAMPLE_SIZE {
    //             let u = (x as f32 + rng.gen::<f32>()) / (WIDTH - 1) as f32;
    //             let v = (y as f32 + rng.gen::<f32>()) / (HEIGHT - 1) as f32;

    //             let ray = camera.get_ray(u, v);
    //             color += raytrace(ray, &scene, MAX_DEPTH);
    //         }

    //         let color = calculate_pixel_color(color, SAMPLE_SIZE);

    //         img.put_pixel(x, HEIGHT - 1 - y, color);
    //     }
    // }

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

fn raytrace(ray: Ray, scene: &BVHNode, depth: u32) -> Color {
    if depth == 0 {
        return Color::default();
    }

    if let Some(hit) = scene.hit(&ray, 0.0 + BIAS, INFINITY) {
        return match hit.material.scatter(&ray, &hit) {
            Some((scattered_ray, attenuatuion)) => {
                attenuatuion * raytrace(scattered_ray, scene, depth - 1)
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

fn generate_random_scene() -> HitList {
    let mut rng = thread_rng();
    let mut scene = HitList::new();

    let ground_mat = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground = Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(ground_mat),
    };
    scene.add(Arc::new(ground));

    for i in -11..11 {
        for j in -11..11 {
            let random: f32 = rng.gen();

            let center = Point3::new(
                i as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                j as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point3::new(4.0, 2.0, 0.0)).norm() > 0.9 {
                if random < 0.8 {
                    let albedo = Color::new_random(0.0, 1.0) * Color::new_random(0.0, 1.0);
                    let material = Arc::new(Lambertian::new(albedo));
                    let center_end: Point3<f32> = center + Vec3::new(0.0, rng.gen(), 0.0);
                    let sphere = MovingSphere {
                        center_start: center,
                        center_end,
                        time_start: 0.0,
                        time_end: 1.0,
                        radius: 0.2,
                        material,
                    };
                    scene.add(Arc::new(sphere));
                } else if random < 0.95 {
                    let albedo = Color::new_random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    let sphere = Sphere {
                        center,
                        radius: 0.2,
                        material,
                    };
                    scene.add(Arc::new(sphere));
                } else {
                    let material = Arc::new(Dielectric::new(1.5));
                    let sphere = Sphere {
                        center,
                        radius: 0.2,
                        material,
                    };
                    scene.add(Arc::new(sphere));
                }
            }
        }
    }

    let big1 = Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric::new(1.5)),
    };
    let big2 = Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    };
    let big3 = Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    };

    scene.add(Arc::new(big1));
    scene.add(Arc::new(big2));
    scene.add(Arc::new(big3));

    scene
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
