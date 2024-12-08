use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Quad<T: Material>
{
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: T,
    normal: Vec3,
    d: f32,
}

impl<T: Material> Quad<T>
{
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: T) -> Quad<T>
    {
        let n = u.cross(v);
        let normal = n.normalize();
        Quad { q, u, v, w: n / n.dot(n), material, normal, d: normal.dot(q) }
    }
}

impl<T: Material> Hittable for Quad<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < 1e-8 { return None; }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;

        if t < t_min || t > t_max
        {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hit_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hit_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_vector));

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0
        {
            return None;
        }

        let mut hit_record = HitRecord {
            point: intersection,
            normal: self.normal,
            t,
            material: &(self.material),
            front_face: false,
            u: alpha,
            v: beta,
        };
        hit_record.set_normal(ray);

        return Some(hit_record)
    }
}