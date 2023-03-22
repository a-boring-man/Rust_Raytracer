use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::material::Materiable;

type Color3 = Vec3;

pub struct Metal {
    albedo: Color3,
}

impl Metal {
    pub fn new(color: &Color3) -> Metal {
        Metal { albedo: color.clone() }
    }
}

impl Materiable for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, atenuation: &mut crate::Color3, r_out: &mut Ray) -> bool {
        let reflected: Vec3 = Vec3::reflect(&r_in.dir().normalized(), &hit_record.normal);
        *r_out = Ray::new(hit_record.p.clone(), reflected);
        *atenuation = self.albedo.clone();
        Vec3::dot(&r_out.dir(), &hit_record.normal) > 0.0
    }
}