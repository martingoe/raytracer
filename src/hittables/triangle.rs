use crate::vec3::{Point3, cross, dot};
use crate::hittables::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;
use std::sync::Arc;
use std::f64::EPSILON;

pub struct Triangle{
    pub(crate) a: Point3,
    pub(crate) b: Point3,
    pub(crate) c: Point3,
    pub(crate) material: Arc<dyn Material>
}
impl Hittable for Triangle{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let h = cross(&ray.direction, &ac);
        let det = dot(&ab, &h);
        if det < EPSILON && det > -EPSILON {
            return None;
        }
        let f = 1.0 / det;

        let s = ray.origin - self.a;
        let u = f * dot(&s, &h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = cross(&s, &ab);
        let v = f * dot(&ray.direction, &q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * dot(&ac, &q);

        if t < t_min || t > t_max {
            return None;
        }
        let normal = cross(&ac, &ab).unit_vector();
        let mut record = HitRecord { point: ray.at(t), normal, material: self.material.clone(), t, front_face: false };
        record.set_face_normal(ray, &normal);
        return Option::from(record);
    }
}