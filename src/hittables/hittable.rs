use std::sync::Arc;

use crate::material::MaterialTrait;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use crate::hittables::triangle::Triangle;
use crate::hittables::bvh::Bvh;
use crate::hittables::sphere::Sphere;
use crate::hittables::hittable_list::HittableList;


pub struct HitRecord {
    pub(crate) point: Point3,
    pub(crate) normal: Vec3,
    pub(crate) material: Arc<dyn MaterialTrait>,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

// pub trait Hittable: Sync + Send {
//     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
//     fn get_max_pos(&self) -> Vec3;
//     fn get_min_pos(&self) -> Vec3;
//     fn get_mean_pos(&self) -> Vec3;
// }

pub trait HittableTrait: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn get_min_pos(&self) -> Vec3;
    fn get_max_pos(&self) -> Vec3;
    fn get_mean_pos(&self) -> Vec3;
}

#[allow(dead_code)]
pub enum Hittable {
    Sphere { sphere: Sphere },
    Bvh { bvh: Bvh },
    Triangle { triangle: Triangle },
    HittableList { hittable_list: HittableList },
}

impl HittableTrait for Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        return match self {
            Hittable::Sphere { sphere } => sphere.hit(ray, t_min, t_max),
            Hittable::Bvh { bvh } => bvh.hit(ray, t_min, t_max),
            Hittable::Triangle { triangle } => triangle.hit(ray, t_min, t_max),
            Hittable::HittableList { hittable_list } => hittable_list.hit(ray, t_min, t_max)
        };
    }

    fn get_min_pos(&self) -> Vec3 {
        return match self {
            Hittable::Sphere { sphere } => sphere.get_min_pos(),
            Hittable::Bvh { bvh } => bvh.get_min_pos(),
            Hittable::Triangle { triangle } => triangle.get_min_pos(),
            Hittable::HittableList { hittable_list } => hittable_list.get_min_pos()
        };
    }

    fn get_max_pos(&self) -> Vec3 {
        return match self {
            Hittable::Sphere { sphere } => sphere.get_max_pos(),
            Hittable::Bvh { bvh } => bvh.get_max_pos(),
            Hittable::Triangle { triangle } => triangle.get_max_pos(),
            Hittable::HittableList { hittable_list } => hittable_list.get_max_pos()
        };
    }

    fn get_mean_pos(&self) -> Vec3 {
        return match self {
            Hittable::Sphere { sphere } => sphere.get_mean_pos(),
            Hittable::Bvh { bvh } => bvh.get_mean_pos(),
            Hittable::Triangle { triangle } => triangle.get_mean_pos(),
            Hittable::HittableList { hittable_list } => hittable_list.get_mean_pos()
        };
    }
}