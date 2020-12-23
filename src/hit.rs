use std::marker::PhantomData;

use crate::vectors::{Point3, Vec3};
use crate::{materials::Material, rays::Ray};

pub struct Hit<'a, T>
where
    T: Material,
{
    pub point: Point3<f32>,
    pub normal: Vec3<f32>,
    pub t: f32,
    pub is_front_facing: bool,
    pub material: &'a T,
}

impl<'a, T> Hit<'a, T>
where
    T: Material,
{
    pub fn new(
        point: Point3<f32>,
        t: f32,
        material: &'a T,
        ray: &Ray,
        outward_normal: &Vec3<f32>,
    ) -> Self {
        let is_front_facing = ray.direction().dot(outward_normal) < 0.0;
        let normal = match is_front_facing {
            true => outward_normal.to_owned(),
            false => -outward_normal.to_owned(),
        };

        Self {
            point,
            t,
            normal,
            is_front_facing,
            material,
        }
    }
}

pub trait Hittable<F>
where
    F: Material,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit<F>>;
}

pub struct HitList<'a, T, F>(Vec<&'a T>, PhantomData<F>)
where
    T: Hittable<F>,
    F: Material;

impl<'a, T, F> HitList<'a, T, F>
where
    T: Hittable<F>,
    F: Material,
{
    pub fn new() -> Self {
        Self(Vec::new(), PhantomData)
    }

    pub fn add(&mut self, obj: &'a T) {
        self.0.push(obj);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<'a, T, F> Hittable<F> for HitList<'a, T, F>
where
    T: Hittable<F>,
    F: Material,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit<F>> {
        let mut closest_hit_t = t_max;
        let mut current_hit: Option<Hit<F>> = None;

        for &obj in self.0.iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest_hit_t) {
                closest_hit_t = hit.t;
                current_hit = Some(hit);
            }
        }

        current_hit
    }
}
