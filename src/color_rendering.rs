use crate::utils::{random_in_hemisphere};
/* Allow rust to know where to find the Vec3 class and everything else needed*/
use crate::{RAY_T_MIN, RAY_T_MAX};
use crate::hittable::*;
use crate::hittablelist::Hittablelist;
use crate::vec3::Vec3;
use crate::ray::Ray;

/* Aliasing type make code intention more readable */
type Color = Vec3;

pub fn background_color(r: &Ray) -> Color {
    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

    if r.dir().x().abs() >= r.dir().y().abs() && r.dir().x().abs() >= r.dir().z().abs() {
        pixel_color.e[0] = (r.dir().x() + 1.0) / 2.0;
    }
    if r.dir().y().abs() >= r.dir().x().abs() && r.dir().y().abs() >= r.dir().z().abs() {
        pixel_color.e[1] = (r.dir().y() + 1.0) / 2.0;
    }
    if r.dir().z().abs() >= r.dir().x().abs() && r.dir().z().abs() >= r.dir().y().abs() {
        pixel_color.e[2] = (-1.0 * r.dir().z() + 1.0) / 2.0;
    }
    pixel_color
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
        let target: Vec3 = &hit_record.p + &hit_record.normal + random_in_hemisphere(&hit_record.normal);
        return 0.5 * ray_color(&Ray::new(hit_record.p.clone(), &target - &hit_record.p), world, depth - 1);
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