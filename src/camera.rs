use crate::vec3::{Vec3, cross, Point3};
use crate::utils::deg_to_rad;
use crate::ray::Ray;


#[derive(Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

pub fn create_camera(look_from: Point3, look_at: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64, focus_dist: f64) -> Camera {
    let theta = deg_to_rad(vfov);
    let h = (theta / 2.0).tan();

    let height = 2.0 * h;
    let width = aspect_ratio * height;

    let w = (look_from - look_at).unit_vector();
    let u = cross(&vup, &w).unit_vector();
    let v = cross(&w, &u);

    let horizontal = u * (focus_dist * width);
    let vertical = v * (focus_dist * height);
    let lower_left_corner = look_from - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

    return Camera { origin: look_from, lower_left_corner, horizontal, vertical };
}

impl Camera {
    pub(crate) fn get_ray(self, s: f64, t: f64) -> Ray {
        return Ray { origin: self.origin, direction: self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin };
    }
}