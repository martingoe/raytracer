use crate::utils::math_utils::clamp;
use crate::vec3::Color;
use std::fs::File;
use std::io::Write;

pub fn write_color(file: &mut File, pixel_color: Color, samples_per_pixel: usize) {
    let new_col = scale_color(&pixel_color, samples_per_pixel);
    // Write the translated [0,255] value of each color component.
    write!(file, "{} ", (256.0 * clamp(new_col.x(), 0.0, 0.999)) as i32).expect("Cannot write to file.");
    write!(file, "{} ", (256.0 * clamp(new_col.y(), 0.0, 0.999)) as i32).expect("Cannot write to file.");
    write!(file, "{} \n", (256.0 * clamp(new_col.z(), 0.0, 0.999)) as i32).expect("Cannot write to file.");
}

pub fn scale_color(color: &Color, samples_per_pixel: usize) -> Color{
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();
    return Color{ e: [r, g, b] };
}