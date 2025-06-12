use rx_bevy_observable::{Observer, ObserverInput};
use rx_bevy_observer_shared::SharedObserver;

#[derive(Default, Debug)]
pub struct MockObserver<T, Error> {
	pub values: Vec<T>,
	pub errors: Vec<Error>,
	pub completed: bool,
}

impl<T, Error> ObserverInput for MockObserver<T, Error> {
	type In = T;
	type InError = Error;
}

impl<T, Error> Observer for MockObserver<T, Error> {
	fn next(&mut self, next: T) {
		self.values.push(next);
	}

	fn error(&mut self, error: Self::InError) {
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

	pub fn new_shared() -> SharedObserver<Self> {
		SharedObserver::new(Self::new())
	}
}
