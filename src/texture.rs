use crate::color::Color;
use crate::perlin::Perlin;
use crate::vec3::Vec3;

pub trait Texture: Sync
{
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

#[derive(Copy, Clone)]
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
    fn value(&self, _: f32, _: f32, _: &Vec3) -> Vec3 {
        return self.albedo;
    }
}

#[derive(Clone, Copy)]
pub struct CheckerTexture<T: Texture, U: Texture>
{
    width: f32,
    height: f32,
    even: T,
    odd: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U>
{
    pub fn new(width: f32, height: f32, even: T, odd: U) -> CheckerTexture<T, U> { CheckerTexture {width, height, even, odd } }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U>
{
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let u2 = (u * self.width).floor() as i32;
        let v2 = (v * self.height).floor() as i32;

        if (u2 + v2) % 2 == 0
        {
            self.even.value(u, v, p)
        }
        else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture
{
    data: Vec<u8>,
    dimensions: (u32, u32)
}

impl ImageTexture {
    pub fn new(path: &str) -> ImageTexture
    {
        let image = image::open(path).expect("Image not found").to_rgb8();
        let (x, y) = image.dimensions();
        ImageTexture {data: image.into_raw(), dimensions: (x, y)}
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _: &Vec3) -> Vec3 {
        let u = if u < 0.0 { 0.0 } else if u > 1.0 {1.0} else { u };
        let v = 1.0 - if v < 0.0 { 0.0 } else if v > 1.0 {1.0} else { v };

        let i = (u * self.dimensions.0 as f32) as usize;
        let j = (v * self.dimensions.1 as f32) as usize;
        let idx = 3 * i + j * 3 * self.dimensions.0 as usize;
        return Color::new(self.data[idx] as f32 / 255.0, self.data[idx + 1] as f32 / 255.0, self.data[idx + 2] as f32 / 255.0,);
    }
}

#[derive(Clone)]
pub struct NoiseTexture
{
    perlin_noise: Perlin,
    scale: f32
}

impl NoiseTexture {
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture{perlin_noise: Perlin::new(), scale }
    }
}

impl Texture for NoiseTexture
{
    fn value(&self, _: f32, _: f32, p: &Vec3) -> Vec3 {
        Color::new(0.5,0.5,0.5) * (1.0 + (self.scale * p.z() + 10.0 * self.perlin_noise.turbulence(p, 7)).sin())
    }
}