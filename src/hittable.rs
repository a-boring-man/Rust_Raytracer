use crate::Ray;
use crate::Vec3;

type Point3 = Vec3;

pub struct Hit_Record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, hit_record: &mut Hit_Record) -> bool;
}