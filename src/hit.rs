use enum_dispatch::enum_dispatch;

use crate::bvh::*;
use crate::materials::Material;
use crate::objects::{moving_sphere::*, plane::*, sphere::*, xy_rect::*, xz_rect::*, yz_rect::*};
use crate::rays::Ray;
use crate::{
    aabb::AABB,
    vectors::{Point3, Vec3},
};
use core::cmp::Ordering;
use std::sync::Arc;

pub struct Hit {
    pub point: Point3<f32>,
    pub normal: Vec3<f32>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub is_front_facing: bool,
    pub material: Arc<Material>,
}

impl Hit {
    pub fn new(
        point: Point3<f32>,
        t: f32,
        u: f32,
        v: f32,
        material: Arc<Material>,
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
            u,
            v,
            normal,
            is_front_facing,
            material,
        }
    }
}

#[enum_dispatch]
pub trait HittableImpl: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;

    fn get_b_box(&self, time0: f32, time1: f32) -> Option<AABB>;
}

pub struct HitList(Vec<Arc<Hittable>>);

impl HitList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, obj: Arc<Hittable>) {
        self.0.push(obj);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<Arc<Hittable>> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&Arc<Hittable>> {
        self.0.get(index)
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Arc<Hittable>, &Arc<Hittable>) -> Ordering,
    {
        self.0.sort_by(compare);
    }
}

impl Default for HitList {
    fn default() -> Self {
        HitList::new()
    }
}

impl HittableImpl for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_hit_t = t_max;
        let mut current_hit: Option<Hit> = None;

        for obj in self.0.iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest_hit_t) {
                closest_hit_t = hit.t;
                current_hit = Some(hit);
            }
        }

        current_hit
    }

    fn get_b_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let mut curr_box = None;

        for object in self.iter() {
            if let Some(b_box) = object.get_b_box(time0, time1) {
                curr_box = match curr_box {
                    Some(curr) => Some(AABB::new_surrounding_box(b_box, curr)),
                    None => Some(b_box),
                };
            } else {
                return None;
            }
        }

        curr_box
    }
}

#[enum_dispatch(HittableImpl)]
pub enum Hittable {
    HitList,
    BVHNode,
    MovingSphere,
    Plane,
    Sphere,
    XYRect,
    YZRect,
    XZRect,
}
