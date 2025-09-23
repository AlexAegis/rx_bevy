use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, ObservableOutput, ObserverInput, Operator, Subscriber, SubscriptionCollection,
};
use rx_bevy_operator_composite::CompositeSubscriber;
use rx_bevy_operator_lift_option::LiftOptionSubscriber;
use rx_bevy_operator_map::MapSubscriber;

pub type FilterMapSubscriber<In, InError, Mapper, Out, Destination> = CompositeSubscriber<
	MapSubscriber<
		In,
		InError,
		Mapper,
		Option<Out>,
		LiftOptionSubscriber<Out, InError, Destination>,
	>,
	Destination,
>;

pub struct FilterMapOperator<In, InError, Mapper, Out, Context = ()>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, Out, InError, Context)>,
}

impl<In, InError, Mapper, Out, Context> FilterMapOperator<In, InError, Mapper, Out, Context>
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

impl<In, InError, Mapper, Out, Context> Operator
	for FilterMapOperator<In, InError, Mapper, Out, Context>
where
	Mapper: 'static + Clone + Fn(In) -> Option<Out>,
	In: 'static,
	Out: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= FilterMapSubscriber<In, InError, Mapper, Out, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection,
	{
		CompositeSubscriber::new(MapSubscriber::new(
			LiftOptionSubscriber::new(destination),
			self.mapper.clone(),
		))
	}
}

impl<In, InError, Mapper, Out, Context> ObserverInput
	for FilterMapOperator<In, InError, Mapper, Out, Context>
where
	Mapper: Fn(In) -> Option<Out>,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Mapper, Out, Context> ObservableOutput
	for FilterMapOperator<In, InError, Mapper, Out, Context>
where
	Mapper: Fn(In) -> Option<Out>,
	Out: 'static,
	InError: 'static,
{
	type Out = Out;
	type OutError = InError;
}

impl<In, InError, Mapper, Out, Context> Clone
	for FilterMapOperator<In, InError, Mapper, Out, Context>
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
