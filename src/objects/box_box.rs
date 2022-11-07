use std::sync::Arc;

use crate::{
    aabb::AABB,
    hit::{Hit, HitList, HittableImpl},
    materials::Material,
    rays::Ray,
    vectors::Point3,
};

use super::{xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect};

pub struct BoxBox {
    box_min: Point3<f32>,
    box_max: Point3<f32>,
    sides: HitList,
}

impl BoxBox {
    pub fn from_points(p0: Point3<f32>, p1: Point3<f32>, material: Arc<Material>) -> Self {
        let mut sides = HitList::new();

        sides.add(Arc::new(
            XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material.clone()).into(),
        ));
        sides.add(Arc::new(
            XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material.clone()).into(),
        ));

        sides.add(Arc::new(
            XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material.clone()).into(),
        ));
        sides.add(Arc::new(
            XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material.clone()).into(),
        ));

        sides.add(Arc::new(
            YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material.clone()).into(),
        ));
        sides.add(Arc::new(
            YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material.clone()).into(),
        ));

        Self {
            box_min: p0,
            box_max: p1,
            sides: sides,
        }
    }
}

impl HittableImpl for BoxBox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn get_b_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}
