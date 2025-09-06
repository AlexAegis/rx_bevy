use std::marker::PhantomData;

use derive_where::derive_where;
use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::MapSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
pub struct MapOperator<In, InError, Mapper, Out = In>
where
	Mapper: Fn(In) -> Out,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, InError, Out)>,
}

impl<In, InError, Mapper, Out> MapOperator<In, InError, Mapper, Out>
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

impl<In, InError, Mapper, Out> Operator for MapOperator<In, InError, Mapper, Out>
where
	In: 'static,
	InError: 'static,
	Mapper: 'static + Clone + Fn(In) -> Out,
	Out: 'static,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		MapSubscriber<In, InError, Mapper, Out, Destination>;

	fn operator_subscribe<
		Destination: 'static
			+ Subscriber<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
		_context: &mut Destination::Context,
	) -> Self::Subscriber<Destination> {
		MapSubscriber::new(destination, self.mapper.clone())
	}
}

impl<In, InError, Mapper, Out> ObservableOutput for MapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Out,
	Out: 'static,
	InError: 'static,
{
	type Out = Out;
	type OutError = InError;
}

impl<In, InError, Mapper, Out> ObserverInput for MapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Out,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Mapper, Out> Clone for MapOperator<In, InError, Mapper, Out>
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
