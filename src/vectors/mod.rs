pub mod utils;

use std::ops::{AddAssign, DivAssign, MulAssign, Neg};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vec3<T>(pub T, pub T, pub T);

pub type Point3<T> = Vec3<T>;

impl<T> Vec3<T>
where
    T: Copy + Clone,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> T {
        self.1
    }

    pub fn z(&self) -> T {
        self.2
    }

    pub fn get(&self, dim: u32) -> Option<T> {
        match dim {
            0 => self.0.into(),
            1 => self.1.into(),
            2 => self.2.into(),
            _ => None,
        }
    }
}

impl Default for Vec3<f32> {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl Vec3<f32> {
    pub fn norm_sqr(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn norm(&self) -> f32 {
        self.norm_sqr().sqrt()
    }
}

impl AddAssign for Vec3<f32> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z());
    }
}

impl Neg for Vec3<f32> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y(), -self.z())
    }
}

impl MulAssign<f32> for Vec3<f32> {
    fn mul_assign(&mut self, c: f32) {
        *self = Self(self.x() * c, self.y() * c, self.z() * c)
    }
}

impl DivAssign<f32> for Vec3<f32> {
    fn div_assign(&mut self, c: f32) {
        *self *= 1.0 / c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_assign() {
        let mut vec3 = Vec3(4.0, 5.0, 6.0);
        vec3 *= 5.0;

        assert_eq!(vec3, Vec3(20.0, 25.0, 30.0));
    }

    #[test]
    fn test_div_assign() {
        let mut vec3 = Vec3(5.0, 10.0, 15.0);
        vec3 /= 5.0;

        assert_eq!(vec3, Vec3(1.0, 2.0, 3.0));
    }
}
