#[macro_use]
extern crate impl_ops;

use rand;

mod vec3;
pub use vec3::Vec3;
pub use vec3::Vec3 as Color;
pub use vec3::Vec3 as Point;

mod ray;
pub use ray::Ray;

mod color;

mod hittable;
pub use hittable::{HitRecord, Hittable, HittableList, Sphere};

mod camera;
pub use camera::Camera;

mod material;
pub use material::{Material, Lambertian, Metal};

pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}