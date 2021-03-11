use crate::vec3::{Color, Point3};

#[derive(Copy, Clone)]
pub enum Texture {
    Solid { color: Color },
    Checker { color1: Color, color2: Color, size: f64 },
}

impl Texture {
    pub fn value_at(&self, u: f64, v: f64, p: Point3) -> Color {
        match self {
            Texture::Solid { color } => *color,
            Texture::Checker { color1, color2, size } => {
                let sin = (size * p.x()).sin() * (size * p.y()).sin() * (size * p.z()).sin();
                return if sin < 0.0 {
                    *color1
                } else {
                    *color2
                }
            }
        }
    }
}