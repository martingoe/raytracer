use std::sync::Arc;

use crate::color_at;
use crate::hittables::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::utils::math_utils::random_double;
use crate::vec3::{Color, dot, random_in_hemisphere, random_in_unit_sphere, reflect, refract, Vec3};

pub(crate) trait MaterialTrait: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, depth: i32, world: Arc<Hittable>) -> Option<Color>;
}

pub enum Material {
    Dielectric { ir: f64, tint: Color, emission: Color },
    Metal { albedo: Color, fuzz: f64, emission: Color },
    Diffuse { albedo: Color, emission: Color },
}

impl MaterialTrait for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, depth: i32, world: Arc<Hittable>) -> Option<Color> {
        match self {
            Material::Dielectric { ir, tint, emission } => {
                let refraction_ratio = if rec.front_face { 1.0 / ir } else { *ir };

                let unit_direction = r_in.direction.unit_vector();
                let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction: Vec3;
                if cannot_refract || schlicks(cos_theta, refraction_ratio) > random_double(0.0, 1.0) {
                    direction = reflect(&unit_direction, &rec.normal);
                } else {
                    direction = refract(&unit_direction, &rec.normal, refraction_ratio);
                }
                return Some(*emission + *tint * color_at(&Ray::new(rec.point, direction), world.clone(), depth - 1));
            }

            Material::Metal { albedo, fuzz, emission } => {
                let reflected = reflect(&r_in.direction.unit_vector(), &rec.normal);

                let scattered = Ray::new(rec.point, reflected + random_in_unit_sphere() * *fuzz);
                if dot(&scattered.direction, &rec.normal) > 0.0 {
                    Some(*emission + *albedo * color_at(&scattered, world.clone(), depth - 1))
                } else {
                    None
                }
            }

            Material::Diffuse { albedo, emission } => {
                let scatter_dir = random_in_hemisphere(&rec.normal);
                return Some(*emission + (*albedo * color_at(&Ray::new(rec.point, scatter_dir), world.clone(), depth - 1) * dot(&scatter_dir.unit_vector(), &rec.normal.unit_vector()) * 2.0));
            }
        }
    }
}

fn schlicks(cos_theta: f64, ref_index: f64) -> f64 {
    // Schlick's approximation
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0);
}