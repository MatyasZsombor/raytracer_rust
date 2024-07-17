use crate::material::Material;
use crate::quad::Quad;
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
    pub u: f32,
    pub v: f32
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

pub fn new_box<T: Material + Clone + 'static>(a: &Vec3, b: &Vec3, mat: &T) -> HittableList
{
    let mut sides = HittableList::new(vec![]);

    let min = Vec3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Vec3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0,0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0,0.0, max.z() - min.z());

    sides.objects.push(Box::new(Quad::new(Vec3::new(min.x(), min.y(), max.z()), dx, dy, mat.clone())));
    sides.objects.push(Box::new(Quad::new(Vec3::new(max.x(), min.y(), max.z()), -dz, dy, mat.clone())));
    sides.objects.push(Box::new(Quad::new(Vec3::new(max.x(), min.y(), min.z()), -dx, dy, mat.clone())));
    sides.objects.push(Box::new(Quad::new(Vec3::new(min.x(), min.y(), min.z()), dz, dy, mat.clone())));
    sides.objects.push(Box::new(Quad::new(Vec3::new(min.x(), max.y(), max.z()), dx, -dz, mat.clone())));
    sides.objects.push(Box::new(Quad::new(Vec3::new(min.x(), min.y(), min.z()), dx, dz, mat.clone())));

    sides
}