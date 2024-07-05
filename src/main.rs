mod vec3;
mod color;
mod ray;

use std::fs;
use crate::color::*;
use colored::{Colorize};
use crate::vec3::Vec3;
use crate::ray::Ray;


fn ray_color(ray: &Ray) -> Color
{
    if sphere_hit(Vec3::new(0.0,0.0,-1.0), 0.5, ray)
    {
        return Color::new(1.0, 0.0,0.0);
    }

    let alpha = 0.5 * (ray.direction().normalize().y() + 1.0);
    (1.0 - alpha) * Color::new(1.0,1.0,1.0) + alpha * Color::new(0.5, 0.7, 1.0)
}

fn sphere_hit(center: Vec3, radius: f32, ray: &Ray) -> bool
{
    let oc: Vec3 = center - ray.origin();

    let a = ray.direction().dot(ray.direction());
    let b = -2.0 * ray.direction().dot(oc);
    let c = oc.dot(oc) - radius * radius;

    b * b - 4.0 * a * c >= 0.0
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


    let mut string: String = "".to_string();

    string.push_str(&format!("P3\n{image_width} {image_height}\n{}\n", 255));

    for j in 0..image_height {
        println!("{} {}", "\rScan lines remaining ".red(), (image_height - j).to_string().red());

        for i in 0..image_width
        {
            let pixel_center = pixel00 + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray = Ray::new(camera_pos, pixel_center - camera_pos);

            let pixel_color = ray_color(&ray);
            string.push_str(&write_color(&pixel_color));
        }
        print!("\x1B[2J\x1B[1;1H");
    }
    println!("{}", "Done".green());


    fs::write("helloworld.ppm", string).unwrap();

}