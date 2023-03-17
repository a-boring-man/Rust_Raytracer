/* responsible for calculating each pixel color and arranging them in a ppm format */
fn write_ppm(width: u16, height: u16, max_value: u8) {

    println!("P3\n{} {}\n{}", width, height, max_value);

    for j in (0..height).rev() {

        eprintln!("\rScanlines remaining: {} ", j);

        for i in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g: f64 = j as f64 / (height - 1) as f64;
            let b: f64 = 0.25;

            let ir: u8 = (255.999 * r) as u8;
            let ig: u8 = (255.999 * g) as u8;
            let ib: u8 = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {

    /* Image Constrains */

    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;
    const MAX_COLOR_VALUE: u8 = 255;

    /* Render */

    write_ppm(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR_VALUE);

    eprintln!("\nDone.\n");
}
