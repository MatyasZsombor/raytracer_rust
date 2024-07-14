use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord<'a>
{
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: &'a dyn Material,
    pub front_face: bool,
}

impl HitRecord<'_> {
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