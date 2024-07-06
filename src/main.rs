mod vec3;
mod color;
mod ray;
mod hittable;
mod camera;

use std::env;
use crate::camera::Camera;
use crate::vec3::Vec3;
use crate::hittable::*;

fn main()
{
    let camera: Camera = Camera::new(16.0/ 9.0, 800, Vec3::new_zero(), 100);

    let mut world : HittableList = HittableList::new(vec![]);
    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let args: Vec<String> = env::args().collect();

    let mut disk_sampling = false;

    if args.len() == 1 && args[0] == "--disk_sampling"
    {
        disk_sampling = true;
    }

    camera.render(&world, disk_sampling);
}