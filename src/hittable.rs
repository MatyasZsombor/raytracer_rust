use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord
{
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Material,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f32, point: Vec3, normal: Vec3, material: Material) -> Self
    {
        HitRecord { point, normal, t, material, front_face: false }
    }

    pub fn set_normal(&mut self, ray: &Ray)
    {
        self.front_face = ray.direction.dot(self.normal) < 0.0;
        self.normal = if self.front_face { self.normal } else { -self.normal };
    }
}

pub trait Hittable
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere
{
    center_vec: Vec3,
    center1: Vec3,
    radius: f32,
    material: Material,
    is_moving: bool,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self
    {
        Sphere { center_vec: center, center1: center, radius, material, is_moving: false }
    }
    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f32, material: Material) -> Self { Sphere {center_vec: center2 - center1, center1, radius, material, is_moving: true} }

    pub fn sphere_center(&self, time: f32) -> Vec3
    {
        return self.center1 + (time * self.center_vec);
    }
}

impl Hittable for Sphere {
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
        let mut hit = HitRecord::new(root, p, (p - center) / self.radius, self.material);
        hit.set_normal(ray);
        Some(hit)
    }
}

pub struct HittableList
{
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self
    {
        HittableList { objects }
    }
}

unsafe impl Sync for HittableList {}

unsafe impl Send for HittableList {}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut res = None;

        for h in &self.objects
        {
            if let Some(hit_record) = h.hit(ray, t_min, closest)
            {
                closest = hit_record.t;
                res = Some(hit_record);
            }
        }

        res
    }
}