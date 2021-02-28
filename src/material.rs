use crate::ray::Ray;
use crate::vec3::{Color, random_in_hemisphere, dot, reflect, random_in_unit_sphere, refract, Vec3};
use crate::hittables::hittable::HitRecord;
use crate::utils::random_double;

pub(crate) trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Diffuse {
    pub(crate) albedo: Color
}

impl Material for Diffuse {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_dir = random_in_hemisphere(&rec.normal);
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        *scattered = Ray { origin: rec.point, direction: scatter_dir };
        *attenuation = self.albedo;

        return true;
    }
}

pub struct Metal {
    pub(crate) albedo: Color,
    pub(crate) fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(&r_in.direction.unit_vector(), &rec.normal);
        *scattered = Ray { origin: rec.point, direction: reflected + random_in_unit_sphere() * self.fuzz };
        *attenuation = self.albedo;

        return dot(&scattered.direction, &rec.normal) > 0.0;
    }
}

pub struct Dielectric {
    pub(crate) ir: f64,
    pub(crate) tint: Color,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = self.tint;
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double(0.0, 1.0) {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = Ray { origin: rec.point, direction };
        return true;
    }
}

fn reflectance(cos_theta: f64, ref_index: f64) -> f64 {
    // Schlick's approximation
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0);
}