/* List of includes */
mod vec3;
mod color_rendering;

/* entry point of the program */
fn main() {

    /* Image Constrains */

    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;
    const MAX_COLOR_VALUE: u8 = 255;

    /* Render */

    color_rendering::write_ppm(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR_VALUE);

    eprintln!("\nDone.\n");
}
