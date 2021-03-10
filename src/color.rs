use std::fs::File;
use crate::vec3::Color;
use std::io::Write;
use crate::utils::math_utils::clamp;

pub fn write_color(file: &mut File, pixel_color: Color, samples_per_pixel: i32){
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write the translated [0,255] value of each color component.
    write!(file, "{} ", (256.0 * clamp(r, 0.0, 0.999)) as i32).expect("Cannot write to file.");
    write!(file, "{} ", (256.0 * clamp(g, 0.0, 0.999)) as i32).expect("Cannot write to file.");
    write!(file, "{} \n", (256.0 * clamp(b, 0.0, 0.999)) as i32).expect("Cannot write to file.");
}