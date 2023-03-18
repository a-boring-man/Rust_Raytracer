/* Allow rust to know where to find the Vec3 class */
use crate::vec3::Vec3;
use crate::ray::Ray;

/* Aliasing type make code intention more readable */
type Color = Vec3;

pub fn ray_color(r: &Ray) -> Color {
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
