use std::f64::consts::PI;
use std::sync::Arc;

use crate::color_at;
use crate::hittables::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::textures::texture::Texture;
use crate::utils::math_utils::random_double;
use crate::vec3::{Color, dot, random_in_hemisphere, random_in_unit_sphere, reflect, refract};

pub(crate) trait MaterialTrait: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        depth: i32,
        world: Arc<Hittable>,
    ) -> Option<Color>;
}

pub enum Material {
    Dielectric {
        ir: f64,
        tint: Texture,
        emission: Color,
    },
    Metal {
        albedo: Texture,
        fuzz: f64,
        emission: Color,
    },
    Diffuse {
        albedo: Texture,
        emission: Color,
    },
    CookTorrance {
        diffuse: Texture,

        k_d: f64,
        specular: Texture,
        roughness: f64,
        emission: Color,
    },
}

impl MaterialTrait for Material {
    fn scatter(
        &self,
        w_o: &Ray,
        rec: &HitRecord,
        depth: i32,
        world: Arc<Hittable>,
    ) -> Option<Color> {
        match self {
            Material::Dielectric { ir, tint, emission } => {
                let refraction_ratio = if rec.front_face { 1.0 / *ir } else { *ir };

                let unit_direction = w_o.direction.unit_vector();
                let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || schlicks(cos_theta, refraction_ratio) > random_double(0.0, 1.0)
                {
                    reflect(&unit_direction, &rec.normal)
                } else {
                    refract(&unit_direction, &rec.normal, refraction_ratio)
                };
                return Some(
                    *emission
                        + tint.value_at(rec.u, rec.v, rec.point)
                        * color_at(&Ray::new(rec.point, direction), world.clone(), depth - 1),
                );
            }

            Material::Metal {
                albedo,
                fuzz,
                emission,
            } => {
                let reflected = reflect(&w_o.direction.unit_vector(), &rec.normal);

                let scattered = Ray::new(rec.point, reflected + random_in_unit_sphere() * *fuzz);
                if dot(&scattered.direction, &rec.normal) > 0.0 {
                    Some(
                        *emission
                            + albedo.value_at(rec.u, rec.v, rec.point)
                            * color_at(&scattered, world.clone(), depth - 1),
                    )
                } else {
                    None
                }
            }

            Material::Diffuse { albedo, emission } => {
                let scatter_dir = random_in_hemisphere(&rec.normal);
                return Some(
                    *emission
                        + (albedo.value_at(rec.u, rec.v, rec.point)
                        * color_at(
                        &Ray::new(rec.point, scatter_dir),
                        world.clone(),
                        depth - 1,
                    )
                        * dot(&scatter_dir.unit_vector(), &rec.normal.unit_vector())
                        * 2.0),
                );
            }
            Material::CookTorrance { diffuse, k_d, specular: specular_color, roughness, emission } => {
                let w_i = random_in_hemisphere(&rec.normal);
                let w_o = w_o.direction * -1.0;
                let color_at_wi = color_at(&Ray::new(rec.point, w_i), world.clone(),
                                           depth - 1);

                let h = (w_o + w_i).unit_vector();
                let wi_dot_h = dot(&w_i, &h);
                let wo_dot_h = dot(&w_o, &h);
                let n_dot_h = dot(&rec.normal, &h);
                let g = ((2.0 * n_dot_h * dot(&rec.normal, &w_o)) / wo_dot_h).min(((2.0 * n_dot_h * dot(&rec.normal, &w_i)) / wo_dot_h)).min(1.0);
                let f = schlicks_color(&specular_color.value_at(rec.u, rec.v, rec.point), wi_dot_h);
                let m_sqr = roughness * roughness;
                let n_dot_h_2 = n_dot_h * n_dot_h;
                let d = 1.0 / (PI * m_sqr * n_dot_h_2 * n_dot_h_2) * ((n_dot_h_2 - 1.0) / (m_sqr * n_dot_h_2)).exp();

                return Option::from(*emission + diffuse.value_at(rec.u, rec.v, rec.point) / PI * *k_d +  color_at_wi * d * f * g * dot(&rec.normal, &w_o) * PI / 2.0 * (1.0 - k_d));
            }
        }
    }
}

fn schlicks_color(color: &Color, wi_dot_h: f64) -> Color {
    *color + (*color * -1.0 + 1.0) * (1.0 - wi_dot_h).powi(5)
}

fn schlicks(cos_theta: f64, ref_index: f64) -> f64 {
    // Schlick's approximation
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0);
}
