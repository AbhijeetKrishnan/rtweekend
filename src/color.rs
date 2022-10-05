use std::io::Write;
use crate::Color;

impl Color {

    pub fn write_color(&self, out: &mut impl Write) {
        let ir = (255.999 * self.x()) as i64;
        let ig = (255.999 * self.y()) as i64;
        let ib = (255.999 * self.z()) as i64;

        out.write(format!("{} {} {}\n", ir, ig, ib).as_bytes());
    }
}