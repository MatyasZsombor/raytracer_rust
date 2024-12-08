use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn linear_to_gamma(linear: f32) -> f32 {
    linear.sqrt()
}

pub fn write_color(pixel_color: &Color) -> String {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let r_byte = (255.999 * r) as i32;
    let g_byte = (255.999 * g) as i32;
    let b_byte = (255.999 * b) as i32;

    format!("{r_byte} {g_byte} {b_byte}\n")
}

