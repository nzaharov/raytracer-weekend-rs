#![warn(clippy::all)]

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use crate::hit::HittableImpl;
use camera::Camera;
use hit::Hittable;
use image::RgbImage;
use indicatif::ParallelProgressIterator;
use materials::MaterialImpl;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rays::{Color, Ray};
use std::path::Path;

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
    background: Color,
    sample_size: u32,
}

impl<'a> Raytracer {
    pub fn new(
        width: u32,
        height: u32,
        camera: &Camera,
        background: &Color,
        sample_size: u32,
    ) -> Self {
        Self {
            height,
            width,
            camera: *camera,
            background: *background,
            sample_size,
        }
    }

    pub fn render(self, scene: Hittable, output: &'a dyn AsRef<Path>) {
        println!("\nStarting workers...\n");

        let subpixels = (0..self.height)
            .rev()
            .collect::<Vec<u32>>()
            .into_par_iter()
            .progress_count(self.height as u64)
            .map_init(thread_rng, |rng, line| {
                (0..self.width)
                    .map(|w| (line, w))
                    .flat_map(|(y, x)| {
                        let mut color = Color::default();
                        for _ in 0..self.sample_size {
                            let u = (x as f32 + rng.gen::<f32>()) / (self.width - 1) as f32;
                            let v = (y as f32 + rng.gen::<f32>()) / (self.height - 1) as f32;

                            let ray = self.camera.get_ray(u, v);
                            color += Self::raytrace(ray, &self.background, &scene, MAX_DEPTH);
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

    fn raytrace(ray: Ray, background: &Color, scene: &Hittable, depth: u32) -> Color {
        if depth == 0 {
            return Color::default();
        }

        let Some(hit) = scene.hit(&ray, BIAS, f32::INFINITY) else {
            return *background;
        };

        let emission = hit.material.emit(hit.u, hit.v, &hit.point);

        let Some((scattered_ray, attenuation)) = hit.material.scatter(&ray, &hit) else {
            return emission;
        };

        emission + attenuation * Self::raytrace(scattered_ray, background, scene, depth - 1)
    }
}
