use rand::prelude::*;
use std::f64::consts::PI;

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