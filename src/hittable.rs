use crate::Ray;
use crate::Vec3;
use crate::RAY_T_MAX;
use crate::material::Materiable;

type Point3 = Vec3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Option<Box<dyn Materiable>>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {

    pub fn new() -> HitRecord{
        HitRecord { p: (Vec3::new(0.0, 0.0, 0.0)), normal: (Vec3::new(0.0, 0.0, 0.0)), mat_ptr: None, t: (RAY_T_MAX), front_face: (true) }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.dir(), outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal.clone()} else {-outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, hit_record: &mut HitRecord) -> bool;
}