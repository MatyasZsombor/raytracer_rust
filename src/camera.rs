use std::fs;
use colored::Colorize;
use rand::Rng;

use crate::color::{Color, write_color};
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Camera
{
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pixel_samples_scale: f32,
    image_height: i32,
    center: Vec3,
    pixel00: Vec3,
    delta_u: Vec3,
    delta_v: Vec3,
}

impl Camera
{
    pub fn new(aspect_ratio: f32, image_width: i32, center: Vec3, samples_per_pixel: i32) -> Self
    {
        let mut image_height = (image_width as f32 / aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height} ;

        let focal = 1.0;
        let viewport_height : f32 = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let camera_pos = Vec3::new_zero();

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let delta_u = viewport_u / image_width as f32;
        let delta_v = viewport_v / image_height as f32;

        let viewport00 = camera_pos - Vec3::new(0.0,0.0,focal) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport00 + 0.5 * (delta_u + delta_v);


        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f32,
            image_height,
            center,
            pixel00,
            delta_u,
            delta_v
        }
    }

    pub fn render(&self, world: &HittableList, disk_sampling: bool)
    {
        let mut string: String = "".to_string();

        string.push_str(&format!("P3\n{} {}\n{}\n", self.image_width, self.image_height, 255));

        for j in 0..self.image_height {
            println!("{} {}", "\rScan lines remaining ".red(), (self.image_height - j).to_string().red());

            for i in 0..self.image_width
            {
                let mut pixel_color = Color::new_zero();

                for _ in 0..self.samples_per_pixel
                {
                    let ray = self.get_ray(i, j, disk_sampling);
                    pixel_color = pixel_color + Camera::ray_color(&ray, world);
                }
                string.push_str(&write_color(&(pixel_color * self.pixel_samples_scale)));
            }

            print!("\x1B[2J\x1B[1;1H");
        }
        println!("{}", "Done".green());
        fs::write("helloworld.ppm", string).unwrap();
    }

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

    fn get_ray(&self, i : i32, j: i32, disk_sampling: bool) -> Ray
    {
        let offset = if disk_sampling
        { Camera::sample_disk(1.0) }
        else { Camera::sample_square() };

        let pixel_sample = self.pixel00
            + ((i as f32 + offset.x()) * self.delta_u)
            + ((j as f32 + offset.y()) * self.delta_v);

        return Ray::new(self.center, pixel_sample - self.center);
    }

    fn sample_square() -> Vec3
    {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5,0.0)
    }

    fn sample_disk(radius: f32) -> Vec3
    {
        let mut rng = rand::thread_rng();
        let mut p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        while p.length_squared() >= 1.0 {
            p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        }
        p * radius
    }
}