use std::marker::PhantomData;

use rx_bevy_observable::Observer;
use rx_bevy_operator::{Operator, OperatorCallback, OperatorInstance};

pub struct MapOperator<In, Out, F> {
	pub callback: F,
	pub _phantom_data_in: PhantomData<In>,
	pub _phantom_data_out: PhantomData<Out>,
}

impl<In, Out, F> Clone for MapOperator<In, Out, F>
where
	F: Clone,
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
		}
	}
}

impl<In, Out, F> Operator for MapOperator<In, Out, F>
where
	F: OperatorCallback<In, Out>,
{
	type In = In;
	type Out = Out;

	type Instance = Self;

	fn create_operator_instance(&self) -> Self::Instance {
		self.clone()
	}
}

impl<In, Out, F> MapOperator<In, Out, F> {
	pub fn new(transform: F) -> Self {
		Self {
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
			callback: transform,
		}
	}
}

impl<In, Out, F> OperatorInstance for MapOperator<In, Out, F>
where
	F: OperatorCallback<In, Out>,
{
	type In = In;
	type Out = Out;

	fn push_forward<Destination: Observer<In = Out>>(
		&mut self,
		value: Self::In,
		destination: &mut Destination,
	) {
		let result = (self.callback)(value);
		destination.on_push(result);
	}
}
