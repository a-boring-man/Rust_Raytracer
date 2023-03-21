use crate::Vec3;
use crate::utils::degree_to_rad;

type Point3 = Vec3;

pub struct Camera {
    origin: Point3,
    fov: f64,
    depth: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(fov: f64) -> Camera {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const IMAGE_HEIGHT: u16 = 400;
        const IMAGE_WIDTH: u16 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u16;
        const INV_IMAGE_HEIGHT_1: f64 = 1.0 / (IMAGE_HEIGHT - 1) as f64;
        const INV_IMAGE_WIDTH_1: f64 = 1.0 / (IMAGE_WIDTH - 1) as f64;
        const VIEWPORT_HEIGHT: f64 = 1.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    
        let FOV: f64 = fov;
    
        /* Camera Initialization */
        let screen_distance: f64 = (VIEWPORT_WIDTH * 0.5) / (degree_to_rad(FOV * 0.5).tan());
        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(-VIEWPORT_WIDTH * 0.5, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT * 0.5, 0.0);
        let depth: Vec3 = Vec3::new(0.0, 0.0, screen_distance);
    }
}