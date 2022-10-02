use std::io::Write;
use crate::Color;

pub fn write_color(out: &mut impl Write, color: Color) {
    let ir = (255.999 * color.x()) as i64;
    let ig = (255.999 * color.y()) as i64;
    let ib = (255.999 * color.z()) as i64;

    out.write(format!("{} {} {}\n", ir, ig, ib).as_bytes());
}