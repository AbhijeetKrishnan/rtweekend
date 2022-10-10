use crate::{clamp, Color};
use std::io::Write;

impl Color {
    pub fn write_color(&self, out: &mut impl Write, samples_per_pixel: u64) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // Divide the color by the number of samples
        let scale = 1.0 / (samples_per_pixel as f64);
        let scaled_r = (r * scale).sqrt();
        let scaled_g = (g * scale).sqrt();
        let scaled_b = (b * scale).sqrt();

        let ir = (256.0 * clamp(scaled_r, 0.0, 0.999)) as u8;
        let ig = (256.0 * clamp(scaled_g, 0.0, 0.999)) as u8;
        let ib = (256.0 * clamp(scaled_b, 0.0, 0.999)) as u8;

        match out.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()) {
            Ok(_) => (),
            Err(_) => panic!("Error while writing color to ppm file"),
        }
    }
}
