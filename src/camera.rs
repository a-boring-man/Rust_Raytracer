use crate::Vec3;
use crate::utils::degree_to_rad;
use crate::ray::*;

type Point3 = Vec3;

pub struct Camera {
    origin: Point3,
    depth: Vec3,
    horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn new(fov: f64) -> Camera {

        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f64 = 1.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    
        /* Camera Initialization */
        let screen_distance: f64 = (VIEWPORT_WIDTH * 0.5) / (degree_to_rad(fov * 0.5).tan());
        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(-VIEWPORT_WIDTH * 0.5, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT * 0.5, 0.0);
        let depth: Vec3 = Vec3::new(0.0, 0.0, screen_distance);

        Camera { origin, depth, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin.clone(), &self.horizontal * u + v * &self.vertical + &self.depth - &self.origin)
    }
}