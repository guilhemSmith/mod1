use super::{Inputs, Renderable, Renderer};
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub trait Entity: Debug {
	fn update(&mut self, _delta: f32, _inputs: &Inputs, _store: &EntityStore) {}
	fn as_renderable(&self) -> Option<&dyn Renderable> {
		return None;
	}
	fn as_any(&self) -> &dyn Any;
}

pub struct EntityStore {
	entities: HashMap<u128, RefCell<Box<dyn Entity>>>,
	new_queue: RefCell<Vec<(u128, Box<dyn Entity>)>>,
	del_queue: RefCell<HashSet<u128>>,
}

impl EntityStore {
	pub fn new() -> Self {
		EntityStore {
			entities: HashMap::new(),
			new_queue: RefCell::new(Vec::new()),
			del_queue: RefCell::new(HashSet::new()),
		}
	}

	pub fn update(&mut self, delta: f32, inputs: &Inputs) {
		// run entities logic
		for (_key, entity) in self.entities.iter() {
			entity.borrow_mut().update(delta, inputs, self);
		}
		// remove dead entities
		for key in self.del_queue.borrow_mut().drain() {
			self.entities.remove(&key);
		}
		// insert new entities
		for (key, new_elem) in self.new_queue.borrow_mut().drain(0..) {
			println!("inserting...");
			self.entities.insert(key, RefCell::new(new_elem));
		}
	}

	pub fn render(&self, renderer: &mut Renderer) -> bool {
		renderer.clear();
		for (_key, entity) in self.entities.iter() {
			if let Some(renderable) = entity.borrow().as_renderable() {
				renderer.render(renderable, self);
			}
		}
		return if let Err(err) = renderer.swap() {
			eprintln!("{}", err);
			false
		} else {
			true
		};
	}

	#[allow(dead_code)]
	pub fn insert(&mut self, entity: Box<dyn Entity>) -> u128 {
		let keys: HashSet<u128> = self.entities.keys().cloned().collect();
		let mut next_key: u128 = 0;
		while keys.contains(&next_key) {
			next_key += 1;
		}
		self.entities.insert(next_key, RefCell::new(entity));
		return next_key;
	}

	#[allow(dead_code)]
	pub fn to_new_queue(&self, entity: Box<dyn Entity>) -> u128 {
		let keys: HashSet<u128> = self.entities.keys().cloned().collect();
		let mut next_key: u128 = 0;
		while keys.contains(&next_key) {
			next_key += 1;
		}
		self.new_queue.borrow_mut().push((next_key, entity));
		return next_key;
	}

	#[allow(dead_code)]
	pub fn to_del_queue(&self, key: u128) {
		self.del_queue.borrow_mut().insert(key);
	}

	#[allow(dead_code)]
	pub fn get(&self, key: u128) -> Option<Ref<'_, Box<dyn Entity>>> {
		return self.entities.get(&key).map(|rcell| rcell.borrow());
	}

	#[allow(dead_code)]
	pub fn get_mut(&self, key: u128) -> Option<RefMut<Box<dyn Entity>>> {
		return self.entities.get(&key).map(|rcell| rcell.borrow_mut());
	}
}
