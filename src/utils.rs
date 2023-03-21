use rand::prelude::*;
use std::f64::consts::PI;

use crate::vec3::Vec3;

pub fn random_number() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random(min: f64, max: f64) -> f64 {
    min + (max - min) * random_number()
}

pub fn sqr(n: f64) -> f64 {
    n * n
}

pub fn degree_to_rad (d: f64) -> f64 {
    d * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {return min};
    if x > max {return max};
    x
}

pub fn random_unit_sphere() -> Vec3 {
    let mut p: Vec3 = Vec3::vec_random_long(-1.0, 1.0);
    loop {
        if p.length2() < 1.0 {break}
        p = Vec3::vec_random_long(-1.0, 1.0);
    }
    return p;
}
#[allow(dead_code)]
pub fn random_unit_vector() -> Vec3 {
    random_unit_sphere().normalized()
}

pub fn random_in_hemisphere(n: &Vec3) -> Vec3 {
    let in_unit_sphere = random_unit_sphere();
    if Vec3::dot(&in_unit_sphere, n) > 0.0 {
        return in_unit_sphere;
    }
    return -&in_unit_sphere;
}