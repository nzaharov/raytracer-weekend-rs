use crate::rays::Ray;
use crate::vectors::{Point3, Vec3};

pub struct Camera {
    origin: Point3<f32>,
    lower_left_corner: Point3<f32>,
    horizontal: Vec3<f32>,
    vertical: Vec3<f32>,
}

impl Camera {
    pub fn new() -> Self {
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f32 = 2.0;
        const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.0;

        let origin = Point3::<f32>::default();
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, direction)
    }
}
