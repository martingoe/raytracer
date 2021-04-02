use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::{FromStr, SplitWhitespace};

use crate::noises::perlin_noise::PerlinNoise;
use crate::vec3::{Color, Point3};
use std::borrow::BorrowMut;

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
    Mapped {
        colors: Vec<Vec<Color>>
    },
}
fn parse_next_f64(iteration: &mut SplitWhitespace) -> f64 {
    return iteration.next().unwrap().parse::<f64>().unwrap();
}
impl Texture {
    pub fn parse_mapped(path: String) -> Texture {
        let file = File::open(path).unwrap();
        let mut result = Vec::new();
        let mut reader = BufReader::new(file);
        let (width, max_value) = Texture::read_base_information(&mut reader);

        let mut i = 0;
        let mut j = 0;
        for (_index, line) in reader.lines().enumerate() {
            if result.len() <= i{
                result.push(Vec::new());
            }
            let line = line.unwrap();
            let mut words = line.split_whitespace();
            result[i].push(Color { e: [parse_next_f64(words.borrow_mut()) / max_value, parse_next_f64(words.borrow_mut()) / max_value, parse_next_f64(words.borrow_mut()) / max_value] });

            j += 1;
            if j == width{
                j = 0;
                i += 1;
            }
        }

        return Texture::Mapped { colors: result };
    }

    fn read_base_information(reader: &mut BufReader<File>) -> (i32, f64) {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        let mut temp = Vec::new();
        reader.read_until(b' ', &mut temp).unwrap();
        let x = std::str::from_utf8(&*temp).unwrap().replace(" ", "");
        let width = i32::from_str(&*x).unwrap();
        temp = Vec::new();
        reader.read_until('\n' as u8, &mut temp).unwrap();

        temp = Vec::new();
        reader.read_until('\n' as u8, &mut temp).unwrap();
        let x = std::str::from_utf8(&*temp).unwrap().replace("\n", "");
        let max_value = f64::from_str(&*x).unwrap();
        reader.read_line(&mut line);
        (width, max_value)
    }


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
            Texture::Mapped { colors } => {
                return colors[(v * colors.len() as f64) as usize][(u * colors[0].len() as f64) as usize];
            }
        }
    }
}
