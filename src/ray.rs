use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray
{
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
    pub time: f32
}

impl Ray
{
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray
    {
        Ray {origin, direction, time}
    }

    pub fn at(self, t: f32) -> Vec3
    {
        return self.origin + t * self.direction;
    }
}