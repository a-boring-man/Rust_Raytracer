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
mod material;
mod lamberian;
mod metal;

/* Shortcut */
use vec3::Vec3;
use ray::Ray;
use crate::color_rendering::*;
use crate::hittablelist::*;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::utils::{random_number, clamp};
use crate::lamberian::*;
use crate::metal::*;
use std::rc::Rc;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

/* Aliasing */
type Color3 = Vec3;

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

    /* Camera Initialization */

    let camera = Camera::new(95.0);
    let sample_size = 100;
    let max_depth = 50;

    /* Building the world */
    let mut world: Hittablelist = Hittablelist::new();

    let material1 = Rc::new(Lamberian::new(&Color3::new(0.8, 0.8, 0.0)));
    let material2 = Rc::new(Lamberian::new(&Color3::new(0.7, 0.3, 0.3)));
    let material3 = Rc::new(Metal::new(&Color3::new(0.8, 0.8, 0.8), 0.3));
    let material4 = Rc::new(Metal::new(&Color3::new(0.8, 0.6, 0.2), 0.7));


    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, 1.0), 100.0, material1)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5, material2)));
    world.add(Rc::new(Sphere::new(Vec3::new(1.0, 0.0, 1.0), 0.5, material3)));
    world.add(Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, 1.0), 0.5, material4)));
    
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
            eprintln!("\rlines to be done: {} ", IMAGE_HEIGHT - j);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color: Color3 = Color3::new(0.0, 0.0, 0.0);
                for _ in 0..=sample_size {
                    let u: f64 = ((i as f64 + random_number()) * INV_IMAGE_WIDTH_1 - 0.5 ) * 2.0;
                    let v: f64 = (0.5 - (j as f64 + random_number()) * INV_IMAGE_HEIGHT_1 ) * 2.0;
                    let r: Ray = camera.get_ray(u, v);
    
                    pixel_color += ray_color(&r, &world, max_depth);
                }
                sample_scalling(&mut pixel_color, sample_size);
                pixel_color = gamma_scaling(&pixel_color);
                
                let points = [Point::new(i as i32, j as i32); 1];
                canvas.set_draw_color(Color::RGB(   (256.0 * clamp(pixel_color.r(), 0.0, 0.999)) as u8,
                                                    (256.0 * clamp(pixel_color.g(), 0.0, 0.999)) as u8,
                                                    (256.0 * clamp(pixel_color.b(), 0.0, 0.999)) as u8));
                canvas.draw_points(points.as_slice()).unwrap();
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        eprintln!("one loop");
    }

    eprintln!("\nDone.\n");
}
