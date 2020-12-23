use rand::prelude::{Rng, ThreadRng};

use super::Vec3;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

impl Add for Vec3<f32> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<f32> for Vec3<f32> {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self(self.x() + rhs, self.y() + rhs, self.z() + rhs)
    }
}

impl Sub for Vec3<f32> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul for Vec3<f32> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f32> for Vec3<f32> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3<f32>> for f32 {
    type Output = Vec3<f32>;

    fn mul(self, rhs: Vec3<f32>) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Vec3<f32> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Vec3<f32> {
    pub fn unit_vector(self) -> Self {
        self / self.norm()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn new_random(rng: &mut ThreadRng, min: f32, max: f32) -> Self {
        Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Vec3::new_random(rng, -1.0, 1.0);
            if p.norm_sqr() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self::random_in_unit_sphere(rng).unit_vector()
    }

    pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: &Vec3<f32>) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(rng);
        if normal.dot(&in_unit_sphere) > 0.0 {
            return in_unit_sphere;
        }
        -in_unit_sphere
    }

    pub fn is_near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x().abs() < eps && self.y().abs() < eps && self.z().abs() < eps
    }

    pub fn reflect(&self, normal: &Vec3<f32>) -> Vec3<f32> {
        *self - 2.0 * self.dot(normal) * *normal
    }

    // coef = eta / eta'
    pub fn refract(&self, normal: &Vec3<f32>, coef: f32) -> Vec3<f32> {
        let normal = *normal;
        let cos_theta = (-*self).dot(&normal).min(1.0);
        let r_perpendicular: Vec3<f32> = coef * (*self + cos_theta * normal);
        let r_parallel: Vec3<f32> = -(1.0 - r_perpendicular.norm_sqr()).abs().sqrt() * normal;

        r_perpendicular + r_parallel
    }
}

impl<T> Display for Vec3<T>
where
    T: Display + Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.x(), self.y(), self.z())
    }
}
