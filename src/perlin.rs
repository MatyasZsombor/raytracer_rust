use std::ops::MulAssign;
use rand::{Rng};
use crate::vec3::Vec3;
fn generate_random_vecs(n: usize) -> Vec<Vec3>
{
    let mut rng = rand::thread_rng();
    let mut rand_vecs = Vec::with_capacity(n);
    for _ in 0..n {
        rand_vecs.push(Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize());
    }
    rand_vecs
}

fn trilinear_interpolation(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut acc = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                acc += (i as f32 * uu + (1.0 - i as f32)*(1.0 - uu))
                    *  (j as f32 * vv + (1.0 - j as f32)*(1.0 - vv))
                    *  (k as f32 * ww + (1.0 - k as f32)*(1.0 - ww))
                    *  c[i as usize][j as usize][k as usize].dot(weight);
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
    random_vectors: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs
    }
}

impl Perlin
{
    pub fn new() -> Self
    {
        Perlin
        {
            random_vectors: generate_random_vecs(256),
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

        let mut c = [[[Vec3::new_zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vectors[
                        self.perm_x[((i + di as i32) & 255) as usize] ^
                        self.perm_y[((j + dj as i32) & 255) as usize] ^
                        self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        trilinear_interpolation(&c, u, v, w)
    }

    pub fn turbulence(&self, point: &Vec3, depth: i32) -> f32
    {
        let mut accum = 0.0;
        let mut weight = 1.0;
        let mut tmp_p = *point;

        for _ in 0..depth {
            accum += weight * self.noise(&tmp_p);
            weight *= 0.5;
            tmp_p *= 2.0;
        }

        f32::abs(accum)
    }
}