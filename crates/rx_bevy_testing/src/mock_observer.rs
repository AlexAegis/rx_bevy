use std::sync::{Arc, RwLock};

use rx_bevy_observable::Observer;

#[derive(Default, Debug)]
pub struct MockObserver<T, Error> {
	pub values: Vec<T>,
	pub errors: Vec<Error>,
	pub completed: bool,
}

impl<T, Error> Observer for MockObserver<T, Error> {
	type In = T;
	type Error = Error;

	fn next(&mut self, next: T) {
		self.values.push(next);
	}

	fn error(&mut self, error: Self::Error) {
		self.errors.push(error);
	}

	fn complete(&mut self) {
		self.completed = true;
	}
}

impl<T, Error> MockObserver<T, Error>
where
	T: Clone,
{
	pub fn new() -> Self {
		MockObserver {
			values: Vec::new(),
			errors: Vec::new(),
			completed: false,
		}
	}

	pub fn new_shared() -> Arc<RwLock<Self>> {
		Arc::new(RwLock::new(Self::new()))
	}
}

pub struct SharedForwardObserver<Destination> {
	pub destination: Arc<RwLock<Destination>>,
	closed: bool,
}

impl<Destination> SharedForwardObserver<Destination> {
	pub fn new(destination: &Arc<RwLock<Destination>>) -> Self {
		Self {
			destination: destination.clone(),
			closed: false,
		}
	}
}

impl<T, Error, Destination> Observer for SharedForwardObserver<Destination>
where
	Destination: Observer<In = T, Error = Error>,
{
	type In = T;
	type Error = Error;

	fn next(&mut self, value: T) {
		if !self.closed {
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.next(value);
		}
	}

	fn error(&mut self, error: Self::Error) {
		if !self.closed {
			self.closed = true;
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.error(error);
		}
	}

	fn complete(&mut self) {
		if !self.closed {
			self.closed = true;
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.complete();
		}
	}
}
