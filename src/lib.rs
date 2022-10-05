#[macro_use] extern crate impl_ops;

mod vec3;
pub use vec3::Vec3 as Vec3;
pub use vec3::Vec3 as Color;
pub use vec3::Vec3 as Point;

mod ray;
pub use ray::Ray as Ray;

mod color;

mod hittable;
pub use hittable::HitRecord as HitRecord;
pub use hittable::Hittable as Hittable;
pub use hittable::HittableList as HittableList;
pub use hittable::Sphere as Sphere;

pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}