/* Allow rust to know where to find the Vec3 class and everything else needed*/
use crate::{RAY_T_MIN, RAY_T_MAX};
use crate::hittable::*;
use crate::hittablelist::Hittablelist;
use crate::vec3::Vec3;
use crate::ray::Ray;

/* Aliasing type make code intention more readable */
type Color = Vec3;

pub fn background_color(r: &Ray) -> Color {
    let unit_direction: Color = r.dir().normalized();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    // let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

    // if r.dir().x().abs() >= r.dir().y().abs() && r.dir().x().abs() >= r.dir().z().abs() {
    //     pixel_color.e[0] = (r.dir().x() + 1.0) / 2.0;
    // }
    // if r.dir().y().abs() >= r.dir().x().abs() && r.dir().y().abs() >= r.dir().z().abs() {
    //     pixel_color.e[1] = (r.dir().y() + 1.0) / 2.0;
    // }
    // if r.dir().z().abs() >= r.dir().x().abs() && r.dir().z().abs() >= r.dir().y().abs() {
    //     pixel_color.e[2] = (-1.0 * r.dir().z() + 1.0) / 2.0;
    // }
    // pixel_color
}

#[allow(dead_code)]
pub fn debug_color(r: &Vec3) -> Color {
    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

    if r.x().abs() >= r.y().abs() && r.x().abs() >= r.z().abs() {
        pixel_color.e[0] = (r.x() + 1.0) / 2.0;
    }
    if r.y().abs() >= r.x().abs() && r.y().abs() >= r.z().abs() {
        pixel_color.e[1] = (r.y() + 1.0) / 2.0;
    }
    if r.z().abs() >= r.x().abs() && r.z().abs() >= r.y().abs() {
        pixel_color.e[2] = (r.z() + 1.0) / 2.0;
    }
    pixel_color
}

pub fn ray_color(r: &Ray, world: &Hittablelist, depth: i16) -> Color {
    let mut hit_record: HitRecord = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if world.hit(r, RAY_T_MIN, RAY_T_MAX, &mut hit_record) {

        let mut scattered_ray: Ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation: Color = Vec3::new(0.0, 0.0, 0.0);

        if let Some(mat) = hit_record.mat_ptr.as_ref() {
            if mat.scatter(r, &hit_record, &mut attenuation, &mut scattered_ray) {
                return attenuation * ray_color(&scattered_ray, world, depth - 1);
            }
        }
        return Color::new(0.0, 0.0, 0.0);
        // return debug_color(&hit_record.normal);
    }
    else {
        let pixel_color = background_color(r);
        return pixel_color;
    }
}

pub fn sample_scalling(pixel_color: &mut Color, sample_size: u8) {
    *pixel_color /= sample_size as f64;
}

pub fn gamma_scaling(pixel_color: &Color) -> Color {
    Color::new(pixel_color.r().sqrt(), pixel_color.g().sqrt(), pixel_color.b().sqrt())
}