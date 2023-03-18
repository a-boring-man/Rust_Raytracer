/* List of includes */
extern crate sdl2;
mod vec3;
mod color_rendering;
mod ray;

/* Shortcut */
use vec3::Vec3;
use ray::Ray;
use crate::color_rendering::ray_color;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

/* Aliasing */
type Point3 = Vec3;

/* entry point of the program */
fn main() {
    
    /* Image Constrains */
    
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u16 = 400;
    const IMAGE_WIDTH: u16 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u16;
    const MAX_COLOR_VALUE: u8 = 255;

    /* Camera cConstrains */

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    
    /* Window thing */

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("super ray tracer", IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for j in 0..IMAGE_HEIGHT {
            eprintln!("\rlines done: {} ", j);
            for i in 0..IMAGE_WIDTH {
                let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
                let v: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
                let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
                let pixel_color = ray_color(&r);

                let points = [Point::new(i as i32, j as i32); 1];
                canvas.set_draw_color(Color::RGB((255.999 * pixel_color.r()) as u8, (255.999 * pixel_color.g()) as u8, (255.999 * pixel_color.b()) as u8));
                canvas.draw_points(points.as_slice()).unwrap();
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    eprintln!("\nDone.\n");
}
