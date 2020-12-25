#![warn(clippy::all)]
use rand::{thread_rng, Rng};
use raytracer::bvh::BVHNode;
use raytracer::hit::HitList;
use raytracer::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use raytracer::objects::{moving_sphere::MovingSphere, sphere::Sphere};
use raytracer::rays::Color;
use raytracer::vectors::{Point3, Vec3};
use raytracer::{camera::Camera, Raytracer};
use std::sync::Arc;

const FILENAME: &str = "output/test.png";
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const SAMPLE_SIZE: u32 = 100;

fn main() {
    // Start timer
    let now = std::time::Instant::now();

    // Dimensions
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

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

    let raytracer = Raytracer::new(WIDTH, HEIGHT, camera, SAMPLE_SIZE);

    // Scene
    let mut list = generate_random_scene();
    let scene = BVHNode::new(&mut list, 0.0, 1.0);

    raytracer.render(scene, &FILENAME);

    println!("Finished in {} ms", now.elapsed().as_millis());
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
