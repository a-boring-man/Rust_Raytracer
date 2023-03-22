use crate::vec3::Vec3;
use crate::material::Materiable;
use crate::utils::random_unit_vector;
use crate::ray::Ray;
use crate::hittable::HitRecord;

type Color3 = Vec3;

pub struct Lamberian {
    albedo: Color3,
}

impl Lamberian {
    pub fn new(albedo: &Color3) -> Lamberian {
        Lamberian { albedo: albedo.clone() }
    }
}

impl Materiable for Lamberian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, atenuation: &mut Color3, r_out: &mut Ray) -> bool {
        let mut scatter_direction: Vec3 = &hit_record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal.clone();
        }

        let scattered_ray: Ray = Ray::new(hit_record.p.clone(), scatter_direction);
        *atenuation = self.albedo.clone();
        true
    }
}