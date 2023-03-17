/* Allow rust to know where to find the Vec3 class */
use crate::vec3::Vec3;
use crate::ray::Ray;

/* Aliasing type make code intention more readable */
type Color = Vec3;

pub fn ray_color(r: &Ray) -> Color {
    let unit_direction: Vec3 = Vec3::unit_vec(r.dir());
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    //eprint!("t = {}", t);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

/* Write the translated [0, 255] value of each color component */
pub fn write_color(pixel_color: Color) {
    println!("{} {} {}",    255.999 * pixel_color.x(),
                            255.999 * pixel_color.y(),
                            255.999 * pixel_color.z());
}

/* Responsible for calculating each pixel color and arranging them in a ppm format */
pub fn write_ppm(width: u16, height: u16, max_value: u8, origin: Vec3, lower_left_corner: Vec3, horizontal_axe: Vec3, vertical_axe: Vec3) {
    println!("P3\n{} {}\n{}", width, height, max_value);
    for j in (0..height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..width {
            let u: f64 = i as f64 / (width - 1) as f64;
            let v: f64 = j as f64 / (width - 1) as f64;
            let r: Ray = Ray::new(origin, lower_left_corner + horizontal_axe * u + vertical_axe * v - origin);
            let pixel_color = ray_color(&r);
            // if (i == 0) {
            //     eprintln!("test en 0,{} {:#?}",j , pixel_color); }
            write_color(pixel_color);
        }
    }
}