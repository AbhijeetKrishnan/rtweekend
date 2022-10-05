use crate::{Ray, Vec3, Point};

pub struct HitRecord {
    p: Point,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = Vec3::dot(r.direction(), &outward_normal) < 0.0;
        let normal = 
            if front_face {
                outward_normal
            } else {
                -outward_normal
            };
        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(cen: Point, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let (front_face, normal) = HitRecord::set_face_normal(r, outward_normal);
        
        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
        })
    }
}