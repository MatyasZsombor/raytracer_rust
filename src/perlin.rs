use rand::{Rng};
use crate::vec3::Vec3;
fn generate_random_floats(n: usize) -> Vec<f32>
{
    let mut rng = rand::thread_rng();
    let mut rand_floats = Vec::with_capacity(n);
    for _ in 0..n {
        rand_floats.push(rng.gen::<f32>());
    }
    rand_floats
}

fn trilinear_interpolation(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut acc = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                acc += (i as f32 * u + (1.0 - i as f32)*(1.0 - u))
                    *  (j as f32 * v + (1.0 - j as f32)*(1.0 - v))
                    *  (k as f32 * w + (1.0 - k as f32)*(1.0 - w))
                    *  c[i as usize][j as usize][k as usize];
            }
        }
    }

    acc
}

fn perlin_generate_perm(n: usize) -> Vec<usize>
{
    let mut p = Vec::with_capacity(n);

    for i in 0..n {
        p.push(i);
    }
    
    permute(&mut p, n);
    
    p 
}

fn permute(p: &mut [usize], n: usize)
{
    let mut rng = rand::thread_rng();
    for i in (0..n).rev() {
        let target = rng.gen_range(0..=i);
        p.swap(i, target);
    }
}

#[derive(Clone)]
pub struct Perlin
{
    random_floats: Vec<f32>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>
}

impl Perlin
{
    pub fn new() -> Self
    {
        Perlin
        {
            random_floats: generate_random_floats(256),
            perm_x: perlin_generate_perm(256),
            perm_y: perlin_generate_perm(256),
            perm_z: perlin_generate_perm(256),
        }
    }

    pub fn noise(&self, point: &Vec3) -> f32
    {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as i32;
        let j = point.y().floor() as i32;
        let k = point.z().floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_floats[
                        self.perm_x[((i + di as i32) & 255) as usize] ^
                        self.perm_y[((j + dj as i32) & 255) as usize] ^
                        self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        trilinear_interpolation(c, u, v, w)
    }
}