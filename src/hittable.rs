use crate::ray::Ray;
use crate::vec3::Vec3;

struct HitRecord
{
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

impl HitRecord {
    pub fn new(t: f32, point: Vec3, normal: Vec3) -> Self
    {
        HitRecord {point, normal, t}
    }
}

trait Hittable
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

struct Sphere
{
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self
    {
        Sphere {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = self.center - ray.origin();

        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0
        {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let mut root = h - sqrt / a;

        if root <= t_min || t_max <= root
        {
            root = (h + sqrt) / a;
            if root <= t_min || t_max <= root
            {
                return None;
            }
        }

        let p = ray.at(root);
        Some(HitRecord::new(root, p, (p - self.center) / self.radius))
    }
}