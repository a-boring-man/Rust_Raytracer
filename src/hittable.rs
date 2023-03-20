use crate::Ray;
use crate::Vec3;

type Point3 = Vec3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.dir(), outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal.clone()} else {-outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, hit_record: &mut HitRecord) -> bool;
}