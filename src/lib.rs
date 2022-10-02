use std::ops;
use std::default::Default;

#[derive(Default, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(self: Self) -> f64 {
        self.x
    }

    pub fn y(self: Self) -> f64 {
        self.y
    }

    pub fn z(self: Self) -> f64 {
        self.z
    }

    pub fn length_squared(self: Self) -> f64 {
        self.x * self.x + self.y + self.y + self.z + self.z
    }

    pub fn length(self: Self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 { 
            x: -self.x, 
            y: -self.y, 
            z: -self.z, 
        }
    }
}

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

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

use Vec3 as Color;
use Vec3 as Point;