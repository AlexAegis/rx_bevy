use std::marker::PhantomData;

use rx_bevy_operator::{Operator, OperatorCallback};

pub struct MapOperator<In, Out, F> {
	pub mapper: F,
	pub _phantom_data_in: PhantomData<In>,
	pub _phantom_data_out: PhantomData<Out>,
}

impl<In, Out, Mapper> Operator for MapOperator<In, Out, Mapper>
where
	Mapper: OperatorCallback<In, Out>,
{
	type In = In;
	type Out = Out;

	type Instance = Self;

	fn create_operator_instance(&self) -> Self::Instance {
		self.clone()
	}

	fn operate(&mut self, next: Self::In) -> Self::Out {
		(self.mapper)(next)
	}
}

impl<In, Out, F> MapOperator<In, Out, F> {
	pub fn new(transform: F) -> Self {
		Self {
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
			mapper: transform,
		}
	}
}

impl<In, Out, F> Clone for MapOperator<In, Out, F>
where
	F: Clone,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
		}
	}
}
