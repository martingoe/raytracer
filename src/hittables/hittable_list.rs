use crate::hittables::hittable::{Hittable, HitRecord};
use std::sync::Arc;
use crate::ray::Ray;

#[derive(Clone)]
pub struct HittableList {
    pub(crate) list: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn add(&mut self, new_element: Arc<dyn Hittable>) {
        self.list.push(new_element)
    }
}

impl Hittable for HittableList {
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
}