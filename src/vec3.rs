use crate::utils::math_utils::random_double;
use std::{fmt, ops};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub(crate) e: [f64; 3],
}

impl Vec3 {
    pub(crate) fn new() -> Vec3 {
        return Vec3 { e: [0.0, 0.0, 0.0] };
    }
    pub(crate) fn unit_vector(self) -> Vec3 {
        return self / self.length();
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        return self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s;
    }
    pub fn x(self) -> f64 {
        return self.e[0];
    }
    pub fn y(self) -> f64 {
        return self.e[1];
    }
    pub fn z(self) -> f64 {
        return self.e[2];
    }
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    return Vec3 {
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
    };
}

pub fn create_vec_3(x: f64, y: f64, z: f64) -> Vec3 {
    return Vec3 { e: [x, y, z] };
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    return u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2];
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        };
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        };
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        return Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        };
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        };
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        return Vec3 {
            e: [self.e[0] + rhs, self.e[1] + rhs, self.e[2] + rhs],
        };
    }
}
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        return Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        };
    }
}

fn random(min: f64, max: f64) -> Vec3 {
    return Vec3 {
        e: [
            random_double(min, max),
            random_double(min, max),
            random_double(min, max),
        ],
    };
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p;
    loop {
        p = random(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        break;
    }
    return p;
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    return if dot(&in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    };
}
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-*uv, n).min(1.0);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    return r_out_perp + r_out_parallel;
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - *n * dot(v, n) * 2.0;
}
pub type Color = Vec3;
pub type Point3 = Vec3;
