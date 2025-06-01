use std::marker::PhantomData;

use rx_bevy_observable::Observer;

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct FnObserver<In, OnPush>
where
	OnPush: Fn(In) -> (),
{
	on_push: OnPush,
	_phantom_data: PhantomData<In>,
}

impl<In, OnPush> Observer<In> for FnObserver<In, OnPush>
where
	OnPush: Fn(In) -> (),
{
	fn on_push(&mut self, value: In) {
		(self.on_push)(value);
	}
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
