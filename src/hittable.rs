use crate::{Ray, Vec3, Point};

pub struct HitRecord{
    p: Point,
    normal: Vec3,
    t: f64,
}

pub trait Hittable {
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool;
}