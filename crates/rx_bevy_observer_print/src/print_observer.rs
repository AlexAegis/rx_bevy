use std::{fmt::Debug, marker::PhantomData};

use rx_bevy_observable::{Observer, ObserverInput};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct PrintObserver<In, InError = ()>
where
	In: Debug,
	InError: Debug,
{
	prefix: &'static str,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> ObserverInput for PrintObserver<In, InError>
where
	In: Debug,
	InError: Debug,
{
	type In = In;
	type InError = InError;
}

impl<T, Error> Observer for PrintObserver<T, Error>
where
	T: Debug,
	Error: Debug,
{
	fn next(&mut self, next: Self::In) {
		println!("{} - next: {:?}", self.prefix, next);
	}

	fn error(&mut self, error: Self::InError) {
		println!("{} - error: {:?}", self.prefix, error);
	}

	fn complete(&mut self) {
		println!("{} - completed", self.prefix);
	}
}

impl<T, Error> PrintObserver<T, Error>
where
	T: Debug,
	Error: Debug,
{
	pub fn new(message: &'static str) -> Self {
		Self {
			prefix: message,
			_phantom_data: PhantomData,
		}
	}
}
