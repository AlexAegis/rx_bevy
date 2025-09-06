use std::marker::PhantomData;

use derive_where::derive_where;
use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
pub struct MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber,
{
	destination: Destination,
	mapper: Mapper,
	_phantom_data: PhantomData<(In, InError, Out)>,
}

impl<In, InError, Mapper, Out, Destination> MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination, mapper: Mapper) -> Self {
		Self {
			destination,
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out, Destination> Observer
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let mapped = (self.mapper)(next);
		self.destination.next(mapped);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_core::Tick) {
		self.destination.tick(tick);
	}
}

impl<In, InError, Mapper, Out, Destination> SubscriptionLike
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: impl Into<Teardown>) {
		self.destination.add(subscription);
	}
}

impl<In, InError, Mapper, Out, Destination> ObserverInput
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Mapper, Out, Destination> ObservableOutput
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber,
{
	type Out = Out;
	type OutError = InError;
}

impl<In, InError, Mapper, Out, Destination> Operation
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Destination = Destination;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}
