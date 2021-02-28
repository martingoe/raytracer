use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};


pub struct HitRecord {
    pub(crate) point: Point3,
    pub(crate) normal: Vec3,
    pub(crate) material: Arc<dyn Material>,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
