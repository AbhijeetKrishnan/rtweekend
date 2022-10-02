use std::io;

use ray_tracing_in_a_weekend::Color;
use ray_tracing_in_a_weekend::color;

const IMAGE_WIDTH: u64 = 256;
const IMAGE_HEIGHT: u64 = 256;

fn main() {
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color = Color::new(
                (i as f64) / (IMAGE_WIDTH - 1) as f64,
                (j as f64) / (IMAGE_HEIGHT - 1) as f64,
                0.25
            );

            let stdout = io::stdout();
            let mut handle = stdout.lock();

            color::write_color(&mut handle, pixel_color);
        }
    }
    eprintln!("\nDone");
}
