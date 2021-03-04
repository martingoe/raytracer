use crate::vec3::{Point3, cross, dot, Vec3};
use crate::ray::Ray;
use crate::material::{Material};
use std::sync::Arc;
use std::f64::EPSILON;
use crate::hittables::hittable::{HitRecord, HittableTrait};

#[derive(Clone)]
pub struct Triangle{
    pub(crate) a: Point3,
    pub(crate) b: Point3,
    pub(crate) c: Point3,
    pub(crate) material: Arc<Material>
}
impl HittableTrait for  Triangle{
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

    fn get_min_pos(&self) -> Vec3 {
        let min_x = self.a.x().min(self.b.x()).min(self.c.x());
        let min_y = self.a.y().min(self.b.y()).min(self.c.y());
        let min_z = self.a.z().min(self.b.z()).min(self.c.z());
        return Vec3{ e: [min_x, min_y, min_z] };
    }

    fn get_max_pos(&self) -> Vec3 {
        let max_x = self.a.x().max(self.b.x()).max(self.c.x());
        let max_y = self.a.y().max(self.b.y()).max(self.c.y());
        let max_z = self.a.z().max(self.b.z()).max(self.c.z());
        return Vec3{ e: [max_x, max_y, max_z] };
    }

    fn get_mean_pos(&self) -> Vec3 {
        return (self.a + self.b + self.c) / 3.0;
    }

}