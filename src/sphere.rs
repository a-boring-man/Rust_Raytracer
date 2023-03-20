use crate::{Ray, hittable::*, Vec3};

type Point3 = Vec3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Sphere {
    pub fn new (center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, hit_record: &mut HitRecord) -> bool {
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
        let normal: Vec3 = (hit_record.p.clone() - self.center.clone()) / self.radius;
        hit_record.set_face_normal(r, &normal);
        return true;
    }
}