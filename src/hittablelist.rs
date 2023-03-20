use crate::hittable::*;

pub struct Hittablelist {
	list: Vec<Box<dyn Hittable>>,
}

impl Hittablelist {

	pub fn new() -> Hittablelist {
		Hittablelist {list: Vec::new()}
	}

	pub fn new_add<T: Hittable>(object: &T) -> Hittablelist {
		Hittablelist { list: vec![Box::new(object)]}
	}
}