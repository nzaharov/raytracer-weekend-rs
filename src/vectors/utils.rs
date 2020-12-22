use image::Rgb;

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
}

impl Into<Rgb<u8>> for Vec3<f32> {
    fn into(self) -> Rgb<u8> {
        Rgb([
            (256.0 * self.x()) as u8,
            (256.0 * self.y()) as u8,
            (256.0 * self.z()) as u8,
        ])
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
