mod vec3;
mod color;
mod ray;
mod hittable;
mod camera;
mod material;
mod sphere;

use std::env;
use rand::Rng;
use crate::camera::Camera;
use crate::color::Color;
use crate::vec3::Vec3;
use crate::hittable::*;
use crate::material::*;
use crate::sphere::Sphere;

fn main()
{
    let camera: Camera = Camera::new(20.0, Vec3::new(13.0,2.0,3.0), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.0),16.0 / 9.0, 800, 10, 50);

    let world = generate_random_scene();

    let args: Vec<String> = env::args().collect();
    let mut disk_sampling = false;

    if args.len() == 2 && args[1] == "disk_sampling".to_string()
    {
        disk_sampling = true;
    }

    camera.render(&world, disk_sampling);
}

fn generate_random_scene() -> HittableList
{
    let mut world = HittableList::new(vec![]);
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));
    let mut rng = rand::thread_rng();


    for a in -11..11 {
        for b in -11..11 {
            let rnd = rng.gen::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9
            {
                if rnd < 0.8
                {
                    let albedo = Color::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * Color::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
                    let sphere_material = Lambertian::new(albedo);
                    world.objects.push(Box::new(Sphere::new_moving(center, center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0),0.2, sphere_material)));
                }
                else if rnd < 0.95
                {
                    let albedo = Color::new(rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0));
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.objects.push(Box::new(Sphere::new(center, 0.2, sphere_material)))
                }
                else
                {
                    let sphere_material = Dielectric::new(1.5);
                    world.objects.push(Box::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0, material1)));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.objects.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));


    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.objects.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}