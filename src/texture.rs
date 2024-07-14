use crate::color::Color;
use crate::vec3::Vec3;

pub trait Texture: Sync
{
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

#[derive(Clone)]
pub struct SolidColor
{
    albedo: Vec3,
}

impl SolidColor
{
    pub fn new(r: f32, g: f32, b: f32) -> Self { SolidColor { albedo: Color::new(r, g, b) } }
}

impl Texture for SolidColor
{
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        return self.albedo;
    }
}

#[derive(Clone)]
pub struct CheckerTexture<T: Texture, U: Texture>
{
    inv_scale: f32,
    even: T,
    odd: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U>
{
    pub fn new(scale: f32, even: T, odd: U) -> CheckerTexture<T, U> { CheckerTexture { inv_scale: 1.0 / scale, even, odd } }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U>
{
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        if x + y + z % 2 == 0
        {
            self.even.value(u, v, p)
        }
        else {
            self.odd.value(u, v, p)
        }
    }
}