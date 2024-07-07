use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub enum Material
{
    Lambertian {attenuation: Vec3 },
    Metal {attenuation: Vec3, fuzz: f32 }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3
{
    *v - 2.0 * v.dot(*n) * *n
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
        }
    }
}