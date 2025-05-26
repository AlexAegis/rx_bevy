use std::marker::PhantomData;

use rx_bevy_observable::Observer;

use rx_bevy_operator::{Operator, OperatorInstance};

pub struct TapOperator<In, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<In>,
}

impl<In, Callback> Clone for TapOperator<In, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Callback> TapOperator<In, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	pub fn new(callback: Callback) -> Self {
		Self {
			_phantom_data: PhantomData,
			callback,
		}
	}
}

impl<In, Callback> Operator for TapOperator<In, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type In = In;
	type Out = In;

	type Instance = Self;

	fn create_operator_instance(&self) -> Self::Instance {
		self.clone()
	}
}

impl<In, Callback> OperatorInstance for TapOperator<In, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	type In = In;
	type Out = In;

	fn push_forward<Destination: Observer<In = Self::Out>>(
		&mut self,
		value: Self::In,
		destination: &mut Destination,
	) {
		(self.callback)(&value);
		destination.on_push(value);
	}
}
