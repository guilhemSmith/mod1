use super::{Inputs, Renderable, Renderer};
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub trait Entity: Debug {
	fn update(&mut self, _delta: f32, _inputs: &Inputs, _store: &EntityStore) {}
	fn start(&mut self, _store: &EntityStore) {}
	fn as_renderable(&self) -> Option<&dyn Renderable> {
		return None;
	}
	fn as_any(&self) -> &dyn Any;
}

pub struct EntityStore {
	entities: HashMap<u128, RefCell<Box<dyn Entity>>>,
	reserved_keys: RefCell<HashSet<u128>>,
	opaques_renderables: HashSet<u128>,
	transparent_renderables: HashSet<u128>,
	new_queue: RefCell<Vec<(u128, Box<dyn Entity>)>>,
	del_queue: RefCell<HashSet<u128>>,
}

impl EntityStore {
	pub fn new() -> Self {
		EntityStore {
			entities: HashMap::new(),
			reserved_keys: RefCell::new(HashSet::new()),
			opaques_renderables: HashSet::new(),
			transparent_renderables: HashSet::new(),
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
			if self.opaques_renderables.contains(&key) {
				self.opaques_renderables.remove(&key);
			}
			if self.transparent_renderables.contains(&key) {
				self.transparent_renderables.remove(&key);
			}
			self.reserved_keys.borrow_mut().remove(&key);
		}
		// insert new entities
		for (key, mut new_elem) in self.new_queue.borrow_mut().drain(0..) {
			new_elem.start(&self);
			if let Some(renderable) = new_elem.as_renderable() {
				if renderable.is_opaque() {
					self.opaques_renderables.insert(key);
				} else {
					self.transparent_renderables.insert(key);
				}
			}
			self.entities.insert(key, RefCell::new(new_elem));
		}
	}

	pub fn render(&self, renderer: &mut Renderer) -> bool {
		renderer.clear();
		for key in self.opaques_renderables.iter() {
			if let Some(entity) = self.entities.get(key) {
				if let Some(renderable) = entity.borrow().as_renderable() {
					renderer.render(renderable, self);
				}
			}
		}
		for key in self.transparent_renderables.iter() {
			if let Some(entity) = self.entities.get(key) {
				if let Some(renderable) = entity.borrow().as_renderable() {
					renderer.render(renderable, self);
				}
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
	pub fn insert(&mut self, mut entity: Box<dyn Entity>) -> u128 {
		let mut keys = self.reserved_keys.borrow_mut();
		let mut next_key: u128 = 0;
		while keys.contains(&next_key) {
			next_key += 1;
		}
		entity.start(&self);
		keys.insert(next_key);
		if let Some(renderable) = entity.as_renderable() {
			if renderable.is_opaque() {
				self.opaques_renderables.insert(next_key);
			} else {
				self.transparent_renderables.insert(next_key);
			}
		}
		self.entities.insert(next_key, RefCell::new(entity));
		return next_key;
	}

	#[allow(dead_code)]
	pub fn to_new_queue(&self, mut entity: Box<dyn Entity>) -> u128 {
		let mut keys = self.reserved_keys.borrow_mut();
		let mut next_key: u128 = 0;
		while keys.contains(&next_key) {
			next_key += 1;
		}
		entity.start(&self);
		keys.insert(next_key);
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
