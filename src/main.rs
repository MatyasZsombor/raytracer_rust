mod vec3;
mod color;
mod ray;
mod hittable;

use std::fs;
use crate::color::*;
use colored::{Colorize};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::*;


fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color
{
    match world.hit(ray, 0.0, f32::INFINITY)
    {

        Some(hit_record) => {
            0.5 * (hit_record.normal + Color::new(1.0,1.0,1.0))
        }
        None => {
            let alpha = 0.5 * (ray.direction().normalize().y() + 1.0);
            (1.0 - alpha) * Color::new(1.0, 1.0, 1.0) + alpha * Color::new(0.5, 0.7, 1.0)
        }
    }
}


fn main()
{
    let image_width = 800;
    let aspect_ratio = 16.0 / 9.0;

    let mut image_height = (image_width as f32 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height} ;

    let focal = 1.0;
    let viewport_height : f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_pos = Vec3::new_zero();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport00 = camera_pos - Vec3::new(0.0,0.0,focal) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00 = viewport00 + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut world : HittableList = HittableList::new(vec![]);
    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut string: String = "".to_string();

    string.push_str(&format!("P3\n{image_width} {image_height}\n{}\n", 255));

    for j in 0..image_height {
        println!("{} {}", "\rScan lines remaining ".red(), (image_height - j).to_string().red());

        for i in 0..image_width
        {
            let pixel_center = pixel00 + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray = Ray::new(camera_pos, pixel_center - camera_pos);

            let pixel_color = ray_color(&ray, &world);
            string.push_str(&write_color(&pixel_color));
        }
        print!("\x1B[2J\x1B[1;1H");
    }
    println!("{}", "Done".green());


    fs::write("helloworld.ppm", string).unwrap();
}