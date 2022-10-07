use crate::{Point, Vec3};

pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> &Point {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}
