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
        let i = ((4.0 * p.x()) as i32 & 255) as usize;
        let j = ((4.0 * p.y()) as i32 & 255) as usize;
        let k = ((4.0 * p.z()) as i32 & 255) as usize;

        self.random_values[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
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
