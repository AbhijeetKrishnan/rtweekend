use std::ops;
use std::fmt;
use std::default::Default;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(self: &Self) -> f64 {
        self.x
    }

    pub fn y(self: &Self) -> f64 {
        self.y
    }

    pub fn z(self: &Self) -> f64 {
        self.z
    }

    pub fn length_squared(self: &Self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self: &Self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self: &Self, rhs: &Self) -> f64 {
        self.x * rhs.x +
            self.y * rhs.y +
            self.z * rhs.z
    }

    pub fn cross(self: &Self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z
        } / self.length()
    }
}

impl_op_ex!(- |v: &Vec3| -> Vec3 { Vec3::new(-v.x, -v.y, -v.z) });

impl ops::Index<u8> for Vec3 {
    type Output = f64;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index")
        }
    }
}

impl ops::IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index")
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
impl_op_ex!(- |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z) });
impl_op_ex!(* |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z) });
impl_op_ex!(* |lhs: &Vec3, rhs: f64| -> Vec3 { Vec3::new(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs) });
impl_op_ex!(* |lhs: f64, rhs: &Vec3| -> Vec3 { Vec3::new(lhs * rhs.x, lhs * rhs.y, lhs * rhs.z) });
impl_op_ex!(/ |lhs: &Vec3, rhs: f64| -> Vec3 { Vec3::new(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs) });
