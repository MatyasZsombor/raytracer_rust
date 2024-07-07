use std::cmp::min;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub enum Material
{
    Lambertian {attenuation: Vec3 },
    Metal {attenuation: Vec3, fuzz: f32 },
    Dielectric {refraction_index: f32}
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

impl Material {
    pub fn scatter(&self, ray: &Ray, record: &HitRecord) -> (Ray, Color, bool)
    {
        match self
        {
            Material::Lambertian { attenuation} => {
                let mut scatter_direction = record.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero()
                {
                    scatter_direction = record.normal;
                }

                (Ray::new(record.point, scatter_direction), *attenuation, true)
            }
            Material::Metal {attenuation, fuzz} => {
                let mut reflected = reflect(&ray.direction(), &record.normal);
                reflected = reflected.normalize() + (*fuzz * Vec3::random_unit_vector());
                let scattered = Ray::new(record.point, reflected);

                (scattered, *attenuation, scattered.direction().dot(record.normal) > 0.0)
            }
            Material::Dielectric {refraction_index} => {
                let refraction = if record.front_face {1.0 / *refraction_index}
                else { *refraction_index };

                let refracted = refract(&ray.direction().normalize(), &record.normal, refraction);
                (Ray::new(record.point, refracted), Color::new(1.0,1.0,1.0), true)
            }
        }
    }
}