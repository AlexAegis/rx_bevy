use std::marker::PhantomData;

use super::Observer;

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct FnObserver<In, OnPush>
where
	OnPush: Fn(In) -> (),
{
	on_push: OnPush,
	_phantom_data: PhantomData<In>,
}

impl<In, OnPush> FnObserver<In, OnPush>
where
	OnPush: Fn(In) -> (),
{
	pub fn new(on_push: OnPush) -> Self {
		Self {
			on_push,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, OnPush> Observer for FnObserver<In, OnPush>
where
	OnPush: Fn(In) -> (),
{
	type In = In;

	fn on_push(&mut self, value: In) {
		(self.on_push)(value);
	}
}
