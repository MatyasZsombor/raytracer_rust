use rand::Rng;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;


pub trait Material
{
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
    fn emitted(&self, u: f32, v: f32, point: &Vec3) -> Vec3;
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3
{
    *v - 2.0 * v.dot(*n) * *n
}

fn refract(uv: &Vec3, n: &Vec3, eta_over_etap: f32) -> Vec3
{
    let cos_theta = (-*uv).dot(*n).min(1.0);
    let r_perpendicular = eta_over_etap * (*uv + cos_theta * *n);
    let r_parallel = -(1.0 - r_perpendicular.length_squared()).abs().sqrt() * *n;
    r_perpendicular + r_parallel
}
#[derive(Clone)]
pub struct Lambertian<T: Texture>
{
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Lambertian<T> { Lambertian { albedo } }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero()
        {
            scatter_direction = record.normal;
        }

        Some((Ray::new(record.point, scatter_direction, ray.time), self.albedo.value(record.u, record.v, &record.point)))
    }

    fn emitted(&self, _u: f32, _v: f32, _point: &Vec3) -> Vec3 {
        Vec3::new_zero()
    }
}

#[derive(Clone)]
pub struct Metal
{
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal { Metal { albedo, fuzz } }
}

impl Material for Metal
{
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = reflect(&ray.direction, &record.normal);
        reflected = reflected.normalize() + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(record.point, reflected, ray.time);

        if scattered.direction.dot(record.normal) > 0.0
        {
            Some((scattered, self.albedo))
        }
        else {
            None
        }
    }

    fn emitted(&self, _u: f32, _v: f32, _point: &Vec3) -> Vec3 {
        Vec3::new_zero()
    }
}

#[derive(Clone)]
pub struct Dielectric
{
    refraction_index: f32,
}

impl Dielectric
{
    pub fn new(refraction_index: f32) -> Dielectric
    {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric
{
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut rng = rand::thread_rng();

        let refraction = if record.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(record.normal).min(1.0);
        let sin_theta: f32 = (1.0 - cos_theta * cos_theta).sqrt();

        let direction =
            if refraction * sin_theta > 1.0 || schlick_approximation(cos_theta, refraction) > rng.gen::<f32>()
            {
                reflect(&unit_direction, &record.normal)
            } else {
                refract(&unit_direction, &record.normal, refraction)
            };

        Some((Ray::new(record.point, direction, ray.time), Color::new(1.0, 1.0, 1.0)))
    }

    fn emitted(&self, _u: f32, _v: f32, _point: &Vec3) -> Vec3 {
        Vec3::new_zero()
    }
}

#[derive(Clone)]
pub struct DiffuseMaterial<T: Texture>
{
    texture: T,
}

impl<T: Texture> DiffuseMaterial<T> {
    pub fn new(texture: T) -> DiffuseMaterial<T> {DiffuseMaterial{texture} }
}

impl<T: Texture> Material for DiffuseMaterial<T> {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, point: &Vec3) -> Vec3 {
        return self.texture.value(u, v, point);
    }
}


fn schlick_approximation(cosine: f32, refraction_index: f32) -> f32
{
    let mut r = (1.0 - refraction_index) / (1.0 + refraction_index);
    r = r * r;
    r + (1.0 - r) * (1.0 - cosine).powi(5)
}