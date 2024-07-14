use std::fs;
use std::time::Instant;
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use rayon::prelude::*;

use crate::color::{Color, write_color};
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera
{
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    pixel_samples_scale: f32,
    image_height: i32,
    center: Vec3,
    pixel00: Vec3,
    delta_u: Vec3,
    delta_v: Vec3,
    defocus_u: Vec3,
    defocus_v: Vec3,
}

impl Camera
{
    pub fn new(focus_distance: f32, defocus_angle: f32, vfov: f32, from: Vec3, at: Vec3, up: Vec3, aspect_ratio: f32, image_width: i32, samples_per_pixel: i32, max_depth: i32) -> Self
    {
        let mut image_height = (image_width as f32 / aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let theta = vfov * std::f32::consts::PI / 180.0;
        let h = f32::tan(theta / 2.0);

        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (from - at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let delta_u = viewport_u / image_width as f32;
        let delta_v = viewport_v / image_height as f32;

        let viewport00 = from - focus_distance * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport00 + 0.5 * (delta_u + delta_v);

        let defocus_radius = focus_distance * f32::tan(defocus_angle * std::f32::consts::PI / 360.0);

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f32,
            max_depth,
            image_height,
            defocus_angle,
            focus_distance,
            center: from,
            pixel00,
            delta_u,
            delta_v,
            defocus_u: u * defocus_radius,
            defocus_v: v * defocus_radius,
        }
    }

    pub fn render(&self, world: &HittableList, disk_sampling: bool)
    {
        let mut string: String = "".to_string();

        string.push_str(&format!("P3\n{} {}\n{}\n", self.image_width, self.image_height, 255));

        let start = Instant::now();

        let pixels = (0..self.image_height).into_par_iter().map(|h| {
            (0..self.image_width).into_par_iter().map(|w| {
                let rng = &mut thread_rng();
                let mut pixel_color = Color::new_zero();

                for _ in 0..self.samples_per_pixel
                {
                    let ray = self.get_ray(w, h, disk_sampling, rng);
                    pixel_color = pixel_color + Camera::ray_color(&ray, self.max_depth, world);
                }
                write_color(&(pixel_color * self.pixel_samples_scale))
            }).collect::<Vec<String>>().join("")
        }).collect::<Vec<String>>().join("");

        println!("{} in {:?}", "Done", start.elapsed());
        string.push_str(pixels.as_str());
        fs::write("helloworld.ppm", string).unwrap();
    }

    fn ray_color(ray: &Ray, max_depth: i32, world: &dyn Hittable) -> Color
    {
        if max_depth <= 0
        {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(ray, 0.001, f32::INFINITY)
        {
            Some(hit_record) => {
                let result = hit_record.material.scatter(&ray, &hit_record);

                if result.2 {
                    return result.1 * Self::ray_color(&result.0, max_depth - 1, world);
                }
                Vec3::new_zero()
            }
            None => {
                let alpha = 0.5 * (ray.direction.normalize().y() + 1.0);
                (1.0 - alpha) * Color::new(1.0, 1.0, 1.0) + alpha * Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn get_ray(&self, i: i32, j: i32, disk_sampling: bool, rng: &mut ThreadRng) -> Ray
    {
        let offset = if disk_sampling
        { Camera::sample_disk(1.0, rng) } else { Camera::sample_square(rng) };

        let pixel_sample = self.pixel00
            + ((i as f32 + offset.x()) * self.delta_u)
            + ((j as f32 + offset.y()) * self.delta_v);

        let ray_origin =
            if self.defocus_angle <= 0.0
            {
                self.center
            } else {
                self.sample_defocus_disk(rng)
            };

        return Ray::new(ray_origin, pixel_sample - ray_origin, rng.gen::<f32>());
    }

    fn sample_square(rng: &mut ThreadRng) -> Vec3
    {
        Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5, 0.0)
    }

    fn sample_disk(radius: f32, rng: &mut ThreadRng) -> Vec3
    {
        let mut p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        while p.length_squared() >= 1.0 {
            p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        }
        p * radius
    }

    fn sample_defocus_disk(&self, rng: &mut ThreadRng) -> Vec3
    {
        let mut p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        while p.length_squared() >= 1.0 {
            p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        }
        self.center + (p.x() * self.defocus_u) + (p.y() * self.defocus_v)
    }
}