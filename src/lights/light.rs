use crate::vec3::{Color, Point3, random_in_unit_sphere};
use crate::hittables::hittable::{HitRecord, Hittable, HittableTrait};
use crate::ray::{create_ray};
use std::sync::Arc;

pub struct Light{
    pub(crate) position: Point3,
    pub(crate) color: Color,
    pub(crate) size: f64
}
impl Light{
    pub fn is_shadow(&self, rec: &HitRecord, world: Arc<Hittable>) -> bool{
        let dir = self.position - rec.point + random_in_unit_sphere() * self.size;
        return world.hit(&create_ray(rec.point, dir), 0.001, 1.0).is_some();
    }
}