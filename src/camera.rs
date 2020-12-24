use rand::{thread_rng, Rng};

use crate::rays::Ray;
use crate::vectors::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3<f32>,
    lower_left_corner: Point3<f32>,
    horizontal: Vec3<f32>,
    vertical: Vec3<f32>,
    _w: Vec3<f32>, // currently unused
    u: Vec3<f32>,
    v: Vec3<f32>,
    lens_radius: f32,
    shutter_open_time: f32,
    shutter_close_time: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3<f32>,
        lookat: Point3<f32>,
        vup: Vec3<f32>,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperature: f32,
        focus_distance: f32,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            _w: w,
            u,
            v,
            lens_radius: aperature / 2.0,
            shutter_open_time: 0.0,
            shutter_close_time: 1.0,
        }
    }

    pub fn set_shutter_open_close_time(&mut self, shutter_open_time: f32, shutter_close_time: f32) {
        self.shutter_open_time = shutter_open_time;
        self.shutter_close_time = shutter_close_time;
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        let origin = self.origin + offset;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin;

        let time = thread_rng().gen_range(self.shutter_open_time..self.shutter_close_time);

        Ray::new(origin, direction, time)
    }
}
