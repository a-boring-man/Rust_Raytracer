use crate::Color3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub trait Materiable {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, atenuation: &mut Color3, r_out: &mut Ray) -> bool;
}
