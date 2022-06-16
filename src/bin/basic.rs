use raytracer::{
    camera::Camera,
    hit::HitList,
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    objects::{plane::Plane, sphere::Sphere},
    rays::Color,
    vectors::{Point3, Vec3},
    Raytracer,
};
use std::{
    sync::Arc,
    time::{Instant, SystemTime},
};

const FILENAME: &str = "basic";
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const SAMPLE_SIZE: u32 = 500;

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
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

    // Camera
    let lookfrom = Point3::new(0.0, 0.0, 0.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 0.5;
    let aperture = 0.01;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        75.0,
        ASPECT_RATIO,
        aperture,
        focus_distance,
    );

    // Scene
    let mut scene = HitList::new();

    let mat_1 = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let sphere1 = Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(mat_1.into()),
    };
    let mat_ground = Metal::new(Color::new(0.9, 0.1, 0.1), 0.5);
    let ground = Plane {
        p1: Point3::new(0.0, 0.0, 1.0),
        p2: Point3::new(0.0, -0.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Arc::new(mat_ground.into()),
    };
    let mat_2 = Lambertian::new(Color::default());
    let sphere2 = Sphere {
        center: Point3::new(1.0, 0.0, -2.0),
        radius: 0.5,
        material: Arc::new(mat_2.into()),
    };
    let mat_metal = Metal::new(Color::new(0.8, 0.8, 0.8), 0.0);
    let metal = Sphere {
        center: Point3::new(-2.0, 0.0, -1.5),
        radius: 0.5,
        material: Arc::new(mat_metal.into()),
    };
    let glass_mat = Dielectric::new(1.5);
    let crystal_ball = Sphere {
        center: Point3::new(0.27, 0.1, -0.5),
        radius: -0.05,
        material: Arc::new(glass_mat.into()),
    };
    scene.add(Arc::new(sphere1.into()));
    scene.add(Arc::new(ground.into()));
    scene.add(Arc::new(sphere2.into()));
    scene.add(Arc::new(metal.into()));
    scene.add(Arc::new(crystal_ball.into()));

    let raytracer = Raytracer::new(WIDTH, HEIGHT, camera, SAMPLE_SIZE);

    raytracer.render(scene.into(), &filename);

    println!("Finished in {} ms", now.elapsed().as_millis());
}
