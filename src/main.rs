mod camera;
mod color;
mod hittable;
mod material;
mod perlin;
mod quad;
mod ray;
mod sphere;
mod texture;
mod vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::*;
use crate::material::*;
use crate::quad::Quad;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use crate::vec3::Vec3;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut disk_sampling = false;

    if args.len() == 3 && args[2] == *"disk_sampling" {
        disk_sampling = true;
    } else if args.len() == 1 {
        println!("Example Usage: raytracer 1 disk-sampling.\nWhere 1 is the scene number and disk-sampling is an optional argument.\nTo see all avaliable scenes types: raytracer list");
        return;
    }

    match args[1].as_str() {
        "list" => {
            println!("1: Bouncing Spheres");
            println!("2: Checkered Spheres");
            println!("3: Earth (Please supply a texture with name earthmap.jpg)");
            println!("4: Perlin spheres");
            println!("5: Quadrilaterals");
            println!("6: Ligt with Perlin Spheres");
            println!("7: Cornell Box");
        }
        "1" => bouncing_spheres(disk_sampling),
        "2" => checkered_spheres(disk_sampling),
        "3" => earth(disk_sampling),
        "4" => perlin_spheres(disk_sampling),
        "5" => quads(disk_sampling),
        "6" => simple_light(disk_sampling),
        "7" => cornell_box(disk_sampling),
        _ => println!("To see all avaliable scenes: raytracer list"),
    }
}

fn cornell_box(disk_sampling: bool) {
    let mut camera: Camera = Camera::new(
        10.0,
        0.0,
        40.0,
        Vec3::new(278.0, 278.0, -800.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        800,
        1000,
        50,
    );
    camera.background = Color::new_zero();

    let mut world = HittableList::new(vec![]);

    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = DiffuseMaterial::new(SolidColor::new(15.0, 15.0, 15.0));

    world.objects.push(Box::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white,
    )));

    camera.render(&world, disk_sampling);
}

fn simple_light(disk_sampling: bool) {
    let mut camera: Camera = Camera::new(
        10.0,
        0.0,
        20.0,
        Vec3::new(26.0, 3.0, 6.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        16.0 / 9.0,
        800,
        10000,
        50,
    );
    camera.background = Color::new_zero();

    let mut world = HittableList::new(vec![]);
    let perlin_texture = NoiseTexture::new(4.0);
    let diffuse_light = DiffuseMaterial::new(SolidColor::new(4.0, 4.0, 4.0));

    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(perlin_texture.clone()),
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(perlin_texture),
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diffuse_light.clone(),
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light,
    )));

    camera.render(&world, disk_sampling);
}

fn quads(disk_sampling: bool) {
    let camera: Camera = Camera::new(
        10.0,
        0.0,
        80.0,
        Vec3::new(0.0, 0.0, 9.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        800,
        100,
        50,
    );
    let mut world = HittableList::new(vec![]);

    let left_red = Lambertian::new(SolidColor::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new(SolidColor::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(SolidColor::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new(SolidColor::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new(SolidColor::new(0.2, 0.8, 0.8));

    world.objects.push(Box::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.objects.push(Box::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    camera.render(&world, disk_sampling);
}

fn perlin_spheres(disk_sampling: bool) {
    let camera: Camera = Camera::new(
        10.0,
        0.0,
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        16.0 / 9.0,
        800,
        100,
        50,
    );
    let mut world = HittableList::new(vec![]);

    let perlin_texture = NoiseTexture::new(4.0);
    let surface1 = Lambertian::new(perlin_texture.clone());
    let surface2 = Lambertian::new(perlin_texture);

    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        surface1,
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        surface2,
    )));
    camera.render(&world, disk_sampling);
}

fn earth(disk_sampling: bool) {
    let camera: Camera = Camera::new(
        10.0,
        0.0,
        20.0,
        Vec3::new(0.0, 0.0, 12.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        16.0 / 9.0,
        800,
        100,
        50,
    );
    let mut world = HittableList::new(vec![]);

    let earth_texture = ImageTexture::new("earthmap.jpg");
    let earth_surface = Lambertian::new(earth_texture);

    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new_zero(), 2.0, earth_surface)));
    camera.render(&world, disk_sampling);
}

fn checkered_spheres(disk_sampling: bool) {
    let camera: Camera = Camera::new(
        10.0,
        0.0,
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        16.0 / 9.0,
        800,
        100,
        50,
    );
    let mut world = HittableList::new(vec![]);

    let checker = CheckerTexture::new(
        75.0,
        100.0,
        SolidColor::new(0.2, 0.3, 0.1),
        SolidColor::new(0.9, 0.9, 0.9),
    );
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(checker),
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(checker),
    )));

    camera.render(&world, disk_sampling);
}

fn bouncing_spheres(disk_sampling: bool) {
    let camera: Camera = Camera::new(
        10.0,
        0.6,
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        16.0 / 9.0,
        800,
        100,
        50,
    );
    let world = generate_random_scene();
    camera.render(&world, disk_sampling);
}

fn generate_random_scene() -> HittableList {
    let mut world = HittableList::new(vec![]);
    let checker = CheckerTexture::new(
        1.0,
        0.5,
        SolidColor::new(0.2, 0.3, 0.1),
        SolidColor::new(0.9, 0.9, 0.9),
    );
    let ground_material = Lambertian::new(checker);

    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let rnd = rng.gen::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if rnd < 0.8 {
                    let color = Color::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
                        * Color::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
                    let albedo = SolidColor::new(color.x(), color.y(), color.z());
                    let sphere_material = Lambertian::new(albedo);
                    world
                        .objects
                        .push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if rnd < 0.95 {
                    let albedo = Color::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                    );
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world
                        .objects
                        .push(Box::new(Sphere::new(center, 0.2, sphere_material)))
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world
                        .objects
                        .push(Box::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
