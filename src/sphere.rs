use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn get_sphere_uv(point: &Vec3) -> (f32, f32)
{
    let theta = point.x().atan2(point.z());
    let radius = point.length();

    let phi = (point.y() / radius).acos();

    let raw_u = theta / std::f32::consts::TAU;
    let u = 1.0 - (raw_u + 0.5);
    let v = 1.0 - phi / std::f32::consts::PI;

    (u, v)
}

pub struct Sphere<T: Material>
{
    center_vec: Vec3,
    center1: Vec3,
    radius: f32,
    material: T,
    is_moving: bool,
}

impl<T: Material> Sphere<T> {
    pub fn new(center: Vec3, radius: f32, material: T) -> Self
    {
        Sphere { center_vec: center, center1: center, radius, material, is_moving: false }
    }
    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f32, material: T) -> Self { Sphere {center_vec: center2 - center1, center1, radius, material, is_moving: true} }

    pub fn sphere_center(&self, time: f32) -> Vec3
    {
        return self.center1 + (time * self.center_vec);
    }
}

impl<T: Material> Hittable for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center = if self.is_moving { self.sphere_center(ray.time) } else { self.center1 };
        let oc = center - ray.origin;

        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c= oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 { return None; }

        let sqrt = discriminant.sqrt();
        let mut root = (h - sqrt) / a;

        if !(t_min < root && root < t_max)
        {
            root = (h + sqrt) / a;
            if !(t_min < root && root < t_max)
            {
                return None;
            }
        }

        let p = ray.at(root);
        let (u, v) = get_sphere_uv(&p);
        let mut hit = HitRecord { point: p, normal: (p - center) / self.radius, t: root, material: &self.material, front_face: false, u, v};
        hit.set_normal(ray);
        Some(hit)
    }
}