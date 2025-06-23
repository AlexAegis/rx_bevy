use std::marker::PhantomData;

use rx_bevy_observable::{
	CompositeSubscriber, ObservableOutput, ObserverInput, Operator, Subscriber,
};
use rx_bevy_operator_lift_option::LiftOptionSubscriber;
use rx_bevy_operator_map::MapSubscriber;

pub type FilterMapSubscriber<In, InError, Mapper, Out, D> = CompositeSubscriber<
	MapSubscriber<In, InError, Mapper, Option<Out>, LiftOptionSubscriber<Out, InError, D>>,
	D,
>;

pub struct FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, Out, InError)>,
}

impl<In, InError, Mapper, Out> FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out> Operator for FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: 'static + Clone + Fn(In) -> Option<Out>,
	In: 'static,
	Out: 'static,
	InError: 'static,
{
	type Subscriber<D: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		FilterMapSubscriber<In, InError, Mapper, Out, D>;

	fn operator_subscribe<
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		CompositeSubscriber::new(MapSubscriber::new(
			LiftOptionSubscriber::new(destination),
			self.mapper.clone(),
		))
	}
}

impl<In, InError, Mapper, Out> ObserverInput for FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Option<Out>,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Mapper, Out> ObservableOutput for FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Option<Out>,
	Out: 'static,
	InError: 'static,
{
	type Out = Out;
	type OutError = InError;
}

impl<In, InError, Mapper, Out> Clone for FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Clone + Fn(In) -> Option<Out>,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}
