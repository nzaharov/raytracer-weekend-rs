#![warn(clippy::all)]

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use crate::hit::HittableImpl;
use camera::Camera;
use hit::Hittable;
use image::RgbImage;
use materials::MaterialImpl;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rays::{Color, Ray};
use std::{f32::INFINITY, path::Path};

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

    pub fn render(self, scene: Hittable, output: &'a dyn AsRef<Path>) {
        println!("\nStarting workers...\n");

        let subpixels = (0..self.height)
            .rev()
            .collect::<Vec<u32>>()
            .into_par_iter()
            .map_init(thread_rng, |rng, line| {
                (0..self.width)
                    .map(|w| (line, w))
                    .flat_map(|(y, x)| {
                        let mut color = Color::default();
                        for _ in 0..self.sample_size {
                            let u = (x as f32 + rng.gen::<f32>()) / (self.width - 1) as f32;
                            let v = (y as f32 + rng.gen::<f32>()) / (self.height - 1) as f32;

                            let ray = self.camera.get_ray(u, v);
                            color += Self::raytrace(ray, &scene, MAX_DEPTH);
                        }

                        Self::calculate_pixel_color(color, self.sample_size)
                    })
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .collect::<Vec<u8>>();

        print!("\nSaving image... ");

        RgbImage::from_vec(self.width, self.height, subpixels)
            .unwrap()
            .save(output)
            .expect("Could not save image");

        println!("Done!");
    }

    fn calculate_pixel_color(color: Color, sample_size: u32) -> [u8; 3] {
        let scale = 1.0 / sample_size as f32;
        let (r, g, b) = (color.x(), color.y(), color.z());

        // Color correction (gamma=2.2)
        let r = (scale * r).powf(1.0 / 2.2);
        let g = (scale * g).powf(1.0 / 2.2);
        let b = (scale * b).powf(1.0 / 2.2);

        [
            (256.0 * r.clamp(0.0, 0.999)) as u8,
            (256.0 * g.clamp(0.0, 0.999)) as u8,
            (256.0 * b.clamp(0.0, 0.999)) as u8,
        ]
    }

    fn raytrace(ray: Ray, scene: &Hittable, depth: u32) -> Color {
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
