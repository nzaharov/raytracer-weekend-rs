use raytracer::{
    camera::Camera,
    hit::HitList,
    materials::lambertian::Lambertian,
    objects::sphere::Sphere,
    textures::noise::Noise,
    vectors::{Point3, Vec3},
    Raytracer,
};
use std::{
    sync::Arc,
    time::{Instant, SystemTime},
};

const FILENAME: &str = "perlin";
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const SAMPLE_SIZE: u32 = 100;

fn main() {
    let now = Instant::now();

    let filename = format!(
        "output/{}_{}.png",
        FILENAME,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // Dimensions
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        ASPECT_RATIO,
        aperture,
        focus_distance,
    );

    // Scene
    let mut scene = HitList::new();

    let perlin = Noise::new(4.0);
    let mat = Lambertian::with_texture(&perlin.into());
    let sphere1 = Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: mat.clone().into(),
    };
    let sphere2 = Sphere {
        center: Point3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: mat.clone().into(),
    };

    scene.add(Arc::new(sphere1.into()));
    scene.add(Arc::new(sphere2.into()));

    let raytracer = Raytracer::new(WIDTH, HEIGHT, camera, SAMPLE_SIZE);

    raytracer.render(scene.into(), &filename);

    println!("Finished in {} ms", now.elapsed().as_millis());
}
