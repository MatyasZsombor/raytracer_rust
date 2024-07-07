mod vec3;
mod color;
mod ray;
mod hittable;
mod camera;
mod material;

use std::env;
use crate::camera::Camera;
use crate::color::Color;
use crate::vec3::Vec3;
use crate::hittable::*;
use crate::material::Material::{Lambertian, Metal};

fn main()
{
    let camera: Camera = Camera::new(16.0 / 9.0, 800, Vec3::new_zero(), 10, 50);

    let material_ground = Lambertian {attenuation: Color::new(0.8,0.8,0.0)};
    let material_center = Lambertian {attenuation: Color::new(0.1,0.2,0.5)};
    let material_left = Metal {attenuation: Color::new(0.8, 0.8, 0.8)};
    let material_right = Metal {attenuation: Color::new(0.8, 0.6, 0.2)};

    let mut world : HittableList = HittableList::new(vec![]);

    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.objects.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.objects.push(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let args: Vec<String> = env::args().collect();
    let mut disk_sampling = false;

    if args.len() == 2 && args[1] == "disk_sampling".to_string()
    {
        disk_sampling = true;
    }

    camera.render(&world, disk_sampling);
}