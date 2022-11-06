use crate::vectors::{Point3, Vec3};
use rand::seq::SliceRandom;

use rand::thread_rng;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    rand_vecs: Vec<Vec3<f32>>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub const DEFAULT_DEPTH: usize = 7;

    pub fn noise(&self, p: &Point3<f32>) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let c = (0..2)
            .map(|dim_i| {
                (0..2)
                    .map(|dim_j| {
                        (0..2)
                            .map(|dim_k| {
                                self.rand_vecs[(self.perm_x[((i + dim_i) & 255) as usize]
                                    ^ self.perm_y[((j + dim_j) & 255) as usize]
                                    ^ self.perm_z[((k + dim_k) & 255) as usize])
                                    as usize]
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Vec<Vec3<f32>>>>>();

        perlin_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, p: &Point3<f32>, depth: usize) -> f32 {
        let mut acc = 0.0;
        let mut p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }

        acc.abs()
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            rand_vecs: (0..POINT_COUNT)
                .map(|_| Vec3::<f32>::new_random(-1.0, 1.0).unit_vector())
                .collect(),
            perm_x: generate_perlin(),
            perm_y: generate_perlin(),
            perm_z: generate_perlin(),
        }
    }
}

fn generate_perlin() -> Vec<i32> {
    let mut permutation = (0..POINT_COUNT as i32).collect::<Vec<i32>>();

    permutation.shuffle(&mut thread_rng());

    permutation
}

fn perlin_interpolation(c: Vec<Vec<Vec<Vec3<f32>>>>, u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    c.iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter().enumerate().flat_map(move |(j, a)| {
                a.iter().enumerate().map(move |(k, x)| {
                    let i = i as f32;
                    let j = j as f32;
                    let k = k as f32;

                    let weights = Vec3::new(u - i, v - j, k - w);

                    (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * x.dot(&weights)
                })
            })
        })
        .sum()
}
