use std::{io, rc::Rc};

use rtweekend as rt;

pub fn ray_color(r: &rt::Ray, world: &impl rt::Hittable, depth: u64) -> rt::Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return rt::Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, rt::INFINITY) {
        Some(rec) => match rec.mat_ptr.scatter(r, &rec) {
            Some((attenuation, scattered)) => attenuation * ray_color(&scattered, world, depth - 1),
            None => rt::Color::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction: rt::Vec3 = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * rt::Color::new(1.0, 1.0, 1.0) + t * rt::Color::new(0.5, 0.7, 1.0)
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
    let mut world = rt::HittableList::new();

    let material_ground = Rc::new(rt::Lambertian::new(rt::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(rt::Lambertian::new(rt::Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(rt::Dielectric::new(1.5));
    let material_right = Rc::new(rt::Metal::new(rt::Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(rt::Sphere::new(
        rt::Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(rt::Sphere::new(
        rt::Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(rt::Sphere::new(
        rt::Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(rt::Sphere::new(
        rt::Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let cam = rt::Camera::new();

    // Render
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = rt::Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rt::random_double(0.0, 1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rt::random_double(0.0, 1.0)) / (IMAGE_HEIGHT - 1) as f64;
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
