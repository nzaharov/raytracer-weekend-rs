use enum_dispatch::enum_dispatch;

use crate::bvh::*;
use crate::materials::Material;
use crate::objects::box_box::BoxBox;
use crate::objects::{moving_sphere::*, plane::*, sphere::*, xy_rect::*, xz_rect::*, yz_rect::*};
use crate::rays::Ray;
use crate::volumes::*;
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

pub struct Translate {
    obj: Box<Hittable>,
    displacement: Vec3<f32>,
}

impl Translate {
    pub fn new(obj: Hittable, displacement: &Vec3<f32>) -> Self {
        Self {
            obj: Box::new(obj),
            displacement: *displacement,
        }
    }
}

impl HittableImpl for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let displaced = Ray::new(
            ray.origin() - self.displacement,
            ray.direction(),
            ray.time(),
        );

        self.obj.hit(&displaced, t_min, t_max).map(|hit| {
            Hit::new(
                hit.point + self.displacement,
                hit.t,
                hit.u,
                hit.v,
                hit.material,
                &displaced,
                &hit.normal,
            )
        })
    }

    fn get_b_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.obj
            .get_b_box(time0, time1)
            .map(|bb| AABB::new(bb.min() + self.displacement, bb.max() + self.displacement))
    }
}

pub struct RotateY {
    obj: Box<Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(obj: Hittable, angle: f32) -> Self {
        let theta = angle.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let bbox = obj.get_b_box(0.0, 1.0);

        if let Some(bbox) = bbox {
            let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
            let mut max = Vec3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max().x() + (1.0 - i as f32) * bbox.min().x();
                        let y = j as f32 * bbox.max().y() + (1.0 - j as f32) * bbox.min().y();
                        let z = k as f32 * bbox.max().z() + (1.0 - k as f32) * bbox.min().z();

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        min.0 = min.0.min(newx);
                        min.1 = min.1.min(y);
                        min.2 = min.2.min(newz);

                        max.0 = max.0.max(newx);
                        max.1 = max.1.max(y);
                        max.2 = max.2.max(newz);
                    }
                }
            }
        }

        Self {
            obj: obj.into(),
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl HittableImpl for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let origin = Vec3::new(
            self.cos_theta * ray.origin().x() - self.sin_theta * ray.origin().z(),
            ray.origin().y(),
            self.sin_theta * ray.origin().x() + self.cos_theta * ray.origin().z(),
        );

        let direction = Vec3::new(
            self.cos_theta * ray.direction().x() - self.sin_theta * ray.direction().z(),
            ray.direction().y(),
            self.sin_theta * ray.direction().x() + self.cos_theta * ray.direction().z(),
        );

        let rotated_ray = Ray::new(origin, direction, ray.time());

        let hit = self.obj.hit(&rotated_ray, t_min, t_max)?;
        let hit_p = hit.point;

        let p = Point3::new(
            self.cos_theta * hit_p.x() + self.sin_theta * hit_p.z(),
            hit_p.y(),
            -self.sin_theta * hit_p.x() + self.cos_theta * hit_p.z(),
        );

        let normal = hit.normal;
        let normal = Vec3::new(
            self.cos_theta * normal.x() + self.sin_theta * normal.z(),
            normal.y(),
            -self.sin_theta * normal.x() + self.cos_theta * normal.z(),
        );

        Some(Hit::new(
            p,
            hit.t,
            hit.u,
            hit.v,
            hit.material,
            &rotated_ray,
            &normal,
        ))
    }

    fn get_b_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        self.bbox
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
    BoxBox,
    Translate,
    RotateY,
    ConstantMedium,
}
