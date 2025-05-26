use std::sync::{Arc, RwLock};

use rx_bevy_observable::Observer;

#[derive(Default, Debug)]
pub struct MockObserver<T> {
	pub values: Vec<T>,
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

impl<T> Observer for MockObserver<T> {
	type In = T;

	fn on_push(&mut self, value: Self::In) {
		self.values.push(value);
	}
}

pub struct FwObserver<T, Destination>
where
	Destination: Observer<In = T>,
{
	pub destination: Arc<RwLock<Destination>>,
}

impl<T, Destination> FwObserver<T, Destination>
where
	Destination: Observer<In = T>,
{
	pub fn new(destination: &Arc<RwLock<Destination>>) -> Self {
		Self {
			destination: destination.clone(),
		}
	}
}

impl<T, Destination> Observer for FwObserver<T, Destination>
where
	Destination: Observer<In = T>,
{
	type In = T;
	fn on_push(&mut self, value: Self::In) {
		let mut lock = self.destination.write().expect("to be unlocked");
		lock.on_push(value);
	}
}
