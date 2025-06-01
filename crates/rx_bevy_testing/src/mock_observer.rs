use std::sync::{Arc, RwLock};

use rx_bevy_observable::Observer;

#[derive(Default, Debug)]
pub struct MockObserver<T> {
	pub values: Vec<T>,
}

impl<T> Observer<T> for MockObserver<T> {
	// type In = T;

	fn on_push(&mut self, value: T) {
		self.values.push(value);
	}
}

impl<T> MockObserver<T>
where
	T: Clone,
{
	pub fn new() -> Self {
		MockObserver { values: Vec::new() }
	}

	pub fn new_shared() -> Arc<RwLock<Self>> {
		Arc::new(RwLock::new(Self::new()))
	}
}

pub struct SharedForwardObserver<Destination> {
	pub destination: Arc<RwLock<Destination>>,
}

impl<Destination> SharedForwardObserver<Destination> {
	pub fn new(destination: &Arc<RwLock<Destination>>) -> Self {
		Self {
			destination: destination.clone(),
		}
	}
}

impl<T, Destination> Observer<T> for SharedForwardObserver<Destination>
where
	Destination: Observer<T>,
{
	fn on_push(&mut self, value: T) {
		let mut lock = self.destination.write().expect("to be unlocked");
		lock.on_push(value);
	}
}
