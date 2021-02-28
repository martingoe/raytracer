use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub(crate) origin: Point3,
    pub(crate) direction: Vec3
}

impl Ray{
    pub(crate) fn at(&self, t: f64) -> Point3{
        return self.origin + self.direction * t;
    }
}