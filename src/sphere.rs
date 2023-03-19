use crate::{Ray, hittable::Hit_Record, hittable::Hittable, Vec3};

type Point3 = Vec3;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, hit_record: &mut Hit_Record) -> bool {
        let oc: Vec3 = r.origin() - self.center.clone();
        let a: f64 = r.dir().length2();
        let half_b: f64 = Vec3::dot(&oc, &r.dir());
        let c: f64 = oc.length2() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd: f64 = discriminant.sqrt();
        let mut root: f64 = (- half_b - sqrtd) / a;
        if root < tmin || tmax < root {
            root = (- half_b + sqrtd) / a;
            if root < tmin || tmax < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(root);
        hit_record.normal = (hit_record.p.clone() - self.center.clone()) / self.radius;
        return true;
    }
}