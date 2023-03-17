/* List of includes */
mod vec3;
mod color_rendering;
mod ray;

/* Shortcut */
use vec3::Vec3;

/* Aliasing */
type Point = Vec3;

/* entry point of the program */
fn main() {

    /* Image Constrains */

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u16 = 200;
    const IMAGE_WIDTH: u16 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u16;
    const MAX_COLOR_VALUE: u8 = 255;

    /* Camera cConstrains */

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin: Point = Point::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH / 2.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT / 2.0, 0.0);
    eprintln!("origin = {:#?}\n, horizontal = {:#?}\n, vertical = {:#?}", origin, horizontal, vertical);
    let lower_left_corner = origin - horizontal - vertical - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    /* Render */

    color_rendering::write_ppm(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR_VALUE, origin, lower_left_corner, horizontal, vertical);

    eprintln!("\nDone.\n");
}
