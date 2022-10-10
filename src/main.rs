use std::{io, rc::Rc};

use rtweekend as rt;
use rt::Hittable;

pub fn ray_color(r: &rt::Ray, world: &rt::HittableList, depth: u64) -> rt::Color {
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

fn random_scene() -> rt::HittableList {
    let mut world = rt::HittableList::new();
    let ground_material = Rc::new(rt::Lambertian::new(rt::Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(rt::Sphere::new(
        rt::Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&ground_material) as Rc<dyn rt::Material>,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rt::random_double(0.0, 1.0);
            let center = rt::Point::new(a as f64 + 0.9 * rt::random_double(0.0, 1.0), 0.2, b as f64 + 0.9 * rt::random_double(0.0, 1.0));

            if (center - rt::Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn rt::Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rt::Color::random(0.0, 1.0) * rt::Color::random(0.0, 1.0);
                    sphere_material = Rc::new(rt::Lambertian::new(albedo));
                    world.add(Box::new(rt::Sphere::new(
                        center,
                        0.2,
                        Rc::clone(&sphere_material) as Rc<dyn rt::Material>,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = rt::Color::random(0.5, 1.0);
                    let fuzz = rt::random_double(0.0, 0.5);
                    sphere_material = Rc::new(rt::Metal::new(albedo, fuzz));
                    world.add(Box::new(rt::Sphere::new(
                        center,
                        0.2,
                        Rc::clone(&sphere_material) as Rc<dyn rt::Material>,
                    )));
                } else {
                    // glass
                    sphere_material = Rc::new(rt::Dielectric::new(1.5));
                    world.add(Box::new(rt::Sphere::new(
                        center,
                        0.2,
                        Rc::clone(&sphere_material) as Rc<dyn rt::Material>,
                    )));
                }
            }
        }
    }

    let material1 = Rc::new(rt::Dielectric::new(1.5));
    world.add(Box::new(rt::Sphere::new(
        rt::Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::clone(&material1) as Rc<dyn rt::Material>,
    )));

    let material2 = Rc::new(rt::Lambertian::new(rt::Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(rt::Sphere::new(
        rt::Point::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&material2) as Rc<dyn rt::Material>,
    )));

    let material3 = Rc::new(rt::Metal::new(rt::Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(rt::Sphere::new(
        rt::Point::new(4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&material3) as Rc<dyn rt::Material>,
    )));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 1200;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 500;
    const MAX_DEPTH: u64 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = rt::Point::new(13.0, 2.0, 3.0);
    let lookat = rt::Point::new(0.0, 0.0, 0.0);
    let vup = rt::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = rt::Camera::new(
        lookfrom,
        lookat,
        vup,
        rt::Degrees(20.0),
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
