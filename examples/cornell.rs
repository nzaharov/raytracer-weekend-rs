use raytracer::{
    camera::Camera,
    hit::{HitList, Hittable},
    materials::{diffuse_light::DiffuseLight, lambertian::Lambertian, Material},
    objects::{xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect},
    rays::Color,
    vectors::{Point3, Vec3},
    Raytracer,
};
use std::time::{Instant, SystemTime};

const FILENAME: &str = "cornell";
const ASPECT_RATIO: f32 = 1.0;
const SAMPLE_SIZE: u32 = 200;

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
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.0;
    let background = Color::new(0.0, 0.0, 0.0);

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

    let light: Material = DiffuseLight::with_color(Color::new(15.0, 15.0, 15.0)).into();
    let light: Hittable = XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
        material: light.into(),
    }
    .into();

    let red: Material = Lambertian::new(Color::new(0.65, 0.05, 0.05)).into();
    let right: Hittable = YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: red.into(),
    }
    .into();

    let green: Material = Lambertian::new(Color::new(0.12, 0.45, 0.15)).into();
    let left: Hittable = YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: green.into(),
    }
    .into();

    let white: Material = Lambertian::new(Color::new(0.73, 0.73, 0.73)).into();
    let top: Hittable = XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: white.clone().into(),
    }
    .into();
    let bot: Hittable = XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: white.clone().into(),
    }
    .into();
    let front: Hittable = XYRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        material: white.clone().into(),
    }
    .into();

    scene.add(light.into());
    scene.add(left.into());
    scene.add(right.into());
    scene.add(bot.into());
    scene.add(top.into());
    scene.add(front.into());

    let raytracer = Raytracer::new(WIDTH, HEIGHT, &camera, &background, SAMPLE_SIZE);

    raytracer.render(scene.into(), &filename);

    println!("Finished in {} ms", now.elapsed().as_millis());
}
