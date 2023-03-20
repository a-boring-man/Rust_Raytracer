use crate::hittable::*;
use crate::Ray;

pub struct Hittablelist {
	list: Vec<Box<dyn Hittable>>,
}

impl Hittablelist {

	pub fn new() -> Hittablelist {
		Hittablelist {list: Vec::new()}
	}

	pub fn new_add<T: Hittable + 'static>(object: Box<T>) -> Hittablelist {
		Hittablelist { list: vec![object]}
	}

	pub fn clear(&mut self) {
		self.list.clear();
	}

	pub fn add<T: Hittable + 'static>(&mut self, object: Box<T>) {
		self.list.push(object);
	}
}

impl Hittable for Hittablelist {
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool {
		let mut tmp_record: HitRecord = HitRecord::new();
		let mut hit_anything: bool = false;
		let mut closest_so_far: f64 = tmax;

		for object in &self.list {
			if object.hit(r, tmin, closest_so_far, &mut tmp_record) {
				hit_anything = true;
				closest_so_far = tmp_record.t;
				*record = tmp_record.clone();
			}
		}
		return hit_anything;
	}
	
}