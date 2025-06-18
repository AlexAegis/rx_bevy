use rx_bevy_observable::{
	Observer, ObserverInput,
	prelude::{ObserverSubscriber, SharedSubscriber},
};

#[derive(Debug)]
pub struct MockObserver<T, Error> {
	pub values: Vec<T>,
	pub errors: Vec<Error>,
	pub completed: bool,
}

impl<T, Error> ObserverInput for MockObserver<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type In = T;
	type InError = Error;
}

impl<T, Error> Observer for MockObserver<T, Error>
where
	T: 'static,
	Error: 'static,
{
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

impl<T, Error> Default for MockObserver<T, Error> {
	fn default() -> Self {
		Self {
			values: Vec::default(),
			errors: Vec::default(),
			completed: false,
		}
	}
}

impl<T, Error> MockObserver<T, Error>
where
	T: 'static + Clone,
	Error: 'static,
{
	pub fn new_shared() -> SharedSubscriber<ObserverSubscriber<Self>> {
		SharedSubscriber::new(ObserverSubscriber::new(Self::default()))
	}
}
