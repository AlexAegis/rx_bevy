use std::{fmt::Debug, marker::PhantomData};

use super::Observer;

pub struct PrintObserver<T>
where
	T: Debug,
{
	_phantom_data: PhantomData<T>,
}

impl<T> PrintObserver<T>
where
	T: Debug,
{
	pub fn new() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<T> Observer<T> for PrintObserver<T>
where
	T: Debug,
{
	fn on_push(&mut self, value: T) {
		println!("{:?}", value);
	}
}
