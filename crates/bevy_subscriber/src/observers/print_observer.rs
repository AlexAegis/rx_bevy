use std::{fmt::Debug, marker::PhantomData};

use super::Observer;

pub struct PrintObserver<T>
where
	T: Debug,
{
	_phantom_data: PhantomData<T>,
	message: String,
}

impl<T> PrintObserver<T>
where
	T: Debug,
{
	pub fn new(message: String) -> Self {
		Self {
			_phantom_data: PhantomData,
			message,
		}
	}
}

impl<T> Observer for PrintObserver<T>
where
	T: Debug,
{
	type In = T;

	fn on_push(&mut self, value: T) {
		println!("{} {:?}", self.message, value);
	}
}
