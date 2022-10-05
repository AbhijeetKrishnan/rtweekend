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