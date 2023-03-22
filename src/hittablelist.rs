use crate::hittable::*;
use crate::Ray;
use std::rc::Rc;

pub struct Hittablelist {
	list: Vec<Rc<dyn Hittable>>,
}

#[allow(dead_code)]
impl Hittablelist {

	pub fn new() -> Hittablelist {
		Hittablelist {list: Vec::new()}
	}

	pub fn new_add<T: Hittable + 'static>(object: Rc<T>) -> Hittablelist {
		Hittablelist { list: vec![object]}
	}

	pub fn clear(&mut self) {
		self.list.clear();
	}

	pub fn add<T: Hittable + 'static>(&mut self, object: Rc<T>) {
		self.list.push(object);
	}
}

impl Hittable for Hittablelist {
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool {
		let mut hit_anything: bool = false;
		let mut closest_so_far: f64 = tmax;
		
		for object in &self.list {
			let mut tmp_record: HitRecord = HitRecord::new();
			if object.hit(r, tmin, closest_so_far, &mut tmp_record) {
				hit_anything = true;
				closest_so_far = tmp_record.t;
				*record = tmp_record;
			}
		}
		return hit_anything;
	}
	
}