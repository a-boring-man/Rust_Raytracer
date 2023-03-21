/* List of includes */
extern crate sdl2;
mod vec3;
mod ray;
mod hittable;
mod hittablelist;
mod sphere;
mod color_rendering;
mod utils;
mod camera;

/* Shortcut */
use vec3::Vec3;
use ray::Ray;
use crate::color_rendering::ray_color;
use crate::hittablelist::*;
use crate::sphere::Sphere;
use crate::utils::degree_to_rad;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

/* Aliasing */
type Point3 = Vec3;

/* Modulable Const */
const RAY_T_MIN: f64 = 0.0001;
const RAY_T_MAX: f64 = 1.0e30;

/* entry point of the program */
fn main() {
    
    /* Image Constrains */
    
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u16 = 400;
    const IMAGE_WIDTH: u16 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u16;
    const INV_IMAGE_HEIGHT_1: f64 = 1.0 / (IMAGE_HEIGHT - 1) as f64;
    const INV_IMAGE_WIDTH_1: f64 = 1.0 / (IMAGE_WIDTH - 1) as f64;

    /* Camera Constrains */

    const FOV: f64 = 95.0;
    const VIEWPORT_HEIGHT: f64 = 1.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;

    /* Camera Initialization */
    let screen_distance: f64 = (VIEWPORT_WIDTH * 0.5) / (degree_to_rad(FOV * 0.5).tan());
    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(-VIEWPORT_WIDTH * 0.5, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT * 0.5, 0.0);
    let depth: Vec3 = Vec3::new(0.0, 0.0, screen_distance);

    /* Building the world */
    let mut world: Hittablelist = Hittablelist::new_add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(-2.0, 0.0, 3.0), 1.0)));
    
    /* Window thing */

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("super ray tracer", IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    /* Event Listening */
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

        /* Main Loop */
        for j in 0..IMAGE_HEIGHT {
            //eprintln!("\rlines done: {} ", j);
            let v: f64 = (0.5 - j as f64 * INV_IMAGE_HEIGHT_1) * 2.0;
            let vertical_factor = &vertical * v;
            for i in 0..IMAGE_WIDTH {
                let u: f64 = (i as f64 * INV_IMAGE_WIDTH_1 - 0.5) * 2.0;
                let r: Ray = Ray::new(origin.clone(), &horizontal * u + &vertical_factor + &depth - &origin);

                let pixel_color = ray_color(&r, &world);
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
