use std::{fmt::Debug, marker::PhantomData};

use rx_bevy_observable::Observer;

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct PrintObserver<T>
where
	T: Debug,
{
	_phantom_data: PhantomData<T>,
	message: &'static str,
}

impl<T> Observer<T> for PrintObserver<T>
where
	T: Debug,
{
	fn on_push(&mut self, value: T) {
		println!("{} {:?}", self.message, value);
	}
}

impl<T> PrintObserver<T>
where
	T: Debug,
{
	pub fn new(message: &'static str) -> Self {
		Self {
			_phantom_data: PhantomData,
			message,
		}
	}
}
