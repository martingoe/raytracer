use std::sync::Arc;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittables::hittable::{Hittable, HitRecord, HittableTrait};
use crate::hittables::bvh::BBox;

#[derive(Clone)]
pub struct HittableList {
    pub(crate) list: Vec<Arc<Hittable>>
}

impl HittableList {
    #[allow(dead_code)]
    pub fn add(&mut self, new_element: Arc<Hittable>) {
        self.list.push(new_element)
    }
}

impl HittableTrait for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut return_value: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in self.list.iter() {
            let option = hittable.clone().hit(ray, t_min, closest_so_far);
            if option.is_some() {
                let rec = option.unwrap();
                if rec.t < closest_so_far {
                    closest_so_far = rec.t;
                    return_value = Option::Some(rec);
                }
            }
        }
        return return_value;
    }

    fn get_min_pos(&self) -> Vec3 {
        unimplemented!()
    }

    fn get_max_pos(&self) -> Vec3 {
        unimplemented!()
    }

    fn get_mean_pos(&self) -> Vec3 {
        unimplemented!()
    }

    fn get_bbox(&self) -> BBox {
        BBox{ bounds: [self.get_min_pos(), self.get_max_pos()] }
    }
}