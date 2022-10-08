use std::io;

use rtweekend::{Color, Hittable, HittableList, Point, Ray, Sphere, Vec3, INFINITY, Camera, random_double};

pub fn ray_color(r: &Ray, world: &impl Hittable, depth: u64) -> Color {

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, INFINITY) {
        Some(rec) => {
            let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
            0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1)
        }
        None => {
            let unit_direction: Vec3 = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let cam = Camera::new();

    // Render
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double(0.0, 1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double(0.0, 1.0)) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            let stdout = io::stdout();
            let mut handle = stdout.lock();

            pixel_color.write_color(&mut handle, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone");
}
