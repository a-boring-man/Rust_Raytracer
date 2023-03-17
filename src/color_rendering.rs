/* Allow rust to know where to find the Vec3 class */
use crate::vec3::Vec3;

/* Shortcut */
type Color = Vec3;

/* Write the translated [0, 255] value of each color component */
pub fn write_color(pixel_color: Color) {
    println!("{} {} {}",    255.999 * pixel_color.x(),
                            255.999 * pixel_color.x(),
                            255.999 * pixel_color.x());
}

/* Responsible for calculating each pixel color and arranging them in a ppm format */
pub fn write_ppm(width: u16, height: u16, max_value: u8) {
    println!("P3\n{} {}\n{}", width, height, max_value);
    for j in (0..height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..width {
            let pixel_color: Color = Color::new_v(  i as f64 / (width - 1) as f64,
                                                    j as f64 / (height - 1) as f64,
                                                    0.25);
            write_color(pixel_color);
        }
    }
}