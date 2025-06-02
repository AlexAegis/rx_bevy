use std::{fmt::Debug, marker::PhantomData};

use rx_bevy_observable::Observer;

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct PrintObserver<T, Error = ()>
where
	T: Debug,
	Error: Debug,
{
	message: &'static str,
	_phantom_data_in: PhantomData<T>,
	_phantom_data_error: PhantomData<Error>,
}

impl<T, Error> Observer for PrintObserver<T, Error>
where
	T: Debug,
	Error: Debug,
{
	type In = T;
	type Error = Error;

	fn on_push(&mut self, value: T) {
		println!("next: {} {:?}", self.message, value);
	}

	fn on_error(&mut self, error: Self::Error) {
		println!("error: {} {:?}", self.message, error);
	}

	fn on_complete(&mut self) {
		println!("completed: {}", self.message);
	}
}

impl<T, Error> PrintObserver<T, Error>
where
	T: Debug,
	Error: Debug,
{
	pub fn new(message: &'static str) -> Self {
		Self {
			message,
			_phantom_data_in: PhantomData,
			_phantom_data_error: PhantomData,
		}
	}
}
