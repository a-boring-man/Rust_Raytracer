use crate::Color3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub trait Materiable {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, atenuation: &mut Color3, r_out: &mut Ray) -> bool;
}

impl<T: Materiable> Materiable for &T {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, atenuation: &mut Color3, r_out: &mut Ray) -> bool {
        (**self).scatter(r_in, hit_record, atenuation, r_out)
    }
}