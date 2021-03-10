use std::f64::consts::PI;

pub fn deg_to_rad(deg: f64) -> f64 {
    return deg * PI / 180.0;
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

pub fn random_double(min: f64, max: f64) -> f64 {
    return min + (max - min) * rand::random::<f64>();
}