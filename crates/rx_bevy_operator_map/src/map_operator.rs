use std::marker::PhantomData;

use rx_bevy_observable::{ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::MapSubscriber;

pub struct MapOperator<Mapper, In, InError, Out>
where
	Mapper: Fn(In) -> Out,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, InError, Out)>,
}

impl<Mapper, In, InError, Out> MapOperator<Mapper, In, InError, Out>
where
	Mapper: Fn(In) -> Out,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<Mapper, In, InError, Out> Operator for MapOperator<Mapper, In, InError, Out>
where
	Mapper: 'static + Clone + Fn(In) -> Out,
	In: 'static,
	Out: 'static,
	InError: 'static,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		MapSubscriber<Mapper, In, InError, Out, Destination>;

	fn operator_subscribe<
		Destination: 'static
			+ Subscriber<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		MapSubscriber::new(destination, self.mapper.clone())
	}
}

impl<Mapper, In, InError, Out> ObservableOutput for MapOperator<Mapper, In, InError, Out>
where
	Mapper: Fn(In) -> Out,
	Out: 'static,
	InError: 'static,
{
	type Out = Out;
	type OutError = InError;
}

impl<Mapper, In, InError, Out> ObserverInput for MapOperator<Mapper, In, InError, Out>
where
	Mapper: Fn(In) -> Out,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<Mapper, In, InError, Out> Clone for MapOperator<Mapper, In, InError, Out>
where
	Mapper: Clone + Fn(In) -> Out,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}
