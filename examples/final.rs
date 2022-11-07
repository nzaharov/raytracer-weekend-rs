use rand::{thread_rng, Rng};
use raytracer::{
    bvh::BVHNode,
    camera::Camera,
    hit::{HitList, Hittable, RotateY, Translate},
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
        Material,
    },
    objects::{box_box::BoxBox, moving_sphere::MovingSphere, sphere::Sphere, xz_rect::XZRect},
    rays::Color,
    textures::{image_texture::ImageTexture, noise::Noise},
    vectors::{Point3, Vec3},
    volumes::ConstantMedium,
    Raytracer,
};
use std::{
    sync::Arc,
    time::{Instant, SystemTime},
};

const FILENAME: &str = "final";
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
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

    // Camera
    let lookfrom = Point3::new(478.0, 278.0, -600.0);
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

    let mut scene = HitList::new();

    // Ground
    let mut ground = HitList::new();
    let ground_mat: Material = Lambertian::new(Color::new(0.48, 0.83, 0.53)).into();

    const BOX_PER_SIDE: u32 = 20;
    for i in 0..BOX_PER_SIDE {
        for j in 0..BOX_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = thread_rng().gen_range(1.0..101.0);
            let z1 = z0 + w;

            let boxbox: Hittable = BoxBox::from_points(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground_mat.clone().into(),
            )
            .into();

            ground.add(boxbox.into());
        }
    }
    let ground: Hittable = BVHNode::new(&mut ground, 0.0, 1.0).into();
    scene.add(ground.into());

    let light: Material = DiffuseLight::with_color(Color::new(7.0, 7.0, 7.0)).into();
    let light: Hittable = XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        material: light.into(),
    }
    .into();
    scene.add(light.into());

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_mat: Material = Lambertian::new(Color::new(0.7, 0.3, 0.1)).into();
    let moving_sphere: Hittable = MovingSphere {
        center_start: center1,
        center_end: center2,
        radius: 50.0,
        time_start: 0.0,
        time_end: 1.0,
        material: moving_sphere_mat.into(),
    }
    .into();
    scene.add(moving_sphere.into());

    let dielectric1: Hittable = Sphere {
        material: Arc::new(Dielectric::new(1.5).into()),
        center: Point3::new(260.0, 150.0, 45.0),
        radius: 50.0,
    }
    .into();
    scene.add(dielectric1.into());

    let metal: Hittable = Sphere {
        material: Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 1.0).into()),
        center: Point3::new(0.0, 150.0, 145.0),
        radius: 50.0,
    }
    .into();
    scene.add(metal.into());

    let subsurface: Hittable = Sphere {
        material: Arc::new(Dielectric::new(1.5).into()),
        center: Point3::new(260.0, 150.0, 45.0),
        radius: 50.0,
    }
    .into();
    let subsurface: Hittable =
        ConstantMedium::new(subsurface, 0.2, Color::new(0.2, 0.4, 0.9)).into();
    scene.add(subsurface.into());

    let mist: Hittable = Sphere {
        material: Arc::new(Dielectric::new(1.5).into()),
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 5000.0,
    }
    .into();
    let mist: Hittable = ConstantMedium::new(mist, 0.0001, Color::new(0.2, 0.4, 0.9)).into();
    scene.add(mist.into());

    let texture = ImageTexture::load_from_file("examples/assets/earthmap.jpg").unwrap();
    let mat: Material = Lambertian::with_texture(&texture.into()).into();
    let earth: Hittable = Sphere {
        center: Point3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        material: mat.into(),
    }
    .into();
    scene.add(earth.into());

    let mat: Material = Lambertian::with_texture(&Noise::new(0.1).into()).into();
    let noiseball: Hittable = Sphere {
        center: Point3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        material: mat.into(),
    }
    .into();
    scene.add(noiseball.into());

    let mut bubbles = HitList::new();
    let white: Arc<Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)).into());
    for _ in 0..1000 {
        let bubble: Hittable = Sphere {
            center: Point3::new_random(0.0, 165.0),
            radius: 10.0,
            material: white.clone(),
        }
        .into();
        bubbles.add(bubble.into());
    }

    let bubbles: Hittable = BVHNode::new(&mut bubbles, 0.0, 1.0).into();
    let bubbles: Hittable = Translate::new(
        RotateY::new(bubbles.into(), 15.0).into(),
        &Vec3::new(-100.0, 270.0, 395.0),
    )
    .into();
    scene.add(bubbles.into());

    let raytracer = Raytracer::new(WIDTH, HEIGHT, &camera, &background, SAMPLE_SIZE);

    raytracer.render(scene.into(), &filename);

    println!("Finished in {} ms", now.elapsed().as_millis());
}
