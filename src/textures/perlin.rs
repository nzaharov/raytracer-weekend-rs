use crate::vectors::Point3;
use rand::distributions::Standard;
use rand::seq::SliceRandom;

use rand::{thread_rng, Rng};

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    random_values: Vec<f32>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn noise(&self, p: &Point3<f32>) -> f32 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let c = (0..2)
            .map(|dim_i| {
                (0..2)
                    .map(|dim_j| {
                        (0..2)
                            .map(|dim_k| {
                                self.random_values[(self.perm_x[((i + dim_i) & 255) as usize]
                                    ^ self.perm_y[((j + dim_j) & 255) as usize]
                                    ^ self.perm_z[((k + dim_k) & 255) as usize])
                                    as usize]
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Vec<f32>>>>();

        trilinear_interpolate(c, u, v, w)
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            random_values: thread_rng()
                .sample_iter(Standard)
                .take(POINT_COUNT)
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

fn trilinear_interpolate(c: Vec<Vec<Vec<f32>>>, u: f32, v: f32, w: f32) -> f32 {
    c.iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter().enumerate().flat_map(move |(j, a)| {
                a.iter().enumerate().map(move |(k, x)| {
                    let i = i as f32;
                    let j = j as f32;
                    let k = k as f32;

                    (i * u + (1.0 - i) * (1.0 - u))
                        * (j * v + (1.0 - j) * (1.0 - v))
                        * (k * w + (1.0 - k) * (1.0 - w))
                        * x
                })
            })
        })
        .sum()
}
