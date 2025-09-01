use rx_bevy_core::{Observer, ObserverInput};
use rx_bevy_ref_subscriber_observer::ObserverSubscriber;
use rx_bevy_ref_subscriber_shared::SharedSubscriber;

#[derive(Debug)]
pub struct MockObserver<T, Error> {
	pub values: Vec<T>,
	pub errors: Vec<Error>,
	pub completed: bool,

	// #[cfg(feature = "tick")]
	pub ticks: Vec<rx_bevy_core::Tick>,
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
	#[inline]
	fn next(&mut self, next: T) {
		self.values.push(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.errors.push(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.completed = true;
	}

	#[inline]
	// #[cfg(feature = "tick")]
	fn tick(&mut self, tick: rx_bevy_core::Tick) {
		self.ticks.push(tick);
	}
}

impl<T, Error> Default for MockObserver<T, Error> {
	fn default() -> Self {
		Self {
			values: Vec::default(),
			errors: Vec::default(),
			completed: false,
			// #[cfg(feature = "tick")]
			ticks: Vec::default(),
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
