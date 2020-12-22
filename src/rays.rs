use crate::vectors::{Point3, Vec3};

pub type Color = Vec3<f32>;

pub struct Ray(Vec3<f32>, Point3<f32>);

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vec3<f32>) -> Self {
        Self(origin, direction)
    }

    pub fn direction(&self) -> Vec3<f32> {
        self.1
    }

    pub fn origin(&self) -> Point3<f32> {
        self.0
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        self.origin() + t * self.direction()
    }
}
