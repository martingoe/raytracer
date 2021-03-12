use std::f64::{MAX, MIN};
use std::sync::Arc;

use crate::hittables::hittable::{HitRecord, Hittable, HittableTrait};
use crate::ray::Ray;
use crate::utils::morton_code::bvh_morton;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Bvh {
    pub(crate) bounds: BBox,
    pub(crate) left: Arc<Hittable>,
    pub(crate) right: Arc<Hittable>,
}


impl HittableTrait for Bvh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let (Hittable::Bvh { bvh }, Hittable::Bvh { bvh: bvh2 }) = (&self.left.as_ref(), &self.right.as_ref()) {
            return self.both_bvh(bvh, bvh2, ray, t_min, t_max);
        }
        let left = self.left.hit(ray, t_min, t_max);
        let right = self.right.hit(ray, t_min, t_max);
        return return_closest(left, right);
    }

    fn get_min_pos(&self) -> Vec3 {
        self.bounds.min()
    }

    fn get_max_pos(&self) -> Vec3 {
        self.bounds.max()
    }

    fn get_mean_pos(&self) -> Vec3 {
        Vec3 { e: [(self.bounds.max().e[0] + self.bounds.min().e[0]) / 2.0, (self.bounds.max().e[1] + self.bounds.min().e[1]) / 2.0, (self.bounds.max().e[2] + self.bounds.min().e[2]) / 2.0] }
    }
    fn get_bbox(&self) -> BBox {
        BBox { bounds: [self.get_min_pos(), self.get_max_pos()] }
    }
}

impl Bvh {
    fn both_bvh(&self, left: &Bvh, right: &Bvh, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let left_intersect = left.bounds.ray_intersects(ray, t_min, t_max);
        let right_intersect = right.bounds.ray_intersects(ray, t_min, t_max);
        if right_intersect.is_none() && left_intersect.is_some() {
            return left.hit(ray, t_min, t_max);
        } else if left_intersect.is_none() && right_intersect.is_some() {
            return right.hit(ray, t_min, t_max);
        } else if left_intersect.is_some() && right_intersect.is_some() {
            let left_t = left_intersect.unwrap();

            let right_t = right_intersect.unwrap();
            return if left_t < right_t {
                Bvh::get_closest_hit(right, left, ray, t_min, t_max, right_t)
            } else {
                Bvh::get_closest_hit(left, right, ray, t_min, t_max, left_t)
            };
        }
        return None;
    }

    fn get_closest_hit(further: &Bvh, closer: &Bvh, ray: &Ray, t_min: f64, t_max: f64, further_box_entry: f64) -> Option<HitRecord> {
        let close_option = closer.hit(ray, t_min, t_max);
        if close_option.is_some() {
            let record = close_option.unwrap();
            if record.t < further_box_entry {
                return Some(record);
            }
            return return_closest(Some(record), further.hit(ray, t_min, t_max));
        }
        further.hit(ray, t_min, t_max)
    }
}

fn return_closest(left: Option<HitRecord>, right: Option<HitRecord>) -> Option<HitRecord> {
    if left.is_some() && right.is_some() {
        let x = left.unwrap();
        let x1 = right.unwrap();
        if x.t <= x1.t {
            return Some(x);
        }
        return Some(x1);
    }
    if left.is_some() {
        return left;
    }
    return right;
}


pub fn initiate_bvh(elements: &mut Vec<Arc<Hittable>>) -> Arc<Hittable> {
    if elements.len() == 1 {
        return elements[0].clone();
    }

    let b_box = surround(elements);
    if elements.len() == 2 {
        return Arc::from(Hittable::Bvh {
            bvh: Bvh {
                bounds: b_box,
                left: elements[0].clone(),
                right: elements[1].clone(),
            }
        });
    }
    let axis = get_axis(&b_box);

    let half = mean_split(elements, axis);
    println!("Previous: {}, Now: {}", elements.len(), half);

    let left = initiate_bvh(&mut elements[..half as usize].to_vec());
    let right = initiate_bvh(&mut elements[half as usize..].to_vec());
    return Arc::from(Hittable::Bvh {
        bvh: Bvh {
            bounds: b_box,
            left,
            right,
        }
    });
}

fn get_axis(bbox: &BBox) -> i32 {
    let lengths = bbox.max() - bbox.min();
    let x = lengths.e[0].abs();
    let y = lengths.e[1].abs();
    let z = lengths.e[2].abs();
    if x >= y && x >= z {
        return 0;
    }
    if y >= x && y >= z {
        return 1;
    }
    return 2;
}

fn mean_split(elements: &mut Vec<Arc<Hittable>>, axis: i32) -> i32 {
    let pivot = elements.iter().fold(0.0 as f64, |acc, x| acc + x.get_mean_pos().e[axis as usize]) / elements.len() as f64;
    let mut count = 0;
    for i in 0..elements.len() {
        let x = elements[i].get_mean_pos().e[axis as usize];
        if x <= pivot {
            let arc = elements.remove(i);
            elements.insert(0, arc);
            count += 1;
        }
    }
    if count == elements.len() {
        count -= 1;
    }
    if count == 0 {
        count += 1;
    }


    return count as i32;
}

pub fn surround(elements: &Vec<Arc<Hittable>>) -> BBox {
    let x_min = elements.iter().fold(MAX, |acc, x| acc.min((*x).get_min_pos().x()));
    let y_min = elements.iter().fold(MAX, |acc, x| acc.min((*x).get_min_pos().y()));
    let z_min = elements.iter().fold(MAX, |acc, x| acc.min((*x).get_min_pos().z()));

    let x_max = elements.iter().fold(MIN, |acc, x| acc.max((*x).get_max_pos().x()));
    let y_max = elements.iter().fold(MIN, |acc, x| acc.max((*x).get_max_pos().y()));
    let z_max = elements.iter().fold(MIN, |acc, x| acc.max((*x).get_max_pos().z()));
    return BBox {
        bounds: [Vec3 { e: [x_min, y_min, z_min] }, Vec3 { e: [x_max, y_max, z_max] }],
    };
}


#[derive(Clone)]
pub struct BBox {
    pub(crate) bounds: [Vec3; 2]
}

impl BBox {
    fn max(&self) -> Vec3 {
        return self.bounds[1];
    }
    fn min(&self) -> Vec3 {
        return self.bounds[0];
    }
    fn surface(&self) -> f64 {
        let l = self.bounds[1].x() - self.bounds[0].x();
        let w = self.bounds[1].y() - self.bounds[0].y();
        let h = self.bounds[1].z() - self.bounds[0].z();
        2.0 * (l * w + l * h + w * h)
    }
    pub(crate) fn half_area(&self) -> f64 {
        let extents = [
            self.max().e[0] - self.min().e[0],
            self.max().e[1] - self.min().e[1],
            self.max().e[2] - self.min().e[2]
        ];
        return extents[0] * (extents[1] + extents[2]) + extents[1] * extents[2];
    }
    pub(crate) fn union(&self, other: &BBox) -> BBox {
        return BBox {
            bounds: [Vec3 {
                e: [
                    if self.min().e[0] < other.min().e[0] { self.min().e[0] } else { other.min().e[0] },
                    if self.min().e[1] < other.min().e[1] { self.min().e[1] } else { other.min().e[1] },
                    if self.min().e[2] < other.min().e[2] { self.min().e[2] } else { other.min().e[2] }
                ]
            },
                Vec3 {
                    e: [
                        if self.max().e[0] < other.max().e[0] { self.max().e[0] } else { other.max().e[0] },
                        if self.max().e[1] < other.max().e[1] { self.max().e[1] } else { other.max().e[1] },
                        if self.max().e[2] < other.max().e[2] { self.max().e[2] } else { other.max().e[2] }
                    ]
                }]
        };
    }
    pub fn ray_intersects(&self, ray: &Ray, t0: f64, t1: f64) -> Option<f64> {
        let mut t_min = (self.bounds[ray.sign[0]].x() - ray.origin.x()) * ray.inv_direction.x();
        let mut t_max = (self.bounds[1 - ray.sign[0]].x() - ray.origin.x()) * ray.inv_direction.x();

        let tymin = (self.bounds[ray.sign[1]].y() - ray.origin.y()) * ray.inv_direction.y();
        let tymax = (self.bounds[1 - ray.sign[1]].y() - ray.origin.y()) * ray.inv_direction.y();
        if (t_min > tymax) || (tymin > t_max) {
            return None;
        }
        if tymin > t_min {
            t_min = tymin;
        }
        if tymax < t_max {
            t_max = tymax;
        }
        let tzmin = (self.bounds[ray.sign[2]].z() - ray.origin.z()) * ray.inv_direction.z();
        let tzmax = (self.bounds[1 - ray.sign[2]].z() - ray.origin.z()) * ray.inv_direction.z();
        if (t_min > tzmax) || (tzmin > t_max) {
            return None;
        }
        if tzmin > t_min {
            t_min = tzmin;
        }
        if tzmax < t_max {
            t_max = tzmax;
        }
        if (t_min < t1) && (t_max > t0) {
            return Some(t_min);
        }
        return None;
    }
}

