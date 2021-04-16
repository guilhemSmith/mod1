use super::Inputs;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub trait Entity: Debug {
	fn update(&mut self, delta: f32, inputs: &Inputs);
	fn as_any(&self) -> &dyn Any;
}

pub struct EntityStore {
	entities: HashMap<u128, Box<dyn Entity>>,
	next_key: u128,
	clear_queue: HashSet<u128>,
}

impl EntityStore {
	pub fn new() -> Self {
		EntityStore {
			entities: HashMap::new(),
			next_key: 0,
			clear_queue: HashSet::new(),
		}
	}

	pub fn update(&mut self, delta: f32, inputs: &Inputs) {
		for (_key, entity) in self.entities.iter_mut() {
			entity.update(delta, inputs);
		}
	}

	pub fn exec_clear(&mut self) {
		for key in self.clear_queue.iter() {
			self.entities.remove(key);
		}
		self.clear_queue = HashSet::new();
	}

	#[allow(dead_code)]
	pub fn insert(&mut self, entity: Box<dyn Entity>) -> u128 {
		while let Some(_) = self.entities.get(&self.next_key) {
			self.next_key += 1;
		}
		self.entities.insert(self.next_key, entity);
		return self.next_key;
	}

	#[allow(dead_code)]
	pub fn queue_to_clear(&mut self, key: u128) {
		self.clear_queue.insert(key);
	}

	#[allow(dead_code)]
	pub fn get(&mut self, key: u128) -> Option<&Box<dyn Entity>> {
		return self.entities.get(&key);
	}

	#[allow(dead_code)]
	pub fn get_mut(&mut self, key: u128) -> Option<&mut Box<dyn Entity>> {
		return self.entities.get_mut(&key);
	}
}
