#[macro_use] extern crate impl_ops;

pub mod vec3;
pub use vec3::Vec3 as Vec3;
pub use vec3::Vec3 as Color;
pub use vec3::Vec3 as Point;

pub mod color;

pub mod ray;
pub use ray::Ray as Ray;