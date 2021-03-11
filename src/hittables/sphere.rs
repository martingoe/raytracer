use crate::vec3::{Point3, dot, Vec3};
use crate::ray::Ray;
use std::sync::Arc;
use crate::material::{Material};
use crate::hittables::hittable::{HitRecord, HittableTrait};
use crate::hittables::bvh::BBox;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct Sphere {
    pub(crate) position: Point3,
    pub(crate) radius: f64,
    pub(crate) material: Arc<Material>,
}
impl Sphere{
    fn get_uv(&self, p: Point3) -> (f64, f64){
        let u = 0.5 + (p.x().atan2(p.z())) / 2.0 * PI;
        let v = 0.5 - (p.y().asin()) / PI;
        return (u, v);
    }
}

impl HittableTrait for Sphere{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.position;
        let a = ray.direction.length_squared();
        let b = dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let squared = discriminant.sqrt();

        let mut t = (-b - squared) / a;
        if t < t_min || t > t_max {
            t = (-b + squared) / a;
            if t < t_min || t_max < t {
                return None;
            }
        }
        let point = ray.at(t);
        let normal = (point - self.position) / self.radius;
        let (u, v) = self.get_uv(normal);
        let mut rec = HitRecord { point, normal, material: self.material.clone(), t, u, v, front_face: false };
        rec.set_face_normal(ray, &normal);

        return Option::from(rec);
    }

    fn get_min_pos(&self) -> Vec3 {
        return self.position + -self.radius;
    }

    fn get_max_pos(&self) -> Vec3 {
        return self.position + self.radius;
    }

    fn get_mean_pos(&self) -> Vec3 {
        return self.position;
    }

    fn get_bbox(&self) -> BBox {
        BBox{ bounds: [self.get_min_pos(), self.get_max_pos()] }
    }
}