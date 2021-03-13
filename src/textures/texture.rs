use crate::noises::perlin_noise::PerlinNoise;
use crate::vec3::{Color, Point3};

pub enum Texture {
    Solid {
        color: Color,
    },
    Checker {
        color1: Color,
        color2: Color,
        size: f64,
    },
    Perlin {
        perlin_noise: PerlinNoise,
        scale: f64,
        color1: Color,
        color2: Color,
    },
}

impl Texture {
    pub fn value_at(&self, u: f64, v: f64, p: Point3) -> Color {
        match self {
            Texture::Solid { color } => *color,
            Texture::Checker {
                color1,
                color2,
                size,
            } => {
                let sin = (size * p.x()).sin() * (size * p.y()).sin() * (size * p.z()).sin();
                return if sin < 0.0 { *color1 } else { *color2 };
            }
            Texture::Perlin {
                perlin_noise,
                scale,
                color1,
                color2,
            } => {
                let value = perlin_noise.get_value(p.x() * scale, p.y() * scale, p.z() * scale);
                return *color1 * value + *color2 * (1.0 - value);
            }
        }
    }
}
