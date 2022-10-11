#[macro_use]
extern crate impl_ops;

use std::io;
use rand;

mod vec3;
pub use vec3::Vec3;
pub use vec3::Vec3 as Color;
pub use vec3::Vec3 as Point;

mod ray;
pub use ray::Ray;

mod color;

mod hittable;
pub use hittable::{HitRecord, Hittable, HittableList, Sphere, HittableObj, MaterialPtr};

pub struct Degrees(pub f64);
pub struct Radians(pub f64);
mod camera;
pub use camera::Camera;

mod material;
pub use material::{Dielectric, Lambertian, Material, Metal};

pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: Degrees) -> Radians {
    Radians(degrees.0 * PI / 180.0)
}

pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn draw_buffer_to_ppm(buffer: Vec<Vec<Color>>, samples_per_pixel: u64) {
    for j in (0..buffer.len()).rev() {
        for i in 0..buffer[0].len() {
            let pixel_color = buffer[j][i];
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            pixel_color.write_color(&mut handle, samples_per_pixel);
        }
    }
}