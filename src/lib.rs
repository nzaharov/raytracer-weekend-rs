#![warn(clippy::all)]
use crate::hit::Hittable;
use camera::Camera;
use image::{Rgb, RgbImage};
use indicatif::ProgressStyle;
use indicatif::{MultiProgress, ProgressBar};
use rand::{thread_rng, Rng};
use rays::{Color, Ray};
use std::{f32::INFINITY, path::Path, sync::Arc};
use std::{sync::mpsc, thread};

pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hit;
pub mod materials;
pub mod objects;
pub mod rays;
pub mod textures;
pub mod vectors;

const MAX_DEPTH: u32 = 50;
const BIAS: f32 = 0.001;

#[derive(Copy, Clone)]
pub struct Raytracer {
    height: u32,
    width: u32,
    camera: Camera,
    sample_size: u32,
}

impl<'a> Raytracer {
    pub fn new(width: u32, height: u32, camera: Camera, sample_size: u32) -> Self {
        Self {
            height,
            width,
            camera,
            sample_size,
        }
    }

    pub fn render<T>(self, scene: T, output: &'a dyn AsRef<Path>)
    where
        T: Hittable + Send + Sync + 'static,
    {
        // Init image
        let mut img = RgbImage::new(self.width, self.height);
        // Chunks
        println!("Preparing chunks...");

        let chunks = self.image_chunks();

        println!("Chunks prepared!");

        // Progress bar init
        let multibar = MultiProgress::new();
        let style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .progress_chars("=> ");

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
                        for _ in 0..self.sample_size {
                            let u = (x as f32 + rng.gen::<f32>()) / (self.width - 1) as f32;
                            let v = (y as f32 + rng.gen::<f32>()) / (self.height - 1) as f32;

                            let ray = self.camera.get_ray(u, v);
                            color += Self::raytrace(ray, &*scene_arc, MAX_DEPTH);
                        }

                        let color = Self::calculate_pixel_color(color, self.sample_size);

                        (x, self.height - 1 - y, color)
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

        print!("\nSaving image... ");

        img.save(output).expect("Could not save image");

        println!("Done!");
    }

    fn image_chunks(&self) -> Vec<Vec<(u32, u32)>> {
        let threads = num_cpus::get();

        (0..self.height)
            .collect::<Vec<u32>>()
            .iter()
            .map(|h| {
                (0..self.width)
                    .map(|w| (*h, w))
                    .collect::<Vec<(u32, u32)>>()
            })
            .enumerate()
            .fold(vec![Vec::new(); threads], |mut acc, (i, band)| {
                acc.get_mut(i % threads).unwrap().extend(band);
                acc
            })
    }

    fn calculate_pixel_color(color: Color, sample_size: u32) -> Rgb<u8> {
        let scale = 1.0 / sample_size as f32;
        let (r, g, b) = (color.x(), color.y(), color.z());

        // Color correction (gamma=2.2)
        let r = (scale * r).powf(1.0 / 2.2);
        let g = (scale * g).powf(1.0 / 2.2);
        let b = (scale * b).powf(1.0 / 2.2);

        Rgb([
            (256.0 * r.clamp(0.0, 0.999)) as u8,
            (256.0 * g.clamp(0.0, 0.999)) as u8,
            (256.0 * b.clamp(0.0, 0.999)) as u8,
        ])
    }

    fn raytrace(ray: Ray, scene: &dyn Hittable, depth: u32) -> Color {
        if depth == 0 {
            return Color::default();
        }

        if let Some(hit) = scene.hit(&ray, 0.0 + BIAS, INFINITY) {
            return match hit.material.scatter(&ray, &hit) {
                Some((scattered_ray, attenuatuion)) => {
                    attenuatuion * Self::raytrace(scattered_ray, scene, depth - 1)
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
}
