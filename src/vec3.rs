use std::default::Default;
use std::{fmt, ops};

use crate::random_double;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(self: &Vec3) -> f64 {
        self.x
    }

    pub fn y(self: &Vec3) -> f64 {
        self.y
    }

    pub fn z(self: &Vec3) -> f64 {
        self.z
    }

    pub fn length_squared(self: &Vec3) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self: &Vec3) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self: &Vec3, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self: &Vec3, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        } / self.length()
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double(min, max),
            y: random_double(min, max),
            z: random_double(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        (self.x().abs() < EPS) && (self.y().abs() < EPS) && (self.z().abs() < EPS)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&-uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                crate::random_double(-1.0, 1.0),
                crate::random_double(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl_op_ex!(-|v: &Vec3| -> Vec3 { Vec3::new(-v.x, -v.y, -v.z) });

impl ops::Index<u8> for Vec3 {
    type Output = f64;
    fn index(&self, index: u8) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}

impl ops::IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index"),
        }
    }
}

impl_op_ex!(+= |v: &mut Vec3, rhs: Vec3| { v.x += rhs.x; v.y += rhs.y; v.z += rhs.z });
impl_op_ex!(-= |v: &mut Vec3, rhs: Vec3| { v.x -= rhs.x; v.y -= rhs.y; v.z -= rhs.z });
impl_op_ex!(*= |v: &mut Vec3, rhs: f64| { v.x *= rhs; v.y *= rhs; v.z *= rhs });
impl_op_ex!(/= |v: &mut Vec3, rhs: f64| { v.x /= rhs; v.y /= rhs; v.z /= rhs });

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z) });
impl_op_ex!(-|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)
});
impl_op_ex!(*|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z)
});
impl_op_ex!(*|lhs: &Vec3, rhs: f64| -> Vec3 { Vec3::new(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs) });
impl_op_ex!(*|lhs: f64, rhs: &Vec3| -> Vec3 { Vec3::new(lhs * rhs.x, lhs * rhs.y, lhs * rhs.z) });
impl_op_ex!(/ |lhs: &Vec3, rhs: f64| -> Vec3 { Vec3::new(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs) });
