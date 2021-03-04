use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub(crate) origin: Point3,
    pub(crate) direction: Vec3,
    pub(crate) inv_direction: Vec3,
    pub(crate) sign: [usize; 3],
}

pub fn create_ray(origin: Point3, direction: Vec3) -> Ray {
    let inv_direction = Vec3 { e: [1.0 / direction.x(), 1.0 / direction.y(), 1.0 / direction.z()] };
    let mut sign: [usize; 3] = [0; 3];
    sign[0] = if inv_direction.x() < 0.0 {1} else {0};
    sign[1] = if inv_direction.y() < 0.0 {1} else {0};
    sign[2] = if inv_direction.z() < 0.0 {1} else {0};

    return Ray { origin, direction, inv_direction, sign };
}

impl Ray {
    pub(crate) fn at(&self, t: f64) -> Point3 {
        return self.origin + self.direction * t;
    }
}