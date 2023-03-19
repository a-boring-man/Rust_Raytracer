/* Allow rust to know where to find the Vec3 class */
use crate::vec3::Vec3;
use crate::ray::Ray;

/* Aliasing type make code intention more readable */
type Color = Vec3;
type Point3 = Vec3;

pub fn hit_sphere(sphere_center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin() - sphere_center;
    let a: f64 = r.dir().length2();
    let b: f64 = 2.0 * Vec3::dot(&oc, &r.dir());
    let c: f64 = oc.length2() - radius * radius;
    let discriminant: f64 = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    else {
        return (-b - discriminant.sqrt()) * 0.5 / a;
    }
}

pub fn background_color(r: &Ray) -> Color {
    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

    if r.dir().x().abs() >= r.dir().y().abs() && r.dir().x().abs() >= r.dir().z().abs() {
        pixel_color.e[0] = (r.dir().x() + 1.0) / 2.0;
    }
    if r.dir().y().abs() >= r.dir().x().abs() && r.dir().y().abs() >= r.dir().z().abs() {
        pixel_color.e[1] = (r.dir().y() + 1.0) / 2.0;
    }
    if r.dir().z().abs() >= r.dir().x().abs() && r.dir().z().abs() >= r.dir().y().abs() {
        pixel_color.e[2] = 1.0 - ((r.dir().z() + 1.0) / 2.0);
    }
    pixel_color
}

pub fn debug_color(r: &Vec3) -> Color {
    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

    if r.x().abs() >= r.y().abs() && r.x().abs() >= r.z().abs() {
        pixel_color.e[0] = (r.x() + 1.0) / 2.0;
    }
    if r.y().abs() >= r.x().abs() && r.y().abs() >= r.z().abs() {
        pixel_color.e[1] = (r.y() + 1.0) / 2.0;
    }
    if r.z().abs() >= r.x().abs() && r.z().abs() >= r.y().abs() {
        pixel_color.e[2] = 1.0 - ((r.z() + 1.0) / 2.0);
    }
    pixel_color
}

pub fn ray_color(r: &Ray) -> Color {

    let sphere_collision: f64 = hit_sphere(&Point3::new(0.0, 0.0, 1.0), 0.5, r);
    if sphere_collision > 0.0 {
        let n: Vec3 = (r.at(sphere_collision) - Vec3::new(0.0, 0.0, 1.0)).normalized();
        let pixel_color = debug_color(&n);
        return pixel_color;
    }
    else {
        let pixel_color = background_color(r);
        return pixel_color;
    }
}
