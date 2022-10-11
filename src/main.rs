use std::{
    sync::{Arc, Mutex},
    thread,
};

use rt::Hittable;
use rtweekend as rt;

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
    let ground_material = Arc::new(rt::Lambertian::new(rt::Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(rt::Sphere::new(
        rt::Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&ground_material) as rt::MaterialPtr,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rt::random_double(0.0, 1.0);
            let center = rt::Point::new(
                a as f64 + 0.9 * rt::random_double(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * rt::random_double(0.0, 1.0),
            );

            if (center - rt::Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: rt::MaterialPtr;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rt::Color::random(0.0, 1.0) * rt::Color::random(0.0, 1.0);
                    sphere_material = Arc::new(rt::Lambertian::new(albedo));
                    world.add(Box::new(rt::Sphere::new(
                        center,
                        0.2,
                        Arc::clone(&sphere_material) as rt::MaterialPtr,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = rt::Color::random(0.5, 1.0);
                    let fuzz = rt::random_double(0.0, 0.5);
                    sphere_material = Arc::new(rt::Metal::new(albedo, fuzz));
                    world.add(Box::new(rt::Sphere::new(
                        center,
                        0.2,
                        Arc::clone(&sphere_material) as rt::MaterialPtr,
                    )));
                } else {
                    // glass
                    sphere_material = Arc::new(rt::Dielectric::new(1.5));
                    world.add(Box::new(rt::Sphere::new(
                        center,
                        0.2,
                        Arc::clone(&sphere_material) as rt::MaterialPtr,
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(rt::Dielectric::new(1.5));
    world.add(Box::new(rt::Sphere::new(
        rt::Point::new(0.0, 1.0, 0.0),
        1.0,
        Arc::clone(&material1) as rt::MaterialPtr,
    )));

    let material2 = Arc::new(rt::Lambertian::new(rt::Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(rt::Sphere::new(
        rt::Point::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&material2) as rt::MaterialPtr,
    )));

    let material3 = Arc::new(rt::Metal::new(rt::Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(rt::Sphere::new(
        rt::Point::new(4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&material3) as rt::MaterialPtr,
    )));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 200; // orig = 1200
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: u64 = 50; // orig = 500
    const MAX_DEPTH: u64 = 5; // orig = 50

    // Buffer
    let pixel_buffer =
        Arc::new(Mutex::new(vec![
            vec![rt::Color::new(0.0, 0.0, 0.0); IMAGE_WIDTH];
            IMAGE_HEIGHT
        ]));

    // World
    let world = Arc::new(random_scene());

    // Camera
    let lookfrom = rt::Point::new(13.0, 2.0, 3.0);
    let lookat = rt::Point::new(0.0, 0.0, 0.0);
    let vup = rt::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Arc::new(rt::Camera::new(
        lookfrom,
        lookat,
        vup,
        rt::Degrees(20.0),
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    ));

    // Render
    let mut handles = vec![];
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..IMAGE_WIDTH {
            // start a thread
            let pixel_buffer_t = Arc::clone(&pixel_buffer);
            let camera_t = Arc::clone(&cam);
            let world_t = Arc::clone(&world);
            let handle = thread::spawn(move || {
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + rt::random_double(0.0, 1.0)) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + rt::random_double(0.0, 1.0)) / (IMAGE_HEIGHT - 1) as f64;
                    let r = camera_t.get_ray(u, v);
                    let ray_color = ray_color(&r, &world_t, MAX_DEPTH);
                    // acquire lock on curr pixel colour and update it
                    let mut pixel_buffer = pixel_buffer_t.lock().unwrap();
                    pixel_buffer[j][i] += ray_color;
                }
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    rt::draw_buffer_to_ppm(pixel_buffer.lock().unwrap().to_vec(), SAMPLES_PER_PIXEL);
    eprintln!("\nDone");
}
