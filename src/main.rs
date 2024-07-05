use std::fmt::format;
use std::fs;
use std::fs::File;

fn main()
{
    let image_width = 256;
    let image_height = 256;

    let mut string: String = "".to_string();

    string.push_str(&format!("P3\n{image_width} {image_height}\n{}\n", 255));


    for j in 0..image_height {
        for i in 0..image_width
        {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            string.push_str(&format!("{ir} {ig} {ib}\n"));
        }
    }

    fs::write("helloworld.ppm", string).unwrap();
}